Title: This Week in Rust 27
Date: 2013-12-16 16:14
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*! It's late this week
because bors was having some issues which bounced every pull request, and it's
no fun writing about nothing.

<!-- more -->

# What's cooking on master?

61 PRs were merged this week.

## Breaking Changes

- The first part of the `box` changes [has
landed](https://github.com/mozilla/rust/pull/10929). `box` is a keyword, and
in an expression is now synonymous with `~` (ie, it allocates an owned box).
- `std::vec::raw` [has seen some
cleanup](https://github.com/mozilla/rust/pull/10984).
- Link meta attributes [have been
replaced](https://github.com/mozilla/rust/pull/10593) with a `pkgid`.  rustdoc
[also now requires](https://github.com/mozilla/rust/pull/10948) a `pkgid`
attribute, rather than the deprecated link meta.
- Some fairly obscure import syntax [has been
changed](https://github.com/mozilla/rust/pull/10808). `use {foo, bar}` is now
permitted, and the old `use foo, bar` is not.
- `Cell` [has been removed](https://github.com/mozilla/rust/pull/10791).
`RefCell` supersedes it.
- Attempting to implement private traits [no longer
works](https://github.com/mozilla/rust/pull/10862).
- The coherence `-Z` flag [has been
removed](https://github.com/mozilla/rust/pull/10909).
- The `self` lifetime [is now
illegal](https://github.com/mozilla/rust/pull/10897), like the rest of the
keywords. Now, lifetime parameters are truly only identifiers.

## Other changes

- A dead code warning [has been
implemented](https://github.com/mozilla/rust/pull/10477). This is pretty
awesome and will do things like warn for test functions that are accidentally
never marked `#[test]` and so are never called. Some bugs in it were
[also](https://github.com/mozilla/rust/pull/10870)
[fixed](https://github.com/mozilla/rust/pull/10994).
- The benchmark runner [now
handles](https://github.com/mozilla/rust/pull/10952) slow benchmarks.
Previously, it wouldn't run a benchmark that took more than 1ms. Now, it will!
- The `shootout-fasta` benchmark [has been
rewritten](https://github.com/mozilla/rust/pull/10933). It's about 10x faster,
and looks nicer.
- Stepping through / breaking on function calls [no longer goes through the
function prelude](https://github.com/mozilla/rust/pull/10966).
- The vector `move_iter` [has been made
faster](https://github.com/mozilla/rust/pull/10995).
- A single [`#[inline]`](https://github.com/mozilla/rust/pull/10918)  has made
`vec::from_elem` 20x faster
- `Buffer` [now has an iterator over its
lines](https://github.com/mozilla/rust/pull/10856). The gotcha is that the
newlines are included in the yielded strings.
- Struct pattern shorthand [has been
improved](https://github.com/mozilla/rust/pull/10833) by allowing ref/mut,
like `let Foo { mut x, .. } = some_foo`, rather than the previous `let Foo {
x: mut x, .. } = some_foo`.
- LTO (link time optimization) [has been
implemented](https://github.com/mozilla/rust/pull/10812).  Rejoice.
- All landing pads (and thus unwinding!) [can now be
omitted](https://github.com/mozilla/rust/pull/10916) when doing LTO.
- `std::io::util` [has been added](https://github.com/mozilla/rust/pull/10895)
with:
	- `LimitReader` that will only read `n` bytes from another reader
	- `NullWriter` that ignores everything written to it
	- `NullReader` that is always EOF
	- `ZeroReader` which returns all zeroes,
	- `MultiWriter` which will replicate a written buffer to many writers
	- `ChainedReader` which reads readers in order to completion
	- `TeeReader` which will write to a Writer while reading
	- a `copy` function which copies the full contents of a reader into a writer.
- Windows application manifests [are now
embedded](https://github.com/mozilla/rust/pull/10878), so we don't get
special cased.
- A `--dep-info` flag [has been
added](https://github.com/mozilla/rust/pull/10698) which is like `-MMD` to C
compilers.
- `&mut` underneath `&` can [now be
frozen](https://github.com/mozilla/rust/pull/10787).
- `PortReader` and `ChanWriter` [have been
implemented](https://github.com/mozilla/rust/pull/10823).
- Some more random distributions [have been
implemented](https://github.com/mozilla/rust/pull/10859).
- We [now use](https://github.com/mozilla/rust/pull/10874) LLVM's integrated
assembler on Windows.

## New contributors

- Cadence Marseille
- Edward Z. Yang
- Erik Price
- Fabrice Desré
- Jan Niklas Hasse
- Richard Diamond

# Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-12-10)
discussed `enum mod`, using types to drive inference (rather than attributes
like `#[no_send]` etc), a formal grammar, unwinding, and some various PRs (all
mentioned above).

# This Week in Servo
Servo is a web browser engine written in Rust and is one of the primary test
cases for the Rust language.

We landed 29 PRs this week.

## Notable additions
- Patrick Walton landed a number of changes to layout to avoid going back to
the DOM for properties to make layout's access to the DOM more opaque. He also
cleaned up a significant number of regressions and added a huge number of ref
tests.
- Keegan McAllister made content tests work in headless mode in
[#1382](https://github.com/mozilla/servo/pull/1382).
- Daniel Glazman added namespace info to elements in
[#1358](https://github.com/mozilla/servo/pull/1358).
- Daniel Hedlund added support for leading font metrics on Linux
[#1352](https://github.com/mozilla/servo/pull/1352).

## Meetings
This week's
[meeting](https://github.com/mozilla/servo/wiki/Meeting-2013-12-09) discussed
an upcoming Servo workweek in SF on January 20th. We also discussed the Rust
version update and some general team frustration at a large number of
regressions lately, primarily caused by the need to disable some of our
automated testing due to instability (particularly at shutdown). This week of
work saw many of those issues fixed.

# Announcements, etc

- [FAQ Cheatsheet](https://github.com/mozilla/rust/wiki/Doc-FAQ-Cheatsheet) -
A cookbook-style cheatsheet of how to do simple things, based on IRC FAQs.
- [Building Rust Code - Current
Issues](http://metajack.im/2013/12/11/building-rust-code--current-issues/)
- [Building Rust Code - Using
Make](http://metajack.im/2013/12/12/building-rust-code--using-make/)
- [A Rust port of kissdb](https://github.com/pirapira/kissdb-rust)
- [Reddit
discussion](http://www.reddit.com/r/rust/comments/1syapv/implement_the_new_box_syntax_for_unique_pointers/)
of the `box` change.
