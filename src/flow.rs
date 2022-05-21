use rubx::{rux_dbg_call, rux_dbg_ifis, rux_dbg_lets, rux_dbg_muts, rux_dbg_reav};

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use crate::setup::PassFrom;
use crate::setup::PassOn;

static DELAY: AtomicU64 = AtomicU64::new(10);

pub fn get_delay() -> u64 {
  rux_dbg_call!();
  rux_dbg_reav!(DELAY.load(Ordering::Acquire));
}

pub fn set_delay(millis: u64) {
  rux_dbg_call!(millis);
  rux_dbg_reav!(DELAY.store(millis, Ordering::Release));
}

pub fn sleep_delay() {
  rux_dbg_call!();
  rux_dbg_reav!(std::thread::sleep(std::time::Duration::from_millis(
    get_delay()
  )));
}

#[derive(Clone, Debug)]
pub struct Chaining {
  pace: Arc<RwLock<Vec<Stocking>>>,
}

#[derive(Clone, Debug)]
pub struct Stocking {
  data: Arc<RwLock<Stock>>,
  fork: Arc<RwLock<Forked>>,
}

#[derive(Debug)]
struct Stock {
  alias: String,
  time: usize,
  errs: Vec<String>,
  outs: Vec<String>,
  done: bool,
}

#[derive(Debug)]
struct Forked {
  out: usize,
  err: usize,
}

impl Chaining {
  pub fn new() -> Chaining {
    rux_dbg_call!();
    rux_dbg_reav!(Chaining {
      pace: Arc::new(RwLock::new(Vec::new())),
    })
  }

  pub fn add(&self, alias: String, time: usize) -> Stocking {
    rux_dbg_call!(self, alias);
    let stocking = Stocking {
      data: Arc::new(RwLock::new(Stock {
        alias,
        time,
        errs: Vec::new(),
        outs: Vec::new(),
        done: false,
      })),
      fork: Arc::new(RwLock::new(Forked { out: 0, err: 0 })),
    };
    self.pace.write().unwrap().push(stocking.clone());
    rux_dbg_reav!(stocking)
  }

  fn get_stocking(&self, from: &PassFrom) -> Option<Stocking> {
    rux_dbg_call!(self, from);
    rux_dbg_reav!(self
      .pace
      .read()
      .unwrap()
      .iter()
      .find(|stocking| {
        let stock = stocking.data.read().unwrap();
        stock.alias == from.alias && stock.time == from.time
      })
      .cloned())
  }

  pub fn get_from(&self, pass: PassOn) -> GetFrom {
    rux_dbg_call!(self, pass);
    rux_dbg_reav!(GetFrom {
      from: self.clone(),
      pass,
      done: false,
      got: 0,
    })
  }
}

impl Stocking {
  pub fn put_err(&self, err: &str) {
    rux_dbg_call!(self, err);
    let mut data_writer = rux_dbg_lets!(self.data.write().unwrap());
    data_writer.errs.push(err.to_string());
    rux_dbg_reav!(());
  }

  pub fn put_out(&self, out: &str) {
    rux_dbg_call!(self, out);
    let mut data_writer = rux_dbg_lets!(self.data.write().unwrap());
    data_writer.outs.push(out.to_string());
    rux_dbg_reav!(());
  }

  pub fn set_done(&self) {
    rux_dbg_call!(self);
    let mut data_writer = rux_dbg_lets!(self.data.write().unwrap());
    rux_dbg_muts!(data_writer.done, true);
    rux_dbg_reav!(());
  }
}

#[derive(Debug)]
pub struct GetFrom {
  from: Chaining,
  pass: PassOn,
  done: bool,
  got: usize,
}

