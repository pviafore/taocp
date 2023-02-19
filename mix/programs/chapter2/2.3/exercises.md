1)  How many different trees with nodes A, B, C

    (A (B C)) (A (C B)) (B (C A)) (B (A C)) (C (A B)) (C (A B)) (A (B (C))) (A (C (B))) (B (A (C))) (B (C (A))) (C (A (B))) (C (B (A)))

    There are 12 trees

2)  How about oriented trees -> the first six can be condensed to three

3)  Prove that there is unique path up to the root (namely a unique sequence of k >= 1 nodes for apath)

    When there is just one node at the tree, there exists one unique path, namely that node itself.

    Given a node X whose parent is Xp, and the path from Xp to root Xr is unique, then there must only be one
    unique way that adds the current node. If there were more than one path from a node to its parent, then
    it would not be so, but since there is only one, we can say that we are unique as long as the path from
    parent to root is unique.

    Thus, sionce the root has a unique path, all the nodes beneath it have a unique path, and so on.

4)  Do higher levels of trees appear further down the page (in top to bottom)

    Yes

5)  If A has three siblings and parent B, what is the degree of B

    4

6)   If X and Y are mth cousins, n times removed, what does that mean for a tree

     It means that X and Y share an ancestor that are m+1 nodes above the x , they are in the same subtree.

     They are at two separate levels, that are n levels apart. X and Y must not share the same parent.

7)   Extend the above to generalize ( I don't know what this is askling)

8)  What binary tree is not a tree?
   
    The empty set

9)  In the binary trees of (1), what is the root

    A

10) A collection of non empty sets are nested if x is in y or vice versa, or they are disjoint. Does a collection ever not represent a tree? 

    If a set refers to itself, the intersection will still be that value, so we can have a recursive set which can't be represneted as a tree.

    Knuth also brings up a forest as an exaple

11)  Define a tree as an infinite tree. Then make a bunch of mathematical properties that I'm not going to list here, because I don't know how to approach this problem. Moving on

12) Under what conditions does a partially ordered set correspond to an unordered tree or forest?

    The first condition I can think of is where you just have a tree that has only subtrees of one node themselves ( a list, in essence)

13) What is the node x is numbered $a_1.a_2....a_k$ in the Dewey decimal system. What are the Dewey numbers of the nodes int he path from X to root?

    $a_1.a_2.\cdots.a_k$

    $a_1.a_2,.\cdots.a_{k-1}$

    $\cdots$

    $a_1.a_2$

    $a_1$

14) Let S be any nonempty set of elements having the form $1.a_1.\cdots.a_k$. Show that S specifies a tree when S is finite and when if satisfies: "If $\alpha.m$ is in the set, then so is $\alpha.(m-1)$ when $m > 1$ or $\alpha$ if $m = 1$

    So the definition of a tree is one specially defined node called the root, or if each other node is a subtree.

    So for the root, we have that when $m = 1$, because that just gives us $\alpha$. Now we have to figure out if every other node is a subtree. Following from the definition of the set above, we can say that any child $1.x$ for value x is at least a child node, because it means that $\alpha$ is 1. We can then use induction for the children of those children, and say that every node has a unique parent that can follow back to the root, and conversely, that each node has a list of nodes that themselves are trees.

15) Invent a notation for Dewey Decimal notation for binary trees.

    1 is the root
    
    1 is the left node, 2 is the right node

    so 1.2.1.1 Would be the root's right nodes' left node's left.

16) Draw trees analogous to Fig 21 corresponding to the arithemetic expressions :

    $ 2(a-b/c)$

                 |*|
                 / \
               |2| [-]
                   / \
                 |a| [\]
                     / \
                   |b| |c|

    $a + b + 5c$

                 |+|
               /  |  \
             |a| |b|  |*|
                     /  \
                    |5| |c|

17) What node is $Z[1,2,2]$ in Fig 19

    Node E

18) In List(3) what is L[5,1,1] and L[3,1]

    $(2)$

    Error, it's an empty set so there is no element 1 inside of it

19) Draw a List diagram analagous to (7) for the List L = $(a, L)$. What is $L[2]$ and what is $L[2,1,1]$

                         *[L]
                         /   \
                       [a]    *
                              |
                             [L]

    $L[2] = (L)$
    $L[2,1,1] = a$

20) Define a 0-2-tree as a tree in which each node has exactly zero or two children. Show that every 0-2-tree has an odd number of nodes and give a one-to-one correspondence between binary trees with n nodes and (ordered) 0-2 trees with 2n+1 nodes


    A 0-2-tree one of the following:

        A single node with no childrent designated as a root
        
        A set of two elements X and Y where both X and Y are disjoint and each a 0-2-tree

    Looking at the root's children, there is either 0 or 2 children. For each of those children, they themselves may only add another 2 children (thus adding to the total by 2). So for all children, since 0 and 2 are even, and we can only a multiple of two at each level, it has to be even. Add in the root, and any even + 1 gives us an odd number.

    I'm not sure what the coorespondence is from binary trees and 0-2-trees.

    For example, a binary tree with three nodes

          [a]
          /  \
        [b]  [c]

     Would somehow be analagous to a 0-2-tree

                [a]
               /  \
             [b]  [c]
            /  \   |  \
           [d] [e][f] [g]

     It looks like the 0-2 tree would always be that binary tree with another level filled in if n is odd.

     If n were even,

        [a]
         \
          [b]

    it would correspond to 

            [a]
            / \
          [b] [c]
              /  \
            [d] [e]

    (notice how b is filled in. It seems like a 0-2-tree will always fill in the children of hte next level, and fill in any nodes along the way. However, this is a way of showing a bijection between the two, as there is no other way to map one to multiple through this method.

21) If a tree has $n_1$ nodes of degree 1, $n_2$ nodes of degree 2, and $n_m$ nodes of degree m, how many terminal nodes does it have

    I think it's $\prod_{x=1}^mx^{n_x}$

    Nope, I wasn't even close. See the book for an answer

22) Create a graphical representation of binary trees for European paper sizes


            [A0]
           /   \
         [A1]  [A1]
               /  \
             [A2] [A3]