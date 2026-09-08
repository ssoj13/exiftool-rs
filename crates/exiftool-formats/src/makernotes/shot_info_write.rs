//! Overlay decrypted Nikon ShotInfo (`0x0091`) fields, then re-encrypt.
//! Cipher keys stay the original IFD serial / shutter (not the new payload values).

use super::*;
use exiftool_attrs::{AttrValue, Attrs};
use exiftool_core::IfdEntry;

/// Fields to overlay inside decrypted ShotInfo. Cipher keys stay original.
pub(crate) struct ShotInfoWrite {
    pub firmware_version: Option<String>,
    pub vibration_reduction: Option<String>,
    pub shutter_count: Option<u32>,
    pub custom_settings: Vec<(String, Attrs)>,
}

impl ShotInfoWrite {
    pub(crate) fn is_empty(&self) -> bool {
        self.firmware_version.is_none()
            && self.vibration_reduction.is_none()
            && self.shutter_count.is_none()
            && self.custom_settings.is_empty()
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

/// Decrypt, patch same-size fields, encrypt. Full-crypt and `NIKON_OFFSETS` piecewise.
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
    match shot_crypt(&ver, count) {
        ShotCrypt::None => None,
        ShotCrypt::Full { big_endian } => {
            let mut payload = nikon_decrypt::decrypt(extra, 4, serial_key, shutter_key);
            if !patch_shot_fields(&mut payload, &ver, count, big_endian, write) {
                return None;
            }
            Some(nikon_decrypt::decrypt(&payload, 4, serial_key, shutter_key))
        }
        ShotCrypt::Offsets { table, big_endian } => {
            let mut payload = extra.to_vec();
            let (_, ranges) = nikon_decrypt::prepare_nikon_offsets_ex(
                &mut payload,
                4,
                table,
                big_endian,
                serial_key,
                shutter_key,
            )?;
            if !patch_shot_fields(&mut payload, &ver, count, big_endian, write) {
                return None;
            }
            nikon_decrypt::apply_offset_ranges(&mut payload, serial_key, shutter_key, &ranges);
            Some(payload)
        }
    }
}

fn patch_shot_fields(
    payload: &mut [u8],
    ver: &str,
    count: usize,
    big_endian: bool,
    write: &ShotInfoWrite,
) -> bool {
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
        if let Some(off) = shutter_count_offset(ver, count) {
            if payload.len() >= off + 4 {
                write_u32(payload, off, n, order);
                changed = true;
            }
        } else if ver.starts_with("0204") && payload.len() >= 0x6e {
            write_u32(payload, 0x6a, n, ByteOrder::BigEndian);
            changed = true;
        }
    }
    if let Some(vr) = &write.vibration_reduction {
        if overlay_vr(payload, ver, vr) {
            changed = true;
        }
    }
    if !write.custom_settings.is_empty() {
        if let Some(table) = super::shot_info_table_name(&ver, count) {
            let order = if big_endian {
                ByteOrder::BigEndian
            } else {
                ByteOrder::LittleEndian
            };
            if overlay_custom_settings_tree(payload, table, order, &write.custom_settings) {
                changed = true;
            }
        }
    }
    changed
}

pub(crate) fn color_balance_tag_name(extra: &[u8]) -> Option<&'static str> {
    if extra.len() < 4 {
        return None;
    }
    let ver = String::from_utf8_lossy(&extra[..4]);
    Some(color_balance_layout(&ver).2)
}

pub(crate) fn rewrite_color_balance(
    extra: &[u8],
    serial_key: u32,
    shutter_key: u32,
    order: ByteOrder,
    levels: &[u16; 4],
) -> Option<Vec<u8>> {
    if extra.len() < 4 {
        return None;
    }
    let ver = String::from_utf8_lossy(&extra[..4]).to_string();
    let (decrypt_start, dir_off, _) = color_balance_layout(&ver);
    let mut payload = if let Some(start) = decrypt_start {
        nikon_decrypt::decrypt(extra, start, serial_key, shutter_key)
    } else {
        extra.to_vec()
    };
    if payload.len() < dir_off + 8 {
        return None;
    }
    for (i, n) in levels.iter().enumerate() {
        write_u16(&mut payload, dir_off + i * 2, *n, order);
    }
    if let Some(start) = decrypt_start {
        Some(nikon_decrypt::decrypt(
            &payload,
            start,
            serial_key,
            shutter_key,
        ))
    } else {
        Some(payload)
    }
}

pub(crate) fn rewrite_lens_data(
    extra: &[u8],
    serial_key: u32,
    shutter_key: u32,
    lens_id: u8,
) -> Option<Vec<u8>> {
    if extra.len() < 4 {
        return None;
    }
    let ver = String::from_utf8_lossy(&extra[..4]).to_string();
    let encrypted = ver.starts_with("02") || ver.starts_with("04") || ver.starts_with("08");
    let mut payload = if encrypted {
        nikon_decrypt::decrypt(extra, 4, serial_key, shutter_key)
    } else {
        extra.to_vec()
    };
    let table_01 = ver.starts_with("0101") || ver.starts_with("02") || ver.starts_with("040");
    let off = if table_01 {
        0x0b
    } else if ver.starts_with("0100") {
        0x06
    } else {
        return None;
    };
    if payload.len() <= off {
        return None;
    }
    payload[off] = lens_id;
    if encrypted {
        Some(nikon_decrypt::decrypt(&payload, 4, serial_key, shutter_key))
    } else {
        Some(payload)
    }
}

