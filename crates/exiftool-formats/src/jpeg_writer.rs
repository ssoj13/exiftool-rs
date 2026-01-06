//! JPEG writer - replaces EXIF/XMP segments in JPEG files.
//!
//! Strategy: Copy all segments, replacing APP1 (EXIF) with new data.
//! Image data (after SOS) is copied verbatim - no recompression.

use crate::{Error, Metadata, ReadSeek, Result};
use std::io::Write;

/// JPEG segment for writing.
#[derive(Debug, Clone)]
pub struct JpegSegment {
    pub marker: u8,
    pub data: Vec<u8>,
}

/// JPEG writer that preserves image data while replacing metadata.
pub struct JpegWriter;

impl JpegWriter {
    /// Write JPEG with new EXIF data.
    /// 
    /// - `input`: source JPEG reader
    /// - `output`: destination writer  
    /// - `exif_data`: new EXIF TIFF bytes (without "Exif\0\0" header)
    /// - `xmp_data`: optional new XMP string
    pub fn write<R, W>(
        input: &mut R,
        output: &mut W,
        exif_data: Option<&[u8]>,
        xmp_data: Option<&str>,
    ) -> Result<()>
    where
        R: ReadSeek,
        W: std::io::Write,
    {
        // Read entire input (with size limit)
        let data = crate::utils::read_with_limit(input)?;
        
        if data.len() < 2 || data[0] != 0xFF || data[1] != 0xD8 {
            return Err(Error::InvalidStructure("not a JPEG file".into()));
        }
        
        // Parse segments
        let segments = Self::parse_segments(&data)?;
        
        // Write SOI
        output.write_all(&[0xFF, 0xD8])?;
        
        let mut wrote_exif = false;
        let mut wrote_xmp = false;
        
        for seg in &segments {
            match seg.marker {
                0xE1 => {
                    // APP1 - check if EXIF or XMP
                    if seg.data.starts_with(b"Exif\x00\x00") {
                        // Replace EXIF
                        if let Some(exif) = exif_data {
                            if !wrote_exif {
                                Self::write_exif_segment(output, exif)?;
                                wrote_exif = true;
                            }
                        }
                        // Skip original EXIF (don't copy)
                    } else if seg.data.starts_with(b"http://ns.adobe.com/xap/1.0/\x00") {
                        // Replace XMP
                        if let Some(xmp) = xmp_data {
                            if !wrote_xmp {
                                Self::write_xmp_segment(output, xmp)?;
                                wrote_xmp = true;
                            }
                        }
                        // Skip original XMP
                    } else {
                        // Other APP1 - copy as-is
                        Self::write_segment(output, seg.marker, &seg.data)?;
                    }
                }
                0xDA => {
                    // SOS - write any pending new segments before this
                    if let Some(exif) = exif_data {
                        if !wrote_exif {
                            Self::write_exif_segment(output, exif)?;
                            wrote_exif = true;
                        }
                    }
                    if let Some(xmp) = xmp_data {
                        if !wrote_xmp {
                            Self::write_xmp_segment(output, xmp)?;
                            wrote_xmp = true;
                        }
                    }
                    // Write SOS and rest of file (image data)
                    Self::write_segment(output, seg.marker, &seg.data)?;
                }
                _ => {
                    // Copy other segments as-is
                    Self::write_segment(output, seg.marker, &seg.data)?;
                }
            }
        }
        
        Ok(())
    }

    /// Write JPEG with updated metadata (convenience method).
    ///
    /// Extracts EXIF and XMP from Metadata struct and writes both.
    pub fn write_metadata<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        // Build EXIF bytes from metadata
        let exif_bytes = crate::utils::build_exif_bytes(metadata)?;
        let exif_data = if exif_bytes.is_empty() {
            None
        } else {
            Some(exif_bytes.as_slice())
        };

        // Get XMP string from metadata
        let xmp_data = metadata.xmp.as_deref();

