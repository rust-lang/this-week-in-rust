Title: This Week in Rust 140
Number: 140
Date: 2016-07-26
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## News & Blog Posts

## New Crates & Project Updates

# Crate of the Week

In what seems to become a kind of tradition, User [gsingh93](https://users.rust-lang.org/users/gsingh93) suggested his [trace](https://crates.io/crates/trace) crate, a syntax extension to insert `print!` statements to functions to help trace execution. Thanks, gsingh93!

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [imag: Make `imag` forward `--debug` and `--verbose` to subcommands](https://github.com/matthiasbeyer/imag/issues/506).
* [moderate] [rust: Very confusing error on attempt to pass
  `path::Path` by
  value](https://github.com/rust-lang/rust/issues/23286). This is bad
  error message that is hit often. Good bug to get familiar with the
  compiler.
* [easy] [rust: move coerce_match, coerce_calls and related tests into
  run-pass-valgrind](https://github.com/rust-lang/rust/issues/21696). Just
  moving tests around. Easy introduction to the build system.
* [easy] rustbyexample.com is in need of maintainers. Good first tasks
  are [writing Mutex examples](https://github.com/rust-lang/rust-by-example/issues/105)
  and [Arc examples](https://github.com/rust-lang/rust-by-example/issues/104).
* [hard] [rustup: Write a GUI installer for rustup on
  Windows](https://github.com/rust-lang-nursery/rustup.rs/issues/253). This
  is involved but should be fun. It's an integration problem, writing
  a Windows GUI that hooks into the MSI installation system and calls
  into the rustup libraries. Required for rustup 1.0.
* [easy] [cargo: Warn on the duplicate entry points for lib and
  bin](https://github.com/rust-lang/cargo/issues/2800).
* [easy] [cargo: Can't specify precise crate version if there are
  multiple versions](https://github.com/rust-lang/cargo/issues/2773).
* [easy] [error-chain: Display implementation should show the error's
  Display, not just the
  description](https://github.com/brson/error-chain/issues/2). Looks
  like a simple fix.
* [easy] [rust: Parsing inconsistencies (lambda, proc,
  return)](https://github.com/rust-lang/rust/issues/28784). This bug
  identifies some bugs where the rustc parser disagrees with the
  reference parser. Good first bug for someone interested in parsers.
* [easy] [imag: `--version` and `--versions` yield helptext instead of version(s)](https://github.com/matthiasbeyer/imag/issues/540).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

76 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-07-11..2016-07-18

* [`mtwt` is now `hygiene` and cleaned up](https://github.com/rust-lang/rust/pull/34860) – nomen est omen
* [`impl<T> From<T> for Option<T>`](https://github.com/rust-lang/rust/pull/34828)
* [MIR optimization test groundwork](https://github.com/rust-lang/rust/pull/34715)
* [Fixed interplay between precision and width in format specifiers](https://github.com/rust-lang/rust/pull/34544) (**breaking change**)
* [`ExactSizeIterator::is_empty()`](https://github.com/rust-lang/rust/pull/34357)
* [No more MIPS Soft-Float](https://github.com/rust-lang/rust/pull/34910)
* [Macros: Statements are now matched greedily](https://github.com/rust-lang/rust/pull/34886)
* [Nested `macro_rules!`](https://github.com/rust-lang/rust/pull/34925)
* `impl Debug for {Entry, VacantEntry, OccupiedEntry}` in [`btree_map](https://github.com/rust-lang/rust/pull/34885) and [`hash_map`](https://github.com/rust-lang/rust/pull/34937)
* [More privacy for linker symbols](https://github.com/rust-lang/rust/pull/34899) (improves perf)
* [`impl AddAssign for String](https://github.com/rust-lang/rust/pull/34890) (wasn't there already?!)
* [beta gets new jemalloc](https://github.com/rust-lang/rust/pull/34927)
* [`cargo publish --dry-run`](https://github.com/rust-lang/cargo/pull/2849)

## New Contributors

* abhi
* Aravind Gollakota
* Ben Boeckel
* Ben Stern
* David
* Dridi Boukelmoune
* Isaac Andrade
* Zhen Zhang

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1574: Introduce more conventions around documenting Rust projects](https://github.com/rust-lang/rfcs/pull/1574).
* [RFC 1644: Default and expanded errors for rustc](https://github.com/rust-lang/rfcs/pull/1644).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Promote `!` to a type](https://github.com/rust-lang/rfcs/pull/1216)
* [Add language support for bitfields](https://github.com/rust-lang/rfcs/pull/1449).
* [Add support for 128-bit integers](https://github.com/rust-lang/rfcs/pull/1504).
* [Add space-friendly arguments](https://github.com/rust-lang/rfcs/pull/1509). Add `-C link-arg` and `-C llvm-arg` which allow you to pass along argument with spaces.
* [Exclude macros from importing with `#[macro_use(not(...))]`](https://github.com/rust-lang/rfcs/pull/1517).
* [Add `global_asm!` for module-level inline assembly](https://github.com/rust-lang/rfcs/pull/1548).
* [Allow all literals in attributes](https://github.com/rust-lang/rfcs/pull/1559).
* [RFC process for formatting style and Rustfmt defaults](https://github.com/rust-lang/rfcs/pull/1607).
* [Replace synchronization primitives with those from parking_lot](https://github.com/rust-lang/rfcs/pull/1632).
* [Dedicated strike team to resolve unsafe code guidelines](https://github.com/rust-lang/rfcs/pull/1643).
* [Add `assert_ne` to compliment `assert_eq`](https://github.com/rust-lang/rfcs/pull/1653).
* [Introduce non-panicking borrow methods on `RefCell<T>`](https://github.com/rust-lang/rfcs/pull/1660).
* [Propose asserts](https://github.com/rust-lang/rfcs/pull/1662). This rfc proposes that the following macros be added: `assert_gt`, `assert_lt`, `assert_ge`, and `assert_le`.

## New RFCs

* [Procedural macros 1.1](https://github.com/rust-lang/rfcs/pull/1681).
* [Startup initialized statics](https://github.com/rust-lang/rfcs/pull/1674). Introduce the ability to initialize (i.e., mutate) static items (even non-mut ones) at the beginning of main in a compiler-guaranteed safe manner.
* [Unified machine word trait](https://github.com/rust-lang/rfcs/pull/1676). Unify functionality peculiar to `i8`…`i64` and `u8`…`u64` in a trait containing the family of `overflowing`/`checked`/`wrapping`/`saturating` variants of arithmetic operations, as well as a few new ones.
* [Add non-panicking `abs()` functions to all signed integer types](https://github.com/rust-lang/rfcs/pull/1678).
* [Add "panic-safe" or "total" alternatives to the existing panicking indexing syntax](https://github.com/rust-lang/rfcs/pull/1679).

# Upcoming Events

* 7/20. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [7/21. Rust Hack & Learn Karlsruhe](http://www.meetup.com/Rust-Hack-Learn-Karlsruhe/events/232621692/).
* 7/27. Rust Community Team Meeting at #rust-community on irc.mozilla.org.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Rust developer at The Blackbird](https://rust.jobboard.io/jobs/394482-rust-developer-at-the-blackbird).
* [Engineering positions at Zcash mention Rust](https://z.cash/blog/hiring.html).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> fzammetti:
> Am I the only one that finds highly ironic the naming of something that's supposed to be new and cutting-edge after a substance universally synonymous with old, dilapidated and broken down?
>
> paperelectron:
> Rust is as close to the bare metal as you can get.

On [/r/programming](https://www.reddit.com/r/programming/comments/4sgzk5/shipping_rust_in_firefox/d59d2lp).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
