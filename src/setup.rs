#[derive(Debug)]
pub struct Chained {
    pub name: String,
    pub ways: Vec<(PassTo, PassOn)>,
}

impl Chained {
    pub fn new(block: &Vec<&str>) -> Self {
        let mut name = String::new();
        let mut ways = Vec::new();
        block.iter().for_each(|line| {
            if line.starts_with("[") {
                name.push_str(line.trim_start_matches("[").trim_end_matches("]"));
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
        Chained { name, ways }
    }
}

#[derive(Debug, PartialEq)]
pub enum PassTo {
    Param,
    Input,
}

#[derive(Debug)]
pub enum PassOn {
    DirectLike(String),
    ExpectAllOutOf(String),
    ExpectEachOutOf(String),
    ExpectNthOutOf(usize, String),
    ExpectAllErrOf(String),
    ExpectEachErrOf(String),
    ExpectNthErrOf(usize, String),
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
        if let Some(dot) = of_setup_part.find(".") {
            let name = &of_setup_part[1..dot];
            let findFrom = of_setup_part.find(":");
            let (what, from) = if let Some(fromPos) = findFrom {
                (
                    &of_setup_part[dot + 1..fromPos],
                    &of_setup_part[fromPos + 1..],
                )
            } else {
                (&of_setup_part[dot + 1..], "out")
            };
            if what == "all" {
                if from == "err" {
                    on_results.push(PassOn::ExpectAllErrOf(String::from(name)));
                } else {
                    on_results.push(PassOn::ExpectAllOutOf(String::from(name)));
                }
            } else if what == "each" {
                if from == "err" {
                    on_results.push(PassOn::ExpectEachErrOf(String::from(name)));
                } else {
                    on_results.push(PassOn::ExpectEachOutOf(String::from(name)));
                }
            } else {
                let nth = what
                    .parse::<usize>()
                    .expect("Could not parse Nth argument.");
                if from == "err" {
                    on_results.push(PassOn::ExpectNthErrOf(nth, String::from(name)));
                } else {
                    on_results.push(PassOn::ExpectNthOutOf(nth, String::from(name)));
                }
            }
        } else {
            let name = &of_setup_part[1..];
            on_results.push(PassOn::ExpectAllOutOf(String::from(name)));
        }
    } else {
        on_results.push(PassOn::DirectLike(String::from(of_setup_part)));
    }
}
