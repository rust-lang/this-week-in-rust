Title: This Week in Rust 57
Date: 2014-11-17
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

# What's cooking on master?

xxx pull requests were [merged in the last week][1]. Woo!

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-11-10..2014-11-17

## Breaking Changes

* The `Extendable` trait for extending a collection via an `Iterator`
  has been [renamed][extend] to `Extend`, and can now be used with
  `EnumSet` and `LruCache`.
* The old 'once_fns' feature gate has been [removed][once] (everybody
  thought it had been removed long ago). This is unrelated to the
  modern `FnOnce` type.
* The `time` crate, which is widely considered to be of poor quality,
  has been [moved out of the distribution][time], but can still be
  accessed via cargo.

[extend]: https://github.com/rust-lang/rust/pull/18475
[once_fns]: https://github.com/rust-lang/rust/pull/18743
[time]: https://github.com/rust-lang/rust/pull/18858

## Other Changes

* BTree [implements][btree] the [collection views][view-rfc] API.
* The `#[stable]` attribute is [no longer inherited][stable] by child
  AST elements. This is intended to reduce the risk of accidentally
  marking things stable.
* `AsRefReader` and `AsRefWriter` have been [renamed][asref] to
  `ByRefReader` and `ByRefWriter` for consistency with their method
  names. The original types remain and are deprecated.

[btree]: https://github.com/rust-lang/rust/pull/18287
[view-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0216-collection-views.md
[stable]: https://github.com/rust-lang/rust/pull/18887
[asref]: https://github.com/rust-lang/rust/pull/18891

## New Contributors
TODO



# Approved RFC's
TODO



# New RFC's
TODO



# Community

## From the Team

* [Weekly-meetings/2014-11-11 TODO](https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2014-11-11.md)
    * [Discuss]()
    * [Reddit]()



## Videos
TODO




## Blog Posts
TODO




## Discuss
TODO




## Reddit
TODO



## New Projects
TODO




## Upcoming Meetups

* [Rust Paris, Nov 17](http://www.meetup.com/Rust-Paris/events/185461312/)
* [Rust Bay Area: Cryptography and Rust, December 18th](http://www.meetup.com/Rust-Bay-Area/events/210632582/)
