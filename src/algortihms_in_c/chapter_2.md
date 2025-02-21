# Chapter 2

This chapter is an introductory chapter used to justify the use of C as the languaged used in this book. Post that, it goes over a single example with the purpose of introducing the C programming language to the reader.

## Example: Euclid's Algorithm
To begin, we'll consider a C program to solve a classic elementary problem: "Reduce a given fraction to its lowest to its lowest terms." We want to write 2/3, not 4/6, 200/300. or 178468/267702. Solving this problem is equivalent to finding the _greatest common divisor_ (gcd) of the numerator and the denominator: the largest integer which divides them both. A fraction is reduced to lowest terms by dividing both numerator and denominator by their greatest common divisor. An effective method for finding the greatest common divisor was discovered by the ancient Greeks over two thousand years ago: it is called _Euclid's algorithm_ because it is spelled out in Euclid's famous treatise _Elements_.

Euclid's method is based on the fact that if _u_ is greater than _v_ then the greatest common divisor of _u_ and _v_ is the same as the greatest common divisor of _v_ and _u_ - _v_. This observation leads to the following implementation in C:

```c
{{#include ../../Algorithms_in_C/ch_2/gcd/gcd.c}}
```

Here is the implementation written in Rust:

```rust
{{#include ../../Algorithms_in_C/ch_2/gcd/rust-gcd/src/main.rs}}
```

### Notes:
I am not the best with C syntax, but there are some things that we need to cover for both C and Rust before moving on:

#### Copying
1. In C, functions create local copies of their arguments, so even though _x_ and _y_ are passed into the _gcd_, their original values are not modified because _u_ and _v_ are not references to _x_ and _y_.

This is why we can write `printf("%d %d\n", x, y, gcd(x,y))`. In theory, the _gcd_ call gets evaluated before the _printf_ call, so, if _x_ and _y_ were references and not copies, their updated values would have been printed in the _printf_ call.

The Rust code is doing the exact same thing, but the mechanics of why it is happening are slightly different.

```
println!(
	"gcd({}, {}) => {}",
    elements[0],
    elements[1],
    gcd(elements[0], elements[1])
);
```

Rust has [_Ownership_](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#ownership-rules), which can be summed up as follows:

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

In general, this means that variables passed to functions and methods or also passing ownership over those varables to functions and methods. I am not sure how things are handled with `println!` because that is a macro, but, the point remains that if _element[0]_ and _element[1]_ are passed to _gcd_ and _gcd_ gets evaluated first, then the `println!` macro should have printed the updated values for _elements[0]_ and _elements[1]_, right? Also, based off the rules of _Ownership_, it seems like we cannot have two versions of _elements[0]_ and _elements[1]_ around. So, what is going on?

The answer is the [_Copy Trait_](https://doc.rust-lang.org/std/marker/trait.Copy.html). The docs cover a lot, but the gist is that if a struct implements the _Copy Trait_, then instead of _moving into the function_ (the transfer of ownership) it will __implicitly__ create a copy of itself and pass that into the function. The data type [u32](https://doc.rust-lang.org/std/primitive.u32.html#impl-Copy-for-u32) implements that _Copy Trait_, so ownership over _elemtns[0]_ and _elements[1]_ was never transfered because they were never moved.

#### scanf

We do not have _scanf_ in Rust. As a result, I had to scan and parse the input myself, which led to a lot of extra code being added to the main function.

### Thoughts

I really appreciated seeing _gcd_ implemented using subtraction. Previously, I have only seen it implemented using remainder operators or mod operators, which have a direct relationship to subtraction, but I never think of them being on the same level. For example, I can understand someone not knowing modular arithmetic, but I would be completely surprised if they did not know how to subtract. So, for me, seeing this implemented using subtraction is a suble remnder that all this stuff can be written in such basic forms that it is hard to imagine anything of this stuff every being "difficult".
