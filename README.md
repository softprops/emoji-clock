# emoji clock [![Build Status](https://travis-ci.org/softprops/emoji-clock.svg?branch=master)](https://travis-ci.org/softprops/emoji-clock)

> 🕒 🐇 an emoji clock to make sure you're never late

[![](https://upload.wikimedia.org/wikipedia/commons/f/f3/De_Alice%27s_Abenteuer_im_Wunderland_Carroll_pic_02.jpg)](https://en.wikipedia.org/wiki/White_Rabbit)

Long ago people used clocks to tell time. These clocks faces 😯 with hands 👐 and people
could easily read them at a glance. Minutes mattered less than hours. These were mostly
replaced with digial clocks with numbers. Today, we half emoji!

## 📦 install

### cargo

Run the following to have the `emojiclock` binary installed under `~/.cargo/bin`

```
$ cargo install emoji-clock-cli
```

## 🤸 usage

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

Doug Tangren (softprops) 2018