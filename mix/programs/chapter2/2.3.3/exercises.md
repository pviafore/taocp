1)  If we only had ltag, info and rtag in a level order, could we omit llinks? Can we reconstruct the llink?

    Yes, I think so, consider the following

    ```
    LTAG              |               |   |   |   |
    INFO     A   D   B   C   E   F   G  K   H  J
    RTAG          |        |          |   |   |  |
    ```

    Anytime you do not immediately have ltag, put the node on a queue. Any time immediately after having a rtag, pop the queue; the popped node's llink will be the next node.

2)  The trees stored in preorder could also have Algorithm F (see [locally-defined-function.mixal](locally-defined-function.mixal)) that goes right to left

    See [locally-defined-postorder.mixal](locally-defined-postorder.mixal). In essence, we just go right to left, but the rest of the algorithm is unchanged. This is because we are guaranteed to do our children before our root node, since in pre-order, the root node always comes first.

3)  Write the differentiation algorithm using the locally-defined algorithm and a stack.

    So instead of using I3 for some extra rlink shenanigans, we'll keep a stack in I3. When we have a nullary operator (var or constant), we just push one value on the stack. For unary operators, read one value of the stack, and put the result back on the stack.

    For binary operators, read two values off the stack and push one value on the stack. See [differentiation.mixal](differentiation.mixal).

4)  The trees in normal tree representation contain in (2) is 10 nodes, 5 terminal. However, if there are llinks and info fields sharing a spot, we have 5 llinks and 15 rlinks. What does this look like with n nodes, m terminal.

    So for the original representation each node will have a llink and rrlink, which means we have n llinks and n rlinks

    For the second case though, we only have n-m lliniks, but and the same number of rlinks + n.

5)  For a triply linked tree, does it help to introduce threads at all?

    The only thing I can think of is if we want to make the tree circular in any way, in case you want to iterate over the level again, but it doesn't make too much sense to go to parent (like we did with binary trees), since we already have a link up.

6)  Given a linked list of tree nodes that only have a parent node 
    specified, create a triply linked tree is set up.

    We will have to iterate over the list, but the tricky thing will be updating rlinks without destroying the list. I'm trying to think if I can do this in one pass. I could do it in multiple passes by looping through the list, and setting up lchilds for everything and removing those nodes through the list (removing rlink support), but keeping a pointer to nodes not added. Then you could go through the nodes not added and see if you can add it to the tree (if what you're adding to has a null rlink, then you should be able to). If you can remove it (null out rlink), and repeat this process until your list of rlinks is empty. This is a n^2 algorithm, since you have to iterate the list n times, and you iterate over n-k nodes, where k is in the number of nodes in the tree. Since we have to look at rlinks, we may have k of those so it's n * (n -k + k) times through.

    I'll write this out in [triple-linked.mixal](triple-linked.mixal), and afterwards I'll see if Knuth has a more elegant way. 
    
    After some thinking and a few false starts, I think I have something simpler. More concretely, my algorithm is:

    - Loop through each node
        - If first hasnt' been set and this has a null parent, set first
        - Can it be added as a lchild? If so, add it. Set it's rlink as null
        - Add it as a rlink along the chain of the parent's lchild's rlink (And set your rlink as null)
        - If it has a null parent and first is set, then set it as the first's rlink chain end.