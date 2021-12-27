use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "My RPN Program",
    version = "1.0.0",
    author = "okaponta",
    about = "Super awesome sample RPN calculator"
)]
struct Opt {
    // Sets the level of verbosity
    #[structopt(short, long)]
    verbose: bool,

    // Formulas written in RPN
    #[structopt(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opt::from_args();

    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified"),
    }
    println!("Is verbosity specified?: {}", opts.verbose);
}
