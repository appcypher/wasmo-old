use clap::{App, Arg, ArgMatches, SubCommand, AppSettings};

pub struct Arguments<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> Arguments<'a> {
    pub fn new() -> Self {
        let matches = App::new("\n               ● WASMO ●\n")
            .about("\nA High-Performance Embeddable WebAssembly Engine  ")
            .setting(AppSettings::ArgRequiredElseHelp)
            .setting(AppSettings::ColoredHelp)
            .setting(AppSettings::ColorAlways)
            .arg(
                Arg::with_name("help")
                    .short("h")
                    .long("help")
                    .help("Print this help information")
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
                    .help("Show version")
            )
            .get_matches();

        Self { matches }
    }
}
