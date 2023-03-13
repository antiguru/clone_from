Derive Clone including `clone_from`
===================================

This crate offers a derive macro called `CloneFrom` to derive `Clone` with a specialized `clone_from` implementation.

When deriving `Clone`, Rust omits a specialized `clone_from` implementation and defers to `clone` instead.
This means that it can behave counter-intuitively, for example when it should reuse allocations, but does not.
This crate aims to be a drop-in replacement for deriving `Clone`, but with a proper `clone_from` implementation.

## Example

```rust
#[derive(CloneFrom)]
struct MyStruct<T> {
    name: String,
    inner: Vec<T>,
}
```

## Considerations

Adding a `clone_from` implementation to all types in Rust seems to add a significant compile-time cost, which is why Rust does not provide this by default.
Many types do not benefit from a specialized `clone_from` implementation, so it's important to add it when it brings a benefit.

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
