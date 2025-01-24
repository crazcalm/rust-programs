# Day 8: Matchsticks

[link to problem](https://adventofcode.com/2015/day/8)

## The Problem

Space on the sleigh is limited this year, and so Santa will be bringing his list as a digital copy. He needs to know how much space it will take up when stored.

It is common in many programming languages to provide a way to escape special characters in strings. For example, C, JavaScript, Perl, Python, and even PHP handle special characters in very similar ways.

However, it is important to realize the difference between the number of characters in the code representation of the string literal and the number of characters in the in-memory string itself.

For example:

>"" is 2 characters of code (the two double quotes), but the string contains zero characters.

> "abc" is 5 characters of code, but 3 characters in the string data.

> "aaa\"aaa" is 10 characters of code, but the string itself contains six "a" characters and a single, escaped quote character, for a total of 7 characters in the string data.

> "\x27" is 6 characters of code, but the string itself contains just one - an apostrophe ('), escaped using hexadecimal notation.

Santa's list is a file that contains many double-quoted string literals, one on each line. The only escape sequences used are \\ (which represents a single backslash), \" (which represents a lone double-quote character), and \x plus two hexadecimal characters (which represents a single character with that ASCII code).

Disregarding the whitespace in the file, what is the number of characters of code for string literals minus the number of characters in memory for the values of the strings in total for the entire file?

For example, given the four strings above, the total number of characters of string code (2 + 5 + 10 + 6 = 23) minus the total number of characters in memory for string values (0 + 3 + 7 + 1 = 11) is 23 - 11 = 12.

### Thoughts

I am not sure of quick way of solving this that is not brute force pattern matching.

The cases seem to be:
- String with characters
  - Char count + 2
- String with characters and esc
  - Char count + 2 + 1 for each escaped character
- Hex String
  - ???
  - I am not sure if each Hex is 6 characters and I am not sure if `chars` will the Hex in pieces or the string presentations...
  
  
  Come to think about it, the escape char list may pose a problem. `""` is fine, but `\\` in a string could prove troublesome because it overlaps with Hex. So, for "\\x27", is the string 2 chars (`\'`) or 7/8 chars?
  
  Meh. I guess I will find out.
  
  #### Thoughts post some investigation
  
  I think I was wrong about this problem. All the input will come in as strings and I need to deduce which parts of it count as part of the string.
  
  For example, if "/x27" comes in, I need to be able to identify that is ti a hex value. Hex values are seemingly single chars, so should have a string count of 1 and a character count of 6.
  
  For the escape characters, I need to figure out what counts as a escaped character. So, "/'", "/"" and "//" all count. 
  
  So, if the string is not a Hex, then I can do a 2 character check to see if the string has escaped characters.
  
  Also, all of the inputs are in quotes, so calling "data.len()" when data is `""` will give me 2.
  This means that I am litary scanning the string trying to decide what the string number should be for all of these. lol
  
  Notes:
  - all strings should be of length 2 or greater.
  - If hex, then the string count is 1
  - If not hex, minus 1 for every `/` in string?
  
  I don't know if this is right, but I can write it up and test it against the test case.
  
  #### Mistakes
  1. I miss interpreted the problem. The hex code can be found within the string at any point in time. For example, `"\x5em\"squulpy"` starts with a hex and then there is the rest of the string. So (I think) `\x5e` should count as 1 string char, but this could have been `"y\x5em\"squulp"`.
  
  2. Over counting. In my first implementation, I did not consider the case `\\\x5e`, which is `\\` + `\x5e`. So the mistake I made was iterating over the it character by character, when really I needed to skip ahead to account for escaped characters that I have already counted.


### Solution
  ```rust
  {{ #include ../../../advent_of_code/2015/day_8/src/main.rs }}
  ```

The code it pretty ugly. That aside, the print statements gave me a ton of audit information. Without that, I definitely would not have solve this one tonight.

Anyway, the trick to solving this problem is understanding the window you are looking at. I choose the window of two characters, the current one and the previous one. I took the approach of counting the escaped characters and adding 2 to the for the quotes (Note: When I iterate over the Vec<char>, i shrunk the bounds to leave out the start and end quotes).

For the while loop, lets examine `aa\\\x5em`:

> My first pair is `aa`. Nothing happens here, so we increment the index by one.

> Next is `a\`. Nothing again. Increment by 1.

> Next is `\\`. We have an escaped backslash. Increment by two to prevent double counting.

> Next is `\x`. We have the start of a hex code. These are exactly 4 characters, so we increment by 4.

The index is not out of bound, so we stop the count. It should be noted that the `m` was never looked at, but you need at least 2 characters for an escape, so not looking at it is fine.

Here is example output that shows why I found the print statements so helpful (It was kind of hard to read it as a code block, so I quoted each line of the output).

> "" - Char count: 2 - Escaped count: 0 - Diff (char - eascped - 2 = string count) 0

> "abc" - Char count: 5 - Escaped count: 0 - Diff (char - eascped - 2 = string count) 3

> "aaa\"aaa" - Char count: 10 - Escaped count: 1 - Diff (char - eascped - 2 = string count) 7

> "\x27" - Char count: 6 - Escaped count: 3 - Diff (char - eascped - 2 = string count) 1

> "\\\x5em\"squulpy" - Char count: 18 - Escaped count: 5 - Diff (char - eascped - 2 = string count) 11

> Chars: 41, Strings: 22, Diff: 19

## The Problem Part 2
Now, let's go the other way. In addition to finding the number of characters of code, you should now encode each code representation as a new string and find the number of characters of the new encoded representation, including the surrounding double quotes.

For example:

> "" encodes to "\"\"", an increase from 2 characters to 6.

> "abc" encodes to "\"abc\"", an increase from 5 characters to 9.

> "aaa\"aaa" encodes to "\"aaa\\\"aaa\"", an increase from 10 characters to 16.

> "\x27" encodes to "\"\\x27\"", an increase from 6 characters to 11.

Your task is to find the total number of characters to represent the newly encoded strings minus the number of characters of code in each original string literal. For example, for the strings above, the total encoded length (6 + 9 + 16 + 11 = 42) minus the characters in the original code representation (23, just like in the first part of this puzzle) is 42 - 23 = 19.

### Thoughts

It is 2 am, I'm tired, and I should go to bed, but this looks a lot simplier than part 1.

> Each line gets a plus 2 because we have new quotes.

I iterate over each char and add 1 everytime I see a `"` of a `\`. This accounts for the new escapes. To get the new size, we `characters.len() + new_escapes + 2 (for the new quotes)`.


> "" => characters.len() + new_escapes + new_qoutes = 2 + 2 + 2 = 6

> "abc" => 5 + 2 + 2 = 9

> "aaa\"aaa" => 10 + 4 + 2 = 16

> "\x27" => 6 + 3 + 2 = 11

### Solution Part 2
  ```rust
  {{ #include ../../../advent_of_code/2015/day_8_part_2/src/main.rs }}
  ```

This was definitely much easier than part 1.
