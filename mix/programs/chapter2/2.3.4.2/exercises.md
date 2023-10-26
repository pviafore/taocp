1)  Prove that if V and V' are vertices of a directed graph and there is an oriented walk from V to V' then there is a simple oriented path from V to V'

    There is a list of nodes $V_0, V_1, ..., V'$ that may not be distinct (Since it is just a walk). We have to find the distinct set of nodes that forms a path. THere is some index $k$ such that $V_k$ will appear twice in the walk. If it does not appear, then we know we have a path. 

    IF we do have a $k$, then we can order the arcs as such: $A_0, V_k, A_1, V_k, A_2$ where $A_0$ is the arcs before the first instance of $V_k$, $A_1$ is the arcs between the first instance of $V_k$ and the last, and $A_2$ is the nodes after the last instance of $V_k$

    Now, we can remove $A_1$ and one of the instances of $V_k$, since if we can leave $V_k$ more than one way in the walk, we can elide whatever's in the middle. We repeat this until there are noly distinct nodes.

2)  Which of the ten fundamental cycles earlier in the book are oreinted circles.

    $C_0, C_8, C_13, C_17, C_19, C20, C25$ (anything with all positive signs since a negative sign indcates an arrow going the wrong way.)

3)  Draw the diagram for a directed graph but not rooted.

    Interesting, because if it's connected, every node has to get to every other node. A root has no outgoing edges, so I feel a balanced graph would be a great example of this:
    
    ```

          ___           ___
         |   |  <----- |   |
         | A |         | B |
         |___|  -----> |___|
    ```

4)  What finite direted graphs can be toplogially sorted? Which ones can't?

    Any cyclic graph can't be topologically supported, since there is no first node in a cycle. Anything acyclic will be able to be topologically sorted.

5)  If G is a graph with an oriented walk that the last node being the same as the first node. PRove itis not an oriented tree.

    So, this is the definition of an oriented cycle, so we have to find a way to say how a cycle does not have a cycle. In an oriented tree, there has to be some sort of node that has an out-degree of zero, as that's the root. The root cannot be in the cycle then. If the root is not in the cycle, then one of the nodes in the cycle must point to the root, but since it's already in the cycle, that means we have at least two outgoing arcs, which is a no-no in the oriented tree.


6)  T/F: A direted graph that is rooted and contains no cycles and no oriented cycles is an oriented tree.

    True: C) is handled because it is considered rooted. Since it is rooted, we konw that every node can point towards the root (no nodes diverge into different sub-trees). 

    Since we have a root, we also know that there is no intial vertex from the root, because it would either go to another node (causing a cycle and we can't have that), or a separate node (and that node would be the root, contradicting us).

    Finally for A, for any node to have one outgoing arc - let's suppose that there are two arcs coming from a node. We know it doesn't diverge because it's rooted, and if it goes to two other nodes, those nodes must go to R, which means we have a cycle along the two paths. So that's a contraditction.

    (Knuth said that it's false on a techniality - one node may have more than one arc to another node)

7)  T/F A directing graph satisfying A/B of oriented tree and having no orineted cycles is an oriented tree.


    False: So we are seeing if the condition of no oriented cyles will mean that there is a path from every vertex to the root. False, as a graph can be disjoint. However, each disjoint sub-graph will be an oriented tree on its own. 

8)  Study the automorphism groups of oriented trees, namely the groups consisting of all permutations of the vertices and arcs.

    :shrug: An automorphism is a mapping of an objet to itself where it preserves structure/adjacency. I found [this](https://www.birs.ca/workshops/2016/16w5048/files/Wagner.pdf) to be useful in introduing the topic. There can be as many as $(n-1)!$ automorphisms. The rest of the math is beyond me so far though.

9)  By assigning diretiosn to the edges, draw an oriented tree with a root of G from page 363.


    All pointing upwards
```
                        G
                /       |       \
              F         C         H
          /   |      /  |  \     /  \
        K     E     A   B   D    I   M
      / | \
    J   N  L
```

