Title: This Week in Rust 60
Date: 2014-12-08
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

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-12-01..2014-12-07

## Breaking Changes

* The [definitions of operators have changed][ops] to be more
  flexible. [RFC][ops-rfc].
* `std::sync` has been [redesigned and rewritten][sync] for the nth
  time as a result of the ongoing runtime decimation.
* `HashMap` [no longer shrinks automatically][shrink], and some
  methods for managing the capacity have changed.

[shrink]: https://github.com/rust-lang/rust/pull/18770
[ops]: https://github.com/rust-lang/rust/pull/19167
[ops-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0439-cmp-ops-reform.md
[sync]: https://github.com/rust-lang/rust/pull/19274

## Other Changes

* `if let`, `while let`, and tuple indexing are [no longer
  feature-gated][ungate]. [RFC][ungate-rfc].
* There's [a new syntax for escaping unicode characters][es6]. The old
  will be deprecated briefly. [RFC][es6-rfc].
* [`&[u8]` implements `Reader` and `&mut [u8]` implements
  `Writer`][sliceio].
* [Typechecking has been moved into its own crate][typeck].
* Many `match` expressions in the compiler were [replaced by `if
  let`][iflet], which appears to be a nice improvement in readability.
* The 'expected <foo>, found <bar>' parse errors are [much more
  accurate about what they actually expect][parse].

[sliceio]: https://github.com/rust-lang/rust/pull/18980
[typeck]: https://github.com/rust-lang/rust/pull/19362
[iflet]: https://github.com/rust-lang/rust/pull/19405/files
[ungate]: https://github.com/rust-lang/rust/pull/19472
[ungate-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0450-un-feature-gate-some-more-gates.md
[es6]: https://github.com/rust-lang/rfcs/pull/446
[es6-rfc]: https://github.com/rust-lang/rfcs/pull/446
[parse]: https://github.com/rust-lang/rust/pull/19494

## New Contributors



# Approved RFC's



# New RFC's



# Community

## From the Team

There was no weekly meeting as the team was at a workweek in
Portland. There weren't a lot of coherent minutes taken this time, but
any discussions of substance will result in RFCs. Topics were largely
around stabilization in preparation for 1.0, and this workweek
featured a greater ratio of hacking to talking than previous ones.

## Blog Posts



## Discussions



## New Projects



## Project Updates



## Upcoming Meetups


