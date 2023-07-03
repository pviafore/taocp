1)  Given the definition of a binary tree from a forest, what is the definition of a forest from a binary tree?

    The root node of the tree is the first node in the forest. I am calling this a forest root.

    - Any left-child of node N is a child in the forest of node N.
    - Any right-child of a forest root is a new forest root for a disjoint forest.
    - Any right-child of a non-forest root (which is designated NFR), is a sibling of the NFR

2)  Give a rule that explains the corresopndence between trees and binary trees using Dewey Decimal as an example.

    In a tree with notation $a_0.a_1.a_2...a_n$, for each term $a_x$, you will go down once to the left, and then $a_x-1$ times to the right. So for 2.2.1, you are going left from the tree node and then right (so the second forest), then left once and right once (the second child), and then just left link (first child).

    In binary, we have 11010. The first number will be 1 to indicate tree, and then we will have $a_0 - 1$ zeroes following, then 1 followed by $a_1-1$ zeros, and so on

3)  What is the relation of Dewey Decimal and preorder and postorder of these nodes

    I can't figure this out, let's see what Knuth has to say in the back of the book....

    Ah, I mis-understood what it was asking. I thought it was trying to figure out how 2.2.1 gave a specific location in pre-order or post-order, and couldn't come up with a way. However, the sorting of dewey-decimal 1, 1.1, 1.1.1, 1.2, 1.2.1, 1.2.2, etc.... is inherently pre-order (which is a lexicographical sort). If you sort shorter sequences later, then you get post order (since the root will always be after it's children): 1.1.1, 1.1, 1.2.1, 1.2.2, 1.2, 1

4)  Do the terminal nodes show up in the same order in pre-order and post-order for trees?

    Yes. First let's consider a single tree (since in both pre-order and post-order, you traverse other trees last). The difference is whether you traverse sub trees or your root first. If you are a terminal node, you always just print yourself out. If you are a non-terminal node, you'll see that you still traverse subtrees left-to-right, which is the same in either case. You just put your root before those subtrees or after, but since your root is non-terminal, it doesn't matter when you print that out.

5) If we had a tree where rlink points to the right most child, and llink points to the nearest sibling on the left, then what sort of order do we have when we do preorder on the forest and postorder on the forest


    Let's just take the following forest


                A             G
               /| \          /  \
              B C D         H   I
                 / \
                E  F

    The tree will look like this

        
        G----I 
        |    |
        |    H
        |
        A---------D--------F
                  |        | 
                  |        E
                  C
                  |
                  B

    Pre-order would be A,B,C,D,E,F,G,H,I.
  
    This would be go-left, print your self, go right, or an in-order traversal

    Post-order would be B,C,E,F,D,A,H,I,G

    This would be go-left, go-right and then print yourself, or post-order traversal

6) If we have a binary tree where every node has zero or two children, and we treat that as a general tree and then make a binary tree correspondence, how do the orders equate in that scenario.

    Imagine we have (A (B, C))

    - Pre-order: ABC
    - In-order:  BAC
    - Post-order: BCA

    The corresponding binary tree from the tree is :

            A
           /
          B
           \
            C

    - Pre-order is ABC (same as pre-order)
    - In-order is BCA (same as post-order)
    - Post-order is CBA , which is not related to the orders above

7)  If we say a forest is partially ordered if a node precedes its descendants, then are the nodes topologically sorted


    - Pre-order: yes - we always visit the root before it's children by definition
    - Post-order no, the root will always appear after the children
    - Reverse pre-order - no , as the children will always be printed before the root
    - Reverse post-order - yes, as the root will always be printed before children

8)  Define T≼T' for trees

    It is true when:

    Same rules as for binary trees, but with the following:

    Check the right trees first (since those are siblings) before the left trees.

    I'm not sure what else would be different.

9)  Show that the total number of nonterminal nodes in a forest has a relation to number of empty right links

    I'm missing a definition somewhere, because these have the same number of right null links


            *                           *
           /                           /
          *                           *
           \                           \
            *                           *
                                         \
                                          *

    But a different number of nonterminal nodes.

    The trees that produce these are as follows

            *                           *
           / \                         /|\
          *   *                       * * *

    Oh, I guess we're measuring non terminal nodes in the forest, not in the tree.

    In that case, we know terminal nodes will always have a left link, and that if you have a non-terminal node, you are part of a sibling chain that will have one null link at the end, and any nodes to your left will have their own sibling chain as well. This keeps it even, except for the sibling chain at the root, so you have one more link than nonterminal nodes in the forest.

