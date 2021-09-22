# Exercises

1) Would the sequence of buffered input still be correct if the `MOVE` instructions were moved around?

Original:
```
ENT1 1000
JBUS *(5)
MOVE 2000(50)
MOVE 2000(50)
IN   2000(5)
```

If the code looked like the following:

```
ENT1 1000
MOVE 2000(50)
MOVE 2000(50)
JBUS *(5)
IN   2000(5)
```

This is moving memory from 2000 to 1000, but there is no guarantee that the previous
input has actually finished, which means that you may be moving memory that has not
finished being written yet.

If you moved the `MOVE` commands after the `IN`, like so

```
ENT1 1000
JBUS *(5)
IN   2000(5)
MOVE 2000(50)
MOVE 2000(50)
```

You end up potentially grabbing the input getting loaded immediately after `IN` (you have no guarantee that
it hasn't already started populating the buffer)

2) Write a buffered output (see buffered_output.mixal). With just a simple write of 100 numbers to output,
   I see a saving of 300 cycles (out of the 1000) per loop. This makes sense because we spend roughly 500
   cycles per loop, but then add another 200+ with the moves
