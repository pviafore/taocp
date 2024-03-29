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

8. See `invert.mixal`

9. Which of the following is a partial ordering on the specified set of S

   S = rational numbers, x ≺ y -> x > y     :    Yes, transitive, anti-symmetry, and reflexive
   S = all people, x ≺ y -> x > y           :    Yes, transitive, anti-symmetry, and reflexive 
   S = all integers, x ≺ y -> x is a multiple of y   :    Yes, transitive, anti-symmetry, and reflexive  ( this is wrong, because -1 and 1 are multiples of each other)
   S = all proofs in book, x ≺ y -> proof of y depends on truth of x   :    Yes, transitive, anti-symmetry, and reflexive
   S = all positive integers, x ≼ y -> x + y is even   :  No: Transtive, but has symmetry, and reflexive
   S = subroutines, x ≺ y -> x calls y  :    Yes, transitive (unless recursive), anti-symmetry, and reflexive (Knuth calls this ambiguous, but I'm taking it from a static call graph ). However, if we have function pointers, then it gets a bit more interesting

10. Can ⊂ be considered a partial ordering?

    It says that Transitivity and anti-symmetry applies, but does reflexiveness apply?

    Is set A a subset of set A? Yes, of course, but how about is there a case where if A is a subset of B, can B be a subset of A?

    Unless we have some Godel set, I would think you can't have this case, but I don't know if I can actually prove it.

11. All the ways of writing out a topological sort:

    1, 3, 7, 4, 9, 2, 5, 8, 6
    1, 3, 7, 9, 2, 5, 4, 8, 6
    1, 3, 7, 9, 2, 4, 5, 8, 6
    1, 3, 7, 9, 2, 5, 8, 5, 6
    1, 3, 7, 9, 5, 2, 8, 5, 6
    1, 3, 7, 9, 4, 2, 5, 8, 6
    1, 3, 7, 4, 9, 2, 5, 8, 6

    Oof, There's apparently 52, so I'm stopping now.

12. There are 2^n subsets of a set of n elements, and these subsets are partially ordered by the set inclusion relation. 
    Give two interesting ways to arrange these subsets in order

    Any subset of size n will precede a subset of size n+1, because a subset of size n+1 contains n elements by definition (And there exists some set of size n that is a subset of n + 1, just remove any one element).

    Therefore, one ordering is based on set size, and one could be in sorted order, where you sort each element of the set ascending,
    and list all sets in sorted order, where a smaller set is always first .

13. Oof, a HM48, I don't even know where to begin. How many ways to arrange a set of sets (2^n) so that they are topologically sorted?

    There's probably some combinatorics that can define this, but the scope is going to be staggering. 

14. Given a set with partial ordering, how can we know a topological sorting will have total ordering if x ≼ y or y ≼ x for any x,y.

    Given an element x in the set S, it has a relationship with every other element in the set. There will be some number of elements
    that are before it, and some that are after it. That means that we have a fixed position for x, since there are no elements that
    aren't forced to be before or after. This means we have a fixed position for every x, given each x. This means one ordering is possible.

15. Given a partial ordering, show that there is a unique set of relations that characterizes this ordering.
  
    Let's try this by contradiction. Let's suppose there are two separate partial orderings that share the same set of relations.
    We can graph these relations out. In order for the two orderings to be different, there must be one element x so that x ≼ y in one
    set, but not in another. However, since they share the same amount of relations, this cannot be true.  I have no idea how to approach this question for an infinite set.

16. If we create an incidence matrix where a(ij) is 1 if x(i) ≼ x(j), can we make all the entries below the diagonal zero?

    This feels like a topological sort. For a(ij) to be 1 (when i == j equals zero). This means for any i > j, we also need to be zero

    0 1 1
    0 0 1
    0 0 0

    This would be 0 preceds 1 and 2, and 1 precedes 2. With a set of 3 elements, there is a maximum of t(n-1) where t is the function
    for triangle numbers (or gaussian sum). This makes sense because for a set of n elements, the maximum number would be every 
    element depending on the next (n -1 relations), then the second element depending on every element that follows (n-2) all the way
    until the second to last element depending on the last element (1 relation). There can't be any more, because then there would be cycles. We can arrange all of these elements to be in the upper triangle of the matrix (of size n-1), with a 0 reperesenting any
    relation that is not present in the maximal set. Everything outside of the matrix can be zero. If anything weren't, we'd have a 
    cycle.

17. What does Algorithm T do if we have the relations as defined in the book?

    9,1,2,3,7,4,5,8,6

18. What is the significance of QLINK when the algorithm terminates?

    I think it's the topological sorted values as stored by a linked list.

19. What happens if we advance qlink during step T5 instead of T7
    
    I think it's fine, but I was wrong. Because step 6 adds things, we don't want to add to a null link queue,
    so an insert would just have to check if the queue is empty before linking a new node

20. What happens if we link all the nodes in a stack instead of a queue.
   
    We get a bit of a depth-first search situation, where we will order based on direct links first, and not
    necessarily the first nodes that we saw that were zero. For instance, we might take a node off the stack, and instead
    of grabbing a sibling node, we'll first descend into the children node to sort. It's still a topological sort (since
    we are only using numbers who have no successors.)

21. Would Algorithm T be any different if relations were repeated? What if there was a self reference?

    If it's repeated, I think we should be fine, because it just adds more successor nodes (They'd be duplicated).
    However, a self-reference won't work, because that's technically a cycle, and there would never be a case where
    the count would be zero (unless we pruned it when reading it in.), so we would never find the queue.

