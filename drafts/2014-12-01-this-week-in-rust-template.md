Title: This Week in Rust 59
Date: 2014-12-1
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

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-11-24..2014-12-1

## Breaking Changes



## Other Changes



## New Contributors



# Approved RFC's

* [Disallow type/lifetime parameter shadowing][rfc459]: Rarely what you want, confusing when it happens.
* [Add "function name macro"][rfc466]: This RFC proposes the addition of a function! macro that expands to the function it's used in. This will greatly help error reporting.
* [extension trait conventions][rfc445]: This is a conventions RFC establishing a definition and naming convention for extension traits: `FooExt`.

[rfc459]: https://github.com/rust-lang/rfcs/pull/459
[rfc466]: https://github.com/rust-lang/rfcs/pull/466
[rfc445]: https://github.com/rust-lang/rfcs/pull/445

# New RFC's

* [Error Reform and Failure Box][rfc492]: This RFC proposes changes to the `Error` trait and the introduction of a `Failure<T>` wrapper type for errors and modifications to the `Error` trait for better error interoperability and debugging support.
* [Remove the `[]` notation for taking a whole slice][rfc491]: &* has emerged as less strange and more idomatic
* [Change DST syntax to `T: Sized?`][rfc490]: Change the syntax for dynamically sized type parameters from `Sized? T` to `T: Sized?`, and change the syntax for traits for dynamically sized types to `trait Foo: Sized?`. Extend this new syntax to work with `where` clauses.
* [rename `lifetime` to `lifespan`][rfc487]: 'nuff said
* [std::ascii reform][rfc486]: Move the `std::ascii::Ascii` type and related traits to a new Cargo package on crates.io, and instead expose its functionality for `u8`, `[u8]`, `char`, and `str` types.
* [Make brackets optional for single attributes][rfc484]: This is an alternative to #483: instead of changing attributes to `@`, just make the brackets optional for `#`.
* [Change attribute syntax to @foo][rfc483]: Change attribute syntax from `#[foo]` and `#![foo]` to `@foo` and `@!foo`.

[rfc492]: https://github.com/rust-lang/rfcs/pull/492
[rfc491]: https://github.com/rust-lang/rfcs/pull/491
[rfc490]: https://github.com/rust-lang/rfcs/pull/490
[rfc487]: https://github.com/rust-lang/rfcs/pull/487
[rfc486]: https://github.com/rust-lang/rfcs/pull/486
[rfc484]: https://github.com/rust-lang/rfcs/pull/484
[rfc483]: https://github.com/rust-lang/rfcs/pull/483

# Community

## From the Team

* [Weekly-meetings/2014-25-12][mtg]: extension trait conventions; shadowed lifetimes; problems with unused type params; es6-style string escaping; int fallback redux redux [Reddit][mtg-reddit].

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2014-11-25.md
[mtg-reddit]: http://www.reddit.com/r/rust/comments/2nhmgj/weekly_meeting_20141125_extension_trait/



## Blog Posts



## Discussions



## New Projects



## Project Updates



## Upcoming Meetups


