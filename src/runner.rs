use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};

use crate::setup::Chained;
use crate::setup::PassOn;
use crate::setup::PassTo;

use rubx::{rux_dbg_call, rux_dbg_reav, rux_dbg_step};

pub fn start(chaineds: Vec<Chained>) {
    rux_dbg_call!(chaineds);
    let results: Results = Arc::new(RwLock::new(Vec::new()));
    rux_dbg_step!(results);
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for chained in chaineds {
        let results = results.clone();
        handles.push(thread::spawn(move || execute(chained, results)));
    }
    rux_dbg_step!(handles);
    for handle in handles {
        handle.join().unwrap();
    }
}

type Results = Arc<RwLock<Vec<Processing>>>;

type Processing = Arc<RwLock<Process>>;

#[derive(Debug)]
struct Process {
    name: String,
    errs: Vec<String>,
    outs: Vec<String>,
    done: bool,
}

fn execute(chained: Chained, results: Results) {
    rux_dbg_call!(chained, results);
    let processing = Arc::new(RwLock::new(Process {
        name: chained.name.clone(),
        errs: Vec::new(),
        outs: Vec::new(),
        done: false,
    }));
    rux_dbg_step!(processing);
    results.write().unwrap().push(processing.clone());
    rux_dbg_step!(results);
    let mut command = Command::new(chained.name.clone());
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
                    command.arg(get_all_err_of_lined(name, results.clone()));
                }
                &PassOn::ExpectEachErrOf(ref name) => {
                    rux_dbg_step!(name);
                    for argument in get_all_err_of_vected(name, results.clone()) {
                        command.arg(argument);
                    }
                }
                &PassOn::ExpectNthErrOf(ref nth, ref of) => {
                    rux_dbg_step!(nth, of);
                    command.arg(get_nth_err_of(nth, of, results.clone()));
                }
                &PassOn::ExpectAllOutOf(ref name) => {
                    rux_dbg_step!(name);
                    command.arg(get_all_out_of_lined(name, results.clone()));
                }
                &PassOn::ExpectEachOutOf(ref name) => {
                    rux_dbg_step!(name);
                    for argument in get_all_out_of_vected(name, results.clone()) {
                        command.arg(argument);
                    }
                }
                &PassOn::ExpectNthOutOf(ref nth, ref of) => {
                    rux_dbg_step!(nth, of);
                    command.arg(get_nth_out_of(nth, of, results.clone()));
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
                        of: results.clone(),
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
        let process = processing.clone();
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
                    process.write().unwrap().errs.push(err_line.clone());
                }
            }
        })
    };
    rux_dbg_step!(read_err);

    let read_out = {
        let stdout = child.stdout.unwrap();
        let process = processing.clone();
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
                    process.write().unwrap().outs.push(out_line.clone());
                }
            }
        })
    };

    if let Some(write_in) = write_in {
        write_in.join().unwrap();
    }
    read_err.join().unwrap();
    read_out.join().unwrap();
    processing.write().unwrap().done = true;
}

fn get_all_err_of_lined(name: &str, results: Results) -> String {
    rux_dbg_call!(name, results);
    loop {
        for process in results.read().unwrap().iter() {
            let reader = process.read().unwrap();
            if reader.name == name {
                rux_dbg_step!(reader.done);
                if reader.done {
                    rux_dbg_reav!(reader.errs.join(" "));
                }
            }
        }
        rux_dbg_step!();
        thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn get_all_err_of_vected(name: &str, results: Results) -> Vec<String> {
    rux_dbg_call!(name, results);
    loop {
        for process in results.read().unwrap().iter() {
            let reader = process.read().unwrap();
            if reader.name == name {
                rux_dbg_step!(reader.done);
                if reader.done {
                    rux_dbg_reav!(reader.errs.clone());
                }
            }
        }
        rux_dbg_step!();
        thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn get_nth_err_of(nth: &usize, name: &str, results: Results) -> String {
    rux_dbg_call!(nth, name, results);
    loop {
        for process in results.read().unwrap().iter() {
            let reader = process.read().unwrap();
            if reader.name == name {
                rux_dbg_step!(reader.errs.len());
                if reader.errs.len() > *nth {
                    rux_dbg_reav!(reader.errs[*nth].clone());
                }
                rux_dbg_step!(reader.done);
                if reader.done {
                    eprintln!("Nth {} Err of {} will never come.", nth, name);
                    rux_dbg_reav!(String::new());
                }
            }
        }
        rux_dbg_step!();
        thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn get_all_out_of_lined(name: &str, results: Results) -> String {
    rux_dbg_call!(name, results);
    loop {
        for process in results.read().unwrap().iter() {
            let reader = process.read().unwrap();
            if reader.name == name {
                rux_dbg_step!(reader.done);
                if reader.done {
                    rux_dbg_reav!(reader.outs.join(" "));
                }
            }
        }
        rux_dbg_step!();
        thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn get_all_out_of_vected(name: &str, results: Results) -> Vec<String> {
    rux_dbg_call!(name, results);
    loop {
        for process in results.read().unwrap().iter() {
            let reader = process.read().unwrap();
            if reader.name == name {
                rux_dbg_step!(reader.done);
                if reader.done {
                    rux_dbg_reav!(reader.outs.clone());
                }
            }
        }
        rux_dbg_step!();
        thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn get_nth_out_of(nth: &usize, name: &str, results: Results) -> String {
    rux_dbg_call!(nth, name, results);
    loop {
        for process in results.read().unwrap().iter() {
            let reader = process.read().unwrap();
            if reader.name == name {
                rux_dbg_step!(reader.errs.len());
                if reader.outs.len() > *nth {
                    rux_dbg_reav!(reader.outs[*nth].clone());
                }
                rux_dbg_step!(reader.done);
                if reader.done {
                    eprintln!("Nth {} Out of {} will never come.", nth, name);
                    rux_dbg_reav!(String::new());
                }
            }
        }
        rux_dbg_step!();
        thread::sleep(std::time::Duration::from_millis(10));
    }
}

impl Chained {
    fn has_inputs(&self) -> bool {
        for (to, _) in &self.ways {
            if to == &PassTo::Input {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
struct GetInputs {
    of: Results,
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
