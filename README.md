# riichi-tools-cli

![Build Status](https://github.com/harphield/riichi-tools-cli/workflows/Build,%20test%20and%20Clippy/badge.svg)

A command line tool for riichi mahjong shenanigans.

## Usage

`riichi-tools-cli [FLAGS] [OPTIONS] [SUBCOMMAND]`

### FLAGS:
`-h, --help`       Prints help information
\
`-j, --json`       Output in json instead of random text
\
`-V, --version`    Prints version information
\
`-v, --verbose`    Verbosity level - changes how much stuff is written out

### SUBCOMMANDS:
#### generate
`riichi-tools-cli generate [FLAGS] [number]` Generate a random hand.

FLAGS:
\
`-c, --complete` Generate complete hands
\
`-s, --shanten` Also include shanten

#### shanten
`riichi-tools-cli shanten <hand>` Find shanten of a hand

#### ukeire
`riichi-tools-cli ukeire <hand>` Find ukeire of a hand

#### score
`riichi-tools-cli score [FLAGS] [OPTIONS] <hand>` Score information for a hand

FLAGS:
\
`-h, --han-fu`     Show han and fu\
`-p, --points`     Show hand points\
`-r, --riichi`     Is the hand in riichi?\
`-t, --tsumo`      Did I selfdraw the hand?\
`-y, --yaku`       Show yaku names

OPTIONS:\
`-m, --my-wind <my-wind>`    My wind. e = east s = south w = west n = north [default: e]\
`-w, --wind <wind>`          Prevalent wind. e = east s = south w = west n = north [default: e]
