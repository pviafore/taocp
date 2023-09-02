1)  List all cycles from B to B in the graph of Fig. 20

    - B-E-D-B
    - B-A-C-D-B
    - B-A-C-D-E-B
    - B-D-E-B
    - B-D-C-A-B
    - B-E-D-C-A-B

2)  Prove that if V and V' are vertices of a graph and if there is a walk from V to V', then there is a path from V to V'

    So if there's a walk, that means we have a series of nodes taht are adjacent to each other from V to V'. If the walk's nodes are distinct, then of course it's a path, as that's the definition of a path.

    However, if the nodes are not distinct, there must be at least one node N that appears more than once in the walk so that we have the sequence: $W_0, N , W_1, N, W'$, where $W_0$ is all the nodes before N (may be empty), $W_1$ is all the nodes between the first and last occurrence of N in the walk. $W'$ is the nodes after the last occurrence of N in the walk. The nodes surrounding N are adjacent, so we we can take the node before the first instance of N and the node after the last instance of N and learn that they are separated by just one node, N. THerefore, we can skip $W_1$ and just have the sequence $W_0,N,W'$. We can then repeat this step until the nodes are distinct, thus becoming a path.

3)  What is the walk from start to stop that traverses the fundamental path plus one traversal of Cycle C2

    start-e1-e2-e6-e7-e9-e10-e11-e12-stop

4)  If G is a finite free tree in which arrows have been drawn on edges, show that $E_1, E_2...E_{n-1}$ are all equal to zero.

    There are no cycles in this graph, so from any start to stop, we are guaranteed to have a path. Since there are no cycles, there is one arrow coming in to each node on the path (minus start) and one arrow coming out of the node (minus stop), you'll always find that an edge is one arrow in, one arrow out, or 1 -1 = 0.

5)  Write the nodes A - S in terms of Edges

    A = 1 + E8
    B = 1 + E8 - E2
    C = 1 + E2
    D = 1 + E8 - E5 
    E = 1 + E17 - E21
    F = 1 + E''13 + E17 - E21
    G = 1 + E''13
    H = E17 - E21
    J = E17 - E21
    K = E20 + E''19
    L = E17 + E''19 + E20 - E21
    P = E17 + E20 - E21
    Q = E20
    R = E17 - E21
    S = E25

6)  Suppose a graph with N vertices and M edges. DEsign an algorithm that takes input pairs of nodes and prints out a subset of edges that forms a free tree, or failure if its impossible.

    This feels very much like a equivalence class problem (we can treat nodes of a graph in the same equivalence class). I'm also reminded of minimum spanning trees, but that's slightly different.

    This would be a n^2 algorithm. For each edge (which is n^2 on it's own), we would track which nodes are part of which tree. Whenever we connect an edge, we do one of the following:

    * Neither node is part of a tree, mark it with a new number
    * One node is part of a tree, the other node gets its number
    * we are connecting two different trees, we pick the lowest number, iterate through all the nodes (this is also a N^2 if we do this per node) and assign any node in either tree the minimum tree number (it will make more sense looking at the code). At the end of this operation, both sets of nodes (one for each tree) have hte same tree number (Whichever was smaller).

    Now, we can go through edges, and make sure that both nodes in the edge are part of the tree marked as 1 (which should be the lowest). If we come across another tree, it's impossible (there are disjoint trees). We also need to mark the nodes somehow as visited as we go, so that we make sure we aren't introducing cycles ( every edge should not have both nodes marked)

    See [free_tree.mixal](free_tree.mixal).

7)  Do the construction for the flow chart in the text


    Fundamental Cycles:

    - C0 = E0 + E1 + E4 + E9
    - C5 = E2 + E5 + E3
    - C6 = E6 - E2 + E4
    - C7 = E7 - E4 - E3
    - C8 = E8 - E9 - E4 - E3

    - E1 = 1
    - E2 = E5 - E6
    - E3 = E5 - E7 - E8
    - E4 = 1 + - E6 -E7 -E8
    - E9 = 1 - E8

8)  Prove that you can reduce the combining of vertices?

    I'll be honest, this chapter is a bit too math-y for me to follow what it's asking for, so I don't even know where to begin on the proof.

    I bet that the proof has to do with the fact that Ei = Ej when i is the edges coming into a box and Ej is coming out of. As long as the arrows don't start pointing to a new "group" outside of the free tree, we should be able to capture all the interactions with the cyclic edges.

    It also has to probably do with the fact that we are collapsing nodes in the free subtree, as those are kind of a critical path for what we're doing. Skipping this problem for now.

9)  Explain how we can avoid artificial splitting of paths.

    Instead of introducing a new node , we can introduce new paths for the incoming nodes to ultimate node (see picture)

    ```
            A                   A
          / |                  |  \
         B -C -- D             C --D
    ```

    Whatever path BA takes, we can introduce to DA instead (this has to be done for all nodes that preceded C, and if they already contain a path, you may need to go further back.)

10) Prove that a graph has n-1 edges as the minimum number of edges if and only if they form a free tree.

    Start with two nodes, and draw a wire between them, this is two nodes and one wire (N-1) and serves as a base case.

    While there are no unconnected nodes( N-2 more left), find the closest node to the graph and connect it. You are always adding an unconnected node (so no cycles, since it is only adjacent to one node at that point), and you add one node and one edge, maintaining n-1.

11) Given the algorithm in the text, show that it creates a tree that minimizes cost.

    I remember learning this 17 years ago, but I'm afraid all my rigor of proofs has left my brain. Let's see how much I remember. (This is definitely a minimum spanning tree though, I knew it as soon as I saw the name Prim.)

    Suppose you have a tree crated by the algorithm that does not minimize cost. That means that for some pair of nodes V1 and V2, there exists a path that is shorter. If the nodes were adjacent, that means that we picked a cost of Ck, where a shorter path Cj was picked. However, during construction, since we are picking the shortest path first, we should have picked Cj, which is a contradiction.

    Now, if there is a pathway from V1 and V2, there is some intermediary node V1' that is one step away from V1 (Since this is a free tree, we know there is only one path). We look at the distance between V1 and V1', if we should have picked a smaller path, then we would have, hence the contradiction. If no smaller path exists, we repeat this check, examining the path from V1' to V2. We're in essence saying that if there were a shorter path along the way, we should have picked it, otherwise it's a contradiction.

12) Now write a computer algorithm for the above

    Maintain a table of all costs and all nodes.
    
    - MST1. Pick the last node and mark it as connected. Set N = size of nodes
    - MST2. Iterate through the costs and find the lowest cost from an uncconected node to a connected node. Track the unconnected node
    - MST3. Output the cost connection. Mark this unconnected node as visited
    - MST4: Decrement N. If Node is 0, then terminate, otherwise go to MST2

    See [prim_mst.mixal](prim_mst.mixal).


13) Consider a graph with n nodesand m edges. Show that you can write any permutation of integers 1-N as a product of transpositions iff the graph is connected.

    If the graph is connected, we need to permute the nodes by swapping edges at most n-1 times. We are looking to swap nodes at most n-1 times. This means that we can swap two nodes to begin with, and then one unswapped node with a swapped node n-2 times. This should feel very familiar to building a MST. We choose to swap the first node with the desired first of the permutation (one that connects the desired node and existing first), and from there, we just swap in the desired edge that connects that node to the graph. 

    Now, if we had n-2 swaps, then we'll always have one node that we may not connect, and this implies it's on its own since we can never connect it to the group. 




















































































































