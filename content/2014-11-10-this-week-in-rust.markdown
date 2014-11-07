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

## New RFC's

## Community

Ideas: blog posts, videos, reddit/hn/discuss threads, new projects, project updates
TODO: Summarize

https://www.reddit.com/r/rust/comments/2l98pn/error_interoperation_now_available_in_the/

## New Contributors
