1.  Why doesn't pushing a stack mention the possibility of OVERFLOW?

    What overflow means is determined by the program. There is no variable saying max size (it may grow until end of memory)
    You can't quite generalize, and you may not need it always if you know an idea of your max size of the stack. (also, it seems that
    getting new memory from AVAIL does the overflow check.)

2. See `insert.mixal`
