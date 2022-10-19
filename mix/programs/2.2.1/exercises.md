1)  An input-restricted deque is a linear list in which items may be inserted at one end but removed from either end.
    It can be a stack or a queue.

    If you are inserting into one end, then you can choose which end you want to make it a stack or queue:
    If you want a stack, remove from the same end you inserted from, otherwise, its a queue.

    Could you do the same for an output-restricted deque -> Yes. Assume you can only remove from one end. If you want
    a queue, you insert from the other end, and if you want a stack, insert from the same end.

2)  Can 123456 be permuted into 325641?

    You would have to push 123, pop 3, pop 2, push 4, push 5, pop 5, push 6, pop 6, pop 4, pop 1

    How about 154623

    If you push 1, pop 1, you have to worry about pushing 2345 to get the next number. Then pop 5, pop 4, push 6, pop 6
    but then you run into trouble. There is no way to do pop of 2 at this point, so its impossible.

3)  Rule to determine if a chain of S(push) and X(pop) is admissible.

    First off, there has to be N S's and X S's (as said in the book). We want to avoid any un-performable actions.
    We can always perform a S, so let's examine X a little bit more.

    When we encounter an X at position i, the number of S's seen so far has to be greater than the number of X's
    (otherwise, we're trying to pop an empty stack if they are equal). So if f(n) = Σ Sn where n=0 to i > Σ Xn where n = 0 to i, where
    Sn is 1 if the symbol at position n is an S, zero otherwise, and Xn is 1 if the symbol at position n is an X zero otherwise.

    Then admissibility is defined as f(n) is true for all n from 0 to N, where N is the size of input

    Now, why would two separate sequences never give the same output?

    At some point, the sequences must differ in at least one place. Assume you have an empty stack or non-empty stack at that point.
    If its empty, one of those sequences is pushing onto the empty stack, and one is popping, which means its not admissible.

    If its non-empty, then you have some top of the stack Z. If you are popping, then the next item in the sequence is Z. But if you are
    pushing, then what happens is that you have pushed a new value A. You may never get Z next, as you are forced to pop A before getting Z,
    meaning you can't get the same output

4)  Find a simple formula for an, the number of permutations of n elements that can be obtained with a stack.

    GENERAL NOTE: The rest of this is wrong, but I kept it here to explain my thought process. Knuth gives a better explanation,
    but I feel its important to preserve my mistakes and thinking process for posterity.

    Hooo boy, how simple is this going to be? If I think about the numbers 1,2, then I can create 1,2, or 2,1 just fine.
    But if I get 1,2,3  can create everything but 3,1,2. The piece here that's interesting is that for some numbers in a,b,c such that
    a < b < c, I can't have a permutation where I have ab in it if c is pushed onto the stack (such as cab, or 312). That's because I can only
    pop a smaller number a before a bigger number if I have not put any number bigger than a on the stack so far. With 312, I can't pop the 1,
    since I've already pushed a 2 on the stack first.

    So which permutations have that value?

    Let's assume I have a permutation P, and each Pn is generated from 1..n. I know I can get any number as the first number, by popping immediately after I push it. so I have N options ( I smell a factorial coming). The possible next numbers cannot be lower than this number, unless its the highest value not popped yet.

    So for a permutation P, how do I know the next number is the max?

    Can I do this recursively?

    a(0) = 0
    a(1) = 1
    a(2) = 2
    a(3) = 5

    I have a relationship for three values that can't be violated. If I assume that for any value of 3 numbers yields 5 permutations),
    then for four values, I should have the following

    1 - all 5 are possible, as after I push and pop 1, I can do any of the original 5 three answers. a(3)

    2 - after I pop 2, 1 has to be on the stack. I have any of the 5 options left, because starting with a number lower is n't too bad

    3 - I bet there's only 3. It can't pop 1 next, and if it pops 4, since there is a 2,1 on the stack, I can't do anything with 1,2

    4 - only one is possible : 4,3,2,1. I can't do anything with 1,2 | 1,3 | 2,3

    That means that a(4) = 14

    What is 5?

    1 - all 14 are possible

    2 - all 14 are possible

    3- 9 are possible

    4- I bet 4 are possible, let's see.  Yup

    5: One is possible

    For a total of 42

    So it seems like for some number N, 1 and 2 is a(n-1), and for N is 1, so we have at least 2 (a(n-1) + 1). Then for the remaining
    N - 3 numbers, it looks like it subtracts by a(n-2). Now this is just a pattern, and has not been proved yet

    So I have the sequence N=1..n where n >= 3.  I know for pushing 1 and 2 that there will be a(n-1) options and for n there will be 1 option.
    The only valid answer for n is 1, or the reverse order, becasue if you've pushed n onto the stack without doing any popping first,
    then you may only pop the stack in reverse order.

    For N=1, you push a 1 first, and immediately pop it, which leaves you with a(n-1) numbers left.

    For N=2, you push a 1 and a 2, pop the 2, and you have a stack with 1 on it, This is the same as if you pushed a 1 first.

    For pushing 3, you have all the same possibilities as you did with N = 2, and you can substitute the 3 for a 2 in all of those cases
    and not worry, because if a < 2 < b where a and b != 3, then a < 3 < b as well. But, any number more than 2 less (So in this case 1),
    can't be popped next, (remember, only the highest number not popped yet can be popped). So for each number more than 2 less (in this case 1), you have to eliminate a(n-2) entries (this ignores any already invalid values).

    So we have (a(n-1)) + 1 + (a(n-1) - 0(a(n-2))) + (a(n-1) - 1(a(n-2))) + (a(n-1) - 2(a(n-2))) + ... + ((a(n-1)) - (N-3)(a(n-2))).
    Doing some gauss addition tricks, we can see that we have an arithmetic series starting at a(n-1) - (N-3)(a(n-2)) and ending at a(n-1),
    which is incrementing a(n-2) for a total of N-3 times.

    A pair will equal 2*a(n-1) - (N-3)(a(n-2)) and there will be (N-2) / 2 pairs of those so that equals an=(n-2)(2a(n-1) - n-3(a(n-2)))/2 total. Add the 1 and original a(n-1) to it as well.

    So for a(4) we have 2(2*5 - 1*2)/2 + 5 + 1  = 14. a(5) we have 3(2*14 - 2*5)/2 + 14 + 1  = 42. Thus I predict a(6) is
    4(2*42 - 3*14)/2 + 42 + 1 = 127

    So a(n) = (n-2)(2(a(n-1)) - (n-3)(a(n-2)))/2 + a(n-1) + 1

    I'm sure there's a better way of writing that or making it a factorial, but I think this is right. It'd probably be better if I could
    do an inductive style proof too.




    Turns out I'm wrong, so ignore the above. Knuth gives it as (2n choose n) - (2n choose n-1), so I was unfortunately off :(.  I bet it has to do with my recursive
    pattern no longer holding correctly, and overcounting the number of inadmissiable numbers.

    Here's a Python program brute forcing the answer (which I should have checked earlier):
