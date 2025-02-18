# Rust Iterator Exhaustion Example

This repository contains a simple example demonstrating a common error in Rust: panicking due to iterator exhaustion.

The `bug.rs` file shows the erroneous code which uses `iter.next().unwrap()` after the iterator has been exhausted. The `bugSolution.rs` file provides a corrected version which handles the potential for `None` by using pattern matching.

This example highlights the importance of checking for the end of iterators in Rust to prevent runtime panics.