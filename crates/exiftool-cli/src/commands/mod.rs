//! Command implementations (delete, validate, import, rename, etc.).
//!
//! Each command module handles one CLI operation.

mod delete;
mod duplicates;
mod extract;
mod html_dump;
mod import;
mod rename;
mod validate;
mod write;

pub use delete::delete_metadata;
pub use duplicates::find_duplicates;
pub use extract::{extract_previews, extract_thumbnails};
pub use html_dump::html_dump;
pub use import::{import_from_csv, import_from_json};
pub use rename::rename_files;
pub use validate::validate_metadata;
pub use write::write_image;
