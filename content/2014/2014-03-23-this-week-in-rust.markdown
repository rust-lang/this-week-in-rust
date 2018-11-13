Title: This Week in Rust 42
Date: 2014-03-24 23:33
Category: This Week in Rust


Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

This weeks flips the switch from libgreen-by-default to libnative-by-default.

<!-- more -->

# What's cooking on master?

91 pull requests were merged this week, topping the [previous
record](http://cmr.github.io/blog/2014/02/23/this-week-in-rust/) of 89.

## Breaking Changes

- Logging [has been extracted to its own
  crate](https://github.com/mozilla/rust/pull/12791). Using the logging macros
  (`debug!` etc) now requires a `#[phase(syntax, link)] extern crate log;`.
- The `AtomicFlag` type [has been
 removed](https://github.com/mozilla/rust/pull/12951).
- `std::vec` [has been renamed](https://github.com/mozilla/rust/pull/12772) to
  `std::slice`, and the old `std::vec_ng` [moved into its
  place](https://github.com/mozilla/rust/pull/13028).
- A `Share` built-in trait [has been
  added](https://github.com/mozilla/rust/pull/12686). It also adds an
  `Unsafe<T>` type, which is now the *only* "safe" way of achieving internal
  mutability (what the `Cell` types etc do). This `Share` trait means that the
  type is thread-safe. See the pull request for more details. In particular,
  taking the address of a static who contains an `Unsafe` member is not
  allowed.
- `std::cast::transmute_immut_unsafe` [has been
  removed](https://github.com/mozilla/rust/pull/13015) because it is
  expressible by safe code.
- `libnative` [is now the default
  runtime](https://github.com/mozilla/rust/pull/12833). In the process, the
  `#[no_uv]` attribute was renamed to `#[no_start]`.
- Atomics [no longer require mutable references to
  change](https://github.com/mozilla/rust/pull/13036), due to the work on
  `Share`, they can safely use interior mutability. Additionally, the generic
  atomics have been hidden and `AtomicU64` has been removed, because not all
  targets support it.
- The `push_bytes` and `read_bytes` methods on `Reader` and `Writer` [have
  been renamed](https://github.com/mozilla/rust/pull/12907) for clarity.
- `RefCell::with` and `RefCell::with_mut` [have been
  removed](https://github.com/mozilla/rust/pull/13052), because the Deref
  changes make them unnecessary.
- The `get` methods on `Ref` and `RefMut` (helpers returned by `RefCell`)
  [have been removed](https://github.com/mozilla/rust/pull/13053), because
  the Deref changes make them unnecessary.
- The `Freeze` trait [has been
  removed](https://github.com/mozilla/rust/pull/13076)
- `std::managed` [has been
  removed](https://github.com/mozilla/rust/pull/13089) from the public API.
- `HashMap` [now correctly uses
  `TotalEq`](https://github.com/mozilla/rust/pull/13088).
- The `to_owned_vec` method on `Iterator` [has been
  removed](https://github.com/mozilla/rust/pull/13090).
- The `equals` method [has been
  removed](https://github.com/mozilla/rust/pull/13102) from `TotalEq`, due to
  the recent changes in `TotalEq`'s semantics.
- Trait implementations may [no
  longer](https://github.com/mozilla/rust/pull/13006) implement a method
  twice.

## Other Changes

- Some bugs with cross-crate autoderef [have been
  fixed](https://github.com/mozilla/rust/pull/13087).
- Some Windows issues with non-English locales [have been
  fixed](https://github.com/mozilla/rust/pull/13078).
- Removing two words from a structure in libsyntax [shaved 100MB off the
  librustc compile](https://github.com/mozilla/rust/pull/13016).
- Some well-placed indirection in librustc [shaved 200MB off the librustc
  compile](https://github.com/mozilla/rust/pull/13013).
- Discarding some data after it's no longer useful [shaved another 100MB off
  the librustc compile](https://github.com/mozilla/rust/pull/12770).
- The new attribute syntax [has been
  implemented](https://github.com/mozilla/rust/pull/13037), though the old has
  not yet been replaced.
- `Vec` is [now in the prelude](https://github.com/mozilla/rust/pull/13020),
  as well as the lint for uses of `~[T]` being made allow by default.
- Some false positives for crate searching [have been
  fixed](https://github.com/mozilla/rust/pull/13017).
- A few bugs with struct ABI on x86 [have been
  fixed](https://github.com/mozilla/rust/pull/12762).
- Mutable slices in `static mut` [are now
  allowed](https://github.com/mozilla/rust/pull/12742).
- Some `@` has been removed from rustc, [yielding an 11k line
  patch](https://github.com/mozilla/rust/pull/12735).
- `bigint` [has seen some
  optimization](https://github.com/mozilla/rust/pull/12924), though not major.

The [doc sprint](http://www.meetup.com/Rust-Bay-Area/events/168366122/)
happened last week. A bunch of pull requests for docs for this landed:

- `std::ops` (from the rollup)
- [`time::Tm`](https://github.com/mozilla/rust/pull/12940)
- [`getopts`](https://github.com/mozilla/rust/pull/12942)
- [Endian conversion in
  `std::mem`](https://github.com/mozilla/rust/pull/12944)
- [`std::sync::atomics`](https://github.com/mozilla/rust/pull/12954)
- [`std::vec_ng`](https://github.com/mozilla/rust/pull/12955)
- [`std::option`](https://github.com/mozilla/rust/pull/12982)
- [`term`](https://github.com/mozilla/rust/pull/12948)

## New Contributors

- Eunchong Yu
- Jonathan S
- Jorge Aparicio
- Ms2ger
- Olle Jonsson
- Ryan Scheel (Havvy)

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2014-03-18)
discussed using GitHub for RFCs, the docsprint, and one of the RFCs.

# RFCs

Some new RFCs:

- [Tweaked Variance Inference](https://github.com/rust-lang/rfcs/pull/12)
- Another struct inheritance RFC, [extending
  enums](https://github.com/rust-lang/rfcs/pull/11)
- Another one, [`abstract struct` and `abstract
  enum`](https://github.com/rust-lang/rfcs/pull/10)
- [Unsurprising module imports](https://github.com/rust-lang/rfcs/pull/18)
- [`Iterable` trait family](https://github.com/rust-lang/rfcs/pull/17)
- [Attributes on match arms and
  statements](https://github.com/rust-lang/rfcs/pull/16)
- [SIMD Support](https://github.com/rust-lang/rfcs/pull/15)
- [Opt-in builtin traits](https://github.com/rust-lang/rfcs/pull/19)

# Project Updates

[rustfind](https://github.com/dobkeratops/rustfind), doomlord's code browser
for Rust, [has been updated](http://www.reddit.com/r/rust/comments/20q3uz/rustfind_code_browser/).
This is a very nice librustc-integrated tool that supports jump-to-definition.

Relatedly, I'm reminded of [unfold](https://github.com/dobkeratops/unfold),
doomlord's tool for displaying "brace context" of code. It includes an "rg"
script, which is a simple grep for item definitions.

In leiu of a "This Week in Servo", Servo [has been making
progress](http://imgur.com/a/O667X) on [Acid2](http://acid2.acidtests.org/).

# Other Announcements

- [Memory Management in C Programs](http://nethack4.org/blog/memory.html) --
  this is a very well written article from the nethack team. All the concepts
  discussed within apply to Rust.
- [Dynamically Sized Types in
  Rust](http://blog.babelmonkeys.de/2014/03/18/dst.html)
- [Announcing the new Rust package manager,
  Cargo](https://mail.mozilla.org/pipermail/rust-dev/2014-March/009087.html).
  It's on [github](https://github.com/carlhuda/cargo).
- [Rust is in the next DWARF
  standard!](http://dwarfstd.org/ShowIssue.php?issue=140129.1)
- [A success story of Rust and
  emscripten](http://www.reddit.com/r/rust/comments/20nnkk/rust_and_emscripten_a_small_success/)
- [A Rust meetup group in
  London](http://www.meetup.com/Rust-London-User-Group/)
- [Subtyping and coercion in
  Rust](http://featherweightmusings.blogspot.com.br/2014/03/subtyping-and-coercion-in-rust.html)
