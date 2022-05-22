use rubx::RubxError;
use rubx::{rux_dbg_call, rux_dbg_lets, rux_dbg_reav, rux_dbg_step};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::process::{ChildStderr, ChildStdin, ChildStdout, Command};

use std::process::Stdio;
use std::sync::Arc;

use crate::flow::Chaining;
use crate::flow::Stocking;
use crate::setup::Chained;
use crate::setup::PassTo;

pub async fn start(pchain: Vec<Chained>) -> Result<(), RubxError> {
  rux_dbg_call!(pchain);
  let chaining = rux_dbg_lets!(Chaining::new());
  let mut handles = Vec::new();
  for chained in pchain {
    let chained_arc = Arc::new(chained);
    for time in 1..=chained_arc.times {
      let chained_cloned = chained_arc.clone();
      let chaining_cloned = chaining.clone();
      handles.push(tokio::spawn(execute(chained_cloned, time, chaining_cloned)));
    }
  }
  for handle in handles {
    handle.await??;
  }
  rux_dbg_reav!(Ok(()));
}

async fn execute(
  chained_arc: Arc<Chained>,
  time: usize,
  chaining: Chaining,
) -> Result<(), RubxError> {
  rux_dbg_call!(chained_arc, chaining);
  let stocking = chaining.add(chained_arc.alias.clone(), time).await;
  let mut command = Command::new(&chained_arc.name);
  for (to, on) in &chained_arc.ways {
    rux_dbg_step!(to, on);
    if to == &PassTo::Param {
      let mut getting = chaining.get_from(on.clone());
      while let Some(passed) = getting.next().await {
        rux_dbg_step!(passed);
        command.arg(passed);
      }
    }
  }

  let mut child = command
    .stdin(if chained_arc.has_inputs() {
      Stdio::piped()
    } else {
      Stdio::null()
    })
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

  let write_in = if chained_arc.has_inputs() {
    let stdin = child.stdin.take().unwrap();
    let chained_clone = chained_arc.clone();
    let chaining_clone = chaining.clone();
    Some(tokio::spawn(write_in(stdin, chained_clone, chaining_clone)))
  } else {
    None
  };
  rux_dbg_step!(write_in);

  let read_out = {
    let stdout = child.stdout.take().unwrap();
    let stocking_clone = stocking.clone();
    tokio::spawn(read_out(stdout, stocking_clone))
  };
  rux_dbg_step!(read_out);

  let read_err = {
    let stderr = child.stderr.take().unwrap();
    let stocking_clone = stocking.clone();
    tokio::spawn(read_err(stderr, stocking_clone))
  };
  rux_dbg_step!(read_err);

  child.wait().await?;
  stocking.set_done().await;
  rux_dbg_reav!(Ok(()));
}

async fn write_in(
  stdin: ChildStdin,
  chained_clone: Arc<Chained>,
  chaining_clone: Chaining,
) -> Result<(), RubxError> {
  let mut writer = BufWriter::new(stdin);
  for (to, on) in &chained_clone.ways {
    rux_dbg_step!(to, on);
    if to == &PassTo::Input {
      rux_dbg_step!(to);
      let mut getting = chaining_clone.get_from(on.clone());
      while let Some(passed) = getting.next().await {
        rux_dbg_step!(passed);
        writer.write(passed.as_bytes()).await?;
      }
    }
  }
  rux_dbg_reav!(Ok(()));
}

async fn read_out(stdout: ChildStdout, stocking_clone: Stocking) -> Result<(), RubxError> {
  let mut reader = BufReader::new(stdout).lines();
  while let Some(line) = reader.next_line().await? {
    rux_dbg_step!(line);
    stocking_clone.put_out(&line).await;
  }
  rux_dbg_reav!(Ok(()));
}

async fn read_err(stderr: ChildStderr, stocking_clone: Stocking) -> Result<(), RubxError> {
  let mut reader = BufReader::new(stderr).lines();
  while let Some(line) = reader.next_line().await? {
    rux_dbg_step!(line);
    stocking_clone.put_err(&line).await;
  }
  rux_dbg_reav!(Ok(()));
}
