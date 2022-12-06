# [Advent of Code](https://adventofcode.com/)

## 2022

Here are some things I have learned about Rust through solving the Advent of code puzzles.

Feature/function | Day/file | Description
:-- | :-- | :--
`filter_map` on iterator | [3](./2022/day03/part1/src/main.rs) | Instead of filtering and then mapping, you just return an optional and all None values are automatically filtered out
`tuples` from `itertools` | [3](./2022/day03/part2/src/main.rs) | Using this seemed like magic to me: you can transform an iterator like [s1, s2, ...] into [(s1, s2, s3), (s4, s5, s6), ...] just by adding `.tuples()` before a `.map(|(s1, s2, s3)| ... )` and it will just work! But keep in mind that extra iterator elements that do not fit into this structure are just ignored.
`split_once(delimiter)` on string | [4](./2022/day04/part1/src/main.rs) | This function returns a a tuple, so it was perfect for splitting a string on a delimiter once.
`windows(usize)` on vector | [6](./2022/day06/part1/src/main.rs) | No, not the OS. This example explains what this function does well: [a,b,c].windows(2) turns into [[a,b], [b,c]]. It creats all possible "windows" of size 2 in from the vector. Very useful for that days task.
`.inspect(\|v\| { dbg!(v); })` on iterator | [6](./2022/day06/part2/src/main.rs) | This is how you debug (i.e. print out all values of each element in iterator) iterators. Super useful, I wish I had known of it sooner!
