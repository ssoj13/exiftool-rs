# Test data

Most golden fixtures in this directory are byte-identical copies of files from ExifTool's own test suite (`t/images` in ExifTool 13.59) and are distributed with ExifTool. Any file-specific copyright remains with the respective rights holders. `seven.7z` and `encoded.7z` are small synthetic archives created for this project (they are not part of ExifTool's test suite) and are covered by the project license. See [NOTICE.md](../../../../NOTICE.md) for ExifTool attribution. The A100 trailer-preservation case uses a minimal synthetic TIFF fixture generated in test code and does not redistribute a camera RAW sample.

Regenerate expected JSON:

```
set UPDATE_GOLDEN=1
cargo test -p exiftool-formats --test golden
```
