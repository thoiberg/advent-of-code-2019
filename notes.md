# Notes/Questions

- difference between `clone`, `cloned` & `to_owned`?
- what does `<_>` mean?
- How does `parse` work?
    - `let distance_to_move: &i32 = &movement[1..].parse().unwrap();` vs `let distance_to_move = &movement[1..].parse::<i32>().unwrap();`
- `FromIterator`
- How do I iterate down, (eg, from 10 -> 1)?
- The `#[]` syntax
- What `derive` is and how it works
- How does `cargo build` know to exclude the tests from consideration?
- The `include_str` macro
- Calling a function directly in a closure
- Difference between `usize` and `u16`/`u32`
- passing in functions as arguments

## Lessons from PR
PR: https://github.com/thoiberg/advent-of-code-2019/pull/1

- I need to refresh my memory on best practices of when to pass by reference and when to pass by value 
- Storing intermediate values in their own variable can really reduce unnecessary computation
- I don't understand how strings map to bytes, I need to get a better understanding. If so, then maybe using `bytes` would have been a the more obvious choice when I was first writing the code.