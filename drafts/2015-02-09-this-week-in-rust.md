Title: This Week in Rust 69
Date: 2015-02-09
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/wiki/Note-guide-for-new-contributors).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors or omissions in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# What's cooking on master?

99 pull requests were [merged in the last week][merged], and 11 [RFCs][rfcs].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-02-02..2015-02-08
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-02-02..2015-02-08

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* `std::path` [has been rewritten][path] to improve ergonomics and
  better support platform-spcific features. The old path module still
  exists as `std::old_path` and remains exported by the prelude (for
  now). [RFC][path-rfc].
* [`std::env`][env] has been added to the standard library as an
  overhaul of the existing `std::os` module, which is now
  deprecated. Part of the almighty [RFC 517][env-rfc].
* And also we've got a [new `std::io` module][io], again part of
  the [mother of RFCs][io-rfc].
* The explicit [closure kind syntax][close] (`|&:|`, `|&mut:|`, `|:|`)
  is obsolete and closure kind is inferred from context.
* In order to make drop semantics optimizable it is no longer possible
  to [move into uninitialized arrays or out of fixed sized
  arrays][array]. [RFC][array-rfc].
* The `#![no_std]` attribute that allows for operation without the
  standard library has [been placed behind the `no_std` feature
  gate][no_std].
* The scope if iterator expressions has been [narrowed][scope] in a
  way that breaks minor corner-cases.
* The deprecated `MaybeOwnedVector` type [has been removed][maybe].

[path]: https://github.com/rust-lang/rust/pull/21759
[path-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0474-path-reform.md
[env]: https://github.com/rust-lang/rust/pull/21787
[env-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0517-io-os-reform.md#stdenv
[io]: https://github.com/rust-lang/rust/pull/21835
[io-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0517-io-os-reform.md
[close]: https://github.com/rust-lang/rust/pull/21843
[array]: https://github.com/rust-lang/rust/pull/21971
[array-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0533-no-array-elem-moves.md
[scope]: https://github.com/rust-lang/rust/pull/21984
[no_std]: https://github.com/rust-lang/rust/pull/21988
[maybe]: https://github.com/rust-lang/rust/pull/22009

## Other Changes

* The `boxed::into_raw` and `Box::frow_raw` functions [convert between
  `Box<T>` and `*mut T`][boxraw], a common pattern for creating raw
  pointers.
* Initial [support for OpenBSD][openbsd] and [BitRig][bitrig], an OpenBSD fork,
  appeared this week, from Sébastien Marie and Dave Huseby respectively.

[boxraw]: https://github.com/rust-lang/rust/pull/21318
[openbsd]: https://github.com/rust-lang/rust/pull/21754
[bitrig]: https://github.com/rust-lang/rust/pull/21959

## New Contributors



# Approved RFC's



# New RFC's



# Community


## Announcements

* [Weekly-meetings/2014-18-11][mtg]: what? [Reddit][mtg-reddit].

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2014-18-11.md
[mtg-reddit]:


## Blog Posts



## Discussions



## Videos



## New Projects



## Project Updates



## Upcoming Events


