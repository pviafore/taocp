1) What is the Value of SUIT(NEXT(TOP)) and NEXT(NEXT(NEXT(TOP)))

Spades, which is 4
Null link

2)Contents(LOC(V)) = V. When does LOC(CONTENTS(V)) == V


This means that V itself must be some link, becasue LOC(X) equals V. In
this case CONTENTS(V) would be the contents of a link address, which is
not the address itself, but rather the value pointed to it.

3) Write an algorithm to draw a card. see `draw.mixal`

4) Write an algorithm to place a card on bottom face down. See `put_on_bottom.mixal`

5) Give an algorithm that draws from the bottom. See `cheater.mixal`

6) SUIT(LOC(CARD)), and SUIT(TOP) are both appropriate. When asking for a field in pseudocode, you expect a lin variable.
   SUIT(CONTENTS(CARD)) would try to get the CONTENTS OF of CARD, which would be the card itself as a memory address and not a suit.
   Similary, SUIT(CARD) would try to get the suit from the CARD's value itself, which is not not a memory address

7) If TOP is a link variable, then it's a pointer to memory. LDA TOP(NEXT) will not work, because TOP(NEXT) will be the address of the card,
   which means that when we LOAD from it, we end up getting the first node of the card stored in A.

   Conversely, if we LD1 TOP, we have an index variable that points to the next card, and then if we LDA 0,1(NEXT), we are looking at the
   card's next value and loading from that instead. Thus, b is correct. Check the exercises to see this in action.

8)  Look at `count.mixal` for an example

9)  Look at `cheater.mixal` and `put_on_bottom.mixal` which already demonstrates printing with parens.
