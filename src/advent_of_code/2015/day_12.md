# Day 12: JSAbacusFramework.io

[link to problem](https://adventofcode.com/2015/day/12)

## The Problem

Santa's Accounting-Elves need help balancing the books after a recent order. Unfortunately, their accounting software uses a peculiar storage format. That's where you come in.

They have a JSON document which contains a variety of things: arrays ([1,2,3]), objects ({"a":1, "b":2}), numbers, and strings. Your first job is to simply find all of the numbers throughout the document and add them together.

For example:

> [1,2,3] and {"a":2,"b":4} both have a sum of 6.

> [[[3]]] and {"a":{"b":4},"c":-1} both have a sum of 3.

> {"a":[-1,1]} and [-1,{"a":1}] both have a sum of 0.

> [] and {} both have a sum of 0.

You will not encounter any strings containing numbers.

What is the sum of all numbers in the document?

### Thoughts

I don't want to use a JSON library for this. This seems like a string parser will do. I think I an use a state machine to capture the number.

States:
- InNumber
- OutNumber

The first character is a `-` or a digit and the rest of the characters are digits. Since this is JSON, the starting state is OutNumber.

Everytime I capture a number, I will add it to the sum.

In terms of representing the number, I will build it as a string and parse it once I hit the OutNumber state.

Given that there are only two states, I can probably use a boolean for them.

### Solution
The code is simplier than I thought. Technically speaking, it is a state machine, but since there are only two states, I do not need a variable to keep track of it.
```rust
{{ #include ../../../advent_of_code/2015/day_12/src/main.rs }}
```

## The Problem Part 2

Uh oh - the Accounting-Elves have realized that they double-counted everything red.

Ignore any object (and all of its children) which has any property with the value "red". Do this only for objects ({...}), not arrays ([...]).

> [1,2,3] still has a sum of 6.

> [1,{"c":"red","b":2},3] now has a sum of 4, because the middle object is ignored.

> {"d":"red","e":[1,2,3,4],"f":5} now has a sum of 0, because the entire structure is ignored.

> [1,"red",5] has a sum of 6, because "red" in an array has no effect.


### Thoughts

They want me to keep track of the curly brackets. That said, this is pretty tricky because "red" could be the last item in the object, and, if it is, I have to ignore all the numbers collected in that object.

I also have to take nested children into consideration as well. So...Is this like a Stack where each entry represents a level of depth? Maybe not a stack. I think I can use a Vec and have each index be a sum for that depth. If I see "red", everything at the idex and past it get set to zero.

At the end of the program, I sum the Vec.

This program will be easier to write if I use a Vec of a fixed size. The input.txt file doesn't look like it has too many nested objects. I will start with a size of 10 and bump it if I need to.

### Second attempt

I underestimated the problem... So here I am days later attempting to solve it again.

This is awkward. Enough time has passed that I forgot what I was doing and which mistakes I made. Oh well.

I think I wanted to solve this recursively? Like, sum all object from the inner nested ones outward. For example,

We know this `{"d":"red","e":[1,2,3,4],"f":5}` is 0 because there is a "red" in the object.
We know this `{"d":"1","e":[1,2,"red",4],"f":5}` is 13 because the red is ignored (array rule).

If we can sum these individually and sum them together as we go back up the stack, we should get the answer.

Now how to I recursively break the structure down...

Something like this might work. Let me try it.
```
def test(vec, start):
   sum = 0
   string = "{"
   for x in range(index, len(vec)):
       if vec[x] == "{":
	     sum += test(vec, x + 1)
	   elif vec[x] == "}":
	     # sum rules
		 # do this is a different function
		 string += "}"
		 sum += func()
	   else:
	     string += vec[x] 
```

I admit defeat. I had a lot of fun, but nothing I did work.

I am ready to move on with life.
