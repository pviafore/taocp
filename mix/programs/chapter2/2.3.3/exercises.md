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

16) Prove the validity of bottom-up locally defined function

    We need to prove the following:

    * Every node is visited
    * Every node is visited after its leaves
    * The $N$ nodes on the stack are the $N$ children of the current node, where $N$ is the degree

    Post-order verifies the first two, so let's focus on the third part.

    Given a terminal node, $N$ is zero, so it's easy to see that meets the criteria, since you're not pulling any nodes from the stack.

    Becasue the left tree and right tree are visited independently, you know the nodes of the left tree will always be before the nodes on the right tree. That means that in no way will your right tree nodes get mixed into the stack before the left tree nodes. Furthermore, if the nodes preceding you are $N_1, N_2,...N_k$ there is some $N_m$ that is the last node in the left tree, and due to postorder, means that it is your left child.

    We can look at this recursively to know that our algorithm is valid, as long as we pop the $DEGREE$ number of nodes and push one back on.

    17) Write a top-down locally defined function algorithm

        We still have post order with degrees, which complicates things.

        I feel like we have to go right-to-left so that we always hit the parent first. So what if I pushed a copy of the node for every degree, and pop the stack every advancement. When I calculate a result, I need to push the new value all the way through the stack if it's zero (it's a terminal node and shouldn't be used for any other calculation). So if we have 

        ```
        0 0 1 0 1 0 3 2
        K H E J F G D A
        ```

        ```
        - Calc/push A *2                 AA
        - Pop-A -> Calc/push D *3        DDDA
        - Pop-D -> Calc/push-through G   DDAG
        - Pop-D -> Calc/push F           FDAG
        - Pop-F -> Calc/push-through J   DAGJ
        - Pop-D -> Calc/push E           EAGJ
        - Pop-E -> Calc/push-through H   AGJH
        - Pop-A -> Calc/push-through K   GJHK
        ```

        What's left on the stack is the terminal nodes in reverse order (which is the order they were visited on).
        We do have to push-through the stack, which does mean it's closer to a $O(n^2)$ algorithm. You could use another stack to store the terminal node values (remember, if the degree is zero, then we know it's terminal) and pop it back out to get them in left-to-right order and keep it $O(n)

        Now, unfortunately, this wouldn't work with a forest, because the stack would be non-empty for the next node of the forest, so you'd have to know when you are done with a forest.

        For easy sakes, in my program, I'll just output if it's a terminal node.

        See [locally-defined-function-topdown.mixal](locally-defined-function-topdown.mixal).

18) Convert an info/rlink/preorder table to a Info/degree/postorder

    I suspect I'll need a stack to know when a family level is done.

    For instance, with 

    ```
    A B C K
    5 3 0 0
    ```

    I know that A's sibling will be at 5 and B's sibling will be at C.

    I also think I'll have some swapping shenanigans. For instance, with the above, I probably put A as the first node of the degree table, and also on the stack. 

    B comes next, and since I know that it's not the sibling of A, then it must be a child, and come before. Put in B in the second place and swap B and A. Increment A's child

    Then C will come next, and I know it's B's sibling. That means that I put C after the B (but before A since I know A isn't the sibling). Update A's info

    Then K will come next. I know that this is the last node, and it has to be C's, not B's, since it came after C (I can track previous parent in a stack too I guess, popping it on a zero) Swap K with A and then C, and update C's

    In essence, when I have a node, I have to put it at the end, swap it left until it's just before it's parent, and update the parent's number.

    A little more formally:

    - Start at the left node, and we are going to iterate for each node
    - Place this node at the end of the list
    - Swap left until we are just to the left of our parent. Update the parent's degree by 1
    - If there is a rlink, Push the node on the stack, along with where it's rlink is. Everything between the current node and rlink is a descendent of the current node.

    See [rlink-to-degree.mixal](rlink-to-degree.mixal).

19) Given the descendants of each node listed in pre-order, can we prove some math things?

    a) The index of a node + the number of descendants is less than or equal to the total number of nodes. Then, any node between the index and the index+descendants also has it's number of descendants + its index is less than the original index + descendants.

    The key part of this is that we're in pre-order: If a node has zero descendents than, index + 0 is less than or equal to N, since the index is less than or equal to n.

    If the node has non-zero number of descendents, then that many nodes must follow it. Since we're in pre-order, we're guaranteed that many nodes follow it, and we won't go past the total number of nodes.

    Now, for any number between these limits, it will be x nodes past our original index. At most, it can have $D_k - x$ descendents where $D_k$ is the descendents of the kth Node. This is because we know that $k+x$ is a descendent of $k$. So even in the worst case, where we have a straight linked list as a child, we will still have $D_k -x$ possible slots left to go, so that means at worst, $j+D_j \le k+D_k $.

    b) Prove that a sequence of numbers satisfying a) will be the list of descendents of the forest

    Say we have a sequence of numbers $d_1,d_2,d_3...,d_n$. Let $d_i$ be the first node of a forest. We can partition this forest after $d_i$ nodes, since anything past $i$ is not a descendent. We can apply this definition recursively to the rest of the trees in the forest as well as the sub trees starting at $d_{i+1}$. Since the first node will have the largest number of descendents, it is the root of that tree, and recursively, we have pre-order sequence of descendants.

    c) Prove that for two forests, we can make a third forest that is the minimum for each node in each respective forest.

    I'm a little more stuck on this one. If we have a forest with some numbers, such as 

    ```
    A B C D
    3 0 1 0
    ```

    Once we have the first node, all the descendents can be what's shown, or they can be anything less (they can't be anything more, as that would invalidate part a)

    So, given two forests, we pick the smallest of the first node at that position. We'll either pick the existing descendents from that forest, or if the other forest has smaller descendants on a per-node basis, we can use that just fine. For example

    ```
                A                  E
              /   \              / | \  \
            B      C            F  G  H  I
                   |
                   D
    ```

    We will effectively swap out A's descendants with E's first three, and that will be fine.

    Now we will have to look at the next first node in the forest. Note that this could be part of previous sub-tree, but if that happens, we'll just pull those nodes up and make it into it's own tree in the forest (assuming it has a smaller descendant count than whatever non-sub-tree we picked)

    Pretty much, this works becasue in cases where we break up a tree, we can always throw the remaining nodes of a tree up as separate trees in a forest. In fact, in the worst case, every tree in teh forest will be a single node,

