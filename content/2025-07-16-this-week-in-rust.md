Title: This Week in Rust 608
Number: 608
Date: 2025-07-16
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

Want TWIR in your inbox? [Subscribe here](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the Linked Page](https://example.com/my_article)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official
* [crates.io: development update | Rust Blog](https://blog.rust-lang.org/2025/07/11/crates-io-development-update-2025-07/)
* [Call for Testing: Speeding up compilation with `hint-mostly-unused`](https://blog.rust-lang.org/inside-rust/2025/07/15/call-for-testing-hint-mostly-unused/)

### Newsletters
* [This Month in Rust OSDev: June 2025](https://rust-osdev.com/this-month/2025-06/)

### Project/Tooling Updates
* [egui 0.32 - Atoms, popups, and better SVG support](https://github.com/emilk/egui/releases/tag/0.32.0)
* [reqwest retries](https://seanmonstar.com/blog/reqwest-retries/)
* [Introducing Rudy: A Toolchain for Rust Debuginfo](https://www.samjs.io/blog/rudy)
* [RootAsRole 3.1.0 - Outperforms sudo, configurability, -u, -g, -E features](https://github.com/LeChatP/RootAsRole/releases/tag/v3.1.0)
* [Helix Release 25.07 Highlights](https://helix-editor.com/news/release-25-07-highlights/)
* [UltraGraph 0.8: 1,300x Faster Graph Analytics — No Cluster Needed](https://deepcausality.com/blog/announcement-ultragraph-0-8)

### Observations/Thoughts
* [placing functions](https://blog.yoshuawuyts.com/placing-functions/)
* [Rust is a great fit for the agentic era](https://kerkour.com/rust-agentic-coding)
* [Here comes the sun: iteratively building a Rust program that gets the current weather conditions](https://bitfieldconsulting.com/posts/here-comes-sun)
* [Thinking in Rust: Ownership, Access, and Memory Safety](https://cocoindex.io/blogs/rust-ownership-access/)
* [Adding lookbehinds to rust-lang/regex](https://systemf.epfl.ch/blog/rust-regex-lookbehinds/)
* [Publish all your crates everywhere all at once](https://www.tweag.io/blog/2025-07-10-cargo-package-workspace/)
* [Variadic Generics ideas that won’t work for Rust](https://poignardazur.github.io/2025/07/09/variadic-generics-dead-ends/)
* [audio] [Traceability](https://sdr-podcast.com/episodes/traceability/)
* [KSAT with Vegard Sandengen](https://corrode.dev/podcast/s04e07-ksat/)

### Rust Walkthroughs
* [Programming Extensible Data Types in Rust with CGP - Part 3: Implementing Extensible Records](https://contextgeneric.dev/blog/extensible-datatypes-part-3/)
* [Axum: Optimizing web API design with the Builder Pattern](https://medium.com/@adefemiadeoye/axum-optimizing-web-api-design-with-the-builder-pattern-08aa8e18a599)
* [Rust unit testing test types](https://jorgeortiz.dev/posts/rust_unit_testing_test_types/)

### Research
* [Tree Borrows](https://plf.inf.ethz.ch/research/pldi25-tree-borrows.html)
* [Securing Mixed Rust with Hardware Capabilities](https://arxiv.org/abs/2507.03344)

### Miscellaneous
* [June 2025 Rust Jobs Report](https://filtra.io/rust/jobs-report/jun-25)

## Crate of the Week

This week's crate is [oxvg](https://github.com/noahbald/oxvg), a SVG optimizer.

Thanks to [Noah Baldwin](https://users.rust-lang.org/t/crate-of-the-week/2704/1450) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

* *No calls for testing were issued this week by [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Diesel: Designing `#[derive(QueryModel)]` Poll I - Default for #[diesel(check_for_backend()]](https://github.com/diesel-rs/diesel/discussions/4680)
* [Diesel: Designing `#[derive(QueryModel)]` Poll II - Joining behaviour for #[diesel(embed)]](https://github.com/diesel-rs/diesel/discussions/4681)
* [Diesel: Designing `#[derive(QueryModel)]` Poll III -  All in one derive? ](https://github.com/diesel-rs/diesel/discussions/4682)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

*No Calls for papers or presentations were submitted this week.*

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

421 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-07-08..2025-07-15

#### Compiler
* [use lld by default on `x86_64-unknown-linux-gnu` stable](https://github.com/rust-lang/rust/pull/140525)
* [apply effects to `otherwise` edge in dataflow analysis](https://github.com/rust-lang/rust/pull/142707)
* [compute all rpitit of a trait](https://github.com/rust-lang/rust/pull/143783)
* [consider nested cases for duplicate RPITIT](https://github.com/rust-lang/rust/pull/143570)
* [propagate from borrowed locals in CopyProp](https://github.com/rust-lang/rust/pull/143624)
* [resolver: refactor macro map into external and local maps](https://github.com/rust-lang/rust/pull/143657)

#### Library
* [constify `Fn*` traits](https://github.com/rust-lang/rust/pull/143640)
* [constify `From` and `Into`](https://github.com/rust-lang/rust/pull/143774)
* [make `Default` const and add some `const Default` impls](https://github.com/rust-lang/rust/pull/134628)
* [slice: mark `rotate_left`, `rotate_right` unstably const](https://github.com/rust-lang/rust/pull/143554)
* [core: add `BorrowedCursor::with_unfilled_buf`](https://github.com/rust-lang/rust/pull/142885)
* [implement `int_format_into` feature](https://github.com/rust-lang/rust/pull/142098)

#### Cargo
* [add `[hints]` table in `Cargo.toml`, and a `hints.mostly-unused` hint](https://github.com/rust-lang/cargo/pull/15673)
* [implementation and tests for `multiple-build-scripts`](https://github.com/rust-lang/cargo/pull/15704)
* [perf: speed up TOML parsing by upgrading toml](https://github.com/rust-lang/cargo/pull/15736)

#### Rustdoc
* [don't mark `#[target_feature]` safe fns as unsafe in rustdoc JSON](https://github.com/rust-lang/rust/pull/143555)

#### Clippy
* [`arithmetic_side_effects`: don't warn on `NonZeroU*.get() - 1`](https://github.com/rust-lang/rust-clippy/pull/15238)
* [`or_fun_call`: lint method calls inside `map_or` first arg](https://github.com/rust-lang/rust-clippy/pull/15074)
* [`{flat_,}map_identity`: recognize `|[x, y]| [x, y]` as an identity function as well](https://github.com/rust-lang/rust-clippy/pull/15229)
* [add `uninlined_format_args` example for `{:?}`](https://github.com/rust-lang/rust-clippy/pull/15228)
* [do not remove method call if type is adjusted](https://github.com/rust-lang/rust-clippy/pull/15181)
* [fix `approx_const` for some new cases](https://github.com/rust-lang/rust-clippy/pull/15236)
* [fix `expect_fun_call` producing invalid suggestions](https://github.com/rust-lang/rust-clippy/pull/15122)
* [fix `legacy_numeric_constants` suggestion when call is wrapped in parens](https://github.com/rust-lang/rust-clippy/pull/15191)
* [fix `manual_abs_diff` suggests wrongly behind refs](https://github.com/rust-lang/rust-clippy/pull/15265)
* [fix `manual_assert` suggests wrongly for macros](https://github.com/rust-lang/rust-clippy/pull/15264)
* [fix `manual_is_variant_and` condition generation](https://github.com/rust-lang/rust-clippy/pull/15206)
* [fix false negative of `expect_used`](https://github.com/rust-lang/rust-clippy/pull/15253)
* [fix manual is multiple of](https://github.com/rust-lang/rust-clippy/pull/15205)
* [fix multiple problems in #15063](https://github.com/rust-lang/rust-clippy/pull/15070)
* [fix suggestion causes error of `needless_for_each`](https://github.com/rust-lang/rust-clippy/pull/15262)
* [skip exit late lint pass on tests](https://github.com/rust-lang/rust-clippy/pull/15222)

#### Rust-Analyzer
* [rust-analyzer: generate `new` for tuple `struct`](https://github.com/rust-lang/rust-analyzer/pull/20109)
* [rust-analyzer: support folding multiline arg list & fn body in one folding range](https://github.com/rust-lang/rust-analyzer/pull/20054)
* [rust-analyzer: assoc type where clause position](https://github.com/rust-lang/rust-analyzer/pull/20235)
* [rust-analyzer: fix display of `use<>` syntax](https://github.com/rust-lang/rust-analyzer/pull/20228)
* [rust-analyzer: fixes for `dyn` inlay hint](https://github.com/rust-lang/rust-analyzer/pull/20212)
* [rust-analyzer: inline asm fixes](https://github.com/rust-lang/rust-analyzer/pull/20210)
* [rust-analyzer: normalize projection types before calculating memory maps](https://github.com/rust-lang/rust-analyzer/pull/20232)
* [rust-analyzer: perf: put the expression stuff in the expression store behind an `Option<Box>`](https://github.com/rust-lang/rust-analyzer/pull/20219)

### Rust Compiler Performance Triage

A busy week with a lot of rollups containing perf. regressions and mixed results. Overall, regressions won slightly, but there were also a few impressive wins on a few primary and secondary benchmarks. A lot of perf. effects are caused by the current rework of attribute parsing, which will hopefully result in slightly improved performance once it's finished.

Triage done by **@kobzol**.
Revision range: [0d11be5a..a9fb6103](https://perf.rust-lang.org/?start=0d11be5aabe0cd49609fff5fce57c4691a22fe55&end=a9fb6103b05c6ad6eee6bed4c0bb5a2e8e1024c6&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.5%  | [0.1%, 1.5%]    | 62    |
| Regressions ❌ <br /> (secondary)  | 0.5%  | [0.1%, 1.8%]    | 78    |
| Improvements ✅ <br /> (primary)   | -0.4% | [-3.9%, -0.1%]  | 40    |
| Improvements ✅ <br /> (secondary) | -1.4% | [-11.6%, -0.0%] | 74    |
| All ❌✅ (primary)                 | 0.1%  | [-3.9%, 1.5%]   | 102   |

5 Regressions, 4 Improvements, 8 Mixed; 5 of them in rollups
47 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/87d471ce81dd139cca60ee46377a4cf5c131f7cc/triage/2025/2025-07-15.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Guarantee 8 bytes of alignment in Thread::into_raw](https://github.com/rust-lang/rust/pull/143859)
* [rustdoc: add ways of collapsing all impl blocks](https://github.com/rust-lang/rust/pull/141663)
* [Stabilize `const_float_round_methods`](https://github.com/rust-lang/rust/pull/143604)
* [Tracking Issue for `#![feature(const_float_round_methods)]`](https://github.com/rust-lang/rust/issues/141555)
* [Add `target_env = "macabi"` and `target_env = "sim"`](https://github.com/rust-lang/rust/pull/139451)


##### [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
* [Closing issues relevant to T-lang on this repo](https://github.com/rust-lang/rfcs/issues/3756)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [repr(scalable)](https://github.com/rust-lang/rfcs/pull/3838)

## Upcoming Events

Rusty Events between 2025-07-16 - 2025-08-13 🦀

### Virtual
* 2025-07-16 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731031)
* 2025-07-17 | Hybrid (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**July, 2025 Computer Programming Language Panel (Special Event)**](https://www.meetup.com/seattle-rust-user-group/events/307698855)
* 2025-07-17 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820305)
* 2025-07-20 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/308383001)
* 2025-07-22 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/tgctrtyhckbdc)
* 2025-07-22 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Crates, Tips & Tricks Lightning Talks - Bring your ideas!**](https://www.meetup.com/women-in-rust/events/307560304)
* 2025-07-24 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567874)
* 2025-07-27 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/bhctrtyhckbkc)
* 2025-07-31 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820306)
* 2025-08-02 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763838567)
* 2025-08-03 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/bhctrtyhclbfb)
* 2025-08-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyhclbjb)
* 2025-08-10 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/bhctrtyhclbnb)
* 2025-08-12 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361531)

### Asia
* 2025-07-19 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi)
    * [**Rust Delhi Meetup #11**](https://www.meetup.com/rustdelhi/events/308666751)
* 2025-07-26 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**July 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/july-2025-rustacean-meetup/)

### Europe
* 2025-07-23 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund/)
    * [**Rust Dortmund Meetup - Teach and Hack**](https://www.meetup.com/rust-dortmund/events/308517530/)
* 2025-07-24 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**July talks: A Crab, a Pufferfish and a State-of-the-art Chess AI**](https://www.meetup.com/rust-and-friends/events/308687848)
* 2025-07-24 | Nuremberg/Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567874/)
* 2025-07-26 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #15**](https://www.meetup.com/stockholm-rust/events/309275728)
* 2025-07-29 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Lightning Talks July 2025**](https://www.meetup.com/rust-manchester/events/308085035)
* 2025-07-29 | Prague, CZ | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic)
    * [**Nix Meetup at Braiins :)**](https://www.meetup.com/rust-czech-republic/events/308963318)
* 2025-07-30 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group)
    * [**Rust Meetup @ BlockTech**](https://www.meetup.com/rust-amsterdam-group/events/308548455)
* 2025-07-31 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #14: Prof. Dr. Claudia Meitinger - Embassy - Möglichkeiten und Herausforderungen im Modul "Interdisciplinary Project"**](https://rust-augsburg.github.io/meetup/Meetup_14.html)
* 2025-08-06 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 08 2025**](https://lu.ma/eoydaar9)
* 2025-08-13 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944036)

### North America
* 2025-07-16 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731031)
* 2025-07-17 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/308979091)
* 2025-07-17 | Hybrid (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**July, 2025 Computer Programming Language Panel (Special Event)**](https://www.meetup.com/seattle-rust-user-group/events/307698855)
* 2025-07-17 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Rust on Bare Metal Series 1 : Introduction to Embedded Development**](https://www.meetup.com/music-city-rust-developers/events/304333113)
* 2025-07-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/308791385)
* 2025-07-24 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/xdxtqtyhckbgc)
* 2025-07-24 | México City, MX | [Rust MX](https://www.meetup.com/rust-mx)
    * [**Construyendo un Runtime Asíncrono desde Cero en Rust**](https://www.meetup.com/rust-mx/events/309687971)
* 2025-07-31 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/308675947)
* 2025-08-07 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**macros!**](https://www.meetup.com/stl-rust/events/306648747)
* 2025-08-12 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/308284338)

### South America
* 2025-07-17 | Florianópolis, BR | [Rust Brazil + Rust Floripa](https://lu.ma/calendar/cal-iOloL5ZqswCO5Mm)
    * [**Rust Floripa**](https://lu.ma/p0umq6vm)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs
<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1llcso7/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Unfortunately -∞ doesn't fit in `usize`.

– [Tomek Czajka on rust-users](https://users.rust-lang.org/t/enumerations-how-are-they-stored-and-other-questions/131667/31)

Thanks to [Kyllingene](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1703) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
