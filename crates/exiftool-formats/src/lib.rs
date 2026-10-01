//! File format parsers and writers for image/video/audio metadata.
//!
//! # Why this crate exists
//!
//! Files (JPEG, TIFF, PNG, CR2, HEIC, …) embed metadata in format-specific locations.
//! This crate provides format detection, parsing, and writing so callers don't need
//! to know the format in advance.
//!
//! # What it provides
//!
//! - **[`FormatRegistry`]** — auto-detect format from magic bytes, parse metadata
//! - **[`default_parsers`]** / **[`parse_with`]** — list of built-in parsers; add/remove in `parsers` module
//! - **Writers** — `JpegWriter`, `TiffWriter`, `PngWriter`, etc. for metadata updates
//! - **[`parse_tiff_exif`]** — shared TIFF/EXIF logic used by JPEG, PNG, WebP, HEIC, …
//!
//! # Where used
//!
//! - **exiftool-cli** — read/write metadata from command line
//! - **exiftool-py** — Python bindings
//! - **Direct** — `FormatRegistry::new().parse(&mut file)?`
//!
//! # Supported Formats
//!
//! | Format | Extension | Parser | Writer |
//! |--------|-----------|--------|--------|
//! | JPEG | .jpg, .jpeg | [`JpegParser`] | [`JpegWriter`] |
//! | TIFF | .tiff, .tif, .dng | [`TiffParser`] | [`TiffWriter`] |
//! | PNG | .png | [`PngParser`] | [`PngWriter`] |
//! | HEIC/HEIF | .heic, .heif | [`HeicParser`] | [`HeicWriter`] |
//! | Canon CR2 | .cr2 | [`Cr2Parser`] | [`Cr2Writer`] |
//! | Canon CR3 | .cr3 | [`Cr3Parser`] | [`Cr3Writer`] |
//! | Nikon NEF | .nef | [`NefParser`] | [`NefWriter`] |
//! | Sony ARW | .arw | [`ArwParser`] | [`ArwWriter`] |
//! | Olympus ORF | .orf | [`OrfParser`] | [`OrfWriter`] |
//! | Panasonic RW2 | .rw2 | [`Rw2Parser`] | [`Rw2Writer`] |
//! | Pentax PEF | .pef | [`PefParser`] | [`PefWriter`] |
//! | Fuji RAF | .raf | [`RafParser`] | [`RafWriter`] |
//! | WebP | .webp | [`WebpParser`] | [`WebpWriter`] |
//! | OpenEXR | .exr | [`ExrParser`] | [`ExrWriter`] |
//! | Radiance HDR | .hdr | [`HdrParser`] | [`HdrWriter`] |
//! | GIF | .gif | [`GifParser`] | [`GifWriter`] |
//! | WAV | .wav, .wave | [`WavParser`] | [`WavWriter`] |
//! | JXL | .jxl | [`JxlParser`] | [`JxlWriter`] |
//! | MP4/MOV | .mp4, .mov, .m4a, .3gp | [`Mp4Parser`] | [`Mp4Writer`] |
//! | MP3 | .mp3 | [`Id3Parser`] | [`Id3Writer`] |
//! | FLAC | .flac | [`FlacParser`] | [`FlacWriter`] |
//! | PNM | .ppm, .pgm, .pbm, .pam | [`PnmParser`] | [`PnmWriter`] |
//!
//! # Quick Start
//!
//! ```no_run
//! use exiftool_formats::{FormatRegistry, FormatParser};
//! use std::io::Cursor;
//!
//! // Load file and auto-detect format
//! let data = std::fs::read("photo.jpg").unwrap();
//!
//! let registry = FormatRegistry::new();
//! // Detect format from first 16 bytes
//! if let Some(parser) = registry.detect(&data[..16.min(data.len())]) {
//!     let mut cursor = Cursor::new(&data);
//!     let metadata = parser.parse(&mut cursor).unwrap();
//!     println!("Format: {}", metadata.format);
//!     for (tag, value) in metadata.exif.iter() {
//!         println!("{}: {}", tag, value);
//!     }
//! }
//! ```
//!
//! # Direct Parser Usage
//!
//! ```no_run
//! use exiftool_formats::{JpegParser, FormatParser};
//! use std::io::Cursor;
//!
//! let jpeg_data: Vec<u8> = std::fs::read("photo.jpg").unwrap();
//! let mut cursor = Cursor::new(&jpeg_data);
//!
//! let parser = JpegParser;
//! let metadata = parser.parse(&mut cursor).unwrap();
//! println!("Camera: {:?}", metadata.exif.get("Make"));
//! ```

