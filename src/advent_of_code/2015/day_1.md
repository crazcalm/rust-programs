# Day 1: Not Quite Lisp

[Link to problem](https://adventofcode.com/2015/day/1)

## The Problem

Santa is trying to deliver presents in a large apartment building, but he can't find the right floor - the directions he got are a little confusing. He starts on the ground floor (floor 0) and then follows the instructions one character at a time.

An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means he should go down one floor.

The apartment building is very tall, and the basement is very deep; he will never find the top or bottom floors.

For example:

    (()) and ()() both result in floor 0.
    ((( and (()(()( both result in floor 3.
    ))((((( also results in floor 3.
    ()) and ))( both result in floor -1 (the first basement level).
    ))) and )())()) both result in floor -3.

To what floor do the instructions take Santa?

### Thoughts

This is a warm up problem, so there is not much to think about, but I did appreciate the opportunity to think about the process I want to use to solve these problems. I just finished reading The C Programming Language book and the small commandline programs they built throughout that book was eye opening. So, for my solutions, I will pass the input to my programs via stdin.

```rust
{{ #include ../../../advent_of_code/2015/day_1/src/main.rs }}
```

## The Problem Part 2

Now, given the same instructions, find the position of the first character that causes him to enter the basement (floor -1). The first character in the instructions has position 1, the second character has position 2, and so on.

For example:

    ) causes him to enter the basement at character position 1.
    ()()) causes him to enter the basement at character position 5.

What is the position of the character that causes Santa to first enter the basement?

### Thoughts

This took me by surprise! I had already started working on day 2. I actually saw that day 2 has a part 2 before I realized that day one had a part 2.

At first glance, this problem looks like it is just about counting the characters I see and reporting when the floor hits -1. In the spirit of unix and in the aim of learning to let go of code, I will create a new project for this.

```rust
{{ #include ../../../advent_of_code/2015/day_1_part_2/src/main.rs }}
```
