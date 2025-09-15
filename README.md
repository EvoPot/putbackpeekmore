# About

A no_std Rust iterator for doing useful things such as putting back, and peeking more. It uses no clone or copy.

## How to use

```rust
use putbackpeekmore::PutBackPeekMore;

fn main() {
    // Create a new iterator :
    let mut iter: PutBackPeekMore<_, 7> = PutBackPeekMore::new(0..10); // The 7 is the "peek buffer size". Keep in mind that if you make this value too small it will result in garbage data.

    // Look at the next value of the iterator
    assert_eq!(iter.peek(), &Some(0));

    // Consume the iterator
    assert_eq!(iter.next(), Some(0));

    //Peek a certain amount
    assert_eq!(iter.peek_value(3), &[Some(1), Some(2), Some(3)]);

    // Put back a value
    iter.put_back(Some(0));
    assert_eq!(iter.next(), Some(0));
}
```