        Self::write(input, output, exif_data, xmp_data)
    }
    
    /// Parse JPEG into segments.
    fn parse_segments(data: &[u8]) -> Result<Vec<JpegSegment>> {
        let mut segments = Vec::new();
        let mut pos = 2; // Skip SOI
        
        while pos < data.len() {
            if data[pos] != 0xFF {
                return Err(Error::InvalidStructure("invalid JPEG marker".into()));
            }
            
            // Skip padding FF bytes
            while pos < data.len() && data[pos] == 0xFF {
                pos += 1;
            }
            
            if pos >= data.len() {
                break;
            }
            
            let marker = data[pos];
            pos += 1;
            
            // EOI - end
            if marker == 0xD9 {
                break;
            }
            
            // Standalone markers (RST, TEM)
            if (0xD0..=0xD7).contains(&marker) || marker == 0x01 {
                continue;
            }
            
            // SOS - rest of file is image data
            if marker == 0xDA {
                // Read length
                if pos + 2 > data.len() {
                    break;
                }
                let len = u16::from_be_bytes([data[pos], data[pos + 1]]) as usize;
                
                // SOS data includes everything until EOI
                let sos_header = &data[pos..pos + len];
                let image_start = pos + len;
                
                // Find EOI
                let mut image_end = data.len();
                for i in (image_start..data.len() - 1).rev() {
                    if data[i] == 0xFF && data[i + 1] == 0xD9 {
                        image_end = i + 2;
                        break;
                    }
                }
                
                // Combine SOS header + image data + EOI
                let mut sos_data = sos_header.to_vec();
                sos_data.extend_from_slice(&data[image_start..image_end]);
                
                segments.push(JpegSegment {
                    marker,
                    data: sos_data,
                });
                break;
            }
            
            // Regular segment with length
            if pos + 2 > data.len() {
                break;
            }
            let len = u16::from_be_bytes([data[pos], data[pos + 1]]) as usize;
            
            if len < 2 || pos + len > data.len() {
                break;
            }
            
            segments.push(JpegSegment {
                marker,
                data: data[pos..pos + len].to_vec(),
            });
            
            pos += len;
        }
        
        Ok(segments)
    }
    
    /// Write a JPEG segment.
    fn write_segment<W: Write>(output: &mut W, marker: u8, data: &[u8]) -> Result<()> {
        output.write_all(&[0xFF, marker])?;
        output.write_all(data)?;
        Ok(())
    }
    
    /// Write EXIF APP1 segment.
    fn write_exif_segment<W: Write>(output: &mut W, exif: &[u8]) -> Result<()> {
        // APP1 marker
        output.write_all(&[0xFF, 0xE1])?;
        
        // Length = 2 (length field) + 6 (Exif\0\0) + exif data
        let len = 2 + 6 + exif.len();
        output.write_all(&(len as u16).to_be_bytes())?;
        
        // EXIF header
        output.write_all(b"Exif\x00\x00")?;
        
        // TIFF data
        output.write_all(exif)?;
        
        Ok(())
    }
    
    /// Write XMP APP1 segment.
    fn write_xmp_segment<W: Write>(output: &mut W, xmp: &str) -> Result<()> {
        let header = b"http://ns.adobe.com/xap/1.0/\x00";
        let xmp_bytes = xmp.as_bytes();
        
        // APP1 marker
        output.write_all(&[0xFF, 0xE1])?;
        
        // Length = 2 + header + xmp
        let len = 2 + header.len() + xmp_bytes.len();
        output.write_all(&(len as u16).to_be_bytes())?;
        
        // XMP header and data
        output.write_all(header)?;
        output.write_all(xmp_bytes)?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use exiftool_attrs::AttrValue;
    use std::io::Cursor;
    
    fn make_minimal_jpeg() -> Vec<u8> {
        vec![
            0xFF, 0xD8, // SOI
            0xFF, 0xE0, 0x00, 0x10, // APP0 JFIF header
            b'J', b'F', b'I', b'F', 0x00, 0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00,
            0xFF, 0xDA, 0x00, 0x02, // SOS with minimal 2-byte length (length field only)
            0xFF, 0xD9, // EOI
        ]
    }
    
    #[test]
    fn write_preserves_image() {
        let input = make_minimal_jpeg();
        let mut cursor = Cursor::new(&input);
        let mut output = Vec::new();
        
        JpegWriter::write(&mut cursor, &mut output, None, None).unwrap();
        assert_eq!(&output[0..2], &[0xFF, 0xD8]);
    }

    #[test]
    fn write_xmp_to_jpeg() {
        let input = make_minimal_jpeg();
        let xmp = r#"<?xml version="1.0"?><x:xmpmeta xmlns:x="adobe:ns:meta/"><rdf:RDF/></x:xmpmeta>"#;
        
        let mut cursor = Cursor::new(&input);
        let mut output = Vec::new();
        
        JpegWriter::write(&mut cursor, &mut output, None, Some(xmp)).unwrap();
        
        // Find XMP APP1 segment
        let xmp_header = b"http://ns.adobe.com/xap/1.0/\x00";
        let found = output
            .windows(xmp_header.len())
            .any(|w| w == xmp_header);
        
        assert!(found, "XMP APP1 segment not found");
        
        // Verify XMP content is in output
        let output_str = String::from_utf8_lossy(&output);
        assert!(output_str.contains("xmpmeta"), "XMP content not found");
    }

    #[test]
    fn write_metadata_with_xmp() {
        let input = make_minimal_jpeg();
        
        let mut metadata = Metadata::new("JPEG");
        metadata.exif.set("Make", AttrValue::Str("TestCam".into()));
        metadata.xmp = Some(r#"<?xml version="1.0"?><x:xmpmeta xmlns:x="adobe:ns:meta/"><rdf:RDF/></x:xmpmeta>"#.to_string());
        
        let mut cursor = Cursor::new(&input);
        let mut output = Vec::new();
        
        JpegWriter::write_metadata(&mut cursor, &mut output, &metadata).unwrap();
        
        // Check EXIF is present
        let exif_header = b"Exif\x00\x00";
        let has_exif = output.windows(6).any(|w| w == exif_header);
        assert!(has_exif, "EXIF not found");
        
        // Check XMP is present
        let xmp_header = b"http://ns.adobe.com/xap/1.0/\x00";
        let has_xmp = output.windows(xmp_header.len()).any(|w| w == xmp_header);
        assert!(has_xmp, "XMP not found");
    }
}
