Title: This Week in Rust 32
Date: 2014-01-11 16:23
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

0.9 was released this week, bringing with it a restructuring of our
documentation and the usual influx of attention.

<!-- more -->

# What's cooking on master?

57 pull requests were merged this week.

## Breaking Changes

- Float literals are now [more
strict](https://github.com/mozilla/rust/pull/11480). Octal and hex float
literals are no longer allowed.
- A nasty soundness bug [reported on
reddit](http://www.reddit.com/r/rust/comments/1uxbgm/moving_ownership_via_borrowed_reference/)
[has been fixed](https://github.com/mozilla/rust/pull/11465).
- `print` and `println` [have been removed from the
prelude](https://github.com/mozilla/rust/pull/11416). Use the `print!` and
`println!` macros instead.
- `Rc` [now supports weak
pointers](https://github.com/mozilla/rust/pull/10926/files), and it no longer
requires the wrapped type to be Freeze or Send. That is, it no longer
statically avoids cycles using the type system. It was found to be too
restrictive. The only constructor is now `Rc::new`.
- The `eof` method on `Reader` [has been
removed](https://github.com/mozilla/rust/pull/11376), as only files really
know when they are at eof. It is now a method on the types it makes sense for
[such as `MemReader` and
`BufReader`](https://github.com/mozilla/rust/pull/11437).
- `std::num` is seeing some simplification. [Many traits have been merged into
`Real`](https://github.com/mozilla/rust/pull/11412).
- Similarly, [`ApproxEq` has been
removed](https://github.com/mozilla/rust/pull/11402).
- The `std::io::Decorator` trait [has been
removed](https://github.com/mozilla/rust/pull/11394).
- A nasty resolve bug [has been
fixed](https://github.com/mozilla/rust/pull/11370). Importing a trait made it
available to all child modules; now it is only available in the module that
imported it, as it should be.
- Some [method renaming](https://github.com/mozilla/rust/pull/10854) has been
done to `Option`. `map_default` is now `map_or` and `mutate_default` is now
`mutate_or_set`.

## Other Changes

- A [lint for unnecessary casts](https://github.com/mozilla/rust/pull/11329)
has been added. It is set to allow by default.
- [Mutable iterators](https://github.com/mozilla/rust/pull/11342) have been
added to `std::trie`.
- A bug in rustpkg preventing it from finding any static libraries, and
libnative, [has been fixed](https://github.com/mozilla/rust/pull/11338).
- `u64_from_be_bytes` [has seen some
optimization](https://github.com/mozilla/rust/pull/11448), it is 3-6x faster.
- `box` has been
[kinda-implemented](https://github.com/mozilla/rust/pull/11055). `box(GC)` and
`box(HEAP)` now work.
- Typed arenas [have been added](https://github.com/mozilla/rust/pull/11358).
They're really fast!
- `stderr`/`stdout` is now
[per-task](https://github.com/mozilla/rust/pull/11353), which will allow
capturing or redirecting a task's output.

## New Contributors

- Andrew Chin
- Clinton Ryan
- Derek Chiang
- Mick Koch
- Nick Cameron
- Niels langager Ellegaard
- Nif Ward
- Raphael Catolino
- Rich Lane
- Dzmitry Malyshau


# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2014-01-07)
discussed Windows XP support, bounded channels, the Rc changes, using guard
pages, loadable syntax extensions, and functions that fail.

# Quotes of the Week

"[I] find it easy to get lost in this file." -
[kud1ing](https://github.com/mozilla/rust/pull/11472/files#r8810362) on
`libc.rs`
- "A language is more than just semantics, syntax, and an implementation -
it's about an ecosystem and a community. Without that it dies." -
[bjz](https://botbot.me/mozilla/rust-internals/msg/9689869/)

# This Week in Servo
Servo is a web browser engine written in Rust and is one of the primary test
cases for the Rust language.

The Servo tree has been mostly frozen for a large part of this week, as the
nearly month-long Rust upgrade is finally getting close and we wanted to avoid
additional rebases. We did land 7 PRs, many with associated fixes, however.

## Notable additions
- Patrick Kim fixed anonymous boxes and images disappearing during line
breaking in [#1461](https://github.com/mozilla/servo/pull/1461) and
[#1463](https://github.com/mozilla/servo/pull/1463).
- Patrick Walton fixed a race issue with window sizes and script code in
[#1409](https://github.com/mozilla/servo/pull/1409).
- Jaeman Park landed CSS parsing and selector matching for pseudo element
:before and :after in [#1464](https://github.com/mozilla/servo/pull/1464).
- Bruno Abinder made some DOM additions in
[#1466](https://github.com/mozilla/servo/pull/1466) and
[#1472](https://github.com/mozilla/servo/pull/1472).

## Meetings
In this week's
[meeting](https://github.com/mozilla/servo/wiki/Meeting-2014-01-06), we
discussed the upcoming workweek, the Rust upgrade, ARM buildbots for Rust and
Servo, and some research we're doing into the breakdown of page load.

# Announcements, etc

- [Bay Area Rust Meetup:
1/28/2014](http://www.meetup.com/Rust-Bay-Area/events/153909222/)
- [The Rust language: memory, ownership and
lifetimes](http://www.youtube.com/watch?v=gfCtbGiHcg0),
Niko's presentation at Linux.conf.au
- [An appeal for correct, capable, future-proof math in nascent programming
languages](http://www.reddit.com/r/rust/comments/1uy7rt/an_appeal_for_correct_capable_futureproof_math_in/)
- [An in-progress redesign of the
website](http://www.reddit.com/r/rust/comments/1v10ac/thehydroimpulserustwebsite/)
- [Combining rust-http with route-recognizer.rs to create routable
HTTP](http://www.reddit.com/r/rust/comments/1v104e/combining_rusthttp_with_routerecognizerrs_to/)
- [A TOML configuration file parser](https://github.com/mneumann/rust-toml)
- [Mapping High-Level Constructs to LLVM
IR](http://llvm.lyngvig.org/Articles/Mapping-High-Level-Constructs-to-LLVM-IR),
a peek at what drives the underbelly of the compiler
- [Add garbage collector to
`std::gc`](https://github.com/mozilla/rust/pull/11399)
- [rust-workspace](https://github.com/HeroesGrave/rust-workspace), a simple
shell to make maintaining Rust projects easier
- [Semantic code browser for
Rust](http://www.reddit.com/r/rust/comments/1usupv/semantic_code_browser_for_rust/)
- [Rvalue Lifetimes in
Rust](http://smallcultfollowing.com/babysteps/blog/2014/01/09/rvalue-lifetimes-in-rust/)
- [Nobody Knows Rust - Steve Klabnik's slides](http://steveklabnik.github.io/nobody_knows_rust/#/)
- [0.9 Release
Announcement](https://mail.mozilla.org/pipermail/rust-dev/2014-January/007753.html)
- [A 3d-printable Rust
logo](https://github.com/cmr/rust-logo-3d/blob/master/rlogo.stl)
