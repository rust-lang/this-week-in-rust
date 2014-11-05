Title: This Week in Rust 21
Date: 2013-10-28 16:20
Category: This Week in Rust

Welcome to another issue of *This Week in Rust!* This week marks the addition
of some more feature gates, removal of `std::io`, and some feature proposals.

<!-- more -->

# What's cooking on master?

47 PRs were merged this week, and we passed issue number 10000.

## Breaking Changes

- Most of the crypto in the stdlib [has been
  removed](https://github.com/mozilla/rust/pull/9744).
- `std::io` [has been removed](https://github.com/mozilla/rust/pull/9901),
  with corresponding improvements in `std::rt::io`.
- The `frame_address` intrinsic [has been
  removed](https://github.com/mozilla/rust/pull/10040).
- `@`-ptrs are [now feature-gated](https://github.com/mozilla/rust/pull/9923),
  reflecting the fact that they will exist in a very different form once they
  are finished.
- All of the logging that used `fmt` (`debug!` and so forth) [have been
  transitioned to `format!`](https://github.com/mozilla/rust/pull/10006).
- `asm!` [is also featured gated](https://github.com/mozilla/rust/pull/10009).
- `sys::log_str` [has moved](https://github.com/mozilla/rust/pull/9937).

## Other changes

- The long-awaited addition of `mut` in patterns [has finally
  happened](https://github.com/mozilla/rust/pull/10026). `let (mut x, y) = (1,
  2);` works.
- The scheduler now uses [lock free data
  structures](https://github.com/mozilla/rust/pull/10080), which are supposed
  to perform much better.
- The homing code [now does less
  work](https://github.com/mozilla/rust/pull/10070), which does one third as
  many `write`'s.
- `stdout` [is now buffered](https://github.com/mozilla/rust/pull/10060).
- A [non-libuv event loop](https://github.com/mozilla/rust/pull/10054) has
  been added.
- `std::rand` has seen [more](https://github.com/mozilla/rust/pull/9810)
  [work](https://github.com/mozilla/rust/pull/10015).
- `rustpkg` [now supports arbitrary
  dependencies](https://github.com/mozilla/rust/pull/9654), such as C
  libraries.
- `mut` is [now allowed on self](https://github.com/mozilla/rust/pull/9989).
- Nested comments [are now
  allowed](https://github.com/mozilla/rust/pull/9936).
- `rustpkg` will now [find crates in the current directory], so you can say
  `rustpkg build` in a source dir and it will build in a workspace.
- Some new float intrinsics [have been
  exposed](https://github.com/mozilla/rust/pull/9986).
- `println!` [allocates less](https://github.com/mozilla/rust/pull/9979).

## New contributors

- Igor Bukanov
- Mark Rowe
- Michael Letterle
- reedlepee

# Announcements etc

- There will be a
  [meetup](http://www.meetup.com/Rust-Bay-Area/events/143439552/) in the San
  Fransisco Mozilla office. Patrick Walton will be talking about sprocketnes.
  If you're in the area, you should definitely go!
- [rust-ci](http://hiho.io/rust-ci/) has been created, which lets you have
  travis automatically rebuild and test your repo daily, to reduce
  language/library breakage.
- [rust-mustache](https://github.com/erickt/rust-mustache) and
  [rust-zmq](https://github.com/erickt/rust-zmq) have been updated for master.
- [Short talk about Rust at Scala.IO in Paris (October
  25th)](http://www.reddit.com/r/rust/comments/1pdrv6/short_talk_about_rust_at_scalaio_in_paris_october/).
- [A Bit of Functional Programming in Rust, or A Misguided First Look at Rust
  for ML
	  Programmers](http://www.reddit.com/r/rust/comments/1pe2lc/a_bit_of_functional_programming_in_rust_or_a/).
- [A draft proposal for single
  inheritance](http://www.reddit.com/r/rust/comments/1p52tj/a_draft_proposal_for_single_inheritance_in_rust/).
- [Iterators yielding mutable
  references](http://www.reddit.com/r/rust/comments/1p4vnk/iterators_yielding_mutable_references/).
- [Servo transitions from libcss to a new CSS library written in
  Rust](http://www.reddit.com/r/rust/comments/1p4qeh/servo_transitions_from_libcss_to_a_new_css/).
- [On stack
  safety](http://www.reddit.com/r/rust/comments/1owhwi/on_stack_safety/).
