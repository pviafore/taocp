1.  Are there ahny other 12 node trees with a minimum path length besides a complete binary tree?

    Yes, you could technically reverse the tree, or move some of the ndoes at the lowest level to external nodes on the same level (so that they are not all on the left)

2.  Draw an extended binary tree with terminal nodes containing weights of the perfect squares (up to 10) iwth minimum weighted path length



```                 


                        385
                      /    \
                     219    \
                    / \      \
                   119 100  166
                  /  \      / \
                55    64    85 81
               /  \        /  \      
              30   25    36    49
             / \
            14  16
           /  \
          5    9
         / \
        1   4

```

3.  Can you create a tree given a set of numbers. Prove that you can only do it when the sums of the path lengths (to external nodes) l $\frac{1}{2^l}$.

    So say you have lengths 1,1. You take the first length and count out that number of arcs from root all the way to the left and then you start enumerating external nodes left to right. 

    If the sums of the the lengths equal a power of 2 , then we know we have a completely defined tree (This is the same as the fractions of $\frac{1}{2^l}$ being summed up to 1), except for the case of 1. 

    So let's start with an empty tree (one external node). We don't have any path lengths, so let's go with a single node. We have two path lengths of 1, or $\frac{1}{2}$ twice, which is sure to add up to one. As long as we are filing out complete tree, we are at most removing one term, and replacing it with two external nodes with one more length, which will be half each of the original term. This is zero-sum, so any tree that has everything filled out will inherently be able to be constructed from a set of lengths.

    If we don't add up to one, then we will have ambiguity. For any missing $\frac{1}{2^l}$ means we can put that node in one of two different places. For instance, with the path length 1,2, I can put this

    ```
    
        1                    1
       / \                  / \
      |2| 3       or       |2| 3
         /                       \
        |4|                      |4|
    ```

    I am missing an external node in these cases, so it's ambiguous.

4)  Show that given a list of weights, there is an extended binary tree that the terminal nodes in left to right order contain the respective weights.

    Let's start with a huffman coded tree. The weights are not in an ascending order, but we can rearrange the tree so that it is. Find the leftmost external node that is not in order. We need to swap subtrees so that the depth of any external node is not changed, because if all external nodes are at the same depth, then the minimum weight does not changer (and we know a huffman coded tree is optimal).

    So once you have your leftmost node that is wrong, find the node that it is supposed to be. Walk up one tree until it is at the same depth as the other node. Then swap the two nodes.

    For instance:

    ```
    
                    95
                   / \
                  42  53
                 /|   | \
                / |   |  \
            |19| |23| 24  |29|
                      / \
                    |11| |13|   
    ```

    The left most wrong node is 19, and we want to replace with 11. Since 11 is deeper, we are going move up to 24 and swap that.
    
    ```
    
                    95
                   / \
                  42  53
                 /|   | \
                / |   |  \
              24 |23| |19||29|
             / \
          |11| |13|   
    ```

    It should be easy to see that can swap the 19 and 23 to get in the same size. This only works for minimum weighted trees, because otherwise we have something like this

    ```
                *
               / \
              *   2
             / \
            1   3    
    ```

    This could not do the switch algorithm, because we would constantly be swapping the top two nodes without changing path distribution.

    This works because you've never revisiting anything that is in order, and you are always preserving the minimum path.

5)  Skipping this one for higher math reasons.

6) If a t-ary tree is extended with square nodes as in 1), what is the relation to number of square and circular nodes.

    So we know we have n + s -1 edges (since every node has a parent except the root). There are tn edgers (assuming n internal nodes), so tn = n + s - 1 or $(t-1)n + 1 = s


7) What's the relation between external and internal path length of a t-ary tree.

   If you have one node, you have 0 I, and t E. If I add a node, I'll be adding one to I, but removing an E, but adding tE. So for every I, I will at least be adding t edges. So for a path of I, I add t edges for each of them. However, E will aready have I taken care of so we need to remove that, giving us (t-1)n, and finally we need to account for the tn edges off of the internal nodes, so we have

   $tI - I + tn$

8) See above

9)  Show that the numbers on the circular nodes is the weighted path length

    So let's take a tree with a single node, it's external nodes are a and b, so the single node are a+b, so that fits. Now, let's make another single node as a left child with d and e. We have weights d and e, so their parent is d+e and the root is d+e+b. Every time you push weights down, you will have to add them every time up to the root, which means for a path of l, you will add weights x and y l times (which is our weighted sum.)

10) How do you construct a t-ary tree? Construct a ternary tree with the squares up to 100.

    I'm going to start with the same way for a binary tree, just gathering the three lowest

