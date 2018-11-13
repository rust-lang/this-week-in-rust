Title: This Week in Rust 52
Date: 2014-06-14 23:29
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

74 pull requests were merged in the last week.

## Breaking Changes

So just dumping the breaking change log raw didn't really work out last time.
The complete log is available
[here](https://gist.github.com/cmr/d0e6d145af65e6d74713), and you can view it
with `git log --grep 'breaking-change' --since 6/7/2014 --until 6/14/2014`.
I've chosen some particular breaking changes I think are more-breaking than
the others.

- `~[T]` [has been 100% removed from the
  language](https://github.com/mozilla/rust/pull/14703). To create a `Box<[T,
  ..N]>`, however, you temporarily need to do `box () ([1, 2, 3])`, rather
  than `box [1, 2, 3]`, which will work in the future.
- Patterns that shadow themselves [are no longer
  allowed](https://github.com/mozilla/rust/pull/14801), for example `let (a,
  a) = (1, 2);`
- Some features [have been
  removed](https://github.com/mozilla/rust/pull/14831) from `format!`, namely
  pluralization and select, and escaping them has changed somewhat to no
  longer use `\`.
- The ["stronger guarantees for mutable
  borrows"](http://smallcultfollowing.com/babysteps/blog/2014/02/25/rust-rfc-stronger-guarantees-for-mutable-borrows/)
  RFC [has been implemented](https://github.com/mozilla/rust/pull/14739).
- The `@`-syntax [has been removed
  entirely](https://github.com/mozilla/rust/pull/14835). Together with `~[T]`,
  this marks the end of the tyrannical rule of sigils!
- `transmute` [no longer casts](https://github.com/mozilla/rust/pull/14859)
  between types which have type parameters.

## Other Changes

- The [`PartialEq`
  docs](http://doc.rust-lang.org/std/cmp/trait.PartialEq.html) have been [revised for
  clarity and correctness](https://github.com/mozilla/rust/pull/14733).
- The identifier name lint [now gives
  suggestions](https://github.com/mozilla/rust/pull/14740) on what you could
  rename your identifier to.
- The loadable syntax extensions work [has been
  generalized](https://github.com/mozilla/rust/pull/14554) to more arbitrary
  compiler plugins.
- Function call overloading [is now
  implemented](https://github.com/mozilla/rust/pull/14590), as part of the
  unboxed closure work.
- Unused struct fields [are now
  detected](https://github.com/mozilla/rust/pull/14696) by the `dead_code`
  lint.
- The docs [are now built with relative
  links](https://github.com/mozilla/rust/pull/14777), letting local, off-line
  copies work as expected.
- `libsync` [is now underneath
  `libstd`](https://github.com/mozilla/rust/pull/14746).
- Debuginfo type identifiers [are now unique,
  cross-crate](https://github.com/mozilla/rust/pull/14819) fixing debuginfo
  with LTO.
- The results of compiler analysis [can now be dumped to
  CSV](https://github.com/mozilla/rust/pull/13222), which the [DXR
  tool](https://wiki.mozilla.org/DXR) will use.
- Rotates and byte swaps [are now
  exposed](https://github.com/mozilla/rust/pull/14866) as nice methods on the
  `Bitwise` trait.

## New Contributors

- Michael Reinhard
- Renato Riccieri Santos Zannon
- Renato Zannon
- Valentin Tsatskin
- Zach Pomerantz
- bachm
- theptrk

# New RFCs

- [Unboxed closures](https://github.com/rust-lang/rfcs/pull/114), the unboxed
  closures RFC to rule them all.
- [Removing integer inference
  fallback](https://github.com/rust-lang/rfcs/pull/115)
- [Feature gate import shadowing](https://github.com/rust-lang/rfcs/pull/116)
- [Rename `unsafe` to `trusted`](https://github.com/rust-lang/rfcs/pull/117)
- [Overloaded arithmetic and logical operators should take `self` and their
  arguments by value](https://github.com/rust-lang/rfcs/pull/118)
- [Add support to serialize::json for incrementally reading multiple JSON
  objects](https://github.com/rust-lang/rfcs/pull/119)
- [Reintroduce `do` keyword as sugar for nested match
  statements](https://github.com/rust-lang/rfcs/pull/120)

# Community Updates

- [Comparing k-NN in Rust](http://huonw.github.io/2014/06/10/knn-rust.html)
- [Error handling in Rust: a k-NN case
  study](http://huonw.github.io/2014/06/11/error-handling-in-rust-knn-case-study.html)
- [Los Angeles Rust
  meetup](http://www.reddit.com/r/rust/comments/27x6b6/los_angeles_rust_meetup/)
- [AnyMap](https://github.com/chris-morgan/anymap), "a safe and convenient
  store for one value of each type".
- [Rust By Example](http://rustbyexample.com/) now lets you execute code
  examples on-page.
- [Piston game engine progress
  update](http://www.reddit.com/r/rust/comments/286vfx/piston_game_engine_update_notice_on_progress/)
- [floor](http://cburgdorf.github.io/Floor/doc/floor/index.html), "a simple
  and lightweight foundation for web applications written in Rust".
- [jit.rs](http://tombebbington.github.io/blog/2014/06/15/rust-libjit-wrapper/),
  a libjit wrapper
- [stompers](https://github.com/mattyhall/stompers), a STOMP client
- [Brooklyn.rs](https://mail.mozilla.org/pipermail/rust-dev/2014-June/010232.html)
- [Rust nightlies archive](http://rustly.kokakiwi.net/)
- [Rusty Tetris](https://github.com/bachm/rusty-tetris), a tetris clone using
  Piston
- [Introduction to
  Rust](http://www.meetup.com/Pittsburgh-Code-Supply/events/184125612/), a
  talk Ben Striegel is giving in Pittsburgh.
