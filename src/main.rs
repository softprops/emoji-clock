extern crate emoji_clock;
extern crate chrono;

use emoji_clock::Clock;
use chrono::Local;

fn main() {
    println!("{}", Clock::Dial(Local::now()))
}
