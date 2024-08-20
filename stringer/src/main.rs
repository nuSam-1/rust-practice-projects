mod api;

use clap::{Parser, Args, command, Subcommand};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Reverse(name)) => {
            match name.string {
                Some(ref _name) => {
                    let reverse = api::stringer::reverse(_name);
                    println!("{}", reverse);
                }
                None => {
                    println!("Please provide a string to reverse");
                }
            }
        }
        Some(Commands::Inspect(name)) => {
            match name.string {
                Some(ref _name) => {
                    let (res, kind) = api::stringer::inspect(_name, name.only_digits);
                    let mut plural_s = "s";
                    if res == 1 {
                        plural_s = "";
                    }

                    println!("{:?} has {} {}{}.", _name, res, kind, plural_s)
                }
                None => {
                    println!("Please provide a string to reverse");
                }
            }
        }
        None => {}
    }
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "A simple CLI to transform and inspect strings")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Reverse(Reverse),
    Inspect(Inspect)
}

/// Reverse a string
#[derive(Args)]
struct Reverse {
    /// The string to reverse
    string: Option<String>
}

/// Inspect a string
#[derive(Args)]
struct Inspect {
    /// The string to inspect
    string: Option<String>,
    /// Only count digits in the input string
    #[arg(short = 'd', long = "digits")]
    only_digits: bool
}

