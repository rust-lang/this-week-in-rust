Title: This Week in Rust 172
Number: 172
Date: 2017-03-07
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## News & Blog Posts
* My shot at RESTful microservices in Rust [Part 1](https://codingwithglee.blogspot.de/2017/02/my-shot-at-restful-microservices-in.html) [Part 2](https://codingwithglee.blogspot.de/2017/03/my-shot-at-restful-microservices-in.html) [Part 3](https://codingwithglee.blogspot.de/2017/03/my-shot-at-restful-microservices-in_6.html).

* [LLVM Social Berlin #6: Mull meets Rust. Implementing mutation testing system for Rust programming language.](https://www.youtube.com/watch?v=VasSufnFswc)

## Other Weeklies from Rust Community

# Crate of the Week

This week's crate of the week is [cargo-fuzz](https://crates.io/crates/cargo-fuzz), a cargo subcommand to run libfuzz on your code. Thanks to [nasa42](https://users.rust-lang.org/users/nasa42) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rails Girls Summer of Code + Servo](https://blog.servo.org/2017/02/27/rgsoc/).
* [discussion] [rust-book: Rust & OOP patterns](https://users.rust-lang.org/t/what-does-rust-oop-mean-to-you/9633).
* [easy] [servo: Looking for something to work on](https://github.com/servo/servo/issues/15162).
* [medium] [clippy: Lint `.into_iter()` if that only forwards to `.iter()`](https://github.com/Manishearth/rust-clippy/issues/1565).
* [medium] [clippy: Warn on `let _ = x.lock();`](https://github.com/Manishearth/rust-clippy/issues/1574).
* [hard] [clippy: Lint crates that can be `#![no_std]` but aren't](https://github.com/Manishearth/rust-clippy/issues/1569).
* [easy/hard] [clippy: Lint functions taking references as arguments but only use them to create an owned value](https://github.com/Manishearth/rust-clippy/issues/1563).
* [easy] [clippy: Lint for iterating over a slice with one (or zero) element](https://github.com/Manishearth/rust-clippy/issues/1540).
* [easy] [clippy: useless_transmute being raised when it's doing multiple casts](https://github.com/Manishearth/rust-clippy/issues/1545).
* [easy] [clippy: Lint to suggest `.saturating_add/sub(x)` for `.checked_add/sub(x).unwrap_or(MAX/MIN)`](https://github.com/Manishearth/rust-clippy/issues/1557).
* [easy] [clippy: Lint against const atomics](https://github.com/Manishearth/rust-clippy/issues/1560).
* [easy] [crates.io: Add rustfmt to run on travis and fail the build](https://github.com/rust-lang/crates.io/issues/575).
* [easy] [crates.io: Run rustfmt on the whole codebase and send in the changes](https://github.com/rust-lang/crates.io/issues/574).
* [easy] [crates.io: Document applying categories/adding new categories](https://github.com/rust-lang/crates.io/issues/544).
* [medium] [crates.io: Yanking a crate should update its max_version](https://github.com/rust-lang/crates.io/issues/76).
* [medium] [crates.io: Be able to search within a keyword or category](https://github.com/rust-lang/crates.io/issues/491).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

105 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-02-27..2017-03-06

* [improve backtrace format](https://github.com/rust-lang/rust/pull/38165) (🎉Yay!🎉)
* [support `x86-interrupt` calling conventions](https://github.com/rust-lang/rust/pull/39832) (also [update LLVM](https://github.com/rust-lang/rust/pull/40207) and the [unstable book](https://github.com/rust-lang/rust/pull/40191))
* [transmuting from fn item types to pointers is now a hard error](https://github.com/rust-lang/rust/pull/34198) (breaking change)
* [simplify `TokenTree`s and fix `macro_rules!`](https://github.com/rust-lang/rust/pull/39419) (macro-breaking change)
* [indexing (`_[_]`) now coerces the argument](https://github.com/rust-lang/rust/pull/40166)
* [syntax: use `TokenStream` instead of `TokenTree` where applicable](https://github.com/rust-lang/rust/pull/40202)
* [`#[proc_macro]` for procedural bang!-macros](https://github.com/rust-lang/rust/pull/40129)
* [have `format!(..)` panic on errors](https://github.com/rust-lang/rust/pull/40117)
* [`impl FromIterator<&char> for String`](https://github.com/rust-lang/rust/pull/40028)
* [on-demand typeck, const-qualification/eval and MIR building](https://github.com/rust-lang/rust/pull/40008) (12/12 would pull at least twice 😁)
* [on-demand destructors](https://github.com/rust-lang/rust/pull/40178)
* [MIR: improved operand lifetimes](https://github.com/rust-lang/rust/pull/40133)
* [fix ICE on exhaustive-pattern check](https://github.com/rust-lang/rust/pull/40285)
* [fix some normalization bugs](https://github.com/rust-lang/rust/pull/40163)
* [fix missing `while let` pattern scope](https://github.com/rust-lang/rust/pull/40242)
* [Let `-Crelocation-model` better control `-pie` linking](https://github.com/rust-lang/rust/pull/40245)
* [don't optimize layout for `#[repr(C)]` or `#[repr(u8)]`](https://github.com/rust-lang/rust/pull/40188)
* [`impl `{`Error`, `Display`}` for std::ffi::FromBytesWithNulError`](https://github.com/rust-lang/rust/pull/39960)
* [`impl RangeArgument for RangeInclusive`](https://github.com/rust-lang/rust/pull/39936)
* [spring clean `std::unicode`](https://github.com/rust-lang/rust/pull/40189)
* [`cargo test --all`: fix doctest dependencies](https://github.com/rust-lang/cargo/pull/3721) (also [in beta](https://github.com/rust-lang/cargo/pull/3781))
* [`cargo test`/`bench` no longer builds binaries with unselected features](https://github.com/rust-lang/cargo/pull/3770)
* [cargo accepts more historically used underscores instead of dashes](https://github.com/rust-lang/cargo/pull/3776)

## New Contributors

* Daniel Xu
* er-1
* Hiroki Kobayashi
* Josef Brandl
* Paul Merrill
* Peter Wagenet
* Tom Anderson
* topecongiro

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: postpone] [Introduce _pattern synonyms_ - used to create new patterns we can pattern match against from real patterns](https://github.com/rust-lang/rfcs/pull/1895).
* [disposition: close] [Add a `Transmute<T>` trait for representing types that can be transmuted to `T`](https://github.com/rust-lang/rfcs/pull/1891).
* [disposition: merge] [Write to standard error with `eprint!` and `eprintln!`](https://github.com/rust-lang/rfcs/pull/1869).
* [disposition: merge] [extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).
* [disposition: close] [Add 'else match' blocks to if expressions](https://github.com/rust-lang/rfcs/pull/1712).
* [disposition: postpone] [Allow uncallable method impls to be omitted](https://github.com/rust-lang/rfcs/pull/1699).
* [disposition: merge] [Deprecate anonymous parameters](https://github.com/rust-lang/rfcs/pull/1685).
* [disposition: postpone] [Add the ability to define closures that are generic over types](https://github.com/rust-lang/rfcs/pull/1650).
* [disposition: close] [Add `&move` pointers, the `DerefMove` trait, and the unsafe `DerefPure` traits](https://github.com/rust-lang/rfcs/pull/1646).

## New RFCs

* [dependent-types (also known as, Π-types and value-types)](https://github.com/rust-lang/rfcs/pull/1931).
* [Introduce `with` bounds for pi types](https://github.com/rust-lang/rfcs/pull/1932).
* [Fully dependent pi types](https://github.com/rust-lang/rfcs/pull/1933).
* [Allow an optional vert at the beginning of a match branch](https://github.com/rust-lang/rfcs/pull/1925).
* [Reduce the number of constraints repeated when writing `T: SomeTrait`](https://github.com/rust-lang/rfcs/pull/1927).
* [Tuple-based variadic generics](https://github.com/rust-lang/rfcs/pull/1935).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

PRs:

* [structs and unions](https://github.com/rust-lang-nursery/fmt-rfcs/pull/53)

Issues in final comment period:

* [`extern` vs `extern "C"`](https://github.com/rust-lang-nursery/fmt-rfcs/issues/52)
* [Whitespace in associated type syntax](https://github.com/rust-lang-nursery/fmt-rfcs/issues/51).
* [types](https://github.com/rust-lang-nursery/fmt-rfcs/issues/15)

Other significant issues:

* [ranges](https://github.com/rust-lang-nursery/fmt-rfcs/issues/60)
* [where clauses](https://github.com/rust-lang-nursery/fmt-rfcs/issues/38)

# Upcoming Events

* [Mar  1. Rust User Group Cologne - Web development in Rust](http://rust.cologne/2017/03/01/web-dev.html).
* [Mar  1. South Florida Rust - Intro to Ownership and Borrowing](https://www.meetup.com/South-Florida-Rust-Meetup/events/237559303/).
* [Mar  1. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar  1. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar  2. GPU glyph rasterization, Rocket, and the orphan rules](https://www.meetup.com/Rust-Bay-Area/events/237709786/).
* [Mar  7. Mozilla Meetup Switzerland - Rust on the Rumprun Unikernel](https://www.meetup.com/de-DE/Mozilla-Meetup-Switzerland/events/237757802/).
* [Mar  7. Rust Oslo - What's New - Focus on web services](https://www.meetup.com/Rust-Oslo/events/237849579/).
* [Mar  8. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/236658966/).
* [Mar  8. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar  8. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar  9. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/237525355/).
* [Mar  9. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Mar 11. Rust NYC - Rust Hack & Learn](https://www.meetup.com/Rust-NYC/events/238057861/).
* [Mar 13. Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/237058819/).
* [Mar 15. Rust Meetup Hamburg - Rust/Ethereum Meetup](https://www.meetup.com/Rust-Meetup-Hamburg/events/237858112/).
* [Mar 15. Rust Los Angeles - Rust LA Monthly Meetup - Hack Night](https://www.meetup.com/Rust-Los-Angeles/events/237757181/).
* [Mar 15. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar 15. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar 16. Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/de-DE/Rust-Modern-Systems-Programming-in-Leipzig/events/237780401/).
* [Mar 16. Thompson Rivers University, BC Canada - Get Rusty](https://www.eventbrite.ca/e/get-rusty-tickets-31407199780).
* [Mar 29. GNOME+Rust Hackfest 2017, Mexico City](https://wiki.gnome.org/Hackfests/Rust2017).
* [Mar 31. Underhanded Rust Contest Submission Deadline](https://underhanded.rs/blog/2016/12/15/underhanded-rust.en-US.html)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> What about the Quote of the Week? I noticed it's missing quite often these days.

— [llogiq on reddit](https://www.reddit.com/r/rust/comments/5vh4uk/this_week_in_rust_170/de2j085/).

Thanks to [tibodelor for the suggestion](https://www.reddit.com/r/rust/comments/5vh4uk/this_week_in_rust_170/de3ppdd/).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
