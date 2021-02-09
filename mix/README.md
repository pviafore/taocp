# MIX

This is a computer simulating a MIX computer as described in Donald Knuth's book : The Art of Computer Programming.

It is still a work in progress.

To run a program:

Create a program in a hex editor by directly editing a binary file. Each word is 5 bytes (each byte is 64 bits), in addition to a sign.

In binary, a halt instruction might look like this:

`2B 00 00 00 02 05`

Once you have your program written, you need to load it on "punch cards". Firsty ou need to create a cardpack (which includes a loading and transfer routine automatically)

`mix create-cardpack <cardpack-name> <program> <start location>`

Pass in the cardpack-name that you want to write to, the program in binary, and the start location (must be greater than 100)

Note that the data cards and transfer cards have every 10 characters preceded by a sign (+ or -) to represent an overpunch on the last digit.

Once you have a hardpack, you can do 

`mix run <cardpack-name>`
