# 2.2.4 

1)  What happens if ptr = LOC(PTR) is what indicates empty list?

    So say we have an empty list where PTR is at address 1000 and set to value 1000. If we insert a node to the left,
    we set PTR to new node, and change PTR's link to the node. If we insert to the right, we update PTR to the node.
    When we delete, We can return the value and just set PTR to LOC(PTR). There doesn't seem to be much of a help, as this is what we would do with a null link too

2) Before and after of concatenation

                         PTR1                                PTR2
   -----      -----     -----             -----    -----    -----
   | 1 | ---> | 2 | --> | 3 |             | 4 |--->| 5 | -->| 6 |
   -----      -----     -----             -----    -----    -----
    ^                     |                  ^                 |
    |                     |                  |                 |
    -----------------------                  -------------------


                                                             PTR1
   -----      -----     -----             -----    -----    -----
   | 1 | ---> | 2 | --> | 3 | ----------> | 4 |--->| 5 | -->| 6 |
   -----      -----     -----             -----    -----    -----
    ^                                                         |
    |                                                         |
    -----------------------------------------------------------

3)  What happens on a concat if both lists are the same.

    We exchange the links of ptr 1 and ptr 2, both of which are pointing ot the leftmost element.
    This is essentially a no-op, but we do null out ptr2.

    What happens if they are in different elements? 

    We'd create two disjoint lists. Assuming PTR1 is to the left of PTR2, PTR 1 would now point to the element just right of 
    ptr2 and continue all the way back to ptr1, and ptr2 would point to the element just right of ptr 1 (which could be itself). We would then lose a ptr to the second list.

4)  What are the insertion and deletion operations that give the effect of a stack using representation (4)?

    Insertion:
        Add node to right, and assign pointer to that
    Deletion:
        If list head, underflow. Else, remove Node from right,

5)  Reverse a circular linked list (see `reverse.mixal`).  

                         PTR1
   -----      -----     -----
   | 1 | ---> | 2 | --> | 3 |
   -----      -----     -----
    ^                     |  
    |                     |  
    -----------------------

    Save off PTR 1's address (3)
    Go to next node (1) and save off ilnk (to 2)
    For next node point its link back to prev

    
                         PTR1
   -----      -----     -----
   | 1 |      | 2 | --> | 3 |
   -----      -----     -----
    |                     ^  
    |                     |  
    -----------------------

    Save current (1) as previous
    Go to next node (through saved link -- 2)
    save next link (3) point current to previous
    if current node is end (3), terminate

                         PTR1
   -----      -----     -----
   | 1 | <--  | 2 |     | 3 |
   -----      -----     -----
    |                     ^  
    |                     |  
    -----------------------
    
    Save current (2) as previous
    Go to next node (through saved link -- 3)
    save next(link - doesn't matter) point current to previous
    if next node is end (3), terminate

                         PTR1
   -----      -----     -----
   | 1 | <--  | 2 | <-- | 3 |
   -----      -----     -----
    |                     ^  
    |                     |  
    -----------------------

6) Draw xz-3 and 0
                                                   
   --------      ---------      ------------
   |   1   | --> |  -3   |  --> |     0    |
   --------      --------       ------------
   | 1|0|1 |     | 0|0|0 |      |-  0|0|1  |
   ---------     ---------      ------------
    ^                                 |  
    |                                 |  
    -----------------------------------
    

       ------------
       |     0    |
       ------------
       |-  0|0|1  |
       ------------
             ^    |  
             |    |  
             -----

7)  Why is it useful to assume that ABC fields of a polynomial appear in a decreasing order?

    We maintain a strict ordering so that we don't have to constantly loop through the circular list looking
    for which elements are in there are not. We advance P and Q at the same time, and  we know we never have to backtrack, because a later element in Q cannot come earlier in P, since we either just checked coefficients that are the same,
    or we just inserted a node that was larger.

8) Why do we need Q1 trailing for the add algorithm

   We need to be able to delete a node when we have zeroed terms out (such as adding +x and -x). You need the 
   previous node to delete efficiently.


9) Would Algorithm A Work if P = Q? 

   Adding two coefficients seems like it would work, but what would happen if we have to insert a new term?
   Well that couldn't happen, so yeah, it should be just fine (since we never go back to an old node in Q)

   What about multiplication?

   If P = M:

    This should be fine as it just changes ABC(P) to 2*ABC(P) and COEF(P) to 2*COEF(P)

   If P=Q:
        This would be a problem, because we are modifying the list of q as we go on (inserting new terms), and 
        that will affect multiplication

   If Q = M this would also be troublesome, becasue M is getting modified along the way, and if we ever advance to a link that was newly inserted (I do wonder if this is possible with not having inverse powers, becasue you would only ever add before the link of M (you cannot multiply two polynomials with positive powers and get a polynomial smaller than it))
    

    Note: It seems like Q = M will work for all cases (it doesn't sound like you worry about adding previous ones), but Knuth does mention that computation blows up if P == -1.

10) If the polynomial is one three byte field instead of x,y,z, what changes happen to Algorithms A and M

    

    I don't think you need to make any changes. You still can maintain a strict ordering, and you can still add them together
    just fine.

11) Implement a copy routine. See `listops.mixal`. Effectively, we save off the first node, allocate space for it and move on to next link, allocating and copying each time, until we reach the end. 

12) Compare the running time of copy to an add when the poly(Q) is 0:

    So for a copy, we have 6u per allocation, and 8u per copy. First thing we do is store a jump (2u), allocate, and copy (14u), plus two register copies for overhead (2u)

    Then, for each element (n - 1 times) we do overhead iterator advancement and check (5u), copy and allocate ( 14u), store previous link (2u),  and 2u for the jump. Then 1 time we will exit which is just the 5u. 

    Finally, we save a link and jump back (3u)

    This means we have 16u + n*(5u) + (n-1)*(18u) + 3u  This gives us (23n + 1) u.

    For the add routine, we are always going to add a new element (element in P but not in Q). This is about (26n+13) (note I don't have jump overflows). It does seem faster though.


13) Implement an erase routine. See `listops.mixal`. Save off pointer to first node, and go to the next link. Move the saved off pointer to AVAIL, and set AVAIL to first node

14) Write a ZERO routine. See `listops.mixal`. 