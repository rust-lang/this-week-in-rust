Title: This Week in Rust 157
Number: 157
Date: 2016-11-22
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

* [Rust and the future of systems programming](https://medium.com/mozilla-tech/rust-and-the-future-of-systems-programming-b75fba746910).
* [sccache, Mozilla’s distributed compiler cache, now written in Rust](https://blog.mozilla.org/ted/2016/11/21/sccache-mozillas-distributed-compiler-cache-now-written-in-rust/).
* [Introducing Reqwest](http://seanmonstar.com/post/153221119046/introducing-reqwest). Reqwest is an ergonomic HTTP Client for Rust.
* [Putting the Rust in Trust](https://robigalia.org/blog/2016/11/15/verfication.html). A path to Rust formal verification.
* [Measuring the impact of github bots on Rust community participation](http://sarah.thesharps.us/2016/11/17/impact-of-bots-on-github-communities/).
* [Computing the result of the 2016 Australian election with Rust](https://medium.com/@michaelsproul/how-to-calculate-a-nation-states-election-result-in-your-bedroom-30f0c5d905af).
* [Unlocking the power of parallelism with Rust](https://medium.com/mozilla-tech/unlocking-the-power-of-parallelism-with-rust-19f7d565d6d).
* [A deep dive into TiKV](https://pingcap.github.io/blog/2016/11/09/Deep-Dive-into-TiKV/). TiKV is a distributed Key-Value database which is based on the design of Google Spanner, F1, and HBase.
* [Packaging Rust binaries](https://chrismacnaughton.com/blog/2016/11/21/packaging-rust-binaries/). Making Rust packages for Linux with Snappy.
* [Rust from a hacker's point of view](https://medium.com/@opensourcegeekz/rust-from-a-hackers-point-of-view-1aa0630ce57d).
* [Understanding lifetimes in Rust](https://dev.to/buntine/aha-understanding-lifetimes-in-rust).
* [Exposing Rust objects to C code](https://people.gnome.org/~federico/news-2016-11.html#14).
* [Debugging Rust code inside a C library](https://people.gnome.org/~federico/news-2016-11.html#16).
* [Rustup tutorial: Downgrading nightlies](https://tureus.github.io/rust/2015/11/16/rustup-tutorial-downgrading.html).

## News & Project Updates

* [The MSP430 LLVM backend has been enabled in the latest nightly, please test it](https://users.rust-lang.org/t/the-msp430-llvm-backend-has-been-enabled-in-the-latest-nightly-please-test-it/8045)!
* [Rust documentation is now optional and opt-in when installing via rustup](https://users.rust-lang.org/t/psa-rust-documentation-is-now-optional-and-opt-in-when-installing-via-rustup/8072).
* [Discovery: Discover the world of microcontrollers through Rust](https://users.rust-lang.org/t/discovery-discover-the-world-of-microcontrollers-through-rust/8015)! Discovery is a book about (programming) embedded systems / microcontrollers aimed at people without previous experience with them.

## Other Weeklies from Rust Community

* [This week in Rust docs 31](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-31). Updates from the Rust documentation team.
* [What's coming up in imag (20)](http://beyermatthias.de/blog/2016/11/18/what-s-coming-up-in-imag-20/). imag is a text based personal information management suite.
* [This week in TiKV 2016-11-21](http://weekly.pingcap.com/2016/11/21/tidb-weekly/#weekly-update-in-tikv). TiKV is a distributed Key-Value database.
* [This week in PlanetKit 4](https://jeffparsons.github.io/2016/11/18/fix-cracks/). PlanetKit generates colorful blobs that might one day resemble planets.

# Crate of the Week

This week's Crate of the Week is [cargo-benchcmp](https://github.com/BurntSushi/cargo-benchcmp). `cargo-benchcmp` generates nice before-after summaries for benchmarks.

Thanks to [bluss](https://users.rust-lang.org/users/bluss) for this week's suggestion. [Submit your suggestions and votes for next week][submit_crate]!

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

105 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-11-14..2016-11-21

* [Add `std::process::abort`](https://github.com/rust-lang/rust/pull/37833).
* [Remove `scope_auxiliary`](https://github.com/rust-lang/rust/pull/37764).
* [rustdoc: add cli argument `--playground-url`](https://github.com/rust-lang/rust/pull/37763).
* [Don't clone in `UnificationTable::probe()`](https://github.com/rust-lang/rust/pull/37848).
* [enable the MSP430 LLVM backend](https://github.com/rust-lang/rust/pull/37672).
* [rustc: Implement `#[link(cfg(..))]` and crt-static](https://github.com/rust-lang/rust/pull/37545).
* [Stabilize RFC 1560 (Changes to name resolution)](https://github.com/rust-lang/rust/pull/37127).
* [Fix grammar verification](https://github.com/rust-lang/rust/pull/37607).
* [Improve `.chars().count()`](https://github.com/rust-lang/rust/pull/37888).
* [Optimise `Chars::last()`](https://github.com/rust-lang/rust/pull/37882).
* [Macro parser performance improvements and refactoring](https://github.com/rust-lang/rust/pull/37701).

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
