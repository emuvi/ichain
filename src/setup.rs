#[derive(Debug)]
pub struct Chained {
  pub name: String,
  pub alias: String,
  pub times: usize,
  pub ways: Vec<(PassTo, PassOn)>,
}

impl Chained {
  pub fn new(block: &Vec<&str>) -> Self {
    let mut name = String::new();
    let mut alias = String::new();
    let mut times = 1;
    let mut ways = Vec::new();
    block.iter().for_each(|line| {
      if line.starts_with("[") {
        let program_body = line.trim_start_matches("[").trim_end_matches("]");
        let body_colon = program_body.find(":");
        let body_asterisk = program_body.rfind("*");
        if let Some(body_colon) = body_colon {
          if let Some(body_asterisk) = body_asterisk {
            name = program_body[..body_colon].to_string();
            alias = program_body[body_colon + 1..body_asterisk].to_string();
            times = program_body[body_asterisk + 1..]
              .parse()
              .expect("Could not parse the times of a program.");
          } else {
            name = program_body[..body_colon].to_string();
            alias = program_body[body_colon + 1..].to_string();
          }
        } else {
          if let Some(body_asterisk) = body_asterisk {
            name = program_body[..body_asterisk].to_string();
            alias = name.clone();
            times = program_body[body_asterisk + 1..]
              .parse()
              .expect("Could not parse the times of a program.");
          } else {
            name = program_body.to_string();
            alias = name.clone();
          }
        }
      } else if line.starts_with(">") {
        let line = &line[1..].trim();
        for on in get_ways_on(line) {
          ways.push((PassTo::Param, on));
        }
      } else if line.starts_with("|") {
        let line = &line[1..].trim();
        for on in get_ways_on(line) {
          ways.push((PassTo::Input, on));
        }
      }
    });
    Chained {
      name,
      alias,
      times,
      ways,
    }
  }

  pub fn has_inputs(&self) -> bool {
    for (to, _) in &self.ways {
      if to == &PassTo::Input {
        return true;
      }
    }
    false
  }
}

#[derive(Debug, PartialEq)]
pub enum PassTo {
  Param,
  Input,
}

#[derive(Clone, Debug)]
pub enum PassOn {
  DirectLike(String),
  ExpectAllOutOf(PassFrom),
  ExpectEachOutOf(PassFrom),
  ExpectForkOutOf(PassFrom),
  ExpectNthOutOf(usize, PassFrom),
  ExpectAllErrOf(PassFrom),
  ExpectEachErrOf(PassFrom),
  ExpectForkErrOf(PassFrom),
  ExpectNthErrOf(usize, PassFrom),
}

#[derive(Clone, Debug)]
pub struct PassFrom {
  pub name: String,
  pub time: usize,
}

fn get_ways_on(of_setup_line: &str) -> Vec<PassOn> {
  let mut results = Vec::new();
  let mut making_actual = String::new();
  let mut inside_quotes = false;
  let mut escaped_char = false;
  for actual_char in of_setup_line.chars() {
    if inside_quotes {
      if escaped_char {
        if actual_char == 'n' {
          making_actual.push('\n');
        } else if actual_char == 'r' {
          making_actual.push('\n');
        } else if actual_char == 't' {
          making_actual.push('\t');
        } else {
          making_actual.push(actual_char)
        }
        escaped_char = false;
      } else if actual_char == '\\' {
        escaped_char = true;
      } else if actual_char == '"' {
        try_push_new_pass_on(&making_actual, &mut results);
        making_actual.clear();
      } else {
        making_actual.push(actual_char);
      }
    } else {
      if actual_char == '"' {
        inside_quotes = true;
      } else if actual_char == ' ' {
        try_push_new_pass_on(&making_actual, &mut results);
        making_actual.clear();
      } else {
        making_actual.push(actual_char);
      }
    }
  }
  try_push_new_pass_on(&making_actual, &mut results);
  results
}

fn try_push_new_pass_on(of_setup_part: &str, on_results: &mut Vec<PassOn>) {
  if of_setup_part.is_empty() {
    return;
  }
  if of_setup_part.starts_with("$") {
    let body = &of_setup_part[1..];
    let body_slash = body.find("/");
    let body_dot = body.find(".");
    let body_colon = body.find(":");
    let name_bound = if let Some(ref body_slash) = body_slash {
      *body_slash
    } else if let Some(ref body_dot) = body_dot {
      *body_dot
    } else if let Some(ref body_colon) = body_colon {
      *body_colon
    } else {
      body.len()
    };
    let name = body[..name_bound].to_string();
    let mut time = 1;
    if let Some(body_slash) = body_slash {
      let slash_bound = if let Some(ref body_dot) = body_dot {
        *body_dot
      } else if let Some(ref body_colon) = body_colon {
        *body_colon
      } else {
        body.len()
      };
      time = body[body_slash + 1..slash_bound]
        .parse()
        .expect("Could not parse the time of a way.");
    }
    let mut what = "all";
    if let Some(body_dot) = body_dot {
      let dot_bound = if let Some(ref body_colon) = body_colon {
        *body_colon
      } else {
        body.len()
      };
      what = &body[body_dot + 1..dot_bound];
    }
    let mut from = "out";
    if let Some(body_colon) = body_colon {
      let colon_bound = body.len();
      from = &body[body_colon + 1..colon_bound];
    }
    if what == "all" {
      if from == "err" {
        on_results.push(PassOn::ExpectAllErrOf(PassFrom { name, time }));
      } else {
        on_results.push(PassOn::ExpectAllOutOf(PassFrom { name, time }));
      }
    } else if what == "each" {
      if from == "err" {
        on_results.push(PassOn::ExpectEachErrOf(PassFrom { name, time }));
      } else {
        on_results.push(PassOn::ExpectEachOutOf(PassFrom { name, time }));
      }
    } else if what == "fork" {
      if from == "err" {
        on_results.push(PassOn::ExpectForkErrOf(PassFrom { name, time }));
      } else {
        on_results.push(PassOn::ExpectForkOutOf(PassFrom { name, time }));
      }
    } else {
      let nth = what
        .parse::<usize>()
        .expect("Could not parse Nth argument.");
      if from == "err" {
        on_results.push(PassOn::ExpectNthErrOf(nth, PassFrom { name, time }));
      } else {
        on_results.push(PassOn::ExpectNthOutOf(nth, PassFrom { name, time }));
      }
    }
  } else {
    on_results.push(PassOn::DirectLike(String::from(of_setup_part)));
  }
}
