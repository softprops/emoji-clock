# emoji clock [![Build Status](https://travis-ci.org/softprops/emoji-clock.svg?branch=master)](https://travis-ci.org/softprops/emoji-clock)

> 🕒 🐇 an emoji clock to make sure you're never late

[![](https://upload.wikimedia.org/wikipedia/commons/f/f3/De_Alice%27s_Abenteuer_im_Wunderland_Carroll_pic_02.jpg)](https://en.wikipedia.org/wiki/White_Rabbit)

Long ago people used clocks to tell time. These clocks had faces 😯 with hands 👐 and people
could easily read them at a glance. Minutes mattered less than hours. These were mostly
replaced with digial clocks with numbers. Today, we have emoji!

## 📦 install

### cargo

Run the following to have the `emojiclock` binary installed under `~/.cargo/bin`

```
$ cargo install emoji-clock-cli
```

## 🤸 usage

The following is the output of the cli's help flag. Enjoy.

```sh
$ emojiclock --help
emoji-clock 0.0.0
softprops <d.tangren@gmail.com>
renders time as an emoji clock

USAGE:
    emojiclock [FLAGS] [time]

FLAGS:
    -c, --copy        Copies to clipboard (where possible)
    -h, --help        Prints help information
    -m, --meridiem    Adds an indicator for what half of the day this time falls within (ante meridiem or post meridiem)
    -V, --version     Prints version information

ARGS:
    <time>    An expression similar to expression that can be provided to GNU date -d expr -
              http://man7.org/linux/man-pages/man1/date.1.html#DATE_STRING [default: now]
```

### specifying time

By default, `emojiclock` will print out the emoji clock closet to "now" but you can also provide
a time that's relative to "now" in roughly english form ( as supported by the [chrono-english crate](https://crates.io/crates/chrono-english) ) as an argument.

```sh
for hour in `seq 1 12`; do emojiclock "$hour hours"; done
🕡
🕢
🕣
🕣
🕣
🕦
🕧
🕜
🕝
🕞
🕟
🕠
```

### clipboard

It's often the case that you don't just want to see the current time in emoji but
you want to capture it to use elsewhere. Use the `-c` ( or `--copy` ) flag to copy the output
to your clipboard

```sh
$ emojiclock -c
# paste it somewhere nice
```

### which 12 hours

Sometimes having context for a.m. or p.m is useful. You can always open your apartment window
and see for yourself, but when that's not convenient enough pass the `-m` ( or `--meridiem` ) flag to capture that context

```sh
$ emojiclock -m
emojiclock -m
🕠🌙
```

Doug Tangren (softprops) 2018