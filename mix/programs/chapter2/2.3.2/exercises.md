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
