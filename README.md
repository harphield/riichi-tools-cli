# riichi-tools-cli

A command line tool for riichi mahjong shenanigans.

## Usage

`riichi-tools-cli [FLAGS] [OPTIONS] [SUBCOMMAND]`

### FLAGS:
`-h, --help`       Prints help information
\
`-V, --version`    Prints version information
\
`-v, --verbose`    Verbosity level - changes how much stuff is written out

### OPTIONS:
`-o, --output-type <output-type>`   `text` or `json`. Default `text`

### SUBCOMMANDS:
#### generate
`riichi-tools-cli generate [FLAGS] [number]` Generate a random hand.
\
FLAGS:
\
`-c, --complete` Generate complete hands
\
`-s, --shanten` Also include shanten

#### shanten
`riichi-tools-cli shanten <hand>` Find shanten of a hand

#### ukeire
`riichi-tools-cli ukeire <hand>` Find ukeire of a hand