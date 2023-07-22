#![warn(missing_docs)]

//! # Example Module
//! 
//! This module serves to demonstrate Rust's documentation system. It doesn't do anything useful.
//!
//! ## Examples
//! 
//! ```
//! use documented::example_function;
//! example_function();
//! ```

/// This is an example function. It prints a message including the value
/// of `n`, and returns `n * 2`.
/// 
/// # Arguments
/// 
/// * `n` - The number to multiply by 2
/// 
/// # Returns
/// 
/// The result of `n * 2`
/// 
/// # Examples
/// 
/// ```
/// assert_eq!(documented::example_function(2), 4);
/// ```
pub fn example_function(n: i32) -> i32 {
    println!("This is an example function. n = {n}");
    n * 2
}

/// Example of an unsafe function
/// 
/// # Safety
/// 
/// This function uses `get_unchecked` for fast access to a vector. This is ok, because the 
/// bounds of the vector are known ahead of time.
pub fn example_unsafe() -> i32 {
    let n = vec![1, 2, 3, 4, 5];
    unsafe {
        *n.get_unchecked(3)
    }
}

/// An example of a panicking function
/// 
/// # Panics
/// 
/// This function panics if the option is `None`.
pub fn panic_example(n: Option<i32>) -> i32 {
    n.unwrap() * 2
}

/// Wraps [`example_function`].
/// 
/// [`example_function`]: #documented.example_function
pub fn example_function_wrapper(n: i32) -> i32 {
    example_function(n)
}

pub mod frobnicator {
    //! The Frobnicator!

    /// Frobnicates the input
    pub fn do_it() {
        println!("Frobnicating!");
    }
}
