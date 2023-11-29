1)  What simplifications can be made if we assume that memory allocation/reservation is last-in-first-out?

Well, I see the words LIFO, I gotta immediately go to a stack. If we store the available nodes as a stack instead of a linked list, we know that the top of the stack is the last one to be reserved, and the first one to be freed. We never have to go any further into the list when we need to release. 

Thus, reservation is just a constant push the node onto the stack, regardless of how big it is. Release is just a pop of the stack.

2)  If you have fixed node size and wanted to store something longer, what is the minimum average size of node size (assuming some amount of storage is used for control structure, say b), given average item length L. 

I'm not sure how to even start this problem, so skipping for now.

3) Comapre best-fit, worst-fit, and first-fit with computer simulation. 

I do not plan on running these simulations at the moment, given lack of random distribution in mix (And don't feel like writing the algorithms into something like Python.)

4) Write a MIX program for Algorithm A.

See [first-fit.mixal](first-fit.mixal)

5) Suppose it is known that N is always 100 or more in Algorithm A. Should you set c = 100 in modified step A4'?

If you set c to 100, you are pretty much saying that if you're block is less than 100 big after the reservation, reserve all 100 of it. This is actually a really good thing, as nothing less than 100 will ever fit in that block. 

6) Can you suggest a way to modify Alogirhtm A so that short blocks don't accumulate in a particular area, and the AVAIL list is still in increasing memory locations.

I'll be honest, I had no idea, but the name of the question was "next-fit". I suppose you keep a pointer to the next area after the one you just allocated. That way, as nodes get allocated, you probably are allocating somewhere in the middle, and there will be gaps as you keep allocating. As the memory gets fragmented with use, consecutive allocations will be spread through the memory.

See [next-fit.mixal](next-fit.mixal)
