use clap::Parser;

/// Describe project here
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Describe argument here
    arg: String,
}

fn main() {
    // parse command line arguments
    let args = Cli::parse();
    // print argument
    run(args.arg);
}

/// Describe function here
fn run(arg: String) {
    println!("Argument: {}", arg);
}
