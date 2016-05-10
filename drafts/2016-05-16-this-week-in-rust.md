Title: This Week in Rust 129
Number: 129
Date: 2016-05-09
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

This week's edition was edited by: [Vikrant](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).

# Updates from Rust Community

## News & Blog Posts


## New Crates & Project Updates


# Crate of the Week

This week's Crate of the Week is [semantic-rs](https://github.com/semantic-rs/semantic-rs), which lets us update our project from the commandline ensuring semver compliance on the way. Thanks to [Florian Gilcher](https://users.rust-lang.org/users/skade) for the suggestion!

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Add error explanations for all error codes](https://github.com/rust-lang/rust/issues/32777).
* [medium] [rustup: Make `rustup default x86_64-unknown-linux-gnu` do something smarter](https://github.com/rust-lang-nursery/rustup.rs/issues/411).
* [easy] [rustup: Bad error when downloading bogus versions](https://github.com/rust-lang-nursery/rustup.rs/issues/390).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

92 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-05-02..2016-05-09

* [Cleaner messaging on Errors](https://github.com/rust-lang/rust/pull/32756) (Yay!)
* [TryFrom/TryInto](https://github.com/rust-lang/rust/pull/33426) (implements [RFC #1542](https://github.com/rust-lang/rfcs/blob/master/text/1542-try-from.md))
* [Let rustc optimize for size](https://github.com/rust-lang/rust/pull/32386)
* [find() on chained iterators no longer 2×slower](https://github.com/rust-lang/rust/pull/33289)
* [`mem::forget()` is now inlined](https://github.com/rust-lang/rust/pull/33357)
* [MIR now supports constant expressions](https://github.com/rust-lang/rust/pull/33130) (This includes four breaking changes!)
* [Less Unicode confusion through more aliases](https://github.com/rust-lang/rust/pull/33128/files)
* [Drink the half-full entropy pool on early-bootup `Hash*` creation](https://github.com/rust-lang/rust/pull/33086) (avoids blocking to wait for entropy)
* [`Duration::new(..)` now panics instead of wrapping](https://github.com/rust-lang/rust/pull/33072)
* [`-Wrapping(_)` negation implemented](https://github.com/rust-lang/rust/pull/33067)
* [`Default` for `&CStr` + `CString`](https://github.com/rust-lang/rust/pull/32990)
* [`UnsafeCell/Cell.get_mut()`](https://github.com/rust-lang/rust/pull/32565)
* [`const_eval` fixes](https://github.com/rust-lang/rust/pull/33339)
* [New armv7-linux-androideabi target](https://github.com/rust-lang/rust/pull/33414)

## New Contributors

* Brandon Edens
* Garrett Squire
* jonathandturner
* Nerijus Arlauskas
* Philipp Matthias Schaefer
* Stephen Mather
* Taylor Cramer
* Wang Xuerui

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1525: Add workspaces to Cargo](https://github.com/rust-lang/rfcs/pull/1525).
* [RFC 1521: Copy/Clone semantics](https://github.com/rust-lang/rfcs/pull/1521).
* [RFC 1542: Add `TryFrom` and `TryInto` traits](https://github.com/rust-lang/rfcs/pull/1542).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [All but the last field of a tuple must be Sized](https://github.com/rust-lang/rfcs/pull/1592).
* [Permit the `..` pattern fragment in more contexts](https://github.com/rust-lang/rfcs/pull/1492).
* [Translate undefined generic intrinsics to an LLVM `unreachable` and a lint](https://github.com/rust-lang/rfcs/pull/1478).
* [Add compiler support for generic atomic operations](https://github.com/rust-lang/rfcs/pull/1477).
* [Add `#[repr(align = "N")]`](https://github.com/rust-lang/rfcs/pull/1358).

## New RFCs

* [Allow type aliases in enumeration repr attributes](https://github.com/rust-lang/rfcs/pull/1605).
* [RFC process for formatting style and Rustfmt defaults](https://github.com/rust-lang/rfcs/pull/1607).
* [clarify 'root' with 'root crate' or 'root `Cargo.toml`'](https://github.com/rust-lang/rfcs/pull/1608).

# Upcoming Events

* [5/10. Rust NYC: Systems Programming (in Rust) and Tasting (in Beer)](http://www.meetup.com/Rust-NYC/events/230401806/).
* [5/10. (San Diego) Eat– Drink– Rust! Downtown Rust Meetup](http://www.meetup.com/San-Diego-Rust/events/230492925/).
* [5/11. Rust Boulder/Denver Monthly Meeting](http://www.meetup.com/Rust-Boulder-Denver/).
* [5/11. Rust LA Monthly Meetup - Hack Night](http://www.meetup.com/Rust-Los-Angeles/events/230365091/).
* 5/11. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [5/12. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [5/12. Rust Bay Area](http://www.meetup.com/Rust-Bay-Area/events/230624722/).
* [5/13. Rust Meetup Darmstadt](http://www.meetup.com/de-DE/Rust-Rhein-Main/events/230396961/).
* [5/13. Hack Chiapas, Autonomous University of Chiapas, Mexico](http://hackchiapas.com/).
* [5/14. Rust Minsk](http://www.meetup.com/Rust-%D0%B2-%D0%9C%D0%B8%D0%BD%D1%81%D0%BA%D0%B5/events/229283305/).
* [5/16. Rust Paris](http://www.meetup.com/Rust-Paris).
* [5/17. Moscow Rust Conference](https://rustycrate.ru/%D0%BD%D0%BE%D0%B2%D0%BE%D1%81%D1%82%D0%B8/2016/04/25/colaboratory-rust.html).
* 5/18. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [5/18. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).

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

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
