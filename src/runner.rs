use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Command, Stdio};
use std::thread::{self, JoinHandle};

use rubx::{rux_dbg_call, rux_dbg_step};

use crate::flow::Chaining;
use crate::setup::Chained;
use crate::setup::PassOn;
use crate::setup::PassTo;

pub fn start(ichain: Vec<Chained>) {
  rux_dbg_call!(ichain);
  let chaining = Chaining::new();
  rux_dbg_step!(chaining);
  let mut handles: Vec<JoinHandle<()>> = Vec::new();
  for chained in ichain {
    let chaining_cloned = chaining.clone();
    handles.push(thread::spawn(move || execute(chained, chaining_cloned)));
  }
  rux_dbg_step!(handles);
  for handle in handles {
    handle.join().unwrap();
  }
}

fn execute(chained: Chained, chaining: Chaining) {
  rux_dbg_call!(chained, chaining);
  let stocking = chaining.add(&chained.name);
  let mut command = Command::new(&chained.name);
  chained.ways.iter().for_each(|(to, on)| {
    rux_dbg_step!(to, on);
    if to == &PassTo::Param {
      match on {
        &PassOn::DirectLike(ref value) => {
          rux_dbg_step!(value);
          command.arg(value);
        }
        &PassOn::ExpectAllErrOf(ref name) => {
          rux_dbg_step!(name);
          command.arg(chaining.get_all_err_of_lined(name));
        }
        &PassOn::ExpectEachErrOf(ref name) => {
          rux_dbg_step!(name);
          for argument in chaining.get_all_err_of_on_vec(name) {
            command.arg(argument);
          }
        }
        &PassOn::ExpectNthErrOf(ref nth, ref of) => {
          rux_dbg_step!(nth, of);
          command.arg(chaining.get_nth_err_of(of, *nth));
        }
        &PassOn::ExpectAllOutOf(ref name) => {
          rux_dbg_step!(name);
          command.arg(chaining.get_all_out_of_lined(name));
        }
        &PassOn::ExpectEachOutOf(ref name) => {
          rux_dbg_step!(name);
          for argument in chaining.get_all_out_of_on_vec(name) {
            command.arg(argument);
          }
        }
        &PassOn::ExpectNthOutOf(ref nth, ref of) => {
          rux_dbg_step!(nth, of);
          command.arg(chaining.get_nth_out_of(of, *nth));
        }
      }
    }
  });

  let child = command
    .stdin(if chained.has_inputs() {
      rux_dbg_step!();
      Stdio::piped()
    } else {
      rux_dbg_step!();
      Stdio::null()
    })
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .unwrap();

  let write_in = if chained.has_inputs() {
    let stdin = child.stdin.unwrap();
    let mut writer = BufWriter::new(stdin);
    Some(thread::spawn(move || {
      for (to, on) in chained.ways {
        rux_dbg_step!(to, on);
        if to == PassTo::Input {
          rux_dbg_step!(to);
          let get_inputs = GetInputs {
            _of: chaining.clone(),
            got: 0,
            on,
          };
          rux_dbg_step!(get_inputs);
          for line in get_inputs.into_iter() {
            rux_dbg_step!(line);
            writer.write(line.as_bytes()).unwrap();
            writer.write("\n".as_bytes()).unwrap();
          }
        }
      }
    }))
  } else {
    None
  };
  rux_dbg_step!(write_in);

  let read_err = {
    let stderr = child.stderr.unwrap();
    let stocking = stocking.clone();
    thread::spawn(move || {
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
  };
  rux_dbg_step!(read_err);

  let read_out = {
    let stdout = child.stdout.unwrap();
    let stocking = stocking.clone();
    thread::spawn(move || {
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
  };

  if let Some(write_in) = write_in {
    write_in.join().unwrap();
  }
  read_err.join().unwrap();
  read_out.join().unwrap();
  stocking.set_done();
}

#[derive(Debug)]
struct GetInputs {
  _of: Chaining,
  got: usize,
  on: PassOn,
}

impl Iterator for GetInputs {
  type Item = String;
  fn next(&mut self) -> Option<String> {
    match self.on {
      PassOn::DirectLike(ref value) => {
        if self.got == 0 {
          self.got += 1;
          return value.clone().into();
        } else {
          return None;
        }
      }
      _ => {
        eprintln!("Get Input from {:?} not supported yet", self.on);
      }
    }
    None
  }
}
