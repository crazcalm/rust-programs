# Day 4: The Ideal Stocking Stuffer

[link to the problem](https://adventofcode.com/2015/day/4)

## The Problem

Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts for all the economically forward-thinking little girls and boys.

To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes. The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a number in decimal. To mine AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.

For example:

>    If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.

>    If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....

Your puzzle input is `iwrupvqb`.

### Thoughts

I took a brief look at the [MD5 Wiki page](https://en.wikipedia.org/wiki/MD5) and no obvious ways to force my wanted output jumped out at me, so I think I am going to go with the brute force approach. In the largest example, it looked like it was a little over a million MD5 computations (assuming that you start from 1).

```rust
{{ #include ../../../advent_of_code/2015/day_4/src/main.rs }}
```

I was right in that brute force was the way to go, but it was also wrong of me to do this problem while tired. I made so many little mistakes that it became frustrating. For example, I miss read the problem and thought that I had to submit the MD5 hash digest as the answer. One nice thing I can say is that even in my tired state, I had to good sense to take input from stdin. By putting the example keys and my key in a file I was able to solve all three problems at once, which allowed me to verify the examples while solving the problem for my input. 

Anyway, `md5::compute` returns the `digest`, but it doesn't implement the [Display Trait](https://doc.rust-lang.org/std/fmt/trait.Display.html) or have a single method to give me back the Hash in string form. I luckily noticed that it implemented the Debug Trait and that the debug string was the has I wanted. Though, I did look at their implementation for the Debug Trait and it look like it converted the digest to LowerHex before so that it could use the LowerHex fmt method to produce the string.... Wait... LowerHex is a standard library type... I was suppose to convert the digest to my wanted type and then do what I wanted to it.

I am going try doing that and see if it cleans up the code a bit.

Yeah, I was still wrong. [LowerHex](https://doc.rust-lang.org/std/fmt/trait.LowerHex.html) is a trait used for formatting strings. The full mapping of formatting trait shorcuts can be found [here](https://doc.rust-lang.org/std/fmt/index.html#formatting-traits).

All in all, the change to the code was small. Now, I am using the LowerHex formatter instead of the Debug formatter. Though this makes my code a little cleaner, both traits were using the same fmt code under the hood, so its all kind of pointless...

## The Problem Part 2

Now find one that starts with six zeroes.

### Thoughts

I was going to go to bed, but I this seems like the exact same problem as the first. so I am going to take a crack at it.

...

They got me. The wording is different between part 1 and part 2. In part 1 it was "at least five zeros" and now it is "starts with six zeros", which means that if it starts with seven zeros, its is not correct! Given that I now have to care about the value that comes after the six zeros, I moved to using regex. The regex that I am currently using is `^0{6}[a-zA-Z1-9]`, but I am pretty sure this is overkill because hexidecimal is a-f and 0-9. Plus, I am explicitly setting it to LowerHex, which means that I do not have to care about the A-F.


```rust
{{ #include ../../../advent_of_code/2015/day_4_part_2/src/main.rs }}
```

Hello 4 am and goodnight to the world.