10) Write a proof that two trees are equivalent or similar.


    Oooof, we'll see how bad I am at writing proofs.

    Let the nodes of trees T and T1 be respectively

    $u_1,u_2,...u_n$  and $u'_1,u'_2,...u'_{n'}$

    in pre-order.

    T and T' are similar iff n=n' and $d(u_j) == d(u'_j)$ for $ 1 \le j \le n$, where d(u) is the degree of the node u

    And they are equivalent if 

    $info(u_j) == info(u'_j)$ for $ 1 \le j \le n$


    Proof. Let's start with a single node. 

    This will have a degree of zero, so it will only be similar for other trees with a single node (n == n'), and those trees will have a degree of zero as well. So two single node trees are similar. It's pretty easy to see if the info is the same between both, it's equivalent as well. 

    Each sub-tree can be checked recursively, so for induction to work, we need to show that the construction of a tree will uphold this property.

    Since we have a list of nodes in pre-order, we can translate that to a list of degrees. For the root node to have a degree of $k$, we know we can split up the rest of the pre-order nodes into $k$ partitions. Similar, for T', we can split up into $k'$ partitions. Each of the partitions will be a sub-tree of the root. 

    If the two roots have different degrees, then they will have a different amount of subtrees, and thus, will not be similar. For each parition, there is some $m$ nodes associated with that partition. We can compare the roots between $m$ and $m'$ to see if their degrees match, and then keep going recursively.

11) Draw e^-x^2


                    **
                    / \
                   e  NEG
                       |
                       **
                      /  \
                    x     2 


        **
        |
        e-------- NEG
                   |
                   **
                   |
                   x ------ 2

12) Give specifications to the PWR operation


    PWR:

 
    $Q <- TREE("+", MULT(Q, MULT(COPY(P2), TREE(**, COPY(P1), TREE("-", COPY(P2), 1)))), MULT(MULT(TREE1("ln", COPY(P1)), Q1)), TREE(**, COPY(P1), COPY(P2)))$

13) Write a copy routine for the differentiation routine

    See [differentiation.mixal](differentiation.mixal)

14) How long does the copy take in the case of n nodes?

    Store/restore/jump overhead:  20

    Each node gets allocated gets allocated 8 cycles (C1, C2, and C4)

    Each node gets a check to see if it has a left node or right node, which is a total of 6 cycles

    For each left node, we have 3 cycles to store the link, and 4 + 5*right nodes

    For each right node, we have 11 cycles

    Then we have pre-successors, which will be 7 + 9*right nodes, * 2, + 5 cycles

    Thus, we have 20 + 6n + 7l + 5r + 11r + 19 + 18r, which will be 39 + 6n + 7l + 34r.

15) Write a div routine for the differentiation routine

    See [differentiation.mixal](differentiation.mixal)

16) Write a pwr routine for the differentiation routine

    See [differentiation.mixal](differentiation.mixal)

17) Write a program to do algebraic simplification

    I'm not going to spend a whole "term project on this", but I will list out the reductions done in the `REDUCE` method in [differentiation.mixal](differentiation.mixal). This should serve as a reference of an idea of what's possible.

    The idea will be to go in a loop as long as reductions can be found, and exit when you have no new reductions.

    First, arithmetic operations done within a single node:

    - Arithmetic on constants
    - Handling zero nodes (todo: x^0 )
    - Handling one nodes  (todo - 1 on the left (1^x) and one and the right for * and /)
    - Multiplication of constant and a fraction
    - Double Negatives 
    - Collapsing of power (x^y^z = x^y*z) (when the power is on the left, todo- on the right)

    Then, checking for optimizations across the tree:

    - Factoring out terms of X 
    
    If none of these apply, we do the following as a second pass:

    - Turn non-constant divisions into the inverse

    I'm not going to go much further than this, because it shows replacing nodes with other nodes, handling nodes at different depths of the tree, replacing nodes with values, and rearranging nodes.

