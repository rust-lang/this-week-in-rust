Title: This Week in Rust 612
Number: 612
Date: 2025-08-13
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
* [Announcing Rust 1.89.0 | Rust Blog](https://blog.rust-lang.org/2025/08/07/Rust-1.89.0/)


### Project/Tooling Updates
* [TangleGuard: Software Architecture Monitoring exclusively for Rust](https://tangleguard.com/)
* [redb v3.0.0 - pure Rust embedded key-value store](https://github.com/cberner/redb/releases/tag/v3.0.0)
* [serde-ply - Modern serde (de)serializer for Ply files](https://www.reddit.com/r/rust/comments/1mp147s/serdeply_modern_speed_convenience_for_a_90s_format/))
* [Bevy's Fifth Birthday](https://bevy.org/news/bevys-fifth-birthday/)
* [warp v0.4](https://seanmonstar.com/blog/warp-v04/)

### Observations/Thoughts
* [Building an asynchronous FUSE Filesystem in Rust](https://r2cn.dev/blog/building-an-asynchronous-fuse-filesystem-in-rust)
* [Nine Rules for Generalizing Your Rust Library: Lessons from Extending RangeSetBlaze to Maps (Part 1)](https://medium.com/@carlmkadie/nine-rules-for-generalizing-your-rust-library-part-1-9f2b08fb5df4)
* [Send/Sync Secret That Separates Professional From Amateur](https://blog.cuongle.dev/p/this-sendsync-secret-separates-professional-and-amateur)
* [hyper HTTP/2 (Didn't) MadeYouReset](https://seanmonstar.com/blog/hyper-http2-didnt-madeyoureset/)
* [video] [BaM #29 - Improving the Rust embedded firmware](https://www.youtube.com/live/5Ca6pQQB-mg?si=yHFQMsDbHEXEfpig)
* [video] [David Sankel – Rust and C++ Interop](https://www.youtube.com/watch?v=xihX4RzStYk)

* [Are we Teaching Rust Effectively?](https://blog.kodewerx.org/2025/08/are-we-teaching-rust-effectively.html)

### Rust Walkthroughs

* [Build with Naz: Capturing Real-Time Build Progress from Cargo Using PTY and OSC Sequences](https://developerlife.com/2025/08/10/pty-rust-osc-seq/)
* [Converting FunctionTrace from C to Rust](https://programsareproofs.com/articles/functiontrace-rust-conversion/)
* [video] [Message framing in Rust and Iroh](https://www.youtube.com/watch?v=h6bBLbcj4Vg)


## Crate of the Week

This week's crate is [MOMA](https://crates.io/crates/moma), a framework for Moving Origin Modular Arithmetic, with applications in number theory, cryptography and bioinformatics.

Thanks to [Neil Crago](https://users.rust-lang.org/t/crate-of-the-week/2704/1462) for the self-suggestion!

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

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
* [arcadia - collage creation](https://github.com/Arcadia-Solutions/arcadia/issues/71)
* [arcadia - retrieve collage data](https://github.com/Arcadia-Solutions/arcadia/issues/73)
* [arcadia - API scraper for TVDB](https://github.com/Arcadia-Solutions/arcadia/issues/6)
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

464 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-08-05..2025-08-12

 #### Compiler
* [account for bare tuples and `Pin` methods in field searching logic](https://github.com/rust-lang/rust/pull/144649)
* [detect `struct` construction with private field in field with default](https://github.com/rust-lang/rust/pull/135846)
* [emit `StorageLive` and schedule `StorageDead` for `let`-`else`'s bindings after matching](https://github.com/rust-lang/rust/pull/143028)
* [enforce tail call type is related to body return type in borrowck](https://github.com/rust-lang/rust/pull/144917)
* [fortify generic param default checks](https://github.com/rust-lang/rust/pull/144977)
* [implement `stability_implications` without a visitor](https://github.com/rust-lang/rust/pull/144873)
* [implement declarative (`macro_rules!`) attribute macros](https://github.com/rust-lang/rust/pull/144579) (RFC [#3697](https://rust-lang.github.io/rfcs/3697-declarative-attribute-macros.html))
* [improve suggestion for "missing function argument" on multiline call](https://github.com/rust-lang/rust/pull/144966)
* [mark all deprecation lints in name resolution as deny-by-default and report-in-deps](https://github.com/rust-lang/rust/pull/143929)
* [parser: recover from attributes applied to types and generic args](https://github.com/rust-lang/rust/pull/144195)
* [recover `for PAT = EXPR {}`](https://github.com/rust-lang/rust/pull/145124)
* [preserve the `.debug_gdb_scripts` section](https://github.com/rust-lang/rust/pull/143679)
* [simplify dead code lint](https://github.com/rust-lang/rust/pull/144863)
* [upgrade `semicolon_in_expressions_from_macros` from warn to deny](https://github.com/rust-lang/rust/pull/144369)
 #### Library
* [stabilize `duration_constructors_lite` feature](https://github.com/rust-lang/rust/pull/145135)
* [stabilize `panic_payload_as_str` feature](https://github.com/rust-lang/rust/pull/144861)
* [stabilize `strict_overflow_ops`](https://github.com/rust-lang/rust/pull/144682)
* [stabilize `unsigned_signed_diff` feature](https://github.com/rust-lang/rust/pull/144900)
* [stabilize const `TypeId::of`](https://github.com/rust-lang/rust/pull/144133)
* [stabilize loongarch32 inline asm](https://github.com/rust-lang/rust/pull/144402)
* [constify remaining traits/impls for `const_ops`](https://github.com/rust-lang/rust/pull/143949)
* [implement `continue_ok` and `break_ok` for ControlFlow](https://github.com/rust-lang/rust/pull/140267)
* [optimize `char::is_alphanumeric`](https://github.com/rust-lang/rust/pull/145027)
* [print thread ID in panic message](https://github.com/rust-lang/rust/pull/115746)
* [`std::sys::io::io_slice`: Add UEFI types](https://github.com/rust-lang/rust/pull/144350)
 #### Cargo
* [accessing each build script's `OUT_DIR` and in the correct order](https://github.com/rust-lang/cargo/pull/15776)
 #### Rustdoc
* [search: prefer stable items in search results](https://github.com/rust-lang/rust/pull/141658)
* [fix caching of intra-doc links on reexports](https://github.com/rust-lang/rust/pull/144970)
 #### Clippy
* [fix `infinite_loop` positive](https://github.com/rust-lang/rust-clippy/pull/15157)
* [do not attempt to compute size of a type with escaping lifetimes](https://github.com/rust-lang/rust-clippy/pull/15434)
* [do not lint for `wildcard_imports` in external macro](https://github.com/rust-lang/rust-clippy/pull/15413)
* [fix `&str` type check in `from_str_radix_10`](https://github.com/rust-lang/rust-clippy/pull/15410)
* [fix suggestion for `collapsible_if` and `collapsible_else_if` when the inner `if` is enclosed in parentheses](https://github.com/rust-lang/rust-clippy/pull/15304)
* [move `cognitive_complexity` lint from `nursery` to `restriction`](https://github.com/rust-lang/rust-clippy/pull/15415)
* [move `crosspointer_transmute` from `complexity` to `suspicious`](https://github.com/rust-lang/rust-clippy/pull/15403)
* [optimize `incompatible_msrv` lint](https://github.com/rust-lang/rust-clippy/pull/15422)
* [optimize `needless_bool` lint](https://github.com/rust-lang/rust-clippy/pull/15423)
* [reuse previous `Vec` allocation in loop](https://github.com/rust-lang/rust-clippy/pull/15428)
 #### Rust-Analyzer
* [add assignment type analysis for ide-completion](https://github.com/rust-lang/rust-analyzer/pull/20381)
* [add remove literal dbg stmt for `remove_dbg`](https://github.com/rust-lang/rust-analyzer/pull/20354)
* [add write! and writeln! to minicore](https://github.com/rust-lang/rust-analyzer/pull/20409)
* [fix `extract_expressions_from_format_string` on write!](https://github.com/rust-lang/rust-analyzer/pull/20418)
* [fix non-lsp compliant `Response` definition](https://github.com/rust-lang/rust-analyzer/pull/20393)
* [fix panic while trying to clear old diagnostics while there's nothing](https://github.com/rust-lang/rust-analyzer/pull/20434)
* [parser: fix parsing of trait bound polarity and for-binders](https://github.com/rust-lang/rust-analyzer/pull/20417)

### Rust Compiler Performance Triage

This week saw almost no regressions, while we got some nice wins. One of them was [#143684](https://github.com/rust-lang/rust/pull/143684), which updated the LLVM version used by the Rust compiler to 21.

Triage done by **@kobzol**.
Revision range: [07b7dc90..6355cd39](https://perf.rust-lang.org/?start=07b7dc90ee4df5815dbb91ef8e98cb93571230f5&end=6355cd39c81e9699b1925c58d2ed3165bcab1715&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.5%  | [0.2%, 0.8%]    | 3     |
| Regressions ❌ <br /> (secondary)  | 0.7%  | [0.1%, 1.5%]    | 8     |
| Improvements ✅ <br /> (primary)   | -1.5% | [-22.8%, -0.2%] | 219   |
| Improvements ✅ <br /> (secondary) | -2.9% | [-18.8%, -0.1%] | 256   |
| All ❌✅ (primary)                 | -1.5% | [-22.8%, 0.8%]  | 222   |

2 Regressions, 2 Improvements, 9 Mixed; 5 of them in rollups
37 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/bd5a0abbedd81c0dcc604f1b79f7f9e1f02e8139/triage/2025/2025-08-12.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Demote x86_64-apple-darwin from Tier 1 to Tier 2 with host tools](https://github.com/rust-lang/rfcs/pull/3841)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Fix overly restrictive lifetime in `core::panic::Location::file` return type](https://github.com/rust-lang/rust/pull/132087)
* [Tracking Issue for `const_array_each_ref`](https://github.com/rust-lang/rust/issues/133289)
* [Require approval from t-infra instead of t-release on tier bumps](https://github.com/rust-lang/rust/pull/144906)
* [const-eval: full support for pointer fragments](https://github.com/rust-lang/rust/pull/144081)
* [Don't warn on never to any `as` casts as unreachable](https://github.com/rust-lang/rust/pull/144804)
* [Port #[link] to the new attribute parsing infrastructure](https://github.com/rust-lang/rust/pull/143193)
* [`c_variadic` : Add future-incompatibility warning for `...` arguments without a pattern outside of extern blocks](https://github.com/rust-lang/rust/pull/143619)
* [Rewrite the new attribute argument parser](https://github.com/rust-lang/rust/pull/144689)
* [Tracking Issue for array::repeat](https://github.com/rust-lang/rust/issues/126695)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: Promote aarch64-pc-windows-msvc to Tier 1](https://github.com/rust-lang/rfcs/pull/3817)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).* or

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Pass pointers to `const` in assembly](https://github.com/rust-lang/rfcs/pull/3848)
* [new] [Include Clang in llvm-tools](https://github.com/rust-lang/rfcs/pull/3847)
* [new] [repr(ordered\_fields)](https://github.com/rust-lang/rfcs/pull/3845)

## Upcoming Events

Rusty Events between 2025-08-13 - 2025-09-10 🦀

### Virtual
* 2025-08-14 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**August, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/307698880)
* 2025-08-14 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820307)
* 2025-08-17 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002458)
* 2025-08-18 | Virtual (Kenya) | [RustaceansKenya](https://lu.ma/RustaceansKenya)
    * [**Rust Embedded Series: 02: Reading Datasheets**](https://lu.ma/6vvg0s9y)
* 2025-08-19 | Virtual (Santa Clara, CA, US) | [UCSC Extension Community](https://www.meetup.com/ucsc-extension-community/events/)
    * [**Programming with Rust**](https://www.meetup.com/ucsc-extension-community/events/310108013)
* 2025-08-19 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/306757756)
* 2025-08-20 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731032)
* 2025-08-21 | Hybrid (Mexico City, MX) | [Rust MX](https://www.meetup.com/rust-mx)
    * [**Polars para análisis y manipulación de datos**](https://www.meetup.com/rust-mx/events/310408223/)
* 2025-08-21 | Virtual (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Agosto de Protocol Buffers!**](https://www.meetup.com/rust-argentina/events/310019573)
* 2025-08-21 | Virtual (London, UK) | [Conf42: Online Tech Events](https://www.meetup.com/conf42/events/)
    * [**Conf42 Rustlang 2025**](https://www.meetup.com/conf42/events/305437705)
* 2025-08-21 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567875)
* 2025-08-24 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002461)
* 2025-08-26 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361442)
* 2025-08-28 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305878943)
* 2025-08-31 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002471)
* 2025-09-02 | Hybrid (Seattle, WA, US) | [RustConf](https://rustconf.com/)
    * [**RustConf 2025**](https://rustconf.com/)
* 2025-09-02 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304234)
* 2025-09-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyhcmbfb)
* 2025-09-06 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763848597)
* 2025-09-07 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002479)
* 2025-09-09 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361533)
* 2025-09-09 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**From First Lines to First Clients: Carol Nichols on Building a Career in Rust**](https://www.meetup.com/women-in-rust/events/310102318)

### Asia
* 2025-08-20 | Seoul, KR | [Seoul Rust](https://www.meetup.com/rust-seoul-meetup)
    * [**Seoul Rust Meetup**](https://www.meetup.com/rust-seoul-meetup/events/310347685)
* 2025-08-23 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**August 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/august-2025-rustacean-meetup/)

### Europe
* 2025-08-13 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/310014719)
* 2025-08-13 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944036)
* 2025-08-16 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Embedded - Workshop #4 @letsboot**](https://www.meetup.com/rust-basel/events/309894848)
* 2025-08-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night - Robot Edition**](https://www.meetup.com/rust-aarhus/events/310039453)
* 2025-08-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592249)
* 2025-08-21 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310062129)
* 2025-08-28 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester August Code Night**](https://www.meetup.com/rust-manchester/events/307919168)
* 2025-08-30 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #16**](https://www.meetup.com/stockholm-rust/events/310322522)
* 2025-09-03 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**From bugs to parallelism to future-proofing: What makes Rust different**](https://www.meetup.com/rust-rhein-main/events/310322369)
* 2025-09-10 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944038)

### North America
* 2025-08-14 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**August, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/307698880)
* 2025-08-14 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Programming a Fighting Robot in Rust with Rex Magana**](https://www.meetup.com/utah-rust/events/310053631)
* 2025-08-14 | Portland, OR, US | [PDXRust](https://www.meetup.com/pdxrust)
    * [**Cross-Magic: personal projects, Rust games, and utilizing AI**](https://www.meetup.com/pdxrust/events/310364279)
* 2025-08-18 | Denver, CO, US | [FOSS Rust Colorado](https://mobilizon.us/@foss_rust_colorado/events)
    * [**FOSS Rust Hack Night**](https://mobilizon.us/events/9092695a-89f0-40fa-b3d0-50072827b0ec)
* 2025-08-20 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731032)
* 2025-08-21 | Hybrid (Mexico City, MX) | [Rust MX](https://www.meetup.com/rust-mx)
    * [**Polars para análisis y manipulación de datos**](https://www.meetup.com/rust-mx/events/310408223/)
* 2025-08-21 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/310321250)
* 2025-08-21 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Rust on Bare Metal Series 2 : Place Holder**](https://www.meetup.com/music-city-rust-developers/events/304333117)
* 2025-08-23 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Somerville Union Square Rust Lunch, Aug 23**](https://www.meetup.com/bostonrust/events/310106302)
* 2025-08-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310205991)
* 2025-08-28 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**We're going again!**](https://www.meetup.com/rust-atl/events/308675976)
* 2025-09-02 - 2025-09-05 | Hybrid (Seattle, WA, US) | [RustConf](https://rustconf.com/)
    * [**RustConf 2025**](https://rustconf.com/)
* 2025-09-03 | Phoenix, AZ, US | [Desert Rust](https://www.meetup.com/desert-rustaceans)
    * [**RustConf 2025 Watch Party (Day 1)**](https://www.meetup.com/desert-rustaceans/events/310345446)
* 2025-09-04 | Phoenix, AZ, US | [Desert Rust](https://www.meetup.com/desert-rustaceans)
    * [**RustConf 2025 Watch Party (Day 2)**](https://www.meetup.com/desert-rustaceans/events/310345459)
* 2025-09-04 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**emulation of retro systems (NES, Gameboy) in Rust**](https://www.meetup.com/stl-rust/events/310116988)
* 2025-09-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Beacon Hill Rust Lunch, Sep 6**](https://www.meetup.com/bostonrust/events/310106310)

### Oceania
* 2025-08-26 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**August Meetup**](https://www.meetup.com/rust-canberra/events/308746519)
* 2025-08-27 - 2025-08-30 | Wellington, NZ | [Rust Forge](https://rustforgeconf.com/)
    * [**Rust Forge**](https://rustforgeconf.com/)

### South America
* 2025-08-21 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Agosto de Protocol Buffers!**](https://www.meetup.com/rust-argentina/events/310019573)

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

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> * solid performance and tools to optimize it further: because the easy thing is generally Fast Enough, it's quick to develop features even in a perf-sensitive project

– [Alice I Cecile on /r/rust](https://www.reddit.com/r/rust/comments/1mn9plk/bevys_fifth_birthday/n85mol9/)

Despite a lack of suggestions, llogiq is feeling pretty good about his choice.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
