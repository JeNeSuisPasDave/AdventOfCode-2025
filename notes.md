# notes from AoC 2025

Just 12 days this year. May be do-able for me.

I'll be doing this in Rust and learning Rust at the same time. In the few days prior to Dec 1 I went through the initial 5 chapters of *The Rust Programming Language* book, and will continue to go through the book (goal being one chapter per day) while attempting the AoC challenge.

I'm using helix editor as my coding tool. I've installed the rust-analyzer component to enable LSP support for Rust within helix. I'm using git as my VCS and have a Rust-specific gitignore file. I'll be adding a rustfmt.toml file to each project folder so that I get automatic formatting of the Rust code, and compiler error flaggin within helix.

I've setup an overall git rep for the 2025 challenge. The daily puzzles will be within the `days` subfolder, with each daily puzzle within its own numbered subfolder (i.e., `days/day01`, `days/day02`, *et cetera*).

## Day 01

*Monday, 2025-12-01*

I'm thinking about creating a "Dial" structure with methods for turning left, right, and counting zeros; also a method to reset the dial to a known starting position and reset the zero-counter.

First thing, though is to build the CLI parsing code and file reading code, along with basic error/panic handling. That's done.

Next step is to develop the "Dial" structure. I created the Dial structure but now need to add methods. To do that I need to read the "5.3 Method Syntax" section of the Rust book.

How to initialize a fixed length array with ascending integer values:

``` rust
    let numarray: [u32; 10] = core::array::from_fn(|i| i + 1);
```

How to initialized a vector with ascending integer values:

``` rust
    let numvec: Vec<u32> = Vec::from_iter(0..10);
```

How to assign a u32 from a usize:

``` rust
    let len: u32 = v.len().try_into().unwrap();
```

How to have optional or default arguments in Rust? You can't. But you can implement Default trait. See:

- https://lucamoller.com/posts/2021-08/rust-doesnt-support-default-function-arguments-or-does-it
- https://www.kirillvasiltsov.com/writing/optional-arguments-in-rust/

For now, I'm just going to have a new(len: u32) and new_default() as constructors.

I got Part 1 (Day 1) working and found the correct answer. Moving on to Part 2 ...

Okay, I've captured the specs and the answer to Part 1. The second part requires some changes to zero counts in the Dial implementation.

I finished the second part. It was important to note that some instructions had clicks > 99. So counting the wraps around the dial was important (and the difference between the naive 2210 versus the correct 6358).

The answer was submitted and confirmed to be correct. Done with Day 1. Woot!

## Day 2

*Tuesday, 2025-12-02*

**08:00**

I setup the project with part 1 specs, example data, and input data.

Next I'll add the CLI wrapper and file reading logic, borrowing heavily from Day01.

I got a pattern for a static regex variable from here (uses LazyLock):

https://stackoverflow.com/a/79017001/1392864

I don't know if that is the most idiomatic way of allocating a regex variable once (and using it across multiple function calls). I'd prefer that the regex be scoped to an associate Struct implementation but I don't think Rust allows that. I do hope it is private to the module, at least.

I decided to support UTF-8 files. I then found that built-in support for UTF-8 file IO is not mature, so I've added a simple crate "utf8-chars" that adds character I/O to BufReader.

**10:48**

I've now got the code reading the input file and parsing it into IdRange objects. Next step is to create a function that produces a list of invalid IDs from a range.

**13:58**

I completed Part 1 and submitted the correct solution.
I ran into problems with overflow and needed to accumulate
into a u64 instead of a u32.

**18:04**

I completed Part 2 and submitted the correct solution.

I didn't write any unit tests this time. I probably should have,
but got lucky in that the example data provided a sufficient
test run to catch all the bugs that could have shown up when
processing the "real" input data.

## Day 3

*Wednesday, 2025-01-03*

**07:10**

I setup the project with part 1 specs, example data, and input data.

Next I'll add the CLI wrapper and file reading logic, borrowing heavily from previous days.

**07:29**

Implemented the boiler plate for reading the input file and accumulating the max joltages. Next up is a method for finding the max joltage for each bank.

**08:52**

Implemented and passing unit tests. Next need to test with example data.

**08:55**

It failed with the example data. I got a total of 354 instead of 357. Sigh. Now I need to troubleshoot (and write a unit test for the failing case).

**09:01**

So the problem was that I thought it could pick the batteries in any order ... e.g. "8917" would have a max of 98, but actually it has to be in sequence, so "8917" would have a max of 97.

**09:58**

Solved the problem and added more tests. I got the correct answer from the example data.

taking a break ...

**10:23**

...resuming. I submitted the answer using the downloaded data and was correct. Now updating with the Part 2 specs.

**10:32**

specs updated. taking another break ...

**14:25**

Started work again. Implemented a nice helper function and a much cleaner way of doing it. The new solution allows a choice of the number of batteries to be selected and then selects that number, from the given bank, that will give the largest joltage.

A much more elegant solution than the Part 1 mess. And fewer LOC as well.

It passes unit tests and the example, so I'm ready to try the downloaded data.

**15:49**

I ran the data and posted the answer. It was correct. I shared on bsky.app and the Twit discord.

Woot!

### addendum

Apparently there is an algorithm called "Remove K Digits" that does this. I should investigate it.

## Day 4

*Thursday, 2025-12-04*

**01:34**

I created the `day04` project and added the specs and example input. And then I added CLI boilerplate. Next step is to implement a "grid" struct and populate it from the input file.

**03:07**

I'm working on populating the grid struct, but am getting bogged down trying to report an Err with a formatted string. I can do Err("wat") but not Err("wat {}", some_value). I'll just us a plain string and move on.

**04:07**

I tried creating and using a custom error type. I'm working through that now, but need to take a break. 

