1.  Why doesn't pushing a stack mention the possibility of OVERFLOW?

    What overflow means is determined by the program. There is no variable saying max size (it may grow until end of memory)
    You can't quite generalize, and you may not need it always if you know an idea of your max size of the stack. (also, it seems that
    getting new memory from AVAIL does the overflow check.)

2. See `insert.mixal`

3. See `delete.mixal`

4. How to handle OVERFLOW with SEQMIN? Not a complete program (so it probably has some errors), but see `overflow.mixal`

5. How do we handle an insert-front to make it output-restricted, and delete from rear. See `deque.mixal`. We could simplify things with a
   doubly-linked list, but that's a later chapter, so I'll just store an extra pointer to rear