mod aac;
mod ai;
mod aiff;
mod alac;
mod ape;
mod arw;
mod arw_writer;
mod asf;
mod au;
mod audible;
mod avi;
mod bmp;
mod braw;
mod composite;
mod cr2;
mod cr2_writer;
mod cr3;
mod cr3_writer;
mod crw;
mod dcr;
mod dcr_writer;
mod dicom;
mod dicom_tags;
mod dpx;
mod dsf;
mod eps;
mod erf;
mod erf_writer;
mod error;
mod exr;
mod exr_writer;
mod fff;
mod fff_writer;
mod fits;
mod flac;
mod flac_writer;
mod flv;
mod gif;
mod gif_writer;
mod hdr;
mod hdr_writer;
mod heic;
mod heic_writer;
mod ico;
mod id3;
mod id3_writer;
mod iiq;
mod iiq_writer;
mod iptc;
mod jp2;
mod jpeg;
mod jpeg_writer;
mod jxl;
mod jxl_writer;
mod makernotes;
mod mef;
mod mef_writer;
mod midi;
mod mkv;
mod mos;
mod mos_writer;
mod mp4;
mod mp4_writer;
mod mpeg_ts;
mod mrw;
mod mxf;
mod nef;
mod nef_writer;
mod nrw;
mod ogg;
mod orf;
mod orf_writer;
mod parsers;
mod pcx;
mod pdf;
mod pef;
mod pef_writer;
mod png;
mod png_writer;
mod pnm;
mod pnm_writer;
mod psd;
mod r3d;
mod raf;
mod raf_writer;
mod registry;
mod riff;
mod rm;
mod rw2;
mod rw2_writer;
mod rwl;
mod rwl_writer;
mod seven_z;
mod sgi;
mod srf;
mod srw;
mod srw_writer;
mod svg;
mod tag_lookup;
mod tak;
mod tga;
mod tiff;
mod tiff_family;
mod tiff_rewrite;
mod tiff_writer;
mod traits;
mod utils;
mod wav;
mod wav_writer;
mod webp;
mod webp_writer;
mod wv;
mod x3f;
mod zip;

pub use aac::AacParser;
pub use ai::AiParser;
pub use aiff::AiffParser;
pub use alac::CafParser;
pub use ape::ApeParser;
pub use arw::ArwParser;
pub use arw_writer::ArwWriter;
pub use asf::AsfParser;
pub use au::AuParser;
pub use audible::AudibleParser;
pub use avi::AviParser;
pub use bmp::BmpParser;
pub use braw::BrawParser;
pub use composite::add_composite_tags;
pub use cr2::Cr2Parser;
pub use cr2_writer::Cr2Writer;
pub use cr3::Cr3Parser;
pub use cr3_writer::Cr3Writer;
pub use crw::CrwParser;
pub use dcr::{DcrParser, K25Parser, KdcParser};
pub use dcr_writer::DcrWriter;
pub use dicom::DicomParser;
pub use dpx::DpxParser;
pub use dsf::{DffParser, DsfParser};
pub use eps::EpsParser;
pub use erf::ErfParser;
pub use erf_writer::ErfWriter;
pub use error::{Error, Result};
use exiftool_attrs::AttrValue;
pub use exr::ExrParser;
pub use exr_writer::ExrWriter;
pub use fff::FffParser;
pub use fff_writer::FffWriter;
pub use fits::FitsParser;
pub use flac::FlacParser;
pub use flac_writer::FlacWriter;
pub use flv::FlvParser;
pub use gif::GifParser;
pub use gif_writer::GifWriter;
pub use hdr::HdrParser;
pub use hdr_writer::HdrWriter;
pub use heic::HeicParser;
pub use heic_writer::HeicWriter;
pub use ico::IcoParser;
pub use id3::Id3Parser;
pub use id3_writer::Id3Writer;
pub use iiq::IiqParser;
pub use iiq_writer::IiqWriter;
pub use iptc::{IptcParser, IptcWriter};
pub use jp2::Jp2Parser;
pub use jpeg::JpegParser;
pub use jpeg_writer::JpegWriter;
pub use jxl::JxlParser;
pub use jxl_writer::JxlWriter;
pub use mef::MefParser;
pub use mef_writer::MefWriter;
pub use midi::MidiParser;
pub use mkv::MkvParser;
pub use mos::MosParser;
pub use mos_writer::MosWriter;
pub use mp4::Mp4Parser;
pub use mp4_writer::Mp4Writer;
pub use mpeg_ts::MpegTsParser;
pub use mrw::MrwParser;
pub use mxf::MxfParser;
pub use nef::NefParser;
pub use nef_writer::NefWriter;
pub use nrw::NrwParser;
pub use ogg::OggParser;
pub use orf::OrfParser;
pub use orf_writer::OrfWriter;
pub use parsers::{default_parsers, parse_with};
pub use pcx::PcxParser;
pub use pdf::PdfParser;
pub use pef::PefParser;
pub use pef_writer::PefWriter;
pub use png::PngParser;
pub use png_writer::PngWriter;
pub use pnm::PnmParser;
pub use pnm_writer::PnmWriter;
pub use psd::PsdParser;
pub use r3d::R3dParser;
pub use raf::RafParser;
pub use raf_writer::RafWriter;
pub use registry::FormatRegistry;
pub use rm::RmParser;
pub use rw2::Rw2Parser;
pub use rw2_writer::Rw2Writer;
pub use rwl::RwlParser;
pub use rwl_writer::RwlWriter;
pub use seven_z::SevenZParser;
pub use sgi::SgiParser;
pub use srf::SrfParser;
pub use srw::SrwParser;
pub use srw_writer::SrwWriter;
pub use svg::SvgParser;
pub use tak::TakParser;
pub use tga::TgaParser;
pub use tiff::{TiffConfig, TiffParser};
pub use tiff_writer::TiffWriter;
pub use traits::{read_detect_header, FormatParser, ReadSeek, DETECT_HEADER_LEN};
pub use utils::{
    build_exif_bytes, build_xmp_string, entry_to_attr, ifd_tags, parse_tiff_exif,
    raw_value_to_attr, read_with_limit, ParseTiffExifOptions, MAX_FILE_SIZE,
};
pub use wav::WavParser;
pub use wav_writer::WavWriter;
pub use webp::WebpParser;
pub use webp_writer::WebpWriter;
pub use wv::WvParser;
pub use x3f::X3fParser;
pub use zip::ZipParser;

