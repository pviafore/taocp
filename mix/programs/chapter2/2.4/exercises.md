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
    2.  If P is NULL, terminate. Set $NAME \leftarrow P(NAME)$.
    3.  If PREV is NULL, go to step 7. Set $PARENTPREV \leftarrow PREV(PARENT)$
    4.  If $PARENTPREV(NAME) == NAME$, terminate with ambiguity
    5.  $PARENTPREV \leftarrow PARENTPREV(PARENT)$. If this is null, go to step 
        6. Otherwise go to step 4.
    6.  Set $PREV \leftarrow PREV(PREV)$. Go to step 3
    7.  $P \leftarrow P(PARENT)$. Go to step 2. 

    See [build_data_table_ambiguous.mixal](build_data_table_ambiguous.mixal)

    After looking at the answer, I'm being over-restrictive - not allowing any structure that could produce a ambiguous reference, whereas it's more asking how to construct the table so that every element has at least one valid reference.

5) Write algorithm B in terms of a linked list of pointer references

    See [reference_check.mixal](reference_check.mixal)

6)  PL1 makes it so only complete references are valid

    We can't skip parents, we have to go one by one. See [reference_check_pl1.mixal](reference_check_pl1.mixal)

7)  What will MOVE CORRESPONDING SALES TO PURCHASES Do?

    It will move Date from sales to purchases, and then transaction's item, quanitty, price and tax to corresponding fields, but not name or address.

8)  When is MOVE CORRESPONDING A to B the same as MOVE A to B?

    When A and B are complete qualifications to elementary elements.

9)  Prove that algorithm C is correct.

    In order to prove this, we need to prove that it picks every pair and no others.

    So to start with, we have two unique references. For each reference, we are going to go to the first node and see if it matches. If either is elementary, we will print out this match, and be done. Consider this a base case. 

    Now, let's see what happens if both descednd. We'll go down the child route for both the references. For the next node on the first reference, we will only output the pair if the name matches with our search, so it's not going to be output anything that doesn't match.

    Now that we have to prove that for each element in the first reference, we will find a value in the second reference if it exists.

    The first thing we do is iterate through the siblings of the second reference. If any of these match, then we know we're good and move on. However, if we don't find a match, one of two things happend: the match is further down, or the match is a cousin (an ancestor's sibling's descendent).

    If it doesn't match, first we'll try to go to the first reference's next sibling to see if it matches any of the Q's siblings. If this doesn't work, we go up one level (not past the original though), and one to the right and try again. This is almost like a depth first search looking for matches.

10) How would you write Algorithm B and C without a NAME field in the data table.

    So, instead of checking names, we can make sure that both elements are in the same row of the data table.

    For Algorithm B, we have a node S. We can walk through the symbol table until we find S, and then just compare that against K, since K has a pointer to the symbol table.

    See [referencecheck_noname.mixal](referencecheck_noname.mixal)


    For Algoirthm C we'll have to do something similar, but look for two different nodes in the symbol table.

    See [move_corresponding_noname.mixal](move_corresponding_noname.mixal)

11) What can we do to speed up Algorithm B and C?

    In general, ignoring algorithmic upgrades, we can do two things to speed things up. Take a one-time hit on setup to prepare the data in a different way, or trade memory for time.

    For algorithm B, the main time taken is the walk up the parent tree for each node, as well as the iteration of prev links. So, one thing we could do is sort the nodes in the prev linked list by number of parents. If we are looking for something with X parents in the reference, we know we can stop once we start hitting x-1 parents, as there is not enough room to check any further. We also could make each node have a lookup table for parents, where the name of the parent goes to the node in the parent chain.

    For Algorithm C, we can do similar things. We can have a lookup table of elementary nodes in each node so that we can descend directly to them (and compare similar names). They can even be qualified by a name chain.

12) Write Algorithm B with PREV/SCOPE links.

    So we need to be able to walk through parent nodes given a node. I could do a linear scan of the data table tracking a stack to know what my parent is, but that's a lot of work. So, what can we do with our scope node?

    So given a child node, we can figure out what tree it is (and what it's parents are) by the following: Look in the current tree (starting with first node in datatable). If the child node is in this tree (between addrses and scope), then recurse into the first child, keeping a stack of parents. If the child node is to the right go to the next child. Stop when you find the node, and then compare your stack to your reference list to see if they match.

    Then go to the previous child node and try again, checking to see if you have found any matches or not.


    See [referencecheck_scope.mixal](referencecheck_scope.mixal)
     
13) Write Algorithm A with PREV/SCOPE links.

    Previous is unchanged.
    
    So, anytime we go to a sibling, if our scope is zero, then set our current node to the scope (We are an elementary node).
    
    If we are going up to a parent, set our current scope to our current node (if it's zero and not going to a sibling), and then set our parent's node (which will be the stack) to the current node. If we're not going to siblings, we keep going up the parent setting this.

    See [build_data_table_scope.mixal](build_data_table_scope.mixal)

14) Write Algorithm C with PREV/SCOPE links.

    This should be the same as Algorithm C with the following modifications:

    - Checking if a child is null: Is your scope the same as yourself
    
    - Going to a child:  you push the current PQ onto the stack, and move one node down from both

    - Are they equal? (Move to the prev for each node, and if they end up equal, then yes)

    - Is sibling null? Does your scope equal the scope of your parent (which is on the stack)

    - Set to sibling: Add 1 to your current scope 

    - Reset to first child: Look at your parent on the stack, and add one to it.

    - Go to parent: Pop Stack

    See [move_corresponding_scope.mixal](move_corresponding_scope.mixal).

     