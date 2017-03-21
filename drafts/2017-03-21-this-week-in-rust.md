Title: This Week in Rust 174
Number: 174
Date: 2017-03-21
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

We don't have a Crate of this Week for lack of suggestions. Sorry.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Crossbeam project is looking for new maintainers](https://internals.rust-lang.org/t/crossbeam-request-for-help/4933).
* [The Underhanded Rust Contest](https://underhanded.rs/blog/2016/12/15/underhanded-rust.en-US.html).
* [medium] [notify-rust: Implement icons and images](https://github.com/hoodie/notify-rust/issues/13). notify-rust let's you send desktop notifications on Linux and BSD.
* [tempdir: TempDir affected by remove_dir_all unreliability on windows](https://github.com/rust-lang-nursery/tempdir/issues/15#issuecomment-286513675).
* [easy] [servo: Looking for something to work on](https://github.com/servo/servo/issues/15162).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

117 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?page=6&q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-03-13..2016-03-20

* [1.17 library stabilizations](https://github.com/rust-lang/rust/pull/40538)
* [`0e+10` is now a valid Rust float literal](https://github.com/rust-lang/rust/pull/40589)
* [Rust works again on pre 1.12 macOS](https://github.com/rust-lang/rust/pull/40482)
* [pass attributes to procedural macros as `TokenStream`](https://github.com/rust-lang/rust/pull/40346) (macro plugin-breaking)
* [fix `include!(_)` regression](https://github.com/rust-lang/rust/pull/40583)
* [add `catch { }` to AST](https://github.com/rust-lang/rust/pull/39921) (plugin-breaking)
* [avoid alignment-related undefined behavior on operand-pair store](https://github.com/rust-lang/rust/pull/40385)
* [`TryFrom<Err=_>` is now `TryFrom<Error=_>`, also subsumes `FromStr`](https://github.com/rust-lang/rust/pull/40281)
* [new `Utf8Error::error_len`](https://github.com/rust-lang/rust/pull/40212)
* [remove `Default` impl for `Box<Path>`](https://github.com/rust-lang/rust/pull/40539)
* [remove a few stray `From<Box<_>>` impls](https://github.com/rust-lang/rust/pull/40009)
* [Fix race condition in `fs::create_dir_all(..)`](https://github.com/rust-lang/rust/pull/39799)
* [LLVM: fix inliner funclet unwind memoization](https://github.com/rust-lang/llvm/pull/66) (unbreaks MSVC optimizer)
* [`cargo -vv` now caps lints at `warn` level](https://github.com/rust-lang/cargo/pull/3827)
* [`cargo` enables default features of dependencies overriding others without those features](https://github.com/rust-lang/cargo/pull/3843)
* [`cargo` now assumes the `--cap-lints` feature is available](https://github.com/rust-lang/cargo/pull/3839)
* [rustdoc displays `const` items](https://github.com/rust-lang/rust/pull/40564)
* [docs are now required again](https://github.com/rust-lang/rust/pull/40526)

## New Contributors

* CrazyMerlyn
* Fabjan Sukalia
* Gibson Fahnestock
* Joel Gallant
* Jonas Bushart
* Joshua Horwitz
* madseagames
* Paul Daniel Faria
* Petr Hosek
* Tobias Schottdorf

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1845: `From<&[T]> for Rc<[T]> + From<&str> for Rc<str>`](https://github.com/rust-lang/rfcs/pull/1845).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Add unstable sort to libcore](https://github.com/rust-lang/rfcs/pull/1884).
* [disposition: merge] [Write to standard error with `eprint!` and `eprintln!`](https://github.com/rust-lang/rfcs/pull/1869).
* [disposition: merge] [Include the `ManuallyDrop` wrapper in `core::mem`](https://github.com/rust-lang/rfcs/pull/1860).
* [disposition: merge] [extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).
* [disposition: close] [Add variable-length arrays to the language](https://github.com/rust-lang/rfcs/pull/1808).
* [disposition: merge] [Extend entry API to work on borrowed keys](https://github.com/rust-lang/rfcs/pull/1769).
* [disposition: merge] [Deprecate anonymous parameters](https://github.com/rust-lang/rfcs/pull/1685).

## New RFCs

* [Allow the name (qualifier) of an enum variant to be elided in expressions and patterns whenever it can be inferred](https://github.com/rust-lang/rfcs/pull/1949).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

PRs in final comment period:

* [structs and unions](https://github.com/rust-lang-nursery/fmt-rfcs/pull/53)

Issues in final comment period:

* [where clauses](https://github.com/rust-lang-nursery/fmt-rfcs/issues/38)

Other significant issues:

* [expressions (tracking issue)](https://github.com/rust-lang-nursery/fmt-rfcs/issues/16)

# Upcoming Events

* [Mar 15. Rust Meetup Hamburg - Rust/Ethereum Meetup](https://www.meetup.com/Rust-Meetup-Hamburg/events/237858112/).
* [Mar 15. Rust Los Angeles - Rust LA Monthly Meetup - Hack Night](https://www.meetup.com/Rust-Los-Angeles/events/237757181/).
* [Mar 15. Rust Dublin - Rust Lightning Talks](https://www.meetup.com/Rust-Dublin/events/237883717/).
* [Mar 15. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar 15. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar 16. Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/de-DE/Rust-Modern-Systems-Programming-in-Leipzig/events/237780401/).
* [Mar 16. Thompson Rivers University, BC Canada - Get Rusty](https://www.eventbrite.ca/e/get-rusty-tickets-31407199780).
* [Mar 21. Rust Paris Meetup #36](https://www.meetup.com/Rust-Paris/events/238240907/).
* [Mar 22. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/238181558/).
* [Mar 22. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar 22. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar 23. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Mar 29. GNOME+Rust Hackfest 2017, Mexico City](https://wiki.gnome.org/Hackfests/Rust2017).
* [Mar 29. South Florida Rust Meetup: Intro to Ownership and Borrowing Part 3](https://www.meetup.com/South-Florida-Rust-Meetup/events/238110251/).
* [Mar 29. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar 29. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar 31. Underhanded Rust Contest Submission Deadline](https://underhanded.rs/blog/2016/12/15/underhanded-rust.en-US.html).
* **[Apr 30. RustFest 2017 - Kyiv, Ukraine](http://2017.rustfest.eu/).**

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust developers at Cornell Tech New York](https://twitter.com/sahuguet/status/839198110819762177).
* [Rust engineer at a startup in San Francisco](https://users.rust-lang.org/t/jobs-in-rust-development/3628/4).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> In #rustlang, None is always an Option<\_>.

— [llogiq on Twitter](https://twitter.com/llogiq/status/837411901437018113).

Thanks to [Johan Sigfrids for the suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/363).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
