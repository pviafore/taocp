So, there's a ton of math questions in this chapter that honestly, I don't even know where to begin. So I'm just going to answer selected questions that I think I have a grasp on.

8) There are six free trees with six points. Draw them and their centroids


```

           * * *              *        *
            \|/                \      /
         * - o - *              o - o 
                               /      \
                              *        *
             o
            / \
           o   *
          /     \
         *       *                 *
        /                         / \
       *                         o   *
                                / \
             o                 *   *
            / \               /
           o   *             *
          / \    \
         *   *    *


             *
             |
             *
             |
             o
            /|\
           * * * 
```

9) Given the fact that for a vertex, only one subtree may have a centroid, prove that there may be at most two centroids in a tree, and that they must be adjacent.

    Given a tree T, we have n subtrees at vertex k. Assume that subtree 1 has a centroid, and no other tree does. So we have subtree 1 with a centroid, all the others without, and vertex k. We know we don't have any cycles, so any other node in a different subtree would have to go through k in order to get to the subtree with the centroid. this means we don't invalidate the assumption up front.
    
    If the subtree that has the centroid has N nodes, and the max of the other subtrees is n-1, then if we look directly at the child node in the centroid subtree, then that subtree has n-1 nodes in its subtree, while going back up to K has at most N nodes (n - 1 + kth node). This first child must be a centroid, as it has N nodes in a subtree as its max.

    So if our center is a centroid (which isn't prohibitied), then the first child of the centroid subtree has to be a centroid too, making it adjacent.

10) Prove that a free tree with n vertices and two centroids consists of two trees with n/2 vertices joined y an edge, we obtain a free tree with 2m vertices and two centroids.

    So, with each centroid, we know that there are at most N nodes from each of them in a subtree. Let's assume that there is one or more nodes between the two centroids. If there are N nodes from each, then those must point towards each other for their maximum node subtree, since if one centroid had N nodes and the other centroid was k nodes away, it would see that the most number of nodes was N+k. This would mean that the centroid could be moved further inward to minimize the most number of nodes of N + k -x, where x is the number of nodes moved inward. Eventually x will = k, which means that the best place to put the centroids are right next to each other (when k = 0).

    Now, if we have two centroids together with a link between them, how do we know that if we split that link there are N/2 nodes on each side. Well, each centroid has at most N nodes in a subtree. This means that beyond each centroid has n-1 nodes. If you split the nodes, you have n-1 nodes plus the centroid in the two trees.

13) What oriented tree from 1 to 10 has the canonical representation: 3,1,4,1,5,9,2,6,5

``` 
             
            5  
           / \
          1   6
         /  \   \
        3   4    \
       /   /      \
      7    8        2
                   /
                  9
                   \
                    10
```

14) True or False: The last entry in canonical order is the root

True, you must handle every terminal node before handling a parent node. so the root node will be last. By the time you get to the root, all other nodes will be removed.

15) Discuss relationships that exist between toposort and and canonical representation

    In canonical representation, you are always processing nodes that have a dependency before their own dependency. The order of the last time a node shows up in canonical order will be a topological sort of all of the terminal nodes. So as long as you visited all the terminal nodes first, you could use canonical order to do the rest of toposort.

16) Design an algorithm that converts from the canonical representation to a tree with parent links.

    First, make a pass through the list to see which are terminal nodes. Then we can put those into a list (a priority queue would be fastest, but I don't feel like it). For the smallest node in the list to the element in canonical order, set the parent node, and decrement a child count. If the child count is zero, set it as a terminal node. Then rescan the list and repeat. As a matter of fact, we can use a child count to track whether we are a terminal or not (0 or not.)

    See [canonical.mixal](canonical.mixal)


26) Draw the (3,2,4),(1,4,2) construction, figure out k, given the following: t=8, colors are RYBRYBRBB and RYBYYBY, and the index sequence is 3; 1,2,3,1; 2,4


Okay, so assuming that 1=red,2=yellow ane 3 = blue, then we know that
 there are 3 red nodes, 2 yellow, and 4 blue (9 total). We also know that 1 arc to parent is red, 4 arcs to parents are yellow, and 2 arcts to parents are blue. We have 9 nodes and 7 arcs (the graph is disjointed).

 Vertex 8 is blue, then we know that the last element in the sequence is 2,4, which means that the 1st blue node in parents is the the 2nd  blue node in k, k+1, etc. Th 2nd blue node in parents is the 4th blue node in k, k+1, etc.

 At this point, I'm not really sure how to take this information and figure out what position t is.