//! XMP (Extensible Metadata Platform) parser.
//!
//! XMP is Adobe's XML-based metadata format using RDF (Resource Description Framework).
//!
//! # Supported Namespaces
//!
//! - `dc:` - Dublin Core (creator, title, description)
//! - `xmp:` - XMP basic (rating, create date)
//! - `exif:` - EXIF properties embedded in XMP
//! - `tiff:` - TIFF properties embedded in XMP
//! - `photoshop:` - Photoshop-specific metadata
//!
//! # Example
//!
//! ```no_run
//! use exiftool_xmp::XmpParser;
//!
//! let xmp_data = std::fs::read_to_string("photo.xmp").unwrap();
//! let attrs = XmpParser::parse(&xmp_data).unwrap();
//!
//! // Access parsed XMP attributes
//! if let Some(rating) = attrs.get_str("XMP:Rating") {
//!     println!("Rating: {}", rating);
//! }
//! ```

mod error;
mod parser;
mod sidecar;
mod writer;

pub use error::{Error, Result};
pub use parser::XmpParser;
pub use sidecar::XmpSidecar;
pub use writer::XmpWriter;

/// XMP namespace URIs.
pub mod ns {
    pub const DC: &str = "http://purl.org/dc/elements/1.1/";
    pub const XMP: &str = "http://ns.adobe.com/xap/1.0/";
    pub const XMP_RIGHTS: &str = "http://ns.adobe.com/xap/1.0/rights/";
    pub const EXIF: &str = "http://ns.adobe.com/exif/1.0/";
    pub const TIFF: &str = "http://ns.adobe.com/tiff/1.0/";
    pub const PHOTOSHOP: &str = "http://ns.adobe.com/photoshop/1.0/";
    pub const IPTC: &str = "http://iptc.org/std/Iptc4xmpCore/1.0/xmlns/";
    pub const RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
}