22. What are some ways to modify Program T to check for invalid cases

    Check that the n is not negative, or is greater than memory space.
    If there are too few relations, set n to be the number of actual relations (and print it out)
    If there are too many, stop at N, and error out with a message
    If any of the relations have a number <= 0, error out with message

23. What are some ways to print out a loop

    If we terminate the algorithm, and any of the count nodes are still 1 (do a linear scan), then we have a loop.
    Just pick one, save it off in a stack, and do depth first search until you find it again (using the stack). 
    Then popping off the stack would give you the loop. (if you find any node with no successors, pop the stack and try the 
    next one)

24. See `loopdetect.mixal`

25. Design an efficient algorithm for doing topological sort that has more nodes than memory can contain. Assume that 
    input, output and temporary working space are all done with tape

    So..., this makes it interesting (I actually had an undergrad problem that was like this but with mergesort).
    Essentially, we have to minimize the amount of times that we are seeking around tape, and have to do memory
    local operations as much as possible. This means that when we scan, we want to look at memory all at once.

    So, first, we can do a mergesort to make sure that all the inputs are sorted, which will at least allow us to keep
    linked lists of successors fairly close to each other in memory (means we can read a whole entire tape block, and 
    check that memory - think of it like reading as a cache line in a modern processor)). Each time we read from tape,
    we essentially are doing slow "page".

    So, let's look at how we can improve toposort. What we want to do is lay tape as follows:
    List of counts in linear memory (keep the queue operations in the same place) on one tape
    Each linked list of successors in a pre-defined size of memory (we know N, so we can make sure that we have room for N relations, which means we have fixed addressing, at the cost of wasted space on tape) on another tape. Links are now links to indices on tape, not into memory. If we wanted to do less wasted space, we can collapse the 
    free space (like we did with linear allocations in an earlier chapter) and keep a link along with the counts.

    We can split our memory in half at first, one for holding some values of counts, and one for the area of tape to be written for a linked list. As we read the sorted relations, we update the count (we know its in order, so we know when we need to write new counts to tape and get to the next block.). We then keep filling out the linked list
    of relations, knowing when we need to write to tape.

    (Of course, we are using buffered input throughout all of this).

    Then, when scanning from zeroes, we can scan our linear memory of counts alone and set that up.

    Then, we need to update all successors, which unfortunately may go to different places in tape, but we will 
    hope for locality in a lot of places.

    Is it the most efficient? Probably not, but it's something.

26. Come up with a relocating algorithm for libraries as described in the book. See `relocate.mixal`

27. See above

28. Given next positions, calculate whether it's a win, loss, or not. In the text, it suggests the following

    For each position we count the number of successors that have not been marked won, and a list of all predecessors.

    Let's do this for Nim: So, can track in a node the following : players turn, nonwinws, score, and link to predecessors.
    Each predecessor will point to the node address

    How to tell if a position is a win:
        If you are at zero, this means the last person took the last stone, and you lose
        Otherwise, for all of your successors, if they are all won, you lose. If any are marked lose, you win

        All winning means that you have zero nonwins listed for your current node

    So, first, set up a successor, predecessor relationship for everything
    For each S,P, you want to add P-link to the S-list. If your current position is a nonwin, increment your predecessors
    nonwin count by 1


    ...... A few days later ......

    After thinking about it more, I changed my code to do a simple dynamic programming to figure out win loss of a position
    linearly, so my example is not a great indication of what Knuth is going for, but was good practice anyways.

    I eventually scrapped it as it was too simple (in Nim, if you can get the opponents score to a multiple of 4, you can force a win, so it really wasn't a great example of linked strategies. In  a real program, we'd go with a min-max tree with alpha-beta pruning.)

    See `nim.mixal`

29. Do a mass erase to memory of a list given a) just FIRST, and b) just FIRST/SECOND. See list_erase.mixal

30. If queues are reprsented with a front and end, but an empty queue is front = 0, r is undefined, what does insert and delete look like.

Insertion: If Front is 0, Front = Rear = Node
Deletion: If Front == Rear, Front = 0