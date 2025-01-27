# Day 9: All in a Single Night
[link to problem](https://adventofcode.com/2015/day/9)

## The Problem
Every year, Santa manages to deliver all of his presents in a single night.

This year, however, he has some new locations to visit; his elves have provided him the distances between every pair of locations. He can start and end at any two (different) locations he wants, but he must visit each location exactly once. What is the shortest distance he can travel to achieve this?

For example, given the following distances:
```
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
```
The possible routes are therefore:

```
Dublin -> London -> Belfast = 982
London -> Dublin -> Belfast = 605
London -> Belfast -> Dublin = 659
Dublin -> Belfast -> London = 659
Belfast -> Dublin -> London = 605
Belfast -> London -> Dublin = 982
```
The shortest of these is `London -> Dublin -> Belfast = 605`, and so the answer is 605 in this example.

What is the distance of the shortest route?

### Thoughts

These problems always scare me because I suck at them. In Python, I would use [itertools.permutations](https://docs.python.org/3/library/itertools.html#itertools.permutations) to get the list of distinct possible routes, sum them up and keep the lowest value, but I don't have that function in Rust. The Python docs have and example of how to write the algorthim and I am hesitant to transcribe it to Rust because I do not fully understand (and will most likely make a mistake...).

I know that this is a simple problem. I know that combinatorially their are `n!/2` options. `n!` because you have `n` options for the first place and `n-1` options for the next, and so on. You divide by `2` because, with reference to this problem, going forward and going backwards gives you the same value. So in there example, we had 3 options, which leads to 3! (6) total permutations, but half of these permuations give identical results.

The input file only has 7 options. `7!`. I was going to say that `7!/2` was small enough to do by hand, but then I opened up a calculator and discovered thats `2,520` options...

Okay, maybe my anxiety was justified.

Anyway, I have thought about cheap ways of solving the problem. I could use a random number generator randomize the travel list and then compute the sums. If I did this enough times, say 10,000 time, I would definitely get the correct answer.

To me, casually throwing my vague understanding of dynamic programming at this problem seems like it would undermine my goals here, which are to learn.

Breath in, breath out and lets think about this.

### Thoughts after calming down

Let's answer some basic stuff first.

#### How do I want to represent the data?

These seems like dictionaries. My London dictionary will have keys for Dublin and Beldfast.

```
 london = {"dublin": 464, "belfast": 518}
 dublin = {"london": 464, "belfast": 141}
 belfast = {"london": 518, "dublin": 141}
```

The above should probably be in a dictionary too so that it can be easily referenced.

```
 distances = {
   "london": {"dublin": 464, "belfast": 518},
   "dublin": {"london": 464, "belfast": 141},
   "belfast": {"london": 518, "dublin": 141},
 }
```

Now, if I was iterating over a list like `["dublin", "london", "belfast"]`, I would start at index 0 and 1 like `distances[index_0][index_1]`, which is `distances['dubline']['london'] = 464`.

This representation makes sense to me bacause it makes the distance reference easy.

### Generating the permutations

The first question I should ask is, do I have to do this? I believe the answer is yes, but I want to take a few minutes and brainstorm if there are other ways to approach this problem.

There is the min distance subset approach where I pick a location and then find the shortest distance to the another location and then pick that location. This process repeats till you get to the end location. Then, you change your start location and try again.

Given that forward a backwards are the same, the swapping of starting locations should help with the pigeon hole issue of placing a extremely far distance at the end of the list because there are no other options left. The reason this will help is because when one of those two locations with that extremely far distance is the starting point, it will never pair with that far away place.

The more I think about it, the more this "min max solution" sounds promising.

I do not have time to write this up now, but I think these are the steps.

#### Steps:
- Parse file to create my `Dict[str: Dict[str: int]]`
- Iterate over the keys of the top level dict to get my starting points.
- Then I have my min max loop process.
  - Find the shortest distance to the next place.
  - Put that location in the `seen` set. (Note: the starting location should be in this set already)
  - Start from that next place and find the closest place that is not in the seen set
  - Repeat till size of seen set is equal to the number of locations.
  - Save that value (and the path for audit purposes) and move on to the next start point.
- Take the samest distanced travelled and hope that is the solution.

I will be really happy if this works, but I will not be able to find that out today.

### Solution
```rust
{{ #include ../../../advent_of_code/2015/day_9/src/main.rs }}
```

I fumbled through this, but I kept the outline steps I wrote in my heads and used it as a guiding star until I completed the implementation.

If you look at `fn main`, I start off creating the data structure I want, which is the nested HashMap that we talked about. I then have a function called `create_data_structure`, which fills in the data for the nested map (just like we talked about).

Everything after this point is related to the min-max loop that we are going to use to find the closest distance.

I create a variable called `shortest_distance` that will hold the the final answer. I initialize it to a way too big number so that it will replaced by the first total distances we get.

For the loop, I iterate of the outer HashMap. This will give use our starting location and the map all the other locations with their distances from the starting location.

We copy these items to variables called `current_loc` and `current_map`. These values will get updated as we travel. For example, if was start at London and then travel to Belfast, our new current location is Belfast, which means that our new current map should be the Belfast map that have all of the locations and distances relative to Belfast.

In order to make sure we do not travel to the same location twice, we create a `seen` HashSet use to track the locations that we have been to.

So the process is:
- We start at a location (set this to current location)
  - We add our current location the seen
  - We find the closest location to our current location
  - We set current location to the closet location (ie we traveled to the next location)
  - Repeat until we have traveled to all the locations (ie seen.len() == HashMap.len())
  
Of course, we did some math to track the total distances traveled, and, when needed, we updated our `shortest_distance` variable.

#### Thoughts on Solution

I find it visually ugly, but I think that is because I muddled through. All the steps I wanted are here, but I did not know how to write the steps. So I would writes things in main to test out ideas and once I got something working, I would copy it into a function and delete it from `main`. This style of programming works, but I care so much about "trying out an idea" that I spent little time on my variable names and code structure and quality.

I think this is my first time using a nested HashMap in Rust, so navigating that was kind of awkward. It was a lot to type and I sometimes got lost in the get a reference or get a mutable reference of the inner HashMap. At one point, I even got tired of typing, so I copied and pasted the signature whenever I needed it. I guess I could have declared the signature as a new type and that would have saved me tome typing.

The CLONING!!! I kept throwing that method at the wall everytime I ran into an issue instead of taking the time to think about my data and if the clone makes sense.

Though I had a clear outline of how I wanted to solve the clostest distance part, I couldn't translate it into code at first. I made a number of mistakes before I got it right.

## The Problem Part 2

The next year, just to show off, Santa decides to take the route with the longest distance instead.

He can still start and end at any two (different) locations he wants, and he still must visit each location exactly once.

For example, given the distances above, the longest route would be 982 via (for example) Dublin -> London -> Belfast.

What is the distance of the longest route?

### Thoughts:

I should be able to copy and paste the solution from part 1 and change the operators from less than to greater than and get the answer... I hope that's it because I really do not want to spend too much more time on this problem.

### Solution Part 2
Change operators and some variable names from like min to max and shortest to longest, but that was all.

```rust
{{ #include ../../../advent_of_code/2015/day_9_part_2/src/main.rs }}
```
