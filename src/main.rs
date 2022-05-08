mod clip;
mod exec;
mod setup;

use setup::Chained;

fn main() {
    let args = clip::parse();
    let input = args
        .value_of("input")
        .expect("You have to pass an input argument.");
    let contents =
        std::fs::read_to_string(input).expect("Something went wrong reading the input file.");
    let mut chaineds: Vec<Chained> = Vec::new();
    let mut block: Vec<&str> = Vec::new();
    contents.lines().for_each(|line| {
        let line = line.trim();
        if !line.is_empty() {
            if line.starts_with("[") {
                if !block.is_empty() {
                    chaineds.push(Chained::new(&block));
                    block.clear();
                }
            }
            block.push(line);
        }
    });
    if !block.is_empty() {
        chaineds.push(Chained::new(&block));
        block.clear();
    }
    if args.is_present("verbose") {
        println!("Chained:");
        chaineds.iter().for_each(|setup| {
            println!("Name: {}", setup.name);
            println!("Ways: {:?}", setup.ways);
        });
    }
    exec::start(chaineds);
}
