![header](https://capsule-render.vercel.app/api?type=waving&height=300&color=gradient&text=checktrait)

Simple tool for compile-time check trait

## Installation

```bash
cargo add --dev checktrait
```

## Usage

Check implementation with `checktrait::*` macros.
The macro will fail to compile if the object/type does not implement the trait.

```rust
checktrait::ty!(MyTrait, MyType);
checktrait::obj!(MyTrait, MyObj);
```
