Title: This Week in Rust 19
Date: 2013-10-12 22:53
Category: This Week in Rust

Welcome to another issue of *This Week in Rust*. This week introduces raw
string literals, the removal of the `rust` tool, feature gating, and the
privacy overhaul.

<!-- more -->

# What's cooking on master?

Only 51 PRs were merged this week, though the ones that were were fairly
large.

## Breaking changes

- Visibility and privacy have been defined, and the new semantics are slightly
  incompatible with the old. The details are
  [here](https://github.com/mozilla/rust/pull/9735).
- Feature gating has been [added to the
  compiler](https://github.com/mozilla/rust/pull/9703). This allows us to
  mask language features that we don't want to commit to for 1.0. The compiler
  will tell you when you are using an "experimental" feature. The current
  experimentals are glob imports, macro definitions, and struct-like variants
  in enums. This is an important step forward for 1.0 which,
  [apparently](http://www.reddit.com/r/rust/comments/1o90f9/fn_types_in_rust_take_3/ccq8qev),
  is right around the corner.
- The `rust` tool [has been
  removed](https://github.com/mozilla/rust/pull/9785).
- `Option`'s API has been [massively
  simplified](https://github.com/mozilla/rust/pull/9359), and changed to be
  composable.
- Strings are [no longer allowed to be modified in safe
  code](https://github.com/mozilla/rust/pull/9750). Things like `let s =
  ~"abcdefg"; s[4] = 0x99;` are now rejected. This is because it allows one to
  create invalid (non-UTF8) string values.
- `extra::rc` has [graduated to
  `std::rc`](https://github.com/mozilla/rust/pull/9794).
- `extra::tempfile::mkdtemp` [has been replaced with an RAII
  wrapper](https://github.com/mozilla/rust/pull/9802).
- `std::rand` [has been improved](https://github.com/mozilla/rust/pull/9695),
  but changes the public API of Rng a bit.
- `IntConvertible` has been removed in favor of [`ToPrimitive` and
  `FromPrimitive`](https://github.com/mozilla/rust/pull/9250), which can be
  derived for enums, making interacting with C APIs substantially more
  [DRY](http://en.wikipedia.org/wiki/Don%27t_repeat_yourself).

## Other changes

- [Raw strings](https://github.com/mozilla/rust/pull/9674) are now supported.
  The basic syntax is `r"Foo"`, but also `r###"Foo"###`, for arbitrary
  nesting of raw strings.
- rustpkg's remote fetching [is
  fixed](https://github.com/mozilla/rust/pull/9741), but [other
  ](https://github.com/mozilla/rust/issues/9482)
  [problems](https://github.com/mozilla/rust/issues/9781) prevent it from
  being fully usable right now.
- debuginfo namespace handling [has been
  unified](https://github.com/mozilla/rust/pull/9658), and marks the ability
  to build all of Rust (libstd, libextra, compiler and tools) with debuginfo,
  which is very awesome.
- Small structs [are immediate](https://github.com/mozilla/rust/pull/9759).
- When there are unbalanced delimiters, [the opening delimiters leading up to
  it](https://github.com/mozilla/rust/pull/9756) are printed, avoiding the
  "Expected a }, saw EOF at line 9001" problem.
- libuv and jemalloc are only [built once, rather than once per
  stage](https://github.com/mozilla/rust/pull/9772).
- Macros [now take attributes](https://github.com/mozilla/rust/pull/9753) and,
  in turn, can be documented.
- In the opposite direction, [macros can now expand to items with
  attributes](https://github.com/mozilla/rust/pull/9783).
- There were some [minor logging
  changes](https://github.com/mozilla/rust/pull/9664), the biggest result of
  which is that inlined functions won't be logged by the modules they're
  inlined into.
- [`std::rt::io::native` has been
  implemented](https://github.com/mozilla/rust/pull/9749). The idea of this is
  to live in `std::io::native` and to use the OS's native APIs rather than
  libuv, and also to not require the scheduler or any other runtime support.
- rustpkg [marks checked out repos as
  read-only](https://github.com/mozilla/rust/pull/9732), to prevent
  modifications.

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-10-08)
discussed some administrative things, the removal of crypto code, the removal
of the `rust` tool, and functions.

# Announcements etc

- [rust-fuse](http://www.reddit.com/r/rust/comments/1o35ns/fuse_userspace_library_in_rust/)
  \- a FUSE userspace library in Rust. This is pure Rust, not using libfuse.
- [ears](http://www.reddit.com/r/rust/comments/1o46cv/ears_a_simple_library_for_playing_sounds/)
  \- a simple library for playing sounds.
- [rustic](https://github.com/pcmattman/rustic) - another operating system
  project.
- [Refactoring the milestones on the issue
  tracker](https://mail.mozilla.org/pipermail/rust-dev/2013-October/005981.html).
- [Fn Types in Rust, Take
  3](http://smallcultfollowing.com/babysteps/blog/2013/10/10/fn-types-in-rust/)
  \- another attempt at tackling the problems with closures.
- mcpherrin in `#rust` says "Hello, everyone!"
