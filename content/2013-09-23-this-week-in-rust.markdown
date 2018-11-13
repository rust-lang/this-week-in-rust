Title: This Week in Rust 16
Date: 2013-09-23 13:08
Category: This Week in Rust
Tags: programming, rust

Welcome to another issue of *This Week in Rust*. We're gearing up for a 0.8
release, tentatively planned for Thursday. Additionally, a huge welcome of
Alex Crichton to the Rust team! He's been doing some great work, it's
wonderful to have another full-time Rust dev.

<!-- more -->

# What's cooking in master?

There were 86 PRs merged this week.

## rustdoc\_ng

rustdoc\_ng was merged into mainline and enabled as the new rustdoc, in time
for 0.8. This makes me incredibly happy, as it marks the conclusion of my
quest to a better rustdoc. Not to say that rustdoc is finished or perfect,
but it's already much better than the old rustdoc. My sincere thanks go to
Jordi Boggiano (Seldaek), who worked unceasingly to create a beautiful,
usable frontend, and to Meret Vollenweider (meretv) for donating her amazing
design skills. Also thanks to Huon Wilson (dbaupp) for moral support and
creating the awesome DocFolder interface that passes over the documentation
use. And, of course, to Alex Crichton, who pushed these past few days to port
the static site generator to Rust and to integrate it into the build system.

The new docs have replaced the old docs. You can view them [on the Rust
website](http://static.rust-lang.org/doc/master/std/index.html). Still to-do
is proper cross-crate doc hyperlinking. To run it on your own crate, build a
recent Rust and do `rustdoc html path/to/your/crate.rs`. It will eventually be
integrated into `rustpkg`.

## Breaking changes

- `extra::par` has been [removed](https://github.com/mozilla/rust/pull/9380).
- Some unsound functional struct updates (FSU, `{a: 5, ..b}`) [are now
  disallowed](https://github.com/mozilla/rust/pull/9350).
- `--cfg debug` is no longer required. If you want to disable debug logging,
  use [`--cfg ndebug`](https://github.com/mozilla/rust/pull/9278).
- The various uses of `NaN` are now [lowercased to
  `nan`](https://github.com/mozilla/rust/pull/9321).
- `std::util::unreachable` has been removed in factor of the
  [`unreachable!`](https://github.com/mozilla/rust/pull/9320) macro.
- `extra::future` has been [cleaned
  up](https://github.com/mozilla/rust/pull/9285) a bit, including some method
  shuffling/renames.
- `extra::getopts` has been [cleaned
  up](https://github.com/mozilla/rust/pull/9267) as well, with most free
  functions now being methods.
- The `from_str` methods in the numeric modules have been
  [removed](https://github.com/mozilla/rust/pull/9275) in favor of the FromStr
  trait and the `from_str` free function in the prelude. Same for
  [`from_str_radix`](https://github.com/mozilla/rust/pull/9209).
- The `Drop` trait now uses [`&mut
  self`](https://github.com/mozilla/rust/pull/9244), as part of the transition
  to by-value drops.
- `extra::json` uses a [different
  encoding](https://github.com/mozilla/rust/pull/9231) when encoding/decoding
  enums, so any Rust-generated JSON before this patch will now be rejected by
  the decoder.

## Other changes

- debuginfo now works for [recursive
  types](https://github.com/mozilla/rust/pull/9168).
- You can now [pass parameters](https://github.com/mozilla/rust/pull/9213) to
  the generated test with `rust test`.
- `extern fn`s from external crates now use the [declared
  ABI](https://github.com/mozilla/rust/pull/9196), rather than assuming cdecl.
- `CString` has gained an
  [`as_str`](https://github.com/mozilla/rust/pull/9220) method.
- Some [overflow bugs](https://github.com/mozilla/rust/pull/9108) in `vec` and
  `str` have been fixed.
- `statics` are now [properly
  inlined](https://github.com/mozilla/rust/pull/9130) cross-crate.
- The `bytes!` macro's error reporting has been [tightened
  up](https://github.com/mozilla/rust/pull/9245) to make it more obvious where
  the error is.
- `rustpkg init` [has been added](https://github.com/mozilla/rust/pull/9236),
  to create a new workspace.
- File IO in newrt has been [massively
  reworked](https://github.com/mozilla/rust/pull/9235) and is much more
  complete than the oldrt.
- The exact meaning of "unsafety" [is now
  documented](https://github.com/mozilla/rust/pull/9258).
- `\0` escapes in strings [is now
  supported](https://github.com/mozilla/rust/pull/9248).
- `rustpkg` now thinks in terms of crates, not packages, to you can now have
  [multiple crates in a single
  package](https://github.com/mozilla/rust/pull/9263) and have things work
  correctly.
- newrt has [pipes and process
  support](https://github.com/mozilla/rust/pull/9260/files) now.
- `format!` now allows [trailing
  commas](https://github.com/mozilla/rust/pull/9299) in its arguments.
- The lexer now throws [vastly better
  errors](https://github.com/mozilla/rust/pull/9308).

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-09-17)
discussed quite a bit, most interesting to me was the crypto discussion,
putting `macro_rules!` behind an experimental flag, and the default arguments
discussion.

# Project announcements etc

- [rustymem](https://github.com/williamw520/rustymem) - a pure-rust memcached
  library.
- ["Where to learn more about Rust's concurrency
  model?"](http://www.reddit.com/r/rust/comments/1myesy/where_to_learn_more_about_rusts_concurrency_model/)
- [q3 now has skeletal
  animation!](https://raw.github.com/jeaye/q3/master/pics/016_1_animated_skele.png).
