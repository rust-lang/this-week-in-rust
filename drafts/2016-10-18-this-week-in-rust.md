Title: This Week in Rust 152
Number: 152
Date: 2016-10-18
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

* [Exploring ARM inline assembly in Rust](http://embed.rs/articles/2016/arm-inline-assembly-rust/). Shows some of the usefulness and plenty of the pitfalls of `asm!`.

## News & Project Updates

## Other Weeklies from Rust Community

## New Crates

# Crate of the Week

*No crate was selected for CotW.*

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: tidy script no longer checks lang features](https://github.com/rust-lang/rust/issues/37013).
* [easy] [Servo: Stylo: Implement font-kerning](https://github.com/servo/servo/issues/13667).
* [hard] [rust: Optimize emscripten targets with emcc](https://github.com/rust-lang/rust/issues/36899).
* [hard] [rust: Tell emscripten to remove exception handling code when the panic runtime is used](https://github.com/rust-lang/rust/issues/36900).
* [easy] [imag: Iterator for Iterator<Item = Result<T, Error>> tracing (wanna learn how to implement iterators?)[https://github.com/matthiasbeyer/imag/issues/813]

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

135 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-10-03..2016-10-10

* [std: Stabilize and deprecate APIs for 1.13](https://github.com/rust-lang/rust/pull/36815).
* [Add ThreadId for comparing threads](https://github.com/rust-lang/rust/pull/36341).
* [Add support for per-target rustflags in `.cargo/config`](https://github.com/rust-lang/cargo/pull/3157).
* [Speed up `plug_leaks`](https://github.com/rust-lang/rust/pull/36917).
* [Cargo: Add `--message-format` flag](https://github.com/rust-lang/cargo/pull/3000).
* [Add Thumbs target definitions to the compiler](https://github.com/rust-lang/rust/pull/36874).
* [rustc: Rename `rustc_macro` to `proc_macro`](https://github.com/rust-lang/rust/pull/36945).
* [Clarify HashMap's capacity handling](https://github.com/rust-lang/rust/pull/36766).
* [Enforce the shadowing restrictions from RFC 1560 for today's macros](https://github.com/rust-lang/rust/pull/36767).
* [Restrict where in the tree platform-specific cfgs may be mentioned](https://github.com/rust-lang/rust/pull/36807).
* [Improve error message and snippet for "did you mean `x`"](https://github.com/rust-lang/rust/pull/36798).
* [Cargo: Warn about path overrides that won't work](https://github.com/rust-lang/cargo/pull/3136).
* [Refactoring/bugfixing around definitions for struct/variant constructors](https://github.com/rust-lang/rust/pull/36814).
* [std: Correct stability attributes for some implementations](https://github.com/rust-lang/rust/pull/36902).

## New Contributors

* Anthony Ramine
* Christopher
* Eric Roshan-Eisner
* Florian Diebold
* KillTheMule
* Mathieu Borderé
* Nick Stevens
* p512
* Razican
* Stephen M. Coakley
* 石博文

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

* [Propose a shorthand syntax for constructing struct-like values with _named_ fields](https://github.com/rust-lang/rfcs/pull/1682).
* [Let a `loop { ... }` expression return a value via `break my_value;`](https://github.com/rust-lang/rfcs/pull/1624).

## New RFCs

* [Extend entry API to work on borrowed keys](https://github.com/rust-lang/rfcs/pull/1769).
* [Abort by default v2](https://github.com/rust-lang/rfcs/pull/1765). Specify abort-by-default in `Cargo.toml` when the user does `cargo new --bin`, as well as various other refinements to the panick strategy system.

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

FCP issues:

* [Guiding principles](https://github.com/rust-lang-nursery/fmt-rfcs/issues/4).
* [Comments](https://github.com/rust-lang-nursery/fmt-rfcs/issues/17).

Other issues getting a lot of discussion:

* [Imports (`use`)](https://github.com/rust-lang-nursery/fmt-rfcs/issues/24).
* [Boolean and arithmetic expressions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/18).
* [Statements](https://github.com/rust-lang-nursery/fmt-rfcs/issues/11).

_No PRs this week._

# Upcoming Events

* [10/13. Rust Orange County Inaugural Meetup](https://www.meetup.com/Rust-Los-Angeles/events/234277000/).
* [10/13. Columbus Rust Society](https://www.meetup.com/columbus-rs/events/233996456/).
* [10/15. South Florida Rust Meetup](http://www.meetup.com/South-Florida-Rust-Meetup/events/234791780/).
* [10/18. London Rust User Group Meetup #9](https://www.meetup.com/Rust-London-User-Group/events/233034964/).
* [10/19. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [10/19. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [10/20. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [10/27 and 10/28 Rust Belt Rust in Pittsburgh, US](http://www.rust-belt-rust.com/). Tickets still available.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# fn work(on: RustProject) -> Money

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Friends of the Forest

Our community likes to recognize people who have made outstanding contributions
to the Rust Project, its ecosystem, and its community. These people are
'friends of the forest'.

This week's friends of the forest are:

* From [AtheMathmo]:

> I'd like to nominate [bluss] for his work on scientific programming in Rust.
> [ndarray] is a monumental project but in addition to that he has worked
> (really) hard to share that knowledge among others and provided easy-to-use
> libraries like [matrixmultiply]. Without bluss' assistance rulinalg would be
> in a far worse state.

* From [nasa42]:

> I'd like to nominate [Yehuda Katz], the lord of package managers.

[AtheMathmo]: https://users.rust-lang.org/t/twir-friends-of-the-forest/7295/9
[bluss]: https://github.com/bluss
[ndarray]: https://github.com/bluss/rust-ndarray
[matrixmultiply]: https://github.com/bluss/matrixmultiply
[nasa42]: https://www.reddit.com/r/rust/comments/576h2q/this_week_in_rust_151/d8pca3k
[Yehuda Katz]: https://github.com/wycats

[Submit your Friends-of-the-Forest nominations for next week][foft]!

[foft]: https://users.rust-lang.org/t/twir-friends-of-the-forest/7295

# Quote of the Week

> < Celti> I just had a recruiter contact me for a Rust job requiring 3+ years of professional experience with it.

— From [#rust](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust).

Thanks to [bluss](https://users.rust-lang.org/users/bluss) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
