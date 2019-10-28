use clap::{App, AppSettings, Arg, ArgMatches};

pub struct Arguments<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> Arguments<'a> {
    pub(crate) fn new() -> Self {
        let matches = App::new("\n               ● WASMO ●\n")
            .about("\nA High-Performance Embeddable WebAssembly Engine  ")
            .setting(AppSettings::ArgRequiredElseHelp)
            .setting(AppSettings::ColoredHelp)
            .setting(AppSettings::ColorAlways)
            .after_help("")
            .arg(
                Arg::with_name("help")
                    .short("h")
                    .long("help")
                    .help("Print this help information"),
            )
            .arg(
                Arg::with_name("FILE")
                    .help("WebAssembly file to run")
                    .index(1),
            )
            .arg(
                Arg::with_name("version")
                    .short("v")
                    .long("version")
                    .help("Show version"),
            )
            .get_matches();

        Self { matches }
    }

    pub(crate) fn get_file_path(&self) -> Result<Option<String>, String> {
        if let Some(s) = self.matches.value_of("FILE") {
            return Ok(Some(s.to_owned()));
        }

        Ok(None)
    }
}
