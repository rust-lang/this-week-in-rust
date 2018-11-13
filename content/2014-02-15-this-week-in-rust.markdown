Title: This Week in Rust 37
Date: 2014-02-15 23:20
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

86 pull requests were merged this week. This ties for week with most merged
pull requests. [A week in September 2013 is the other record
holder](http://cmr.github.io/blog/2013/09/23/this-week-in-rust/). To cope with
the massively inflated queue, there were two roll-ups (not counted).

## Breaking Changes

- `extern mod` [is now written](https://github.com/mozilla/rust/pull/12017)
`extern crate`.
- The big codegen compiler flags pull request I warned about last week [indeed
landed](https://github.com/mozilla/rust/pull/12084). Many `-Z` options are now
under `-C`, and a lot of previously-bare flags (such as `--linker`) are now
also under `-C`.
- `std::util` [has been removed](https://github.com/mozilla/rust/pull/11956).
`swap` and `replace` now live in `std::mem`.
- `do` is once again [a reserved
word](https://github.com/mozilla/rust/pull/12170).
- `extra::rational`, `extra::bigint`, and `extra::complex` [have been
moved](https://github.com/mozilla/rust/pull/12154) into `libnum` as part of
the libextra dissolution.
- The borrow checker's treatment of closures [has been
revamped](https://github.com/mozilla/rust/pull/12158). It fixes all known
soundness issues with closures. Unfortunately, it also breaks some programs
that used to compile.
- Channels [have been rewritten](https://github.com/mozilla/rust/pull/11578)
to use the internally-upgradable design [that was hashed out on the
list](https://mail.mozilla.org/pipermail/rust-dev/2014-January/007924.html).
Rather than having a separate `SharedChan`, `Chan` is now cloneable.
- The `Seek` API [has changed a
bit](https://github.com/mozilla/rust/pull/12204).
- The breaking changes in the [first
rollup](https://github.com/mozilla/rust/pull/12248) are the removal of
`ptr::offset`, `ptr::mut_offset`, `ptr::is_null`, and `ptr::is_not_null` as
free functions and the movement of `extra::hex` and `extra::base64` to
`libserialize`.
- `std::num::Orderable` [has been
removed](https://github.com/mozilla/rust/pull/12061).
- `std::ptr` [saw some more
cleanup](https://github.com/mozilla/rust/pull/12282), most notably every
function ending in `_ptr` has had that suffix removed. `to_unsafe_ptr` and
`to_mut_unsafe_ptr` have also been removed.

## Other Changes

- Process arguments and environment variables [now use the
`from_utf8_lossy` function](https://github.com/mozilla/rust/pull/12283) that
was introduced last week, rather than failing on invalid utf8. Additionally,
there are now `args_as_bytes` and `env_as_bytes` functions to get arguments
and the environment raw.
- The makefiles [have been
refactored](https://github.com/mozilla/rust/pull/12274), and there is now a
`make help` and `make tips` for hints on how to use the build system.
- In yet another multi-thousand-line patch by eddyb, [`ast_map::Path` no
longer requires cloning](https://github.com/mozilla/rust/pull/12162), due to
clever devilry.
- green task spawning [was sped up by almost
5x](https://github.com/mozilla/rust/pull/12172).
- We now [bundle and use
compiler-rt](https://github.com/mozilla/rust/pull/12027) for intrinsics rather
than using the system libgcc. We still depend on libgcc for unwinding,
- The pidigits benchmark was made 20x faster [by optimizing
bigint](https://github.com/alexcrichton/rust/commit/7dc187afd8a19dad05dbf1a689e6b6f400f7bc0a).

## New Contributors

- Bruno de Oliveira Abinader
- Eduard Bopp
- Edward Wang
- Jake Kerr
- Liigo Zhuang
- Matthijs van der Vleuten
- Peiyong Lin
- Tobias Bucher
- WebeWizard

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2014-02-11)
discussed struct construction sugar, what to allow in statics, the crate
keyword, a `finally` macro, and implicit trait bounds.

# This Week in Servo

Servo is a web browser engine written in Rust and is one of the primary test
cases for the Rust language.

This week, we landed 18 PRs.

## Notable additions

- Bruno Abinader landed several DOM fixes, including
[#1648](https://github.com/mozilla/servo/pull/1648) and
[#1646](https://github.com/mozilla/servo/pull/1646)
- Hyun June Kim landed initial `:hover` support in
[#1633](https://github.com/mozilla/servo/pull/1633)
- Keegan McAllister restored task failure handling in
[#1691](https://github.com/mozilla/servo/pull/1691)
- Rui renamed the .rc files to .rs in the main Servo repository in
[#1617](https://github.com/mozilla/servo/pull/1617)
- Simon Sapin made some updates to attribute selector namespaces in
[#1653](https://github.com/mozilla/servo/pull/1653) and
[#1661](https://github.com/mozilla/servo/pull/1661)
- Lars Bergstrom began the removal of non-script-crate `@mut`s in preparation
for a Rust upgrade in [#1663](https://github.com/mozilla/servo/pull/1663)
- Austin King added some `window.console` support in
[#1666](https://github.com/mozilla/servo/pull/1666)
- Marek Šuppa landed a fix to our contributing document in
[#1649](https://github.com/mozilla/servo/pull/1649)
- Patrick Walton made extensive optimizations to style sharing in
[#1644](https://github.com/mozilla/servo/pull/1644)

## New contributors

- Austin King (ozten)
- Marek Šuppa (mrshu)

## Meetings

In this week's
[meeting](https://github.com/mozilla/servo/wiki/Meeting-2014-02-10), we
discussed our embedding plans, ACID2 status, improving the availability of
E-Easy issues, and doing a Rust upgrade (we are more than one month behind
Rust master).

# Announcements, etc

There is simply too much happening in the community to keep track of! I
recommend browsing [the Rust subreddit](http://www.reddit.com/r/rust) for
goings-on. Some notable ones:

- [Rust By Example:
HashMap](http://pzol.github.io/getting_rusty/posts/20140203_hashmap/)
- [State machines using phantom
types](https://gist.github.com/bvssvni/8970459)
- [golo-lang.org](http://golo-lang.org/)'s homepage design [adapted to
Rust](http://adridu59.github.io/rust-www/). There is some discussion [on
reddit](http://www.reddit.com/r/rust/comments/1xx3ll/rfc_gololangorg_ported_to_rust/)
about this.
