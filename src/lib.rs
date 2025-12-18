use std::{
    ffi::OsStr,
    io::ErrorKind,
    path::PathBuf,
    process::{Command, Output},
};

use crate::{
    error::MustangError,
    file_handle::{FileInput, FileOutput},
};

pub mod error;
pub mod file_handle;

#[derive(Debug)]
pub struct MustangCLI {
    graalvm_bin: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Action {
    ExtractXmlFromPdf,
    A3Only,
    CombineXmlAndPdf,
    Ubl,
    Upgrade,
    Validate,
    XmlToHtml,
    XmlToPdf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Format {
    FacturX,
    Zugferd,
    OrderX,
    CrossIndustryDespatchAdvice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Version {
    V1,
    V2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ProfileV1 {
    BASIC,
    COMFORT,
    EXTENDED,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ProfileV2 {
    MINIMUM,
    BasicWl,
    BASIC,
    CIUS,
    EN16931,
    XRechnung,
    EXTENDED,
}

impl ProfileV2 {
    pub fn as_str(&self) -> &str {
        match self {
            Self::MINIMUM => "M",
            Self::BASIC_WL => "W",
            Self::BASIC => "B",
            Self::CIUS => "C",
            Self::EN16931 => "E",
            Self::X_RECHNUNG => "X",
            Self::EXTENDED => "T",
        }
    }
}

impl ProfileV1 {
    pub fn as_str(&self) -> &str {
        match self {
            Self::BASIC => "B",
            Self::COMFORT => "C",
            Self::EXTENDED => "T",
        }
    }
}

impl Version {
    pub fn as_str(&self) -> &str {
        match self {
            Self::V1 => "1",
            Self::V2 => "2",
        }
    }
}

impl Action {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ExtractXmlFromPdf => "extract",
            Self::A3Only => "a3only",
            Self::CombineXmlAndPdf => "combine",
            Self::Ubl => "ubl",
            Self::Upgrade => "upgrade",
            Self::Validate => "validate",
            Self::XmlToHtml => "visualize",
            Self::XmlToPdf => "pdf",
        }
    }
}

impl Format {
    pub fn as_str(&self) -> &str {
        match self {
            Self::FacturX => "fx",
            Self::Zugferd => "zf",
            Self::OrderX => "ox",
            Self::CrossIndustryDespatchAdvice => "da",
        }
    }
}

macro_rules! args {
    ($($arg:expr),*) => {
        &[$(AsRef::<OsStr>::as_ref($arg)),*]
    };
    ($($arg:expr),*,) => {
        &[$(AsRef::<OsStr>::as_ref($arg)),*]
    };
}

impl MustangCLI {
    pub fn new(graalvm_bin: PathBuf) -> Result<Self, MustangError> {
        if !graalvm_bin.exists() {
            return Err(MustangError::ExecutableNotFound(graalvm_bin.clone()));
        }
        Ok(Self { graalvm_bin })
    }

    pub fn extract_xml_from_pdf(
        &self,
        input: &FileInput,
        output: &mut FileOutput,
    ) -> Result<(), MustangError> {
        self.run_command(
            Action::ExtractXmlFromPdf,
            args!("--source", input, "--out", output),
        )
    }

    pub fn a3_only(&self, input: &FileInput, output: &mut FileOutput) -> Result<(), MustangError> {
        self.run_command(Action::A3Only, args!("--source", input, "--out", output))
    }

    pub fn combine_xml_and_pdf(
        &self,
        input: &FileInput,
        xml: &FileInput,
        output: &mut FileOutput,
        format: Format,
        version: Version,
        profile: Profile,
    ) -> Result<(), MustangError> {
        self.run_command(
            Action::CombineXmlAndPdf,
            args!("--source", input, "--source-xml", xml, "--out", output),
        )
    }

    fn run_command(&self, action: Action, args: &[&OsStr]) -> Result<(), MustangError> {
        self.handle_output(self.start_command(action).args(args).output()?)
    }

    fn start_command(&self, action: Action) -> Command {
        let mut c = Command::new(&self.graalvm_bin);
        c.arg("--action").arg(action.as_str());
        c
    }

    fn handle_output(&self, output: Output) -> Result<(), MustangError> {
        if !output.status.success() {
            return Err(MustangError::ExecutionFailed {
                status: output.status,
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }
        Ok(())
    }
}
