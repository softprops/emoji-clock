//! Emoji clock... what it says on the tin

extern crate chrono;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fmt;

use chrono::{Local, Timelike};

/// Renders a clock in emoji
pub enum Clock {
    Dial(char),
    DialCtx(char, char),
}

impl Clock {
    /// Return a 12 hour clock dial
    pub fn new<T>(time: &T) -> Clock
    where
        T: Timelike,
    {
        Clock::Dial(
            DIALS
                .get(&(time.hour12().1, time.minute() < 30))
                .cloned()
                .unwrap_or_else(|| '⌛'),
        )
    }

    /// Return a 12 hour clock dial with indication of the before/after
    /// midnight
    pub fn with_ctx<T>(time: &T) -> Clock
    where
        T: Timelike,
    {
        let (is_pm, hour) = time.hour12();
        Clock::DialCtx(
            DIALS
                .get(&(hour, time.minute() < 30))
                .cloned()
                .unwrap_or_else(|| '⌛'),
            if is_pm { '🌙' } else { '🌞' },
        )
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Clock::Dial(face) => write!(f, "{}", face),
            Clock::DialCtx(face, ctx) => write!(f, "{}{}", face, ctx),
        }
    }
}

lazy_static! {
    static ref DIALS: HashMap<(u32, bool), char> = hashmap!(
        (1, true) => '🕐',
        (1, false) => '🕜',
        (2, true) => '🕑',
        (2, false) => '🕑',
        (3, true) => '🕒',
        (3, false) => '🕞',
        (4, true) => '🕓',
        (4, false) => '🕟',
        (5, true) => '🕔',
        (5, false) => '🕠',
        (6, true) => '🕕',
        (6, false) => '🕡',
        (7, true) => '🕖',
        (7, false) => '🕢',
        (8, true) => '🕗',
        (8, false) => '🕣',
        (9, true) => '🕘',
        (9, false) => '🕣',
        (10, true) => '🕙',
        (10, false) => '🕣',
        (11, true) => '🕚',
        (11, false) => '🕦',
        (12, true) => '🕛',
        (12, false) => '🕧'
    );
}

fn main() {
    println!("{}", Clock::with_ctx(&Local::now()))
}
