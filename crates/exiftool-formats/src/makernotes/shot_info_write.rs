//! Overlay decrypted Nikon ShotInfo (`0x0091`) fields, then re-encrypt.
//! Cipher keys stay the original IFD serial / shutter (not the new payload values).

use super::*;
use exiftool_core::IfdEntry;

/// Fields to overlay inside decrypted ShotInfo. Cipher keys stay original.
pub(crate) struct ShotInfoWrite {
    pub firmware_version: Option<String>,
    pub vibration_reduction: Option<String>,
    pub shutter_count: Option<u32>,
}

impl ShotInfoWrite {
    pub(crate) fn is_empty(&self) -> bool {
        self.firmware_version.is_none()
            && self.vibration_reduction.is_none()
            && self.shutter_count.is_none()
    }
}

pub(crate) fn decrypt_keys_from_ifd(entries: &[IfdEntry], model: Option<&str>) -> (u32, u32) {
    let mut serial_str = String::new();
    let mut shutter = 0u32;
    for e in entries {
        match e.tag {
            0x00A0 => {
                if let RawValue::String(s) = &e.value {
                    serial_str = s.clone();
                }
            }
            0x00A7 => {
                if let Some(n) = e.value.as_u32() {
                    shutter = n;
                }
            }
            _ => {}
        }
    }
    (nikon_decrypt::serial_key(&serial_str, model), shutter)
}

fn write_u32(data: &mut [u8], offset: usize, value: u32, byte_order: ByteOrder) {
    if offset + 4 > data.len() {
        return;
    }
    let b = match byte_order {
        ByteOrder::LittleEndian => value.to_le_bytes(),
        ByteOrder::BigEndian => value.to_be_bytes(),
    };
    data[offset..offset + 4].copy_from_slice(&b);
}

/// Decrypt, patch same-size fields, encrypt. `NIKON_OFFSETS` layouts are left unchanged.
pub(crate) fn rewrite_shot_info_full(
    extra: &[u8],
    serial_key: u32,
    shutter_key: u32,
    write: &ShotInfoWrite,
) -> Option<Vec<u8>> {
    if write.is_empty() || extra.len() < 4 {
        return None;
    }
    let ver = String::from_utf8_lossy(&extra[..4]).to_string();
    let count = extra.len();
    let ShotCrypt::Full { big_endian } = shot_crypt(&ver, count) else {
        return None;
    };
    let mut payload = nikon_decrypt::decrypt(extra, 4, serial_key, shutter_key);
    let mut changed = false;
    let fw_len = if ver.starts_with("08") { 8 } else { 5 };
    if let Some(fw) = &write.firmware_version {
        if payload.len() >= 4 + fw_len {
            let mut body = fw.as_bytes().to_vec();
            if body.len() > fw_len {
                body.truncate(fw_len);
            }
            while body.len() < fw_len {
                body.push(0);
            }
            payload[4..4 + fw_len].copy_from_slice(&body);
            changed = true;
        }
    }
    let order = if big_endian {
        ByteOrder::BigEndian
    } else {
        ByteOrder::LittleEndian
    };
    if let Some(n) = write.shutter_count {
        if let Some(off) = shutter_count_offset(&ver, count) {
            if payload.len() >= off + 4 {
                write_u32(&mut payload, off, n, order);
                changed = true;
            }
        } else if ver.starts_with("0204") && payload.len() >= 0x6e {
            write_u32(&mut payload, 0x6a, n, ByteOrder::BigEndian);
            changed = true;
        }
    }
    if let Some(vr) = &write.vibration_reduction {
        if overlay_vr(&mut payload, &ver, vr) {
            changed = true;
        }
    }
    if !changed {
        return None;
    }
    Some(nikon_decrypt::decrypt(&payload, 4, serial_key, shutter_key))
}

pub(crate) fn crypt_shot_info(data: &[u8], serial: u32, shutter: u32) -> Vec<u8> {
    nikon_decrypt::decrypt(data, 4, serial, shutter)
}

fn overlay_vr(payload: &mut [u8], ver: &str, label: &str) -> bool {
    let t = label.trim();
    if ver.starts_with("0209") {
        if payload.len() <= 586 {
            return false;
        }
        match vr_on(t) {
            Some(true) => payload[586] |= 0x08,
            Some(false) => payload[586] &= !0x08,
            None => return false,
        }
        return true;
    }
    if ver.starts_with("0204") {
        if payload.len() <= 0x82 {
            return false;
        }
        match vr_on(t) {
            Some(true) => payload[0x82] = 1,
            Some(false) => payload[0x82] = 0,
            None => return false,
        }
        return true;
    }
    if ver.starts_with("0207") {
        if payload.len() <= 0x75 {
            return false;
        }
        let code = match t {
            "Off" => 0,
            "On (1)" | "On" => 1,
            "On (2)" => 2,
            "On (3)" => 3,
            _ => return false,
        };
        payload[0x75] = code;
        return true;
    }
    false
}

fn vr_on(label: &str) -> Option<bool> {
    if label.eq_ignore_ascii_case("off") {
        Some(false)
    } else if label.to_ascii_lowercase().starts_with("on") {
        Some(true)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrite_0204_firmware_and_vr_roundtrip() {
        let serial = 7u32;
        let shutter = 99u32;
        let mut plain = vec![0u8; 140];
        plain[0..4].copy_from_slice(b"0204");
        plain[4..9].copy_from_slice(b"1.00\0");
        write_u32(&mut plain, 0x6a, 100, ByteOrder::BigEndian);
        plain[0x82] = 1;
        let extra = nikon_decrypt::decrypt(&plain, 4, serial, shutter);
        let write = ShotInfoWrite {
            firmware_version: Some("2.10".into()),
            vibration_reduction: Some("Off".into()),
            shutter_count: Some(200),
        };
        let out = rewrite_shot_info_full(&extra, serial, shutter, &write).unwrap();
        let back = nikon_decrypt::decrypt(&out, 4, serial, shutter);
        assert_eq!(&back[0..4], b"0204");
        assert_eq!(&back[4..8], b"2.10");
        assert_eq!(back[0x82], 0);
        assert_eq!(read_u32(&back, 0x6a, ByteOrder::BigEndian), 200);
    }
}