/// Info about a single page/subfile in multi-page TIFF.
#[derive(Debug, Clone, Default)]
pub struct PageInfo {
    /// Page index (0-based).
    pub index: usize,
    /// Image width in pixels.
    pub width: u32,
    /// Image height in pixels.
    pub height: u32,
    /// Bits per sample.
    pub bits_per_sample: u16,
    /// Compression type.
    pub compression: u16,
    /// Subfile type (0=full-res, 1=reduced-res/thumbnail, 2=multi-page).
    pub subfile_type: u32,
    /// IFD offset in file.
    pub ifd_offset: u64,
}

impl PageInfo {
    /// Check if this is a thumbnail/reduced resolution image.
    pub fn is_thumbnail(&self) -> bool {
        // SubfileType bit 0 = reduced resolution
        self.subfile_type & 1 != 0
    }

    /// Check if this is a page of a multi-page document.
    pub fn is_page(&self) -> bool {
        // SubfileType bit 1 = multi-page
        self.subfile_type & 2 != 0
    }
}

/// Metadata extracted from a file.
#[derive(Debug, Clone)]
#[must_use]
pub struct Metadata {
    /// File format name.
    pub format: &'static str,
    /// Parsed EXIF attributes.
    pub exif: exiftool_attrs::Attrs,
    /// Raw EXIF data offset in file.
    pub exif_offset: Option<usize>,
    /// XMP data (if present).
    pub xmp: Option<String>,
    /// Thumbnail data (if present) - small embedded preview.
    pub thumbnail: Option<Vec<u8>>,
    /// Preview data (if present) - larger embedded JPEG (RAW files).
    pub preview: Option<Vec<u8>>,
    /// ICC color profile data (if present).
    pub icc: Option<Vec<u8>>,
    /// Pages/subfiles info (multi-page TIFF).
    pub pages: Vec<PageInfo>,
}

impl Metadata {
    /// Create new empty metadata.
    pub fn new(format: &'static str) -> Self {
        Self {
            format,
            exif: exiftool_attrs::Attrs::new(),
            exif_offset: None,
            xmp: None,
            thumbnail: None,
            preview: None,
            icc: None,
            pages: Vec::new(),
        }
    }

    /// Get number of pages (0 if not multi-page).
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    /// Check if this is a multi-page file.
    pub fn is_multi_page(&self) -> bool {
        self.pages.len() > 1
    }

