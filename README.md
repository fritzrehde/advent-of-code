# My [Advent of Code](https://adventofcode.com/) solutions

AoC year | Language | Year completed
:-- | :-- | :--
2021 | Python | 2024
2022 | Rust | 2022
2023 | Rust | 2023
2024 | Python | 2024

## Notes

### 2022

Here are some (quite random) things I have learned about Rust through solving the 2022 Advent of Code puzzles.

Feature/function | Day | Description
:-- | :-- | :--
[`Iterator::filter_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map) | [3](./2022/day03/part1/src/main.rs) | Instead of filtering and then mapping, you just return an optional and all None values are automatically filtered out
[`Itertools::tuples()`](https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.tuples) | [3](./2022/day03/part2/src/main.rs) | Using this seemed like magic to me: you can transform an iterator like `[s1, s2, ...]` into `[(s1, s2, s3), (s4, s5, s6), ...]` just by adding `.tuples()` before a `.map(\|(s1, s2, s3)\| ... )` and it will just work! But keep in mind that extra iterator elements that do not fit into this structure are just ignored.
[`str::split_once(&self, delimiter)`](https://doc.rust-lang.org/std/primitive.str.html#method.split_once) | [4](./2022/day04/part1/src/main.rs) | This function returns a a tuple, so it was perfect for splitting a string on a delimiter once.
[`Vec::windows(&self, usize)`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.windows) | [6](./2022/day06/part1/src/main.rs) | No, not the OS. This example explains what this function does well: `[a,b,c].windows(2)` turns into `[[a,b], [b,c]]`. It creates all possible "windows" of size 2 in from the vector. Very useful for that days task.
[`Iterator::inspect(\|v\| { dbg!(v); })`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.inspect) | [6](./2022/day06/part2/src/main.rs) | This is how you debug (i.e. print out all values of each element in iterator) iterators. Super useful, I wish I had known of it sooner!
