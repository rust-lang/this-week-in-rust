Title: This Week in Rust 5
Date: 2013-07-06 15:45
Category: This Week in Rust

Hello and welcome to the fifth issue of *This Week in Rust*, a weekly overview
of Rust and its community.

`0.7` was released this week. Hello to the newcomers! I've also decided to put
breaking changes first. Feel free to skip the rest, it's relatively
unimportant.

<!-- more -->

# Newcomers

There's already a lot of traffic from Rust newbies, so you get your own
section! Welcome to Rust. I wrote [The State of Rust
0.7](http://cmr.github.io/blog/2013/07/05/the-state-of-rust/) especially for
newcomers, so you should read that. Jump on IRC if you have any questions or
need help. We're a quite friendly bunch, and we usually don't bite.

# What's cooking on master?

Issue churn this week was +12. 35 PRs were merged, total PR churn was -8.
There continues to be a lot more cleanup than breaking changes, which is
encouraging! As I understand it, graydon wants to focus this release cycle on
cleanup, rather than language features. Hopefully the compiler can get into a
much better state.

## Breaking changes

- dbaupp [continues](https://github.com/mozilla/rust/pull/7487) to
  [slaughter](https://github.com/mozilla/rust/pull/7566) the free functions in
  `std::vec` where methods can replace them.
- He also [added a lint for lowercase
  statics](https://github.com/mozilla/rust/pull/7523), which is enabled by
  default because of an astoundingly poor error message.
- Seldaek [moved a bunch of iter
  stuff](https://github.com/mozilla/rust/pull/7474) to `extra`.

## Notable compiler additions, bugfixes, and cleanup

- doener [removed an extra layer of
  indirection](https://github.com/mozilla/rust/pull/7452) that method calls incurred.
- Blei [fixed a codegen problem](https://github.com/mozilla/rust/pull/7457)
  with structs containing `f32` when used with FFI.
- I [propagated the great renaming](https://github.com/mozilla/rust/pull/7468)
  throughout the rest of the codebase (besides compiletest, apparently).
- acrichto [rewrote some str code](https://github.com/mozilla/rust/pull/7465)
  to avoid allocations.
- strcat is [removing](https://github.com/mozilla/rust/pull/7495) headers
  from exchange allocs (see also
  [#7605](https://github.com/mozilla/rust/pull/7605) and
  [#7521](https://github.com/mozilla/rust/pull/7521)). They are entirely
  unused, they just need to be removed and the fallout fixed throughout the
  compiler.
- yjh0502 [fixed a bug](https://github.com/mozilla/rust/pull/7443) that
  allowed duplicate struct fields (like `struct Foo {a: uint, a: uint}`)
- acrichto [turned on](https://github.com/mozilla/rust/pull/7409) LLVM
  threading.
- Luqman [changed configure](https://github.com/mozilla/rust/pull/7498) to
  require either wget *or* curl.
- Dretch [improved the error
  message](https://github.com/mozilla/rust/pull/7510) for implementing unknown
  traits to mention the trait name.
- sankha93 [improved the error
  message](https://github.com/mozilla/rust/pull/7531) for trying to capture
  environment in a plain `fn`.
- bblum [improved the error mssage](https://github.com/mozilla/rust/pull/7534)
  for using a moved value, it now gives better suggestions than just `copy`.
- sanxiyn [fixed a bug](https://github.com/mozilla/rust/pull/7543) where
  eligible newtype structs weren't marked as an immediate value (and thus not
  passed in registers when they could have been).
- Luqman [paved the way](https://github.com/mozilla/rust/pull/7547) for 64-bit
  windows support.
- jensnockert [added byte swapping
  intrinsics](https://github.com/mozilla/rust/pull/7194) that specialize
  per-platform, avoiding unnecessary operations.
- jld [removed an unused function](https://github.com/mozilla/rust/pull/7554)
- sully [fixed more default method bugs](https://github.com/mozilla/rust/pull/7545).

## Notable library additions, bugfixes, and cleanup
- sfackler [fixed up some
  documentation](https://github.com/mozilla/rust/pull/7549) related to the
  drop/finalize renaming.
- acrichto [fixed a correctness
  bug](https://github.com/mozilla/rust/pull/7530) in TreeMap's `Ord`
  implementation.
- sfackler [much improved](https://github.com/mozilla/rust/pull/7513) and
  genericized the base64 handling.
- graydon did a [bunch of cleanup](https://github.com/mozilla/rust/pull/7518)
  in `extra::stats`.
- Seldaek [fixed a patological case with
  `str::each_split_within`](https://github.com/mozilla/rust/pull/7475).


# Meetings

The [Tuesday meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-07-02)
featured more discussion about `@` and `@mut`, that was honestly over my head
(as most of the type system stuff is). If someone wants to write some more
here, feel free to email me (<corey+blog@octayn.net>) a paragraph or two. You
will get attribution, of course.

# Discussion + Blog posts

- [The State of Rust 0.7](http://cmr.github.io/blog/2013/07/05/the-state-of-rust/)
- ["Rust switches to external iteration" (D forums)](http://forum.dlang.org/thread/kr2vpp$2jmf$1@digitalmars.com)
- [Segmented stacks](https://mail.mozilla.org/pipermail/rust-dev/2013-July/004686.html)
- [Is Rust Slim Yet? (Is Rust Fast Yet v2)](http://huonw.github.io/isrustfastyet/mem/)
- [Rust Design Patterns](http://joshldavis.com/rust-design-patterns/)
- [Program to an Interface, Fool](http://joshldavis.com/2013/07/01/program-to-an-interface-fool/)
- [Would You Bet $100,000,000 on [Rust]?](http://www.reddit.com/r/rust/comments/1hg88c/i_think_i_would_take_that_for_rust_when_its_done/)
- [mw's third status report](http://michaelwoerister.github.io/2013/06/28/Status-Update-3.html)
