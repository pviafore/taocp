This is another set of mostly math exercises that I don't know how to solve, but I'll put my musings in here for posterity sakes.

1)  Find a simple correspondence between binary trees with n nodes and dissections of a n+2 sided convex polygon into n triangles.

    I think depending on how you slice the polygon, you are breaking the polygon into two halves, one with N+2-k points and one with k points. This is the bifurcation of a binary tree, and depending on how big you are picking k, dependso n how deep it will go. Imagine a pentagon that is separated into a triangle and a square. One left of this binary tree is the triangle and ends with one deep (each leaf is a triangle), and the other square can be further bisected into two triangles:

    ```
    
                /\
               /  \
              |    |                         *
              |    |
              |____|    
    ``` 


    ```
    
                /\
               /__\
              |    |                         *
              |    |                       /  \
              |____|                      *    ?
    ``` 
    ```
    
               /\
              /__\
              |\  |                         *
              | \ |                       /  \
              |__\|                      *    *
                                             / \
                                            *   *
    ``` 

2) How about the conjecture for non-overlapping diagonals in a r sided polynomial.

    Similarily to number 1, the way you draw diagonals would be the number of trees where the number of points in each region would be the number of nodes in a tree you could draw, splitting up into triangles again. It's not just enough to dissect a polygon, but you are looking at multiple dissections at once.

    Beyond this (and the next two problems), I don't know wanything else.