18) Create an efficient algorithm that transforms an array of node-to-parent lookups to a pre-order doubly-linked list.

    For simplicity, I'm keeping the info of the nodes to a single byte, so that a double linked list node is just one word wide.

    Assuming that a table of N words can be used as a lookup (which means linear space considerations), we can create a lookup of node to farthest-right child.

    If it's zero, then you need to insert to the left link and set the thread to root. If it's set, you need to set right link, copying it's right thread to what you just inserted if need to.

    So you iterate through the nodes, looking at their parent and inserting according to the rules above. If you see a zero, then you know that node is a root node -- save if off.

    After this linear time transformation, you have a binary tree representation of the tree, and even better, it's threaded.

    Now, we can pre-order through the binary tree with the help of the threads. You can create a new list of nodes by just doing the pre-order, and saving off the llink/rlink setup. At the very end, set the last node's right link to the root and the root's left link to the current.

    So we have O(n) transformation of the tree, and a O(n) transformation to the tree. We have 2 space allocation of n nodes, one to store the to-child-lookup and one for the doubly linked list.

    If we need to do a in-place, then it's probably worth keeping a stack instead of threads, since we can store more information (such as the right thread of the node, since we might overwrite that as we work.)

    I was trying to think if there's a way to do it without having to create the intermediate tree, but I don't know how we'll know what the next node is in pre-order in just one pass. 

    For the non-in-place double transform method, see [parent-to-preorder.mixal](parent-to-preorder.mixal)

19) Define an algorithm to define ≼ for a free lattice, given the six rules in the book.

    So for this, we're going to have to take two trees and see if something satisfies the relational operator. We can track the operations for the two trees, building a new tree as we go. The new tree can keep track of not just variables but a list of variables: so that when we see a V b ≼ a, we can keep track of a, and b moving forward (a being left link of the root and b being the right link of a)

    We'll start by setting up a tree with a node being ≼ and each formula being a left link and right link (in the binary tree representation of a tree). We'll then go post-order through the tree to see if we have any relation operators, in which case we'll simplify given the rules. 

    Our nodes will be one of the following:

    - Or
    - And
    - Relational
    - Variable-set
    - Join
    - Meet
    - False
    - True

    I'm going to do a similar reduction algorithm as my differentiations where I will be constantly looping through the tree looking for reductions, until no more reductions are found. By the end of it, you'll get the following: 

    - and nodes
    - or nodes,
    - true nodes
    - false nodes

    Once we get to here, we can do another post-order resolution of boolean logic. 

    See [free-lattice.mixal](free-lattice.mixal).

20) Prove thatif u and v are nodes of a forest, u is a proper ancestor of v iff u precedes v in pre order and u followsw v in post order

    In order to be a proper ancestor, they need to be in the same tree.

    If u precedes v in preorder, then u is either part of the same tree and a root visited first which is u, then v, or v is in a separate tree (traversed after u's tree)

    However, if u follows v in post-order, then either u is in the same tree (nodes in the tree are visited after the root, so if v is visited first, u may be a root of it), or u is in a following tree. 

    If v is in a separate tree, then u would precede v in post-order (since we have to traverse our tree first).But, since it doesn't, we have to assume they are in the same tree.

21) How would we do ternary operators in the differentiation routine?

    Assuming commutative property of terms, you could *fold* the terms together. Say you had a root node R which contained a ternary operator (say +). The left link would be the first operand, and there would be a chain of rlinks for each operand. You would calculate the first two, and store it into the first llink, and then calculate the llink and the second rlink, then llink and the third rlink, storing in your llink the entire way. When you are done, you copy up that value to your root node R, and clean up all the children.

22) When talking about embedding trees in one another, prove that T can be embedded into T' if T has just one more node, or if they both have more than one node and either T is in the leftmost of T', T is in the rest of T', or left(T) is in left(T') and rest(T) is in rest(T')

    So for adding one more node:

    Inserting a node into the T' will not change the order of nodes in pre-order and post-order - at most it will insert a new node between existing nodes.

    Now, for more than one node, I don't know how to tackle this. If your entire tree is in the left subtree (and this recurses) then it's easy to say yes, it's in there. If it's in the rest of the tree, then it's also easy to say, yup, because the insertion of extra nodes will not change order of pre-order and post-order in these cases.

    However, if the root node is needed for embedding in the left subtree case, then you have to check both left and rest, to make sure you can handle nodes in between the root node of T and l(T)

    I have no idea if the converse holds, but I suspect not.