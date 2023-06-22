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