```
                             385
                          /  / \
                           /  |   \
                           /  |     \
                          /   |       194
                        /     |      /  |  \ 
                       91     |      49 64 81 
                    /  |  \   |  
                   30  25 36  | 
                /  |  \       100
               5   9  16 
             / | \
            1  4  0
```

How could it get much more optimal than this? Well, I  guess I could do something like this


```
                          385 
                        /  |    \
                       /  245    \
                      /   / | \   \
                    30    81 64 100 \
                   / | \            110
                  5  9  16          / | \
                 / \                49  25 36
                 1  4
```

Branching out like this doesn't give me anything better.

So, it looks like a huffman coding would work pretty well, you just have to account for zero nodes to fill out the tree.

11) Is there any connection between a complete binary tree and the Dewey Decimal Notation?

    The bottom row will be an enumeration of numbers in dewey decimal notation, and the row above will also be sequential after that as well. I'm not sure past this note, though. You will have every prefix show up two times all the way up to the last number (but not including). For instance, if you have a depth of 5, you will see every number 1000 to 1111 twice.

12) Suppose a node has been chosen at random. Show that the average size of the subtree is related to the path of the length of the tree.

So I think the probabilities of picking nodes are as follows

* 1/n for root
* 2/n for the next row
* 4/n for the next row
* $\frac{n+1}{2n}$ for the final row

This is assuming a complete tree, but I think the probabilities work for imbalanced trees, because every imbalanced tree can be mirrored to give same probabilities (maybe I don't know).

So I know I have a path length of 0, for the first row, 2/n * 1 for the next row, 4/n * 2 for the next row, and $\frac{ln+1}{2n}$. So it seems like if pick a node, on average, I will get depth/path length. I don't know what the average subtree size is for that node, but I bet it is $2(d-cd) - 1$, where where d is total depth and cd is current depth. 

I also realized that it's probably half of the trees where this happens, so let's divide everything by 2 and say that the average subtree size is the total depth /2 /2, or just the total depth. Now I'm going to look in the back of the book and see just how wrong this entire problem was. (super wrong - go check back of the book for the real answer)

13) Design an algorithm that starts with m weights and constructs a extended binary tree with a minimum weighted path length, with 2m weight comparisons.

    So we can maintain two queues, as suggested in the text. As part of this, I'll have to know what nodes I'm putting in which place so that the tree is appropriately ordered. The nice thing is that I'm building from the bottom up, so I can shuffle things around if I need to.

    See [huffman_queue.mixal](huffman_queue.mixal).

14) Show that a huffman tree after k steps will have m-k binary trees in a forest that have minimum weight across all m-k forests.

    I think we can kinda see this by induction. With one node, you are picking the two smallest weights. 
    This is a single binary tree in the forest, and there is no smaller tree by definition of huffman picks. Now, we need to pick two new weights. If we are continuing on the tree, we know that the binary tree will be minimal. But let's say that a new tree is formed. If it's there exists a more minimal forest out there, we would need smaller values than what we have, which is impossible, since we've picked the 4 smallest values. 

15) Show that a huffman like construction will also minimize $l+w$ and $wx^l$, where w is weight and l is length.

    For w+l cases, we are trying to minimnize the maximum length. The maximum length will need to go to the lowest weight (we're still picking lowest weights), because if we were to pick a larger weight, we can always minimize further by picking a lower weight( even a large weight compared to a low length won't make that much of a difference.) The big reason why is that each length is different by each other by 1 (a tree of 5 deep can have nodes that are 4 deep). This means we are never more than one away, and since we aren't doing fractional weights (or if we were, we could scale them to integers), then the weights will scale linearly.

    For hte $wx^l$, it's similar. $x^l$ will only get bigger (and way bigger faster than any multiplication) so that the multiplication of w is not going to be a factor in the long run.

16) If we have two sets of weights, and and the sum of the first is less than the sum of the second, prove that the minimum weight path of the first is less than the minimum path of the second.


    I am not totally sure how to solve this, but I feel like it will be something along the lines of if one weight in the first is really big, another weight in the first has to be really small to balance it out. If weights 1 had 100 and 1 for instance, and the other one had 51 and 51, you still would have similar trees satisfying this condition. Now let's examine if that 1 and 100 are five levels apart, The truth is, the 1 will be very far buried in a tree, and it's impact as it goes through the length multiplications will be far less than the not-as-small number in weights two. At least, for some level this will be true, and the deeper the tree, the more the gap will expand. From here there, I don't know how to actually prove that.

17) Higher math one that I don't understand, skipping this one.