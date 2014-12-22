Title: This Week in Rust 62
Date: 2014-12-22
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

This Week in Rust is openly developed [on Github](https://github.com/cmr/this-week-in-rust).
If you find any errors or omissions in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# What's cooking on master?

105 pull requests were [merged in the last week][1].

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-12-15..2014-12-22

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* Macros using parens and square brackets (`macro!()`, macro![]`) are
  [parsed as expressions][mac] if not followed by a semicolon. This
  makes expressions like `vec![1i, 2, 3].len();` work as
  expected. [RFC][mac-rfc].
* [Binops take their arguments by value][binops], including `Add`,
  `Sub`, `Mul`, `Div` `Rem`, `BitAnd`, `BitOr`, `BitXor`, `Shl`,
  `Shr`. This breaks all existing implementations!  `String` and `Vec`
  addition now reuse the LHS buffer. [RFC][binops-rfc].
* The `Neg` and `Not` unary ops [also take their arguments by
  value][unops].
* [A number of changes to the runtime appeared][rt], including
  renaming `std::task` to `std::thread`, that maps more directly to OS
  threads.
* The in-tree `getopts`, `log`, `regex`, and `regex_macros` crates are
  [deprecated][crates] in favor of the ones from `crates.io`. Nearly
  everything that isn't std is moving to the registry.
* Command-line flags to `rustc` are [being adjusted][rustcflags] with
  an eye towards stabilizing and futureproofing. See PR for details.
* `TreeMap`, `TreeSet`, `TrieMap`, `TrieSet`, `LruCache` and `EnumSet`
  have all been [removed from the `collections` crate][rmcoll], and
  can for now live in [collect-rs]. [RFC][rmcoll-rfc].
* The `VecMap` iterators are now [newtypes instead of type
  aliases][vecmap] to encapsulate the implementation.
* The `BTreeMap`, `BTreeSet`, `HashMap`, and `HashSet` iterators are
  [also newtypes][morenewtypes]..
* [`is_power_of_two` no longer incorrectly considers 0 to be a power
  of 2][two].
* `regex::Captures::at` and `Captures::name` [return `Option`][regex].

[binops]: https://github.com/rust-lang/rust/pull/19448
[binops-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0439-cmp-ops-reform.md
[unops]: https://github.com/rust-lang/rust/pull/19899
[rt]: https://github.com/rust-lang/rust/pull/19654
[two]: https://github.com/rust-lang/rust/pull/19640
[vecmap]: https://github.com/rust-lang/rust/pull/19720
[morenewtypes]: https://github.com/rust-lang/rust/pull/19770
[regex]: https://github.com/rust-lang/rust/pull/19818
[crates]: https://github.com/rust-lang/rust/pull/19820
[rustcflags]: https://github.com/rust-lang/rust/pull/19900
[rmcoll]: https://github.com/rust-lang/rust/pull/19955
[collect-rs]: https://github.com/Gankro/collect-rs/
[rmcoll-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0509-collections-reform-part-2.md
[mac]: https://github.com/rust-lang/rust/pull/19984
[mac-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0378-expr-macros.md

## Other Changes

* Backtraces are [demangled correctly on Windows][bt].
* `RingBuf` now exposes its buffers via the [`as_slices`
  method][as_slices].
* A number of collections implement [a new method called
  `drain`][drain] which removes all members of the collection without
  deallocating the underlying buffers.
* Work is progressing on [generalized where clauses][where] which is
  necessary for associated types to be fully useful. [RFC][where-rfc].

[bt]: https://github.com/rust-lang/rust/pull/19819
[as_slices]: https://github.com/rust-lang/rust/pull/19903
[drain]: https://github.com/rust-lang/rust/pull/19946
[where]: https://github.com/rust-lang/rust/pull/20073
[where-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0135-where.md

## New Contributors



# Approved RFC's



# New RFC's



# Community

## From the Team

* [Weekly-meetings/2014-18-11][mtg]:  [Reddit][mtg-reddit].

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2014-18-11.md
[mtg-reddit]:



## Blog Posts



## Discussions



## New Projects



## Project Updates



## Upcoming Events


