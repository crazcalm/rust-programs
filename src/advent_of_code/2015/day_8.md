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
  - all strings shoulf be of length 2 or greater.
  - If hex, then the string count is 1
  - If not hex, minus 1 for every `/` in string?
  
  I don't know if this is right, but I can try ti write it up and test it against the test case.
  
  I am not done, but I have to step away to do something. Anyway, one thing I overlooked is that hex are located within of the string.
  
  For example, `"\x5em\"squulpy"` starts with a hex and then there is the rest of the string. So (I think) `\x5e` should count as 1 string char.
  
  I think this means that if I see `\x` in the string, I should subtract 3 (this is because 4 chars become 1).
  
  Now I am over counting the somewhere. Though I would like to continue, I have to go.

