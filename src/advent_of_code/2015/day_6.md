# Day 6: Probably a Fire Hazard

Because your neighbors keep defeating you in the holiday house decorating contest year after year, you've decided to deploy one million lights in a 1000x1000 grid.

Furthermore, because you've been especially nice this year, Santa has mailed you instructions on how to display the ideal lighting configuration.

Lights in your grid are numbered from 0 to 999 in each direction; the lights at each corner are at 0,0, 0,999, 999,999, and 999,0. The instructions include whether to turn on, turn off, or toggle various inclusive ranges given as coordinate pairs. Each coordinate pair represents opposite corners of a rectangle, inclusive; a coordinate pair like 0,0 through 2,2 therefore refers to 9 lights in a 3x3 square. The lights all start turned off.

To defeat your neighbors this year, all you have to do is set up your lights by doing the instructions Santa sent you in order.

For example:
> turn on 0,0 through 999,999 would turn on (or leave on) every light.

> toggle 0,0 through 999,0 would toggle the first line of 1000 lights, turning off the ones that were on, and turning on the ones that were off.

> turn off 499,499 through 500,500 would turn off (or leave off) the middle four lights.

After following the instructions, how many lights are lit?

### Thoughts

This seems like a mini parser... We have strings like `turn on 0,0 through 999,999`, which are `<command> <start_coordinates> through <end_coordinates>`.

I don't think I want to write a parser though, so lets split the words up by white space.

Command:
- turn on
- turn off
- toggle

The coordindates can be split by `,` to get the x and y.

I need to think about th lights situation. I am thinking about making them an enum.

Enum:
 - on
 - off
 
Enum Methods:
  - turn on
  - tun off
  - toggle
  
Then the grid will be a 2 dimensional Vec filled with my enums. I can iterate over it and call the appropriate method.

Speaking of iteration, I need to be able to figure out the start and stop of my iterations...

Lets say, they give me 2,4 through 5,6. This means that I from 2,4 to 2,999, the to 3,0 to 3,999, 4,0 to 4,999, then 5,0 to 5,6. Wait, I don't think I read that properly...

> Each coordinate pair represents opposite corners of a rectangle, inclusive; a coordinate pair like 0,0 through 2,2 therefore refers to 9 lights in a 3x3 square. The lights all start turned off.

Opposite corners! So 0,0 through 2,2 is 0,0 | 0,1 | 1,2 | 1,0 | 1,1 | 1,2 | 2,0 | 2,1 | 2,2.

This means that 2,4 through 5,6 is 2,4 | 2,5 | 2,6 | 3,4 | 3,5 | 3,6 | 4,4 | 4,5 | 4,6 | 5,4 | 5,5 | 5,6.

In the above cases 2..=5, 4..=6.

Note:
I thing the coordinates are (y, x) in the above case because `Vec[y][x]`, but, since we only care about the number of lights on, I think I can get away with using `Vec[x][y]`. 


They are giving us opposite corners of the rectangle. All of the example are bottom left corner to top right corner, but I do not know if all the inputs are these corners in this order.
- I took a quick glance at the inputs, and I think they are all bottom left to top right.

This means that I can for like:

```
for x in 2..=5:
  for y in 4..=6:
     vec[x][y] = vec[x][y].<command>
```

I will try to write it up, but it is getting late, so I do not know if I will finish it tonight.

```rust
{{ #include ../../../advent_of_code/2015/day_6/src/main.rs }}
```

It took longer than I thought to write, but it wasn't that bad. To make the program less confusing, I decided to keep the coordinates as (x1, y1), (x2, y2). Outside of that, some annoying things were:
- Indecing on a Vec<&str>.
  - In Rust, we cannot use negative indexes to get the items of that end of the Vec
- Parsing the coordinates:
  - I had to make sure their type was usize so that I can use them when indexing the grid.
  - Also, the long string of methods I used to actually parse the &str to usize was annoying. Cloning &str clones the pointer and not the value, so I converted to a String, but Strings do not have a split method. So I converted the String to str and then did the split and parse.
- The program is long! I did not like how long this is.