impl Iterator for GetFrom {
  type Item = String;
  fn next(&mut self) -> Option<String> {
    rux_dbg_call!(self);
    if rux_dbg_ifis!(self.done) {
      rux_dbg_reav!(None);
    }
    match &self.pass {
      PassOn::DirectLike(value) => {
        rux_dbg_muts!(self.done, true);
        rux_dbg_reav!(value.clone().into());
      }
      PassOn::ExpectAllOutOf(from) => match self.from.get_stocking(from) {
        Some(stocking) => loop {
          let reader = stocking.data.read().unwrap();
          if rux_dbg_ifis!(reader.done) {
            rux_dbg_muts!(self.done, true);
            rux_dbg_reav!(Some(reader.outs.join(" ")));
          }
          sleep_delay();
        },
        None => {
          eprintln!(
            "Could not get the chaining of alias {} and time {}.",
            from.alias, from.time
          );
          rux_dbg_muts!(self.done, true);
          rux_dbg_reav!(None);
        }
      },
      PassOn::ExpectEachOutOf(from) => match self.from.get_stocking(from) {
        Some(stocking) => loop {
          let reader = stocking.data.read().unwrap();
          if rux_dbg_ifis!(self.got < reader.outs.len()) {
            let found = rux_dbg_lets!(reader.outs[self.got].clone());
            rux_dbg_muts!(self.got, self.got + 1);
            rux_dbg_reav!(Some(found));
          }
          if rux_dbg_ifis!(reader.done) {
            rux_dbg_muts!(self.done, true);
            rux_dbg_reav!(None);
          }
          sleep_delay();
        },
        None => {
          eprintln!(
            "Could not get the chaining of alias {} and time {}.",
            from.alias, from.time
          );
          rux_dbg_muts!(self.done, true);
          rux_dbg_reav!(None);
        }
      },
      PassOn::ExpectForkOutOf(from) => match self.from.get_stocking(from) {
        Some(stocking) => loop {
          let mut fork_writer = stocking.fork.write().unwrap();
          let data_reader = stocking.data.read().unwrap();
          if rux_dbg_ifis!(fork_writer.out < data_reader.outs.len()) {
            let found = rux_dbg_lets!(data_reader.outs[fork_writer.out].clone());
            rux_dbg_muts!(fork_writer.out, fork_writer.out + 1);
            rux_dbg_reav!(Some(found));
          }
          if rux_dbg_ifis!(data_reader.done) {
            rux_dbg_muts!(self.done, true);
            rux_dbg_reav!(None);
          }
          sleep_delay();
        },
        None => {
          eprintln!(
            "Could not get the chaining of alias {} and time {}.",
            from.alias, from.time
          );
          rux_dbg_muts!(self.done, true);
          rux_dbg_reav!(None);
        }
      },
      PassOn::ExpectNthOutOf(nth, from) => match self.from.get_stocking(from) {
        Some(stocking) => loop {
          let reader = stocking.data.read().unwrap();
          if rux_dbg_ifis!(*nth < reader.outs.len()) {
            rux_dbg_muts!(self.done, true);
            rux_dbg_reav!(Some(reader.outs[*nth].clone()));
          }
          if rux_dbg_ifis!(reader.done) {
            eprintln!(
              "Nth {} Out of {} and time {} will never come.",
              nth, from.alias, from.time
            );
            rux_dbg_muts!(self.done, true);
            rux_dbg_reav!(None);
          }
          sleep_delay();
        },
        None => {
          eprintln!(
            "Could not get the chaining of alias {} and time {}.",
            from.alias, from.time
          );
          rux_dbg_muts!(self.done, true);
          rux_dbg_reav!(None);
        }
      },
      PassOn::ExpectAllErrOf(from) => match self.from.get_stocking(from) {
        Some(stocking) => loop {
          let reader = stocking.data.read().unwrap();
          if rux_dbg_ifis!(reader.done) {
            rux_dbg_muts!(self.done, true);
            rux_dbg_reav!(Some(reader.errs.join(" ")));
          }
          sleep_delay();
        },
        None => {
          eprintln!(
            "Could not get the chaining of alias {} and time {}.",
            from.alias, from.time
          );
          rux_dbg_muts!(self.done, true);
          rux_dbg_reav!(None);
        }
      },
      PassOn::ExpectEachErrOf(from) => match self.from.get_stocking(from) {
        Some(stocking) => loop {
          let reader = stocking.data.read().unwrap();
          if rux_dbg_ifis!(self.got < reader.errs.len()) {
            let found = rux_dbg_lets!(reader.errs[self.got].clone());
            rux_dbg_muts!(self.got, self.got + 1);
            rux_dbg_reav!(Some(found));
          }
          if rux_dbg_ifis!(reader.done) {
            rux_dbg_muts!(self.done, true);
            rux_dbg_reav!(None);
          }
          sleep_delay();
        },
        None => {
          eprintln!(
            "Could not get the chaining of alias {} and time {}.",
            from.alias, from.time
          );
          rux_dbg_muts!(self.done, true);
          rux_dbg_reav!(None);
        }
      },
      PassOn::ExpectForkErrOf(from) => match self.from.get_stocking(from) {
        Some(stocking) => loop {
          let mut fork_writer = stocking.fork.write().unwrap();
          let data_reader = stocking.data.read().unwrap();
          if rux_dbg_ifis!(fork_writer.err < data_reader.errs.len()) {
            let found = rux_dbg_lets!(data_reader.errs[fork_writer.err].clone());
            rux_dbg_muts!(fork_writer.err, fork_writer.err + 1);
            rux_dbg_reav!(Some(found));
          }
          if rux_dbg_ifis!(data_reader.done) {
            rux_dbg_muts!(self.done, true);
            rux_dbg_reav!(None);
          }
          sleep_delay();
        },
        None => {
          eprintln!(
            "Could not get the chaining of alias {} and time {}.",
            from.alias, from.time
          );
          rux_dbg_muts!(self.done, true);
          rux_dbg_reav!(None);
        }
      },
      PassOn::ExpectNthErrOf(nth, from) => match self.from.get_stocking(from) {
        Some(stocking) => loop {
          let reader = stocking.data.read().unwrap();
          if rux_dbg_ifis!(*nth < reader.errs.len()) {
            rux_dbg_muts!(self.done, true);
            rux_dbg_reav!(Some(reader.errs[*nth].clone()));
          }
          if rux_dbg_ifis!(reader.done) {
            eprintln!(
              "Nth {} Err of {} and time {} will never come.",
              nth, from.alias, from.time
            );
            rux_dbg_muts!(self.done, true);
            rux_dbg_reav!(None);
          }
          sleep_delay();
        },
        None => {
          eprintln!(
            "Could not get the chaining of alias {} and time {}.",
            from.alias, from.time
          );
          rux_dbg_muts!(self.done, true);
          rux_dbg_reav!(None);
        }
      },
    }
  }
}
