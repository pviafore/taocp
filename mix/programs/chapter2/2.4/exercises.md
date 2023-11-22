1)  Are the data items in COBOL in any order?

    Pre-order, since we always go down the tree first before exploring siblings.

2)  Comment about the running time of Algorithm A.

    I implemented Algorithm A in [build_data_table.mixal](build_data_table.mixal).

    For running time, we have a linear scan of input, so that will be our N. For each of these input, one of three things happens:

    1.  It's a new node that is descending. A2, A3, A4, A6 and A7 for each of these elements.
    2.  One at the same level. A2, A3, A4, A5, A6, and A7 happen.
    3.  One at a level up above. A2, A3, A4, A5 (multiple times), A6 and A7


    A5's pop is amortized, it will happen on every node in 2 and 3, and each "multiple" time is matching with a node from A. 

    So, we have A1 + N(A2 + A3 + A4 + A5 + A6 + A7). 

    Based on my code:

    - A1: 6
    - A2: 5
    - A3: 12
    - A4: 11
    - A5: (comparison): 9
    - A5: (pop): 7
    - A6: 6
    - A7: 8

3)  How would you change PL/I semantics into Algoirthm A?

    So it's not enough to check if an element is equal to the stack to see if we are the same level. Really, we need to know if we are between our top level on the stack and the second-top level on the stack, as well. If we're increasing, we know we are the next level.

    It also means we might be at a sibling rather than a child in A4

    See [build_data_table_pl1.mixal](build_data_table_pl1.mixal)

4)  How would you guard against ambiguity with multiple items with the same name.

    For each inserted element, you would need to look at it's previous chain. 

    1.  Grab your parent node $P$ and your previous node $PREV$.
    2.  Set $NAME \leftarrow P(NAME)$.
    3.  Set $PARENTPREV \leftarrow PREV(PARENT)$
    4.  If $PARENTPREV(NAME) == NAME$, terminate with ambiguity
    5.  $PARENTPREV \leftarrow PARENTPREV(PREV)$. If this is null, go to step 
        6. Otherwise go to step 4.
    6.  Set $PREV \leftarrow PREV(PREV). If this is null, go to step 7. Otherwise, go to 3.
    7.  $P \leftarrow P(PREV)$. If this is null, terminate. Otherwise go to step 2. 

    See [build_data_table_ambiguous.mixal](build_data_table_ambiguous.mixal)