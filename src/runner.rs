use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Command, Stdio};
use std::thread::{self, JoinHandle};

use rubx::{rux_dbg_call, rux_dbg_lets, rux_dbg_step};

use crate::flow::Chaining;
use crate::setup::Chained;
use crate::setup::PassTo;

pub fn start(pchain: Vec<Chained>) {
  rux_dbg_call!(pchain);
  let chaining = rux_dbg_lets!(Chaining::new());
  let mut handles: Vec<JoinHandle<()>> = Vec::new();
  for chained in pchain {
    let chaining_cloned = chaining.clone();
    handles.push(
      thread::Builder::new()
        .name(chained.name.clone())
        .spawn(move || execute(chained, chaining_cloned))
        .expect("Could not create the thread for the chained."),
    );
  }
  rux_dbg_step!(handles);
  for handle in handles {
    handle.join().unwrap();
  }
}

fn execute(chained: Chained, chaining: Chaining) {
  rux_dbg_call!(chained, chaining);
  let stocking = chaining.add(&chained.alias, 1);
  let mut command = Command::new(&chained.name);
  chained.ways.iter().for_each(|(to, on)| {
    rux_dbg_step!(to, on);
    if to == &PassTo::Param {
      for passed in chaining.get_from(on.clone()) {
        rux_dbg_step!(passed);
        command.arg(passed);
      }
    }
  });

  let child = command
    .stdin(if chained.has_inputs() {
      Stdio::piped()
    } else {
      Stdio::null()
    })
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .unwrap();

  let write_in = if chained.has_inputs() {
    let stdin = child.stdin.unwrap();
    let mut writer = BufWriter::new(stdin);
    Some(
      thread::Builder::new()
        .name(format!("{}_in", &chained.name))
        .spawn(move || {
          for (to, on) in chained.ways {
            rux_dbg_step!(to, on);
            if to == PassTo::Input {
              rux_dbg_step!(to);
              for passed in chaining.get_from(on) {
                rux_dbg_step!(passed);
                writer.write(passed.as_bytes()).unwrap();
              }
            }
          }
        })
        .expect(&format!("Could not create the thread {}_in", &chained.name)),
    )
  } else {
    None
  };
  rux_dbg_step!(write_in);

  let read_err = {
    let stderr = child.stderr.unwrap();
    let stocking = stocking.clone();
    thread::Builder::new()
      .name(format!("{}_err", &chained.name))
      .spawn(move || {
        let mut reader = BufReader::new(stderr);
        let mut err_line = String::new();
        loop {
          err_line.clear();
          let size = reader.read_line(&mut err_line).unwrap();
          rux_dbg_step!(size);
          if size == 0 {
            break;
          } else {
            rux_dbg_step!(err_line);
            stocking.put_err(&err_line);
          }
        }
      })
      .expect(&format!(
        "Could not create the thread {}_err",
        &chained.name
      ))
  };
  rux_dbg_step!(read_err);

  let read_out = {
    let stdout = child.stdout.unwrap();
    let stocking = stocking.clone();
    thread::Builder::new()
      .name(format!("{}_out", &chained.name))
      .spawn(move || {
        let mut reader = BufReader::new(stdout);
        let mut out_line = String::new();
        loop {
          out_line.clear();
          let size = reader.read_line(&mut out_line).unwrap();
          rux_dbg_step!(size);
          if size == 0 {
            break;
          } else {
            rux_dbg_step!(out_line);
            stocking.put_out(&out_line);
          }
        }
      })
      .expect(&format!(
        "Could not create the thread {}_out",
        &chained.name
      ))
  };

  if let Some(write_in) = write_in {
    write_in.join().unwrap();
  }
  read_err.join().unwrap();
  read_out.join().unwrap();
  stocking.set_done();
}
