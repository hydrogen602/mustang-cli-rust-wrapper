#[cfg(test)]
mod tests {
    use std::{
        env::{self, VarError},
        fs,
        path::PathBuf,
    };

    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    use crate::*;

    struct Sample {
        path: PathBuf,
    }

    // static LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    // fn single_threaded_guard<'m>() -> Option<MutexGuard<'m, ()>> {
    //     if env_flag("USE_TRACING_AGENT") {
    //         println!("Running single threaded due to USE_TRACING_AGENT");
    //         // run single threaded if tracing agent is enabled
    //         Some(LOCK.lock().unwrap())
    //     } else {
    //         None
    //     }
    // }

    impl Sample {
        #[allow(unused)]
        fn html(&self) -> PathBuf {
            // find self.path/*_analyzed-content.html
            glob::glob(&format!("{}/*_analyzed-content.html", self.path.display()))
                .unwrap()
                .next()
                .unwrap()
                .unwrap()
        }

        fn pdf(&self) -> PathBuf {
            // find {name}_fx.pdf
            glob::glob(&format!("{}/*_fx.pdf", self.path.display()))
                .unwrap()
                .next()
                .unwrap()
                .unwrap()
        }

        fn xml(&self) -> PathBuf {
            // find {name}.xml
            glob::glob(&format!("{}/*.xml", self.path.display()))
                .unwrap()
                .next()
                .unwrap()
                .unwrap()
        }

        #[allow(unused)]
        fn validation_report(&self) -> PathBuf {
            // find {name}_fx_validation_report.pdf
            glob::glob(&format!(
                "{}/*_fx_validation_report.pdf",
                self.path.display()
            ))
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
        }
    }

    fn all_samples() -> Vec<Sample> {
        let folder = Path::new("samples/ZF24_DE/Beispiele");
        fs::read_dir(folder)
            .unwrap()
            .flat_map(|e| {
                // there should only be folders in the Beispiele folder
                fs::read_dir(e.unwrap().path()).unwrap().map(|e| Sample {
                    path: e.unwrap().path(),
                })
            })
            .collect()
    }

    fn env_flag(name: &str) -> bool {
        match env::var(name) {
            Ok(x) => {
                let x = x.trim().to_ascii_lowercase();
                if x == "" || x == "0" || x == "false" {
                    false
                } else {
                    true
                }
            }
            Err(VarError::NotPresent) => false,
            Err(e) => panic!("Err: {}", e),
        }
    }

    fn cli() -> MustangCLI {
        let use_graalvm = env_flag("USE_GRAALVM");
        println!("USE_GRAALVM: {}", use_graalvm);

        if use_graalvm {
            MustangCLI::from_graalvm_exe("Mustang-CLI-2.20.0")
                .unwrap()
                .with_log_print()
        } else {
            let tracing_agent = env_flag("USE_TRACING_AGENT");
            println!("USE_TRACING_AGENT: {}", tracing_agent);

            let mut java_args = vec![];
            if tracing_agent {
                java_args.push(OsString::from(
                    "-agentlib:native-image-agent=config-output-dir=tracing-agent/dir-{pid}-{datetime}/",
                ));
            }

            let java_path = env::var("JAVA_HOME").unwrap();
            let java_path = Path::new(&java_path).join("bin/java");
            MustangCLI::from_jar(java_path, "Mustang-CLI-2.20.0.jar", java_args)
                .unwrap()
                .with_log_print()
        }
    }

    #[test]
    fn test_extract_xml_from_pdf() {
        // let cli = MustangCLI::from_graalvm_exe("Mustang-CLI-2.20.0").unwrap();
        let cli = cli();

        // let _ = single_threaded_guard();

        let tracing_agent = env_flag("USE_TRACING_AGENT");

        let samples = all_samples();
        assert!(samples.len() > 20);
        let one_sample = |sample: Sample| {
            println!("Processing sample: {:?}", sample.path);
            let input = FileInput::from_path(sample.pdf()).unwrap();
            let mut output = FileOutput::temp().unwrap();

            cli.extract_xml_from_pdf(&input, &mut output).unwrap();

            let expected = fs::read(sample.xml()).unwrap();
            let output = output.read_bytes().unwrap();
            // fs::write("out.xml", &output).unwrap();

            // remove all \r
            let output = output
                .into_iter()
                .filter(|&c| c != b'\r')
                .collect::<Vec<_>>();
            let expected = expected
                .into_iter()
                .filter(|&c| c != b'\r')
                .collect::<Vec<_>>();

            // let out = diff::slice(&expected, &output);
            // let is_empty = out.is_empty();
            // for compare in out {
            //     match compare {
            //         diff::Result::Left(l) => println!("Left: '{}'", l),
            //         diff::Result::Right(r) => println!("Right: '{}'", r),
            //         _ => (),
            //     }
            // }
            // assert!(is_empty);
            assert_eq!(output, expected);
        };

        samples.into_par_iter().for_each(one_sample);
    }

    #[test]
    fn test_add_xml_to_pdf() {
        let cli = cli();

        // let _ = single_threaded_guard();

        let samples = all_samples().into_iter().next().unwrap();
        let input = FileInput::from_path(samples.xml()).unwrap();
        let pdf_input = FileInput::from_path("samples/sample.pdf").unwrap();
        let mut output = FileOutput::temp().unwrap();

        cli.combine_xml_and_pdf(
            &pdf_input,
            &input,
            &mut output,
            Format::Zugferd,
            Config::FacturXOrZugferdV2 {
                profile: defs::ProfileV2::EN16931,
            },
            &[],
        )
        .unwrap();

        assert!(output.path().exists());

        // extract it again
        let input = output.into();
        let mut output = FileOutput::temp().unwrap();
        cli.extract_xml_from_pdf(&input, &mut output).unwrap();

        let expected = fs::read(samples.xml()).unwrap();
        let output = output.read_bytes().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_validate() {
        let cli = cli();

        // let _ = single_threaded_guard();

        let sample = all_samples().into_iter().next().unwrap();
        let input = FileInput::from_path(sample.xml()).unwrap();
        cli.validate(&input, false, None, false).unwrap();

        let input = FileInput::from_path(sample.pdf()).unwrap();
        cli.validate(&input, false, None, false).unwrap();
    }
}
