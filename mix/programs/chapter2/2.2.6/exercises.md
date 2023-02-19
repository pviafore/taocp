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


10) Can you think of a better way to organize a table for searches

    Hashing the fields you're looking for and using a hash table lookup

11) If we have a 200x200 matrix with at most 4 entries per row. How much storage for this (three words per node, one per head)

    $200 \times 3 \times 4 + 200 + 200 = 2800 $ words

12) What is `VAL(Q0)`, `VAL(P0)` and `VAL(P1)` at the beginning of step S7, in terms of a,b,c,d.

    $VAL(Q0) = c $

    $VAL(P0) = \dfrac{b}{a}$ 

    $VAL(P1) = d$

13) Why were circular lists used instead of straight circular lists?

    There's no condition for end of list and resetting/inserting/deleting, instead, when the algorithm hits the end, it is ready to go for the next iteration

    You could check full null every time you advance a list and then reset back to the beginning/end (depending on direction), but that's extra instructions.

14) The pivot algorithm saves time in a sparse matrix, since it avoids zero elements. Show that this savings in running time can be achieved in a large sparse matrix that is stored sequentially, with an auxillary table LINK[J]

    If you have a large matrix in memory, you could save a separate table in one dimension, and make sure that it has a list of nodes per row. So for each node $0 \le i \le j$, $LINK[i]$, that could contain a linked list where the value is the node that is in that row, making it easy to figure out which elements are in the row and skipping zeroes.

    I do not know how to do it with a singular table with no linked lists

15) Write the pivot algorithm

    See [`pivot.mixal`](pivot.mixal). I do not have floating point operations yet, so I am going to do all integer division. Unfortunately, this means the pivot point will be zero when printed out, so sorry.

16) Write an algorithm that copies a sparse matrix

    Assuming you are given $M,N$ for a $M \times N$ matrix, as well as the locations of `BASEROW[1]` and `BASECOL[1]`.
    Also assuming there is a auxilary table `PTR[J]`, where $1 \le J \le M$.

    C1. (Initialize) Set $I=1$

    C2. (Start New Column) If $I==N$, terminate. $NEWBASECOL[I] \leftarrow AVAIL, OLDPTR \leftarrow BASECOL[I], LASTUP \leftarrow NEWBASECOL[I]$. This sets the new base column and sets up OLDPTR to iterate through the column

    C3. (Copy Column) $OLDPTR \leftarrow OLDPTR(UP), J \leftarrow OLDPTR(ROW) $. 
    
    If $PTR[J] == null$, then $PTR[J] \leftarrow AVAIL$. 

    If $OLDPTR(ROW) == -1$, then $ LEFT(PTR[J]) \leftarrow BASEROW
    [J], NEWNODE(UP) \leftarrow NEWBASECOL[I], I \leftarrow I+1$ and go to step C2.
    
    
    $NEWNODE \leftarrow AVAIL, LASTUP(UP) \leftarrow NEWNODE, LASTUP \leftarrow NEWNODE. NEWNODE(LEFT) \leftarrow PTR[J], PTR[J] \leftarrow NEWNODE, NEWNODE(VAL) \leftarrow OLDPTR(VAL)$

    This advances the old pointer to the value to copy, and copies it over to a new node. The linkage for the node is set up for left and up.

    Repeat Step C3.

    
    See [copy.mixal](copy.mixal)

17) Write an algorithm for multiplying a matrix

    Assuming we know M and N for a $M \times N$ matrix. 

    M1.(Initialize) Set $I=1$ for the column. 

    M2.(Iterate Column) If $ I \gt N$, terminate. Set $J \leftarrow M,  NEWBASECOL[I] \leftarrow AVAIL, LASTNEW \leftarrow NEWBASECOL[I], LASTNEW(ROW) \leftarrow -1$

    M3. (Iterate Row) If J == 0, $LASTNEW(UP) \leftarrow NEWBASECOL[I],I \leftarrow I+1,$ and go to M2. Set $K \leftarrow N, SUM \leftarrow 0, M1BC \leftarrow MATRIX1\_BASECOL[I](UP), M2BR \leftarrow MATRIX2\_BASEROW[J](LEFT),$

    M4. (Multiply) If  $K == 0$ and go to M5


    Otherwise, if $K == M1BC(ROW)$, then $VAL1 \leftarrow M1BC(VAL)$ and $M1BC \leftarrow M1BC(LEFT)$, else $VAL1 \leftarrow 0$. If and $K == M2BR(COL)$, then $VAL2 \leftarrow M2BR(VAL)$ and $M2BR \leftarrow M2BR(UP)$, else $VAL2 \leftarrow 0$.  

    $SUM \leftarrow SUM + VAL1 \times VAL2, K \leftarrow K-1, $

    Repeat Step M4.

    M5. (Set Value)  If $SUM == 0$, Go to M3. If $PTR[J] == NULL$, $PTR[J] \leftarrow AVAIL, NEWBASEROW[J] \leftarrow PTR[J], PTR[J](COL) \leftarrow -1$  

    $NEWNODE \leftarrow AVAIL, LASTNEW(UP) \leftarrow NEWNODE, NEWNODE(LEFT) \leftarrow PTR[J], PTR[J] \leftarrow NEWNODE, NEWNODE(ROW) \leftarrow J, NEWNODE(COL) \leftarrow ROW, NEWNODE(VAL) \leftarrow SUM, LASTNEW \leftarrow NEWNODE, NEWBASEROW[J](LEFT) \leftarrow NEWNODE$

    $J \leftarrow J-1$, go to step M3


    See [multiply.mixal](multiply.mixal)


