# RAW Formats

Camera RAW formats are read-only. They contain full sensor data and extensive 
manufacturer-specific metadata.

## Canon

| Format | Extension | Notes |
|--------|-----------|-------|
| CR2 | .cr2 | TIFF-based, older DSLRs |
| CR3 | .cr3 | ISOBMFF-based, modern cameras |
| CRW | .crw | Legacy CIFF format |

## Nikon

| Format | Extension | Notes |
|--------|-----------|-------|
| NEF | .nef | TIFF-based |
| NRW | .nrw | Coolpix RAW |

## Sony

| Format | Extension | Notes |
|--------|-----------|-------|
| ARW | .arw | Alpha RAW |
| SRF | .srf | Legacy format |
| SR2 | .sr2 | Legacy format |

## Fujifilm

| Format | Extension | Notes |
|--------|-----------|-------|
| RAF | .raf | Fuji RAW |

## Olympus

| Format | Extension | Notes |
|--------|-----------|-------|
| ORF | .orf | OM-D/PEN series |

## Panasonic

| Format | Extension | Notes |
|--------|-----------|-------|
| RW2 | .rw2 | Lumix RAW |

## Pentax

| Format | Extension | Notes |
|--------|-----------|-------|
| PEF | .pef | Pentax Electronic File |

## Other RAW Formats

| Format | Extension | Manufacturer |
|--------|-----------|--------------|
| DNG | .dng | Adobe (open standard) |
| DCS | .dcr | Kodak |
| KDC | .kdc | Kodak |
| ERF | .erf | Epson |
| MEF | .mef | Mamiya |
| MOS | .mos | Leaf |
| MRW | .mrw | Minolta |
| RWL | .rwl | Leica |
| SRW | .srw | Samsung |
| X3F | .x3f | Sigma/Foveon |
| 3FR | .3fr | Hasselblad |
| FFF | .fff | Hasselblad |
| IIQ | .iiq | Phase One |

## Cinema RAW

| Format | Extension | Notes |
|--------|-----------|-------|
| BRAW | .braw | Blackmagic RAW |
| R3D | .r3d | RED Digital Cinema |

## RAW Detection

RAW files are detected by format signature and/or Make tag:

```rust
if metadata.is_camera_raw() {
    // Read-only, cannot write
    println!("RAW file from: {}", metadata.exif.get_str("Make").unwrap_or("Unknown"));
}
```

## Embedded Previews

Most RAW files contain embedded JPEG previews:

```rust
// Small thumbnail (usually 160x120)
if let Some(thumb) = &metadata.thumbnail {
    // JPEG data
}

// Full-size preview (many RAW files)
if let Some(preview) = &metadata.preview {
    // Often full resolution JPEG
}
```

## MakerNotes

RAW files contain manufacturer-specific data in MakerNotes IFD. Supported vendors:

- Canon
- Nikon  
- Sony
- Fujifilm
- Olympus
- Panasonic
- Pentax
- Leica
- Hasselblad
- Phase One
- Apple
- Google
- GoPro
- DJI
- Samsung
- Ricoh
- Sigma
- Kodak
- Minolta
- Casio
- Huawei
- Xiaomi
