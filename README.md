# A Simulator for Conway's Game of Life
### Written in Rust
### Written for the terminal (& GUI kinda)

A terminal program for playing with different boards of Conway's Game of Life. 
This project was intended as a way to learn Rust and perform some basic operations. 
The code should be complete, but I haven't really tested any large or functional boards (I probably won't).

#### Overall To-dos:
1) Properly handle text inputs for non-GameAction keyboard events (rn they run ~ twice)
2) Add a more expandable way to add additional keyboard actions into the loop
3) Refactor & improve text-based experience
   1) More reliable functions in text::_ (Results, Options, etc.)
   2) Condensed commands, ideally have mode & arg in 1 read
   3) Transition GUI to use new, nice text::_ functions (once available)
4) Redo the test suite, make them actually useful / with asserts
5) Have the window update independently of the events, so the game will run when played
6) Why does it take so long to close ?


----
### Refactorings:
1) Revamp the legacy text control mode into a better version
