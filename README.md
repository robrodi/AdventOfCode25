<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code {year}

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2025 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2025/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2025/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2025/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2025/day/4) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2025/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2025/day/6) | ⭐ | ⭐ |
| [Day 7](https://adventofcode.com/2025/day/7) | ⭐ | ⭐ |
| [Day 8](https://adventofcode.com/2025/day/8) | ⭐ | ⭐ |
| [Day 9](https://adventofcode.com/2025/day/9) | ⭐ |   |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](./src/bin/01.rs) | `114.2µs` | `119.1µs` |
| [Day 2](./src/bin/02.rs) | `137.7ms` | `110.4ms` |
| [Day 3](./src/bin/03.rs) | `32.3µs` | `101.8µs` |
| [Day 4](./src/bin/04.rs) | `1.0ms` | `29.4ms` |
| [Day 5](./src/bin/05.rs) | `1.0ms` | `957.7µs` |
| [Day 6](./src/bin/06.rs) | `61.0µs` | `281.3µs` |
| [Day 7](./src/bin/07.rs) | `2.9ms` | `-` |

**Total: 284.07ms**
<!--- benchmarking table --->


## Usage

### ➡️ Scaffold a day

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>
```

Individual solutions live in the `./src/bin/` directory as separate binaries. _Inputs_ and _examples_ live in the the `./data` directory.

Every [solution](https://github.com/fspoettel/advent-of-code-rust/blob/main/src/template.txt) has _tests_ referencing its _example_ file in `./data/examples`. Use these tests to develop and debug your solutions against the example input. In VS Code, `rust-analyzer` will display buttons for running / debugging these unit tests above the unit test blocks.

> [!TIP]
> If a day has multiple example inputs, you can use the `read_file_part()` helper in your tests instead of `read_file()`. If this e.g. applies to day 1, you can create a second example file `01-2.txt` and invoke the helper like `let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));`. This supports an arbitrary number of example files.

### ➡️ Download input for a day

```sh
# example: `cargo download 1`
cargo download <day>
```

### ➡️ Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>
```

#### Submitting solutions
Append the `--submit <part>` option to the `solve` command to submit your solution for checking.

### ➡️ Run all solutions
This runs all solutions sequentially and prints output to the command-line. Same as for the `solve` command, the `--release` flag runs an optimized build.

```sh
cargo all
```


### ➡️ Benchmark your solutions
```sh
# example: `cargo time 8 --store`
cargo time <day> [--all] [--store]
```

### ➡️ Run all tests
To run tests for a specific day, append `--bin <day>`, e.g. `cargo test --bin 01`. You can further scope it down to a specific part, e.g. `cargo test --bin 01 part_one`.

```sh
cargo test
```

### ➡️ Format code

```sh
cargo fmt
```

### ➡️ Lint code

```sh
cargo clippy
```

## Optional template features

### Use DHAT to profile heap allocations

If you are not only interested in the runtime of your solution, but also its memory allocation profile, you can use the template's [DHAT](https://valgrind.org/docs/manual/dh-manual.html) integration to analyze it. In order to activate DHAT, call the `solve` command with the `--dhat` flag.

```sh
cargo solve 1 --dhat

# output:
#     Running `target/dhat/1`
# dhat: Total:     276 bytes in 3 blocks
# dhat: At t-gmax: 232 bytes in 2 blocks
# dhat: At t-end:  0 bytes in 0 blocks
# dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
# Part 1: 9001 (4.1ms)
```

The command will output some basic stats to the command-line and generate a `dhat-heap.json` report in the repo root directory.

You can pass the report a tool like [dh-view](https://nnethercote.github.io/dh_view/dh_view.html) to view a detailed breakdown of heap allocations.

## Useful crates

-   [itertools](https://crates.io/crates/itertools): Extends iterators with extra methods and adaptors. Frequently useful for aoc puzzles.
-   [regex](https://crates.io/crates/regex): Official regular expressions implementation for Rust.

A curated list of popular crates can be found on [blessed.rs](https://blessed.rs/crates).

Do you have aoc-specific crate recommendations? [Share them!](https://github.com/fspoettel/advent-of-code-rust/edit/main/README.md)

## Footnotes

[^1]: The session cookie might expire after a while (~1 month) which causes the downloads to fail. To fix this issue, refresh the `.adventofcode.session` file.
[^2]: The session cookie might expire after a while (~1 month) which causes the automated workflow to fail. To fix this issue, refresh the AOC_SESSION secret.
[^3]:
    <img src="https://user-images.githubusercontent.com/1682504/198838369-453dc22c-c645-4803-afe0-fc50d5a3f00c.png" alt="Set a breakpoint" width="450" />

[^4]:
    <img alt="Run debugger" src="https://user-images.githubusercontent.com/1682504/198838372-c89369f6-0d05-462e-a4c7-8cd97b0912e6.png" width="450" />

[^5]:
    <img alt="Inspect debugger state" src="https://user-images.githubusercontent.com/1682504/198838373-36df6996-23bf-4757-9335-0bc4c1db0276.png" width="450" />
