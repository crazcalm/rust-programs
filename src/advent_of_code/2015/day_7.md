# Day 7: Some Assembly Required

[link to problem](https://adventofcode.com/2015/day/7)

## The Problem

This year, Santa brought little Bobby Tables a set of wires and [bitwise logic gates](https://en.wikipedia.org/wiki/Bitwise_operation)! Unfortunately, little Bobby is a little under the recommended age range, and he needs help assembling the circuit.

Each wire has an identifier (some lowercase letters) and can carry a [16-bit](https://en.wikipedia.org/wiki/16-bit) signal (a number from 0 to 65535). A signal is provided to each wire by a gate, another wire, or some specific value. Each wire can only get a signal from one source, but can provide its signal to multiple destinations. A gate provides no signal until all of its inputs have a signal.

The included instructions booklet describes how to connect the parts together: x AND y -> z means to connect wires x and y to an AND gate, and then connect its output to wire z.

For example:

> 123 -> x means that the signal 123 is provided to wire x.

> x AND y -> z means that the [bitwise AND](https://en.wikipedia.org/wiki/Bitwise_operation#AND) of wire x and wire y is provided to wire z.

> p LSHIFT 2 -> q means that the value from wire p is [left-shifted](https://en.wikipedia.org/wiki/Logical_shift) by 2 and then provided to wire q.

> NOT e -> f means that the [bitwise complement](https://en.wikipedia.org/wiki/Bitwise_operation#NOT) of the value from wire e is provided to wire f.

Other possible gates include OR ([bitwise OR](https://en.wikipedia.org/wiki/Bitwise_operation#OR)) and RSHIFT ([right-shift](https://en.wikipedia.org/wiki/Logical_shift)). If, for some reason, you'd like to emulate the circuit instead, almost all programming languages (for example, C, JavaScript, or Python) provide operators for these gates.

For example, here is a simple circuit:
```
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
```
After it is run, these are the signals on the wires:
```
d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456
```
In little Bobby's kit's instructions booklet (provided as your puzzle input), what signal is ultimately provided to wire a?

### Thoughts

Though I have a general idea of what all of this means, it is best to refresh myself on the terms and operations before breaking down the problem.

So bitwise operations happen on a [bit array](https://en.wikipedia.org/wiki/Bit_array), which you can think of as your binary representation of a number using 1's and 0's. We were told that our wire can carry up to a 16 bit signal. A bit is single place holder for a 0 or a 1. A 16 bit signal is 16 bits, which implies that we have 16 place holders for a 0 or 1.

For example:
> 0000000000000000 and 1111111111111111 are both 16 bits

Remember that we are using a binary representation of a number, which is really your number (which is typically in base 10) converted to base 2. Given that there are 16 place holders and there are 2 options for each place (the options being 1 or 2), the largest number we can represent is (2^16 -1) == 65536 -1 == 65535 (I think the minus 1 is becauase we start our count at zero, but I am not sure). Anyway, I mention that because they mention that a number can be from 0 to 65535.

It should be noted that all the numbers in this range is positive. This is because we ran out of bits. If we wanted to represent a negative number, we would have to use a bit to represent whether or not the number was positive or negative. In doing so, we would lose a factor of 2 because one bit would be dedicated to the sign (positive or negative) of the number.

Moving on to the bitwise operators, the list we are given is:
- AND
- OR
- LSHIFT
- RSHIFT
- NOT (complement)

The Wiki page gives examples of these operations using 4 bits ([a nibble or half a byte](https://en.wikipedia.org/wiki/Nibble)):

```
    0101 (decimal 5)
AND 0011 (decimal 3)
  = 0001 (decimal 1)
```

```
   0101 (decimal 5)
OR 0011 (decimal 3)
 = 0111 (decimal 7)
```

```
NOT 0111  (decimal 7)
  = 1000  (decimal 8)
```

Shifts involve shifting all bits over to the left or right X number times. It should be noted that there is a different between Arithmetic shifts and Logical shift, and our link was to Logical shifts.

For logical shifts, all bits move a placeholder to the left and the least significat bit (LSB -- placeholder 0) gets replaced with a 0. The same process happens for right shift just in the opposite direction. So the most significant bit is replaced with 0. You may be curious about the bits that get pushed off the end. They are gone forever. 

For example:
```
0011 (3) + LSHIFT 1 => 0110 (6)
0011 (3) + LSHIFT 2 => 1100 (12)
```

```
0011 (3) + RSHIFT 1 => 0001 (1)
0011 (3) + RSHIFT 2 => 0000 (0)
```

#### Thoughts on the actual problem

This is like a mini compiler. I say that because we are given a string to parse to set variables. We are then given other strings that use our stored variables in operations, which then set other variables. Then, at the end of the program, we need to be able to state the current value of one of our variables.

There example was:
```
123 -> x            x = 123 == 0000000001111011
456 -> y            y = 456 == 0000000111001000
x AND y -> d        
                    0000000001111011
               AND  0000000111001000
				--------------------
				    0000000001001000

                    d = 72 == 0000000001001000

x OR y -> e
                    0000000001111011
	            OR  0000000111001000
				--------------------
				    0000000111111011

                    e = 507 == 0000000111111011

x LSHIFT 2 -> f
                    f = 492 == 0000000111101100

y RSHIFT 2 -> g
                    g = 114 == 0000000001110010 

NOT x -> h
                    h = 65412 == 1111111110000100
NOT y -> i
                    i = 65079 == 1111111000110111
```

##### The Parsing:

Spliting by whitespace, we have:

```
[123, ->, x]           | len 3
[456, ->, y]           | len 3
[x, AND, y, ->, d]     | len 5
[x, OR, y, ->, d]      | len 5
[x, LSHIFT, 2, ->, f]  | len 5
[y, RSHIFT, 2, ->, g]  | len 5
[NOT, x, ->, h]        | len 4
[NOT, y, ->, i]        | len 4
```

If the len is 3, we can directly set a variable.

If the len is 4, we take the complement of one variable to set another.

if the lent is 5, we check the item at index 1 to figure out which operation we are doing.

##### The Operations

The [Rust Book](https://doc.rust-lang.org/book/appendix-02-operators.html) as a page of all the operators.

```
AND
&	expr & expr	Bitwise AND	BitAnd
&=	var &= expr	Bitwise AND and assignment	BitAndAssign


OR
|	expr | expr	Bitwise OR	BitOr
|=	var |= expr	Bitwise OR and assignment	BitOrAssign

RSHIFT
>>	expr >> expr	Right-shift	Shr
>>=	var >>= expr	Right-shift and assignment	ShrAssign

LSHIFT
<<	expr << expr	Left-shift	Shl
<<=	var <<= expr	Left-shift and assignment	ShlAssign
```

Note: The `NOT` (!) operator was not listed on that page, but !x works on u16.

##### Storing values
Is value is a `u16` and I think I can store them in a HashMap<(String, u16)>


It took so long to write all of this out, that I do not feel like writing it up anymore...

### Mistakes...

I did not pay enough attention to this one line...
```
A gate provides no signal until all of its inputs have a signal. 
```

This means that the data will come out of order.

Off the top of my head, I am thinking about maintaining a list with the data that cannot be process yet and continuously iterating over it until it all gets processed, but I am not sure if I can do that easily though.

Regardless, I need to be able to reject data that connot be processed yet and dave it for later.

Note: Though the command are out of order, the order of which they are seen might actually matter because a series of commands can produce different outputs based on the order (Consider the case where a varaible x was set twice. Any commands using x should have two differnt results depending on which version of x they see. -- debunked! I took a quick glance at our data and nothing is being set twice).

Alright, I guess I can keep a left of "still need to process" items. At the start of each input iteration, I can try to process the items in that list. If nothing gets processed, I move onto the streamed input.

At the end of the program. I try to process the "still need to process" items over and over again until the entire list is done?

### Solution
```rust
{{ #include ../../../advent_of_code/2015/day_7/src/main.rs }}
```

This was pretty painful. I made silly mistakes, like not understand that the commands are coming out of order. Post that, I made a bunch of small programming errors in the `needed_vars_set` function that led to infinite loops.

All in all, I am really glad that I took time to think through the parts of the problem I did understand correctly. I think that if I were confused by any of the bitwise operations or the initial line parsing, I would have made so many mistakes that I would have given up out of frustration.


## The Problem Part 2

Now, take the signal you got on wire a, override wire b to that signal, and reset the other wires (including wire a). What new signal is ultimately provided to wire a?

### Thoughts

ITS THE SAME PROBLEM WITH ONE VARIABLE CHANGED!!!

I updated the value for `b` in the input and ran the code again to get the answer. Off the top of my head, I am not exactly sure how this problem is any different from part 1.

Meh. At least I solved it.
