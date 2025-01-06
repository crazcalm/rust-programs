# Day 3: Perfectly Spherical Houses in a Vacuum

[link to problem](https://adventofcode.com/2015/day/3)

## The Problem

Santa is delivering presents to an infinite two-dimensional grid of houses.

He begins by delivering a present to the house at his starting location, and then an elf at the North Pole calls him via radio and tells him where to move next. Moves are always exactly one house to the north (^), south (v), east (>), or west (<). After each move, he delivers another present to the house at his new location.

However, the elf back at the north pole has had a little too much eggnog, and so his directions are a little off, and Santa ends up visiting some houses more than once. How many houses receive at least one present?

For example:

>   _> delivers presents to 2 houses: one at the starting location, and one to the east.
	
>^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.

> ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.

### Thoughts

If you treat the directions as 2d grid coordinates and then put the seen coordinates into a set, you are pretty much done. The only part I was confused about was whether or not I would make the borrow checker angry by passing my varaibles (that I still plan on using) to the HashSet. Upon reviewing the Rust Book, I performed a "[Stack-Only Data Copy](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy)", which means that, for integers that exist entirely on the stack, a copy is made when you assign them to another variable leaving both variable valid.


```rust
{{ #include ../../../advent_of_code/2015/day_3/src/main.rs }}
```

## Problem 2

The next year, to speed up the process, Santa creates a robot version of himself, Robo-Santa, to deliver presents with him.

Santa and Robo-Santa start at the same location (delivering two presents to the same starting house), then take turns moving based on instructions from the elf, who is eggnoggedly reading from the same script as the previous year.

This year, how many houses receive at least one present?

For example:

>    ^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.

>   ^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.

>    ^v^v^v^v^v now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.

### Thoughts

The problem seems more or less the same, with the difference being that even instructions go to Santa and the odd instructions go to the Robot.

For a moment there, I was going to try to think of clever ways to split up the program, but the simple solution of maintaining two set of grid coordinates seems fine.


```rust
{{ #include ../../../advent_of_code/2015/day_3_part_2/src/main.rs }}
```

During my first pass at it, I put all the code in main. Though the answer was right, the repetition of code within the if blocks was hurting my eyes. So I pulled that code out into its own function. At the end of the day it is the same code, but the function of all reference does give the impression that I know what I am doing. hahaha