    /// Check if this is a camera RAW file.
    ///
    /// Detection methods:
    /// 1. By format name (ARW, CR2, CR3, NEF, ORF, RW2, PEF, RAF)
    /// 2. By Make tag for TIFF-based RAW (catches renamed files)
    pub fn is_camera_raw(&self) -> bool {
        // Known RAW format names
        const RAW_FORMATS: &[&str] = &[
            "ARW", "CR2", "CR3", "NEF", "ORF", "RW2", "PEF", "RAF", "SRW", "RWL", "3FR", "FFF",
            "ERF", "MEF", "DCR", "KDC", "K25", "MOS", "IIQ", "SRF", "SR2",
        ];

        if RAW_FORMATS.contains(&self.format) {
            return true;
        }

        // TIFF-based RAW detection via Make tag
        if self.format == "TIFF" {
            if let Some(make) = self.exif.get_str("Make") {
                let make_lower = make.to_lowercase();
                const RAW_VENDORS: &[&str] = &[
                    "sony",
                    "nikon",
                    "canon",
                    "fuji",
                    "olympus",
                    "panasonic",
                    "pentax",
                    "leica",
                    "ricoh",
                    "hasselblad",
                    "phase one",
                    "samsung",
                ];
                return RAW_VENDORS.iter().any(|v| make_lower.contains(v));
            }
        }

        false
    }

    /// Check if this format supports writing.
    ///
    /// JPEG, PNG, TIFF, DNG, EXR, HDR, WebP, HEIC/AVIF, GIF, PNM, JXL,
    /// TIFF-family RAW, RAF, CR3, MP4/MOV family, WAV, FLAC, MP3.
    pub fn is_writable(&self) -> bool {
        const WRITABLE: &[&str] = &[
            "JPEG", "PNG", "TIFF", "DNG", "EXR", "HDR", "WebP", "HEIC", "HEIF", "AVIF", "PNM",
            "PBM", "PGM", "PPM", "PAM", "GIF", "JXL",
        ];
        // TIFF-family RAW via tiff_rewrite. CR3 via Cr3Writer (CMT + CTBO).
        const RAW_WRITABLE: &[&str] = &[
            "RAF", "NEF", "NRW", "ARW", "SRF", "SR2", "CR2", "ORF", "RW2", "PEF", "SRW", "RWL",
            "3FR", "FFF", "ERF", "MEF", "DCR", "KDC", "K25", "MOS", "IIQ", "CR3",
        ];
        if RAW_WRITABLE.contains(&self.format) {
            return true;
        }
        const MEDIA: &[&str] = &[
            "MP4", "MOV", "M4V", "M4A", "M4B", "M4P", "3GP", "3G2", "F4V", "WAV", "FLAC", "MP3",
        ];
        if MEDIA.contains(&self.format) {
            return true;
        }
        WRITABLE.contains(&self.format) && !self.is_camera_raw()
    }

    /// Get interpreted value for a tag.
    ///
    /// Returns human-readable string for known enum values (e.g., Orientation -> "Rotate 90 CW").
    /// Falls back to raw string value if no interpretation available.
    pub fn get_interpreted(&self, key: &str) -> Option<String> {
        // Try to get numeric value for interpretation
        if let Some(num) = self.exif.get_i32(key) {
            // Strip group prefix for interpretation lookup
            let tag_name = key.split(':').next_back().unwrap_or(key);
            if let Some(interpreted) = exiftool_tags::interp::interpret_value(tag_name, num as i64)
            {
                return Some(interpreted);
            }
        }
        // Fall back to string value
        self.exif.get_str(key).map(|s| s.to_string())
    }

    /// Get display value for a tag with special formatting.
    ///
    /// Applies formatting for:
    /// - ExposureTime -> "1/125 sec"
    /// - FNumber -> "f/2.8"
    /// - FocalLength -> "50 mm"
    /// - GPS coordinates -> "40° 42' 46.08" N"
    pub fn get_display(&self, key: &str) -> Option<String> {
        let tag_name = key.split(':').next_back().unwrap_or(key);

        match tag_name {
            "ExposureTime" => self
                .exif
                .get_f64(key)
                .map(exiftool_tags::interp::format_exposure_time),
            "FNumber" | "ApertureValue" => self
                .exif
                .get_f64(key)
                .map(exiftool_tags::interp::format_fnumber),
            "FocalLength" | "FocalLengthIn35mmFilm" => self
                .exif
                .get_f64(key)
                .map(exiftool_tags::interp::format_focal_length),
            "GPSLatitude" => self
                .exif
                .get_f64(key)
                .map(|v| exiftool_tags::interp::format_gps_coord(v, true)),
            "GPSLongitude" => self
                .exif
                .get_f64(key)
                .map(|v| exiftool_tags::interp::format_gps_coord(v, false)),
            _ => self.get_interpreted(key),
        }
    }

    /// Set file type and MIME type attributes.
    ///
    /// Single source of truth for file type setting used by all format parsers.
    pub fn set_file_type(&mut self, file_type: &str, mime_type: &str) {
        self.exif
            .set("File:FileType", AttrValue::Str(file_type.to_string()));
        self.exif
            .set("File:MIMEType", AttrValue::Str(mime_type.to_string()));
    }
}
