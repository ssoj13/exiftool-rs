#!/usr/bin/env python3
"""
bootstrap.py - Unified local build/test/check script for exiftool-rs.

Cross-platform, Python 3, stdlib only. Adapted from gitnexus-rs bootstrap.

Commands:
    b        Build workspace (release by default, -d for debug)
    t        Run workspace tests
    i        Install CLI `exif` (`cargo install --path crates/exiftool-cli`)
    c        cargo fmt --all --check + clippy --workspace --all-targets -D warnings
    cl       cargo clean
    p        Python bindings (maturin); requires a subcommand
    m        mdbook in docs/; requires a subcommand
    h        Print help

Flags:
    -d, --debug       Debug profile for build / python / CLI install
    -n, --nocapture   Show test output
    -f, --force       cargo install --force (`i`)

Examples:
    python bootstrap.py b
    python bootstrap.py t -n
    python bootstrap.py c
    python bootstrap.py i
    python bootstrap.py p b
    python bootstrap.py p d
    python bootstrap.py m b
"""

from __future__ import annotations

import argparse
import os
import platform
import shutil
import subprocess
import sys
import time
from pathlib import Path


ROOT_DIR = Path(__file__).parent.resolve()
IS_WINDOWS = platform.system() == "Windows"

CLI_CRATE = ROOT_DIR / "crates" / "exiftool-cli"
CLI_BIN = "exif"
PY_MANIFEST = ROOT_DIR / "crates" / "exiftool-py" / "Cargo.toml"
PY_PACKAGE = ROOT_DIR / "crates" / "exiftool-py" / "python" / "exiftool_py"
WHEELS_DIR = ROOT_DIR / "target" / "wheels"
BOOK_DIR = ROOT_DIR / "docs"

COMMANDS = ["b", "t", "i", "c", "cl", "p", "m", "h"]
XTASK_COMMANDS = {"codegen", "dump"}
PY_SUB = {"b", "d", "i"}
BOOK_SUB = {"b", "s", "cl"}
LEAF = {"b", "t", "i", "c", "cl"}


class C:
    RST = "\033[0m"
    RED = "\033[91m"
    GRN = "\033[92m"
    YLW = "\033[93m"
    CYN = "\033[96m"
    WHT = "\033[97m"

    @classmethod
    def init(cls) -> None:
        if IS_WINDOWS:
            os.system("")


