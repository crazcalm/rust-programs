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

## Thoughts

This is a warm up problem, so there is not much to think about, but I did appreciate the opportunity to think about the process I want to use to solve these problems. I just finished reading The C Programming Language book and the small commandline programs they built throughout that book was eye opening. So, for my solutions, I will pass the input to my programs via stdin.

```rust
{{ #include ../../../advent_of_code/2015/day_1/src/main.rs }}
```
