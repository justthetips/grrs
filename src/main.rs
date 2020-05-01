use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;
use structopt::StructOpt;
use std::fs::File;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli{
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf
}

fn main() {
    let args = Cli::from_args();

    println!("Opening {:#?}", args.path);

    let f = File::open(args.path).unwrap();
    let file = BufReader::new(&f);

    let mut writer = BufWriter::new(std::io::stdout());

    for (_, line) in file.lines().enumerate(){
        let l = line.unwrap();
        if l.contains(&args.pattern) {
            writeln!(writer, "{}", l).unwrap();
        }
        
    }
}

