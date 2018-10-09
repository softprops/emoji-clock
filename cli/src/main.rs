extern crate chrono;
extern crate emoji_clock;
#[macro_use]
extern crate structopt;
extern crate chrono_english;

// Third party
use chrono::Local;
use chrono_english::{parse_date_string, Dialect};
use structopt::StructOpt;

// Ours
use emoji_clock::Clock;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(
    name = "emoji-clock",
    about = "renders time as an emoji clock"
)]
struct Options {
    #[structopt(
        help = "an expression similar to GNU date -d expr - http://man7.org/linux/man-pages/man1/date.1.html#DATE_STRING. defaults to 'now'",
        default_value = "now"
    )]
    time: String,
    #[structopt(
        short = "c",
        long = "ctx",
        help = "adds a sun/moon indicator for what half of the day this time falls within"
    )]
    ctx: bool,
}

fn main() {
    match Options::from_args() {
        Options { time, ctx } => match parse_date_string(&time, Local::now(), Dialect::Us) {
            Ok(wound) => println!(
                "{}",
                if ctx {
                    Clock::DialCtx(wound)
                } else {
                    Clock::Dial(wound)
                }
            ),
            Err(err) => eprintln!("{}", err),
        },
    }
}
