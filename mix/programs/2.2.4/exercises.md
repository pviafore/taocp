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