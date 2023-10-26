1)  If a set contains finite sequences of positive integers and states that this is essentially an oriented tree. What is the root of the tree, and what are the arcs?

    The root is the empty set, and the arcs are the sets in which the starting set contains the fin set that is one element bigger.

2)  Show that if rotation of tetrad types is allowed, you're always allowed to tile the plane

    Assuming

    ```
    \  A  /
     \   /
    B  x   C
      / \  
     / D \
    ```

    Then you can do the following

    ```
        \A/   \D/
        BxC   CxB
        /D\   /A\

        \D/   \A/
        CxB   BxC
        /A\   /D\
    ```

    You can repeat this in any direction.

3)  If it is possible tile the upper right quadrant of the plane when given an infinite set of tetrad types, is it always possible to tile the whole plane?

    If we have an infinite set of tetrad types, then I think we have an infinite set of children, which means we can't necessary prove whether we can or can't (becaue due to the infinite nature, we'd have to check them all exhaustively ). I don't know if I can actually prove this, but based on the messaging of this chapter, this is what I think it's going for.

4)  If you can tile the plane, can you tile it with a toroidal solution. Use this assumption with infinity lemma to design an algorithm that given the specifications of any finite set of tetrad types,determine in a finite number of steps if you can tile the plane with these types.

    So the goal is to first find something that tiles the upper right quadrant in a rectangle. So first we need to find all the cycles in the horizontal direction and vertical directions. We know that at most, ther are n tetrads in either direction where n is the number of finite tetrad types (but you'll have to check about n! in each direction to find the cycles - however there's a very early out for them.) The idea is that if you have a cycle, you know you can tile to the left and down indefinitely, which lines right up with Wang's thoerem. Now, you have to find the cycles that line up all the way (which you can probably do with some nice tree algorithm, but I don't see it yet.). Finite running time, yes, but a long running time.

5) This problem's description terrifies me, so that's a no from me. I think you'd need to show that there's no cycles (I don't really know though.)

6) There are some proofs here that I don't even know 

7) see above

8) see above