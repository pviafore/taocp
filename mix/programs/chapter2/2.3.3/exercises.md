1)  If we only had ltag, info and rtag in a level order, could we omit llinks? Can we reconstruct the llink?

    Yes, I think so, consider the following

    ```
    LTAG              |               |   |   |   |
    INFO     A   D   B   C   E   F   G  K   H  J
    RTAG          |        |          |   |   |  |
    ```

    Anytime you do not immediately have ltag, put the node on a queue. Any time immediately after having a rtag, pop the queue; the popped node's llink will be the next node.

2)  The trees stored in preorder could also have Algorithm F (see [locally-defined-function.mixal](locally-defined-function.mixal)) that goes right to left

    See [locally-defined-postorder.mixal](locally-defined-postorder.mixal). In essence, we just go right to left, but the rest of the algorithm is unchanged. This is because we are guaranteed to do our children before our root node, since in pre-order, the root node always comes first.

3)  Write the differentiation algorithm using the locally-defined algorithm and a stack.

    So instead of using I3 for some extra rlink shenanigans, we'll keep a stack in I3. When we have a nullary operator (var or constant), we just push one value on the stack. For unary operators, read one value of the stack, and put the result back on the stack.

    For binary operators, read two values off the stack and push one value on the stack. See [differentiation.mixal](differentiation.mixal).

4)  The trees in normal tree representation contain in (2) is 10 nodes, 5 terminal. However, if there are llinks and info fields sharing a spot, we have 5 llinks and 15 rlinks. What does this look like with n nodes, m terminal.

    So for the original representation each node will have a llink and rrlink, which means we have n llinks and n rlinks

    For the second case though, we only have n-m lliniks, but and the same number of rlinks + n.

5)  For a triply linked tree, does it help to introduce threads at all?

    The only thing I can think of is if we want to make the tree circular in any way, in case you want to iterate over the level again, but it doesn't make too much sense to go to parent (like we did with binary trees), since we already have a link up.

6)  Given a linked list of tree nodes that only have a parent node 
    specified, create a triply linked tree is set up.

    We will have to iterate over the list, but the tricky thing will be updating rlinks without destroying the list. I'm trying to think if I can do this in one pass. I could do it in multiple passes by looping through the list, and setting up lchilds for everything and removing those nodes through the list (removing rlink support), but keeping a pointer to nodes not added. Then you could go through the nodes not added and see if you can add it to the tree (if what you're adding to has a null rlink, then you should be able to). If you can remove it (null out rlink), and repeat this process until your list of rlinks is empty. This is a n^2 algorithm, since you have to iterate the list n times, and you iterate over n-k nodes, where k is in the number of nodes in the tree. Since we have to look at rlinks, we may have k of those so it's n * (n -k + k) times through.

    I'll write this out in [triple-linked.mixal](triple-linked.mixal), and afterwards I'll see if Knuth has a more elegant way. 
    
    After some thinking and a few false starts, I think I have something simpler. More concretely, my algorithm is:

    - Loop through each node
        - If first hasnt' been set and this has a null parent, set first
        - Can it be added as a lchild? If so, add it. Set it's rlink as null
        - Add it as a rlink along the chain of the parent's lchild's rlink (And set your rlink as null)
        - If it has a null parent and first is set, then set it as the first's rlink chain end.

7)  What classes would appear in the book if the relation $9 \equiv 3$ was never given?

    {1,5} {6,8,9}, {3, 7, 2, 4},

8)  Design an algorithm that determines if two elements are equivalent, given Algorithm E has already run and set up the table

    I'm not going to write this MIX program because it's very trivial.
    Essentially, if you are looking for equivalence for j and k, you just compare if the parents for each node (going up the chain for each) resolve to the same value.

9)  Show the table and diagram of all equivalence classes after running all the input


    $ 1 \equiv 5,  6 \equiv 8, 7 \equiv 2,   9 \equiv 8,   3 \equiv 7,   4 \equiv 2,  9 \equiv 3 $

    ```
    PARENT[k]  5  0  2  2  0  8  2  2  8  
    k          1  2  3  4  5  6  7  8  9
    ```


    ```
            5                  2
            |               / / \ \
            1               3 4  7 8
                                  / \
                                6    9
    ```

