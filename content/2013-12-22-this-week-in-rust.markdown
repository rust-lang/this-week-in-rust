Title: This Week in Rust 29
Date: 2013-12-22 06:57
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)

This week brings a bunch of optimizations, cleanups, and the `std::comm`
rewrite. The `pkgid` attribute has also seen some changes.

<!-- more -->

This week also brings a bunch of tiny bugfixes, many of which I do not mention
below. Out of the 52 PRs this week, I only mention 16. The decision to include
a PR or not is fairly arbitrary, but I try to maintain a "significance
barrier" so that the list is not huge and hard to read (and also to save me
work!). Examples of things which I will stop including mentions of are test
fixes, minor documentation changes, build infrastructure fixes, and any
cleanup to the compiler that doesn't result in a relatively significant
speedup or memory usage decrease. I don't do this out of malice, but out of
laziness and convenience. `<3`

# What's cooking on master?

52 PRs were merged this week.

## Breaking Changes

- `std::comm` [has been
rewritten](https://github.com/mozilla/rust/pull/10830). In short, use
`Chan::new` and `SharedChan::new` to construct a Port/Chan pair. We're in the
ballpark of Go's performance, which is exciting. Read the PR for more details.
- Some more `std::vec::raw`
[cleanup](https://github.com/mozilla/rust/pull/10996) has turned some free
functions into methods. `.as_mut_buf` and friends [have also been
removed](https://github.com/mozilla/rust/pull/11029), replaced by using
`.as_ptr`, `.as_mut_ptr`, and `.len`.
- `extra::ebml` [has been partially
de-@d](https://github.com/mozilla/rust/pull/11057).
- A ton of useless traits, `ToOption` etc,  [have been
removed](https://github.com/mozilla/rust/pull/10967).
- A nasty bug where having an `impl` for a type forced it to be public [has
been fixed](https://github.com/mozilla/rust/pull/11019). Some types which were
previously accessible may not be anymore, due to this.
- The `pkgid` crate attribute [has been
renamed](https://github.com/mozilla/rust/pull/11041) to `crate_id`.
- `extra::sort` [has been
removed](https://github.com/mozilla/rust/pull/11064), and there is now a
`sort` method on mutable slices (`&mut [T]`).

## Other Changes

- The `remove` and `insert` methods on vectors have had some unsafe code added
to make them [3x faster](https://github.com/mozilla/rust/pull/11061).
- The `sum` method on `extra::stat::Stat` [has been made more
accurate](https://github.com/mozilla/rust/pull/10927).
- debuginfo for by-value `self` [has been
fixed](https://github.com/mozilla/rust/pull/11033).
- `return` is now [allowed in
closures](https://github.com/mozilla/rust/pull/11024) for early return.
- `crate_id` (then `pkgid`) [now
supports](https://github.com/mozilla/rust/pull/10972) specifying the crate
name. `#foo:1.0` as the fragment will make the crate name `foo`, for example
`gl` in `github.com/bjz/gl-rs#gl:1.0`.
- A `Pod` kind [has been added](https://github.com/mozilla/rust/pull/10924)
for types where a memcpy is a logical copy. This behaves very similarly to the
old `Copy` kind.
- Stability attributes (`#[deprecated]`, `#[experimental]`, etc) [are now
checked for method calls](https://github.com/mozilla/rust/pull/10990).
- Metadata reading has been made [4x
faster](https://github.com/mozilla/rust/pull/11017).
- msys terminals are [now
supported](https://github.com/mozilla/rust/pull/11031) by `extra::term`.

## New Contributors

- Carter Tazio Schonwald

# Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-12-17)
discusses some pull requests, `pkgid`, renaming `extern mod`, the stdlib
module hierarchy, and renaming some of the vector methods.

# This Week in Servo
Servo is a web browser engine written in Rust and is one of the primary test
cases for the Rust language.

We landed 19 PRs this week.

## Notable additions
- Pradeep Kumar re-enabled parallel selector matching
[#1429](https://github.com/mozilla/servo/pull/1429).
- Daniel Hedlund fixed window redraw on Linux
[#1414](https://github.com/mozilla/servo/pull/1414).
- Tetsuharu Ohzeki significantly cleaned up the compositor source code in
[#1425](https://github.com/mozilla/servo/pull/1425).
- Simon Sapin added support for non-UTF8 stylesheets
[#1377](https://github.com/mozilla/servo/pull/1377).
- Patrick Walton cleaned up made several PRs that clean up layout's dependency
on DOM nodes.
- Isabelle Carter added support for positioned offsets for layout in
[#1407](https://github.com/mozilla/servo/pull/1407).

## Meetings
In this week's
[meeting](https://github.com/mozilla/servo/wiki/Meeting-2013-12-16), we
discussed using the Critic review tool from Opera with github. We are
currently experimenting it, with jgraham's support. Jack also discussed his
ongoing Rust upgrade which, as always, has uncovered several interesting Rust
compiler bugs.

We also sketched out our current planning
[roadmap](https://github.com/mozilla/servo/wiki/Roadmap), along with a cheat
sheet of some of the
[research](https://github.com/mozilla/servo/wiki/Research) topics we're also
investigating.

# Announcements, etc

- The SF meetup happened. It was recorded and [is available on Air
Mozilla](https://air.mozilla.org/rust-meetup-december-2013/). The video is
well worth watching.
- [A broad vision for the Rust docs
stack](http://www.reddit.com/r/rust/comments/1t87a3/a_broad_vision_for_the_rust_docs_stack/).
- [Two bugs in the borrow checker every Rust developer should know
about](http://blog.ezyang.com/2013/12/two-bugs-in-the-borrow-checker-every-rust-developer-should-know-about/).
- [Building Rust Code - Using Make Part
2](http://metajack.im/2013/12/19/building-rust-code--using-make-part-2/).
- [Video: 2D portaling demo in Rust and
rust-sdl2](http://www.reddit.com/r/rust/comments/1tdw88/video_2d_portaling_demo_in_rust_rustsdl2/).
- [What do you want in a Rust
book?](http://www.reddit.com/r/rust/comments/1teet8/what_do_you_want_in_a_rust_book/).
- [An etched copper Rust
logo](http://www.reddit.com/r/rust/comments/1tcat1/i_etched_myself_a_rust_logo/).
- [Ohcount Rust support](https://github.com/blackducksw/ohcount/pull/30) has
been merged, so hopefully ohloh will have more accurate information for Rust
projects soon.
