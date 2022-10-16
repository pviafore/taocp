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
