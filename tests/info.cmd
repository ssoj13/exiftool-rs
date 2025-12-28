@echo off
REM Examples:
REM   info.cmd photo.jpg              - full JSON report
REM   info.cmd -g DateTimeOriginal *  - get capture date only

if "%1"=="-g" (
    ..\target\release\exif.exe %*
) else (
    ..\target\release\exif.exe -f json -o report.json %*
)