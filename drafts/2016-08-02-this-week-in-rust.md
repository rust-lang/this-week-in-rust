Title: This Week in Rust 141
Number: 141
Date: 2016-08-02
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

* [easy] [rust: move coerce_match, coerce_calls and related tests into
  run-pass-valgrind](https://github.com/rust-lang/rust/issues/21696). Just
  moving tests around. Easy introduction to the build system.
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
* [easy] [rust: rustbuild should warn on quotes in
  PATH](https://github.com/rust-lang/rust/issues/34959). Easy fix for
  Rust's new build system.
* [easy] [rust: Make it easier to locate deadlocked
  tests](https://github.com/rust-lang/rust/issues/2873).  A simple
  enhancement to the test runner to notify the user about long-running
  tests.
* [easy] [rust: Remove obsolete no_stack_check attribute and
  test](https://github.com/rust-lang/rust/issues/34915). Easy
  introduction to the compiler and contribution process.
* [easy] [rustfmt: Wrap overlong function
  signatures](https://github.com/rust-lang-nursery/rustfmt/issues/1049).
  There are several rustfmt bugs along these lines so this is a
  fertile entrypoint for contributors.
* [easy] [rustup: Add command to install shell
  completions](https://github.com/rust-lang-nursery/rustup.rs/issues/387#issuecomment-234675568).
* [easy] [rustup: Clean up toolchain directories after
  uninstall](https://github.com/rust-lang-nursery/rustup.rs/issues/596#issuecomment-233716695).
  A simple bug with a clear fix.
* [moderate] [rust: improve error message when resolution via Deref
  actually required
  DerefMut](https://github.com/rust-lang/rust/issues/28419). Good
  first type system bug.
* [easy] [imag: Switch to clap in the `imag` binary](https://github.com/matthiasbeyer/imag/issues/566).

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

* Evgeny Safronov
* Matt Horn

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1559: Allow all literals in attributes](https://github.com/rust-lang/rfcs/pull/1559).

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
* [Some internal and language-level changes to name resolution](https://github.com/rust-lang/rfcs/pull/1560).
* [Define a best practices procedure for making bug fixes in the compiler](https://github.com/rust-lang/rfcs/pull/1589).
* [RFC process for formatting style and Rustfmt defaults](https://github.com/rust-lang/rfcs/pull/1607).
* [Add a compiler flag that emits crate dependencies on a best-effort basis](https://github.com/rust-lang/rfcs/pull/1622).
* [Replace synchronization primitives with those from parking_lot](https://github.com/rust-lang/rfcs/pull/1632).
* [Dedicated strike team to resolve unsafe code guidelines](https://github.com/rust-lang/rfcs/pull/1643).
* [Add `assert_ne` to compliment `assert_eq`](https://github.com/rust-lang/rfcs/pull/1653).
* [Introduce non-panicking borrow methods on `RefCell<T>`](https://github.com/rust-lang/rfcs/pull/1660).
* [Propose asserts](https://github.com/rust-lang/rfcs/pull/1662). This rfc proposes that the following macros be added: `assert_gt`, `assert_lt`, `assert_ge`, and `assert_le`.
* [Clarify behaviour when writing to a union field that implements Drop](https://github.com/rust-lang/rfcs/pull/1663).

## New RFCs

* [Add API documentation front page styleguide](https://github.com/rust-lang/rfcs/pull/1687).
* [Deprecate anonymous parameters](https://github.com/rust-lang/rfcs/pull/1685).
* [Add an environment variable to choose between whether to link the static CRT or dynamic CRT](https://github.com/rust-lang/rfcs/pull/1684).
* [Create a team responsible for documentation for the Rust project](https://github.com/rust-lang/rfcs/pull/1683).
* [Propose a shorthand syntax for constructing struct-like values with _named_ fields](https://github.com/rust-lang/rfcs/pull/1682).

# Upcoming Events

* 7/27. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [7/27. Rust Berlin July Meetup](http://www.meetup.com/Rust-Berlin/events/232583152/).
* [7/28. Rust Bay Area: Machine Learning, Bioinformatics, and Embedded OSes](http://www.meetup.com/Rust-Bay-Area/events/232406852/).
* 8/3. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* 8/4. Rust release triage at #rust-triage on irc.mozilla.org.
* [8/8. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> you have a problem. you decide to use Rust. now you have a Rc<RefCell<Box<Problem\>\>\>

[kmc on #rust](https://botbot.me/mozilla/rust/2016-07-25/?msg=70207904&page=14).

Thanks to [Alex Burka](https://users.rust-lang.org/users/durka) for the tip. [Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
