1) Can lists be described in graph-theoretic terminology?

    If you assume that every list eleemtn is a child of it's list head, I'd say it's a directed graph, that may contian cycles if the lists are recursive. It's also oriented.

2) Can lists have threads like we did with trees?

    Without cycles, you could absolutely do it for making the elements be in a straight linked list. With cycles though, it gets trickier, because you would have an infinite amount of nodes, and you wouldn't necessarily be linking to the next node, you'd be linking to the first node with that atom. In fact, you could set it up so that you probably cycle through a subset of nodes without ever hitting the rest of the nodes, so that's less useful.

3) Prove the validity of Algorithm E

It looks like this algorithm goes down the left side, marking the entire way (this is the combination of E2 and E4/E5). It will go all the way down to the left, then try right nodes. If at any time a right node is not found, then it restores the chain and goes up one (in which case it will try right nodes again, since the left node is marked.)

So, does every node get hit? Yes, every node is marked because we will always go down a left side all the way, go down the right node once, and then all the way down the left node, repeating until we have the first terminal node. From this terminal node, we will always visit the parent afterwards by virtue of going up, and then we will always check the right node (Since the left node is marked). We repeat, and every time we go up, we either go down the right node because its unmarked, or up again becasue both nodes are marked.

Do we always go up the right way? Yes, because we build a linked list of parents as we traverse down the node. Every node can get to it's immediate parent.

4)  Write a mix program for Algorithm E.

    See [mark_e.mixal](mark_e.mixal).

5)  Combine Algorith E and B so that you use an auxillary stack first, and change the tree if the stack is full.


    See [markeb.mixal](mark_eb.mixal).

6)  In the equation $c_1N+c_2M$, where does the $c_2M$ come from?

    The sweep part of the algorithm, where c2 is the cost to restore memory and M is the number of memory cells.

7)  Design a marking algorithm with no auxillary stack on a binary tree, with just a mark, alink and blink (no atom field to mess with).

    Do an in-order traversal using a stack of nodes pointed above. Only mark the node when you visit it, this way, you know whether to check right link (not marked yet) or go to parent ( you have already been marked).

    See [mark_tree.mixal](mark_tree.mixal).

8)  Design a marking algorithm that deals with variable sided links/nodes

    When we descend to a node, instead of setting up the stack to point to list head, move all the way to the last node (Since we know the size). Then, whenever we go up the stack, we restore the link, decrement the position by node size. If we are already marked (current list head), we go up again, otherwise, we alter the link to set up the stack and descend down current link.

    See [mark_variable.mixal](mark_variable.mixal)