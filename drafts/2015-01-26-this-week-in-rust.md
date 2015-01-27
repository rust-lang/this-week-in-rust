Title: This Week in Rust 67
Date: 2015-01-26
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

143 pull requests were [merged in the last week][merged], and XXX [RFCs][rfcs].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-01-19..2015-01-25
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-01-19..2015-01-25

Flavio, Steve and Alex all made rollups. Thanks!

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* The `Show` and `String` formatting traits [have been renamed][fmt]
  to `Debug` and `Display` to more clearly reflect their related
  purposes. Automatically getting a string conversion to use with
  `format!{":?}")` is now written `#[derive(Debug)]`.
* Both the `#[start]` and `#[main]` attributes are [feature
  gated][gatemain] as a precaution. Use `#![feature(start)]` and
  `#![feature(main)]` to get them back.

[fmt]: https://github.com/rust-lang/rust/pull/21457
[gatemain]: https://github.com/rust-lang/rust/pull/21257

## Other Changes

* Abstract [OS-specific string types][osstr], `std::ff::{OsString,
  OsStr}`, provide strings in platform-specific encodings for easier
  interop with system APIs. [RFC][osstr-rfc].
* `extern crate` and `use` [no longer have to be written only at the
  top of a module][viewitems], but can be intermixed with other item
  definitions.
* Brian Leibig [added his LALR grammar][lalr], which parses almost all
  the Rust files that rustc can.
* The (oft-neglegcted) grammar from the manual was [extracted to its
  own file][grammar].
* The [`unconditional_recursion`][recur] lint detects basic
  infinite recursion scenarios that are probably not intended.

[osstr]: https://github.com/rust-lang/rust/pull/21488
[osstr-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0517-io-os-reform.md
[viewitems]: https://github.com/rust-lang/rust/pull/20179
[grammer]: https://github.com/rust-lang/rust/pull/19353
[recur]: https://github.com/rust-lang/rust/pull/20373
[lalr]: https://github.com/rust-lang/rust/pull/21452

## New Contributors

* Adam Roben
* Alexis
* Barosl LEE
* blackbeam
* Chris Thorn
* Daniel Griffen
* Daniel Raloff
* Eunji Jeong
* Flavio Percoco Premoli
* GuillaumeGomez
* Ignacio Corderi
* Jay True
* JP Sugarbroad
* KernelJ
* Kim Røen
* Logan Chien
* Luke Francl
* Michael Pankov
* Ryan Levick
* Sean Patrick Santos
* Steven Allen
* Tim Parenti
* Toby Scrace
* Tristan Storch
* visualfc
* Wangshan Lu
* Willson Mock

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