def fmt_time(ms: float) -> str:
    if ms < 1000:
        return f"{ms:.0f}ms"
    if ms < 60000:
        return f"{ms / 1000:.1f}s"
    mins = int(ms // 60000)
    secs = (ms % 60000) / 1000
    return f"{mins}m{secs:.0f}s"


def header(text: str) -> None:
    line = "=" * 60
    print(f"\n{C.CYN}{line}\n{text}\n{line}{C.RST}")


def step(text: str) -> None:
    print(f"  {C.WHT}{text}{C.RST}")


def ok(text: str) -> None:
    print(f"  {C.GRN}[OK] {text}{C.RST}")


def warn(text: str) -> None:
    print(f"  {C.YLW}[WARN] {text}{C.RST}")


def err(text: str) -> None:
    print(f"  {C.RED}[ERR] {text}{C.RST}")


def run(args: list[str], cwd: Path | None = None, capture: bool = False) -> tuple[int, str, float]:
    start = time.perf_counter()
    result = subprocess.run(args, cwd=cwd or ROOT_DIR, capture_output=capture, text=True)
    elapsed_ms = (time.perf_counter() - start) * 1000
    output = (result.stdout or "") + (result.stderr or "") if capture else ""
    return result.returncode, output, elapsed_ms


def which(cmd: str) -> Path | None:
    found = shutil.which(cmd)
    return Path(found) if found else None


def check_cargo() -> bool:
    if not which("cargo"):
        err("Rust/Cargo not found")
        step("Install Rust from https://rustup.rs/")
        return False
    return True


def release_cli_bin() -> Path:
    name = f"{CLI_BIN}.exe" if IS_WINDOWS else CLI_BIN
    return ROOT_DIR / "target" / "release" / name


def require_tool(name: str, hint: str) -> bool:
    if which(name):
        return True
    err(f"`{name}` not found on PATH")
    step(hint)
    return False


def strip_stale_pyd() -> None:
    if not PY_PACKAGE.is_dir():
        return
    for pyd in PY_PACKAGE.glob("*.pyd"):
        pyd.unlink()
        step(f"Removed stale {pyd.name}")


def run_build(args: argparse.Namespace) -> int:
    header("BUILD")
    cmd = ["cargo", "build", "--workspace", "--all-targets"]
    if args.debug:
        step("Mode: debug")
    else:
        cmd.append("--release")
        step("Mode: release")
    print()
    code, _, elapsed = run(cmd)
    if code == 0:
        ok(f"Build successful ({fmt_time(elapsed)})")
        step(f"CLI: {release_cli_bin()}")
    else:
        err("Build failed")
    print()
    return code


def run_test(args: argparse.Namespace) -> int:
    header("TEST")
    cmd = ["cargo", "test", "--workspace"]
    if args.nocapture:
        cmd.extend(["--", "--nocapture"])
    print()
    code, _, elapsed = run(cmd)
    if code == 0:
        ok(f"Tests passed ({fmt_time(elapsed)})")
    else:
        err("Tests failed")
    print()
    return code


def run_check(_args: argparse.Namespace) -> int:
    header("CHECK")
    passed = True

    step("Checking formatting (workspace)...")
    code, _, elapsed = run(["cargo", "fmt", "--all", "--check"])
    if code == 0:
        ok(f"Format OK ({fmt_time(elapsed)})")
    else:
        err("Format check failed")
        passed = False

    print()
    step("Running clippy (--workspace --all-targets -D warnings)...")
    clippy = [
        "cargo",
        "clippy",
        "--workspace",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ]
    code, _, elapsed = run(clippy)
    if code == 0:
        ok(f"Clippy OK ({fmt_time(elapsed)})")
    else:
        err("Clippy failed")
        passed = False

    print()
    if passed:
        ok("All checks passed")
    else:
        err("Some checks failed")
    print()
    return 0 if passed else 1


def run_install(args: argparse.Namespace) -> int:
    header("INSTALL")
    cmd = ["cargo", "install", "--path", str(CLI_CRATE)]
    if (ROOT_DIR / "Cargo.lock").is_file():
        cmd.append("--locked")
    if args.force:
        cmd.append("--force")
    if args.debug:
        cmd.append("--debug")
        step("Mode: debug")
    else:
        step("Mode: release")
    print()
    code, _, elapsed = run(cmd)
    if code == 0:
        ok(f"Installed {CLI_BIN} ({fmt_time(elapsed)})")
        step("Binary is on PATH (cargo bin dir)")
    else:
        err("Install failed")
    print()
    return code


def run_clean(_args: argparse.Namespace) -> int:
    header("CLEAN")
    code, _, elapsed = run(["cargo", "clean"])
    if code == 0:
        ok(f"cargo clean ({fmt_time(elapsed)})")
    else:
        err("cargo clean failed")
    print()
    return code


def run_python(args: argparse.Namespace) -> int:
    header("PYTHON")
    if not require_tool("maturin", "Install: python -m pip install maturin"):
        return 1

    strip_stale_pyd()
    manifest = str(PY_MANIFEST)
    sub = args.sub

    if sub == "d":
        step("maturin develop")
        print()
        code, _, elapsed = run(["maturin", "develop", "--manifest-path", manifest])
        if code == 0:
            ok(f"Dev install done ({fmt_time(elapsed)})")
        else:
            err("maturin develop failed")
        print()
        return code

    build_cmd = ["maturin", "build", "--manifest-path", manifest]
    if args.debug:
        step("Mode: debug")
    else:
        build_cmd.append("--release")
        step("Mode: release")
    print()
    code, _, elapsed = run(build_cmd)
    if code != 0:
        err("maturin build failed")
        print()
        return code
    ok(f"Wheel built ({fmt_time(elapsed)})")

    if sub == "b":
        print()
        return 0

    wheels = sorted(
        WHEELS_DIR.glob("exiftool_py*.whl"),
        key=lambda p: p.stat().st_mtime,
        reverse=True,
    )
    if not wheels:
        err(f"No wheel in {WHEELS_DIR}")
        return 1
    wheel = wheels[0]
    step(f"pip install {wheel.name} --force-reinstall")
    code, _, elapsed = run(
        [sys.executable, "-m", "pip", "install", str(wheel), "--force-reinstall"]
    )
    if code == 0:
        ok(f"Wheel installed ({fmt_time(elapsed)})")
    else:
        err("pip install failed")
    print()
    return code


def run_book(args: argparse.Namespace) -> int:
    header("BOOK")
    if not require_tool("mdbook", "Install: cargo install mdbook"):
        return 1

    sub = args.sub
    if sub == "s":
        step("mdbook serve docs/  (http://localhost:3000)")
        print()
        code, _, _ = run(["mdbook", "serve", str(BOOK_DIR)])
        return code

    if sub == "cl":
        output_dir = BOOK_DIR / "book"
        if output_dir.exists():
            shutil.rmtree(output_dir)
            ok(f"Removed {output_dir}")
        else:
            step("Nothing to clean")
        print()
        return 0

    step("mdbook build docs/")
    print()
    code, _, elapsed = run(["mdbook", "build", str(BOOK_DIR)])
    if code == 0:
        ok(f"Book built ({fmt_time(elapsed)})")
        step(f"Output: {BOOK_DIR / 'book' / 'index.html'}")
    else:
        err("mdbook build failed")
    print()
    return code


def run_xtask(extra_args: list[str]) -> int:
    header("XTASK")
    step(" ".join(["cargo", "xtask", *extra_args]))
    print()
    code, _, elapsed = run(["cargo", "xtask", *extra_args])
    if code == 0:
        ok(f"xtask done ({fmt_time(elapsed)})")
    else:
        err("xtask failed")
    print()
    return code


HELP_TEXT = """
EXIFTOOL-RS BUILD SYSTEM

CLI binary is `exif` (crates/exiftool-cli). Python bindings use maturin.
Tag tables: cargo xtask. Docs: mdbook in docs/.

COMMANDS
  b       cargo build --workspace --all-targets (--release unless -d)
  t       cargo test --workspace
  i       cargo install --path crates/exiftool-cli
  c       cargo fmt --all --check + clippy --workspace --all-targets -D warnings
  cl      cargo clean
  p b     maturin build wheel
  p d     maturin develop
  p i     maturin build + pip install --force-reinstall
  m b     mdbook build docs/
  m s     mdbook serve docs/
  m cl    remove docs/book/
  h       help

OPTIONS
  -d, --debug        debug profile for b / p / i
  -n, --nocapture    show test output
  -f, --force        cargo install --force (i)

XTASK
  codegen, dump      passed through to `cargo xtask`

EXAMPLES
  python bootstrap.py b
  python bootstrap.py b -d
  python bootstrap.py t -n
  python bootstrap.py c
  python bootstrap.py i
  python bootstrap.py i -f
  python bootstrap.py p b
  python bootstrap.py p d
  python bootstrap.py p i
  python bootstrap.py m b
  python bootstrap.py m s
  python bootstrap.py codegen
"""

PY_HELP = """
PYTHON (maturin, crates/exiftool-py)

  p b     build wheel
  p d     develop (editable)
  p i     build wheel and pip install --force-reinstall

  -d      debug wheel (p b / p i)

EXAMPLES
  python bootstrap.py p b
  python bootstrap.py p d
  python bootstrap.py p i
"""

BOOK_HELP = """
BOOK (mdbook, docs/)

  m b     build
  m s     serve (http://localhost:3000)
  m cl    remove docs/book/

EXAMPLES
  python bootstrap.py m b
  python bootstrap.py m s
  python bootstrap.py m cl
"""

DISPATCH = {
    "b": run_build,
    "t": run_test,
    "i": run_install,
    "c": run_check,
    "cl": run_clean,
    "p": run_python,
    "m": run_book,
}


def main() -> int:
    if hasattr(sys.stdout, "reconfigure"):
        sys.stdout.reconfigure(line_buffering=True)

    C.init()

    if len(sys.argv) > 1 and sys.argv[1] in XTASK_COMMANDS:
        if not check_cargo():
            return 1
        return run_xtask(sys.argv[1:])

    parser = argparse.ArgumentParser(
        description="exiftool-rs build system",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "command",
        nargs="?",
        choices=COMMANDS,
        default="h",
        help="b, t, i, c, cl, p, m, h",
    )
    parser.add_argument("sub", nargs="?", default=None, help="subcommand for p / m")
    parser.add_argument("-d", "--debug", action="store_true", help="Debug mode")
    parser.add_argument("-n", "--nocapture", action="store_true", help="Show test output")
    parser.add_argument(
        "-f",
        "--force",
        action="store_true",
        help="Force cargo install (i)",
    )
    args = parser.parse_args()

    if args.command == "h":
        print(HELP_TEXT)
        return 0

    if args.command in LEAF and args.sub is not None:
        err(f"`{args.command}` takes no subcommand")
        print(HELP_TEXT)
        return 1

    if args.command == "p":
        if args.sub is None:
            print(PY_HELP)
            return 0
        if args.sub not in PY_SUB:
            err(f"Unknown python subcommand: {args.sub}")
            print(PY_HELP)
            return 1

    if args.command == "m":
        if args.sub is None:
            print(BOOK_HELP)
            return 0
        if args.sub not in BOOK_SUB:
            err(f"Unknown book subcommand: {args.sub}")
            print(BOOK_HELP)
            return 1

    if not check_cargo():
        return 1

    handler = DISPATCH.get(args.command)
    if handler:
        return handler(args)

    print(HELP_TEXT)
    return 0


if __name__ == "__main__":
    sys.exit(main())
