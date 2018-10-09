extern crate chrono;
extern crate emoji_clock;
#[macro_use]
extern crate structopt;
extern crate chrono_english;
extern crate clipboard;

// Third party
use chrono::Local;
use chrono_english::{parse_date_string, Dialect};
use clipboard::{ClipboardContext, ClipboardProvider};
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
        help = "An expression similar to expression that can be provided to GNU date -d expr - http://man7.org/linux/man-pages/man1/date.1.html#DATE_STRING",
        default_value = "now"
    )]
    time: String,
    #[structopt(
        short = "m",
        long = "meridiem",
        help = "Adds an indicator for what half of the day this time falls within (ante meridiem or post meridiem)"
    )]
    meridiem: bool,
    #[structopt(
        short = "c",
        long = "copy",
        help = "Copies to clipboard (where possible)"
    )]
    copy: bool,
}

fn main() {
    match Options::from_args() {
        Options {
            time,
            meridiem,
            copy,
        } => match parse_date_string(&time, Local::now(), Dialect::Us) {
            Ok(wound) => {
                let dial = if meridiem {
                    Clock::DialMeridiem(wound)
                } else {
                    Clock::Dial(wound)
                };
                if copy {
                    if let Ok(mut clipboard) = ClipboardContext::new() {
                        drop(clipboard.set_contents(format!("{}", dial)));
                    }
                }
                println!("{}", dial);
            }
            Err(err) => eprintln!("{}", err),
        },
    }
}
