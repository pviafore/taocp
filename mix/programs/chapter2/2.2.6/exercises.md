1)  Given a $m x n$ 1-indexed matrix, what is the location of J,K if every piece is 2 words

    $LOC(A[0,0]) + 2(n(J-1) + (K-1))$

2)  Give the general formula for location of n-dimensional array (assuming c words) when upper and lower bounds are arbitrary (may not start at zero)

    $LOC(A[I_1, I_2, I_3, ..., I_k]) = LOC(A[0,0,0,...,0]) + \sum_{r=1}^k a_r (I_r-l_r)$

    where

    $a_r = c \prod_{s=r+1}^k (u_s - l_s + 1)$


3)  Give the formula for a triangular matrix that starts at index 1

    We can just make sure that each number is subtracted by one

    $LOC(A[J,K]) = LOC(A[0,0]) + \dfrac{J(J-1)}{2} + (K-1)$

4)  Come up with the formula for the upper triangular matrix and show that we can 

    So B[0,0] is really A[0,1] and B [0,n] is really A[0,n+1]

    Sequentially, this B[0,0] starts A[0,1]

    Then it's a little transposed because the columns go along the rows

    That means that where we have J, we should then use k, and where we have K, we need to use -j.  So, we have $nJ + ? + K$ (since there are k elements in a row before we need to get to the value). Since we're using -J, we can substitute to get $nJ - \dfrac{J(J-1)}{2} + K$

5) Using indirect addressing feature of MIX, how can we load the location
   of an array with a single MIX instruction. 

   Assuming we're looking for J,K and J is in I1 and K is in I2

   So we know we have to multply J by N in order to get where we want. We can create a chain of memory that is N addresses of NEXTADDR,7:1. The last one can add ,7:2 to add the K.

   For a triangular matrix, you would need a chain for each possible J, because I don't see how you can decide in a single instruction to only go J-1/2 times through an address.

   Knuth points out that you can just index into a lookuptable based on j,
   and that is probably way easier (and faster, too!)

6) What is the formula for a Tetrahedral Arrays (lower and upper)?

    For upper triangle, I noticed the formula was triangle numbers, so I googled [Tetrahedral Numbers](https://en.wikipedia.org/wiki/Tetrahedral_number) and lo and behold it's a thing. 

    So, I think to figure this out , $LOC(A[I,J,K]) = LOC(A[0,0,0]) + \dfrac{I(I+1)(I+2)}{6} + \dfrac{J(J+1)}{2} + K$

    Now for the upper triangular array, I have no idea. It probably has some negative shenanigans and I have to transpose three dimensions to figure out how to rotate the tetrahedral array, and I couldn't figure that out. Looking at the answers didn't help, unfortunately :).

7)   Find a general formula for k-dimensional tetrahedral arrays.

   Well, it seems that binomial coefficients can easily scale up for higher order triangular numbers, so I suspect its the following:

   $LOC(A[I_1, I_2, ..., I_k]) = LOC(A[0, 0, ..., 0]) + \sum_{n=1}^k \binom{I_n + k-n}{k-n+1}$


8)   Is there a neat way to store six tetrahedral arrays in memory? 

   If you take a three dimensional memory of a cube that is $2n + 2$ on each side, you can store up to 8 tetrahedral arrays inside, because you in each square of $(n+1) \times (n+1) \times (n+1)$, and there are 8 such cubes in our bigger cube. I don't know if there's a better way than this, but this is the best I can come up with.


9)  Suppose you have a table as indicated for sparse matrix. How do youfind the address of all blue-eyed blond women of ages 21 to 23.  Make one pass through the lists.

    See `find_person.mixal`. Pretty much, you keep a pointer of each list and you find the minimum value. If all elements are that minimum, then you have one person. Otherwise, advance anything at the minimum one link. If any list pointer goes back around, then you know you're done.

    F1.  Set `PFEMALE <- FEMALE, PA21 <- A21, PA22 <- A22, PA23 <- A23, PBLUE <- BLUE, PBLONDE <- BLONDE`

    F2.  If `PFEMALE == NULL OR PBLUE == NULL OR PBLONDE == NULL or ALL(PA21==NULL AND P22==NULL AND P23==NULL)`, terminate the alrogrithm.  Set `MINROW = MIN(PFEMALE, MAX(PA21, PA22, PA23), PBLUE, PBLONDE)`

    F3. If `X==MINROW` for all `X=(PFEMALE, PBLONDE, PBLUE)` and `X == MINROW` for any `X=(PA21, PA22, PA23)`, print out all attributes. Advance all X to X(NEXT).

    F4. For each X!=MINROW for all `X=(PFEMALE, PBLONDE, PBLUE, PA21, PA22, PA23)`, set `X=X(nNEXT)`. If X < MINROW, repeat this step. Otherwise, Go to step F2.