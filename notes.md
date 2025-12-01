# notes from AoC 2025

Just 12 days this year. May be do-able for me.

I'll be doing this in Rust and learning Rust at the same time. In the few days prior to Dec 1 I went through the initial 5 chapters of *The Rust Programming Language* book, and will continue to go through the book (goal being one chapter per day) while attempting the AoC challenge.

I'm using helix editor as my coding tool. I've installed the rust-analyzer component to enable LSP support for Rust within helix. I'm using git as my VCS and have a Rust-specific gitignore file. I'll be adding a rustfmt.toml file to each project folder so that I get automatic formatting of the Rust code, and compiler error flaggin within helix.

I've setup an overall git rep for the 2025 challenge. The daily puzzles will be within the `days` subfolder, with each daily puzzle within its own numbered subfolder (i.e., `days/day01`, `days/day02`, *et cetera*).

## Day 01

I'm thinking about creating a "Dial" structure with methods for turning left, right, and counting zeros; also a method to reset the dial to a known starting position and reset the zero-counter.

First thing, though is to build the CLI parsing code and file reading code, along with basic error/panic handling. That's done.

Next step is to develop the "Dial" structure. I created the Dial structure but now need to add methods. To do that I need to read the "5.3 Method Syntax" section of the Rust book.
