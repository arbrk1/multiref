[![crate](https://img.shields.io/crates/v/multiref)](https://crates.io/crates/multiref/)
[![docs](https://docs.rs/multiref/badge.svg)](https://docs.rs/multiref/)

# Multireferences for Rust

Allowing to pass from `[&T]` to (an analogue of) `&[T]` safely 
without copying anything. 

## More precisely

This crate provides two helper types `Slice` and `Pair` 
that allow the following conversions:

### The inverse distributive law

* `&'a [&'x T] -> &'a Slice<T>` (and a mutable equivalent)
* `&'a (&'x A, &'x B) -> &'a Pair<A, B>` (and a mutable equivalent)

### The forward distributive law

* `&'a Slice<T> -> &'a [&'a T]` (and a mutable equivalent)
* `&'a Pair<A, B> -> &'a (&'a A, &'a B)` (and a mutable equivalent)

## What for?

To move lifetimes from the depths of a type expression closer to its head.

Such a manipulation can be useful in a cps-heavy code.

## Details

[HERE](https://docs.rs/multiref/)

## Usage

Simply include 

```
multiref = "0.1"
```

in your `Cargo.toml`.

The crate doesn't use any of the `std` library and has 
the `#![no_std]` attribute.


## Similiar crates

There is a similiarly named crate 
[multi_mut](https://crates.io/crates/multi_mut/). 
It allows to extract multiple nonintersecting mutable references 
from the `std::collections` maps.

However it has nothing to do with the distributive laws.


## Versions

* `0.1.2`: fixed a soundness issue with pairs
* `0.1.1`: `modify` methods for pairs and &#8220;slices&#8221;.
* `0.1.0`: Initial version.

