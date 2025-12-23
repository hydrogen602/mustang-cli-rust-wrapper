use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
    process::{Command, Output},
};

use crate::{
    defs::{Action, Config, Format, Language, Versioned},
    error::MustangError,
    file_handle::{FileInput, FileOutput},
};

pub mod defs;
pub mod error;
pub mod file_handle;
#[cfg(feature = "jlink")]
pub mod file_utils;
mod tests;

macro_rules! args {
    ($($arg:expr),*) => {
        &[$(AsRef::<OsStr>::as_ref($arg)),*]
    };
    ($($arg:expr),*,) => {
        &[$(AsRef::<OsStr>::as_ref($arg)),*]
    };
}

#[derive(Debug)]
pub struct MustangCLI {
    runner: RunnerMustangCLI,
    log_print: bool,
}

#[derive(Debug)]
pub enum RunnerMustangCLI {
    /// e.g. for GraalVM native-image
    Exe {
        bin: PathBuf,
        extra_args: Vec<OsString>,
    },
    Jar {
        java_path: PathBuf,
        jar_path: PathBuf,
        java_args: Vec<OsString>,
    },
}

impl MustangCLI {
    pub fn with_log_print(mut self) -> Self {
        self.log_print = true;
        self
    }

    pub fn from_graalvm_exe(
        graalvm_bin: impl AsRef<Path>,
        extra_args: Vec<OsString>,
    ) -> Result<Self, MustangError> {
        let graalvm_bin = graalvm_bin
            .as_ref()
            .canonicalize()
            .map_err(MustangError::ExecutableOrJavaNotFound)?;

        Ok(Self {
            runner: RunnerMustangCLI::Exe {
                bin: graalvm_bin,
                extra_args,
            },
            log_print: false,
        })
    }

    pub fn from_jar(
        java_path: impl AsRef<Path>,
        jar_path: impl AsRef<Path>,
        java_args: Vec<OsString>,
    ) -> Result<Self, MustangError> {
        let java_path = java_path
            .as_ref()
            .canonicalize()
            .map_err(MustangError::ExecutableOrJavaNotFound)?;
        let jar_path = jar_path
            .as_ref()
            .canonicalize()
            .map_err(MustangError::ExecutableOrJavaNotFound)?;

        Ok(Self {
            runner: RunnerMustangCLI::Jar {
                java_path,
                jar_path,
                java_args,
            },
            log_print: false,
        })
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
        profile_and_version: Config,
        attachments: &[FileInput],
    ) -> Result<(), MustangError> {
        let attachments_str = attachments
            .iter()
            .map(|a| a.path().as_os_str())
            .collect::<Vec<_>>()
            .join(",".as_ref());
        self.run_command(
            Action::CombineXmlAndPdf,
            args!(
                "--source",
                input,
                "--source-xml",
                xml,
                "--out",
                output,
                "--format",
                &format,
                "--version",
                &profile_and_version.version(),
                "--profile",
                profile_and_version.profile_as_str(),
                "--no-additional-attachments",
                "--attachments",
                &attachments_str,
            ),
        )
    }

    pub fn ubl(&self, input: &FileInput, output: &mut FileOutput) -> Result<(), MustangError> {
        self.run_command(Action::Ubl, args!("--source", input, "--out", output))
    }

    pub fn upgrade(&self, input: &FileInput, output: &mut FileOutput) -> Result<(), MustangError> {
        self.run_command(Action::Upgrade, args!("--source", input, "--out", output))
    }

    pub fn validate(
        &self,
        input: &FileInput,
        no_notices: bool,
        log_append: Option<&str>,
        log_as_pdf: bool,
    ) -> Result<(), MustangError> {
        let mut args: Vec<&OsStr> = Vec::new();
        args.extend(args!("--source", input));
        if no_notices {
            args.extend(args!("--no-notices"));
        }
        if let Some(log_append) = log_append {
            args.extend(args!("--logAppend", log_append));
        }
        if log_as_pdf {
            args.extend(args!("--log-as-pdf"));
        }
        self.run_command(Action::Validate, &args)
    }

    pub fn visualize(
        &self,
        input: &FileInput,
        output: &mut FileOutput,
        language: Language,
    ) -> Result<(), MustangError> {
        self.run_command(
            Action::XmlToHtml,
            args!("--language", &language, "--source", input, "--out", output),
        )
    }

    pub fn xml_to_pdf(
        &self,
        input: &FileInput,
        output: &mut FileOutput,
    ) -> Result<(), MustangError> {
        self.run_command(Action::XmlToPdf, args!("--source", input, "--out", output))
    }

    fn run_command(&self, action: Action, args: &[&OsStr]) -> Result<(), MustangError> {
        self.handle_output(self.start_command(action).args(args).output()?)
    }

    fn start_command(&self, action: Action) -> Command {
        let mut c = match &self.runner {
            RunnerMustangCLI::Exe { bin, extra_args } => {
                let mut c = Command::new(bin);
                c.args(extra_args);
                c
            }
            RunnerMustangCLI::Jar {
                java_path,
                jar_path,
                java_args,
            } => {
                let mut c = Command::new(java_path);
                c.args(java_args);
                c.arg("-jar").arg(jar_path);
                c
            }
        };
        c.args(args!("--action", &action, "--disable-file-logging"));
        c
    }

    fn handle_output(&self, output: Output) -> Result<(), MustangError> {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        if self.log_print {
            println!("Mustang CLI stdout:\n{}", stdout);
            println!("Mustang CLI stderr:\n{}", stderr);
        }
        if !output.status.success() {
            return Err(MustangError::ExecutionFailed {
                status: output.status,
                stdout: stdout.to_string(),
                stderr: stderr.to_string(),
            });
        }
        Ok(())
    }
}
