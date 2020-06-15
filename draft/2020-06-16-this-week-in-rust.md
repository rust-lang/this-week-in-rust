Title: This Week in Rust 343
Number: 343
Date: 2020-06-16
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

There is no *This Week in Rust* podcast this week, next week's episode will cover both this week and next week.

Check out [this week's *This Week in Rust Podcast*](https://rustacean-station.org/episode/017-twir-341-342/)

# Updates from Rust Community

## News & Blog Posts

* [RustConf 2020 Registration is Open](https://rustconf.com/)
* [You Want to Learn Rust but You Don’t Know Where to Start](https://towardsdatascience.com/you-want-to-learn-rust-but-you-dont-know-where-to-start-fc826402d5ba)
* [7 Awesome Rust-powered Command-line Utilities](https://towardsdatascience.com/awesome-rust-powered-command-line-utilities-b5359c38692)
* [Rust-Powered Command-Line Utilities to Increase Your Productivity](https://towardsdatascience.com/rust-powered-command-line-utilities-to-increase-your-productivity-eea03a4cf83a)
* [Microsoft: Rust is the Industry's 'Best Chance' at Safe Systems Programming](https://thenewstack.io/microsoft-rust-is-the-industrys-best-chance-at-safe-systems-programming/)
* [Tour of Rust: Chapter 7 - Object Oriented Programming](https://tourofrust.com/chapter_7_en.html)
* [Rust Analyzer Changelog #29](https://rust-analyzer.github.io/thisweek/2020/06/15/changelog-29.html)
* [Rustls Security Review & Audit Report](https://github.com/ctz/rustls/blob/master/audit/TLS-01-report.pdf)
* [NDArray Index Arrays and Mask Index Arrays](https://shahinrostami.com/posts/programming/rust-notebooks/ndarray-index-arrays-and-mask-index-arrays/)
* [audio] [AreWePodcastYet - Interview with Tim McNamara, author of Rust in Action](https://soundcloud.com/arewepodcastyet/awpy-05-tim-mcnamara-timclicks)
* [video] [Rust Notebooks (Jupyter and Evcxr) - Getting Started](https://www.youtube.com/watch?v=SZKEzNL9als)

# Crate of the Week

This week's crate is [cargo-spellcheck](https://github.com/drahnr/cargo-spellcheck), a cargo subcommand to spell-check your docs.

Thanks to [Bernhard Schuster](https://users.rust-lang.org/t/crate-of-the-week/2704/777) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [gooseberry: Add related tags to each page](https://github.com/out-of-cheese-error/gooseberry/issues/3)
* [gooseberry: General contributing](https://github.com/out-of-cheese-error/gooseberry/blob/master/CONTRIBUTING.md)
* [rust: `fs::remove_dir_all` rarely succeeds for large directories on windows](https://github.com/rust-lang/rust/issues/29497#issuecomment-573353391)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

350 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-06-01..2020-06-08

* [InstCombine: don't optimize `&mut *x` into `x`](https://github.com/rust-lang/rust/pull/72820)
* [add `-Z span-debug` to allow for easier debugging of proc macros](https://github.com/rust-lang/rust/pull/72799)
* [avoid setting wrong obligation cause span of associated type mismatch](https://github.com/rust-lang/rust/pull/72807)
* [be more careful around `ty::Error` in generators](https://github.com/rust-lang/rust/pull/72764)
* [fulfill: try using `SmallVec` or `Box` for `stalled_on`](https://github.com/rust-lang/rust/pull/72776)
* [`impl AsRef<[T]> for vec::IntoIter<T>`](https://github.com/rust-lang/rust/pull/72583)
* [chalk: get ready for the first publish](https://github.com/rust-lang/chalk/pull/483)
* [free `default()` forwarding to `Default::default()`](https://github.com/rust-lang/rust/pull/73001)
* [stabilize `std::io::Buf{Reader, Writer}::capacity`](https://github.com/rust-lang/rust/pull/72924)
* [add associated consts `MIN`/`MAX` for `Wrapping<Int>`](https://github.com/rust-lang/rust/pull/72891)
* [de-promote Duration::from_secs](https://github.com/rust-lang/rust/pull/71796)
* [compiler-builtins: manually patch ret instruction for LVI](https://github.com/rust-lang/compiler-builtins/pull/359)
* [cargo: add environment variables to identify the binary and crate name](https://github.com/rust-lang/cargo/pull/8270)
* [cargo: allow Windows dylibs without dll suffix](https://github.com/rust-lang/cargo/pull/8310)
* [cargo: better error message when passing in relative path to `Workspace::new`](https://github.com/rust-lang/cargo/pull/8321)
* [cargo: don't hash executable filenames on apple platforms](https://github.com/rust-lang/cargo/pull/8329)
* [cargo: support `{prefix}` and `{lowerprefix}` markers in `config.json` `dl` key](https://github.com/rust-lang/cargo/pull/8267)
* [cargo: warn if using hash in git URL](https://github.com/rust-lang/cargo/pull/8297)
* [cargo: reset lockfile information between resolutions](https://github.com/rust-lang/cargo/pull/8274)
* [crates.io: fix issue where crates.io allowed the plus sign in crate names](https://github.com/rust-lang/crates.io/pull/2551)
* [docs.rs: print a backtrace for crates which fail to build](https://github.com/rust-lang/docs.rs/pull/823)
* [rustfmt: pick up comments between visibility modifier and item name](https://github.com/rust-lang/rustfmt/pull/4239)
* [rustfmt: preserve Markdown line breaks in inner and outer block doc comments](https://github.com/rust-lang/rustfmt/pull/4233)
* [rustfmt: use rewrite buffer to determine if comment should be on a newline](https://github.com/rust-lang/rustfmt/pull/4229)
* [rustfmt: feat: conditionally allow unstable opts on stable/beta](https://github.com/rust-lang/rustfmt/pull/4228)

## Rust Compiler Performance Triage

* [2020-06-09](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020.md#2020-06-09)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [add Windows system error codes that should map to `io::ErrorKind::TimedOut`](https://github.com/rust-lang/rust/pull/71756)
* [disposition: merge] [impl `PartialEq<Vec<B>> for &[A], &mut [A]`](https://github.com/rust-lang/rust/pull/71660)

## New RFCs

[RFC: add the Freeze trait to libcore/libstd](https://github.com/rust-lang/rfcs/pull/2944)

# Upcoming Events

### Online
* [June 18. Zurich, CH - Remote - Embedded Rust Update: probe.rs](https://www.meetup.com/Rust-Zurich/events/271020472/)
* [June 18. Turin, IT - Remote - Rust Turin Study Group](https://community.mozilla.org/events/gruppo-di-studio-di-rust-2/)
* [June 25. Edinburgh, UK - Remote - Pirrigator - Growing Tomatoes Free From Memory Errors and Race Conditions](https://www.meetup.com/rust-edi/events/271129693/)
* [June 25. Berlin, DE - Remote - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybcjbhc/)

### North America
* [June 17. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybcjbwb/)
* [June 18. Durham, NC - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybcjbdc/)
* [June 30. Dallas, TX - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/nppvrrybcjbnc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> You don't declare lifetimes. Lifetimes come from the shape of your code, so to change what the lifetimes are, you must change the shape of the code.

– [Alice Ryhl on rust-users](https://users.rust-lang.org/t/lifetime-of-a-returned-iterator/43732/2)

Thanks to [RustyYato](https://users.rust-lang.org/t/twir-quote-of-the-week/328/883) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/gvwvep/this_week_in_rust_341/)</small>
