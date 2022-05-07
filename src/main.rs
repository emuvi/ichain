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
        let clean = line.trim();
        if !clean.is_empty() {
            if clean.starts_with("[") {
                if !block.is_empty() {
                    setups.push(Setup::new(&block));
                    block.clear();
                }
            }
            block.push(clean);
        }
    });
    if !block.is_empty() {
        setups.push(Setup::new(&block));
        block.clear();
    }
    setups.iter().for_each(|setup| {
        println!("{:?}", setup);
    });
}

#[derive(Debug)]
struct Setup {
    pub name: String,
    pub pass: Vec<Pass>,
}

impl Setup {
    pub fn new(block: &Vec<&str>) -> Self {
        let mut name = String::new();
        let mut pass = Vec::new();
        block.iter().for_each(|line| {
            if line.starts_with("[") {
                name.push_str(line.trim_start_matches("[").trim_end_matches("]"));
            } else {
                let mut is_input = true;
                let line = if line.starts_with(">") {
                    is_input = false;
                    &line[1..]
                } else if line.starts_with("|") {
                    &line[1..]
                } else {
                    &line[..]
                };
                if is_input {
                    pass.push(Pass::InputDirectLike(String::from(line)));
                } else {
                    pass.push(Pass::ParamDirectLike(String::from(line)));
                }
            }
        });
        Setup { name, pass }
    }
}

#[derive(Debug)]
enum Pass {
    ParamDirectLike(String),
    ParamExpectAllOf(String),
    ParamExpectEachOf(String),
    ParamExpectNthOf(u32, String),
    InputDirectLike(String),
    InputExpectAllOf(String),
    InputExpectEachOf(String),
    InputExpectNthOf(u32, String),
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
