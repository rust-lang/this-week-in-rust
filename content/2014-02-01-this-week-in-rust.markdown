Title: This Week in Rust 35
Date: 2014-02-01 18:45
Category: This Week in Rust


Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

This was a good week, with default type parameters, `do` removal, `@[]`
removal, and some code bloat reduction.

<!-- more -->

# What's cooking on master?

61 pull requests were merged this week.

## Breaking Changes

- `do` [has been removed](https://github.com/mozilla/rust/pull/11868). Replace
`do spawn { ... }` with `spawn(proc() { ... })`.
- `libextra` [has exploded](https://github.com/mozilla/rust/pull/11787). [It
exploded some more](https://github.com/mozilla/rust/pull/11867). It will
continue to explode until it no longer exists.
- `#[simd]` [has been feature
gated](https://github.com/mozilla/rust/pull/11738).
- We [now generate static
libraries](https://github.com/mozilla/rust/pull/11706) by default, rather than
dynamic libraries.
- `@[]` and `@str` [have been removed from the
language](https://github.com/mozilla/rust/pull/11974). All that is left is
`@T`!
- Various methods on vectors [now return
Options](https://github.com/mozilla/rust/pull/11944), to be in line with the
rest of everything else.
- `next_power_of_two_opt` [has been renamed
to](https://github.com/mozilla/rust/pull/11930) `checked_next_power_of_two`.
Additionally, the various functions for division in `std::uint` were removed.
- `#[no_send]` etc [have been removed in favor of marker
types](https://github.com/mozilla/rust/pull/11768). Additionally, variance of
types and lifetimes can explicitly be annotated with these marker types. They
are zero-sized and have no runtime impact.
- `std::borrow` [has been
removed](https://github.com/mozilla/rust/pull/11895).
- `Times` [has finally been
removed](https://github.com/mozilla/rust/pull/11672). Good riddance.
- The various `CopyableVector`-like types have [been
renamed](https://github.com/mozilla/rust/pull/11893) to
`CloneableVector` etc.

## Other Changes

- There are now lints for [unused
values](https://github.com/mozilla/rust/pull/11754). Types annotated with
`#[must_use]` will now warn when (surprise) their results are not used.
- debuginfo [should now be fixed on OS
X](https://github.com/mozilla/rust/pull/11864), due to us now explicitly
setting the DWARF version.
- `fp-elim` [has been re-disabled, but only if debuginfo generation is
disabled](https://github.com/mozilla/rust/pull/11879).
- Errors from `#[deriving(...)]` have
[seen](https://github.com/mozilla/rust/pull/11826) some
[work](https://github.com/mozilla/rust/pull/11834).
- The evil environment pointers [has been removed from bare
functions](https://github.com/mozilla/rust/pull/11595), as well as `self` now
being a mostly-normal argument, to the compiler. This was a huge effort (88
changed files with 1,436 additions and 2,138 deletions) by Eduard Burtescu,
and is awesome!
- Also from Eduard is [default type
parameters](https://github.com/mozilla/rust/pull/11217)! They are currently
behind a feature flag, but allow you to say, for example, `struct Foo<T =
uint>` and use `Foo` as a bare type.
- The tydesc [is used in less
places](https://github.com/mozilla/rust/pull/11909) to help fend off code
bloat.
- The occasional infinite recursion in some recursive types [has been
fixed](https://github.com/mozilla/rust/pull/11839).
- Take glue [has been removed from
tydescs](https://github.com/mozilla/rust/pull/11723), also to fend off code
bloat.
- `fail!()` [also generates less code
now](https://github.com/mozilla/rust/pull/11841), to fend off bloat. In
particular, `fn main() { fail!() }` now compiles 2-3x faster due to generating
less code.

## New Contributors

- JeremyLetang
- Johannes Muenzel
- Keshav Kini
- Michael Darakananda
- Nathaniel Herman

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2014-01-28)
discussed default type parameters, the unused result lint, minor changes to
the attribute syntax, how to deal with I/O splitting, and the visit glue.

# This Week in Servo

Servo is a web browser engine written in Rust and is one of the primary test
cases for the Rust language.

This week, we landed 32 PRs, significantly above our usual PR landing rate!

## Notable additions

- Patrick Walton landed a huge number of PRs that significantly improved our
performance and parallelism
[#1600](https://github.com/mozilla/servo/pull/1600),
[#1564](https://github.com/mozilla/servo/pull/1564),
[#1567](https://github.com/mozilla/servo/pull/1567),
[#1589](https://github.com/mozilla/servo/pull/1589),
[#1566](https://github.com/mozilla/servo/pull/1566)
[#1574](https://github.com/mozilla/servo/pull/1574),
[#1571](https://github.com/mozilla/servo/pull/1571),
[#1559](https://github.com/mozilla/servo/pull/1559), etc.
- Rui (xiongmao86) fixed a linking issue in our Makefiles in
[#1603](https://github.com/mozilla/servo/pull/1603).
- Isabelle Carter landed multiple display list support in
[#1579](https://github.com/mozilla/servo/pull/1579).
- Lars Bergstrom finally re-enabled ref tests in
[#1565](https://github.com/mozilla/servo/pull/1565) and started a quest to
improve test reliability in
[#1597](https://github.com/mozilla/servo/pull/1597) and
[#1570](https://github.com/mozilla/servo/pull/1570).
- Bruno Abinader pushed on more DOM features in
[#1583](https://github.com/mozilla/servo/pull/1583) and
[#1580](https://github.com/mozilla/servo/pull/1580).
- Clark Gaebel (wowus) enabled a limit on the number of redirects Servo
follows in [#1562](https://github.com/mozilla/servo/pull/1562).
- Tetsuharu Ohzeki cleaned up Node's `remove` and `insert` members
[#1582](https://github.com/mozilla/servo/pull/1582).
- Deokjin Kim implemented `whitespace:pre` in
[#1547](https://github.com/mozilla/servo/pull/1547).
- Patrick Kim landed border support for inline flows in
[#1546](https://github.com/mozilla/servo/pull/1546).
- Simon Sapin used his CSS wizardry to redo a performance tweak we made so
that it's actually spec-compliant in
[#1560](https://github.com/mozilla/servo/pull/1560).

## New contributors

- Clark Gaebel (wowus)
- Rui (xiongmao86)

## Meetings

In this week's
[meeting](https://github.com/mozilla/servo/wiki/Meeting-2014-01-27), we
discussed moving the build of fontconfig on Linux into Servo to avoid
situations where the system version is old and not threadsafe, what's blocking
the JSManaged conversion (which is required for our next Rust upgrade), and
the status of ref and content tests.

Josh Matthews is at FOSDEM this weekend, giving a
[talk](http://www.joshmatthews.net/fosdemservo/) on Servo. Look for video on
air.mozilla.org shortly!

# Announcements, etc

- [An xxHash implementation in
Rust](http://www.reddit.com/r/rust/comments/1wqjsf/more_xxhash_benchmarks/),
as well as some benchmarks comparing to gcc and clang.
- [Using LLVM From Within
Rust](http://hydrocodedesign.com/2014/01/31/llvm-with-rust/)
- [Felix's CodeMesh presentation](http://vimeo.com/85253071)
- [A 1.0 roadmap
spreadsheet](https://docs.google.com/a/octayn.net/spreadsheet/ccc?key=0AlWBWplsaTZvdGdSdEVzZW1BeDA5dm0zM2FFeW0ySEE&usp=drive_web#gid=0)
- [rust-lua](https://github.com/kballard/rust-lua), safe bindings to Lua 5.1
- [InfoQ interview with
Felix](http://www.infoq.com/interviews/klock-rust?utm_source=infoq&utm_medium=videos_homepage&utm_campaign=videos_row1#.Uupx3qddN-w.reddit)
- [Parameters and Bounds in
Rust](http://blog.safaribooksonline.com/2014/01/30/parameters-bounds-rust/)
- [Network Communication and Serialization in
Rust](http://blog.safaribooksonline.com/2014/01/28/network-communication-serialization-rust/)
- [Recording of the Bay Area Rust January
meetup](https://air.mozilla.org/rust-meetup-january-2014/), wherein Brian
talks about 1.0, Niko talks about DST, Kevin talks about rust-lua, and I talk
about evangelism.
- [Deprecating
rustpkg](https://mail.mozilla.org/pipermail/rust-dev/2014-January/008224.html)
