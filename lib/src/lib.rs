//! Emoji clock... what it says on the tin

extern crate chrono;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate lazy_static;

// Std lib
use std::collections::HashMap;
use std::fmt;

// Third party
use chrono::Timelike;

lazy_static! {
    static ref DIALS: HashMap<(u32, bool), char> = hashmap!(
        (1, true) => '🕐',
        (1, false) => '🕜',
        (2, true) => '🕑',
        (2, false) => '🕝',
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

/// Renders a clock in emoji
pub enum Clock<T> {
    /// Dial with 12 hour time
    Dial(T),
    /// Dial with 12 hour time and a.m/p.m indication ( 🌞/🌙 )
    DialMeridiem(T),
}

impl<T> fmt::Display for Clock<T>
where
    T: Timelike,
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            Clock::Dial(time) => write!(
                f,
                "{}",
                DIALS
                    .get(&(time.hour12().1, time.minute() < 30))
                    .cloned()
                    .unwrap_or_else(|| '⌛')
            ),
            Clock::DialMeridiem(time) => {
                let (is_pm, hour) = time.hour12();
                write!(
                    f,
                    "{}{}",
                    DIALS
                        .get(&(hour, time.minute() < 30))
                        .cloned()
                        .unwrap_or_else(|| '⌛'),
                    if is_pm { '🌙' } else { '🌞' }
                )
            }
        }
    }
}
