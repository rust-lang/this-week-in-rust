Title: This Week in Rust 158
Number: 158
Date: 2016-11-29
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## Blog Posts

- [Go To Statement Considered (Mostly) Harmless](http://jamey.thesharps.us/2016/11/go-to-statement-considered-harmless.html): Translating C `goto` statements to Rust with Corrode, a C to Rust translator.

## News & Project Updates

## Other Weeklies from Rust Community

# Crate of the Week

Since there were no nominations, this week has to go without a Crate of the Week. Sorry. [Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [less easy] [rayon: Parity with the `Iterator` trait](https://github.com/nikomatsakis/rayon/milestone/2).
* [easy] [rust: Compiling `libunwind` with `--test` for arm-musl targets produces dynamically linked binaries](https://github.com/rust-lang/rust/issues/37811).
* [less easy] [rust: Separate foreign items in HIR](https://github.com/rust-lang/rust/issues/37713).
* [less easy] [rust: Separate trait items from trait](https://github.com/rust-lang/rust/issues/37712).
* [easy] [rust: docs: Explain why/when `.lines()` returns an error](https://github.com/rust-lang/rust/issues/37744).
* [easy] [git-series: Highlight trailing whitespace](https://github.com/git-series/git-series/issues/31).
* [easy] [git-series: Support rebase --exec](https://github.com/git-series/git-series/issues/24).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

66 pull requests were [merged in the last week][merged]. Not much, but there were a good number of awesome changes:

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-11-21..2016-11-28

* [Implement `break` with value](https://github.com/rust-lang/rust/pull/37487) ([RFC #1624](https://github.com/rust-lang/rfcs/blob/master/text/1624-loop-break-value.md))
* [Stabilized name resolution changes](https://github.com/rust-lang/rust/pull/37127) ([RFC #1560](https://github.com/rust-lang/rfcs/blob/master/text/1560-name-resolution.md))
* [Implement panic-safe slicing](https://github.com/rust-lang/rust/pull/36340) ([RFC #1679](https://github.com/rust-lang/rfcs/blob/master/text/1679-panic-safe-slicing.md))
* [Make more types uninhabited](https://github.com/rust-lang/rust/pull/36449)
* [Pad const enums only once](https://github.com/rust-lang/rust/pull/38023)
* [Simplify `HashMap` probing](https://github.com/rust-lang/rust/pull/38022)
* [Reduce type construction calls](https://github.com/rust-lang/rust/pull/37979), [Reduce allocations while walking types](https://github.com/rust-lang/rust/pull/37760),
  [make HirVec smaller](https://github.com/rust-lang/rust/pull/37642) – nnethercote is at it again…
* [Improve macro name resolution performance](https://github.com/rust-lang/rust/pull/37951)
* [Forward `ExactSizeIterator` on some adapters](https://github.com/rust-lang/rust/pull/37944) (for improved performance in some cases)
* [Faster `.is_empty()` for slice and vec iterators](https://github.com/rust-lang/rust/pull/37943)
* [Faster character count](https://github.com/rust-lang/rust/pull/37888)
* [`.set_permissions(_)` for open `File`s](https://github.com/rust-lang/rust/pull/37886)
* [Lifetimes in associated types now a hard error](https://github.com/rust-lang/rust/pull/37843)
* [`Peekable` now remembers seeing a `None`](https://github.com/rust-lang/rust/pull/37834)
* [Epic AST/HIR symbol refactoring](https://github.com/rust-lang/rust/pull/37824)
* [Simplified directory ownership](https://github.com/rust-lang/rust/pull/37602) (breaking change)
* [Crate type metadata](https://github.com/rust-lang/rust/pull/37681)
* [`-Z print-type-sizes`](https://github.com/rust-lang/rust/pull/37770)
* [rustbuild can now `bench`](https://github.com/rust-lang/rust/pull/38008)
* [Cargo now compiles OpenSSL from source on OS X](https://github.com/rust-lang/cargo/pull/3332)


## New Contributors

* Brett Cooley
* John Downey
* jsen-
* Robert Vally
* Steve Smith

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1725: unaligned access via `std::ptr`](https://github.com/rust-lang/rfcs/pull/1725).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Require documentation for all new features](https://github.com/rust-lang/rfcs/pull/1636).

## New RFCs

* [Implement adaptive hashing for HashMap](https://github.com/rust-lang/rfcs/pull/1796).
* [Process for abandoning crates](https://github.com/rust-lang/rfcs/pull/1794).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

PRs:

* [Customising Rustfmt](https://github.com/rust-lang-nursery/fmt-rfcs/pull/33).

Ready for PR:

* [Comments](https://github.com/rust-lang-nursery/fmt-rfcs/issues/17).
* [Simple blocks, `{ ... }`](https://github.com/rust-lang-nursery/fmt-rfcs/issues/21).

Final comment period:

* [Statements](https://github.com/rust-lang-nursery/fmt-rfcs/issues/11).
* [Imports (`use`)](https://github.com/rust-lang-nursery/fmt-rfcs/issues/24).

Other notable issues:

* [block vs visual indentation](https://github.com/rust-lang-nursery/fmt-rfcs/issues/8).
* [function declarations](https://github.com/rust-lang-nursery/fmt-rfcs/issues/39).
* [match](https://github.com/rust-lang-nursery/fmt-rfcs/issues/34).

# Upcoming Events

* [11/30. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [11/30. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [12/1. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [12/1. Rust DC Hack Session — Part 2](https://www.meetup.com/RustDC/events/234593927/).
* [12/7. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [12/7. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# fn work(on: RustProject) -> Money

* [Mozilla Research Internship (US/INTL) - University 2017](https://careers.mozilla.org/position/gh/503816).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust iterators are the best thing since [Bread]

— [Yaniel on Rust (lang) Matrix channel](https://matrix.to/#/!zXfJBqSUvXySmsZMtB:jki.re/%2414789013526180qkxyq:kolm.io).

Thanks to [Elahn](https://users.rust-lang.org/users/elahn) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
