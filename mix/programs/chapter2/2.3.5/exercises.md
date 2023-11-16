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

9)  Write a sweep that compacts nodes, while preserving A and B links

    So we need two poineters, one that goes through memory forward, and one that goes through memory backwards.

    For each node you iterate backwards, first see if it's marked. If it's not, keep going.

    If the node is marked, check if either of it's alink or blink is pointing to a relocated addres. If so, update the links (more on relocation later)

    Then, move the first ponter forward until you find an empty space. Move the marked node into the empty space, and change the original marked node to atom = 2 and put into the ALINK where the node ended up. This is the relocation information.  

    Terminate when you are out of memory, or when your first pointer meets the marked pointer.

    This is a single scan of memory giving you constant time (increment/decrement pointer, udpate of links, or an optional move and indicate relocated 

    See [compact_sweep.mixal](compact_sweep.mixal)

10) Copy a list that uses list heads

    The big trick here is handling cycles and multiple nodes coming into a single element. We don't want to add more storage anywhere, so we are limited to the ref node of tree heads. In the original list, we can have the list heads point to the new list heads, that way, if we ever reach a node with a zero in ref, we can assume we need to allocate a new node, and if we reach a node with a nonzero ref, then we just have to point to the link provided. This will require a reiteration of the original tree to make list heads set back to zero.

    When we are iterating, we can see if we are a list head, and if we are, go one to hte right, then go down. If we are not a list head, try to go down (assuming you are not an atom). As you go right, or down, you will need to keep a stack of nodes kept throughout the tree to know how to get back to the previous node. On a right move, you know who the list head you came from (and we can use mark bits to know whether we went down the ref or to the rlink). On a down move, you know the node that was your parent.

    See [list_copy.mixal](list_copy.mixal).


11) Write a program that checks if two lists are equivalent when they are expanded.

    If the lists were finite and DAGs, no problem, but now you have to figure out how to expand two lists. Imagine A: (a: (a: B)), B: (a: B), and C(a: C) are the same, even though they are not going to have list heads at the same place or not. In each case though, it's a series of repeating "A" values.

    So, for each list head in one, we probably have to use the ref field to store the matching equivalent node (on both sides). For a given list head, it is equivalent to another list head if it has the same number of nodes to the right, and for every node underneath it, it will check the list head underneath it. Since we can see if a list head is already used or not, we can always compare if they point to a similar structure.'

    See [list_check](list_check.mixal)

12) How can you do garbage collection in real-time, with strict limits on list operation runtime?

    In order to do this, you have to be able to chunk the operations into smaller atoms. Then for each operation on a list you can do a little more garbage collection to speed things up.

    For example, here are some atomic operations that you can do

    Mark Phase:

    - Go to next top-level tree and mark it
    - Go to next node in a tree and mark if unmarked
    - Go up a node in a tree and restore links
    
    Sweep Phase:
    - Iterate one node and check if marked
    - Return node to memory


For each of these 5 pieces, there is a cost $(CM_1, CM_2, CM_3, CS_1, CS_2)$. There is some maximum value of these that defines the minimum budget B that can happen as part of a list operation. Each list operation (or whenever a programmer has budget), they can do these five operations until they run out of their budge. 

Now, there is some care, mainly:

- What happens if a node is allocated during hte sweep phase, and we've already iterated past it, or we haven't iterated to it yet.
- What happens if a node is deleted from a tree mid iteration of tree during the mark phase.
- What happens if a node is added to a tree mid iteration of a tree during the mark phase.

To address these, any new allocation should always come with a mark bit set, and only the sweep operation should reset it to zero. This means we are conservative when we are sweeping and new nodes will never be swept. This also covers adding a node to a tree mid iteration.

Furthermore, we can't modify a tree in-place, an auxillary stack should be used to track which nodes we've seen. 

Lastly, when removing a node, we should not modify it's links. If it's on the stack, it will still be marked, and anything that it touches will be marked, but that will be caught on consecutive sweeps. It means we may not correctly sweep something in one go, but we'll always get it on the next go.