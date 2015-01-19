Title: This Week in Rust 66
Date: 2015-01-19
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

119 pull requests were [merged in the last week][merged].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-01-12..2015-01-18

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* `AtomicInt` and `AtomicUint` have been [renamed][atomic] to
  `AtomicIsize` and `AtomicUsize` to match their corresponding integer
  types.
* To fix a bug in coherence [builtin traits can only be implemented
  for structs and enums][cohere].

[atomic]: https://github.com/rust-lang/rust/pull/20896
[cohere]: https://github.com/rust-lang/rust/pull/21167

## Other Changes

* Certain long error messages of the form 'expected foo found bar' are
  now [split neatly across multiple lines][multiline]. Examples in the
  PR.
* UFCS method calls can now be [qualified by the trait][ufcs] of the
  method.  This can be used to disambiguate method calls when multiple
  applicable methods are in scope, e.g. `<i32 as Add<_>>::add(1, 2)`
  which is equivalent to `1.add(2)`. [RFC][rfcs-rfc].
* Negative impls are [partially implemented][negimpl], though appear
  to still be special-cased to the `Send` and `Sync`
  traits. [RFC][negimpl-rfc].
* Mutexes on Windows are faster now they are [implemented with Slim
  Reader Writer Locks][mutex].
* The `#[rustc_on_unimplemented]` attribute, requiring the
  'on_unimplemented' feature, lets rustc [display custom error
  messages when a trait is expected to be implemented for a type but
  is not][onun].
* [Preliminary support for PowerPC][powerpc].

[multiline]: https://github.com/rust-lang/rust/pull/19870
[mutex]: https://github.com/rust-lang/rust/pull/20367
[onun]: https://github.com/rust-lang/rust/pull/20889
[negimpl]: https://github.com/rust-lang/rust/pull/20972
[negimpl-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0019-opt-in-builtin-traits.md
[powerpc]: https://github.com/rust-lang/rust/pull/20980
[ufcs]: https://github.com/rust-lang/rust/pull/21077
[ufcs-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0132-ufcs.md

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


