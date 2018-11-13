Title: This Week in Rust 39
Date: 2014-03-02 12:43
Category: This Week in Rust


Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

<!-- more -->

# What's cooking on master?

83 pull requests were merged this week.

## Breaking Changes

- IterBytes and `std::to_bytes` [have been
removed](https://github.com/mozilla/rust/pull/12492). If you were depending on
them for anything non-`Hash` related, look into using `serialize` instead.
- `break` and `continue` [are now hygienic in
macros](https://github.com/mozilla/rust/pull/12338), with respect to labels.
Unfortunately there's a bug in it where [loop labels hide variables of the
same name](https://github.com/mozilla/rust/issues/12512).
- `extern fn` [now always means `extern "C"
fn`](https://github.com/mozilla/rust/pull/12328), rather than being
context-dependent.
- `std::run` [has been removed](https://github.com/mozilla/rust/pull/12380),
and `std::io::process` improved to pick up the slack.
- `#[deriving(ToStr)]` [has been
removed](https://github.com/mozilla/rust/pull/12412) in favor of
`#[deriving(Show)]`. `ToStr` is now implemented over all types which implement
`Show`, as `format!("{}" val)`.
- `flate` [now returns a
`CVec<u8>`](https://github.com/mozilla/rust/pull/12445) rather than a `~[u8]`,
for efficiency.
- `extra::json` [has been
liberated](https://github.com/mozilla/rust/pull/12453) and now lives in
`libserialize`.
- `swap_remove` on vectors [now returns an
`Option`](https://github.com/mozilla/rust/pull/12481).
- A curious bug where bindings in match weren't treated as assignment [has
been fixed](https://github.com/mozilla/rust/pull/12508). See the associated
bug report for examples which this will break.
- Some useless reexports [have been
removed](https://github.com/mozilla/rust/pull/12485) from the prelude.
- The `Bool` trait [has been
removed](https://github.com/mozilla/rust/pull/12473).
- A bug where operator overloads were over-zealous [has been
fixed](https://github.com/mozilla/rust/pull/12493).
- Statics are [no longer allowed to contain anything with
destructors](https://github.com/mozilla/rust/pull/11979). The exact rules are
laid out quite nicely in the PR.
- `collections:List` [has been refactored to use
iterators](https://github.com/mozilla/rust/pull/12348).
- `HashMap` [is now generic over
hashers](https://github.com/mozilla/rust/pull/12544).
- `assert_eq!` [now uses `{}` instead of `{:?}` for the error
message](https://github.com/mozilla/rust/pull/12626).
- Type parameter overrides [are now feature
gated](https://github.com/mozilla/rust/pull/12525).

## Other Changes

- Invalid crate handling [is now more
graceful](https://github.com/mozilla/rust/pull/12645).
- `std::io::stdout()` and `stderr()` [are buffered by default
now](https://github.com/mozilla/rust/pull/12630).
- Improper operator overloads [are no longer an
ICE](https://github.com/mozilla/rust/pull/12638).
- There's now a warning for [publicly exposing private types in function
signatures](https://github.com/mozilla/rust/pull/12595).
- `libnative`'s Windows file handling [has been
rewritten](https://github.com/mozilla/rust/pull/12584) and now actually works.
rustc uses libnative by default now.
- Generated binary size [has been brought down a
bit](https://github.com/mozilla/rust/pull/12616). There is much more room for
improvement.
- "Strict Version Hashes" [have been
introduced](https://github.com/mozilla/rust/pull/12533). This is a purely
internal change. The hash is of the crate's public ABI and rustc checks this
when linking crates together, to avoid problems such as "def id drift".


## New Contributors

- Daniel Fagnan
- Felix Crux
- Gary M. Josack
- George Papanikolaou
- Jag Talon
- Johannes Löthberg
- Mickaël Delahaye

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2014-02-25)
discussed TotalEq/TotalOrd, weak extern functions, channel naming, and a brief
discussion on `use` paths being absolute.

It was announced that a Sam Wright has been contracted to work on the
tutorial, and the Michael Woerister has been contracted to work on debug info.

# This Week in Servo

Servo is a web browser engine written in Rust and is one of the primary test
cases for the Rust language.

This week, we landed 27 PRs.

## Notable additions

- Josh Matthews landed the massive changes to remove `@mut` from the DOM types
in preparation for a Rust upgrade in
[#1735](https://github.com/mozilla/servo/pull/1735)
[#1591](https://github.com/mozilla/servo/pull/1591) and
[#1755](https://github.com/mozilla/servo/pull/1755).
- Tetsuharu Ohzeki refactored our commandline options out of the rendering
crate and into a more reasonable place in
[#1738](https://github.com/mozilla/servo/pull/1738).
- Ms2ger simplified our Document implementation in
[#1739](https://github.com/mozilla/servo/pull/1739).
- Patrick Walton removed the need to explicitly create leaf sets during
parallel layout in [#1700](https://github.com/mozilla/servo/pull/1700).
- Harrison Gentry cleaned up some type names in
[#1781](https://github.com/mozilla/servo/pull/1781).
- Keegan McAllister moved fontconfig in-tree on Linux in order to get a
thread-safe implementation in
[#1780](https://github.com/mozilla/servo/pull/1780).
- Saurabh Anand made the DOM parser pass in the correct content types in
[#1775](https://github.com/mozilla/servo/pull/1775).
- tgkokk fixed the bindings generator to no longer produce .pyc files in
[#1735](https://github.com/mozilla/servo/pull/1735).


## New contributors
- Saurabh Anand (sawrubh)
- Harrison Gentry (hgentry)
- tgkokk

## Meetings

At this week's
[meeting](https://github.com/mozilla/servo/wiki/Meeting-2014-02-24), we
discussed embedding, the JSManaged conversion, a Rust upgrade, vtables, and
recruiting efforts.

# Announcements, etc

- [RFC: Opt-in Builtin
Traits](http://smallcultfollowing.com/babysteps/blog/2014/02/28/rust-rfc-opt-in-builtin-traits/)
- [Structural single-inheritance
counter-proposal](https://github.com/mozilla/rust/issues/9912#issuecomment-36073562)
- [RFC: Stronger Guarantees for Mutable
Borrows](http://smallcultfollowing.com/babysteps/blog/2014/02/25/rust-rfc-stronger-guarantees-for-mutable-borrows/)
- [Dark](https://github.com/kvark/dark), an experimental universal data
compressor based on the BWT-DC scheme
- [cargo-lite
v1.1.0 release](https://mail.mozilla.org/pipermail/rust-dev/2014-March/008832.html)
- [Revamped parallel layout in
servo](http://pcwalton.github.io/blog/2014/02/25/revamped-parallel-layout-in-servo/)
- [February Bay Area Rust
Recording](https://air.mozilla.org/rust-meetup-february-2014/)
- [A bloom filter implementation](https://github.com/brianmadden/rust-bloom-filter)
- [A tweening library](https://github.com/hoeppnertill/redox-tween)
