# Day 6: Probably a Fire Hazard

[link to problem](https://adventofcode.com/2015/day/6)

## The problem

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

## The Problem Part 2

You just finish implementing your winning light pattern when you realize you mistranslated Santa's message from Ancient Nordic Elvish.

The light grid you bought actually has individual brightness controls; each light can have a brightness of zero or more. The lights all start at zero.

The phrase turn on actually means that you should increase the brightness of those lights by 1.

The phrase turn off actually means that you should decrease the brightness of those lights by 1, to a minimum of zero.

The phrase toggle actually means that you should increase the brightness of those lights by 2.

What is the total brightness of all lights combined after following Santa's instructions?

For example:

> turn on 0,0 through 0,0 would increase the total brightness by 1.

> toggle 0,0 through 999,999 would increase the total brightness by 2000000.

### Thoughts

I actually think this is easier than part one. I don't need an enum anymore. I can have a regular struct that has an internal counter that goes up and down with a min value of zero. And the structure of the program is identical to the first one.

I am going to try to write this up quickly and move on to my next task for the night.

```rust
{{ #include ../../../advent_of_code/2015/day_6_part_2/src/main.rs }}
```

The changes were so minor that this win doesn't feel like it counts. I changed the enum to a struct and updated the methods to do the brightness thing. I also added a new method get the current brightness level, but my counter is of type usize, so I don't have to worry about the borrow checker. Anyway, since I kept the name `Light` when I made the change from enum to struct, all the type checking and setup remained valid. 
The next thing I changes was in the `change_lights` function, and all I had to do was call the `turn_on`, `turn_off`, and `toggle` method on the `Lights`.

Lastly, I changed how I do the counting at the end of the program. Then I updated the last print statement for kicks.

This feels like the part 2 was way too easy when compared to the part 1, but its probably the case that I made part 1 more complicated than it needed to be. I used an enum when 0, 1 and a few if statements would have worked. If that was my starting point, then part 2 would have been a lot more difficult than part 2. 
