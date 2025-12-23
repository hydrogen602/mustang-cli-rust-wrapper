use std::process::Command;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long)]
    java_home: String,
    #[arg(long)]
    jmods: Option<String>,
    #[arg(long)]
    target: String,
}

fn main() {
    // println!("cargo::rerun-if-changed=Makefile");
    // println!("cargo::rerun-if-env-changed=NO_BUILD");

    // let no_build = env::var_os("NO_BUILD").is_some();
    // if no_build {
    //     println!("cargo:warning=NO_BUILD is set, skipping build");
    //     return;
    // }

    // println!("cargo::rerun-if-env-changed=JAVA_HOME");
    // println!("cargo::rerun-if-env-changed=JLINK_JMODS");

    // let java_home = env::var("JAVA_HOME").expect("JAVA_HOME is not set");
    // let jmods = match env::var("JLINK_JMODS") {
    //     Ok(x) => Some(x),
    //     Err(VarError::NotPresent) => None,
    //     Err(VarError::NotUnicode(e)) => {
    //         panic!("JLINK_JMODS is not a valid unicode string: {}", e.display())
    //     }
    // };

    let args = Args::parse();

    let java_home_arg = format!("JDK_HOME={}", args.java_home);
    let jmods_arg = args.jmods.map(|jmods| format!("JLINK_JMODS={}", jmods));

    let keywords = target_translate(&args.target);

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
            "Running `file` on jlink's produced binary does not contain expected keyword: {}, likely it is the wrong architecture: {}",
            keyword,
            file_output
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
