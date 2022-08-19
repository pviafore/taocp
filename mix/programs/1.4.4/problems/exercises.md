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

7) Write a WORDIN subroutine that does not use a sentinel value to accomplish its goals (see wordin_nosentinel.mixal)
    Instead of looking for a sentinel value just check when the index is past the buffer. The calling code
    just doesn't need to call wordin when things are done

    Knuth instead provides a coroutines method that claims is faster. I have no doubts about the claim, but
    I feel coroutines muddy the water some (shame on me for being a higher level developer and not trying
    to eke out every cycle, I know)

    The one interesting thing about a coroutine approach (and I am coming from a place of unfamiliarity
    with that sort of coroutine thinking), is that the input of both buffers look sequential from the
    functions standpoint, which is nice. It doesn't even look like a circular buffer, just a for loop

8) Describe buffer transitions according to figure 23 in the book

    The two red buffers have text inside of them, waiting for an IO operation to consume one. The other 4 are green,
    meaning that they are free to be assigned. Since there is no "current" block, we have not assigned a buffer yet, and we must
    have just released one. We'll hit assign, which will take the next green node and mark it yellow (start writing to it).
    At some point, IO will complete, which will make the next red block be printed out, and that will turn green.

    As we release, the current node becomes red, and the previous node is starting to be printed.

    Side note-  why 6 buffers? in the example, it describes 5 blocks to be read at a time. I assume the 5 buffers that are
    being processed plus a sixth buffer

9/10/11) Given some assign/compute/release times, how will the buffering work with 2/4/1 buffers

```A,1000,R,1000,A,1000,R,1000,A,1000,R,1000,A,1000,R,1000,
   A,7000,R,5000,A,7000,R,5000,A,7000,R,5000,A,7000,R,5000,
   A,1000,R,1000,A,2000,R,1000```

```
T           N=3(in book)                N=2                         N=4                           N=1
0           ASSIGN(BUF1)                ASSIGN(BUF1)                ASSIGN(BUF1)                  ASSIGN(BUF1)
1000        RELEASE, OUT BUF1           RELEASE, OUT BUF1           RELEASE, OUT BUF1             RELEASE(OUT,BUF1)
2000        ASSIGN(BUF2)                ASSIGN(BUF2)                ASSIGN(BUF2)                  ASSIGN(wait)
3000        RELEASE                     RELEASE                     RELEASE
4000        ASSIGN(BUF3)                ASSIGN(wait)                ASSIGN(BUF3)
5000        RELEASE                                                 RELEASE
6000        ASSIGN(wait)                                            ASSIGN(BUF4)
7000                                                                RELEASE
8000                                                                ASSIGN(wait)
8500        BUF1 assigned, OUT BUF2     BUF1 assigned, OUT BUF2     Buf1 assigned, OUT BUF2      Buf1 assigned
9500        RELEASE                     RELEASE                                                  RELEASE, OUT BUF1
10500       ASSIGN(wait)                ASSIGN(wait)                                             ASSIGN(wait)
15500                                                               RELEASE
16000       BUF2 assigned, OUT BUF3     Buf2 assigned, OUT BUF1     OUT BUF3
17000                                   RELEASE                                                  Buf 1 assigned
18000                                   ASSIGN(wait)                                             RELEASE, OUT BUF1
19000                                                                                            ASSIGN(wait)
20500                                                               ASSIGN(BUF2)
23000       RELEASE
23500       OUT BUF1                    Buf1 assigned, OUT BUF2     OUT BUF4
25500                                                                                            BUF1 assigned
26500                                                                                            RELEASE, OUT BUF1
27500                                                               RELEASE                      ASSIGN(wait)
28000       ASSIGN(BUF3)
30500                                   RELEASE
31000       OUT BUF2                    OUT BUF1                    OUT BUF1
32500                                                               ASSIGN(BUF3)
34000                                                                                            BUF1 Assigned
35000       RELEASE
35500                                   ASSIGN(BUF2)
38500       OUT BUF3                    Output stops                OUT BUF2
39500                                                               RELEASE
40000       ASSIGN(BUF1)
41000                                                                                            RELEASE, OUT BUF 1
42500                                   RELEASE, OUT BUF2
44500                                                               ASSIGN(BUF4)
46000       Output Stops                                            OUT BUF3                     ASSIGN(wait)
47000       RELEASE, OUT BUF1
47500                                   ASSIGN(BUF1)
48500                                                                                            Buf1 Assigned
50000                                   Output stops
51500                                                               RELEASE
52000       ASSIGN(BUF2)
53500                                                               OUT BUF4
54500       Output Stops                RELEASE, OUT BUF1
55500                                                                                            RELEASE, OUT BUF1
56500                                                               ASSIGN(BUF1)
57500                                                               RELEASE
58500                                                               ASSIGN(BUF2)
59000       RELEASE, OUT BUF2
59500                                   ASSIGN(BUF2)
60500                                                               RELEASE                      ASSIGN (wait)
61000                                                               OUT BUF1
61500                                                               Stop Compute
62000                                   Output stops
63000                                                                                            Buf 1 assigned
64000       ASSIGN(BUF3)
65000       RELEASE
66000       ASSIGN(BUF1)
66500       OUT BUF3                    RELEASE, OUT BUF2
68000       RELEASE
68500                                                               OUT BUF2
69000       Computation stops
70000                                                                                            RELEASE, OUT BUF1
72500                                   ASSIGN(BUF1)
73500                                   RELEASE
74000       OUT BUF1                    ASSIGN BUF2, OUT BUF1
75000                                                                                             ASSIGN(wait)
76000                                   RELEASE                     Output stops
77000                                   Compute Stops
77500                                                                                             BUF 1 (assigned)
81500       Output stops                OUT BUF2
84500                                                                                             RELEASE(BUF1), OUT BUF1
89000                                   Output stops
89500                                                                                             Assign(wait)
92000                                                                                             BUF1 assigned
93000                                                                                             RELEASE, OUT BUF 1
94000                                                                                             Assign(wait)
100500                                                                                            Buf 1 assigned
102500                                                                                            RELEASE, OUT BUF1
103500                                                                                            Compute stops
110000                                                                                            Output stops

```