10) Design an algorithm that takes an oriented tree and converts it to a free tree with a different root.

    Suppose you have nodes $R$ and $V_j$ where $j$ is the new root index. We essentially need to make it so that all links point to $V_j$. Now, any descendent of $V_j$ already points to it, becasue they would have to go throug $V_j$ to get to the root ( $V_j$ connects to the root, and if a descendant has a different path, that means we have a cycle and no longer an oriented tree.). Any other node will need to go to R somehow, and if it can get to R, it can now get to $V_j$

    So, we are going to perform a reversing operation: 

    - P1. Save 0 as L. Set N =  $V_j$
    - P2. If PARENT(N) == 0, set PARENT(N) to L and terminate. Otherwise, X = N, N = PARENT(N), PARENT(X) = L, L = X and repeat P2.

    See [reroot.mixal](reroot.mixal)


11) Change the algorithm from last section that determines free tree, and have it print out free trees and all fundamental cycles.

    So, we can use the previous algorithm to determine free-tree, which also checks if a cycle is present (Both nodes are connected already when checking edges.)  However, we can store these arcs in a new list for after the tree is built.

    Then for each edge that's in a cycle, we can do the following (assuming arc from a to b).

    We can re-root the tree so that a is the root. Then, we can start at b and just walk parents until we get to a. So we have a->b and then the links from b->a which is a cycle.

    I do have to tweak the algorithm to store parents along the way though when I figure out the free tree.

    However, after a day of thinking about this, I realized this only works if it's an oriented tree, and we have no guarantee that we are.

    So what we can do is just do BFS on a connected node to find out where the cycles are. Honestly, I don't feel like doing a BFS with dynamic storage in mix, so I looked at the back to get a hint.

    I saw in the back that it mentioned 2.3.3E, which is the equivalence tree, so it seems like we want to get it so that each oriented sub-tree is separate, but I didn't know how to join them. So, what Knuth suggests in the answers is to connect the root to the other trees, but indicate that the link is going a different way. The key is always propagating the connection up to the root of each tree, and we assume cycles will make their way to the root. We will reroot nodes to make sure that cycles will always bubble up to them.

    See [cycles.mixal](cycles.mixal)

12) Is the degree of an oriented tree the same as an in-degree or out-degree

    In-degree as the number of sub-trees is the same as the number of trees pointing to this node.

13) Prove that if a direted graph G has a root R, then there exists an oriented tree with root R, but all the same vertices as G.


    This seems inutitive to me, so I'm not sure how I want to go about proving it. I know that for every node in G, there is a path to R. For it to be an oriented tree, we already have a root, we have to prove that each node can be reduced to only having one outgoing arc (the big missing part of our graph.)
    

    Say we have a node with multiple arcs leaving it. We need to make sure that each vertex it connects to is part of the actual sub-tree. We can pick just one of them to connect to, and the rest should be connecte to the root in their own way.

14) Let G be a balanced digraph from the book, and G' bet he oriented Subtree of V0 V1 V2 and arcs e01 and e21. What are all teh oriented walks that meet Theorem D

    V1 is the root

    * e12,e20,e00,e01,e10,e'01,e'12,e22,e21 (note that the quoted edges could be replaced with their counterparts for four variations on this)
    * e12,e20,e01,e'12,e22,e21,e10,e00,e'01 (and it's four variations)

15) True or false: A directed graph that is connected and balanced is strongly connected.

    Yes, since it is balanced and connected, there exist a Eulerian Trail, which means that every edge must be traversed once. Since it's connected, there are no isolated vertices, which means we can get to any vertex since there is an edge going into it (the Eulerian Trail will lead us to thato edge.)

