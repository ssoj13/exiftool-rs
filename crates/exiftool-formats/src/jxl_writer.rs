//! JPEG XL format writer.
//!
//! Handles JXL container format (ISOBMFF): replaces/inserts Exif box.
//! Codestream-only files are passed through unchanged.

use crate::{Error, Metadata, ReadSeek, Result};
use std::io::{SeekFrom, Write};

const JXL_CONTAINER_MAGIC: &[u8] = &[
    0x00, 0x00, 0x00, 0x0C, 0x4A, 0x58, 0x4C, 0x20,
    0x0D, 0x0A, 0x87, 0x0A,
];

/// JXL format writer.
pub struct JxlWriter;

impl JxlWriter {
    /// Write JXL container with updated Exif.
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        input.seek(SeekFrom::Start(0))?;
        let data = crate::utils::read_with_limit(input)?;

        if data.len() < 12 {
            return Err(Error::InvalidStructure("JXL file too small".into()));
        }

        // Codestream-only: pass through
        if &data[0..2] == &[0xFF, 0x0A] {
            output.write_all(&data)?;
            return Ok(());
        }

        if &data[0..12] != JXL_CONTAINER_MAGIC {
            return Err(Error::InvalidStructure("Invalid JXL container magic".into()));
        }

        let exif_payload: Vec<u8> = if let Ok(eb) = crate::utils::build_exif_bytes(metadata) {
            if eb.is_empty() {
                vec![]
            } else {
                [0u8; 4].iter().chain(eb.iter()).cloned().collect()
            }
        } else {
            vec![]
        };

        let mut pos = 12;
        let file_size = data.len();
        let mut out = Vec::new();
        out.extend_from_slice(&data[0..12]);
        let mut found_exif = false;
        let exif_insert_pos = out.len();

        while pos + 8 <= file_size {
            let box_start = pos;
            let box_size = u32::from_be_bytes(data[pos..pos + 4].try_into().unwrap()) as usize;
            let box_type = &data[pos + 4..pos + 8];

            let (_, total_len) = if box_size == 1 {
                if pos + 16 > file_size {
                    break;
                }
                let ext = u64::from_be_bytes(data[pos + 8..pos + 16].try_into().unwrap()) as usize;
                (16, ext)
            } else if box_size == 0 {
                (8, (file_size - box_start).min(16 * 1024 * 1024))
            } else {
                (8, box_size)
            };

            if box_type == b"Exif" {
                found_exif = true;
                if !exif_payload.is_empty() {
                    write_box(&mut out, b"Exif", &exif_payload);
                }
            } else {
                let copy_end = (box_start + total_len).min(file_size);
                out.extend_from_slice(&data[box_start..copy_end]);
            }
            pos = box_start + total_len;

            if pos >= file_size {
                break;
            }
        }

        if !found_exif && !exif_payload.is_empty() {
            let mut new_out = Vec::new();
            new_out.extend_from_slice(&out[0..exif_insert_pos]);
            write_box(&mut new_out, b"Exif", &exif_payload);
            new_out.extend_from_slice(&out[exif_insert_pos..]);
            out = new_out;
        }

        output.write_all(&out)?;
        Ok(())
    }
}

fn write_box(buf: &mut Vec<u8>, box_type: &[u8; 4], payload: &[u8]) {
    let size = 8 + payload.len() as u32;
    buf.extend_from_slice(&size.to_be_bytes());
    buf.extend_from_slice(box_type);
    buf.extend_from_slice(payload);
}
