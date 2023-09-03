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