12) Given a control routine, show how it can shut off input if the last character of a card is a period

Control routine:

```
CONT1   JMP  COMPUTE
1H      J6Z  *-1
        IN   0,5(U)
        JMP  COMPUTE
        LD5  -1,5
        DEC6  1
        JMP   1B
```

The idea is that you `JRED` to `CONTROL` whenever you want, so the idea is that you should
resume right after you jump to compute. This means that line after the second jmp compute
should only be executed after we've just read input. We could do something like the following:

```
CONT1   JMP  COMPUTE
1H      J6Z  *-1
        IN   0,5(U)
        JMP  COMPUTE
        ST5  RESTORE5(0:2) * save off I5
        LD5  15,5(5:5)   * read the buffer that we just read ( last char)
        CMP5 PERIOD
        JNE  RESTORE5   * if its not a period , just continue on
        JE   COMPUTE    * go back to computing
        JMP  *+1        * co routine will come back here, go back to compute
RESTOER5  ENT5 *
        LD5  -1,5
        DEC6  1
        JMP   1B
PERIOD  CON   40
```

13) What instructions should be included at the end of compute to output all information
    still in buffers?


We need to make sure that every "Red" buffer is output

```
1H   OUT 0,5(U)
     LD5 -1,5   * go to next red buffer
     DEC6 1     * I6 = number of red buffers
     J6NZ 1B    * if there are more buffers, output them
     JBUS *(U)  * wait for last output
     HLT        * end
```

Of course, my answer above was way less elegant than Knuth's. His is just jump
to control as long as 6 is not zero, which makes sense because my code above
is pretty similar to `CONTROL`.

14)  What would happen if we did `ASSIGN...ASSIGN...RELEASE...RELEASE` instead of alternating?

You would fill two buffers in computation at once, which might be useful if had a lot of time between releases, but
not necessarily between assigns. Your program can fill two buffers worth of data, and while lots of time is spent
post release, you are doing your output. However, you would not be able to do this with just one buffer, as the
second assign would block forever.

Note - The answer above is how I answered, but it isn't necessarily right. Knuth instead focuses on a program that
needs two buffers worth of data

15)  Write a short program that copies 100 blocks from tape 0 to tape 1, using three buffers. See copy_tape.mixal
     The goal is to make it so that devices are already busy, and there is always a place to write

     Knuth does a batch of three outputs/inputs in one loop, which will decrease the amount of superfluous looping
     statements

16) Write a green-yellow-red-purple with three coroutines  (one input, one output, one computation)

    We won't be able to do a n < N check in these cases, See green_yellow_red_purple.mixal.

17) We need to adapt code to pool multiple buffer algorithms. We will have a linked list of red buffers and
    a linked list of green buffers to handle multiple devices. See pooled_buffers.mixal

    Now, there needs to be some analysis for this too. I'll be honest, I'm not seeing a huge benefit of one or the other
    Compared to a ringed buffer, you still are querying for a red buffer or green buffer. However, if you had input
    devices that shared a pooled buffer, that took different amount of times, you could handle them in different orders,
    whereas the ringed buffer probably doesn't work well with out-of-order buffers being ready. With a sufficiently sized
    number of devices sharing the pool, the more likely you will handle data being ready in any order.

    However, this does cause issues when you are trying to block on data, but that can be checked with certain flags too

18) We need to change the Assign/Release/Control routines to use an interrupt based mechanism. See interrupts.mixal
    At its heart, we need to enter a control routine with an interrupt and trigger the read, and then the interrupt
    afterwards will occur when its done reading.

    After thinking about the issue, I can't get rid of the JRED, because even in an interrupt case, I have to know when the
    device is ready. So (and based on the answers in the back that I glanced at), it really means don't use jred for a
    coroutine, but let any asynchrony happen in an interrupt context
