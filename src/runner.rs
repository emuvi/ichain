use rubx::{rux_dbg_call, rux_dbg_lets, rux_dbg_reav, rux_dbg_step};
use thread_priority::{ThreadBuilder, ThreadPriority};

use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::thread::JoinHandle;

use crate::flow::Chaining;
use crate::setup::Chained;
use crate::setup::PassTo;

pub fn start(pchain: Vec<Chained>) {
  rux_dbg_call!(pchain);
  let chaining = rux_dbg_lets!(Chaining::new());
  let mut handles: Vec<JoinHandle<()>> = Vec::new();
  for chained in pchain {
    let chained_arc = Arc::new(chained);
    for time in 1..=chained_arc.times {
      let chained_cloned = chained_arc.clone();
      let chaining_cloned = chaining.clone();
      handles.push(
        ThreadBuilder::default()
          .name(format!("{}_{}", chained_cloned.alias, time))
          .priority(ThreadPriority::Max)
          .spawn(move |result| {
            rux_dbg_call!(result);
            execute(chained_cloned, time, chaining_cloned)
          })
          .expect("Could not create the thread for the chained."),
      );
    }
  }
  rux_dbg_step!(handles);
  for handle in handles {
    rux_dbg_step!(handle);
    handle.join().unwrap();
  }
  rux_dbg_reav!(());
}

fn execute(chained_arc: Arc<Chained>, time: usize, chaining: Chaining) {
  rux_dbg_call!(chained_arc, chaining);
  let stocking = chaining.add(chained_arc.alias.clone(), time);
  let mut command = Command::new(&chained_arc.name);
  chained_arc.ways.iter().for_each(|(to, on)| {
    rux_dbg_step!(to, on);
    if to == &PassTo::Param {
      for passed in chaining.get_from(on.clone()) {
        rux_dbg_step!(passed);
        command.arg(passed);
      }
    }
  });
  let child = command
    .stdin(if chained_arc.has_inputs() {
      Stdio::piped()
    } else {
      Stdio::null()
    })
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .unwrap();

  let write_in = if chained_arc.has_inputs() {
    let stdin = child.stdin.unwrap();
    let mut writer = BufWriter::new(stdin);
    let chained_clone = chained_arc.clone();
    Some(
      ThreadBuilder::default()
        .name(format!("{}_{}_in", chained_arc.alias, time))
        .priority(ThreadPriority::Min)
        .spawn(move |result| {
          rux_dbg_call!(result);
          for (to, on) in &chained_clone.ways {
            rux_dbg_step!(to, on);
            if to == &PassTo::Input {
              rux_dbg_step!(to);
              for passed in chaining.get_from(on.clone()) {
                rux_dbg_step!(passed);
                writer.write(passed.as_bytes()).unwrap();
              }
            }
          }
          rux_dbg_reav!(());
        })
        .expect(&format!(
          "Could not create the thread {}_{}_in",
          chained_arc.alias, time
        )),
    )
  } else {
    None
  };
  rux_dbg_step!(write_in);

  let read_err = {
    let stderr = child.stderr.unwrap();
    let stocking_clone = stocking.clone();

    ThreadBuilder::default()
      .name(format!("{}_{}_err", chained_arc.alias, time))
      .priority(ThreadPriority::Min)
      .spawn(move |result| {
        rux_dbg_call!(result);
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
            stocking_clone.put_err(&err_line);
          }
        }
        rux_dbg_reav!(());
      })
      .expect(&format!(
        "Could not create the thread {}_{}_err",
        chained_arc.alias, time
      ))
  };
  rux_dbg_step!(read_err);

  let read_out = {
    let stdout = child.stdout.unwrap();
    let stocking_clone = stocking.clone();

    ThreadBuilder::default()
      .name(format!("{}_{}_out", chained_arc.alias, time))
      .priority(ThreadPriority::Max)
      .spawn(move |result| {
        rux_dbg_call!(result);
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
            stocking_clone.put_out(&out_line);
          }
        }
        rux_dbg_reav!(());
      })
      .expect(&format!(
        "Could not create the thread {}_{}_out",
        chained_arc.alias, time
      ))
  };

  rux_dbg_step!(write_in);
  if let Some(write_in) = write_in {
    write_in.join().unwrap();
  }
  rux_dbg_step!(write_in);
  read_err.join().unwrap();
  rux_dbg_step!(read_err);
  read_out.join().unwrap();
  rux_dbg_step!(read_out);
  stocking.set_done();
  rux_dbg_reav!(());
}
