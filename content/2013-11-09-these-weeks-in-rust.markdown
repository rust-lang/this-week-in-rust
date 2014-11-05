Title: These Weeks in Rust 22
Date: 2013-11-09 05:21
Category: This Week in Rust

Welcome to a mega-*This Week in Rust*. I was swamped this past week with
schoolwork, so TWiR was put off. This week's combines the past two weeks of
progress. These past two weeks were fairly exciting in terms of language and
library progress. The next few weeks should be even more exciting.

<!-- more -->

# What's cooking on master?

There were 108 PRs merged these past two weeks.

## Breaking Changes

- The very long-awaited [enum discriminant size
  patch](https://github.com/mozilla/rust/pull/9613) has landed. This will
  affect FFI. Size of enum discriminant is now configurable via the `repr`
  attribute, and will by default shrink to the smallest needed.
- The extension traits for `Reader` and `Writer` have [been transformed into
  default methods on their respective
  trait](https://github.com/mozilla/rust/pull/10079).
- Non-string literals are now
  [disallowed](https://github.com/mozilla/rust/pull/10166) in attributes.
- Type parameters are now
  [forbidden](https://github.com/mozilla/rust/pull/10189) on inner statics
  (statics inside functions).
- The interface to flush stdout [has
  changed](https://github.com/mozilla/rust/pull/10218). It was previously
  unsound by allowing aliased `&mut`.
- `Result`'s API has changed [quite a
  bit](https://github.com/mozilla/rust/pull/10119), to be more consistent with
  `Option`, and hopefully simpler.
- Linker arguments [no longer](https://github.com/mozilla/rust/pull/10199)
  propagate across crates. This means that if you link to a crate, its linker
  arguments won't be automatically added when your crate is linked.
- The memory intrinsics [have been
  simplified](https://github.com/mozilla/rust/pull/10251). A single intrinsic
  for `memcpy`/`memmove`/`memset` is now exposed, rather than one per
	  platform.
- `#[link(name = "...")]` is now [taken into
  account](https://github.com/mozilla/rust/pull/10260) by rustc when creating
  build artifacts.
- `std::rt::io::file` [has been fleshed out and
  tweaked](https://github.com/mozilla/rust/pull/10179). In particular, it has
  been renamed to `std::rt::io::fs`, many previously-free functions are now
  associated functions on `std::rt::io::File`, and `FileInfo` has been renamed
  to `FileStat`.

## Other Changes

- Calling variadic functions with the C FFI [is now
  implemented](https://github.com/mozilla/rust/pull/10064). This is a pretty
  sweet change. The only thing missing in our C FFI now is unions.
- We [now have](https://github.com/mozilla/rust/pull/10243) octal numeric
  literals, for all your esoteric numeric needs!
- An `Any` type [has been added](https://github.com/mozilla/rust/pull/9967),
  and it is now possible to retrieve the object a task failed with. Previously
  tasks could only fail with a string, now they can fail with anything.
- A `concat!` syntax extension [has been
  added](https://github.com/mozilla/rust/pull/9740) for compile-time string
  concatenation.
- Timers are [now also ports](https://github.com/mozilla/rust/pull/10083), and
  the creator of a timer can cancel it.
- As the first part of closure reform, `proc` is [now sugar](https://github.com/mozilla/rust/pull/10132) for `~once
  fn`, and `|A| -> B` (and `fn(A) -> B` for bare functions) [are now
  allowed](https://github.com/mozilla/rust/pull/10187) in types.
- The section in the tutorial on vectors and strings [has been
  rewritten](https://github.com/mozilla/rust/pull/10354) for correctness with
  modern Rust.
- A bunch of C++ has been removed and rewritten. [Thread
  creation](https://github.com/mozilla/rust/pull/10290), [memory
  regions](https://github.com/mozilla/rust/pull/10094) (used for debugging and
  `@`-boxes, from what I can tell), and an [unused
  `array_list`](https://github.com/mozilla/rust/pull/10163/files).
- Bounds check failures are [now marked as a cold
  path](https://github.com/mozilla/rust/pull/10113), and a `cold` [function
  attribute](https://github.com/mozilla/rust/pull/10127) has been added.
- The build system [can cross-compile to iOS
  now](https://github.com/mozilla/rust/pull/10203), even though Rust doesn't
  actually run on that platform (yet!).
- `std::rand` [now implements the Gamma
  distribution](https://github.com/mozilla/rust/pull/10223).
- Cross-crate destructor inlining [now
  works](https://github.com/mozilla/rust/pull/10242).
- A `type_id` intrinsic [has been
  added](https://github.com/mozilla/rust/pull/10182).
- Everything in the runtime that uses `libuv` has been [split into its own
  crate](https://github.com/mozilla/rust/pull/10058). This means that the
  runtime really is pluggable: you can implement your own event loop and so
  forth.

## New Contributors

Welcome to our new contributors!

- Brian
- Carol Willing
- Dirkjan Bussink
- Guillaume Pinot
- Gyorgy Andrasek
- Joshua Yanovski
- Mat Carberry
- Noufal Ibrahim
- Robert Irelan
- Tomas Sedovic
- Jennifer Ward
- Patrick Kim

At .85 new contributors a day, we'll soon dwarf every other language in the
"awesome volunteer" category.

# Weekly Meetings

Last week's
[meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-10-29)
discussed segmented stacks (spoiler: [they're not coming
back](https://mail.mozilla.org/pipermail/rust-dev/2013-November/006314.html) )
and placement new (we want it, how do we want it?).

This week's
[meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-11-05)
discussed the future of libextra, more stack things, octal literals, vector
representation, and temporary ("rvalue") lifetimes.

# Announcements etc

- **Reminder from ~~the Ministry of Truth~~ ChrisMorgan**: Rust is awesome.
- Rust Skåne, [has an event page
  now](http://www.foocafe.org/event/a-friendly-introduction-to-rust). It will
  be December 3 at 17:30 in Foo Cafe.
- [Integermingled Parameter
  Lists](http://smallcultfollowing.com/babysteps/blog/2013/10/29/intermingled-parameter-lists/),
  and [take
  2](http://smallcultfollowing.com/babysteps/blog/2013/11/04/intermingled-parameter-lists/).
- [speculate](https://github.com/haxney/speculate) - a parallel speculative
  execution library.
- [mcchat](https://github.com/luqmana/mcchat) - a pure-Rust Minecraft chat
  client.
