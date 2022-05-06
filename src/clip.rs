use clap::{Arg, ArgMatches, Command};

pub fn parse() -> ArgMatches {
    Command::new("ichain")
        .version(clap::crate_version!())
        .about("IChain is a command program that chains muliple executions of programs passing inputs and outputs as configured in a INI like file.")
        .author("Éverton M. Vieira <everton.muvi@gmail.com>")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("INI")
                .takes_value(true)
                .required(true)
                .help("Chain the programs configured in INI file."),
        )
        .get_matches()
}
