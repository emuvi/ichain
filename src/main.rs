mod clip;
mod flow;
mod runner;
mod setup;

use rubx::{rux_dbg_info, rux_dbg_step};
use setup::Chained;

fn main() {
  let args = clip::parse();
  if args.is_present("debug-times") {
    rubx::rux_debug::put_dbg_times();
  }
  if args.is_present("debug-calls") {
    rubx::rux_debug::put_dbg_calls();
  }
  if args.is_present("debug-reavs") {
    rubx::rux_debug::put_dbg_reavs();
  }
  if args.is_present("debug-steps") {
    rubx::rux_debug::put_dbg_steps();
  }
  if args.is_present("debug-tells") {
    rubx::rux_debug::put_dbg_tells();
  }
  if args.is_present("verbose") {
    rubx::rux_debug::put_verbose();
  }
  if args.is_present("archive") {
    rubx::rux_debug::put_archive();
  }

  if args.is_present("delay") {
    let delay = args
      .value_of("delay")
      .expect("Was expected a delay argument.");
    flow::set_delay(delay.parse().expect("Could not parse the delay argument."));
  }
  let input = args
    .value_of("input")
    .expect("You have to pass an input argument.");
  rux_dbg_info!("PChain starting", input);
  let contents =
    std::fs::read_to_string(input).expect("Something went wrong reading the input file.");
  let mut pchain: Vec<Chained> = Vec::new();
  let mut block: Vec<&str> = Vec::new();
  rux_dbg_step!(contents);
  contents.lines().for_each(|line| {
    let line = line.trim();
    if !line.is_empty() {
      if line.starts_with("[") {
        if !block.is_empty() {
          pchain.push(Chained::new(&block));
          block.clear();
        }
      }
      block.push(line);
    }
  });
  if !block.is_empty() {
    pchain.push(Chained::new(&block));
    block.clear();
  }
  runner::start(pchain);
}
