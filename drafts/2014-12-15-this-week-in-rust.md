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

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-12-01..2014-12-08

## Breaking Changes

* Implementations of `Copy` must [now be declared
  explicitly][optin]. This is part of the [opt-in-builtin traits RFC
  (colloquially knows as OIBIT)][optin-rfc] which is supposed to
  remove some potential footguns from the typesystem.
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

## Other Changes

* Lifetime elision [works on unboxed closure type sugar][sugar].
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

[sugar]: https://github.com/rust-lang/rust/pull/19589
[recur]: https://github.com/rust-lang/rust/pull/19466
[btreeset]: https://github.com/rust-lang/rust/pull/19514
[extend]: https://github.com/rust-lang/rust/pull/19626

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


