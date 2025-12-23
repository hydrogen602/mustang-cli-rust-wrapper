use std::path::PathBuf;

pub fn jre_home() -> PathBuf {
    let p = PathBuf::from(env!("OUT_DIR"));
    p.join("jre")
}

pub fn jar() -> PathBuf {
    let p = PathBuf::from(env!("OUT_DIR"));
    p.join("Mustang-CLI-2.20.0.jar")
}
