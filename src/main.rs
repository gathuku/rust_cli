use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    // /// The pattern to look for
    // pattern: String,
    // /// The path to the file to read
    // #[structopt(parse(from_os_str))]
    // path: std::path::PathBuf,
    firstName: String,
    secondName: String,
}


fn main() {
 let args = Cli::from_args();
 print("Your name is"+args.firstName);
}