```

stacks = [("", [1], list(range(2,6)))]
while any(v for t,s,v in stacks):
    new_stacks = []
    # simulate a push
    text, stack, values = stacks.pop(0)
    if values:
        new_stacks.append((text, stack + values[:1], values[1:]))
    # simulate a pop
    if stack:
        new_stacks.append((text+str(stack[-1]), stack[:-1], values))
    if not values and not stack:
        new_stacks.append((text,stack,values))
    print(new_stacks)
    stacks += new_stacks

print(len(stacks))
```

5)  However, above was not all wrong, because I proved that there are no indices i < j < k such that Pj < Pk < Pi

6) If you have a queue instead of a stack, you can only get the values in the order you received them.

7)  What permutation of 1234 cannot be obtianed with an input restricted deque, but not an output

    So we can input on one side, but can remove on any side. With an output queue, we can output on one side, but enter on any side

    Since both can be used as a stack, let's look at the values that can't be represented as a stack and see how to represent them
    in a queue

    1423   -> IR / OR
    2413   -> IR / OR
    4123   -> IR / OR
    4132   -> IR only
    4312   -> IR / OR
    4213   -> OR only
    4231   -> Neither
    3124   -> IR / OR
    3142   -> IR / OR
    3412   -> IR / OR

    So 4213 is Output-Restricted only, 3412 is Input restricted only and 4231 can't for either

8)  A general deque can be done as a stack, input-restricted or output-restricted, so it can take all those
    permutations that thsoe types can, so let's look at 4231, since that was a neither. L for left push, R
    for right push, and X for left pop and Z for right pop: 1234 = LLRRZXZZ

9)  Say Bn is the number of permutations that can be produced with an input-restricted queue.
    Show that Bn is also the number of permutations that can be produced with an output-restricted queue.

    In this case, I think there needs to be a bijection between permutations to show they are one-to-one
    Valid permutations map to valid permutations. Input-restricted only map to output-restricted only, and
    neither maps to neither. This means there needs to be some transformation function.

    Since both will share IR/OR and neither permutations, then we really just have to map an IR-only to an OR-only

    Say I have an IR-only permutation, such as 4132. This works for IR, because for indices a,b,c,d I have
    Pd < Pa < Pc < Pb.  This allows me to pop the D first, but choose either end to pop for. An OR can't take this,
    because it requires you to take either the smallest or largest of the values during each pop, since if you pushed d
    (in order to pop it first), you would have had to push a,b,c first. Therefore, if you're going to pop d first, you need
    Pd < Pb < Pa < Pc. So you can rotate the three b/a/c to left in that equality to get IR to OR.

    I have no idea if this is right, but it seems that Knuth mentions reversing operations. He says there is a bijection for sure
    but I'm not sure if my logic works.

10) S is for insert left, Q for insert right, and X for pop left. (for OR-deque)

    QQXSXSXX == SXQSXSXX (for 1234)

    I want to find one admissible sequence. For one, We should define that the element must always be an S,
    when inserting into an empty queue, because whether you insert left or insert right, there is one element.

    Then, the following is equivalent SQX and SXQ. Any Q that happens between a S and its corresponding X should be done after the X
    THis should remove all ambiguities.

    Things I forgot from Knuth: The number of X's should equal the number of S's and Q's

11) I can't do generating functions, and I don't understand them. Skip

12) Oof, no

13) A M48 problem. THat will be tricky. I don't want to spend a research term figuring this out, so I might just cite some interesting
    resources I found along the way.

    I don't have access to the original Journal of Algorithms, but I did find a paper in DMTCS https://hal.archives-ouvertes.fr/hal-00962390/document. Cool beans, the number of permutations of a stack matches Catalan numbers. You can find more about
    the original algorithm and data at this paper: https://arxiv.org/pdf/1208.1532.pdf. That paper also provides an algorithm for counting
    the number of permutations. I cannot say how accurate it is, but I also found https://www.sciencedirect.com/science/article/pii/S0195669816300610.

14) If you only have two stacks, can you implement a queue.

    If you push data into a stack, its LIFO. However, if you pop that stack into another stack, that stack will be popped in
    the original order, thus making the two stacks act as a FIFO

    Consider two stacks, input-stack and output-stack. The push/pop operations would be as follows.

    Push: push into input-stack

    Pop: check if output-stack is empty. If not, pop from it. Else, pop all of input-stack into output-stack.
