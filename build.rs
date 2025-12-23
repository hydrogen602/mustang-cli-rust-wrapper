use std::{env::VarError, path::Path, process::Command};

fn main() {
    if !std::env::var("CARGO_FEATURE_JLINK").is_ok() {
        return;
    }

    println!("cargo::rerun-if-changed=Makefile");
    println!("cargo::rerun-if-env-changed=JAVA_HOME");
    println!("cargo::rerun-if-env-changed=JLINK_JMODS");

    // cp Makefile to OUT_DIR
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR is not set");
    let out_dir = Path::new(&out_dir);
    let makefile_path = out_dir.join("Makefile");
    std::fs::copy("Makefile", makefile_path).expect("Failed to copy Makefile");

    // cd OUT_DIR
    std::env::set_current_dir(out_dir).expect("Failed to set current directory");

    build_jre();

    // panic!("{}", out_dir.display());
}

fn build_jre() {
    let java_home = std::env::var("JAVA_HOME").expect("JAVA_HOME is not set");
    let jmods = match std::env::var("JLINK_JMODS") {
        Ok(x) => Some(x),
        Err(VarError::NotPresent) => None,
        Err(VarError::NotUnicode(e)) => {
            panic!("JLINK_JMODS is not a valid unicode string: {}", e.display())
        }
    };

    let java_home_arg = format!("JDK_HOME={}", java_home);
    let jmods_arg = jmods.map(|jmods| format!("JLINK_JMODS={}", jmods));

    let target = std::env::var("TARGET").expect("TARGET is not set");
    let keywords = target_translate(&target);

    let status = Command::new("make")
        .arg("clean-jre")
        .status()
        .expect("Failed to run make");
    assert!(status.success(), "Failed to run make");

    let status = Command::new("make")
        .arg(java_home_arg)
        .args(jmods_arg)
        .arg("build-jre")
        .status()
        .expect("Failed to run make");
    assert!(status.success(), "Failed to run make");

    // confirm its the right binary
    let make_output = Command::new("make")
        .arg("print-jre-java-bin")
        .output()
        .expect("Failed to run make");
    assert!(make_output.status.success(), "Failed to run make");
    let java_bin = String::from_utf8_lossy(&make_output.stdout);
    let java_bin = java_bin.trim();

    let file_output = Command::new("file")
        .arg(java_bin)
        .output()
        .expect("Failed to run file");
    assert!(file_output.status.success(), "Failed to run file");
    let file_output = String::from_utf8_lossy(&file_output.stdout);
    for keyword in keywords {
        assert!(
            file_output.contains(keyword),
            "Running `file` on jlink's produced binary does not contain expected keyword: {}, likely it is the wrong architecture: {}. JAVA_HOME={}",
            keyword,
            file_output,
            java_home
        );
    }
}

/// Return a list of keywords that `file` should
/// return when running on a binary of this target.
fn target_translate(target: &str) -> &'static [&'static str] {
    match target {
        "aarch64-apple-darwin" => &["Mach-O", "64-bit", "arm64"],
        "x86_64-apple-darwin" => &["Mach-O", "64-bit", "x86_64"],
        _ => panic!("Unsupported target: {}", target),
    }
}
