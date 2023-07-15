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