# permut_lib
This crate provide simple function to generate permutations on a dictionry of a u8 chars set.
I used it for generate unic Items into my proc macros

# Functions
```rust
pub fn create_permut(chars: &[u8], count: usize) -> Vec<Vec<u8>>
```
Universal function for generate unic sequences of a u8 chars, like "A", "B", "C", ... , "AA", "AB", ..., "AAAAAB",...

# Examples
```rust
use permut_lib::permut::*;
create_permut(CHARS_CUPS_LATIN, 195); // result: "A", "B", "C", ... , "AA", "AB" ...  
```
