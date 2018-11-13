Title: This Week in Rust 23
Date: 2013-11-19 01:42
Category: This Week in Rust

Welcome to another issue of *This Week in Rust*.

<!-- more -->

# What's cooking on master?

46 PRs were merged this week.

## Breaking changes

- `std::rt::io` [has moved on](https://github.com/mozilla/rust/pull/10423) to
  become `std::io`. Yay!
- The `#[fixed_stack_segment]` attribute [has been
  removed](https://github.com/mozilla/rust/pull/10407).
- `as` casts to functions are [no longer
  allowed](https://github.com/mozilla/rust/pull/9788).
- The return value of the `io::process` is no longer an int, but [an enum that
  also represents termination by
  signal](https://github.com/mozilla/rust/pull/10109). It's also ostensibly
  more portable: to check if a subprocess succeeded, just call `.success()` on
  the return value.
- `std::cast::unsafe_copy` has [has been
  removed](https://github.com/mozilla/rust/pull/10497). It is identical to
  `std::ptr::read_ptr`.
- Trailing `::` is [no longer
  allowed](https://github.com/mozilla/rust/pull/10420) in paths

## Other Changes

- Vectors have [more overflow
  checking](https://github.com/mozilla/rust/pull/10417). This was the cause
  of a few mysterious segfaults.
- Crate maps [can now be generated for
  libraries](https://github.com/mozilla/rust/pull/10422), for when you want to
  embed a Rust library in a non-Rust application.
- Creation of buffered readers/writers [has been optimized a
  bit](https://github.com/mozilla/rust/pull/10424).
- If you use OS X, you'll be happy to see that the frivolous "no debug symbols
  in executable" warning [has been stomped
  out](https://github.com/mozilla/rust/pull/10198).
- The missing-documentation lint is [now more
  accepting](https://github.com/mozilla/rust/pull/10277), and won't warn about
  private items.
- `BufWriter`, an implementation of Writer for already-existing buffers, [is
  now implemented](https://github.com/mozilla/rust/pull/10451).
- The native IO backend has seen [a good
  boost](https://github.com/mozilla/rust/pull/10457); it will now be
  seamlessly fallen back to when the libuv backend isn't available (you can
  test this out by adding `#[no_uv];` to your crates).
- A lint for numeric literals which overflow their type [has been
  implemented](https://github.com/mozilla/rust/pull/10018).
- A `Buffer` trait [has been
  added](https://github.com/mozilla/rust/pull/10466) to `std::io`, with
  `read_line`, `read_until`, and `read_char`.
- The error message when the trait's method declaration and the method
  declaration in the implementation aren't the same [now includes the trait
  name](https://github.com/mozilla/rust/pull/10509).
- The libuv bindings were [largely
  rewritten](https://github.com/mozilla/rust/pull/10321) for performance, at
  the sacrifice of some flexibility (which was of questionable value).
- `_`-prefixed variables [no
  longer](https://github.com/mozilla/rust/pull/10518) get an unused `mut`
  warning.

## New Contributors

Our first-time contributors this week are:

- Jaemin Moon
- Jay Anderson
- Joe Schafer
- Matthew Iselin
- Zach Kamsler

# Weekly Meeting

There was no meeting this week, as a bunch of the core developers were in
South Korea for the Servo workweek with Samsung.

## Announcements etc

- [(ML) Rethinking Linking in
  Rust](https://mail.mozilla.org/pipermail/rust-dev/2013-November/006686.html)
- [(ML) The future of M:N
  threading](https://mail.mozilla.org/pipermail/rust-dev/2013-November/006550.html)
- [(ML) Type system
  thoughts](https://mail.mozilla.org/pipermail/rust-dev/2013-November/006714.html)
- [(ML)Changing
  roles](https://mail.mozilla.org/pipermail/rust-dev/2013-November/006431.html),
  Tim's "going-away" post. He did great work on rustpkg, and I hope he finds
  time to continue to contribute to Rust.
- [Booting to Rust (with
  UEFI)](http://blog.theincredibleholk.org/blog/2013/11/18/booting-to-rust/)
- [Treating Vectors Like Any Other
  Container](http://smallcultfollowing.com/babysteps/blog/2013/11/14/treating-vectors-like-any-other-container/)
- [Moving forward with Rust
  website](http://adrientetar.legtux.org/blog_fr.php?post=0)
