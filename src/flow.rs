use rubx::{rux_dbg_call, rux_dbg_reav, rux_dbg_step};

use std::sync::{Arc, RwLock};

use crate::setup::PassOn;

#[derive(Clone, Debug)]
pub struct Chaining {
  pace: Arc<RwLock<Vec<Stocking>>>,
}

#[derive(Clone, Debug)]
pub struct Stocking {
  data: Arc<RwLock<Stock>>,
}

#[derive(Debug)]
struct Stock {
  name: String,
  errs: Vec<String>,
  outs: Vec<String>,
  done: bool,
}

impl Chaining {
  pub fn new() -> Chaining {
    Chaining {
      pace: Arc::new(RwLock::new(Vec::new())),
    }
  }

  pub fn add(&self, name: &str) -> Stocking {
    let result = Stocking {
      data: Arc::new(RwLock::new(Stock {
        name: name.to_string(),
        errs: Vec::new(),
        outs: Vec::new(),
        done: false,
      })),
    };
    self.pace.write().unwrap().push(result.clone());
    result
  }

  fn get_stocking(&self, name: &str) -> Option<Stocking> {
    self
      .pace
      .read()
      .unwrap()
      .iter()
      .find(|stocking| {
        let stock = stocking.data.read().unwrap();
        stock.name == name
      })
      .cloned()
  }

  pub fn get_all_err_of_lined(&self, name: &str) -> String {
    rux_dbg_call!(self, name);
    loop {
      for stocking in self.pace.read().unwrap().iter() {
        let reader = stocking.data.read().unwrap();
        if reader.name == name {
          rux_dbg_step!(reader.done);
          if reader.done {
            rux_dbg_reav!(reader.errs.join(" "));
          }
        }
      }
      rux_dbg_step!();
      std::thread::sleep(std::time::Duration::from_millis(3));
    }
  }

  pub fn get_all_err_of_on_vec(&self, name: &str) -> Vec<String> {
    rux_dbg_call!(self, name);
    loop {
      for stocking in self.pace.read().unwrap().iter() {
        let reader = stocking.data.read().unwrap();
        if reader.name == name {
          rux_dbg_step!(reader.done);
          if reader.done {
            rux_dbg_reav!(reader.errs.clone());
          }
        }
      }
      rux_dbg_step!();
      std::thread::sleep(std::time::Duration::from_millis(3));
    }
  }

  pub fn get_nth_err_of(&self, name: &str, nth: usize) -> String {
    rux_dbg_call!(self, name, nth);
    loop {
      for stocking in self.pace.read().unwrap().iter() {
        let reader = stocking.data.read().unwrap();
        if reader.name == name {
          rux_dbg_step!(reader.errs.len());
          if nth < reader.errs.len() {
            rux_dbg_reav!(reader.errs[nth].clone());
          }
          rux_dbg_step!(reader.done);
          if reader.done {
            eprintln!("Nth {} Err of {} will never come.", nth, name);
            rux_dbg_reav!(String::new());
          }
        }
      }
      rux_dbg_step!();
      std::thread::sleep(std::time::Duration::from_millis(3));
    }
  }

  pub fn get_all_out_of_lined(&self, name: &str) -> String {
    rux_dbg_call!(self, name);
    loop {
      for stocking in self.pace.read().unwrap().iter() {
        let reader = stocking.data.read().unwrap();
        if reader.name == name {
          rux_dbg_step!(reader.done);
          if reader.done {
            rux_dbg_reav!(reader.outs.join(" "));
          }
        }
      }
      rux_dbg_step!();
      std::thread::sleep(std::time::Duration::from_millis(3));
    }
  }

  pub fn get_all_out_of_on_vec(&self, name: &str) -> Vec<String> {
    rux_dbg_call!(self, name);
    loop {
      for stocking in self.pace.read().unwrap().iter() {
        let reader = stocking.data.read().unwrap();
        if reader.name == name {
          rux_dbg_step!(reader.done);
          if reader.done {
            rux_dbg_reav!(reader.outs.clone());
          }
        }
      }
      rux_dbg_step!();
      std::thread::sleep(std::time::Duration::from_millis(3));
    }
  }

  pub fn get_nth_out_of(&self, name: &str, nth: usize) -> String {
    rux_dbg_call!(self, name, nth);
    loop {
      for stocking in self.pace.read().unwrap().iter() {
        let reader = stocking.data.read().unwrap();
        if reader.name == name {
          rux_dbg_step!(reader.outs.len());
          if nth < reader.outs.len() {
            rux_dbg_reav!(reader.outs[nth].clone());
          }
          rux_dbg_step!(reader.done);
          if reader.done {
            eprintln!("Nth {} Out of {} will never come.", nth, name);
            rux_dbg_reav!(String::new());
          }
        }
      }
      rux_dbg_step!();
      std::thread::sleep(std::time::Duration::from_millis(3));
    }
  }

