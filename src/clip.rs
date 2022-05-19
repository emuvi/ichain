use clap::{Arg, ArgMatches, Command};

pub fn parse() -> ArgMatches {
  Command::new("pchain")
    .version(clap::crate_version!())
    .about("PChain is a command program that chains multiple executions of programs in parallel passing inputs and outputs between then as configured in a PCH file.")
    .author("Éverton M. Vieira <emuvi@outlook.com.br>")
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
      Arg::new("debug-times")
        .short('m')
        .long("debug-times")
        .takes_value(false)
        .required(false)
        .help("If has debug symbols, will show the time of each logged."),
    )
    .arg(
      Arg::new("delay")
        .short('d')
        .long("delay")
        .value_name("TIME")
        .takes_value(true)
        .required(false)
        .help("How much time to wait until next attempt get the value to pass."),
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
