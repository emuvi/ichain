mod clip;

fn main() {
    let args = clip::parse();
    let input = args
        .value_of("input")
        .expect("You have to pass an input argument.");
    let contents =
        std::fs::read_to_string(input).expect("Something went wrong reading the input file.");
    println!("{}", contents);
}
