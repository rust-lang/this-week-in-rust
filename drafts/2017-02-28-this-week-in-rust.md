Title: This Week in Rust 171
Number: 171
Date: 2017-02-28
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

# Crate of the Week

This week's crate of the week is [CDRS](https://crates.io/crates/cdrs), a client for Apache Cassandra written completely in Rust. Thanks to [Alex Pikalov](https://users.rust-lang.org/users/AlexPikalov) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: `std::ffi::FromBytesWithNulError` is not an `std::error::Error`](https://github.com/rust-lang/rust/issues/39925).
* [easy] [clippy: Lint functions taking references as arguments but only use them to create an owned value](https://github.com/Manishearth/rust-clippy/issues/1563).
* [easy] [clippy: Lint for iterating over a slice with one (or zero) element](https://github.com/Manishearth/rust-clippy/issues/1540).
* [easy] [clippy: useless_transmute being raised when it's doing multiple casts](https://github.com/Manishearth/rust-clippy/issues/1545).
* [easy] [clippy: Lint to suggest `.saturating_add/sub(x)` for `.checked_add/sub(x).unwrap_or(MAX/MIN)`](https://github.com/Manishearth/rust-clippy/issues/1557).
* [easy] [clippy: Lint against const atomics](https://github.com/Manishearth/rust-clippy/issues/1560).
* [easy] [crates.io: Insufficient spacing between Dev-Dependencies and Versions](https://github.com/rust-lang/crates.io/issues/235).
* [easy] [crates.io: Document applying categories/adding new categories](https://github.com/rust-lang/crates.io/issues/544).
* [medium] [crates.io: Yanking a crate should update its max_version](https://github.com/rust-lang/crates.io/issues/76).
* [medium] [crates.io: Exact match not being obvious](https://github.com/rust-lang/crates.io/issues/493).
* [medium] [crates.io: Be able to search within a keyword or category](https://github.com/rust-lang/crates.io/issues/491).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

107 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-02-13..2017-02-20

* [stabilize Field-init shorthand](https://github.com/rust-lang/rust/pull/39761) ([RFC #1682](https://github.com/rust-lang/rfcs/blob/master/text/1682-field-init-shorthand.md))
* [ignore expected types in diverging blocks](https://github.com/rust-lang/rust/pull/39485)
* [make derive macro passes independent](https://github.com/rust-lang/rust/pull/39572)
* [improved derive macro suggestions](https://github.com/rust-lang/rust/pull/39752)
* [fix macro sequence repetition ICE](https://github.com/rust-lang/rust/pull/39730)
* [fix segfault on disordered BTrees](https://github.com/rust-lang/rust/pull/39457)
* [`Hash`{`Map`, `Set`}`::retain(_)`](https://github.com/rust-lang/rust/pull/39560)
* [adaptive `HashMap`](https://github.com/rust-lang/rust/pull/38368)
* [`TypeId` now implements (`Partial`)`Ord`](https://github.com/rust-lang/rust/pull/38981)
* [more `Cell` methods for non-`Copy` types](https://github.com/rust-lang/rust/pull/39793)
* [`Cell::swap(..)`](https://github.com/rust-lang/rust/pull/39716)
* [`is_ascii_...(_)`](https://github.com/rust-lang/rust/pull/39659)
* [conversions between various string types](https://github.com/rust-lang/rust/pull/39594)
* [`TraitSelect` now with less `Vec`](https://github.com/rust-lang/rust/pull/39912)
* [MIR: `SwitchInt` all the branches!](https://github.com/rust-lang/rust/pull/39456)
* [fix two ICEs in path resolution](https://github.com/rust-lang/rust/pull/39939)
* [erase late-bound regions in `get_vtable_methods`](https://github.com/rust-lang/rust/pull/39887)
* [port books to mdbook](https://github.com/rust-lang/rust/pull/39633) (bye bye rustbook)

## New Contributors

* Amos Onn
* Andrew Gaspar
* Benoît CORTIER
* Brian Vincent
* Dmitry Guzeev
* Glyne J. Gittens
* Jeff Muizelaar
* Luxko
* Matt Williams
* Michal Nazarewicz
* Mikhail Pak
* Sebastian Waisbrot

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

* [disposition: merge] [extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).
* [disposition: merge] [Write to standard error with `eprint!` and `eprintln!`](https://github.com/rust-lang/rfcs/pull/1869).
* [disposition: close] [Add a `Transmute<T>` trait for representing types that can be transmuted to `T`](https://github.com/rust-lang/rfcs/pull/1891).
* [disposition: merge] [Deprecate anonymous parameters](https://github.com/rust-lang/rfcs/pull/1685).
* [disposition: close] [Warn by default when encountering a statement which only consists of an equality comparison](https://github.com/rust-lang/rfcs/pull/1812).
* [Allow uncallable method impls to be omitted](https://github.com/rust-lang/rfcs/pull/1699).

## New RFCs

* [Reverting default unsafety](https://github.com/rust-lang/rfcs/pull/1901). Provide ability to mark unsafe-by-default entities, like foreign items, as safe.
* [Unsized Rvalues](https://github.com/rust-lang/rfcs/pull/1909). Allow for local variables, function arguments, and some expressions to have an unsized type.
* [Add a `#[safe("Reason")]` to annotate why unsafe blocks are actually safe](https://github.com/rust-lang/rfcs/pull/1910).
* [Macros 1.2: Fast-track to stabilize function-like procedural macros](https://github.com/rust-lang/rfcs/pull/1913). Stabilize function-like procedural macros (whose usage looks like `foo!(...)`), like this was done in “Macros 1.1” for custom `derive`, before “Macros 2.0” is fully ready.
* [Extend and stabilize the `FixedSizeArray` trait, as a stop-gap solution for integer parameters in generics](https://github.com/rust-lang/rfcs/pull/1915).
* [Unsafe lifetime](https://github.com/rust-lang/rfcs/pull/1918). Add a new special lifetime, `'unsafe`, that implicitly satisfies any constraint, but may only be instantiated within an unsafe context.

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

* [Feb 22. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Feb 22. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Feb 22. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/236658946/).
* [Feb 23. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Feb 23. Rust Dresden](https://forum.rustplatz.de/t/neues-rust-meetup-in-dresden/156/24).
* [Feb 23. Rust Copenhagen: Hack Night](http://cph.rs)
* [Feb 24. Crate Polishing Workshop, Darmstadt/Germany](https://www.meetup.com/Rust-Rhein-Main/events/237509289/).
* [Mar  1. Rust User Group Cologne - Web development in Rust](http://rust.cologne/2017/03/01/web-dev.html).
* [Mar  1. South Florida Rust - Intro to Ownership and Borrowing](https://www.meetup.com/South-Florida-Rust-Meetup/events/237559303/).
* [Mar  1. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar  1. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar  7. Rust Oslo - What's New - Focus on web services](https://www.meetup.com/Rust-Oslo/events/237849579/).
* [Mar  8. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar  8. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar  9. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/237525355/).
* [Mar  9. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Mar 29. GNOME+Rust Hackfest 2017, Mexico City](https://wiki.gnome.org/Hackfests/Rust2017).
* [Mar 31. Underhanded Rust Contest Submission Deadline](https://underhanded.rs/blog/2016/12/15/underhanded-rust.en-US.html)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Full Stack Developer for Resin Supervisor (JavaScript + Rust)](https://resin.workable.com/jobs/399897).
* [Postdoc positions for RustBelt project](http://lists.seas.upenn.edu/pipermail/types-announce/2017/006485.html) at MPI-SWS.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
