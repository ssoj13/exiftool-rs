//! Central list of format parsers.
//!
//! # Why this module exists
//!
//! Formats are registered in one place so adding/removing support is trivial:
//! edit `default_parsers()` — no need to touch [`FormatRegistry`] or multiple files.
//!
//! # What it does
//!
//! - [`default_parsers()`] — returns the full list of built-in parsers (60+ formats)
//! - [`parse_with()`] — parse a file using a custom parser list (for minimal builds/testing)
//!
//! # How it works
//!
//! 1. Each parser implements [`FormatParser`] (`can_parse`, `parse`, etc.)
//! 2. Order matters: parsers with unique magic bytes (JPEG, PNG, RAF) come first
//! 3. TIFF-based RAW (CR2, NEF, ARW…) come last — they share TIFF header,
//!    so detection falls back to extension/magic variants
//! 4. [`FormatRegistry::new()`] calls `default_parsers()` and builds the registry
//!
//! # Where used
//!
//! - [`FormatRegistry::new()`] in `registry.rs` — builds default registry
//! - [`FormatRegistry::with_parsers()`] — accepts custom list from `default_parsers()` or user-built
//! - CLI (`exiftool-cli`) — uses `FormatRegistry::new()` for auto-detection
//! - Python bindings (`exiftool-py`) — same
//!
//! # Adding/removing formats
//!
//! To add: insert `Box::new(XxxParser)` in `default_parsers()` (place by detection order).
//! To remove: comment out or delete the line.

use crate::FormatParser;
use std::io::{Read, Seek};

// Re-export all parsers for registration
use crate::{
    AacParser, AiParser, AiffParser, ApeParser, ArwParser, AsfParser, AuParser, AudibleParser,
    AviParser, BmpParser, BrawParser, CafParser, Cr2Parser, Cr3Parser, CrwParser, DcrParser,
    DffParser, DpxParser, DsfParser, EpsParser, ErfParser, ExrParser, FffParser, FlacParser,
    FlvParser, GifParser, HdrParser, HeicParser, IcoParser, Id3Parser, IiqParser, Jp2Parser,
    JpegParser, JxlParser, K25Parser, KdcParser, MefParser, MidiParser, MkvParser, MosParser,
    Mp4Parser, MpegTsParser, MrwParser, MxfParser, NefParser, NrwParser, OggParser, OrfParser,
    PcxParser, PdfParser, PefParser, PngParser, PnmParser, PsdParser, R3dParser, RmParser,
    RafParser, Rw2Parser, RwlParser, SgiParser, SrfParser, SrwParser, SvgParser, TakParser,
    TgaParser, TiffParser, WavParser, WebpParser, WvParser, X3fParser,
};

/// Build the default list of format parsers.
///
/// Used by [`crate::FormatRegistry::new()`]. Edit this function to add or remove formats.
/// Parser order: unique magic first (JPEG, PNG, …), TIFF-based last.
#[must_use]
pub fn default_parsers() -> Vec<Box<dyn FormatParser>> {
    vec![
        // === Unique magic bytes (order: most specific first) ===
        Box::new(JpegParser),
        Box::new(PngParser),
        Box::new(GifParser),
        Box::new(BmpParser),
        Box::new(IcoParser),
        Box::new(WebpParser::new()),
        Box::new(RafParser),
        Box::new(ExrParser),
        Box::new(HdrParser),
        Box::new(Cr3Parser),
        Box::new(HeicParser),
        Box::new(Mp4Parser),
        Box::new(Id3Parser),
        Box::new(FlacParser),
        Box::new(SvgParser),
        Box::new(EpsParser),
        Box::new(AiParser),
        Box::new(PnmParser),
        Box::new(JxlParser),
        Box::new(Jp2Parser),
        Box::new(AviParser),
        Box::new(WavParser),
        Box::new(AiffParser),
        Box::new(AuParser),
        Box::new(OggParser),
        Box::new(ApeParser),
        Box::new(WvParser),
        Box::new(DsfParser),
        Box::new(DffParser),
        Box::new(CafParser),
        Box::new(TakParser),
        Box::new(MidiParser),
        Box::new(AudibleParser),
        Box::new(AacParser),
        Box::new(AsfParser),
        Box::new(MpegTsParser),
        Box::new(DpxParser),
        Box::new(FlvParser),
        Box::new(MxfParser),
        Box::new(R3dParser),
        Box::new(BrawParser),
        Box::new(RmParser),
        Box::new(MkvParser),
        Box::new(TgaParser),
        Box::new(PcxParser),
        Box::new(SgiParser),
        Box::new(CrwParser),
        Box::new(X3fParser),
        Box::new(MrwParser),
        Box::new(PsdParser),
        Box::new(PdfParser),
        // === TIFF-based (extension/magic, last to avoid false positives) ===
        Box::new(Cr2Parser::new()),
        Box::new(NefParser::new()),
        Box::new(ArwParser::new()),
        Box::new(OrfParser::new()),
        Box::new(Rw2Parser::new()),
        Box::new(PefParser::new()),
        Box::new(NrwParser::new()),
        Box::new(SrfParser::new()),
        Box::new(FffParser::new()),
        Box::new(ErfParser::new()),
        Box::new(MefParser::new()),
        Box::new(SrwParser::new()),
        Box::new(RwlParser::new()),
        Box::new(DcrParser::new()),
        Box::new(KdcParser::new()),
        Box::new(K25Parser::new()),
        Box::new(MosParser::new()),
        Box::new(IiqParser::new()),
        Box::new(TiffParser::default()),
    ]
}

/// Parse file with a custom parser list.
///
/// Use when you need a minimal set of formats (e.g. only JPEG+PNG) or for testing.
/// Reads 16-byte header, finds first matching parser, then parses.
pub fn parse_with<R: Read + Seek>(
    parsers: &[Box<dyn FormatParser>],
    reader: &mut R,
) -> crate::Result<crate::Metadata> {
    let mut header = [0u8; 16];
    reader.read_exact(&mut header)?;
    reader.seek(std::io::SeekFrom::Start(0))?;

    let parser = parsers
        .iter()
        .find(|p| p.can_parse(&header))
        .ok_or(crate::Error::UnsupportedFormat)?;

    parser.parse(reader)
}