16) In a game of "Clock", prove that you win if and only if you have an oriented tree of bottom cards to the pile they point to.

    So we only care about bottom cards because that triggers a game end condition. Cards on the top will inevitably point to a bottom, so what matters is the that whichever order of bottom cards you exhaust, they always point to a valid pile (if they ever point to an empty pile, then you lose)

    So, we can't have cycles (once a pile is exhausted, you won't ever get a card directing you to that pile again). However, you can exhaust a pile too early, which means that the oriented tree (of how you exhaust piles) ends, which means you'd have more than one (and the entire graph wouldn't be rooted, since you can't get to some root). The root is the last card of the last pile.

    So if we are able to exhaust all 13 piles, we have a root, everything points to the root, and a pile will only ever point to one other pile (So we have only one outgoing arc)

17) What's the probability of "Clock" that you win, and what's the probability that there are k cards face down.

    So we only care about bottom cards, so we can just worry about one copy of each card (the math is the same regardless of how many copies of the card you have).

    If you have one pile, you have a 1/1 chance of succeeding.
    
    If you have two piles, the first bottom card will either point to another pile, or it's own pile. If it points to it's own pile, we can't finish the other pile, so we have a 1/2 chance of succeeding.

    If you have three piles, you can fail by the bottom card being your own pile, or you can pick either of the other two piles. You have a 2/3 chance of picking correctly, multiplied by a 1/2 chance. Numerators and denominators will cancel out except for the first numerator (1) and the nth denominator (n).

    This means we have 1/13 as a possibility (I accidentally saw this answer, but it was fun proving it to myself with the above explanation).

    Now, for k cards, that means we've played 52-k cards so far. However, we have to play at least 4 cards to play, so we know it's a probability of 1/52 * 1/51 * 1/50 * 1/49 before we get to an actual probability. So 49-52 cards is impossible. So if 48 cards are available it's $\dfrac{1}{52*51*50*49}$. In order to have 47 cards, we would have $\dfrac{1}{\dfrac{52!}{47!}}$ This means to have k cards, we have $\dfrac{1}{\dfrac{52!}{k!}}. But, this is only if there isn't a solve, since with k = 0 we already have 1/13.

    Now let's see if the above is right. I looked at Knuth, but I'm not sure if I'm equal or not, as I went a bit diferently then them.

18) Given a graph G that has a matrix (n + 1 vertices and M edges), then
    we have a matrix $A_{ij}$ = +1 if the arc goes from i to j, -1 if it goes from j to i, 0 otherwise. If m= n, show that the determinant of A0 (A0 is the first row and column deleted) is equal to 0 if G is not a free tree, and $\pm1$ otherwise.

    So, if m = n, that means we have 1 more edge than we do have the same number of edges as nodes -1, that either every node is connected (With no cycles), or we have disjoint trees that have cycles in them.

    So with A0, it has the column deleted (since there is no e0 ), which essentially deletes all edges to and from the first node. NOw I just need to know what a determinant is (I forget all my linear algebra, not that I paid a lot of attention in the first place.)

    So, after a quick meander to wikipedia, I feel no closer to the answer, but I'll talk through some things to see if it sparks anything.

    So if there's node that is disjoint, it means that column of it would be zero. That means that we're going to have a determinan of zero (either it's zero multiplied by it's cofactor), or when this row or column is in another cofactor, it will also be a zero. 



    Now can I prove this if zero for disjoint nodes for more than one node that is disjoint, but connected to each other.  There will be a cycle (since the edge that would connect them now contributes to one of the sub-graphs), so something about the cycle probably forces a determinant to zero (or disjoint grouping are alwys zero). Let's say we have a group of 3 with a cycle and group of 2. We have n=4 and m = 4.

    Edges would be (0,1), (0,2), (1,2), (3,4)
    The matrix would look like this

    ``` V0  V1  V2  V3  V4
    e1: 1   -1  0   0   0
    e2: 1   0   -1  0   0
    e3: 0   1   -1  0   0
    e4: 0   0   0   1   -1
    ```

    It's worth noting that if there is a disjoint graph, then you can rearrange the matrix by swapping rows and columns until all the nodes in one group are on the lower right

    ```
    A       0
    0       B
    ```

    So one interesting thing is every row has at least one 1 and -1. If we are deleting the first column, that also means that we are eliminating any outgoing edges from V0. What's interesting is that this leads to an imbalance as the ingoing edges to V0. I don't know if this is significant or not.

    Due to those disjoint nature, each diagonal will always contain a zero (note that I have not proven this, but I'm assuming so), no matter how you rearrange the problem. I think it's because I can add one column to another to zero it out, whereas if you don't have any cycles, you won't be able to zero out a column.

    If you don't have a disjoint section, it will be possible to create one diagonal that is all 1s and all -1s (this is pretty much saying that you can rearrange edges or renumber nodes so that you can have every node have a edge going to it or from it. Since you have m edges with n+1 nodes and m=n, you're saying that every node is connected to every other node, thus given your free tree.

    Now, what about $A^T_0A_0$'s determinant. This supposedly is the number of free subtrees. This part I'm less sure about. It might be something to do with out-degrees or number of permutations of links minus mandatory links or something like that.I think when you multiply the matrix transpose by the matrix you're mapping how many ways a node can get to another and the non-zero diagonal is the product of all nodes options. (And the opposite diagonalization is the amount of overlaps?). As with all of the following answers, dont' trust my incredibly poor linear algebra / proof / math skills and check with Knuth what the answer is. (Yeah, after looking it up it involves the Binet-Cauchy formula and I have no idea what that is.)

19) SHow that if we have a matrix of n+1 x n+1 size, and if a00 is 0 and ajj = 1, then the determinatnt of A0 is [G is an oriented tree with root V0].  Then, show that the det A0 is the number of oriented subtrees of G rooted at V0.

    So if A00 is 0, that means that there are no arcs from A0 to anything else, meaning everything else must come to it if it's connected.

    Ajj beign 1 means that that there is some terminal node that only has one arc comiing out of it. I think this means that from the previous problem, we will have $\pm1$, but this is a pure guess. This is beyond me unfortunately (and I fear a lot of the future ones are too.)

