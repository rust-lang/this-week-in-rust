Title: This Week in Rust 48
Date: 2014-05-11 21:58
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

76 pull requests were merged this week.

## Breaking Changes

- A significant amount of functionality [has
  moved](https://github.com/mozilla/rust/pull/13901) from `libstd` to a new
  `libcore` crate, which is intended to be usable in a freestanding
  environment (it doesn't use any allocation etc). See the pull request for
  further details.
- All mentions of `~` outside of `~str` and `~[]` [have been
  removed from the language](https://github.com/mozilla/rust/pull/13958).
- `~[T]` [no longer implements
  `FromIterator`](https://github.com/mozilla/rust/pull/13963), and many APIs
  have been updated to use `Vec`.
- The `local_data` API [has been
  modernized](https://github.com/mozilla/rust/pull/13835) to use methods on
  keys, RAII, and removal of essentially unused features.
- Trait inheritence with incorrect use of lifetimes [has been
  fixed](https://github.com/mozilla/rust/pull/14055). See the second commit
  for a testcase and explanation.
- Cross-crate enum variant privacy [has been
  fixed](https://github.com/mozilla/rust/pull/14001). Previously, private
  variants were always usable without restriction.
- Format string parsing [has been moved outside of
  `std`](https://github.com/mozilla/rust/pull/13985).
- The `bump_box_refcount` function [has been
  removed](https://github.com/mozilla/rust/pull/14019).
- `unsafe extern fn` definitions [are now
  allowed](https://github.com/mozilla/rust/pull/14005), with a corresponding
  change in how that type is written (previously the parser accepted `extern
  unsafe fn()`).
- `bitflags!` [now allows setting attributes on the generated
  type](https://github.com/mozilla/rust/pull/13897), and `FilePermissions` has
  been ported to it.

## Other Changes

- `Box<T>` [is the new syntax for
  `~T`](https://github.com/mozilla/rust/pull/13904).
- Sockets [now have non-blocking
  IO](https://github.com/mozilla/rust/pull/13814)!
- `TcpStream` and `UnixStream` [have methods to cancel
  reads/writes](https://github.com/mozilla/rust/pull/13751).
- There is now limited support for [mixing `rlib`s and
  `dylib`s](https://github.com/mozilla/rust/pull/13892) when linking to
  dependencies.
- The `stats` crate [has been generalized to the `Float`
  trait](https://github.com/mozilla/rust/pull/13822).
- A `graphviz` crate [has been
  added](https://github.com/mozilla/rust/pull/13749) for generating `dot`
  files.
- Some missing cases in the `type_limits` lint [have been
  fixed](https://github.com/mozilla/rust/pull/13936).
- `box` [has been added](https://github.com/mozilla/rust/pull/13908) to the
  pattern grammar.
- An injection bug in rustdoc's web frontend [has been
  fixed](https://github.com/mozilla/rust/pull/13895).
- Use of non-existent method which has the same name as an existing static
  method [now has a note
  attached](https://github.com/mozilla/rust/pull/13685).

## New Contributors

- Aaron Raimist
- Ali Smesseim
- Dirk Leifeld
- James Laverack
- Lucas Dohmen
- Phil Ruffwind
- Tim Brooks

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2014-05-06)
discussed many things. I think I'm going to stop including this section of
TWiR because the meeting notes are very well-kept (thanks Lars!) and there's
often too much to effectively summarize.

# RFCs

- [Remove `*mut T`, add `*const T`](https://github.com/rust-lang/rfcs/pull/68)
- [Add byte and byte string
  literals](https://github.com/rust-lang/rfcs/pull/69)
- [Allow blocks in constants](https://github.com/rust-lang/rfcs/pull/71)
- [Algebraic effect system](https://github.com/rust-lang/rfcs/pull/73)
- [Split Iterator into Iterator and
  FiniteIterator](https://github.com/rust-lang/rfcs/pull/74)

# Community Updates

- [Bay Area Rust, May 2014:
  Testing](https://air.mozilla.org/rust-meetup-may-2014/)
- [New meetup at Pittsburgh Code and
  Supply](http://www.reddit.com/r/rust/comments/253vxq/rust_lang_meetup_pittsburgh_code_supply/)
- [Rust for C++ programmers - part 5: borrowed
  references](http://featherweightmusings.blogspot.co.nz/2014/05/rust-for-c-programmers-part-5-borrowed.html)
- [Header compression library for
  HTTP/2](http://www.reddit.com/r/rust/comments/24unld/header_compression_library_for_http2_written_in/)
- [rust-graphics](http://www.reddit.com/r/rust/comments/259wwp/rustgraphics_how_rusts_type_system_might_improve/),
  how Rust's type system might improve graphics programming
- [Informal survey: Which is clearer, mutability or
  uniqueness?](http://www.reddit.com/r/rust/comments/2581s5/informal_survey_which_is_clearer_mutability_or/)
- [Teepee design: header
  representation](http://www.reddit.com/r/rust/comments/254q2o/teepee_design_header_representation/)
- [How to test Rust on
  travis-ci](http://bettong.net/2014/05/09/how-to-test-rust-on-travis-ci/)
