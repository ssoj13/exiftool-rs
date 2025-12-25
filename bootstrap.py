#!/usr/bin/env python3
"""
Bootstrap script for exiftool-rs

Usage:
    python bootstrap.py              # Show help
    python bootstrap.py build        # Build all crates (release)
    python bootstrap.py build --debug # Build all crates (debug)
    python bootstrap.py python       # Build Python wheel (release)
    python bootstrap.py python --debug # Build Python wheel (debug)
    python bootstrap.py python dev   # Install Python module in dev mode
"""

from __future__ import annotations

import argparse
import shutil
import subprocess
import sys
from pathlib import Path

# Colors for terminal output
class Colors:
    CYAN = "\033[96m"
    GREEN = "\033[92m"
    RED = "\033[91m"
    RESET = "\033[0m"

def print_color(msg: str, color: str = "") -> None:
    """Print colored message to terminal."""
    if color and sys.stdout.isatty():
        print(f"{color}{msg}{Colors.RESET}")
    else:
        print(msg)

def run(cmd: list[str], check: bool = True) -> int:
    """Run command and return exit code."""
    result = subprocess.run(cmd)
    if check and result.returncode != 0:
        sys.exit(result.returncode)
    return result.returncode

def check_cargo() -> None:
    """Ensure cargo is available."""
    if not shutil.which("cargo"):
        print_color("Error: Rust/Cargo not found!", Colors.RED)
        print("Install from: https://rustup.rs/")
        sys.exit(1)

def cmd_build(args: argparse.Namespace) -> None:
    """Build all Rust crates."""
    print_color("Building exiftool-rs...", Colors.CYAN)
    print()

    cmd = ["cargo", "build"]
    if not args.debug:
        cmd.append("--release")

    code = run(cmd, check=False)
    if code == 0:
        print()
        print_color("[OK] Build complete!", Colors.GREEN)
    sys.exit(code)

def cmd_python(args: argparse.Namespace) -> None:
    """Build Python module."""
    print_color("Building Python module (exiftool-py)...", Colors.CYAN)
    print()

    # Check maturin
    if not shutil.which("maturin"):
        print("Installing maturin...")
        code = run(["pip", "install", "maturin"], check=False)
        if code != 0:
            print_color("Error: Failed to install maturin", Colors.RED)
            sys.exit(1)
        print_color("[OK] maturin installed", Colors.GREEN)
        print()

    manifest_path = Path("crates/exiftool-py/Cargo.toml")
    python_dir = Path("crates/exiftool-py/python/exiftool_py")

    # Clean .pyd files to avoid "already added" error
    if python_dir.exists():
        for pyd in python_dir.glob("*.pyd"):
            pyd.unlink()

    subcmd = args.subcmd if args.subcmd else ""

    if subcmd == "dev":
        print("Installing in development mode...")
        code = run(["maturin", "develop", "--manifest-path", str(manifest_path)], check=False)
    elif subcmd in ("install", "-i", "--install"):
        print("Building and installing wheel...")
        build_cmd = ["maturin", "build", "--manifest-path", str(manifest_path)]
        if not args.debug:
            build_cmd.append("--release")
        code = run(build_cmd, check=False)
        if code == 0:
            wheels_dir = Path("target/wheels")
            wheels = sorted(wheels_dir.glob("exiftool_py*.whl"), key=lambda p: p.stat().st_mtime, reverse=True)
            if wheels:
                wheel = wheels[0]
                print(f"Installing {wheel.name}...")
                code = run(["pip", "install", str(wheel), "--force-reinstall"], check=False)
    else:
        if args.debug:
            print("Building debug wheel...")
            code = run(["maturin", "build", "--manifest-path", str(manifest_path)], check=False)
        else:
            print("Building release wheel...")
            code = run(["maturin", "build", "--manifest-path", str(manifest_path), "--release"], check=False)

    if code == 0:
        print()
        print_color("[OK] Python module built!", Colors.GREEN)
        print()
        print("Usage:")
        print('  import exiftool_py as exif')
        print('  img = exif.open("photo.jpg")')
        print("  print(img.make, img.model)")

    sys.exit(code)

def cmd_test(args: argparse.Namespace) -> None:
    """Run tests."""
    print_color("Running tests...", Colors.CYAN)
    cmd = ["cargo", "test"] + args.extra
    sys.exit(run(cmd, check=False))

def cmd_codegen(args: argparse.Namespace) -> None:
    """Regenerate tag tables."""
    print_color("Regenerating tag tables...", Colors.CYAN)
    sys.exit(run(["cargo", "xtask", "codegen"], check=False))

def show_help() -> None:
    """Show help message."""
    print_color("exiftool-rs bootstrap script", Colors.CYAN)
    print()
    print("USAGE:")
    print("  python bootstrap.py [COMMAND] [OPTIONS]")
    print()
    print("COMMANDS:")
    print("  build              Build all Rust crates (release)")
    print("  build --debug      Build in debug mode")
    print("  python             Build Python wheel (release)")
    print("  python --debug     Build Python wheel (debug)")
    print("  python dev         Install in development mode (editable, debug)")
    print("  python install     Build and pip install (release)")
    print("  test               Run tests")
    print("  codegen            Regenerate tag tables from ExifTool")
    print()
    print("EXAMPLES:")
    print("  python bootstrap.py build")
    print("  python bootstrap.py build --debug")
    print("  python bootstrap.py python")
    print("  python bootstrap.py python --debug")
    print("  python bootstrap.py python dev")
    print("  python bootstrap.py test")

def main() -> None:
    """Main entry point."""
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("command", nargs="?", default="")
    parser.add_argument("subcmd", nargs="?", default="")
    parser.add_argument("--debug", action="store_true")
    parser.add_argument("-h", "--help", action="store_true")
    args, extra = parser.parse_known_args()
    args.extra = extra

    # Check cargo for all commands except help
    if args.command and args.command not in ("help", "-h", "--help"):
        check_cargo()

    match args.command:
        case "build":
            cmd_build(args)
        case "python":
            cmd_python(args)
        case "test":
            cmd_test(args)
        case "codegen":
            cmd_codegen(args)
        case "help" | "-h" | "--help" | "":
            show_help()
        case _:
            print_color(f"Unknown command: {args.command}", Colors.RED)
            print()
            show_help()
            sys.exit(1)

if __name__ == "__main__":
    main()
