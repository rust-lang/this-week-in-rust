Title: This Week in Rust 53
Date: 2014-06-22 13:40
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

This week, it was anounced that Steve Klabnik [has been hired to work
exclusively on our
documentation](http://www.reddit.com/r/rust/comments/28bew8/rusts_documentation_is_about_to_drastically/),
starting tomorrow.

<!-- more -->

# What's cooking on master?

73 pull requests were merged in the last week.

## Breaking Changes

The complete breaking change log is available
[here](https://gist.github.com/cmr/d0e6d145af65e6d74713), and you can view it
with `git log --no-merges --grep 'breaking-change' --since 6/14/2014 --until 6/21/2014`.

## Other Changes

- Cross compiling to iOS [is now
  supported](https://github.com/rust-lang/rust/pull/14715).
- The borrow checker [has seen a bunch of
  cleanup](https://github.com/rust-lang/rust/pull/14947) removing the obsolete
  notion of "restrictions".
- Byte strings and byte literals [have been added to the
  language](https://github.com/rust-lang/rust/pull/14880).
- Dataflow [uses the new CFG](https://github.com/rust-lang/rust/pull/14873)
  rather than a syntax-based analysis.
- The non-exhaustive pattern error [now gives a pattern which is not
  matched](https://github.com/rust-lang/rust/pull/14731).
- The stability index [has seen some
  extension](https://github.com/rust-lang/rust/pull/15029).

## New Contributors

- Christopher Bergqvist
- Conrad Kleinespel
- John Schmidt
- Nathan Typanski
- Niklas Koep

# New RFCs

- [Revised UFCS performance](https://github.com/rust-lang/rfcs/pull/132)
- [Flexible target specification](https://github.com/rust-lang/rfcs/pull/131)
- [Remove special treatment of Box by borrow
  checker](https://github.com/rust-lang/rfcs/pull/130)
- [Refine the `asm!` extension](https://github.com/rust-lang/rfcs/pull/129)
- [Rename mod.rs files to self.rs](https://github.com/rust-lang/rfcs/pull/128)
- [Opt-in builtin traits take 2, default and negative
  impls](https://github.com/rust-lang/rfcs/pull/127)
- [Add optional type parameter to
  `include_bin!`](https://github.com/rust-lang/rfcs/pull/126)
- [Add prefetch intrinsics](https://github.com/rust-lang/rfcs/pull/125)
- [Add `cloned` and `stable`
  keywords](https://github.com/rust-lang/rfcs/pull/124)
- [Rename `Share` to `Threadsafe`](https://github.com/rust-lang/rfcs/pull/123)
- [Syntax sugar for prefix-style type parameter
  lists](https://github.com/rust-lang/rfcs/pull/122)

# Community Updates

- [Snowmew's architecture part 2: data
  management](http://csherratt.github.io/csherratt/blog/2014/06/22/snowmews-architecture-part-2/)
- [`rust-osc`](http://www.reddit.com/r/rust/comments/2828nq/rustosc_open_sound_control_10_over_udp_in_rust/),
  Open Sound Control 1.0 over UDP in Rust.
- [Piston Game Engine: Progress
  Update](http://www.reddit.com/r/rust/comments/28srso/piston_game_engine_notice_on_progress/)
- [`Checked<T>`](https://gist.github.com/Florob/0ec238fa00a0c9b40bf7), a type
  for more ergonomic checked integer arithmetic.
- [Weekly meeting
  notes](http://www.reddit.com/r/rust/comments/28exbu/meetingweekly20140617_rfcs_unsafe_fields_loadable/)
- [`rust-empty` 0.5
  released](http://www.reddit.com/r/rust/comments/28cu3g/rustempty_05_released_compile_and_run_on_file/)



# This Week in Servo
Servo is a web browser engine written in Rust and is one of the primary test cases for the Rust language.

In the last two weeks, we landed 53 PRs.

## Notable additions

- Brian Anderson landed parallel renedering
- Lars Bergstrom and Manish Goregaokar changed Servo to use a prebuilt Rust compiler (reducing build times from hours to < 10 minutes!) and enabled Travis CI builds
- ms2ger added support for rapidly failing the script task when the JS engine goes OOM
- fdipilla added support for spaces in paths in our configure scripts
- Tetsuharu OHZEKI added support for internal mutability for many pieces of code in Servo
- Martin Robinson added overflow support to child layer sizes
- Matt Brubeck separated desktop and mobile zoom calculations
- schaars ensured that noscript elements are not displayed
- Bruno Abinader implemented querySelectorAll
- Luqman Aden fixed up build failures on Android
- Glenn Watson added a Rust string interning repo (though he declined to name it "yarnbox")

## New Contributors
Pierre Louis Aublin (schaars)
fdipilla

## Meetings and Notes

There were meetings [two weeks ago](https://github.com/mozilla/servo/wiki/Meeting-2014-06-09) discussing the next workweek and build system issues and [last week](https://github.com/mozilla/servo/wiki/Meeting-2014-06-17) about the move to Travis CI, embedding support, and the web platform tests.
