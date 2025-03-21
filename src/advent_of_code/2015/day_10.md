# Day 10 Elves Look, Elves Say

[link to problem](https://adventofcode.com/2015/day/10)

## The Problem

Today, the Elves are playing a game called look-and-say. They take turns making sequences by reading aloud the previous sequence and using that reading as the next sequence. For example, 211 is read as "one two, two ones", which becomes 1221 (1 2, 2 1s).

Look-and-say sequences are generated iteratively, using the previous value as input for the next step. For each step, take the previous value, and replace each run of digits (like 111) with the number of digits (3) followed by the digit itself (1).

For example:

> 1 becomes 11 (1 copy of digit 1).

> 11 becomes 21 (2 copies of digit 1).

> 21 becomes 1211 (one 2 followed by one 1).

> 1211 becomes 111221 (one 1, one 2, and two 1s).

> 111221 becomes 312211 (three 1s, two 2s, and one 1).

Starting with the digits in your puzzle input, apply this process 40 times. What is the length of the result?

Your puzzle input is 1113122113.

### Thoughts

This seems like a string builder program. I read in the input string, I count how many times I see the same char, then I write it out literally.

Take 111221 for example:
- 111 has a count of 3, so it becomes 31
- 22 has a count of 2, so it becomes 22
- 1 has a count of 1, so it becomes 11

Putting that all together gives us 312211, which is the answer.

### Solution
For context, problem part 1 and part 2 were the same. The only difference between them was how many you run the process.

I wrote the solution out a couple of days ago, but I remember it pretty quick. I iterated over the string and looked at the previous character and the current character. If those two characters are equal, I up the count by 1. If not, I append the count and previous character to my result string, set the current count equal to 1 and start the process over again. Once the iteration is finished, you have to remember to add the last count and character to the result string.

The above code got put into a function so that I can take its output and use it as input. This also made it easy to call it 40 or 50 times too. 

```rust
{{ #include ../../../advent_of_code/2015/day_10/src/main.rs }}
```