10) How can you make the class equivalence more efficient than n^2

    So if you have input of $i \equiv j$, sort the list by $i$ and make sure that $i \lt j$.  

    Now, (after an $ n \log{n} $ operation), we can start constructing the tree. Due to the sorted nature, we should have far fewer tree merges. We also can allocate some additional space to track "top node", so that each node caches which node is the top. When we merge a tree T1 into T2 (when a leaf node of both of them share an equivalence class), we will just update T2's top node's top node to point to T1's T1. Then to check equivalence or not, you hop through top nodes (updating as you go to fix any later issues).

11) Design an algorithm that will figure out how to overlay arrays of data, so to minimize space, given equivalence class of array indices

    The idea is that for some equivalence class $x[i] \equiv y[j]$, we do the following:

    1. Check if y has a parent. If not, set y's parent to x, and delta to $i - j$. If $ i \gt j$, check if y's end is past x's UBD. If so, update x's UBD.

    2.  If y has a parent, compare it's parent with x's parent (walking up the chain as needed). Sum up the parent's offsets + x and subtract y to get the current delta. If they share the same parent at the top, make sure that delta that is already set matches, otherwise you have conflicting information.

        Check [array_overlap.mixal](array_overlap.mixal)

12) After the algorithm of adding terminates, is Q0 always the address of the root fo the sum? Has P and Q been set to their original values?

    Yes, A11 always ensures that Q is at the root. However, the original term may have been erased in Q (P will still the same), so it's not guaranteed.


13) Give a proof that EXP(P) = EXP(Q) and CV(UP(P)) = CV(UP(Q)) at the beginning of step A8

    First, you can check out [add_poly.mixal](add_poly.mixal) to see how add is done.

    So, how do we get to step A8? We've either deleted a zero term right after adding in A3, or we've detected a zero polynomial after deleting a row.

    So if we've added a node in A3, we know that either this was the first insert, or an insert after we went to the right. In either case, we start at A1. In A1 we either go all the way to the bottom (the node above will be a leaf node). If $CV(Q) \lt CV(P)$, then we are going to insert a new node that CV(Q) to the value of CV(P). If they are equal, then that proves that we have CV(P) == CV(Q). Since P and Q then immediately go down one level in either A1 or A2, we have CV(UP(P)) == CV(UP(Q)). 

    On the first insert, we know the left hand side will be all zeroes, so we know that the EXP(P) == EXP(Q) == 0. On subsequent inserts, we advance to the left of our row until we find the right place where the exponent should go. If there is no place where they match, we insert a node to the right with the right exponent so that they do match (moving upwards if needed).

14) Formally prove or disprove the validity of Algorithm A

    This isn't going to be a full formal proof (in fact, nowhere close), but here's what we have to prove individually for Algorithm A to be valid:

    - Every node of P is added into Q
    - When a term is added, it is added into the right place (see exercise 13)
    - Deleted terms were terms that have a CV of 0 and a non-zero exponent (step A3) or the only zero / zero term in a child (step A9)

    So we need to show that every node of P is used.

    So for A1, we go all the way down for each node. Then after inserting, we move P to the left, circling all the way around. Once we have a row done, we do step A6 to move upward. This will hit every node with the combination (go all the way down, then circle around left, and go up, then go left again.) You can see something similar in add_poly.mixal where I print hte polynomial for hitting every node.

    Sorry it's nowhere close to a proof, but proofs unfortunately aren't my thing.

15) Design an algorithm to compute the product of two polynomials

    First, we are going to iterate every constant node of P with non zero EXP and CV. For each of these nodes, we are going to iterate every constant node of Q with non zero EXP and CV, giving us a (P,Q)-tuple. Each constant node with this property has a 1:1 relation with a term in the polynomial.

    For each pq-tuple, we need to compute the product. First we'll copy the UP chain of the P polynomial, inserting in any zero terms if needed to make it a valid polynomial. This isolates all the specific variables of the term to just this polynomial and represents a specific term. 

    Then, starting at the bottom, we look at Q and P in concert, variable by variable. We multiply CVs of constants and add EXP of like variables. If there is a gap in the EXP (there is a relational aspect to the variable names, remember), then we just add the term of Q in (Similar to a downward insertion). 

    Once we have this product of two terms represented as a polynomial, we just add it to the polynomial called M.

    Check out [multiply_poly.mixal](multiply_poly.mixal)