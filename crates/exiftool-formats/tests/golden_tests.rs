//! Golden file integration tests.
//!
//! Compares parser output against saved expected JSON files.
//!
//! # Usage
//!
//! ```bash
//! # Run tests normally
//! cargo test -p exiftool-formats --test golden_tests
//!
//! # Update golden files when parser changes
//! UPDATE_GOLDEN=1 cargo test -p exiftool-formats --test golden_tests
//! ```
//!
//! # Adding test images
//!
//! 1. Add image file to `tests/testdata/`
//! 2. Run `UPDATE_GOLDEN=1 cargo test` to generate golden file
//! 3. Review the generated JSON in `tests/golden/expected/`
//! 4. Commit both the image and the golden file

mod golden;

pub use golden::*;