20) I'm going to skip some of these math problems that I have no idea where to start, sorry.

21) See above

22) Let G be a balanced directed graph with no isolated vertices. Show that the number of Eulerian trails is the sum of out degrees multiplied by the number of rooted orieented trees at V1 multiplied by the product of out degrees -1 factorial.

    So if I think of eulerian trails, I can think of each out degree being used for each oriented tree, and you can pick different combinations of oriented trees, which gives you the factorial part of it, but I don't know how to pick the rest of the function apart.

23) Skipping this one

24) With a graph G that satisfies Kirchoff's law, and E0 = 1, prove there is an oriented walk in G from fin(e0) to init(e0) that does not use E0, and that edge Ej appears exactly Ej times.

    So if we have Kirchoff's law, the the number of arrows coming in are the same as the number of arrows coming out. This means that we have cycles, so we know we don't have to go through E0. Now, for every time we enter a node, we will have to leave that node, and while I believe that will play into the Ej times, I don't know how to prove it.

25) Design a cmoputer representation for directed graph taht generalizes the right threaded binary tree representation of a tree. Use ALINK, BLINK, ATAG, and BTAG, there is only one node for each arc of the directed graph and the if we add an arc from R to some new vertex, then the representation of hte graph is the same as the right htreaded representation of the oriented tree and that changing ALink/ATAG with BLINK/BTAG is the same as reversing the arcs of the graph

    So each vertex in the tree is actually the edge from one vertex to another in the graph. I'm going to have to do something along the lines of having something on the A side represent an arc that leaves the origin node and B repersent sibling arcs for arcs that enter the same node that I enter. That way, if I reverse them, I'm effectively changing my incoming and outgoing arcs. Now, what about the tags? They help with the threaded nature, and probably say that if I don't have any more arcs that enter the same node that I enter, I thread up to the arcs that enter the node that I originated from (and vice versa for A). I think this does what I need for figuring this out.

26) Exercise about markov chains - skipping

27) Skipping due to lack of math knowledge

28) Skipping due to lack of math knowledge