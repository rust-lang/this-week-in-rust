Title: This Week in Rust 30
Date: 2013-12-30 00:48
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

It's been a slow week due to the holidays. In the next week or two 0.9 is
being released. It's an exciting release, but in more subtler ways than the
previous 3. Many small details, especially around the runtime and linking,
have changed that make Rust faster and more flexible without necessarily being
a breaking change. As always, the detailed changelog will have the
nitty-gritties.

<!-- more -->

# What's cooking on master?

36 pull requests were merged this week. bors was feeling unwell for a bit,
due to a deadlock in a scheduler test that was fixed today and a deadlock in
(incorrect usage of) LLVM.

## Breaking changes

- The `comm` primitives are [never `Freeze`
anymore](https://github.com/mozilla/rust/pull/11111).
- The `link` attribute is [now
forbidden](https://github.com/mozilla/rust/pull/11091) on crates. All hail
`crate_id`!
- [All of our C++ dependencies have been
removed](https://github.com/mozilla/rust/pull/11121). This is only breaking
because it changes the debugging experience; `rust_begin_unwind` is gone and
`catch throw` doesn't work because we don't use C++ exceptions anymore. To set
a breakpoint on task failure, `break _Unwind_RaiseException`.
- The underbelly of the runtime has been [completely
overhauled](https://github.com/mozilla/rust/pull/10965). Alex wrote an [email
to the
list](https://mail.mozilla.org/pipermail/rust-dev/2013-December/007565.html)
about the practical implications of this.
- `std::result::collect` [now uses an
iterator](https://github.com/mozilla/rust/pull/11098).
- `ClonableIterator` [has been
renamed](https://github.com/mozilla/rust/pull/11160) to `CloneableIterator`.

## Other Changes

- libnative has [process and TCP](https://github.com/mozilla/rust/pull/11159)
implementations.
- Coercion of types into trait objects [is now
supported](https://github.com/mozilla/rust/pull/11156), which means `as
~SomeTrait` and `as &Reader` can be left out.
- I normally wouldn't mention this since it's internal to the compiler, but
Patrick made a heroic effort to [remove `@mut` from all the
places](https://github.com/mozilla/rust/pull/11058).
- rustdoc can [now test doc
comments](https://github.com/mozilla/rust/pull/11120). See the pull request
for details on how and what is tested (also in the rustdoc manual).

## New contributors

- Sébastien Paolacci

# Meeting

There was no meeting this week due to the holiday.

# This Week in Servo
Servo is a web browser engine written in Rust and is one of the primary test
cases for the Rust language.

Mozilla is on an extended holiday break until January 2nd, but we still landed
2 PRs this week.

## Notable additions
- Jack Moffitt re-enabled building with make to enable work on cross-targeting
ARM in [#1441](https://github.com/mozilla/servo/pull/1441).
- ms2ger cleaned up how we handle namespaces in DOM elements
[#1438](https://github.com/mozilla/servo/pull/1438)

# Announcements, etc

- [rust-openssl](https://mail.mozilla.org/pipermail/rust-dev/2013-December/007575.html)
has been formed from the union of sfackler's rust-ssl and erickt's rustcrypto.
- [Concurrency models, Rust, and
Servo](http://www.lars.com/concurrency/rust/servo/2013/12/21/concurrency-rust-and-servo.html).
- [Rust is surprisingly
expressive](http://words.steveklabnik.com/rust-is-surprisingly-expressive).
- [irust](https://mail.mozilla.org/pipermail/rust-dev/2013-December/007486.html),
a basic REPL written in Ruby.
