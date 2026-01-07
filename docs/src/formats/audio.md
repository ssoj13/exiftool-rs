# Audio Formats

Audio files contain metadata in various tag formats: ID3, Vorbis Comments, 
APE tags, etc.

## MP3

| Feature | Support |
|---------|---------|
| ID3v1 | ✓ |
| ID3v2 | ✓ |
| APE tags | ✓ |

**Tags:** Title, Artist, Album, Year, Genre, Track, etc.

**Extensions:** `.mp3`

## FLAC

Free Lossless Audio Codec with Vorbis Comments.

| Feature | Support |
|---------|---------|
| Vorbis Comments | ✓ |
| STREAMINFO | ✓ |
| Pictures | Detected |

**Extensions:** `.flac`

## WAV

Microsoft WAVE audio.

| Feature | Support |
|---------|---------|
| INFO chunk | ✓ |
| Format info | ✓ |

**Extensions:** `.wav`

## AAC

Advanced Audio Coding.

| Feature | Support |
|---------|---------|
| ADTS header | ✓ |

**Extensions:** `.aac`

## OGG

Ogg container with Vorbis/Opus.

| Feature | Support |
|---------|---------|
| Vorbis Comments | ✓ |
| Opus tags | ✓ |

**Extensions:** `.ogg`, `.opus`

## AIFF

Apple audio format.

| Feature | Support |
|---------|---------|
| COMM chunk | ✓ |
| NAME/AUTH | ✓ |
| Annotations | ✓ |

**Extensions:** `.aiff`, `.aif`

## Other Audio Formats

| Format | Extension | Notes |
|--------|-----------|-------|
| CAF | .caf | Apple Core Audio |
| AU | .au | Sun/NeXT audio |
| APE | .ape | Monkey's Audio |
| WavPack | .wv | Hybrid lossy/lossless |
| TAK | .tak | Tom's Audio Kompressor |
| DSF/DFF | .dsf, .dff | DSD audio |
| MIDI | .mid | Sequence info |
| ASF/WMA | .wma, .asf | Windows Media |
| RealMedia | .rm, .ra | RealAudio |
| Audible | .aa, .aax | Audiobook format |

## Common Audio Metadata

```rust
// Duration
metadata.exif.get_str("Duration")  // "3:45" or "225.5"

// Sample rate
metadata.exif.get_u32("AudioSampleRate")  // 44100

// Channels
metadata.exif.get_u32("AudioChannels")  // 2

// Bit rate
metadata.exif.get_str("AudioBitrate")  // "320 kbps"

// ID3 tags
metadata.exif.get_str("Title")
metadata.exif.get_str("Artist")
metadata.exif.get_str("Album")
metadata.exif.get_str("Year")
metadata.exif.get_str("Genre")
```
