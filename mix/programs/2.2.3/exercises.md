1.  Why doesn't pushing a stack mention the possibility of OVERFLOW?

    What overflow means is determined by the program. There is no variable saying max size (it may grow until end of memory)
    You can't quite generalize, and you may not need it always if you know an idea of your max size of the stack. (also, it seems that
    getting new memory from AVAIL does the overflow check.)

2. See `insert.mixal`

3. See `delete.mixal`

4. How to handle OVERFLOW with SEQMIN? Not a complete program (so it probably has some errors), but see `overflow.mixal`

5. How do we handle an insert-front to make it output-restricted, and delete from rear. See `deque.mixal`. We could simplify things with a doubly-linked list, but that's a later chapter, so I'll just store an extra pointer to rear

6. If you are inserting at the rear, you are eventually removing from the front. When setting from the rear, is there a way to 
   avoid setting the link the last node to null if you change how you look for underflow when inserting?

   If you check if you are the same node as the last node as R. If you are, you know you can set F to null link (empty list),
   and the R pointer to something that points to the null link. That way, you rely on checking both pointers rather than 
   knowing there is a null link (I like the null link way more)

7. How do you invert the links of a linked list?

   At first, I thought about treating it like two stacks, but I'm wondering if we can do it actually in one data structure.
   If I have a -> b -> c -> d, I can do the following: 
   
   Starting conditions: z = link(a), x = a
   For node x: y=x, x = z, link(x) = y, z = link(z) until z == 0

   Then at the end, start of the list is equal to x

8. See `insert.mixal`