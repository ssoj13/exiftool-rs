# Supported Formats

exiftool-rs supports 90+ file formats across images, RAW, audio, and video.

## Format Support Matrix

| Category | Formats | Count |
|----------|---------|-------|
| Images | JPEG, PNG, TIFF, WebP, HEIC, GIF, BMP, etc. | 25+ |
| RAW | CR2, CR3, NEF, ARW, ORF, RW2, RAF, DNG, etc. | 30+ |
| Audio | MP3, FLAC, WAV, AAC, OGG, AIFF, etc. | 20+ |
| Video | MP4, MOV, AVI, MKV, etc. | 15+ |

## Read vs Write

Most formats support reading. Writing is available for:

| Format | Read | Write |
|--------|:----:|:-----:|
| JPEG | ✓ | ✓ |
| PNG | ✓ | ✓ |
| TIFF | ✓ | ✓ |
| DNG | ✓ | ✓ |
| WebP | ✓ | ✓ |
| HEIC/HEIF | ✓ | ✓ |
| EXR | ✓ | ✓ |
| HDR | ✓ | ✓ |
| RAW formats | ✓ | ✗ |
| Audio | ✓ | ✗ |
| Video | ✓ | ✗ |

## Auto-Detection

Format is detected from file headers, not extensions:

```rust
let registry = FormatRegistry::new();
let metadata = registry.parse(&mut reader)?;
println!("Detected: {}", metadata.format);
```

This means:
- Renamed files are handled correctly
- Corrupt/truncated files are detected
- No file extension required
