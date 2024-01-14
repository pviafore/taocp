1)  What simplifications can be made if we assume that memory allocation/reservation is last-in-first-out?

    Well, I see the words LIFO, I gotta immediately go to a stack. If we store the available nodes as a stack instead of a linked list, we know that the top of the stack is the last one to be reserved, and the first one to be freed. We never have to go any further into the list when we need to release. 

    Thus, reservation is just a constant push the node onto the stack, regardless of how big it is. Release is just a pop of the stack.

2)  If you have fixed node size and wanted to store something longer, what is the minimum average size of node size (assuming some amount of storage is used for control structure, say b), given average item length L. 

    I'm not sure how to even start this problem, so skipping for now.

3)  Comapre best-fit, worst-fit, and first-fit with computer simulation. 

    I do not plan on running these simulations at the moment, given lack of random distribution in mix (And don't feel like writing the algorithms into something like Python.)

4)  Write a MIX program for Algorithm A.

    See [first-fit.mixal](first-fit.mixal)

5)  Suppose it is known that N is always 100 or more in Algorithm A. Should you set c = 100 in modified step A4'?

    If you set c to 100, you are pretty much saying that if you're block is less than 100 big after the reservation, reserve all 100 of it. This is actually a really good thing, as nothing less than 100 will ever fit in that block. 

6)  Can you suggest a way to modify Alogirhtm A so that short blocks don't accumulate in a particular area, and the AVAIL list is still in increasing memory locations.

    I'll be honest, I had no idea, but the name of the question was "next-fit". I suppose you keep a pointer to the next area after the one you just allocated. That way, as nodes get allocated, you probably are allocating somewhere in the middle, and there will be gaps as you keep allocating. As the memory gets fragmented with use, consecutive allocations will be spread through the memory.

See [next-fit.mixal](next-fit.mixal)

7)  Give an example where first-fit fails compared to best-fit

    Say you have three blocks open with size 100, 10 and 100, but three reservations  of 1, 100, and 100. With first-fit, you will put 1 in the first slot of 100, the second 100 in the third block, and nowhere to put the third block.  With best fit, the 1 will go in the 10, and the two 100s match up.

8) Modify Alogirthm A to be best-fit

    See [best-fit.mixal](best-fit.mixal)

9) What can we do about best-fit to make it search not so much.

    If we find a way to sort by size, then best-fit works quite well. However if we just do this, we merely kick the can down to release having to search through the entire list. So, we could increase the control data of each block by adding a link for size. Whenever you downsize a block, however, you'll need to remove it from the list and re-insert it, which is guaranteed to be earlier in the list. The bigger the allocation you make, the earlier you'll move it into the list. The smaller the allocation, the earlier in the list you were in to begin with.

    So you have two links, one for address space

10) Show how to modify Algorithm B so that a block of N consecutive cells is not assumed to be unavailable, in other words, the freed area might overlap other freed areas.

    So for unmodified, see [release.mixal](release.mixal). 

    The main thing that breaks down when you may have overlapping blocks is that P0 + N is not guarantee to be a block. Instead, we need to find the first block preceding P0, and then check every block that would start within P0+N after that. For each one of these, we will remove it from the list.

    We have to be mindful of interrupting the middle of a block, as well as lining up perfectly with a block.


    * Find the first block P that is greater than or equal to P0
    * Is the size is smaller than block size, do normal stuff
    * if the size is bigger than block size, start a loop for Q0
    * Q0 should keep going until the accumulated block size is bigger than size
    * We should do the boundary checking and collapsing with P, and Q0 (note that Q0 will equal P if hte size is smaller than block size) 

    See [release_indiscriminate.mixal](release_indiscriminate.mixal)
