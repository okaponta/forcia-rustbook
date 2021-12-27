use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "My RPN Program",
    version = "1.0.0",
    author = "okaponta",
    about = "Super awesome sample RPN calculator"
)]
struct Opt {
    #[structopt(short, long, help = "Sets the level of verbosity")]
    verbose: bool,

    #[structopt(name = "FILE", help = "Formulas written in RPN")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opt::from_args();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);

        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
        }
    } else {
        // ファイルを指定しなかった場合
        println!("No file is specified");
    }
}
