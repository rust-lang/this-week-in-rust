Title: This Week in Rust 33
Date: 2014-01-18 20:21
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

This was a big week. rvalue lifetimes and external syntax extensions landed,
as well as a bunch of codegen optimization.

<!-- more -->

# What's cooking in master?

69 pull requests were merged this week. As of writing,
[bors](http://buildbot.rust-lang.org/bors/bors.html) has 14 approved pull
requests waiting.

## Breaking Changes

- `std::num`'s ongoing overhaul continues. It's going to be drastically
simplified, and not try to be a general-purpose numeric library.
[num-rs](https://github.com/bjz/num-rs) is intended to provide some base
algebraic types. [#11504](https://github.com/mozilla/rust/pull/11504) removes
gamma/bessel and does some cleanup of cmath.
[#11548](https://github.com/mozilla/rust/pull/11548) merges Bitwise and
BitCount and removes Bounded and Bitwise from the prelude.
[#11622](https://github.com/mozilla/rust/pull/11622) simplifies Primitive.
- Destructuring `@` patterns are [no longer
supported](https://github.com/mozilla/rust/pull/11305).
- The return type of `Chan::try_recv` has been extended to [indicate why it
did not return a value](https://github.com/mozilla/rust/pull/11112). That is,
whether there was legitimately no value, or if the other end had hung up.
- Disk-relative paths on Windows [are now properly
generated](https://github.com/mozilla/rust/pull/11579) when joining two paths.
- FromBase64 and FromHex [now use error
enums](https://github.com/mozilla/rust/pull/11597).
- `std::io` [has been shuffled around a
bit](https://github.com/mozilla/rust/pull/11598).
- (Almost?) all iterator types [have been
renamed](https://github.com/mozilla/rust/pull/11001).
- unsafe functions can [no longer be coerced to
closures](https://github.com/mozilla/rust/pull/11605).
- Unnecessary (useless) visibility modifiers on `use` and `extern mod` [are
now forbidden](https://github.com/mozilla/rust/pull/11607).
- Building on last week's per-task stdout/stderr, [rustc's error reporting has
been cleaned up significantly](https://github.com/mozilla/rust/pull/11620).

## Other Changes

- Externally loadable syntax extensions [have
landed](https://github.com/mozilla/rust/pull/11151)! We can now write
syntax extensions outside of the compiler, and export macros.
- The "rvalue lifetime" issue has seen some [significant work put into
it](https://github.com/mozilla/rust/pull/11585). Notably, `for x in [1, 2, 3,
4].iter() { .. }` should now work, among many other papercut annoyances with
rvalues.
- Error messages involving inferred integer types [are now much more
sane](https://github.com/mozilla/rust/pull/11513). Rather than `<V0>` or
`<VI1>`, it now prints `<generic integer #0>` etc.
- Accidentally including a semicolon on the last line of a function [now has a
better error message](https://github.com/mozilla/rust/pull/11482), if the
types line up.
- Some bugs with trait object coercion [have been
fixed](https://github.com/mozilla/rust/pull/11525).
- The runtime [now has its own
documentation](https://github.com/mozilla/rust/pull/11501), viewable
[here](http://static.rust-lang.org/doc/master/guide-runtime.html).
- rustdoc's search is now [more
forgiving](https://github.com/mozilla/rust/pull/11438) on the input it
accepts.
- Inserting into tries has been optimized significantly, [with no unsafe
code](https://github.com/mozilla/rust/pull/11546). Its iterator has [also been
optimized](https://github.com/mozilla/rust/pull/11497), albeit with unsafe
code.
- [The reference-counting headers have been
removed](https://github.com/mozilla/rust/pull/11535). The associated
"managed-unique" concept [has also been
removed](https://github.com/mozilla/rust/pull/11565).
- The recent OS X 10.9 breakage [should be
fixed](https://github.com/mozilla/rust/pull/11604), as we will no longer use
`ar s` on that platform.

## New Contributors

- Clinton Ryan
- Derek Guenther
- Lucy
- Petter Remen
- Shamir Khodzha
- Yehuda Katz

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2014-01-14)
discussed associated function resolution, changes to our calling convention
(specifically the fate of the ubiquitous environment pointer), how we want to
handle feature freeze, static items with destructors, and documentation on
primitives.

# Quotes of the Week

> who says you can't make money selling a compiler? You can, as long as you
convince people it's a video game

— tjc on the Rust-the-game/Rust-the-language confusion 

> who needs pure functional when you have pure cool?

— eddyb

# This Week in Servo

Servo is a web browser engine written in Rust and is one of the primary
test cases for the Rust language.

This week, we landed 22 PRs. There will be a Servo workweek in the
Mozilla San Francisco office next week, Tuesday through Friday.

## Notable additions

- Jack Moffitt landed the huge Rust upgrade
[#1473](https://github.com/mozilla/servo/pull/1473), bringing us very close to
the 0.9 release. Hooray!
- Lars Bergstrom got NVidia-on-Linux working in
[#1487](https://github.com/mozilla/servo/pull/1487) and fixed the content test
reliability in [#1500](https://github.com/mozilla/servo/pull/1500).
- Tetsuharu Ohzeki implemented `removeAttribute` on Element in
[#1448](https://github.com/mozilla/servo/pull/1448).
- jgraham landed innerHTML support
[#1450](https://github.com/mozilla/servo/pull/1450).
- Bruno Abinader implemented several fixes related to DOMImplementation:
[#1497](https://github.com/mozilla/servo/pull/1497),
[#1488](https://github.com/mozilla/servo/pull/1488).
- Corey Richardson fixed up an error case in startup on Linux in
[#1502](https://github.com/mozilla/servo/pull/1502).
- Isabelle Carter landed the initial work for handling `position:fixed` in
[#1440](https://github.com/mozilla/servo/pull/1440).

## New Contributors

- Corey Richardson

## Meetings

In this week's
[meeting](https://github.com/mozilla/servo/wiki/Meeting-2014-01-13), we
discussed the need to clean up our test harnesses, the removal of `@` in
support of the next Rust upgrade, and the workweek..


# Announcements, etc

- [The first pure-Rust AES
implementation](https://github.com/DaGenix/rust-crypto/pull/21). According to
enix in IRC, this is also the first bit slicing implementation in a high level
language (ie, not assembly or a port of assembly).
- [rust-re2](https://github.com/nickdesaulniers/rust-re2), re2 bindings
- [A 30 Minute Introduction to
Rust](http://words.steveklabnik.com/a-30-minute-introduction-to-rust)
- [ObjCrust](https://github.com/shilgapira/ObjCrust), building an iOS static
library with Rust.A
- [The Periodic Table of Rust
Types](http://cosmic.mearie.org/2014/01/periodic-table-of-rust-types/)
- [Sodium Oxide](https://github.com/dnaq/sodiumoxide), NaCl bindings, updated
to 0.9
- [capnproto-rust benchmark
update](http://dwrensha.github.io/capnproto-rust/2014/01/15/benchmark-update.html)
- [rust-phf](https://github.com/sfackler/rust-phf), compile-time hash maps. It
also serves as a demonstration of loadable syntax extensions!
- [Rust has do-while
loops](http://www.reddit.com/r/rust/comments/1v9rgp/rust_has_dowhile_loops/),
a dirty hack no one should use.
- [Understanding Pointers, Ownership, and Lifetimes in
Rust](http://paulkoerbitz.de/posts/Understanding-Pointers-Ownership-and-Lifetimes-in-Rust.html)
- [A Just-in-time Compiler (Written) In
Rust](http://hydrocodedesign.com/2014/01/17/jit-just-in-time-compiler-rust/)
- [jba](https://github.com/alexcrichton/jba/tree/rust), a GameBoy emulator
written in Rust.
