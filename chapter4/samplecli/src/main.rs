use clap::{App, Arg};

fn main() {
    let matches = App::new("My RPN Program")
        .version("1.0.0")
        .author("okaponta")
        .about("Super awesome sample RPN calculator")
        .arg(
            Arg::with_name("formula_file")
                .help("Formulas written in RPN")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Sets the level of verbosity")
                .short("v")
                .long("verbose")
                .required(false),
        )
        .get_matches();

    match matches.value_of("formula_file") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified"),
    }
    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", verbose);
}
