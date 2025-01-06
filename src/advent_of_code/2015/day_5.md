# Day 5: Doesn't He Have Intern-Elves For This?

[link to the problem](https://adventofcode.com/2015/day/5)

## The Problem

Santa needs help figuring out which strings in his text file are naughty or nice.

A nice string is one with all of the following properties:

1. It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
2. It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
3. It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.

For example:
> ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.

> aaa is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.

> jchzalrnumimnmhp is naughty because it has no double letter.

> haegwjzuvuyypxyu is naughty because it contains the string xy.

> dvszwmarrgswjxmb is naughty because it contains only one vowel.

How many strings are nice?

### Thoughts

This sounds like a one pass scan of the word with various check. For example, the first check is the vowels, so you add a set of vowels and check the current character against it and count the positive hits.

The second condition is two character condition. If you are iterating over the input one character at a time, it makes sense to use index 0 to set the first `prev_char` and then use the pair of the previous character and the current character to check this condition (Must remember to update `prev_char`).

The third condition has a similar setup to the second condition, but it is a "not containt", which means means that we can use this for early termination (ie, continue to the next input, but we don't have to. This could just be a flag as well).

On the face of it, this seems pretty straight forward. Lets try to write it up.

```rust
{{#include ../../../advent_of_code/2015/day_5/src/main.rs }}
```

It was actually that straight forward. The only new insight I had while writing it up is that the `prev_char` is easier to handle as a [Option](https://doc.rust-lang.org/std/option/enum.Option.html) because I could then use a match statement to handle all of the stuff linked to `prev_char` in one place.

## The Problem Part 2

Realizing the error of his ways, Santa has switched to a better model of determining whether a string is naughty or nice. None of the old rules apply, as they are all clearly ridiculous.

Now, a nice string is one with all of the following properties:

1. It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
2. It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.

For example:

> qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).

> xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.

> uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.

> ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.

How many strings are nice under these new rules?

### Thoughts

I agree that the first set of ruls was ridiculous, but I am not sure I am a fan of the new rules...

The first rules is tricky beause they cannot overlap. For example, if I kept a hashmap of each pair of a string, then `aaa` would count as 2 pairs. And `aaaa` should count as 2 pairs, but how do you know that they are not overlapping? You cannot split them by even and odd indices because the first pair may be one an even index while the second pair is on an odd index.

I am going to have to continue thinking about this.

The second condition is more straight forward. It is asking if the current character is equal to the character that appeared 2 indices ago. That is only slightly different than the `prev_char` in part 1. In thoery, you can use the `prev_char` result to setup a match for `prev_prev_char` and then do the check there (LOL). I'll probably write it up that way because thinking about it makes me giggle.

Alright, I need to figure out rule 1.

What do I know:
- I know I can use a hashmap to do a seen check on the pairs.
- I also know that the "no overlap" is currently an issue that I need to overcome.

How do I prevent overlap?
- I could have a second hashmap to keep track of the indices? For example, take
`xyxy`. When we see `xy` for the first time, one hashmap will keep a count seens, which will get set to one and the other will keep an array of indices where this has been seen (current entry being (0,1)). Then, when the second `xy` comes along, we can check the hashmap to array first and verify that the previous `xy` has different indices before sending it to the hashmap count to up the count. Though I think this will work, it sounds like too much work for this problem..

- For an overlap to exist, the previous char must be equal to the current char, right? Maybe I can take advantage of that. I can keep track of the `prev_char` and the `prev_prev_char`. If `prev_prev_char` is None, and the `prev_char` is `a`... I might need a `prev_prev_prev_char` for this so that I have 4 chars that I can compare against because if all 4 are `a`, then we have `aaa` and that is fine. If we have `None, aaa`, then we know we have overlap.

The seconds approach sounds more straight forward, but only of you have the chars on an array where you can use the index to explicitly previous chars... 

I guess that makes sense. I can convert the line to an Vec<chars> and then iterate over the Vec. I will have access to the index and for rule 2, I can check to if index i and index i-2 are the same char. For rule one, if the current char and the previous char are equal, then I can check the two character before that and if they are all eqal, it is not an overlap.

I think this will work. Let me try writing it up.


```rust
{{#include ../../../advent_of_code/2015/day_5_part_2/src/main.rs }}
```

That actually went much smoother than I expected it to go! When writing it up, I realized that I did not need the first index of the `vec_chars`, so I created a slice of it that goes from index 1 to the end and iterated over that. This meant that I could note use `enumerate` to manage the index for me, which was fine.

I then used a match statement to manage the different cases that I was talking about before for rule 1. If the index is 1, then there are only 2 chars so overlap is not possible. If the index is 2, then there are 3 chars. Overlap is now possible, so you have to make sure that all three chars are not the same value (Note: I think I wrote the case compliment if statements because it is easier for me to think about when I want to act as appose to when I don't want to act, but I probably could have gotten away with a simpler conditional if I did it the other way... I may change it...). If the index is 3, then there are 4 chars which means that have to check the `aaaa` and `baaa` cases.

One thing that is cool about the HashMap is that we have the [Entry API](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry), which allows me to create an entry counter in one line via defining what I want to happen for new entry and what I want to happen for existing entries.

With regard to Rust, I can say that today was the first time I used the `any` method that is defined for iterators. It is pretty cool and useful that that HashMap `values` method returns a type (The the is Values) that implements and iterator. 


Note: I updated the `match` index == 2 `if` check. It is much easier to read now.
