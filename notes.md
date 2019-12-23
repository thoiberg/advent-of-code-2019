# Notes/Questions

- difference between `clone`, `cloned` & `to_owned`?
- what does `<_>` mean?
- How does `parse` work?
    - `let distance_to_move: &i32 = &movement[1..].parse().unwrap();` vs `let distance_to_move = &movement[1..].parse::<i32>().unwrap();`
- `FromIterator`
- How do I iterate down, (eg, from 10 -> 1)?
- The `#[]` syntax
- What `derive` and how it works
- How does `cargo build` know to exclude the tests from consideration?
- The `include_str` macro