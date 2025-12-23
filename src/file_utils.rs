use std::path::PathBuf;

use crate::{MustangCLI, error::MustangError};

/// Return the JRE home directory as built by build.rs
pub fn jre_home() -> PathBuf {
    let p = PathBuf::from(env!("OUT_DIR"));
    p.join("jre")
}

/// Return the jar as setup by build.rs
pub fn jar() -> PathBuf {
    let p = PathBuf::from(env!("OUT_DIR"));
    p.join("Mustang-CLI-2.20.0.jar")
}

/// Use the jre and jar as setup by build.rs
pub fn build_rs_mustang_cli() -> Result<MustangCLI, MustangError> {
    let java_home = jre_home();
    let jar = jar();
    MustangCLI::from_jar(java_home.join("bin/java"), jar, vec![])
        .map(|cli| cli.with_java_home(java_home))
}
