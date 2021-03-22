use crate::Cmd::{Generate, Shanten, Ukeire};
use riichi_tools_rs::riichi::hand::Hand;
use serde_json::json;
use std::str::FromStr;
use structopt::StructOpt;

/// Riichi tools CLI
#[derive(StructOpt, Debug)]
#[structopt(name = "riichi-tools-cli")]
struct Opt {
    /// Verbosity level - changes how much stuff is written out.
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
        /// How many hands do we generate?
        #[structopt(default_value = "1")]
        number: i32,

        /// Generate complete hands
        #[structopt(short, long)]
        complete: bool,

        /// Also include shanten
        #[structopt(short, long)]
        shanten: bool,
    },
    /// Find ukeire of a hand
    Ukeire { hand: String },
}

#[derive(Debug)]
pub enum OutputType {
    Text,
    Json,
}

impl FromStr for OutputType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "text" => Ok(OutputType::Text),
            "json" => Ok(OutputType::Json),
            _ => Err("Wrong output type".to_string()),
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    let verbose = opt.verbose;
    let output_type = opt.output_type;

    if let Some(command) = opt.cmd {
        match command {
            Shanten { hand } => {
                if let Ok(mut h) = Hand::from_text(&hand[..], true) {
                    match output_type {
                        OutputType::Text => match verbose {
                            0 => println!("{}", h.shanten()),
                            _ => println!("{}: {}", h.to_string(), h.shanten()),
                        },
                        OutputType::Json => match verbose {
                            0 => {
                                println!("{{ {} }}", h.shanten());
                            }
                            _ => {
                                let json_hand = json!({
                                    "hand": h.to_string(),
                                    "shanten": h.shanten()
                                });
                                println!("{}", json_hand);
                            }
                        },
                    }
                } else {
                    println!("invalid");
                }
            }
            Generate {
                complete,
                number: count,
                shanten,
            } => {
                if complete {
                    match output_type {
                        OutputType::Text => {
                            for _i in 0..count {
                                println!(
                                    "{}{}",
                                    Hand::random_complete_hand(true, false),
                                    if shanten { " -1" } else { "" }
                                );
                            }
                        }
                        OutputType::Json => {
                            println!("{{\n\t\"hands\": [");

                            for i in 0..count {
                                println!(
                                    "\t\t[\n\t\t\t\"hand\": \"{}\"{}\n\t\t]{}",
                                    Hand::random_complete_hand(true, false),
                                    if shanten { format!(",\n\t\t\t\"shanten\": {}", -1) } else { String::from("") },
                                    if i < count - 1 { "," } else { "" }
                                );
                            }

                            println!("\t]\n}}");
                        }
                    }
                } else {
                    match output_type {
                        OutputType::Text => {
                            for _i in 0..count {
                                let mut hand = Hand::random_hand();
                                println!(
                                    "{}{}",
                                    hand.to_string(),
                                    if shanten { format!(" {}", hand.shanten()) } else { String::from("") }
                                );
                            }
                        }
                        OutputType::Json => {
                            println!("{{\n\t\"hands\": [");

                            for i in 0..count {
                                let mut hand = Hand::random_hand();
                                println!(
                                    "\t\t{{\n\t\t\t\"hand\": \"{}\"{}\n\t\t}}{}",
                                    hand.to_string(),
                                    if shanten { format!(",\n\t\t\t\"shanten\": {}", hand.shanten()) } else { String::from("") },
                                    if i < count - 1 { "," } else { "" }
                                );
                            }

                            println!("\t]\n}}");
                        }
                    }
                }
            }
            Ukeire { hand } => {
                if let Ok(mut h) = Hand::from_text(&hand[..], true) {
                    let ukeire = h.find_shanten_improving_tiles(None);
                    match output_type {
                        OutputType::Text => {
                            for (dis_tile_o, imp_tiles, value) in ukeire.iter() {
                                match dis_tile_o {
                                    None => {}
                                    Some(dis_tile) => {
                                        print!("{}: ", dis_tile);
                                    }
                                }

                                print!("{} ", value);

                                for (tile, cnt) in imp_tiles.iter() {
                                    print!("{}({}) ", tile, cnt);
                                }

                                println!();
                            }
                        }
                        OutputType::Json => {
                            println!("{}", serde_json::to_string(&ukeire).unwrap());
                        }
                    }
                } else {
                    println!("invalid");
                }
            }
        }
    }
}
