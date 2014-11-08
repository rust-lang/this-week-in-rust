Title: This Week in Rust 56
Date: 2014-11-10
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

# What's cooking on master?

TODO: Update this Monday morning

XXX pull requests were [merged in the last week][1]. Woo!

[1]: https://github.com/rust-lang/rust/pulls?page=1&q=is%3Apr+is%3Amerged+updated%3A%3E%3D2014-11-03

## Breaking Changes

TODO: Link to complete breaking changes log

* [Flexible target specification][flex] has finally landed. This makes
  it much easier to create custom toolchains for unsupported
  platforms. [RFC][flex-rfc].
* [Error interoperation][err] improves the ergonomics of error
  handling when multiple error types are involved. [RFC][err-rfc].
* There has been a minor breaking change to the [serialization of
  tuples][tup].
* Minor [changes to macro interpolation][mac] have resulted the
  removal the `$foo:matchers` type of `macro_rules!` argument.
* Socket construction is now more flexibly done through a
  [`ToSocketAddr`] type.
* Some changes have been made to the [`BytesContainer`], which is used
  to construct `Path`s, causing breakage is some cases.
* The comparision types have been [updated for DST][cmp], resulting in
  changes to how they are invoked for references to unsized types
  (i.e. `&str` and `&[T]`).

[flex]: https://github.com/rust-lang/rust/pull/16156
[flex-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0131-target-specification.md
[err]: https://github.com/rust-lang/rust/pull/17753
[err-rfc]: https://github.com/rust-lang/rfcs/blob/master/active/0070-error-chaining.md
[tup]: https://github.com/rust-lang/rust/pull/17595
[mac]: https://github.com/rust-lang/rust/pull/17830
[`ToSocketAddr`]: https://github.com/rust-lang/rust/pull/18462
[`BytesContainer`]: https://github.com/rust-lang/rust/pull/18463
[cmp]: https://github.com/rust-lang/rust/pull/18467

## Other Changes

* New [blanket impls] of the unboxed closure types allow them to
  interoperate.  See
  [test](https://github.com/rust-lang/rust/blob/master/src/test/run-pass/unboxed-closures-fn-as-fnmut-and-fnonce.rs)
  [cases])https://github.com/rust-lang/rust/blob/master/src/test/run-pass/unboxed-closures-fnmut-as-fnonce.rs)
  for examples.
* impls can now be [defined on trait objects][impltrait].
* P1start has been [converting][help] compiler messages that provide
  suggestions from 'notes' to 'help' messages.
* The ['exceeding_bitshifts'][bitshift] lint (deny by default) catches
  overlong shifts (which are currently undefined behavior) of static
  size.
* Ariel [removed][unsafe-rustc] a bunch of unsafe code from the compiler. Yay!

[blanket impls]: https://github.com/rust-lang/rust/pull/18388
[impltrait]: https://github.com/rust-lang/rust/pull/18447
[help]: https://github.com/rust-lang/rust/pull/18132
[bitshift]: https://github.com/rust-lang/rust/pull/18206
[unsafe-rustc]: https://github.com/rust-lang/rust/pull/18318



## Approved RFC's
* [Num reform](https://github.com/rust-lang/rfcs/blob/master/text/0418-struct-variants.md): Strips down `std::num` to minimally support generic primitive numbers, without supporting a full mathematical hierarchy.

* [Higher-ranked trait bounds](https://github.com/rust-lang/rfcs/blob/master/text/0387-higher-ranked-trait-bounds.md): Add the ability to have trait bounds that are polymorphic over lifetimes. Necessary for unboxed closures.

* [un-feature-gating struct variants](https://github.com/rust-lang/rfcs/blob/master/text/0418-struct-variants.md): Woo!

* [Multiple lifetime bounds](https://github.com/rust-lang/rfcs/blob/master/text/0192-bounds-on-object-and-generic-types.md): Removes special cases from the type system and makes more complex lifetime relationships be expressed that were previously only inferable.



## New RFC's
* [Macro reform](https://github.com/rust-lang/rfcs/pull/453): Prepares macros for 1.0 stabilization. Renames `macro_rules!` to `macro!`, and introduces more robust support for module importing and exporting.

* [Change integer fallback RFC to suggest `i32` instead of `int` as the fallback](https://github.com/rust-lang/rfcs/pull/452): Changes the fallback for performance and portability. 

* [Un-feature-gate if let and tuple indexing](https://github.com/rust-lang/rfcs/pull/450): The features are well-behaved and used by many projects; ship 'em!

* [Prohibit unused type parameters in impls](https://github.com/rust-lang/rfcs/pull/453): Require that every impl type parameter appears textually within the input type parameters of the trait reference or the impl self type.

* [ES6-style unicode string escaping](https://github.com/rust-lang/rfcs/pull/446): Remove `\u203D` and `\U0001F4A9` unicode string escapes, and add ECMAScript 6-style `\u{1F4A9}` escapes instead. Strong positive feedback, some concern with how it interacts with format strings.

* [extension trait conventions](https://github.com/rust-lang/rfcs/pull/445): Establishes a definition and naming convention for extension traits: traits which aren't intended for generic programing, but instead extending existing types. If extending a `Foo`, use `FooExt`. If Extending a `Foo`'s when they impl another trait like `Add`, use `FooAddExt`. 

* [cmp and ops reform](https://github.com/rust-lang/rfcs/pull/439): Refactors `Cmp` and the operator overloading traits. Generally positive feedback. Highlights include:
  * Make basic unary and binary operators work by value and use associated types.
  * Generalize comparison operators to work across different types; drop Equiv.
  * Refactor slice notation in favor of range notation so that special traits are no longer needed.
  * Add IndexSet to better support maps.
  * Clarify ownership semantics throughout.

* [Change precedence of `+` in type grammar](https://github.com/rust-lang/rfcs/pull/438): Update type grammar to make `+` have lower precedence, consistent with the expression grammar, resolving a grammatical ambiguity.

* [Relocate and improve c_str](https://github.com/rust-lang/rfcs/pull/435): 
  * Move the c_str module out of std to rid the latter of type dependencies on libc.
  * Split the current CString into a low-level type CStrBuf and a length-aware CString to make computation costs explicit.
  * Provide custom destructors and purpose-specific, mnemonically named constructors.
  * Add some methods and trait implementations to make the types more useful.
  * Remove the Clone implementation due to lack of purpose.
Lots of discussion of how to structure libc, not a lot of consensus.

* [rename `lifetime` to `scope`](https://github.com/rust-lang/rfcs/pull/431): Highly controversial. Some community members argue that this change in terminology has been much more effective when introducing the actual concepts to newbies. Others argue that scope is already a well established concept in programming languages.

* [Finalizing more naming conventions](https://github.com/rust-lang/rfcs/pull/430): finalizes a few long-running de facto conventions, including capitalization/underscores, and the role of the unwrap method. Generally positive feedback, some discussion of naming consts like enum variants.



## Community

Ideas: blog posts, videos, reddit/hn/discuss threads, new projects, project updates
TODO: Summarize

https://www.reddit.com/r/rust/comments/2l98pn/error_interoperation_now_available_in_the/

## New Contributors
