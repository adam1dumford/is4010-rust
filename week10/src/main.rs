// Week 10: Mastering ownership and borrowing
//
// This lab has two parts:
//
// PART 1 — Borrow checker puzzles (7 problems)
//   Each problem is a function that has a compile error related to ownership or
//   borrowing. Read the comment above each one, fix the bug, then uncomment the
//   call in main() to verify it runs.
//
// PART 2 — Implementation exercises (5 functions)
//   Write functions that demonstrate correct ownership/borrowing patterns.
//   The test suite at the bottom verifies your implementations.
//
// Run: cargo test

fn main() {
    println!("Week 10: Mastering ownership and borrowing");
    println!("Uncomment one problem at a time and fix it!\n");

    // All problems are now fixed and uncommented!
    problem_1();
    problem_2();
    problem_3();
    problem_4();
    problem_5();
    problem_6();
    problem_7();
}

// ============================================================================
// PROBLEM 1: Value used after move
// ============================================================================
// Fix: Pass a reference `&s1` instead of giving away ownership.
// ============================================================================

fn problem_1() {
    println!("Problem 1: Value used after move");
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Borrowing instead of moving
    println!("  The length of '{}' is {}.", s1, len);
}

#[allow(clippy::ptr_arg)]
fn calculate_length(s: &String) -> usize {
    s.len()
}

// ============================================================================
// PROBLEM 2: Immutable and mutable borrow conflict
// ============================================================================
// Fix: Ensure the immutable borrow `r1` is used and its lifetime ends
// before we create the mutable borrow `r2`.
// ============================================================================

fn problem_2() {
    println!("Problem 2: Mutable and immutable borrow conflict");
    let mut s = String::from("hello");

    let r1 = &s; // immutable borrow
    println!("  Immutable: {}", r1); // Use it so its scope ends here

    let r2 = &mut s; // mutable borrow is now safe to create
    println!("  Mutable: {}", r2);
}

// ============================================================================
// PROBLEM 3: Mutating through an immutable reference
// ============================================================================
// Fix: Make `s` mutable, pass a mutable reference `&mut s`, and accept `&mut String`.
// ============================================================================

fn problem_3() {
    println!("Problem 3: Mutating through an immutable reference");
    let mut s = String::from("hello"); // Added mut
    add_to_string(&mut s); // Pass mutable reference
    println!("  Result: {}", s);
}

fn add_to_string(s: &mut String) {
    // Accept mutable reference
    s.push_str(", world");
}

// ============================================================================
// PROBLEM 4: Multiple mutable borrows
// ============================================================================
// Fix: Wrap the first mutable borrow in its own scope so it is dropped
// before the second one is created.
// ============================================================================

fn problem_4() {
    println!("Problem 4: Multiple mutable borrows");
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("  First mutable borrow: {}", r1);
    } // r1 goes out of scope here

    let r2 = &mut s; // Now safe to create a new mutable borrow
    println!("  Second mutable borrow: {}", r2);
}

// ============================================================================
// PROBLEM 5: Dangling reference
// ============================================================================
// Fix: Change the return type to `String` so ownership is transferred to the caller.
// ============================================================================

fn problem_5() {
    println!("Problem 5: Dangling reference");
    let r = create_string();
    println!("  Got: {}", r);
}

fn create_string() -> String {
    String::from("hello") // Return the expression directly
}
// ============================================================================
// PROBLEM 6: Ownership in loops
// ============================================================================
// Fix: Pass `data` as an immutable reference so the loop doesn't consume it.
// ============================================================================

fn problem_6() {
    println!("Problem 6: Ownership in loops");
    let data = String::from("Rust");

    for i in 0..3 {
        print_with_number(&data, i); // Pass a reference
    }
}

#[allow(clippy::ptr_arg)]
fn print_with_number(s: &String, n: i32) {
    // Accept a reference
    println!("  {}: {}", n, s);
}

// ============================================================================
// PROBLEM 7: Lifetime — reference doesn't live long enough
// ============================================================================
// Fix: Move the declaration of `s` to the outer scope so it lives long enough.
// ============================================================================

fn problem_7() {
    println!("Problem 7: Lifetime extension");
    let s = String::from("inner scope"); // Moved to outer scope
    let result;
    {
        result = &s;
    }
    println!("  Result: {}", result);
}

// ============================================================================
// PART 2 — Implementation exercises
// ============================================================================

/// Takes ownership of a String, converts it to uppercase, and returns it.
pub fn to_uppercase_owned(s: String) -> String {
    s.to_uppercase()
}

/// Borrows a String immutably and returns its length.
#[allow(clippy::ptr_arg)]
pub fn string_length(s: &String) -> usize {
    s.len()
}

/// Borrows a String mutably and appends `suffix` to it in place.
pub fn append_suffix(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

/// Creates a new owned String by concatenating two borrowed string slices.
pub fn concat_strings(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

// ============================================================================
// TESTS — DO NOT MODIFY
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_uppercase_owned() {
        let s = String::from("hello");
        let result = to_uppercase_owned(s);
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_to_uppercase_owned_already_upper() {
        let s = String::from("RUST");
        assert_eq!(to_uppercase_owned(s), "RUST");
    }

    #[test]
    fn test_string_length() {
        let s = String::from("testing");
        let len = string_length(&s);
        assert_eq!(len, 7);
        // Original string must still be usable after the borrow.
        assert_eq!(s, "testing");
    }

    #[test]
    fn test_string_length_empty() {
        let s = String::from("");
        assert_eq!(string_length(&s), 0);
    }

    #[test]
    fn test_append_suffix() {
        let mut s = String::from("hello");
        append_suffix(&mut s, ", world");
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_append_suffix_empty() {
        let mut s = String::from("");
        append_suffix(&mut s, "hi");
        assert_eq!(s, "hi");
    }

    #[test]
    fn test_concat_strings() {
        let result = concat_strings("hello", " world");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_concat_strings_empty() {
        assert_eq!(concat_strings("", "abc"), "abc");
        assert_eq!(concat_strings("abc", ""), "abc");
    }
}
