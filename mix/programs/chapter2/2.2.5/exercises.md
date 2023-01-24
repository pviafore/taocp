1)  Give specifications for insertion and deletion of left end of node P

    Insert: P <- AVAIL. INFO(P) <- Y. RLINK(P) = LEFT. LLINK(LEFT) = P. LLINK(P) = NULL. If LEFT was NULL, then RIGHT = P. LEFT = P

    DELETE: Y = INFO(P). LEFT = RLINK(P). LLINK(RLINK(P)) = NULL. If LEFT = NULL, then RIGHT = NULL

2) Explain why a single linked list cannot allow efficient operation as a general deque, you can only do one end.
   
   Given a node in the middle, you would always need to know the node before to get to it (potentially iterating through
   the list if needed) if you wanted to delete it. As Knuth points out in the answers, if you wanted to delete many nodes after one another, you could not do so from reverse order like a deque.

3) How would you prove CALLUP, CALLDOWN, and CALLCAR uses three separate variables.

    With three variables, there are 8 different possibilities of states. On an elevator going from floor 1 to 3, you could
    press both CALLUP and CALLDOWN on floor 2. When it arrives, you should only see callup dim (and viceversa for going down from floor 2 to 3 with CALLDOWN). This proves at least that the CALLUP and CALLDOWN are independent. Now we have to show independence for the CALLCAR. With an accomplice, we can stand at floor 2, and ask the accomplice to press floor 2 from floor zero. We should not see either of our lights light up (and we can press both and make sure the elevator callcar does not light up.)

4) What would happen if we deleted E9 from the algorithm

   So E9 can happen if there is a large amount of people going in and out of the elevator such that we make a decision 
   while people are egressing. If that decision gets made before any CALLCAR button is hit, then the elevator will mark that 
   it needs to go to floor 2 (stopping along the way for stops and potentially resetting things). However, if you delete it,
   you'll wait for a CALLCAR to indicate where to go.

5) If user 10 arrives on floor 2 at time 1048, show that elevator goes up after receiving people on floor 1, despite 8 wanting to go down.

    In this case, there will be user 10 looking to go up, and user 8 looking to go down. Since both are set, we look for CALLUP first, and that user goes in first pressing the button, setting the elevator to go up, not down.

    NOTE - I am wrong on this one. The real answer is that the elevator goes neutral, since there is no reason for it to keep going down. At this point, it will start moving back up, and not worry about 8, it focuses on user 7.

6) How would you change coroutines to use indicator lights.

   I would have two separate queues (this way, you aren't messing with removing from middle of queues) - one for going up and one for going down. In Step E4, when adding people in, check only the queue for the direction that you are about to be going in. (i.e. if you are about to start moving down, only pull from the queue for going down)

   In U3 and U5, you will need to insert into the queue for the direction the person wants to go in, and also make sure you remove from that queue as well.

7) What would happen if you changed JANZ U4A to JANZ Cycle? What error would show up?

   I think what would happen is if a person doesn't want to give up because people are getting off /on, they would just
   get removed in the cycle (because they are not added back )

   (This may be wrong - Knuth mentioned that it assumes that the user is in the wait list, but it might not be)

8) Implement step E8

    DEC4    1
    ENTA    61
    JMP     HOLDC
    LDA     CALL,4(3:5)
    JAP     1F      * check if any button is pressed
    ENT1    -2,4    * check if we're floor 2
    J1Z     2F
    LDA     CALL,4(1:1) * if up is pressed 
    JAZ     E8
2H  LDA     CALL-1,4 * check for lower floor calls
    LDA     CALL-2,4
    LDA     CALL-3,4
    LDA     CALL-4,4
    JANZ    E8
1H  ENTA    23
    JMP     E2A


9)  Implement The code for decision subroutine

DECISION     STJ   9F
             J5Z   9F
             LDX   ELEV1+2(NEXTINST)  * load next instruction
             CMP  =E1=
             JNE  1F
             LDX  CALL,4(1:5)
             JXZ  1F
* at E1 and no calls
             ENTA 20
             JMP HOLD
             ST6 TMP   * save of for restoring
             ENT6 ELEV1
             ENT3 E3
             ST3  2,6(NEXTINST)
             LD6 TMP
             JMP 9F
* check for any calls
1H           ENT3 0
2H           LDX  CALL,3
             JXNZ D4
             INC3 1
             JMP 2B
             ENT3 2
D4           ST3 TMP
             CMP4 TMP
             JG   3F   * floor is higher
             ENT5 1
             JMP 4F
3H           ENT5 -1
4H           LDX ELEV1+2,NEXTINST
             CMPX =E1=
             JNE 9F
             CMP3 =2=
             ENTA 20
             JMP HOLD
             ST6 TMP   * save of for restoring
             ENT6 ELEV1
             ENT3 E3
             ST3  2,6(NEXTINST)
             LD6 TMP
9H           JMP   *
TMP          CON  0


11) For a sparse update of calculations, we can keep a list of updates that need to be made.
    We can store the variable location, the calculation. Then we can just iterate through
    the list of things to update, iterate through all at once (maybe ordered by variable location)
    to compute new values, and then iterate again to apply the values. I don't know why we need
    two tables as suggested in the text, so I'm probably missing something. See `sparse_mem.mixal`

12) Why are double linked lists useful in the simulation?

    We need to delete elements from the queue if they give up, which means they may not be in the front of the queue
    Since there may be more than one of these, it doesn't work to keep a trailing value.

    Similarly, we're removing things from the wait list (and they may be in the middle as well) to shuffle it around.