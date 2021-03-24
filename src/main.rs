use crate::Cmd::{Generate, Score, Shanten, Ukeire};
use riichi_tools_rs::riichi::hand::Hand;
use riichi_tools_rs::riichi::rules::{GameLength, Rules};
use riichi_tools_rs::riichi::table::Table;
use serde_json::{json, Map};
use structopt::clap::AppSettings;
use structopt::StructOpt;

/// Riichi tools CLI
#[derive(StructOpt, Debug)]
#[structopt(name = "riichi-tools-cli", global_settings = &[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp])]
struct RiichiToolsCLI {
    /// Verbosity level - changes how much stuff is written out.
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Output in json instead of random text
    #[structopt(short, long)]
    json: bool,

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
        number: u32,

        /// Generate complete hands
        #[structopt(short, long)]
        complete: bool,

        /// Also include shanten
        #[structopt(short, long)]
        shanten: bool,
    },
    /// Find ukeire of a hand
    Ukeire { hand: String },
    /// Score information for a hand
    Score {
        hand: String,

        /// Show hand points
        #[structopt(short, long)]
        points: bool,

        /// Show han and fu
        #[structopt(short, long)]
        han_fu: bool,

        /// Show yaku names
        #[structopt(short, long)]
        yaku: bool,

        /// Is the hand in riichi?
        #[structopt(short, long)]
        riichi: bool,

        /// Did I selfdraw the hand?
        #[structopt(short, long)]
        tsumo: bool,

        /// My wind.
        /// e = east
        /// s = south
        /// w = west
        /// n = north
        #[structopt(short, long, default_value = "e")]
        my_wind: char,

        /// Prevalent wind.
        /// e = east
        /// s = south
        /// w = west
        /// n = north
        #[structopt(short, long, default_value = "e")]
        wind: char,
    },
}

#[derive(Debug)]
pub enum OutputType {
    Text,
    Json,
}

fn main() {
    let app = RiichiToolsCLI::from_args();
    let verbose = app.verbose;
    let output_type = if app.json {
        OutputType::Json
    } else {
        OutputType::Text
    };

    if let Some(command) = app.cmd {
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
                            println!("{{\n  \"hands\": [");

                            for i in 0..count {
                                if shanten {
                                    println!(
                                        "    [\n      \"hand\": \"{}\"{}\n    ]{}",
                                        Hand::random_complete_hand(true, false),
                                        format!(",\n      \"shanten\": {}", -1),
                                        if i < count - 1 { "," } else { "" }
                                    );
                                } else {
                                    println!(
                                        "    \"{}\"{}",
                                        Hand::random_complete_hand(true, false),
                                        if i < count - 1 { "," } else { "" }
                                    );
                                }
                            }

                            println!("  ]\n}}");
                        }
                    }
                } else {
                    match output_type {
                        OutputType::Text => {
                            for _i in 0..count {
                                let mut hand = Hand::random_hand(Some(&Rules {
                                    game_length: GameLength::Tonpuusen,
                                    aka_ari: true,
                                    kuitan_ari: false,
                                }));
                                println!(
                                    "{}{}",
                                    hand.to_string(),
                                    if shanten {
                                        format!(" {}", hand.shanten())
                                    } else {
                                        String::from("")
                                    }
                                );
                            }
                        }
                        OutputType::Json => {
                            println!("{{\n  \"hands\": [");

                            for i in 0..count {
                                let mut hand = Hand::random_hand(Some(&Rules {
                                    game_length: GameLength::Tonpuusen,
                                    aka_ari: true,
                                    kuitan_ari: false,
                                }));
                                if shanten {
                                    println!(
                                        "    [\n      \"hand\": \"{}\"{}\n    ]{}",
                                        hand.to_string(),
                                        format!(",\n      \"shanten\": {}", hand.shanten()),
                                        if i < count - 1 { "," } else { "" }
                                    );
                                } else {
                                    println!(
                                        "    \"{}\"{}",
                                        hand,
                                        if i < count - 1 { "," } else { "" }
                                    );
                                }
                            }

                            println!("  ]\n}}");
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
            Score {
                hand,
                points,
                han_fu,
                yaku,
                riichi,
                tsumo,
                my_wind,
                wind,
            } => {
                if let Ok(mut h) = Hand::from_text(&hand[..], true) {
                    if h.shanten() >= 0 {
                        println!("not complete");
                    } else if !points && !han_fu && !yaku {
                        println!("Set at least one score parameter (points, han_fu or yaku)");
                    } else {
                        let mut table = Table::from_map(&Map::new()).unwrap();
                        table.set_my_hand(h.clone());
                        if riichi {
                            table.set_my_riichi(true);
                        }

                        if tsumo {
                            table.set_my_tsumo(true);
                        }

                        match my_wind {
                            'e' => table.set_my_seat_wind(1),
                            's' => table.set_my_seat_wind(2),
                            'w' => table.set_my_seat_wind(3),
                            'n' => table.set_my_seat_wind(4),
                            _ => panic!("Wrong wind - use e,s,w,n"),
                        }

                        match wind {
                            'e' => table.set_prevalent_wind(1),
                            's' => table.set_prevalent_wind(2),
                            'w' => table.set_prevalent_wind(3),
                            'n' => table.set_prevalent_wind(4),
                            _ => panic!("Wrong wind - use e,s,w,n"),
                        }

                        let (yakus, score) = table.yaku().unwrap();

                        if let OutputType::Json = output_type {
                            println!("{{");
                        }

                        match verbose {
                            0 => {}
                            _ => match output_type {
                                OutputType::Text => print!("{};", h),
                                OutputType::Json => println!("  \"hand\": \"{}\",", h),
                            },
                        }

                        if points {
                            match output_type {
                                OutputType::Text => print!("{};", score.total_points()),
                                OutputType::Json => {
                                    println!("  \"points\": {},", score.total_points())
                                }
                            }
                        }

                        if han_fu {
                            match output_type {
                                OutputType::Text => print!("{};{};", score.han, score.fu),
                                OutputType::Json => {
                                    println!("  \"han\": {},\n  \"fu\": {},", score.han, score.fu)
                                }
                            }
                        }

                        if yaku {
                            let count = yakus.len();
                            let mut i = 0;

                            match output_type {
                                OutputType::Text => {
                                    for y in yakus.iter() {
                                        print!(
                                            "\"{}\"{}",
                                            y.get_name(),
                                            if i < count - 1 { "," } else { "" }
                                        );

                                        i += 1;
                                    }
                                }
                                OutputType::Json => {
                                    println!("  \"yaku\": [");

                                    for y in yakus.iter() {
                                        println!(
                                            "    \"{}\"{}",
                                            y.get_name(),
                                            if i < count - 1 { "," } else { "" }
                                        );

                                        i += 1;
                                    }

                                    println!("  ]");
                                }
                            }
                        }

                        if let OutputType::Json = output_type {
                            println!("}}");
                        } else {
                            println!();
                        }
                    }
                } else {
                    println!("invalid");
                }
            }
        }
    }
}
