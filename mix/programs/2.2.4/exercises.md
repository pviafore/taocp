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