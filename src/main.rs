extern crate chrono;
extern crate emoji_clock;
#[macro_use]
extern crate structopt;
extern crate chrono_english;

use chrono_english::{parse_date_string, Dialect};

use chrono::Local;
use structopt::StructOpt;

use emoji_clock::Clock;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(
    name = "emoji-clock",
    about = "renders time as an emoji clock"
)]
struct Options {
    #[structopt(
        help = "an expression similar to GNU date -d expr - http://man7.org/linux/man-pages/man1/date.1.html#DATE_STRING. defaults to 'now'"
    )]
    time: Option<String>,
}

fn main() {
    match Options::from_args() {
        Options { time } => {
            let t = time
                .map(|time| {
                    parse_date_string(&time, Local::now(), Dialect::Us).expect("invalid date")
                })
                .unwrap_or_else(Local::now);
            println!("{}", Clock::Dial(t));
        }
    }
}
