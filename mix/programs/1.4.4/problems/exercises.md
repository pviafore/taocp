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

3) Write a buffered output (wordout.mixal), and describe layout and beginning/end changes to program.

   One interesting thing to note:
   My first attempt I saw a 3x slowdown over buffered output. I'm adding 13 cycles inside my tight loop now, and that executes 100 times
   This means that I do 1300 extra cycles (plus my 600 (now has a jump) cycles inside my loop) which means I spend ~2000 cycles (roughly) between my outputs.

   Since my IO is set to 1000 cycles per io instruction, and my ~2000 cycles per output loop is about triple my 700 cycles in buffered_output.mixal, it makes sense to me that this happens. I've eliminated the move's, but had to pay a cost for
   putting WORDOUT in a tight loop. I fear this is due to the blocking of one word records  - 100 per block. If I were to
   run 5 word blocks, then I'm not only adding 300-ish instructions per loop, compared to the extra 200 cycles of the move in the
   buffered example. Let's go big and say I am doing 25 blocks or 4 loops per output cycle -> this means I now have about 50 extra
   cycles, which is a 4x improvement over the move method. (Don't take my word for this, these are guesses - measure, measure, measure).

   Of course, this is entirely negated because as I make bigger blocks, I need more instructions moving memory into buffers (which gets
   back to the move problem).

   The one benefit of this is readability and understandability - if you are working with one word records, it makes sense to only get
   one word at a time. You can also stop in the middle of a record (is this only useful for the last record though?) Beware the cost of
   time, but don't discount readability either.

   The other benefit is that your IO device is not sitting idle at all as your compute gets larger, but at some point, you may be artificially inflating compute time while waiting for IO (and this can introduce power/energy concerns for low-power devices). If you're going to wait the time for your I/O device anyway, then this may be a slight improvement, because you aren't waiting on the moves between I/O. However, if your compute time is slower than I/O time (like my example with 1000 cycles per I/O), then you won't see the benefit (especially if the buffer swapping is enough to push it over the edge). This matters more the more output you do.

   Moral of the story: don't inflate compute past your IO time if you don't have to,
   and don't let your IO sit idle - it will typically be your long pole.

    Also I might just be totally misunderstanding something, so we'll see what I may have missed. I wonder if its a bit of an
    apples and oranges comparison though.

4) With a single IO device, you can at most cut your running time in half.

    Unbuffered time (UB) = Compute Time (C) + IO Time (T) = 1 for the purposes of this ratio
    Buffered time (B) = max(C,T)

    If C > T, then B = 1-T = C. However, the ratio of B/UB is now at 1-T/1 C is larger than T in this case, so as C gets bigger,
    T gets smaller, and this number approaches 1. As T gets bigger (it can never go more than .5 since C is bigger), at most, this
    can be at 1/2. This argument is symmetrical if you flip T and C in this example.

    However, if C == T, you get B = .5 (C and T must equal .5 of B). This gives us the ratio 2:1 or 1/2, the ideal scenario.

    Also woo, I finally solved a problem with an "M" indicating math on it.

5) Generalize the previous case to N IO devices

    Well in this case, your unbuffered time is C + T0 + T1 + T2 where Tn is the n-th numbered IO device.

    If we say that any one device takes the largest time, we have a similar case where we have 1-Tx:1 is our ratio, where
    Tx is the sum of all devices that aren't taking the longest time which is going to approach 1 as one device dominates.
    If they are all equal though, you see 1:n as the ratio because Tx will become (n-1+1)t, which means 1-Tx is just n+1*t:1,
    or 1/n+1 (the n+1 is the n devices plus compute time)


6)  What instructions must be put at the beginning of WORDIN (in the book)

    We have to set I6 to the buffer we want to fill, and at least start populating it

```
    IN INBUF1(U)
    JBUS * (U)    * get the initial buffer filled
    ENT6 INBUF-1  * we increment first thing in wordin
    JMP WORDIN
```

    According to knuth, he just does a

```
    IN INBUF1(U)
    ENT6 INBUF2+99
```

    This works because we immediately have a sentinel value which will populate the other buffer,
    blocking until the first read completes

7)
