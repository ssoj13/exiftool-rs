// Workaround for PyO3 not emitting -lpython on macOS with Homebrew Python
// PyO3 detects Python correctly but fails to emit the link flag on macOS
// due to Framework vs regular library handling differences

fn main() {
    // Only apply workaround on macOS
    if std::env::consts::OS != "macos" {
        return;
    }

    // Get Python ldflags via python3-config
    let output = std::process::Command::new("python3-config")
        .arg("--ldflags")
        .output();

    let output = match output {
        Ok(o) if o.status.success() => o,
        _ => return, // Silently skip if python3-config not available
    };

    let ldflags = String::from_utf8_lossy(&output.stdout);
    
    // Extract -lpythonX.Y flag
    for flag in ldflags.split_whitespace() {
        if flag.starts_with("-lpython") {
            let lib_name = &flag[2..]; // Strip "-l"
            println!("cargo:rustc-link-lib={}", lib_name);
            break;
        }
    }
}
