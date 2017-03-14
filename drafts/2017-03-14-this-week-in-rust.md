Title: This Week in Rust 173
Number: 173
Date: 2017-03-14
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

* [Rust's type system is Turing-complete: An exploration of type-level programming in Rust](https://sdleffler.github.io/RustTypeSystemTuringComplete/). A response to [Rust's type system is not Turing complete](https://www.reddit.com/r/rust/comments/5y4x9r/challenge_rusts_type_system_is_not_turing_complete/) challenge.

## Other Weeklies from Rust Community

# Crate of the Week

This week's crate of the week is [µtest](https://github.com/japaric/utest), a testing framework for embedded software. Thanks to [nasa42](https://users.rust-lang.org/users/nasa42) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [The Underhanded Rust Contest](https://underhanded.rs/blog/2016/12/15/underhanded-rust.en-US.html).
* [Rails Girls Summer of Code + Servo](https://blog.servo.org/2017/02/27/rgsoc/).
* [easy] [servo: Looking for something to work on](https://github.com/servo/servo/issues/15162).
* [easy] [clippy:  Extend option-map-unwrap-or-else to Result](https://github.com/Manishearth/rust-clippy/issues/1590).
* [easy] [clippy: Always true expressions in if expressions](https://github.com/Manishearth/rust-clippy/issues/1593).
* [easy] [clippy: Spot wrong usage of bitwise and operator](https://github.com/Manishearth/rust-clippy/issues/1594).
* [easy] [clippy: should_implement_trait should include ToOwned, FromStr](https://github.com/Manishearth/rust-clippy/issues/1600).
* [easy] [crates.io: bad appveyor badges for projects with underscore](https://github.com/rust-lang/crates.io/issues/587).
* [easy] [crates.io: Document applying categories/adding new categories](https://github.com/rust-lang/crates.io/issues/544).
* [medium] [crates.io: Be able to search within a keyword or category](https://github.com/rust-lang/crates.io/issues/491).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

142 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?page=6&q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-03-06..2016-03-13

* [1.16 release notes](https://github.com/rust-lang/rust/pull/39835) (merged prematurely)
* [beta backports](https://github.com/rust-lang/rust/pull/40401)
* [handle procedural macros during `save-analysis`](https://github.com/rust-lang/rust/pull/40311)
* [export attributes in `save-analysis`](https://github.com/rust-lang/rust/pull/39820)
* [rustc: exit quickly on only `--emit dep-info`](https://github.com/rust-lang/rust/pull/40336)
* [`save-analysis` now works in the presence of type errors](https://github.com/rust-lang/rust/pull/40344)
* [fix ICE on constant enums](https://github.com/rust-lang/rust/pull/40285)
* [fix incorrect span label formatting](https://github.com/rust-lang/rust/pull/40287)
* [fix const expressions with macros](https://github.com/rust-lang/rust/pull/40272)
* [fix `personality_fn` within compiler_builtins](https://github.com/rust-lang/rust/pull/40254)
* [new `syntax::ItemKind::MacroDef`, nix unnamed macros](https://github.com/rust-lang/rust/pull/40220) (breaking change)
* [update syntax for `pub(restricted)`](https://github.com/rust-lang/rust/pull/40340)
* [spans for individual path segments](https://github.com/rust-lang/rust/pull/40369)
* [fix obligation cause body mismatch](https://github.com/rust-lang/rust/pull/40404)
* [No more StorageLive for the `!` type](https://github.com/rust-lang/rust/pull/40372)
* [no more MIR pass plugins](https://github.com/rust-lang/rust/pull/40239) (even if there ever was one)
* [MIR inlining](https://github.com/rust-lang/rust/pull/39648)
* placement-in for [`HashMap`](https://github.com/rust-lang/rust/pull/40390), [`VecDeque`](https://github.com/rust-lang/rust/pull/40389)
* [reduce adaptive `HashMap` size overhead](https://github.com/rust-lang/rust/pull/40237)
* [`OsString::shrink_to_fit`](https://github.com/rust-lang/rust/pull/40410)
* [fix rustdoc display of associated consts](https://github.com/rust-lang/rust/pull/40419)
* [improve rustdoc sidebar style](https://github.com/rust-lang/rust/pull/40265)
* [decimate the old `configure` script further](https://github.com/rust-lang/rust/pull/39770)
* [rustbuild: fix compiler docs](https://github.com/rust-lang/rust/pull/40448)
* [rustbuild now copies instead of links](https://github.com/rust-lang/rust/pull/39518)
* [crates.io now uses Diesel](https://github.com/rust-lang/crates.io/pull/609)

## New Contributors

* Ben Schreiber
* deso
* lukaramu
* Maik Riechert

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


* [disposition: merge] [Write to standard error with `eprint!` and `eprintln!`](https://github.com/rust-lang/rfcs/pull/1869).
* [disposition: merge] [extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).
* [disposition: merge] [`From<&[T]> for Rc<[T]> + From<&str> for Rc<str>`](https://github.com/rust-lang/rfcs/pull/1845).
* [disposition: close] [Add variable-length arrays to the language](https://github.com/rust-lang/rfcs/pull/1808).
* [disposition: merge] [Extend entry API to work on borrowed keys](https://github.com/rust-lang/rfcs/pull/1769).
* [disposition: merge] [Deprecate anonymous parameters](https://github.com/rust-lang/rfcs/pull/1685).

## New RFCs

* [Intra Rustdoc Links](https://github.com/rust-lang/rfcs/pull/1946).
* [Polymorphic Numeric Constants](https://github.com/rust-lang/rfcs/pull/1945).
* [Improve match ergonomics](https://github.com/rust-lang/rfcs/pull/1944).
* [Guess diagnostics](https://github.com/rust-lang/rfcs/pull/1941). `rustfix` should be the semantic companion to `rustfmt`, which automatically changes code to be more idiomatic.
* [Support the `#[must_use]` attribute on arbitrary functions](https://github.com/rust-lang/rfcs/pull/1940).
* [Allow the `?` operator to be used in `main`, and in `#[test]` functions and doctests](https://github.com/rust-lang/rfcs/pull/1937).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

PRs in final comment period:

* [structs and unions](https://github.com/rust-lang-nursery/fmt-rfcs/pull/53)

Issues in final comment period:

* [where clauses](https://github.com/rust-lang-nursery/fmt-rfcs/issues/38)

Other significant issues:

* [expressions (tracking issue)](https://github.com/rust-lang-nursery/fmt-rfcs/issues/16)

# Upcoming Events

* [Mar  9. San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/237602716/).
* [Mar  9. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/237525355/).
* [Mar  9. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Mar 11. Rust NYC - Rust Hack & Learn](https://www.meetup.com/Rust-NYC/events/238057861/).
* [Mar 13. Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/237058819/).
* [Mar 15. Rust Meetup Hamburg - Rust/Ethereum Meetup](https://www.meetup.com/Rust-Meetup-Hamburg/events/237858112/).
* [Mar 15. Rust Los Angeles - Rust LA Monthly Meetup - Hack Night](https://www.meetup.com/Rust-Los-Angeles/events/237757181/).
* [Mar 15. Rust Dublin - Rust Lightning Talks](https://www.meetup.com/Rust-Dublin/events/237883717/).
* [Mar 15. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar 15. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar 16. Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/de-DE/Rust-Modern-Systems-Programming-in-Leipzig/events/237780401/).
* [Mar 16. Thompson Rivers University, BC Canada - Get Rusty](https://www.eventbrite.ca/e/get-rusty-tickets-31407199780).
* [Mar 21. Rust Paris Meetup #36](https://www.meetup.com/Rust-Paris/events/238240907/).
* [Mar 22. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar 22. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar 23. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
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

> I sure need to work on my quotes.

— [llogiq on his QotW from last week](https://www.reddit.com/r/rust/comments/5wt2vq/this_week_in_rust_171/deczcbo/).

Thanks to [slashgrin for the suggestion](https://www.reddit.com/r/rust/comments/5wt2vq/this_week_in_rust_171/ded0un1/).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
