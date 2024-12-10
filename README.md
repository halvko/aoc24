# Template for solving Advent of Code puzzles in Rust

## Usage

1. Create a new project from the template repository:
   - Using GitHub’s templating feature: Simply click the Use this template
     [button](https://github.com/new?template_name=advent-of-code-rust-template&template_owner=bravit)
     on the repository page, create a new repository, and then open it in your editor of choice.

2. Make sure you have a working Rust installation.
   - In a terminal (cmd, or what you have on your system), try running `cargo --version`. If this
     prints something which looks like a version number, you are all set! Yay!
   - If the above didn't work, I recommend going to [https://rustup.rs](rustup.rs), which will give
     you a method for installing rustup (a Rust toolchain manager) for your system (If this does
     not apply to you, you know who you are and you don't need my help)
      - If you are new to Rust, I strongly recommend to get in the habit of actually reading any
        and all error messages! The Rust Project and much of the Rust ecosystem developers care a
        lot about having helpfull errors, they often tell you exactly how to fix the problem.

2. Whenever you're ready to start solving a new day's puzzle:
   - Open the `bin` folder, copy and paste the `NN.rs` file into it, and give it the corresponding
     name (`01.rs`, `02.rs`, etc.).
   - In the `input` folder, create and fill the input data file (`01.txt`, `02.txt`, etc.).
   - Fill in the `DAY` constant in the freshly created file.
   - Run the current day's solution to check if it compiles (you can use the gutter icon next to
     the `main` function).
   - Fill in `<TEST-INPUT>`.
   - Write the expected answer for the test data in the `assert_eq` statement in *Part 1*.
   - Now you're ready to write your solution in the `part1` function (inside `main`).
   - Open a terminal in the folder, and run `cargo run --bin 01` for running the `main` function in
     the `bin/01.rs` file.
     - If you need more performance, you can do `cargo run --bin 01 --release`

3. When you're done with the first part of the puzzle, use folding to hide *Part 1*.

4. Uncomment *Part 2*, fill in the test data assertion, and start solving it.
