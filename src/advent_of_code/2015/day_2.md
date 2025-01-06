# Day 2: I Was Told There Would Be No Math

[Link to problem](https://adventofcode.com/2015/day/2)

## The Problem

The elves are running low on wrapping paper, and so they need to submit an order for more. They have a list of the dimensions (length l, width w, and height h) of each present, and only want to order exactly as much as they need.

Fortunately, every present is a box (a perfect right rectangular prism), which makes calculating the required wrapping paper for each gift a little easier: find the surface area of the box, which is 2\*l\*w + 2\*w\*h + 2\*h\*l. The elves also need a little extra paper for each present: the area of the smallest side.

For example:
	
> A present with dimensions 2x3x4 requires 2\*6 + 2\*12 + 2\*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.

> A present with dimensions 1x1x10 requires 2\*1 + 2\*10 + 2\*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.

All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?

### Thoughts

For this problem, I wanted to play around with variable initialization. I couldn't get the `collect` call to return the just the three values in like a tuple that I could distribute to individual variables, so I settled on putting it into a Vec and separating them later.

The `side` variables are meh. I could have done something more fancy with them, but I was tired and it didn't seem worth it. The `extra` variable initialization though! I like that one. I rarely use random code block at all in my code, but using it like this to initialize a semi complicated variable lets me see the power of these blocks!

```rust
{{ #include ../../../advent_of_code/2015/day_2/src/main.rs }}
```


## The Problem Part 2

The elves are also running low on ribbon. Ribbon is all the same width, so they only have to worry about the length they need to order, which they would again like to be exact.

The ribbon required to wrap a present is the shortest distance around its sides, or the smallest perimeter of any one face. Each present also requires a bow made out of ribbon as well; the feet of ribbon required for the perfect bow is equal to the cubic feet of volume of the present. Don't ask how they tie the bow, though; they'll never tell.

For example:

> A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet.

> A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.

How many total feet of ribbon should they order?


### Thoughts

I made this problem extremely annoying by trying to be fancy. If you put all the sides into an Vec and sort it, then the first two entries will be the smaller two sides. That is straight forward. Another thing that should have been straight forward is getting the cubed size. All you have to do is multiple `dimension[0] * dimension[1] * dimension[2]`, but no. I wanted to be fancy and use `reduce`.

Fun fact. If you are iterating over a Vec, then you have access to references of the elements. If you call reduce on a Vec of references, then Rust expects you to deal with references in your reduce. I don't want to deal with references. As such, it took me 10 minutes to figure out that I had to use `map` to make non-reference copies of the elements of the Vec before calling reduce.

All in all, being fancy was stupid. Next time just mupltiple the items together...

```rust
{{ #include ../../../advent_of_code/2015/day_2_part_2/src/main.rs }}
```
