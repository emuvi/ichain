use clap::{Arg, ArgMatches, Command};

pub fn parse() -> ArgMatches {
  Command::new("ichain")
    .version(clap::crate_version!())
    .about("IChain is a command program that chains multiple executions of programs in parallel passing inputs and outputs between then as configured in a ICH file.")
    .author("Éverton M. Vieira <everton.muvi@gmail.com>")
    .arg(
      Arg::new("verbose")
        .short('v')
        .long("verbose")
        .takes_value(false)
        .required(false)
        .help("Prints verbose information."),
    )
    .arg(
      Arg::new("archive")
        .short('a')
        .long("archive")
        .takes_value(false)
        .required(false)
        .help("Saves the archive log on a file."),
    )
    .arg(
      Arg::new("debug-calls")
        .short('c')
        .long("debug-calls")
        .takes_value(false)
        .required(false)
        .help("If has debug symbols, is debugged the functions calls."),
    )
    .arg(
      Arg::new("debug-reavs")
        .short('r')
        .long("debug-reavs")
        .takes_value(false)
        .required(false)
        .help("If has debug symbols, is debugged the functions returns."),
    )
    .arg(
      Arg::new("debug-steps")
        .short('s')
        .long("debug-steps")
        .takes_value(false)
        .required(false)
        .help("If has debug symbols, is debugged the functions operations."),
    )
    .arg(
      Arg::new("debug-tells")
        .short('t')
        .long("debug-tells")
        .takes_value(false)
        .required(false)
        .help("If has debug symbols, is debugged the functions iterations."),
    )
    .arg(
      Arg::new("input")
        .short('i')
        .long("input")
        .value_name("FILE")
        .takes_value(true)
        .required(true)
        .help("Chain the programs configured in the FILE path."),
    )
    .get_matches()
}
