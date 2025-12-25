//! OpenEXR format writer.
//!
//! EXR metadata is stored in header attributes.
//! Full EXR write requires the `exr` crate's complex API.

use crate::{Metadata, ReadSeek, Result};
use std::io::Write;

/// OpenEXR format writer.
/// 
/// Note: Full EXR read/modify/write requires loading all pixel data.
/// This implementation updates header attributes while preserving image data.
pub struct ExrWriter;

impl ExrWriter {
    /// Write EXR with updated metadata, preserving pixel data.
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        use exr::prelude::*;

        // Read source into buffer (with size limit)
        let source_data = crate::utils::read_with_limit(input)?;

        // Parse and re-serialize with updated attributes
        // The exr crate's read/write API handles this
        let image = read()
            .no_deep_data()
            .largest_resolution_level()
            .all_channels()
            .all_layers()
            .all_attributes()
            .from_buffered(std::io::Cursor::new(&source_data))
            .map_err(|e| crate::Error::InvalidStructure(format!("EXR read: {}", e)))?;

        // Update layer attributes
        let updated_layers: exr::prelude::SmallVec<[_; 2]> = image.layer_data
            .into_iter()
            .map(|layer| {
                let mut attrs = layer.attributes;
                Self::update_attrs(&mut attrs.other, metadata);
                Layer { attributes: attrs, ..layer }
            })
            .collect();

        let updated_image = Image {
            attributes: image.attributes,
            layer_data: updated_layers,
        };

        // Write to buffer
        let mut buffer = Vec::new();
        updated_image.write()
            .to_buffered(std::io::Cursor::new(&mut buffer))
            .map_err(|e| crate::Error::InvalidStructure(format!("EXR write: {}", e)))?;

        output.write_all(&buffer)?;
        Ok(())
    }

    /// Update custom attributes from metadata.
    fn update_attrs(
        attrs: &mut std::collections::HashMap<exr::meta::attribute::Text, exr::meta::attribute::AttributeValue>,
        metadata: &Metadata,
    ) {
        use exr::meta::attribute::{Text, AttributeValue};

        if let Some(v) = metadata.exif.get_str("Software") {
            attrs.insert(Text::from("software"), AttributeValue::Text(Text::from(v)));
        }
        if let Some(v) = metadata.exif.get_str("Artist") {
            attrs.insert(Text::from("owner"), AttributeValue::Text(Text::from(v)));
        }
        if let Some(v) = metadata.exif.get_str("ImageDescription") {
            attrs.insert(Text::from("comments"), AttributeValue::Text(Text::from(v)));
        }
        if let Some(v) = metadata.exif.get_str("DateTime") {
            attrs.insert(Text::from("capDate"), AttributeValue::Text(Text::from(v)));
        }
        if let Some(v) = metadata.exif.get_str("Make") {
            attrs.insert(Text::from("cameraMake"), AttributeValue::Text(Text::from(v)));
        }
        if let Some(v) = metadata.exif.get_str("Model") {
            attrs.insert(Text::from("cameraModel"), AttributeValue::Text(Text::from(v)));
        }
    }
}

#[cfg(test)]
mod tests {
    // EXR tests require sample files - integration tests use actual EXR files
}
