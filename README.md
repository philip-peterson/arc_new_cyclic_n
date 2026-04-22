# arc_new_cyclic_n

Variants of [`Arc::new_cyclic`](https://doc.rust-lang.org/std/sync/struct.Arc.html#method.new_cyclic) that accept more than one value at a time.

Useful when constructing multiple `Arc`s that hold weak references to each other. This crate provides `arc::new_cyclic_2` and `arc::new_cyclic_3` — nothing more, nothing less.

## Usage

```rust
use arc_new_cyclic_n::arc;

let (a, b) = arc::new_cyclic_2(|weak_a, weak_b| {
    (MyStruct { other: weak_b.clone() }, MyStruct { other: weak_a.clone() })
});
```

## License

MIT
