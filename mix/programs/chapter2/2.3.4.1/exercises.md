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

5)  Suppose a graph with N vertices and M edges. DEsign an algorithm that takes input pairs of nodes and prints out a subset of edges that forms a free tree, or failure if its impossible.

    This feels very much like a equivalence class problem (we can treat nodes of a graph in the same equivalence class). I'm also reminded of minimum spanning trees, but that's slightly different.

    This would be a n^2 algorithm. For each edge (which is n^2 on it's own), we would track which nodes are part of which tree. Whenever we connect an edge, we do one of the following:

    * Neither node is part of a tree, mark it with a new number
    * One node is part of a tree, the other node gets its number
    * we are connecting two different trees, we pick the lowest number, iterate through all the nodes (this is also a N^2 if we do this per node) and assign any node in either tree the minimum tree number (it will make more sense looking at the code). At the end of this operation, both sets of nodes (one for each tree) have hte same tree number (Whichever was smaller).

    Now, we can go through edges, and make sure that both nodes in the edge are part of the tree marked as 1 (which should be the lowest). If we come across another tree, it's impossible (there are disjoint trees). We also need to mark the nodes somehow as visited as we go, so that we make sure we aren't introducing cycles ( every edge should not have both nodes marked)

    See [free_tree.mixal](free_tree.mixal).