# MIX

This is a computer simulating a MIX computer as described in Donald Knuth's book : The Art of Computer Programming.
Mix Language: https://en.wikipedia.org/wiki/MIX

It is still a work in progress, there may still be bugs. For full transparency, this is a project in which I'm learning
Rust. I expect it to be terrible Rust (unwraps with no regard for error checking, superfluous borrows, etc.). Hopefully,
it will get better as time goes on.

To run the program, write a MIXAL file (examples in "programs"), and run
`.mix run <program-filename> <start-location>`. This will assemble the mixal file to a mix file, and then convert the
mix file to a cardpack.

## Assembling

If you'd like to create a .mix file, you can assemble a .mixal file to a .mix with

`.mix run --from mix <mix-filename> <start-location>`

## Creating a Cardpack
If you have a mix file, you can create a a cardpack with this computer.

Create a program in a hex editor by directly editing a binary file. Each word is 5 bytes (each byte is 64 bits), in addition to a sign.

In binary, a halt instruction might look like this:

`2B 00 00 00 02 05`

Once you have your program written, you need to load it on "punch cards". First you need to create a cardpack (which includes a loading and transfer routine automatically)

`mix create-cardpack <cardpack-name> <program> <start location>`

Pass in the cardpack-name that you want to write to, the program in binary, and the start location (must be greater than 100)

Note that the data cards and transfer cards have every 10 characters preceded by a sign (+ or -) to represent an overpunch on the last digit.

Once you have a hardpack, you can do

`mix run --from cardpack <cardpack-name>`

## Notes on MIXAL

Some notes on the current implementation of MIXAL:

* If no label is supplied, the command has to start with a space " "
* Can't mix'n'match operators yet, it doesn't compute left-to-right
* You can put the text "BEGIN" anywhere you want to indicate the program should start there, otherwise it's the first address
* If you use * as a location before an ORIG statement, it will start at location 100


## Running programs

### Timing

To get timing information (based on cycles), pass `-t` to the `run` invocation

### Tracing

To get show every instruction that gets run, pass `-x` to the `run` invocation

### Debugging

You can single-step through code by passing in the `--debugger` option. You will see the current instruction and
the main registers.

Commands:

* `n` or `next` - single-step to the next instruction
* `q` or `next` - quit the program
* `b <location>` or `breakpoint <location>` - breakpoint at the location
* `c <number of breakpoints to skip>` or `continue <number of breakpoints to skip>` - run until next breakpoint or program halt. If you specify number of breakpoints, it wil skip that many breakpoints being hit before hitting again. If left off, the next breakpoint will hit
* `m <location>` or `memory <location>` - show the memory at specified location
* `B <value>` or `bytes <value>` - Break a value out into each individual byte
* `r` or `reset timing` - Set the timer to zero (useful for profiling specific parts of code)
* `x` or `time` - show the current timer
* `l` or `list` - show the source code around your current location

### Populating input devices

You can supply additional cards (newline separated card per line) with `--data-cards-file`.
If this file is a program, also pass in `--as-program-cards`

You can supply a tape device with `--tape-file`.

## Emulating

Section 1.4.3 in the book provided an incomplete MIX emulator written in MIX
In the `mix/1.4.3/problems`, you will find `emulator_complete.mixal`. Despite its name,
there are a few restrictions:

* You must pass cards in as "program style" to simulate a program
* No floating point numbers
* Not all IO operations are available (only units 16 and 18)
