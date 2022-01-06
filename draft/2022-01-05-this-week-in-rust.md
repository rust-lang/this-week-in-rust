Title: This Week in Rust 424
Number: 424
Date: 2022-01-05
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Project/Tooling Updates
* [This Week in Glean: Glean in 2021](https://fnordig.de/2021/12/17/glean-in-2021/)

* [What's new in SeaORM 0.5.0](https://www.sea-ql.org/SeaORM/blog/2022-01-01-whats-new-in-0.5.0/)

* [SixtyFPS (GUI crate): Changelog for 2nd of January 2022](https://sixtyfps.io/thisweek/2022-01-03.html)

### Research

### Observations/Thoughts
* [Profiling linkers](https://fasterthanli.me/articles/profiling-linkers)
* [Debian's approach to Rust - Dependency handling](https://diziet.dreamwidth.org/10559.html)
* [Why is my Rust build so slow?](https://fasterthanli.me/articles/why-is-my-rust-build-so-slow)
* [Nine Rules for Writing Python Extensions in Rust & Rayon](https://towardsdatascience.com/nine-rules-for-writing-python-extensions-in-rust-d35ea3a4ec29?sk=f8d808d5f414154fdb811e4137011437)

### Rust Walkthroughs
* [series] [Don't shell out!](https://fasterthanli.me/series/dont-shell-out)

### Miscellaneous

## Crate of the Week

This week's crate is [fltk-rs](https://crates.io/crates/fltk), a crate with bindings to the [FLTK](https://github.com/fltk/fltk) GUI toolkit.

Thanks to [Mark Summerfield](https://users.rust-lang.org/t/crate-of-the-week/2704/999) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

230 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-12-27..2022-01-03

* [allow loading LLVM plugins with both legacy and new pass manager](https://github.com/rust-lang/rust/pull/91125)
* [suggest `while let x = y` when encountering `while x = y`](https://github.com/rust-lang/rust/pull/92402)
* [refactor variance diagnostics to work with more types](https://github.com/rust-lang/rust/pull/89336)
* [extend check for `UnsafeCell` in consts to cover unions](https://github.com/rust-lang/rust/pull/90383)
* [parse and suggest moving where clauses after equals for type aliases](https://github.com/rust-lang/rust/pull/92118)
* [relax `priv-in-pub` lint on generic bounds and where clauses of trait impls](https://github.com/rust-lang/rust/pull/90586)
* [perf: store liveness in interval sets for region inference](https://github.com/rust-lang/rust/pull/90637)
* [add `try_reserve` and `try_reserve_exact` for `OsString`](https://github.com/rust-lang/rust/pull/92338)
* [support `\[x; n\]` expressions in `concat_bytes!`](https://github.com/rust-lang/rust/pull/92066)
* [std-simd: impl `std::simd::StdFloat`](https://github.com/rust-lang/portable-simd/pull/219)
* [rustdoc: use `ThinVec` for `GenericArgs` bindings](https://github.com/rust-lang/rust/pull/92395)
* [clippy: extend `unused_io_amount` to cover async io](https://github.com/rust-lang/rust-clippy/pull/8179)
* [clippy: fix `enum_variants` false positive on prefixes that are not camel-case](https://github.com/rust-lang/rust-clippy/pull/8127)
* [clippy: fixed issues with `to_radians` and `to_degrees` lints](https://github.com/rust-lang/rust-clippy/pull/8187)
* [clippy: limit the `identity_op` lint to integral operands](https://github.com/rust-lang/rust-clippy/pull/8183)
* [clippy: `erasing_op` lint ignored when operation `Output` type is different from the type of constant `0`](https://github.com/rust-lang/rust-clippy/pull/8204)
* [clippy: perf: cache test item names](https://github.com/rust-lang/rust-clippy/pull/8182)
* [clippy: fix `redundant_closure` fp with `Rc<F>`/`Arc<F>`](https://github.com/rust-lang/rust-clippy/pull/8193)
* [clippy: `wrong_self_convention`: match `SelfKind::No` more restrictively](https://github.com/rust-lang/rust-clippy/pull/8208)
* [rustfmt: do not flatten match arm block with leading attributes](https://github.com/rust-lang/rustfmt/pull/5158)
* [rustfmt: improve out of line module resolution](https://github.com/rust-lang/rustfmt/pull/5142)
* [rustfmt: support parsing of asm! args](https://github.com/rust-lang/rustfmt/pull/5156)

### Rust Compiler Performance Triage

Relatively quiet week, mostly rustdoc improvements.

Triage done by **@simulacrum**.
Revision range: [3d57c61a..e91ad5fc62](https://perf.rust-lang.org/?start=3d57c61a9e04dcd3df633f41142009d6dcad4399&end=e91ad5fc62bdee4a29c18baa5fad2ca42fc91bf4&absolute=false&stat=instructions%3Au)

2 Regressions, 1 Improvements, 6 Mixed; 0 of them in rollups

26 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-12-28.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Amend RFC 2996 to replace `Stream` with  `AsyncIterator`](https://github.com/rust-lang/rfcs/pull/3208)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No new RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Clarify the guarantees that ThreadId does and doesn't make.](https://github.com/rust-lang/rust/pull/84083)
* [disposition: merge] [Remove effect of `#[no_link]` attribute on name resolution](https://github.com/rust-lang/rust/pull/92034)
* [disposition: merge] [Tracking Issue for Stdin::lines, Stdin::split forwarder methods](https://github.com/rust-lang/rust/issues/87096)
* [disposition: merge] [Tracking issue for Result::cloned, Result::cloned_err, Result::copied, Result::copied_err](https://github.com/rust-lang/rust/issues/63168)
* [disposition: merge] [Remove unnecessary bounds for some Hash{Map,Set} methods](https://github.com/rust-lang/rust/pull/91593)

### [New RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No new RFCs were published this week.*

## Upcoming Events

Rusty Events between 12/29/2021 - 1/31/2022 🦀

### Online

* [January 5, 2022 | Indianapolis, IN, US | **Indy.rs - with Social Distancing** | Indy Rust](https://www.meetup.com/indyrs/events/qwtdjsydccbhb/)
* [January 6, 2022 | Cardiff, UK | **Rust Book Study Session - Automated Tests & Building a Command Line Program** | Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/282667031/)
* [January 6, 2022 | Nürnberg, DE | **Rust Nürnberg online #8**| Rust Nuremberg](https://www.meetup.com/rust-noris/events/282344613/)
* [January 8, 2022 | Various cities | **Rust GameDev Monthly Meetup** | Rust GameDev](https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com)
* [January 11, 2022 | Dallas, TX, US | **Second Tuesday Meetup**| Dallas Rust](https://www.meetup.com/Dallas-Rust/events/vqtjcsydccbpb/)
* [January 11, 2022 | Seattle, WA, US | **Monthly meetup** | Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrydccbpb/)
* [January 12, 2022 | Boulder, CO, US | **Monthly Meetup** | Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydccbqb/)
* [January 12, 2022 | Los Angeles, CA, US | **Live Coding Session - Mob Programming a Rust Code Kata [Virtual] Jan. 2022** | Rust Los Angeles](https://www.meetup.com/Rust-Los-Angeles/events/282580016/)
* [January 12, 2022 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/gjrtqsydccbqb/)
* [January 13, 2022 | Warsaw, PL | **Rust Warsaw #4** | Rust Warsaw](https://www.meetup.com/pl-PL/Rust-Warsaw/events/282879405/)
* [January 18, 2022 | Washington, DC, US| **Mid-month Rustful** | Rust DC](https://www.meetup.com/RustDC/events/vdhxgsydccbxb/)
* [January 19, 2022 | Vancouver, BC, CA | **Rust Study/Hack/Hang-out night** |Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydccbzb)
* [January 25, 2022 | Dallas, TX, US | **Last Tuesday Meetup** | Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrydccbhc/)

### North America

* [January 6, 2022 | Austin, TX, US | **Rust Lunch** | Rust ATX](https://www.meetup.com/rust-atx/events/282756864/)
* [January 12, 2022 | Atlanta, GA, US | **Grab a beer with fellow Rustaceans** | Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsydccbqb/)
* [January 13, 2022 | Columbus, OH, US | **Monthly Meeting** | Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgrydccbrb/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I performed an extremely scientific poll on twitter, and determined this is not how it's pronounced
>
> ----
>
> Well, it really is Vec<T, A>, pronounced Veck-tah. 😛
>
> ----
>
> Look, I moved away from Boston to avoid this sort of thing 😄.

– [the8472 & Thom Chiovoloni on github](https://github.com/rust-lang/rust/pull/92463#discussion_r777059401)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1159) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