fn write_u16(data: &mut [u8], offset: usize, value: u16, byte_order: ByteOrder) {
    if offset + 2 > data.len() {
        return;
    }
    let b = match byte_order {
        ByteOrder::LittleEndian => value.to_le_bytes(),
        ByteOrder::BigEndian => value.to_be_bytes(),
    };
    data[offset..offset + 2].copy_from_slice(&b);
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
            custom_settings: Vec::new(),
        };
        let out = rewrite_shot_info_full(&extra, serial, shutter, &write).unwrap();
        let back = nikon_decrypt::decrypt(&out, 4, serial, shutter);
        assert_eq!(&back[0..4], b"0204");
        assert_eq!(&back[4..8], b"2.10");
        assert_eq!(back[0x82], 0);
        assert_eq!(read_u32(&back, 0x6a, ByteOrder::BigEndian), 200);
    }

    #[test]
    fn rewrite_0245_offsets_firmware() {
        let serial = 7u32;
        let shutter = 99u32;
        let mut plain = vec![0u8; 200];
        plain[0..4].copy_from_slice(b"0245");
        plain[4..8].copy_from_slice(b"1.00");
        plain[0x24..0x28].copy_from_slice(&2u32.to_le_bytes());
        plain[0x28..0x2c].copy_from_slice(&120u32.to_le_bytes());
        plain[0x2c..0x30].copy_from_slice(&160u32.to_le_bytes());
        plain[120..124].copy_from_slice(b"ABCD");
        plain[160..164].copy_from_slice(b"EFGH");
        let ranges = nikon_decrypt::nikon_offset_ranges_plain(&plain, 4, 0x24, false).unwrap();
        let mut extra = plain.clone();
        nikon_decrypt::apply_offset_ranges(&mut extra, serial, shutter, &ranges);
        let write = ShotInfoWrite {
            firmware_version: Some("2.10".into()),
            vibration_reduction: None,
            shutter_count: None,
            custom_settings: Vec::new(),
        };
        let out = rewrite_shot_info_full(&extra, serial, shutter, &write).unwrap();
        let mut back = out;
        nikon_decrypt::prepare_nikon_offsets(&mut back, 4, 0x24, false, serial, shutter).unwrap();
        assert_eq!(&back[0..4], b"0245");
        assert_eq!(&back[4..8], b"2.10");
        assert_eq!(&back[120..124], b"ABCD");
    }

    #[test]
    fn rewrite_d500_custom_settings_offset() {
        let serial = 7u32;
        let shutter = 99u32;
        let mut plain = vec![0u8; 320];
        plain[0..4].copy_from_slice(b"0238");
        plain[0x0c..0x10].copy_from_slice(&1u32.to_le_bytes());
        plain[0x10..0x14].copy_from_slice(&200u32.to_le_bytes());
        plain[88..92].copy_from_slice(&200u32.to_le_bytes());
        let ranges = nikon_decrypt::nikon_offset_ranges_plain(&plain, 4, 0x0c, false).unwrap();
        let mut extra = plain.clone();
        nikon_decrypt::apply_offset_ranges(&mut extra, serial, shutter, &ranges);
        let mut g = Attrs::new();
        g.set("CustomSettingsBank", AttrValue::Str("B".into()));
        let write = ShotInfoWrite {
            firmware_version: None,
            vibration_reduction: None,
            shutter_count: None,
            custom_settings: vec![("CustomSettingsD500".into(), g)],
        };
        let out = rewrite_shot_info_full(&extra, serial, shutter, &write).unwrap();
        let mut back = out;
        nikon_decrypt::prepare_nikon_offsets(&mut back, 4, 0x0c, false, serial, shutter).unwrap();
        assert_eq!(back[200] & 0x3, 1);
    }

    #[test]
    fn rewrite_color_balance_0205() {
        let serial = 7u32;
        let shutter = 99u32;
        let mut plain = vec![0u8; 40];
        plain[0..4].copy_from_slice(b"0205");
        let extra = nikon_decrypt::decrypt(&plain, 4, serial, shutter);
        let out = rewrite_color_balance(
            &extra,
            serial,
            shutter,
            ByteOrder::LittleEndian,
            &[11, 22, 33, 44],
        )
        .unwrap();
        let back = nikon_decrypt::decrypt(&out, 4, serial, shutter);
        assert_eq!(&back[0..4], b"0205");
        assert_eq!(u16::from_le_bytes([back[18], back[19]]), 11);
        assert_eq!(u16::from_le_bytes([back[20], back[21]]), 22);
        assert_eq!(u16::from_le_bytes([back[22], back[23]]), 33);
        assert_eq!(u16::from_le_bytes([back[24], back[25]]), 44);
    }

    #[test]
    fn rewrite_lens_data_0201() {
        let serial = 7u32;
        let shutter = 99u32;
        let mut plain = vec![0u8; 32];
        plain[0..4].copy_from_slice(b"0201");
        plain[0x0b] = 10;
        let extra = nikon_decrypt::decrypt(&plain, 4, serial, shutter);
        let out = rewrite_lens_data(&extra, serial, shutter, 99).unwrap();
        let back = nikon_decrypt::decrypt(&out, 4, serial, shutter);
        assert_eq!(back[0x0b], 99);
        assert_eq!(&back[0..4], b"0201");
    }
}
