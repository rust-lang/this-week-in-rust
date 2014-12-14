Title: This Week in Rust XX
Date: YYYY-MM-DD
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

# What's cooking on master?

XXX pull requests were [merged in the last week][1].

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-12-08..2014-12-15

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* Implementations of `Copy` must [now be declared
  explicitly][optin]. This is part of the [opt-in-builtin traits RFC
  (colloquially knows as OIBIT)][optin-rfc] which is supposed to
  remove some potential footguns from the typesystem.
* Most use of closures have been [converted][unboxed] to 'unboxed
  closures', the new, more flexible, closure model. As a result of API
  changes some downstream code will break, the PR has detailed
  instructions for the transition.
* Slight adjustments have been made to the `fmt` API to make them
  [safe][fmt]. These APIs are rarely used directly.
* It's now [impossible to explicitly call the `call` method][call] of
  the closure types without turning on a feature
  gate. Futureproofing. Doesn't affect the normal calling syntax.
* The `Option` and `Result` variants are [no longer reexported from
  their respective modules][reexp]. This won't break most code
  because the variants are part of the prelude.

[optin]: https://github.com/rust-lang/rust/pull/19566
[optin-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0019-opt-in-builtin-traits.md
[fmt]: https://github.com/rust-lang/rust/pull/19506
[call]: https://github.com/rust-lang/rust/pull/19587
[reexp]: https://github.com/rust-lang/rust/pull/19653
[unboxed]: https://github.com/rust-lang/rust/pull/19467

## Other Changes

* Type bounds can be [constrained by the type of an associated
  type][assoc-eq], as in `fn sum_uints<I>(iter: I) where I: Iterator,
  I::A = uint { ... }`. [RFC][assoc-eq-rfc].
* Lifetime elision [works on unboxed closure type sugar][sugar].
* The [testing guide][testing] has been overhauled.
* [`unsafe impl` and `unsafe trait` have landed][unsafe] as port of
  [OIBIT][oibit-rfc]. This is required to convert `Send` and `Sync`
  into library types.
* [`BTreeSet` implements `BitOr`, `BitAnd`, `BitXor`, and `Sub`][btreeset].
* The `recursion_limit` attribute [can control how deeply various
  algorithms in the compiler recurse][recur]. It can be used to
  convince the typechecker to keep working on particularly complicated
  types.
* [`String` implements `FromIterator<&str>` and
  `Extend<&str>`][extend], which means that iterators of `&str` can be
  collected into or appended on to a single string, e.g. `let s:
  String = vec!["foo", "bar"].collect();`, `let s = String::new();
  s.extend(vec!["foo", "bar"]);`.
* New `os::unix` and `os::windows` modules provide [platform-specific
  interop with `std::io`][io].
* The `TupleN` traits [are deprecated][tuplen] because tuple indexing
  is part of the language.

[testing]: http://doc.rust-lang.org/guide-testing.html
[sugar]: https://github.com/rust-lang/rust/pull/19589
[recur]: https://github.com/rust-lang/rust/pull/19466
[btreeset]: https://github.com/rust-lang/rust/pull/19514
[extend]: https://github.com/rust-lang/rust/pull/19626
[io]: https://github.com/rust-lang/rust/pull/19169
[assoc-eq]: https://github.com/rust-lang/rust/pull/19391
[assoc-eq-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md#constraining-associated-types
[tuplen]: https://github.com/rust-lang/rust/pull/19677
[unsafe]: https://github.com/rust-lang/rust/pull/19703
[oibit-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0019-opt-in-builtin-traits.md

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



## Upcoming Meetups

* There will be a [talk about Rust at PyCon][pycon].

[pycon]: https://us.pycon.org/2015/schedule/presentation/411/
