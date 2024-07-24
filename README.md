# A Rusty Song Title Scroller

This is a program that uses ```playerctl``` to fetch title metadata from media sources that support [mpris](http://specifications.freedesktop.org/mpris-spec/latest/) and continually print it to stdout.

If the song title is over 30 characters long it will scroll right and left to show the entire title.

> ```max_length``` can not be easily changed yet, sorry. You can try though :/

## Demo
![demo gif](https://github.com/Onkornix/titleScroll/blob/main/demo_gif1.gif)

## Requirements
```rust``` -> (https://www.rust-lang.org/tools/install)

```cargo``` (or ```rustc```)

```playerctl``` -> (https://github.com/altdesktop/playerctl)

## Build
1. ```git clone https://github.com/Onkornix/titleScroll.git```
2. ```cd titleScroll```
3. ```cargo build --release``` or ```rustc -o song_title_scroller src/main.rs```
> if you used ```cargo build```, then the binary will be in ```target/release/song_title_scroller```

## Usage
In my case I am using Xmobar as my system bar which has the builtin command ```CommandReader``` which runs a program and continually displays it's stdout.

You can use it by adding ```Run CommandReader "path/to/song_title_scroller" "alias"``` to your xmobarrc and putting the alias somewhere in your format section.

if you don't use Xmobar, then I don't know the exact way to accomplish this. But there's probably a similar builtin that works the same as ```CommandReader```. Good luck!
