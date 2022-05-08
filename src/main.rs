mod clip;

fn main() {
    let args = clip::parse();
    let input = args
        .value_of("input")
        .expect("You have to pass an input argument.");
    let contents =
        std::fs::read_to_string(input).expect("Something went wrong reading the input file.");
    let mut setups: Vec<Setup> = Vec::new();
    let mut block: Vec<&str> = Vec::new();
    contents.lines().for_each(|line| {
        let line = line.trim();
        if !line.is_empty() {
            if line.starts_with("[") {
                if !block.is_empty() {
                    setups.push(Setup::new(&block));
                    block.clear();
                }
            }
            block.push(line);
        }
    });
    if !block.is_empty() {
        setups.push(Setup::new(&block));
        block.clear();
    }
    if args.is_present("verbose") {
        println!("Chained:");
        setups.iter().for_each(|setup| {
            println!("Name: {}", setup.name);
            println!("Ways: {:?}", setup.ways);
        });
    }
}

#[derive(Debug)]
struct Setup {
    name: String,
    ways: Vec<(PassTo, PassOn)>,
}

impl Setup {
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
        Setup { name, ways }
    }
}

#[derive(Debug)]
enum PassTo {
    Param,
    Input,
}

#[derive(Debug)]
enum PassOn {
    DirectLike(String),
    ExpectAllOf(String),
    ExpectEachOf(String),
    ExpectNthOf(u32, String),
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
            let what = &of_setup_part[dot + 1..];
            if what == "all" {
                on_results.push(PassOn::ExpectAllOf(String::from(name)));
            } else if what == "each" {
                on_results.push(PassOn::ExpectEachOf(String::from(name)));
            } else {
                let nth = what.parse::<u32>().expect("Could not parse Nth argument.");
                on_results.push(PassOn::ExpectNthOf(nth, String::from(name)));
            }
        } else {
            let name = &of_setup_part[1..];
            on_results.push(PassOn::ExpectAllOf(String::from(name)));
        }
    } else {
        on_results.push(PassOn::DirectLike(String::from(of_setup_part)));
    }
}