18) Given the matrix, 

	$
	\begin{bmatrix} 
	1 & 2 & 3 \\
	0 & 1 & 2\\
	0 & 0 & 1 \\
	\end{bmatrix}
	$

    Invert it.

    For the first row, the pivot element is 3, so the first pivot is : 

	$
	\begin{bmatrix} 
	1/3 & 2/3 & 1/3 \\
	-2/3 & -1/3 & -2/3\\
	-1/3 & -2/3 & -1/3 \\
	\end{bmatrix}
	$

    In the second row, the pivot is in column 1, so

	$
	\begin{bmatrix} 
	1/2 & 1/2 & 0 \\
	-3/2 & 1/2 & 1\\
	-1/2 & -1/2 & 0 \\
	\end{bmatrix}
	$


    Our final pivot point is the -1/2

	$
	\begin{bmatrix} 
	0 & 1 & 0 \\
	-2 & 1 & 1\\
	1 & -2 & 0 \\
	\end{bmatrix}
	$

    The permutation goes like this, 

    Row 1 <- 3, 2 <- 1, 3<- 2
	$
	\begin{bmatrix} 
	-2 & 1 & 1\\
	1 & -2 & 0 \\
	0 & 1 & 0 \\
	\end{bmatrix}
	$


    Column 1<-3, 2 <- 1, 3 <-2
	$
	\begin{bmatrix} 
	1 & -2 & 1 \\
	0 & 1 & -2  \\
	0 & 0 & 1 &\\
	\end{bmatrix}
	$


19) How would you do the invert with sparse matrices?

    You would need a table CR to populate C[k]'s BASEROW poitner and table CC to poulate C[K]'s BASECOL pointer with. 
    Then for each $1 \le k \le N$ row , you would need to iterate through the row and find the greatest value (and checking that the column is not already in C[k]).

    Once you have the element, you perform a pivot (Sorry, I don't have code for this - I don't have the floating point operations implemented yet in MIX).

    Then comes the row/column swap. Let's say you're doing the rows first. 

    You would set the BASEROW values to be what the CR[k] table was, which would permute the rows just fine. You would need to re-link all the column values, which you can do by iterating BASEROW in parallel and seeing which values match up for which column.

    Then you could do the same thing with BASECOL and CC[K], which would let you permute the column heads (which have been updated from row permutes from before), and then do the relinking of the rows in a similar manner.

20) Come up with the indexing/storage scheme for tridiagonal matrices

So there are three elements for each row, except for the first and last.

So, when we get to row J and column I, we know that the position of that row is going to start with $3(J-1)$. Then for each row, we know we have skipped $J-1$ elements so far, so the storage function should be

$LOC[A[0,0]] + 3(J-1) + (I - J + 1) $


21) Suggest a storage allocation for $N \times N$ matrix, where N is variable

    I'm assuming this means that that N will change during runtime, but storage alloation is the same:

    so element 1 has 1x1, 2,3,4 will be 1,2, 2,1 and 2,2, 3 will be 1,3, 2,3, 3,1, 3,2 3,3

    So it looks like we grow by 1,3,5,7,11 (the growth of perfect squares)

    So, we need some function that expresses these numbers based on index

    So, for each element of $x=max(I,J)$, we know that we have done $(x-1)^2$ elements so far to get to the right "growth". 

    But how do we know which element to get to. We assume the first element is $1,x$, so we can start with $J-1$ as the index, and we need to add $I-(x)$ to get further (for the column you add, this second term should always be zero).

    So, that gives us 

    $LOC(A[0,0]) + (x-1)^2 + y$
    
    where $x = max(I,J)$
    and $y = J$ if J \le I, otherwise $y = I + x - 1$ 


22) Find a polynomial that assumes each nonnegative integer value exactly once as the indicies run through all k dimensional nonnegative integer vectors, with the additional property that if the sum of indices is less than another sum of indices, than the polynomial is also less than.

    I have no idea what this is asking for, so skipping this one

23) What is the storage function for an extensible matrix?

     so in memory, it would look like this ( with pipes separating when it grows)

     1,1   | 1,2  | 2,1  2,2 |  1,3, 2,3 |  3,1, 3,2, 3,3

     So it grows to the right by M whenever J < I, and when I > J, it looks like we grow beneath. 

     If $J \le I$, then $LOC(A[0,0]) + (I-2)(I-1)  + J - 1$
     Otherwise $LOC(A[0,0]) + J(J - 1) + I - 1 $


24) HOw is it possible to read/write any element of a sparse array (that is still contiguous in memory), when most are zero, and you don't want to set everything to zero.

You could have a separate array (maybe even just a word per element) that stores whether it is set or not. That can be set to zero much quicker. If there were bit manipulation functions in your assembly language, you could even do it as part of a bitfield, but alas, MIX does not have that.

Alternatively, you could have a list per row/column or something of that regard that's sorted of storage, for quicker lookup (since you can subdivide by row).

