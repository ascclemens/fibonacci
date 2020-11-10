//! Generate Fibonacci sequence numbers.
//!
//! ```
//! # use fibonacci::Fibonacci;
//! // Collect all of the Fibonacci numbers that fit inside a u8.
//! let some_numbers: Vec<u8> = Fibonacci::default().collect();
//! assert_eq!(some_numbers, vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233]);
//! ```

#![feature(test)]

extern crate num;

#[cfg(test)]
mod test;

use num::{Zero, One, CheckedAdd};
use std::fmt::Debug;

/// A generic Fibonacci sequence generator.
///
/// The iterator ends whenever the generic type overflows from the result of adding the last two
/// numbers together. For example, a `Fibonacci<u8>` will only generate 12 elements.
///
/// ```
/// # use fibonacci::Fibonacci;
/// assert_eq!(12, Fibonacci::<u8>::default().take(13).collect::<Vec<_>>().len())
/// ```
///
/// If you need more elements than primitives can provide, try `Fibonacci<num::BigUint>`, which will
/// never overflow.
#[derive(Debug)]
pub struct Fibonacci<T> {
  last_two: (T, T)
}

impl<T> Default for Fibonacci<T>
  where T: Debug + Zero + One
{
  fn default() -> Self {
    Fibonacci { last_two: (T::zero(), T::one()) }
  }
}

impl<T> Iterator for Fibonacci<T>
  where T: Debug + CheckedAdd + Clone
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    let n = self.last_two.0.checked_add(&self.last_two.1);
    if let Some(ref x) = n {
      std::mem::swap(&mut self.last_two.0, &mut self.last_two.1);
      self.last_two.1 = x.clone();
    }
    n
  }
}

impl<T> Fibonacci<T>
  where T: Debug + Zero + One
{
  /// Reset this generator back to its state at construction.
  ///
  /// Calling this will cause the generator to continue generating numbers from the start, even if
  /// it has terminated.
  pub fn reset(&mut self) {
    self.last_two = (T::zero(), T::one());
  }
}