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
    pub args: Vec<String>,
}

impl Setup {
    pub fn new(block: &Vec<&str>) -> Self {
        let mut name = String::new();
        let mut args = Vec::new();
        block.iter().for_each(|line| {
            if line.starts_with("[") {
                name.push_str(line.trim_start_matches("[").trim_end_matches("]"));
            } else {
                args.push(String::from(*line));
            }
        });
        Self { name, args }
    }
}
