use crate::Cmd::{Shanten, Generate};
use riichi_tools_rs::riichi::hand::Hand;
use structopt::StructOpt;
use std::str::FromStr;

/// Riichi tools CLI
#[derive(StructOpt, Debug)]
#[structopt(name = "riichi-tools-cli")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    #[structopt(short, long, default_value = "text")]
    output_type: OutputType,

    #[structopt(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(StructOpt, Debug, PartialEq)]
pub enum Cmd {
    /// Find shanten of a hand
    Shanten { hand: String },
    /// Generate a random hand
    Generate {
        /// Must be complete
        #[structopt(short, long)]
        complete: bool,

        /// How many hands do we generate?
        #[structopt(short, long, default_value = "1")]
        number: i32,
    },
    /// command C
    C {},
}

#[derive(Debug)]
pub enum OutputType {
    Text,
    Json,
}

impl FromStr for OutputType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match &s {
            &"text" => Ok(OutputType::Text),
            &"json" => Ok(OutputType::Json),
            _ => Err("Wrong output type".to_string())
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    // println!("{:#?}", opt);
    let verbose = opt.verbose;
    let output_type = opt.output_type;

    if let Some(command) = opt.cmd {
        // println!("{:#?}", command);

        match command {
            Shanten { hand } => {
                if let Ok(mut h) = Hand::from_text(&hand[..], true) {
                    match output_type {
                        OutputType::Text => {
                            match verbose {
                                0 => println!("{}", h.shanten()),
                                _ => println!("{}: {}", h.to_string(), h.shanten()),
                            }
                        },
                        OutputType::Json => {
                            
                        }
                    }
                } else {
                    println!("invalid");
                }
            },
            Generate {
                complete, number: count,
            } => {
                if complete {
                    match output_type {
                        OutputType::Text => {
                            for _i in 0..count {
                                println!("{}", Hand::random_complete_hand(true, false));
                            }
                        },
                        OutputType::Json => {

                        }
                    }
                } else {
                    match output_type {
                        OutputType::Text => {
                            for _i in 0..count {
                                println!("{}", Hand::random_hand());
                            }
                        },
                        OutputType::Json => {

                        }
                    }
                }
            }
            _ => {}
        }
    }
}
