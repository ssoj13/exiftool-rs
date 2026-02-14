//! WAV format writer.
//!
//! Writes/updates RIFF LIST INFO chunk from metadata.
//! Preserves fmt, data, and other chunks.

use crate::{Error, Metadata, ReadSeek, Result};
use std::io::{SeekFrom, Write};

/// WAV format writer.
pub struct WavWriter;

impl WavWriter {
    /// Write WAV with updated LIST INFO chunk.
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        input.seek(SeekFrom::Start(0))?;
        let data = crate::utils::read_with_limit(input)?;

        if data.len() < 12 {
            return Err(Error::InvalidStructure("WAV file too small".into()));
        }
        if &data[0..4] != b"RIFF" || &data[8..12] != b"WAVE" {
            return Err(Error::InvalidStructure("Invalid WAV magic".into()));
        }

        let riff_size = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
        let list_info = build_list_info(metadata)?;

        let mut pos = 12;
        let mut found_list_info = false;
        let mut new_chunks: Vec<Chunk> = Vec::new();
        let mut insert_info_after: Option<usize> = None;

        while pos + 8 <= data.len() && pos < 12 + riff_size {
            let chunk_id: [u8; 4] = data[pos..pos + 4].try_into().unwrap();
            let chunk_size = u32::from_le_bytes(data[pos + 4..pos + 8].try_into().unwrap()) as usize;
            pos += 8;

            if chunk_id == *b"LIST" && pos + 4 <= data.len() {
                let form_type = &data[pos..pos + chunk_size];
                if form_type.len() >= 4 && &form_type[0..4] == b"INFO" {
                    found_list_info = true;
                    new_chunks.push(Chunk {
                        id: *b"LIST",
                        data: list_info.clone(),
                    });
                } else {
                    new_chunks.push(Chunk {
                        id: *b"LIST",
                        data: data[pos..pos + chunk_size].to_vec(),
                    });
                }
                pos += chunk_size;
            } else {
                if chunk_id == *b"fmt " {
                    insert_info_after = Some(new_chunks.len() + 1);
                }
                if pos + chunk_size <= data.len() {
                    new_chunks.push(Chunk {
                        id: chunk_id,
                        data: data[pos..pos + chunk_size].to_vec(),
                    });
                }
                pos += chunk_size;
            }

            if chunk_size % 2 == 1 && pos < data.len() {
                pos += 1;
            }
        }

        if !found_list_info {
            let idx = insert_info_after.unwrap_or(new_chunks.len());
            new_chunks.insert(idx.min(new_chunks.len()), Chunk {
                id: *b"LIST",
                data: list_info,
            });
        }

        // Build output
        let mut body = Vec::new();
        for c in &new_chunks {
            body.extend_from_slice(&c.id);
            body.extend_from_slice(&(c.data.len() as u32).to_le_bytes());
            body.extend_from_slice(&c.data);
            if c.data.len() % 2 == 1 {
                body.push(0);
            }
        }

        output.write_all(b"RIFF")?;
        output.write_all(&((4 + body.len()) as u32).to_le_bytes())?; // WAVE + body
        output.write_all(b"WAVE")?;
        output.write_all(&body)?;

        Ok(())
    }
}

struct Chunk {
    id: [u8; 4],
    data: Vec<u8>,
}

fn build_list_info(metadata: &Metadata) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    out.extend_from_slice(b"INFO");

    let mappings = [
        ("Title", b"INAM"),
        ("Artist", b"IART"),
        ("Comment", b"ICMT"),
        ("Copyright", b"ICOP"),
        ("Software", b"ISFT"),
        ("Genre", b"IGNR"),
    ];

    for (key, tag) in mappings {
        let val = metadata.exif.get_str(key).or_else(|| metadata.exif.get_str(&format!("RIFF:{}", key)));
        if let Some(val) = val {
            if !val.is_empty() {
                let bytes = format!("{}\0", val).into_bytes();
                out.extend_from_slice(tag);
                out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
                out.extend_from_slice(&bytes);
                if bytes.len() % 2 == 1 {
                    out.push(0);
                }
            }
        }
    }

    Ok(out)
}