see https://notes.kodekloud.com/docs/Rust-Programming/Collections-Error-Handling/Creating-Custom-Error-Types

I added a unit test to produce the error, but the test isn't actually checking that an error took place.

**07:30**

Implemented some unit tests to check for errors when invalid chars in the input file or when inconsistent line lengths are in the input file. And I implemented the code to add all the grid rows found in the input.

**12:40**

Resuming work on the puzzle.

**14:13**

Got implementation done. Unit tests working, and got correct answer with example input. About to try actual input file.

**14:51**

I submitted the Part 1 answer and it was correct. I worked on Part 2, which I was suprised to find not that much of an extension from what I did in Part 1.

Finished the implementation. Learned more about mutability in Rust. It ran a couple seconds on the input data, which makes me think I could have made this more efficient. For now, though, it is enough to have submitted a correct answer for Part 2 and earned my gold stars.

## Day 5

*Friday, 2025-12-05*

**04:29**

Capturing the specs, setting up the project with CLI wrapper.

*Note: about day04, I think the impl would have been faster if I had maintained a list of cells containing rolls and only examined those cells each iteration (obviously removing cells from the list when rolls were removed from the grid). I may try it. Also I should figure out how to time my execution so I can verify performance improvements.*

**09:27**

I finished part 1. I had some clever way of reducing the search space, but it has some kind of bug that prevented me from finding a containing range for some of the ingredient IDs. I got 672 with my clever solution. The correct answer is 701, which I found by doing a brute force search (looking at each range in the order of input).

I'll debug it later. For now I need to move on to Part 2.

**11:21**

Done with part 2. To solve this one I needed to merge overlapping ranges. The most efficient way to do that was during the code that adds each range read from the input. Once ranges are merged, then searching for an id in ranges is faster because the set of merged ranges is much smaller than the original input list of ranges.

I did have to write range copy constructors. I guess I don't really understand ownership and references and lifetimes yet. That is something that I need to get up to speed on.

## Day 6

*Saturday, 2025-12-06*

**04:00**

I'm about to pull the specs. The chatter yesterday was that this is the point where AoC starts to become more computer sciencey and much harder for normal workaday programmers. We shall see.

**05:15**

Created a couple structs to help with this, but then got sidetracked into figuring out how to remove trailing space in helix-editor. Back out of the rabbit hole now and about to try to populate structs from a problem set.

**07:28**

Got the example working. Ran the proper input file and got the correct result. Part 1 solved. This time I put the example input into a unit test so I could run with 'cargo test'.

**10:47**

Finished part 2. I added a couple additional structs and enums to better handle the weird input scheme. Luckily BTreeMap has a double-linked index so I could do iter().rev() to easily handle the right-to-left requirements.

## Day 7

*Sunday, 2025-12-07*

**08:44**

Just finished creating the project and capturing the specs.

**08:59**

Captured the input file and added CLI wrapper and unit test boilerplate to the main.rs. Ready to start solving the puzzle.

**10:46**

Got a working implementation that solves the example correction.

**10:49**

Input file was processed correctly. Moving on to Part 2.

**11:43**

Implemented part 2 and got the correct answer with the example input.

**11:49**

I might have some kind of infinite loop or terrible inefficiency in the code. The code ran long enough that I was compelled to halt it with ctrl-C. I don't know what is going on.

Okay, the code seems to be working but starts getting exponentially slow with each line processed. It becomes noticeable after line 90 or so.

I think the solution will be to just track the count of timelines that have a path through a particular location in the manifold, rather than tracking the paths themselves. Going to refactor into that implementation now.

**12:35**

Implemented the much faster approach, just keep track of the paths through a given point, using a BTreeMap to accumulate the counts by position. I ran the test example and submitted the result; it was correct.

## Day 8

*Monday, 2025-12-08*

**00:29**

Created empty project. Populated specs and example input.

**01:03**

Added CLI and test wrapper. Added puzzle input file.

**02:37**

I couldn't find a Rust crate that would help me with this. `geo` only deals with plane geometry (2-D). And other crates looked dodgey. So I'm creating my own structs and functions to handle this. First thing is a Point struct with a distance_from() function. That done, I think I need to create a Circuit struct and have collections of circuits. The Circuit struct would need to contain an ordered collection of connected points (junction boxes) and produce the endpoints (junction boxes at each end of the circuit) and determine whether a point is already in a circuit.

Anyway, I need to go back to sleep, but I'll consider circuits and finding the nearest points/circuits when I resume work on the puzzle.

**11:39**

Got my idea of how to do this implement in a test. It is doing what I wanted but not getting the correct answer. I think I don't understand the specs correctly. I'm assuming that I can connect only an unconnected box to the beginning or end of another circuit (which might be a circuit of 1 box), but that may be wrong.

**11:57**

I have to say I'm struggling with understanding the Day 8 Part 1 specs. I was assuming a linear circuit, so I'd only connect boxes to the endpoint of the circuit ... but I get the wrong answers. So maybe a "circuit" is just bag of nearby junction boxes, and they might be wired together in any old way. I'm going to try that.

Also, I got a hint about discovered neighbors already being in a circuit count toward the "connections made" counter may be critical. I would never have interpreted it that way. We shall see.

**20:58**

I spent a lot of time on this and haven't solved it (with the example input) yet. I'm close, but I'm getting circuits sized 4,3,2,2,2 instead of 5,4,2,2. Some subtle bug. I need sleep and fresh eyes.

*Tuesday, 2025-12-09*

**04:30**

I just realized I don't need to do the square root to calculation the distance. Which means I don't need any f64 values. It could be that fixing that will eliminate my bug.

**04:55**

I removed the square root distance operation, and switched from f64 to u64 distance units. Unfortunately, the bug remains.
