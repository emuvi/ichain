use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};

use crate::setup::Chained;
use crate::setup::PassOn;
use crate::setup::PassTo;

pub fn start(chaineds: Vec<Chained>) {
    let results: Results = Arc::new(RwLock::new(Vec::new()));
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for chained in chaineds {
        let results = results.clone();
        handles.push(thread::spawn(move || execute(chained, results)));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

type Results = Arc<RwLock<Vec<Processing>>>;

type Processing = Arc<RwLock<Process>>;

struct Process {
    name: String,
    errs: Vec<String>,
    outs: Vec<String>,
    done: bool,
}

fn execute(chained: Chained, results: Results) {
    let processing = Arc::new(RwLock::new(Process {
        name: chained.name.clone(),
        errs: Vec::new(),
        outs: Vec::new(),
        done: false,
    }));
    results.write().unwrap().push(processing.clone());
    let mut command = Command::new(chained.name.clone());
    chained.ways.iter().for_each(|(to, on)| {
        if to == &PassTo::Param {
            match on {
                &PassOn::DirectLike(ref value) => {
                    command.arg(value);
                }
                &PassOn::ExpectAllErrOf(ref name) => {
                    command.arg(get_all_err_of_lined(name, results.clone()));
                }
                &PassOn::ExpectEachErrOf(ref name) => {
                    for argument in get_all_err_of_vected(name, results.clone()) {
                        command.arg(argument);
                    }
                }
                &PassOn::ExpectNthErrOf(ref nth, ref of) => {
                    command.arg(get_nth_err_of(nth, of, results.clone()));
                }
                &PassOn::ExpectAllOutOf(ref name) => {
                    command.arg(get_all_out_of_lined(name, results.clone()));
                }
                &PassOn::ExpectEachOutOf(ref name) => {
                    for argument in get_all_out_of_vected(name, results.clone()) {
                        command.arg(argument);
                    }
                }
                &PassOn::ExpectNthOutOf(ref nth, ref of) => {
                    command.arg(get_nth_out_of(nth, of, results.clone()));
                }
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
        Some(thread::spawn(move || {
            for (to, on) in chained.ways {
                if to == PassTo::Input {
                    let get_inputs = GetInputs {
                        of: results.clone(),
                        got: 0,
                        on,
                    };
                    for line in get_inputs.into_iter() {
                        writer.write(line.as_bytes()).unwrap();
                        writer.write("\n".as_bytes()).unwrap();
                    }
                }
            }
        }))
    } else {
        None
    };

    let read_err = {
        let stderr = child.stderr.unwrap();
        let process = processing.clone();
        thread::spawn(move || {
            let mut reader = BufReader::new(stderr);
            let mut err_line = String::new();
            loop {
                err_line.clear();
                let size = reader.read_line(&mut err_line).unwrap();
                if size == 0 {
                    break;
                } else {
                    process.write().unwrap().errs.push(err_line.clone());
                }
            }
        })
    };

    let read_out = {
        let stdout = child.stdout.unwrap();
        let process = processing.clone();
        thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            let mut out_line = String::new();
            loop {
                out_line.clear();
                let size = reader.read_line(&mut out_line).unwrap();
                if size == 0 {
                    break;
                } else {
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
    loop {
        for process in results.read().unwrap().iter() {
            let reader = process.read().unwrap();
            if reader.name == name {
                if reader.done {
                    return reader.errs.join(" ");
                }
            }
        }
        thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn get_all_err_of_vected(name: &str, results: Results) -> Vec<String> {
    loop {
        for process in results.read().unwrap().iter() {
            let reader = process.read().unwrap();
            if reader.name == name {
                if reader.done {
                    return reader.errs.clone();
                }
            }
        }
        thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn get_nth_err_of(nth: &usize, name: &str, results: Results) -> String {
    loop {
        for process in results.read().unwrap().iter() {
            let reader = process.read().unwrap();
            if reader.name == name {
                if reader.errs.len() > *nth {
                    return reader.errs[*nth].clone();
                }
                if reader.done {
                    panic!("Nth Err {} of {} will never come.", nth, name);
                }
            }
        }
        thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn get_all_out_of_lined(name: &str, results: Results) -> String {
    loop {
        for process in results.read().unwrap().iter() {
            let reader = process.read().unwrap();
            if reader.name == name {
                if reader.done {
                    return reader.outs.join(" ");
                }
            }
        }
        thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn get_all_out_of_vected(name: &str, results: Results) -> Vec<String> {
    loop {
        for process in results.read().unwrap().iter() {
            let reader = process.read().unwrap();
            if reader.name == name {
                if reader.done {
                    return reader.outs.clone();
                }
            }
        }
        thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn get_nth_out_of(nth: &usize, name: &str, results: Results) -> String {
    loop {
        for process in results.read().unwrap().iter() {
            let reader = process.read().unwrap();
            if reader.name == name {
                if reader.outs.len() > *nth {
                    return reader.outs[*nth].clone();
                }
                if reader.done {
                    panic!("Nth Out {} of {} will never come.", nth, name);
                }
            }
        }
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
