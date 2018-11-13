Title: This Week in Rust 17
Date: 2013-09-30 13:11
Category: This Week in Rust
Tags: rust, programming

Welcome to another issue of *This Week in Rust*! This week saw the release of
0.8, the removal of `@fn`, and a bunch of other changes. The tutorial has been
updated significantly, surrounding pointers and boxes, so you should re-read
those sections if you're still feeling shaky on when you should use which.

<!-- more -->

# What's cooking on master?

There were 79 PRs merged this week.

## Breaking changes

- `printf!` and `printfln!` have been
  [removed](https://github.com/mozilla/rust/pull/9550).
- `continue` is now a keyword, a [synonym for
  `loop`](https://github.com/mozilla/rust/pull/9504), preceeding `loop`'s
  removal (https://github.com/mozilla/rust/issues/9467)
- Item visibility is [properly encoded in
  metadata](https://github.com/mozilla/rust/pull/9432), so code which used
  private items that may have worked before is now rejected.
- Char literals which should have been escaped [are now rejected when they
  aren't escaped](https://github.com/mozilla/rust/pull/9335).
- `impl Foo for T;` is now [disallowed](https://github.com/mozilla/rust/pull/9336).
  Use `impl Foo for T {}`.
- `@fn` has been [removed from the
  language](https://github.com/mozilla/rust/pull/9310). Replace it with
  `@Trait` objects, if you really need `@fn` (you probably don't).
- `start` [no longer takes a crate
  map](https://github.com/mozilla/rust/pull/9301).
- Some more keywords have been
  [reserved](https://github.com/mozilla/rust/pull/9389): `alignof`,
  `offsetof`, and `sizeof`.

## Everything else

- A ton of documentation work was put in this week. The box section of the
  tutorial has been [rewritten](https://github.com/mozilla/rust/pull/9589),
  and there's a new [rustpkg
  tutorial](https://github.com/mozilla/rust/pull/9439) too. The module
  tutorial saw the file section
  [extended](https://github.com/mozilla/rust/pull/9398). There was a bunch of
  module documentation updated/added too.
- rustdoc(_ng) also saw a lot of work. It [completely
  replaced](https://github.com/mozilla/rust/pull/9402) the old rustdoc, as
  well as seeing a [lot](https://github.com/mozilla/rust/pull/9577) of
  [fixes](https://github.com/mozilla/rust/pull/9475). Huge thanks to Alex
  Crichton for all the work with it!
- `std::vec` has seen some [nice
  changes](https://github.com/mozilla/rust/pull/9583) to `connect_vec` and
  `concat_vec` (they were really awful before).
- `rustpkg test` is [implemented](https://github.com/mozilla/rust/pull/9549).
- `type_use` has been [removed](https://github.com/mozilla/rust/pull/9538).
  This did some nasty things and LLVM's `mergefunc` pass will do a better job
  of the same optimization (once it's enabled).
- `with_c_str` is now [optimized](https://github.com/mozilla/rust/pull/9352)
  to be allocation-free for short vectors.
- The logging system, when using the new formatting code (so `debug2` etc) is
  [allocation-free](https://github.com/mozilla/rust/pull/9261).
- `std::rand` has seen some
  [cleanup](https://github.com/mozilla/rust/pull/9362), in preparation for its
  overhaul.

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-09-24)
discussed a bunch of things, the main things of importance to me being:

- `loop` -> `continue`
- Allowing nested comments

# Announcements

- [wxRust](https://github.com/kenz-gelsoft/wxRust) - A binding to the
  wxWidgets toolkit.
- [widmann](http://www.reddit.com/r/rust/comments/1neu74/the_start_of_a_sinatra_clone/)
  \-  A Sinatra clone
- [gl-rs](http://www.reddit.com/r/rust/comments/1nab2s/rustdev_opengl_glrs_updated_with_command_line/)
  has been updated to generate only bindings for a select version / set of
  extensions.
- [Debugging Rust in
  Eclipse](http://www.reddit.com/r/rust/comments/1n8y5b/debugging_rust_in_eclipse/)
  \- Michael Woerister's final GSoC post. He did really great work this
  summer!
- [Rust for Rubyists](https://github.com/steveklabnik/rust_for_rubyists) has
  been open sourced!
- The [Claymore](https://code.google.com/p/claymore-game/) project has just
  come to my attention. Apparently it has been going on for quite some time.
  There are some screenshots at <http://claymore-dev.blogspot.com/>.
- [rustenstein3d](https://github.com/JeremyLetang/rustenstein3D/) - a
  Wolfenstein3D engine
- [clio](https://github.com/eevee/clio), a roguelike. This one also slipped
  under my radar for a while.
- [rust-gmp](https://github.com/thestinger/rust-gmp) has been updated for
  0.8/master.
- [rustdoc_ng](https://github.com/cmr/rustdoc_ng), being merged into mainline,
  has concluded as a project.
- And, of course, the [0.8
  release](http://www.reddit.com/r/rust/comments/1n7q8v/08_released/)
  discussion on reddit.
