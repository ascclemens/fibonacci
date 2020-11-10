# Fibonacci sequence generator in Rustlang

A Fibonacci generator written in Rust. It's generic for types that implement `num::Zero`,`num::One`, `num::CheckedAdd`, and `Clone`.

There is no stipulation that the type be `num::Unsigned`, but the generator will never generate a negative number.

The generator handles overflow by returning `None`, stopping the iterator. For example, `Fibonacci<u64>` will overflow after the ninety-second element on standard amd64 system, so the iterator will end after producing its ninety-second element.

The following all hold true:
```rust
assert_eq!(12, Fibonacci::<u8>::default().take(300).collect::<Vec<_>>().len());
assert_eq!(23, Fibonacci::<u16>::default().take(300).collect::<Vec<_>>().len());
assert_eq!(46, Fibonacci::<u32>::default().take(300).collect::<Vec<_>>().len());
assert_eq!(92, Fibonacci::<u64>::default().take(300).collect::<Vec<_>>().len());
assert_eq!(184, Fibonacci::<u128>::default().take(300).collect::<Vec<_>>().len());
```