  pub fn get_from(&self, pass: PassOn) -> GetFrom {
    GetFrom {
      from: self.clone(),
      pass,
      done: false,
      got: 0,
    }
  }
}

impl Stocking {
  pub fn put_err(&self, err: &str) {
    rux_dbg_call!(self, err);
    let mut writer = self.data.write().unwrap();
    writer.errs.push(err.to_string());
  }

  pub fn put_out(&self, out: &str) {
    rux_dbg_call!(self, out);
    let mut writer = self.data.write().unwrap();
    writer.outs.push(out.to_string());
  }

  pub fn set_done(&self) {
    rux_dbg_call!(self);
    let mut writer = self.data.write().unwrap();
    writer.done = true;
    rux_dbg_step!(writer.done);
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
    if self.done {
      rux_dbg_reav!(None);
    }
    match &self.pass {
      PassOn::DirectLike(value) => {
        self.done = true;
        rux_dbg_reav!(value.clone().into());
      }
      PassOn::ExpectAllOutOf(name) => match self.from.get_stocking(name) {
        Some(stocking) => loop {
          let reader = stocking.data.read().unwrap();
          if reader.done {
            self.done = true;
            rux_dbg_reav!(reader.errs.join(" "));
          }
          std::thread::sleep(std::time::Duration::from_millis(3));
        },
        None => {
          eprintln!("Get from chaining with name: {} not found.", name);
          self.done = true;
          rux_dbg_reav!(None);
        }
      },
      PassOn::ExpectEachOutOf(name) => match self.from.get_stocking(name) {
        Some(stocking) => loop {
          let reader = stocking.data.read().unwrap();
          if self.got < reader.outs.len() {
            let found = reader.outs[self.got].clone();
            self.got += 1;
            rux_dbg_reav!(Some(found));
          }
          rux_dbg_step!(reader.done);
          if reader.done {
            self.done = true;
            rux_dbg_reav!(None);
          }
          std::thread::sleep(std::time::Duration::from_millis(3));
        },
        None => {
          eprintln!("Get from chaining with name: {} not found.", name);
          self.done = true;
          rux_dbg_reav!(None);
        }
      },
      PassOn::ExpectNthOutOf(nth, name) => match self.from.get_stocking(name) {
        Some(stocking) => loop {
          let reader = stocking.data.read().unwrap();
          if *nth < reader.outs.len() {
            self.done = true;
            rux_dbg_reav!(Some(reader.outs[*nth].clone()));
          }
          rux_dbg_step!(reader.done);
          if reader.done {
            eprintln!("Nth {} Out of {} will never come.", nth, name);
            self.done = true;
            rux_dbg_reav!(None);
          }
          std::thread::sleep(std::time::Duration::from_millis(3));
        },
        None => {
          eprintln!("Get from chaining with name: {} not found.", name);
          self.done = true;
          rux_dbg_reav!(None);
        }
      },
      PassOn::ExpectAllErrOf(name) => match self.from.get_stocking(name) {
        Some(stocking) => loop {
          let reader = stocking.data.read().unwrap();
          if reader.done {
            self.done = true;
            rux_dbg_reav!(reader.errs.join(" "));
          }
          std::thread::sleep(std::time::Duration::from_millis(3));
        },
        None => {
          eprintln!("Get from chaining with name: {} not found.", name);
          self.done = true;
          rux_dbg_reav!(None);
        }
      },
      PassOn::ExpectEachErrOf(name) => match self.from.get_stocking(name) {
        Some(stocking) => loop {
          let reader = stocking.data.read().unwrap();
          if self.got < reader.errs.len() {
            let found = reader.errs[self.got].clone();
            self.got += 1;
            rux_dbg_reav!(Some(found));
          }
          rux_dbg_step!(reader.done);
          if reader.done {
            self.done = true;
            rux_dbg_reav!(None);
          }
          std::thread::sleep(std::time::Duration::from_millis(3));
        },
        None => {
          eprintln!("Get from chaining with name: {} not found.", name);
          self.done = true;
          rux_dbg_reav!(None);
        }
      },
      PassOn::ExpectNthErrOf(nth, name) => match self.from.get_stocking(name) {
        Some(stocking) => loop {
          let reader = stocking.data.read().unwrap();
          if *nth < reader.errs.len() {
            self.done = true;
            rux_dbg_reav!(Some(reader.errs[*nth].clone()));
          }
          rux_dbg_step!(reader.done);
          if reader.done {
            eprintln!("Nth {} Err of {} will never come.", nth, name);
            self.done = true;
            rux_dbg_reav!(None);
          }
          std::thread::sleep(std::time::Duration::from_millis(3));
        },
        None => {
          eprintln!("Get from chaining with name: {} not found.", name);
          self.done = true;
          rux_dbg_reav!(None);
        }
      },
    }
  }
}