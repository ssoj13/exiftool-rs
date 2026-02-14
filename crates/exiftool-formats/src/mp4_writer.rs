//! MP4/MOV format writer.
//!
//! Writes XMP metadata via UUID box (BE7ACFCB-97A9-42E8-9C71-999491E3AFAC).
//! Strategy: copy all boxes, replace or inject XMP uuid box inside moov.

use crate::{Error, Metadata, ReadSeek, Result};
use std::io::Write;

/// XMP UUID per Adobe XMP in QuickTime spec.
const XMP_UUID: [u8; 16] = [
    0xBE, 0x7A, 0xCF, 0xCB, 0x97, 0xA9, 0x42, 0xE8,
    0x9C, 0x71, 0x99, 0x94, 0x91, 0xE3, 0xAF, 0xAC,
];

/// MP4/MOV writer.
pub struct Mp4Writer;

impl Mp4Writer {
    /// Write MP4/MOV with updated metadata (XMP).
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        let data = crate::utils::read_with_limit(input)?;

        if data.len() < 12 || &data[4..8] != b"ftyp" {
            return Err(Error::InvalidStructure("Invalid MP4/MOV file".into()));
        }

        let xmp_data = metadata.xmp.as_deref().filter(|s| !s.trim().is_empty());
        let xmp_bytes = xmp_data.map(|s| s.as_bytes());

        let mut pos = 0usize;
        let reserve_len = data.len()
            + xmp_bytes.map(|b| 64 + b.len()).unwrap_or(0);
        let mut output_buf = Vec::with_capacity(reserve_len);
        let mut found_xmp_uuid = false;

        while pos + 8 <= data.len() {
            let (box_size, header_size) = Self::read_box_header(&data[pos..])?;

            if box_size == 0 || pos + box_size as usize > data.len() {
                output_buf.extend_from_slice(&data[pos..]);
                break;
            }

            let box_type = [data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]];
            let payload_start = pos + header_size as usize;

            if box_type == *b"uuid" && payload_start + 16 <= data.len() {
                let uuid = &data[payload_start..payload_start + 16];
                if uuid == XMP_UUID {
                    found_xmp_uuid = true;
                    if let Some(xb) = xmp_bytes {
                        let new_uuid = Self::build_uuid_xmp_box(xb);
                        output_buf.extend_from_slice(&new_uuid);
                    }
                    pos += box_size as usize;
                    continue;
                }
            }

            output_buf.extend_from_slice(&data[pos..pos + box_size as usize]);
            pos += box_size as usize;
        }

        if !found_xmp_uuid {
            if let Some(xb) = xmp_bytes {
                let new_uuid = Self::build_uuid_xmp_box(xb);
                let ftyp_end = if data.len() >= 8 {
                    let sz = u32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;
                    if sz >= 8 && sz <= data.len() {
                        sz
                    } else {
                        8
                    }
                } else {
                    8
                };
                let mut final_buf = Vec::with_capacity(output_buf.len() + new_uuid.len());
                final_buf.extend_from_slice(&output_buf[..ftyp_end]);
                final_buf.extend_from_slice(&new_uuid);
                final_buf.extend_from_slice(&output_buf[ftyp_end..]);
                output.write_all(&final_buf)?;
                return Ok(());
            }
        }

        output.write_all(&output_buf)?;
        Ok(())
    }

    fn read_box_header(data: &[u8]) -> Result<(u64, u8)> {
        if data.len() < 8 {
            return Err(Error::InvalidStructure("Truncated box".into()));
        }
        let size32 = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        let (size, header_size) = if size32 == 1 {
            if data.len() < 16 {
                return Err(Error::InvalidStructure("Truncated extended size".into()));
            }
            let ext = u64::from_be_bytes([
                data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15],
            ]);
            (ext, 16u8)
        } else if size32 == 0 {
            return Err(Error::InvalidStructure("Box size 0 not supported for write".into()));
        } else {
            (size32 as u64, 8u8)
        };
        Ok((size, header_size))
    }

    fn build_uuid_xmp_box(xmp_bytes: &[u8]) -> Vec<u8> {
        let size = 8 + 16 + xmp_bytes.len() as u64;
        let mut buf = Vec::with_capacity(size as usize);
        if size <= u32::MAX as u64 {
            buf.extend_from_slice(&(size as u32).to_be_bytes());
        } else {
            buf.extend_from_slice(&1u32.to_be_bytes());
            buf.extend_from_slice(&size.to_be_bytes());
        }
        buf.extend_from_slice(b"uuid");
        buf.extend_from_slice(&XMP_UUID);
        buf.extend_from_slice(xmp_bytes);
        buf
    }
}
