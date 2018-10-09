extern crate chrono;
#[macro_use]
extern crate maplit;

use chrono::{Local, Timelike};

fn main() {
    let now = Local::now();
    let dials = hashmap!(
        (1, true) => '🕐',
        (1, false) => '🕑',
        (2, true) => '🕝',
        (2, false) => '🕝',
        (3, true) => '🕒',
        (3, false) => '🕕',
        (4, true) => '🕓',
        (4, false) => '🕟',
        (5, true) => '🕔',
        (5, false) => '🕠',
        (6, true) => '🕕',
        (6, false) => '🕕',
        (7, true) => '🕖',
        (7, false) => '🕖',
        (8, true) => '🕗',
        (8, false) => '🕣',
        (9, true) => '🕘',
        (9, false) => '🕣',
        (10, true) => '🕙',
        (10, false) => '🕣',
        (11, true) => '🕚',
        (11, false) => '🕦',
        (12, true) =>> '🕛',
        (12, false) =>> '🕧'
    );
    println!(
        "{}",
        dials
            .get(&(now.hour12().1, now.minute() < 30))
            .cloned()
            .unwrap_or_else(|| '⌛')
    )
}
