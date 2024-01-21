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

11) Show that the improvements to Algorithm A (using a roving pointer) can also
    make Algorithm B faster.


    So, the idea is a next-fit, keeping a roving pointer. I think that you could check the roving pointer when determining what to release. (Or if your address is less than the rover, you can start at the beginning). This means that instead of a worst case of the entire list, you will at worst, have a worse case of the memory size - 1, but at best, your worst case is half the list. 

12) Modify Algorithm A so that it follows boundary-tag conventions, use A4', and incorporate roving pointer.


    See [next_fit_boundary.mixal](next_fit_boundary.mixal).

13) See above

14) What difference would it make to Algorithm C and the algorithm of 12) if the size field were not present in the last word of a free block, or if it were note present in the first word of a reserved block.

If you were missing the last word of a free block, then the reservation would be trivially faster (don't have to update the size),  but liberation would be much slower. To check for a lower bound, you have to go to your next block (based on your size), then use the back link to find out what your previous block was.

If you were missing the size of the reserved block, then you don't know how much you are freeing, which makes this very very tough, as in, you can't do it without leaking some.

15) Show how to speed up the Algorithm C if you don't change links for each of the four cases on if previous tag and next tag ore plus or minus

| PREV TAG| NEXT TAG |    Result    |
|---------|----------|--------------|
|   +     |    +     |  No change, there is reserved blocks on both sides    |
|   -     |    +     |  just update size in previous block                   |
|   +     |    -     |  update size in current block, and updates links in current block            |
|   -     |    -     |  update size in prev block and update links in prev blocks            |


16) Write Algorithm C with improvements from 15.

    See [liberate_long.mixal](liberate_long.mixal).

17) What should be the contents of LOC(AVAIL) and LOC(AVAIL) + 1 be when there are no available blocks present.

    They should both point to the current list head. Since the size is zero, and we should use the AVAIL as a sentinel.

18) Why does first-fit pictures have high amounts of memory available in high memory locations, whereas best-fit pictures have high amounts of memory in low memory locations.

    For first-fit pictures, there is a preference for finding the earliest memory location. The only way you pick a later position is if there is literally no spot in low memories. The odds are against you. So as memory is liberated from the beginning, you are more likely to pick that memory again to re-reserve.

    For best-fit however, you are always going to pick the smallest that fits. Inevitably, on a large reserve, you're going to start reserving from the back end of memory. As you free up earlier locations, you aren't going to be reserving back in the beginning until memory is almost full. At some point there will be small gaps in the back end of memory (which will be filled by small allocations), while large allocations will still be the latest possible block.

19) Write a reservation algorithm that does not use the last tag or size, collapses adjacent free blocks (liberation doesn't do it), and does not cause undue fragmentation.

    In order to not have undue fragmentation, you should be allocating at the furthest possible. Even though we are doing first-fit, we can do all the collapsing of the current node first, and allocate towards the end of that block.


    For example:

    - Iterate through avail list
    - Each time you iterate to a new node, look forward and collapse
    - Check current node for fit after the collapse.
    
    This way, we'll always strive to have bigger blocks and collapse rather than fitting first and collapsing after (which would definitely fragment more).

    See [reserve_notag.mixal](reserve_notag.mixal).

20) Why is it desirable to have the lists in buddy system doubly linked?

You need to remove links arbitrarily, and when you add them back in, you don't necessarily want to have to search for the right place to put them. More specifically, the buddy link can be removed just by knowing the address of the buddy link.

21) Examine the ratio of sums 1+2+4+4+8+8+8+8 .... to 1+2+3+4+5+6......

    So the first term is $2^{\lceil{lg(n)}\rceil}$, where the next term is n+1. I don't know how to do infinite series, so someone with more Taylor series knowledge can figure it out from here.

22) Why can't we just allocate 11 blocks in a buddy system by doing 2 4 blocks, 1 2 block and 1 1 block?

    You'd have to keep a list of all alocations, because there is no guarantee that the blocks are next to each other, and what happens if you need to spread data across two blocks (say a logical piece of information 5 bytes wide.)

23) What is the binary address of the buddy of the block of size 4 whose binary address is 011011110000?  What about if it's size 16?

    011011110100

    011011100000

24) Accorrding the the text, the largest block of size 2^m has no buddy, since it is all of storage. Could we define it's own buddy as zero, and check that the size of k is the entire size?

    I think so, yes, but I don't know why you would. We use the buddy system to know which node to collapse, but there's never a need to collapse.

25) Criticize the idea -> Dynamic storage allocation will never reserve above a certain threshold, so don't start with large blocks available, and you can skip combining if you go bigger than this limit.

    Well, at first, I don't know what to criticize. I thought that crossing certain boundaries could be problematic, but you don't cross between buddies.  You are imposing checking and branching on every allocation and collapse to make sure this doesn't happen, which is a lot to pay for each allocation or liberation. Given that a large block in practice is immediately broken up, it seems we pay an ever-occurring cost rather than a one-time cost at the beginning.

26) Explain how you can use the buddy system for memory that is not an even 2^m.

    You can go as far as you can , since you will know if any buddy goes above the memory range. Say you had 11 byte storage. You allocate $2^{m-1}$ first, which in this case is 8. That leaves the next three blocks. You break up these blocks until you have it taken care of (first into a 4 block, and then that in a 2 block, and then the second block in a 1 block). Now, you have your 11 blocks, and they are all on the "left" side of partial block. Any buddy any of these blocks have will inherently be above the memory range, so you can check for that, and never collapse buddies if you are in a partial block.

27) Write a mix program for Algorithm R and measure it's running time.

    See [buddy-reserve.mixal](buddy-reserve.mixal)