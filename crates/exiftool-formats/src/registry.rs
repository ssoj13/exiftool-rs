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
    AacParser, AiParser, AiffParser, ApeParser, ArwParser, AsfParser, AuParser, AudibleParser, AviParser, BmpParser, BrawParser, CafParser, Cr2Parser, 
    Cr3Parser, CrwParser, DcrParser, DffParser, DpxParser, DsfParser, EpsParser, ErfParser, ExrParser, FffParser, 
    FlvParser, FormatParser, GifParser, HdrParser, FlacParser, HeicParser, IcoParser, Id3Parser, IiqParser, 
    Jp2Parser, JpegParser, JxlParser, K25Parser, KdcParser, MefParser, MkvParser, MosParser, 
    MidiParser, Mp4Parser, MpegTsParser, MrwParser, MxfParser, NefParser, NrwParser, OggParser, OrfParser, PcxParser, PefParser, 
    PngParser, PnmParser, R3dParser, RmParser, RafParser, Result, Rw2Parser, RwlParser, SgiParser, SrfParser, SrwParser, 
    SvgParser, TakParser, TgaParser, TiffParser, WavParser, WebpParser, WvParser, X3fParser,
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
        r.register(Box::new(EpsParser));           // EPS/PostScript
        r.register(Box::new(AiParser));            // Adobe Illustrator
        r.register(Box::new(PnmParser));           // PPM/PGM/PBM/PAM
        r.register(Box::new(JxlParser));           // JPEG XL
        r.register(Box::new(Jp2Parser));           // JPEG 2000
        r.register(Box::new(AviParser));           // AVI video
        r.register(Box::new(WavParser));           // WAV audio
        r.register(Box::new(AiffParser));          // AIFF/AIFC audio
        r.register(Box::new(AuParser));            // Sun AU audio
        r.register(Box::new(OggParser));           // OGG Vorbis/Opus
        r.register(Box::new(ApeParser));           // Monkey's Audio
        r.register(Box::new(WvParser));            // WavPack
        r.register(Box::new(DsfParser));           // DSD Stream File
        r.register(Box::new(DffParser));           // DSDIFF
        r.register(Box::new(CafParser));           // Core Audio Format (ALAC)
        r.register(Box::new(TakParser));           // TAK lossless
        r.register(Box::new(MidiParser));          // MIDI
        r.register(Box::new(AudibleParser));       // Audible AA/AAX
        r.register(Box::new(AacParser));           // AAC ADTS
        r.register(Box::new(AsfParser));           // ASF/WMA/WMV
        r.register(Box::new(MpegTsParser));        // MPEG-TS/M2TS
        r.register(Box::new(DpxParser));           // DPX film scan
        r.register(Box::new(FlvParser));           // Flash Video
        r.register(Box::new(MxfParser));           // MXF broadcast
        r.register(Box::new(R3dParser));           // RED R3D
        r.register(Box::new(BrawParser));          // Blackmagic RAW
        r.register(Box::new(RmParser));            // Real Media
        r.register(Box::new(MkvParser));           // MKV/WebM video
        r.register(Box::new(TgaParser));           // TGA image
        r.register(Box::new(PcxParser));           // PCX image
        r.register(Box::new(SgiParser));           // SGI/RGB image
        r.register(Box::new(CrwParser));           // Canon CRW (CIFF)
        r.register(Box::new(X3fParser));           // Sigma X3F
        r.register(Box::new(MrwParser));           // Minolta MRW
        
        // TIFF-based formats (detected by extension, not magic)
        r.register(Box::new(Cr2Parser::new()));   // Canon CR2
        r.register(Box::new(NefParser::new()));   // Nikon NEF
        r.register(Box::new(ArwParser::new()));   // Sony ARW
        r.register(Box::new(OrfParser::new()));   // Olympus ORF (has special IIRO magic)
        r.register(Box::new(Rw2Parser::new()));   // Panasonic RW2 (has 0x55 magic)
        r.register(Box::new(PefParser::new()));   // Pentax PEF
        r.register(Box::new(NrwParser::new()));   // Nikon NRW (Coolpix)
        r.register(Box::new(SrfParser::new()));   // Sony SRF/SR2
        r.register(Box::new(FffParser::new()));   // Hasselblad 3FR/FFF
        r.register(Box::new(ErfParser::new()));   // Epson ERF
        r.register(Box::new(MefParser::new()));   // Mamiya MEF
        r.register(Box::new(SrwParser::new()));   // Samsung SRW
        r.register(Box::new(RwlParser::new()));   // Leica RWL
        r.register(Box::new(DcrParser::new()));   // Kodak DCR
        r.register(Box::new(KdcParser::new()));   // Kodak KDC
        r.register(Box::new(K25Parser::new()));   // Kodak K25
        r.register(Box::new(MosParser::new()));   // Leaf MOS
        r.register(Box::new(IiqParser::new()));   // Phase One IIQ
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
