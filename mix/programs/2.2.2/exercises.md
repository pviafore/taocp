1)  In the queue operations in the book, how many items can be in teh queue at one time without overflow occurring?

    There can be M-1 items, since the rear is one pointer past the last element, and you don't want f==r

2)  Add operations for a general deque

    Insert into front:
```
    X[F] = Y
    If F == 1, F = M, otherwise, F <-- F - 1
    If F == R; then Overflow
```
    Delete from back:
```
    If F == R; then underflow
    X[R] = Y
    If R == M, R = 1 otherwise, R<-- R + 1
```

3a) If we have indirect addressing, how do we get a relative address?

    Assuming the offest is in 1, and BASE stores the memory address, we can do `LDA BASE,7:1`
    This replaces 3 commands, but adds an extra amount of time for one command, so we have 5 cycles, instead of 8.

3b) With indirect addressing, how do you look up an element of a table when you have multiple tables.

    Assuming the Ith element is denoted by I1, and the Jth table in I2, and tables are at BASE, you can do `LDA BASE,2:7`, but
    you would need each base to be stored as BASE,1

3c) ENT4 X,7 would be the same as LD4 X(0:2)

4a) Infinite loop - Assuming that BASE is in location 1000, and contains the value 1000,7:1 => `LDA BASE,7:1`

4b) Get Link(Link(x)) into register A where X Stores the x variable's link: `LDA X,7:7(0:2)` (which does a double hop)

4c) Get Link(Link(Link(x)) We can't do this, as we would need to triple hop, which means we need to address to something that has a 7:7, which is disallowed. We'd need more instructions

4d) Load A the contents of location RI1 - RI6, Given that BASE1 contains NOP 0,5:6, BASE2 contians BASE1,7;4, BASE3 contains BASE2,7:3, BASE4 contains BASE3,7:2, and we do a `LDA BASE4,7:1`

4e) Quadruple the current value of ri6: `LD6 BASE2,7:6` assuming that BASE2 contains BASE1,7:6 and BASE 1 contains NOP 0,6:6

5a) If you allowed 7:7, you'd be essentially saying that when you load an address through indirect addressing, you would need to
    take the address in the 0:2 field, dereference it, and then whatever addrs you' given there, dereference that again. I can already get
    in an infinite loop or a long chain of indirect addressing with just one 7, so I don't think it's that.

    Maybe it has something to do with what hardware would have to do? Why would it need a three-bit stack? ( I feel like this is a clue)

    What would happen if we did LDA X,7:1 where X was X+1,7:7. It would look at the address in X ( which has value X + 1), hop that with a
    7, and then hop the next 7. Even if address X+1 had index registers, would that be a problem? The 3-bit stack makes me think we need
    to track index register values (7 values being possible after all, and you can have 8 with three bits)

    It's not a order-dependency, it's clear that I1 gets done first, and then I2. If you had X+1's value be 0,1, then you would end up just
    double hopping based on I1, which would essentially mean you are doubling I1.

    Could there be some non-determinism? I don't see how, so I'm still thinking it severely complicates what a circuit needs to do.

    Maybe it's something where you need to track what address you came from, because you would have to know what to calculate. But then the stack would have to be more than 3 bits. As far as hardware goes, I see a circuit operating in a loop as long as the modifier is 7,
    and then a serial circuit afterwards as long as that modifier is 7. If this isn't allowed ,then why is LDA X,7:1 where X contains X,1:7.

    I think I'm going to look it up and write down some of my thoughts.

    After looking it up, I think I was close, just not able to do the specifics. The 7:7 has a giant branching effect, and you would have to
    save the results off as you descend into madness. The stack would track which registers need to be applied as you go up and down
    the tree of addresses.

5b) Why would you not need to do this with current hardware? You could make it so that anytime you had a 7 (be it first or last), you add
    whatever index register at that point, and take the result of 7 to add to it. So you have a process and accumulate (reminds me
    of tail recursion to some point). Since you never have to step back up, you never need to have a stack of what needs to be added.

    So, for your first adress, you set the accumulator to zero (plus whatever index registers are set). Then for each 7 at the top level you do the following:

    Go to the address listed. If there are just index registers, add them to the accumulated value and the address. If there is a seven, add just the index registers and recurse. (you may need to add the index register to the address first if the 7 is last.)

5c) Any address that originates from a 7:7 must only have 0:0 as then index field. This way, you cut off the branching factor.
    I'm not sure this is enough for Knuth - let's see what he says. He had, of course, a more elegant answer

6) No, No, Overflow, Underflow

7) No, we must have inserted at least once to trigger a repack, so inc has to be at least one

8) Handle repacking for queues being handled circularly. If it a circular queue, give it a fixed size. Calculate the new bases for
   circular queues so that the end of memory is just enough to hold the fixed memory, and then consider L-infinity to be the space
   before the base of the circular queues. You pretty much dedicate memory for your circular queues.

   Ah, I misunderstood, he was mentioning whether the circular queues were over/under-flowing. Let's consider two cases:
   If we need to rearrange a circular queue, it's either contiguous or wrapping around. In both cases, we want four points
   top, bottom, and front, rear. Top and bottom dictate the size of the memory, and can be the same calculations for our normal
   top and bottom calculations. If front < rear, we're contiguous, and we can just keep elements where they are (they will wrap around
   to fill more space). However, if front > rear, we've wrapped around, and we need to fill space in the middle. We can move the wrapped around early parts of the queue down until they hit bottom., and we can move the front of the queue up until it hits top.

9) This math is a little beyond me, so I'm going to have to skip this one.

10,11,12, 13,14,15) See #9

16) Can two queues grow towards each other? They can, but they may trample well before they are full. As long as there are never an
    equal number of deletions to insertions, the queues will march towards each other, even if they only have one value. This is not
    a great answer, but if you have times where the queue is deleted completely, you'll be fine. Same for a single stack and queue.

17) I don't think I have the math to prove this formally, but one thing to note is that when all queues start at zero, they will
    split out and repack after the first insert, which will be closer to them more equally being spaced out.

18) What's nice is that since the memory is growing proportional to queue usage and recent usage, it will slow down as more insertions happen
    This means it will be constant to some degree, similar to a vector push_back or hash table insert. I don't have much more insight than that,
    unfortunately.

19) We could just have any array access be -1, but in lieu of that: Insert of a stack changes by checking for overflow first (T=0). With deleting, we want to make sure that we don't underflow, we need to check if T is -1, or we can decrement T first.

    For queues:F=R=0 to start with. inserts should go back to R=0 if R=m, and deletes should reset F to zero if F == M.
