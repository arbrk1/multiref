//! # Multireferences (aka _the inverse distributive law_)
//!
//! Ever wanted to get `&[T]` from `[&T]`?
//!
//! Semantically, it doesn't make sense (because a slice wraps a block of 
//! _contiguous_ elements). But sometimes it's very convenient to be 
//! able to &#8220;algebraically&#8221; extract a common lifetime 
//! from a bunch of references.
//!
//! This crate provides two helper types 
//! `Slice` and [`Pair`](struct.Pair.html) 
//! that allow the following conversions:
//!
//! * `&'a [&'x T] -> &'a Slice<T>` (and a mutable equivalent)
//! * `&'a (&'x A, &'x B) -> &'a Pair<A, B>` (and a mutable equivalent)
//!
//! Moreover, each of these types provides `.as_ref()` and `.as_mut()` 
//! methods (with signatures different from the ones used by the `AsRef` and 
//! `AsMut` traits) implementing the forward distributive law:
//!
//! * `&'a Slice<T> -> &'a [&'a T]` (and a mutable equivalent)
//! * `&'a Pair<A, B> -> &'a (&'a A, &'a B)` (and a mutable equivalent)
//!
// //! Also there is a macro `declare_named_tuple!` that introduces 
// //! a user-defined helper type which allows to name 
// //! the individual wrapped references.
//!
//! ## Motivation
//!
//! _The following text is somewhat long. Unfortunately, I do not 
//! know any realistic uses of the inverse distributive law in situations 
//! not involving a formal argument in a contra-contravariant position._
//!
//! ### Preliminaries
//!
//! Suppose you have a following trait:
//!
//! ```
//! trait Info {
//!     type RelevantPart: ?Sized;
//!
//!     fn info<E, Info>(&self, extractor: E) -> Info where
//!         E: FnOnce(&Self::RelevantPart) -> Info;
//! }
//! ```
//!
//! I.e. a type implementing `Info` can temporarily give access to 
//! some its part. For example:
//!
//! ```
//! # trait Info { type RelevantPart: ?Sized; 
//! #     fn info<E, Info>(&self, extractor: E) -> Info where
//! #     E: FnOnce(&Self::RelevantPart) -> Info; }
//! # use std::collections::HashMap;
//! struct Configuration {
//!     fields: HashMap<String, String>,
//! }
//!
//! impl Info for Configuration {
//!     type RelevantPart = str;
//!     
//!     fn info<E, Info>(&self, extractor: E) -> Info where
//!         E: FnOnce(&str) -> Info 
//!     {
//!         match self.fields.get("name") {
//!             Some(name) => extractor(name),
//!             None       => extractor("UNKNOWN"),
//!         }
//!     }
//! }
//! ```
//! 
//! If you are interested whether the continuation-passing style is necessary, 
//! try to write a non-cps equivalent
//!
//! ```
//! # struct Foo; impl Foo {
//! fn info<'a>(&'a self) -> &'a str 
//! # { todo!() } }
//! ```
//!
//! for some dynamically generated string (e.g. the current timestamp) instead 
//! of static `"UNKNOWN"`.
//!
//! The only safe way to get the `&'a str` from such a string seems to be 
//! to embed this string directly in the `Configuration`. But it can't be done 
//! through a shared reference (and if it could, it would be a rather 
//! strange-looking solution, because this string has nothing to do with 
//! the configuration).
//!
//! ### The problem
//!
//! Now suppose that you want to give two fields to the extractor. 
//! What the `RelevantPart` would be?
//!
//! The laziest solution is to define
//! 
//! ```
//! type RelevantPart = (String, String);
//! ```
//!
//! But such a type requires cloning the strings. It would be better to have
//!
//! ``` ignore
//! type RelevantPart = (&'a str, &'a str);
//! ```
//!
//! but our trait doesn't have the `'a` parameter. And if it had it 
//! would not work either. E.g. a `&str` borrowed from a dynamically 
//! generated analogue of `"UNKNOWN"` must have its lifetime fully 
//! contained in the `info` method. But the `'a` lifetime is external
//! to this method.
//!
//! ### A solution
//!
//! ```
//! # trait Info { type RelevantPart: ?Sized; 
//! #     fn info<E, Info>(&self, extractor: E) -> Info where
//! #     E: FnOnce(&Self::RelevantPart) -> Info; }
//! # use std::collections::HashMap;
//! # struct Configuration { fields: HashMap<String, String> }
//! fn make_error_string() -> String { unimplemented!() }
//!
//! use multiref::Pair;
//!
//! impl Info for Configuration {
//!     type RelevantPart = Pair<str, str>;
//!     
//!     fn info<E, Info>(&self, extractor: E) -> Info where
//!         E: FnOnce(&Pair<str,str>) -> Info 
//!     {
//!         let error_string = make_error_string();
//!         // for simplicity we generate an error string unconditionally
//!
//!         let foo: &str = match self.fields.get("foo") {
//!             Some(foo) => &foo,
//!             None      => &error_string,
//!         };
//!
//!         let bar: &str = match self.fields.get("bar") {
//!             Some(bar) => &bar,
//!             None      => &error_string,
//!         };
//!
//!         extractor( (&(foo, bar)).into() )
//!     }
//! }
//! ```
//!
//!
//! ## Warning
//!
//! This crate uses some `unsafe` code. I believe it to be sound, but
//! it may not be the case.


#![no_std]

mod slice;
mod pair;
mod named_tuple;

pub use pair::Pair;

