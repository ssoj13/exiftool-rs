//! Format registry for auto-detection.
//!
//! Registered parsers:
//! - JpegParser - JPEG with EXIF/XMP
//! - PngParser - PNG with eXIf/tEXt/iTXt chunks  
//! - GifParser - GIF87a/GIF89a
//! - BmpParser - Windows Bitmap
//! - IcoParser - Windows Icon/Cursor
//! - TiffParser - TIFF/DNG
//! - Cr2Parser - Canon CR2 (TIFF-based)
//! - Cr3Parser - Canon CR3 (ISOBMFF)
//! - RafParser - Fujifilm RAF
//! - NefParser - Nikon NEF (TIFF-based)
//! - ArwParser - Sony ARW (TIFF-based)
//! - OrfParser - Olympus ORF (TIFF-based)
//! - Rw2Parser - Panasonic RW2 (TIFF-based)
//! - PefParser - Pentax PEF (TIFF-based)
//! - WebpParser - WebP (RIFF container)
//! - ExrParser - OpenEXR
//! - HdrParser - Radiance HDR/RGBE
//! - HeicParser - HEIC/HEIF/AVIF
//! - Mp4Parser - MP4/MOV/M4A/3GP
//! - Id3Parser - MP3 (ID3v1/v2)
//! - FlacParser - FLAC audio
//! - SvgParser - SVG (XML-based vector graphics)

use crate::{
    ArwParser, BmpParser, Cr2Parser, Cr3Parser, ExrParser, FormatParser, GifParser, HdrParser, 
    FlacParser, HeicParser, IcoParser, Id3Parser, JpegParser, Mp4Parser, NefParser, OrfParser, PefParser, PngParser, 
    RafParser, Result, Rw2Parser, SvgParser, TiffParser, WebpParser,
};

/// Registry of format parsers with auto-detection.
pub struct FormatRegistry {
    parsers: Vec<Box<dyn FormatParser>>,
}

impl Default for FormatRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatRegistry {
    /// Create registry with all built-in parsers.
    pub fn new() -> Self {
        let mut r = Self { parsers: vec![] };
        // Order matters: more specific formats first!
        // Formats with unique magic bytes
        r.register(Box::new(JpegParser));
        r.register(Box::new(PngParser));
        r.register(Box::new(GifParser));          // GIF87a/GIF89a magic
        r.register(Box::new(BmpParser));          // BM magic
        r.register(Box::new(IcoParser));          // ICO/CUR magic
        r.register(Box::new(WebpParser::new()));  // RIFF/WEBP magic
        r.register(Box::new(RafParser));          // FUJIFILM magic
        r.register(Box::new(ExrParser));
        r.register(Box::new(HdrParser));
        r.register(Box::new(Cr3Parser));          // ISOBMFF with ftypcrx
        r.register(Box::new(HeicParser));         // ISOBMFF with ftyp (heic/avif)
        r.register(Box::new(Mp4Parser));           // ISOBMFF with ftyp (mp4/mov)
        r.register(Box::new(Id3Parser));           // MP3 with ID3 tags
        r.register(Box::new(FlacParser));          // FLAC audio
        r.register(Box::new(SvgParser));           // SVG vector graphics
        
        // TIFF-based formats (detected by extension, not magic)
        r.register(Box::new(Cr2Parser::new()));   // Canon CR2
        r.register(Box::new(NefParser::new()));   // Nikon NEF
        r.register(Box::new(ArwParser::new()));   // Sony ARW
        r.register(Box::new(OrfParser::new()));   // Olympus ORF (has special IIRO magic)
        r.register(Box::new(Rw2Parser::new()));   // Panasonic RW2 (has 0x55 magic)
        r.register(Box::new(PefParser::new()));   // Pentax PEF
        r.register(Box::new(TiffParser::default())); // Generic TIFF (last!)
        r
    }

    /// Register a format parser.
    pub fn register(&mut self, parser: Box<dyn FormatParser>) {
        self.parsers.push(parser);
    }

    /// Detect format from magic bytes (first 16 bytes recommended).
    pub fn detect(&self, header: &[u8]) -> Option<&dyn FormatParser> {
        self.parsers
            .iter()
            .find(|p| p.can_parse(header))
            .map(|p| p.as_ref())
    }

    /// Get parser by format name.
    pub fn get(&self, name: &str) -> Option<&dyn FormatParser> {
        self.parsers
            .iter()
            .find(|p| p.format_name().eq_ignore_ascii_case(name))
            .map(|p| p.as_ref())
    }

    /// Get parser by file extension.
    pub fn by_extension(&self, ext: &str) -> Option<&dyn FormatParser> {
        let ext_lower = ext.to_lowercase();
        self.parsers
            .iter()
            .find(|p| p.extensions().iter().any(|e| e.eq_ignore_ascii_case(&ext_lower)))
            .map(|p| p.as_ref())
    }

    /// Parse file with auto-detection.
    pub fn parse<R: std::io::Read + std::io::Seek>(
        &self,
        reader: &mut R,
    ) -> Result<crate::Metadata> {
        let mut header = [0u8; 16];
        reader.read_exact(&mut header)?;
        reader.seek(std::io::SeekFrom::Start(0))?;

        let parser = self
            .detect(&header)
            .ok_or(crate::Error::UnsupportedFormat)?;

        parser.parse(reader)
    }
}
