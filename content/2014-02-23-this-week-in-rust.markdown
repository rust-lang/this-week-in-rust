Title: This Week in Rust 38
Date: 2014-02-23 16:09
Category: This Week in Rust


Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

This is the busiest week in This Week in Rust's history, [and the pull request
queue isn't getting any
shorter](http://buildbot.rust-lang.org/bors/bors.html). This is a mixed
blessing: tons of work is getting done, but it takes forever to get merged.

<!-- more -->

# What's cooking on master?

89 pull requests were merged this week. This is the most pull requests merged
in a week, ever. 10 1.0 issues were closed this week, and 0 opened.

## Breaking Changes

- Unique vector patterns (matching on a `~[]`) [has been removed from the
language](https://github.com/mozilla/rust/pull/12244). One can still match
against a slice.
- `std::unstable` [has been
dismantled](https://github.com/mozilla/rust/pull/12311). Even as we march
towards 1.0 stability, I will still consider changes to any public interface
breaking.
- `Bitwise::population_count` [has been
renamed](https://github.com/mozilla/rust/pull/12331) to
`Bitwise::count_ones`.
- `TaskBuilder` [has seen some
cleanup](https://github.com/mozilla/rust/pull/12232) to actually use the
emerging "builder" pattern.
- As part of `std::num`s [continued
revamp](https://github.com/mozilla/rust/issues/10387), [the `Real` trait has
been dismantled](https://github.com/mozilla/rust/pull/12321) in favor of the
`Float` trait.
- Another issue with imports [has been
fixed](https://github.com/mozilla/rust/pull/12245). In particular, when
importing an item where there are two items with the name, one private and one
public, the private one was accidentally imported too. This shouldn't affect
any code, as it's a fairly convoluted case.
- UTF-16 handling [has been
refactored](https://github.com/mozilla/rust/pull/12317) to match the standard
string APIs.
- `extra::test` [has been
liberated](https://github.com/mozilla/rust/pull/12343) into a `libtest`.
- `extra::time` [has been
liberated](https://github.com/mozilla/rust/pull/12411) into a `libtime`.
- `EnumSet` [has been moved](https://github.com/mozilla/rust/pull/12415) into
`libcollections`.
- The `Integer` trait [has
migrated](https://github.com/mozilla/rust/pull/12326) to `libnum`.
- `std::hash` [has been rewritten and
redesigned](https://github.com/mozilla/rust/pull/11863).
- `std::trie` and `std::hashmap` [have been
moved](https://github.com/mozilla/rust/pull/12428) into `libcollections`.

## Other Changes

- Pull request 12345 was opened. In the spirit of significant arbitrary
numbers, [it is absolutely
menial](https://github.com/mozilla/rust/pull/12345).
- `rustdoc` [will now test code blocks by
default](https://github.com/mozilla/rust/pull/12298), not requiring a "rust"
annotation.
- Unix domain sockets [are now
implemented](https://github.com/mozilla/rust/pull/12103) in `libnative`.
- `MutexArc` [no longer has a `Freeze`
bound](https://github.com/mozilla/rust/pull/12336).
- rustdoc has seen [a bunch of minor
fixes](https://github.com/mozilla/rust/pull/12339).
- More noteworthy, there is now [syntax
highlighting](https://github.com/mozilla/rust/pull/12416) in rustdoc output!
- The `unnecessary_parens` lint [now looks at
assignments](https://github.com/mozilla/rust/pull/12366) for extraneous
parenthesis.
- A bug with infinitely recursing macro errors [has been
fixed](https://github.com/mozilla/rust/pull/12370).
- `std::str::is_utf8` [has been internally
refactored](https://github.com/mozilla/rust/pull/12314) to use 100% safe code,
as well as get a bit of a speed boost.
- `format!` [now handles temporaries
better](https://github.com/mozilla/rust/pull/12349).
- If you find yourself working with libsyntax, [there is now a `-Z`
option](https://github.com/mozilla/rust/pull/12387) for dumping the AST as
JSON. This can be useful when trying to figure out what code corresponds to
what structure or enum variant.
- Using channels outside of the runtime [has been
fixed](https://github.com/mozilla/rust/pull/12397).
- Generic Drop implementations with trait bounds [has been
fixed](https://github.com/mozilla/rust/pull/12403). Yay!
- Integer formatting [has been
rewritten](https://github.com/mozilla/rust/pull/12382). It's now cleaner, does
less allocation, and is 3-6x faster in many cases.
- `std::io::stdin` [is now buffered by
default](https://github.com/mozilla/rust/pull/12422).
- All language items [must now be
reachable](https://github.com/mozilla/rust/pull/11603). Previously this caused
a linker error.

## New Contributors

- Axel Viala
- Craig MacKenzie
- Douglas Young
- Dylan Braithwaite
- Ehsanul Hoque
- Sterling Greene

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2014-02-18)
discussed the Hash changes, debug assertions, and commit log administrivia.

# This Week in Servo
Servo is a web browser engine written in Rust and is one of the primary test
cases for the Rust language.

This week, we landed 15 PRs.

## Notable additions

- Sankha Narayan Guria made drawing a single line much more efficient in
[#1709](https://github.com/mozilla/servo/pull/1709)
- Lars Bergstrom removed the last of the `@mut`s not in script in
[#1712](https://github.com/mozilla/servo/pull/1712)
- Junyoung Cho fixed up a bug where we were removing `&nbsp;`s in
[#1727](https://github.com/mozilla/servo/issues/1727)
- Youngmin Yoo added support for the `<object>` element in
[#1664](https://github.com/mozilla/servo/pull/1664)
- Keegan McAllister made use of the border box more consistent in layout
in [#1699](https://github.com/mozilla/servo/pull/1699)
- Peiyong Lin fixed up the naming of some of our flow methods in
[#1693](https://github.com/mozilla/servo/pull/1693)
- Simon Sapin refactored the `cascade` methods in
[#1706](https://github.com/mozilla/servo/pull/1706)
- Adam Sinnett corrected the parent type names of Text, Comment, and PI
types in [#1702](https://github.com/mozilla/servo/pull/1702)
- Patrick Walton added some inlining that sped up flow construction even
more in [#1602](https://github.com/mozilla/servo/pull/1602)

## New contributors

- Peiyong Lin (lpy)
- Adam Sinnett (quandrum)

## Meetings

We did not have a meeting this week because of President's Day in the US.

# Announcements, etc

- [`rust-story`](https://github.com/drbawb/rust-story), a Rust port of the
"Reconstructing Cave Story" video series
- [`rust-gamedev-kit`](http://www.reddit.com/r/rust/comments/1y69r0/rlanerustgamedevkit/),
a collection of libraries for gamedev
- [Periodic Table of Rust
Types](http://www.reddit.com/r/rust/comments/1yfdzh/the_periodic_table_of_rust_types/)
- [A Dominion simulator, in
Rust](http://www.reddit.com/r/rust/comments/1ykop6/so_i_wrote_a_dominion_simulator_in_rust_though/)
- [Travis CI is building pull
requests](https://mail.mozilla.org/pipermail/rust-dev/2014-February/008763.html)
- [A terminal tetris game, in
Rust](http://www.reddit.com/r/rust/comments/1yr2uz/tetris_game_in_rust/)
