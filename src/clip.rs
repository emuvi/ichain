use clap::{ArgMatches, Command};

pub fn parse() -> ArgMatches {
    Command::new("ichain")
        .version(clap::crate_version!())
        .about("IChain is a command program that chains muliple executions of programs passing inputs and outputs as configured in a INI like file.")
        .author("Éverton M. Vieira <everton.muvi@gmail.com>")
        .get_matches()
}
