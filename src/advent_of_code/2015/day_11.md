# Day 11: Corporate Policy

[link to problem](https://adventofcode.com/2015/day/11)

## The Problem

Santa's previous password expired, and he needs help choosing a new one.

To help him remember his new password after the old one expires, Santa has devised a method of coming up with a password based on the previous one. Corporate policy dictates that passwords must be exactly eight lowercase letters (for security reasons), so he finds his new password by incrementing his old password string repeatedly until it is valid.

Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and so on. Increase the rightmost letter one step; if it was z, it wraps around to a, and repeat with the next letter to the left until one doesn't wrap around.

Unfortunately for Santa, a new Security-Elf recently started, and he has imposed some additional password requirements:

> Passwords must include one increasing straight of at least three letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.

> Passwords may not contain the letters i, o, or l, as these letters can be mistaken for other characters and are therefore confusing.

> Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.

For example:

> hijklmmn meets the first requirement (because it contains the straight hij) but fails the second requirement requirement (because it contains i and l).

> abbceffg meets the third requirement (because it repeats bb and ff) but fails the first requirement.

> abbcegjk fails the third requirement, because it only has one double letter (bb).

> The next password after abcdefgh is abcdffaa.

> The next password after ghijklmn is ghjaabcc, because you eventually skip all the passwords that start with ghi..., since i is not allowed.

Given Santa's current password (your puzzle input), what should his next password be?

Your puzzle input is `hxbxwxba`.

### Thoughts

The password being a fixed size great help. 8 characters is small enough that I can create a function for each one of these checks and not care how long it will take to process.

For no reason in particular, I am thinking about fancy stuff like a validate function that takes a list of functions and a string as input. The string will then get tested against all the functions in the list and if any of the function return false, it will return false.

The reason I can do this is because all the validation checks have the same function signature, which is that they take in a string reference and return a boolean.

The harder part of this question might be the incrementing of strings. I'm silly. I have been sitting here struggling to think of ways to interate over the charaters while skipping i, o, and l, like that is actually a requirement. I will use the numbers 0 - 25 and i will convert them back and forth between characters as needed. I'm thinking a struct that holds an internal list of numbers. It has an increment method that cycles up and another method that returns the string representation of the array so that I can pass it to the validate function.

I could take time to think about the other parts of the problem, but I'm pretty tired. So lets write this up and go to bed.

### Solution

This took a lot longer to write than I thought it would and, with regard to speed of the code, this is, by far, the slowest thing I have written so far.

```rust
{{ #include ../../../advent_of_code/2015/day_11/src/main.rs }}
```

I did everything I said I was going to do. I created a struct called Password that has a increment method and a to_string method. Internally, it manipulates and Vec of numbbers, which makes the implementation for the increment method manageable.

I have a validate function that takes a list of functions with the signature of `fn (&str) -> bool` and it also has a parameter called input which is a String. The validate function iterates over the functions in the list, it passes a reference to `input` to those functions and makes sure they all return true.

I then wrote out each of the requirements as functions.

Architectually, I really like this code because everything is broken down into small components. Maintaining this code would a breeze. That said, the reason the code is slow to solve the problem is because I separated everything out.

> The next password after ghijklmn is ghjaabcc, because you eventually skip all the passwords that start with ghi..., since i is not allowed.

My code increments that password one unit at a time, which means that I have to go through every `ghi....` combination before getting to the correct answer. For this test input, the code took so long to complete that I was positive that I had an infinite loop somewhere. It wasn't until I killed the program and added print statements that realized that the program is fine, its just slow. That said, it was really cool to see the password increment up super fast.

## The Problem Part 2

After I found the password in part 1, they wanted me to find the next password. So the code stays the same, but I did manually increment part 1's password by one to that it would be invalid. Otherwise the code returns the input because the input is a valid password.
