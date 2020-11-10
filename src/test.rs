use num::BigUint;

use crate::Fibonacci;

#[test]
fn first_ten() {
  let ten: Vec<u8> = Fibonacci::default().take(10).collect();
  assert_eq!(ten, vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
}

#[test]
fn overflow() {
  assert_eq!(92, Fibonacci::<u64>::default().take(93).collect::<Vec<_>>().len());
}

#[test]
fn non_overflow() {
  assert_eq!(300, Fibonacci::<BigUint>::default().take(300).collect::<Vec<_>>().len());
}

#[test]
fn reset() {
  let mut fib: Fibonacci<u8> = Fibonacci::default();
  for _ in 0..12 {
    fib.next();
  }
  assert_eq!(None, fib.next());
  fib.reset();
  assert_eq!(Some(1), fib.next());
}