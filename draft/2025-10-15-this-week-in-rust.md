Title: This Week in Rust 621
Number: 621
Date: 2025-10-15
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

* [Title of the linked Page](https://example.com/my_article)

If you add a link to a non-text content please prefix it with `[video]` or `[audio]`:

* [video] [Title of the linked video](https://example.com/my_video_article)
* [audio] [Title of the linked audio file](https://example.com/my_podcast)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official
* [Program management update — September 2025](https://blog.rust-lang.org/inside-rust/2025/10/14/program-management-update-2025-09/)

### Foundation
* [Rust Foundation Board Updates](https://rustfoundation.org/media/introducing-the-rust-foundations-newest-project-directors-october-2025/)

### Newsletters
* [This Month in Rust OSDev: September 2025](https://rust-osdev.com/this-month/2025-09/)

### Project/Tooling Updates
* [Gccrs after libcore](https://lwn.net/SubscriberLink/1040197/0733825193ca1f04/)
* [A new API for interrupt-aware spinlocks](https://lwn.net/SubscriberLink/1039374/664ea18bb8a3c1a8/)
* [Announcing Heave 0.1.0: an EAV data model rust library that can persist custom structs onto a SQLite DB with no friction at all!](https://www.rustydonkey.dev/blog/2025.10.08_introduction_to_heave/)
* [GuardianDB 0.10.15 - Introducing: the embedded iroh node](https://www.willsearch.com.br/?page_id=19)
* [Linebender in September 2025](https://linebender.org/blog/tmil-21/)
* [egui 0.33.0 - `egui::Plugin`, better kerning, kitdiff viewer](https://github.com/emilk/egui/releases/tag/0.33.0)
* [Making Slint Desktop-Ready](https://slint.dev/blog/making-slint-desktop-ready)
* [Avian Physics 0.4](https://joonaa.dev/blog/09/avian-0-4)
* [rustc_codegen_gcc: Progress Report #38](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-38)
* [CGP v0.5.0 Release - Auto dispatchers, extensible datatype improvements, monadic computation, RTN emulation, modular serde, and more](https://contextgeneric.dev/blog/v0-5-0-release/)
* [Diesel-Async 0.7](https://blog.weiznich.de/blog/diesel-async-0-7/)

### Observations/Thoughts
* [We need (at least) ergonomic, explicit handles](https://smallcultfollowing.com/babysteps/blog/2025/10/13/ergonomic-explicit-handles/)
* [To panic or not to panic](https://www.ncameron.org/blog/to-panic-or-not-to-panic/)
* [Recursive type state in Rust](https://www.jessestuart.ca/posts/2025-10-10-recursive-type-state-in-rust/)
* [Talk about memory safety at ONE Conference](https://tweedegolf.nl/en/blog/195/talk-about-memory-safety-at-one-conference)
* [A Little Rust Trait Limitation](https://www.thecodedmessage.com/posts/rust-trait-limitation/)
* [Effects in Rust (and Koka)](https://aloso.foo/blog/2025-10-10-effects/)
* [video] [Oxidize Conference 2025](https://www.youtube.com/playlist?list=PLilpJp3WAOvcn5_VDv3VIkQzniMWl_BfO)
* [video] [Rust 2025: 400K Salaries, AI, Defence & Borrow Checker — Jon Gjengset on Rust & the Future of Coding](https://www.youtube.com/watch?v=nOSxuaDgl3s)
* [audio] [Netstack.FM Episode 9 – gRPC with Lucio Franco](https://netstack.fm/#episode-9)

### Rust Walkthroughs
* [Building SQLite extensions in Rust](https://kerkour.com/sqlite-extension-rust)
* [Axum Backend Series: JWT with Refresh Token](https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-jwt-refresh-token/)
* [series] [The Impatient Programmer's Guide to Bevy and Rust: Chapter 1 - Let There Be a Player](https://aibodh.com/posts/bevy-rust-game-development-chapter-1/)
* [series] [The Impatient Programmer's Guide to Bevy and Rust: Chapter 2 - Let There Be a World](https://aibodh.com/posts/bevy-rust-game-development-chapter-2/)
* [video] [Building Embedded TUIs with Rust & Ratatui — Tokyo Rust Meetup 2025](https://www.youtube.com/watch?v=F04kQMKwrwQ)

* [video] [Build with Naz : Eliminate off by one errors with Rust type system design](https://www.youtube.com/watch?v=IkCUhGAyS9U)

### Research

* [Papers on C-to-Rust Translation](https://hjaem.info/c-to-rust-papers)

### Miscellaneous
* [Rust Maintainers Fund - RustNL](https://rustnl.org/fund/)
* [🦀 Pack of 50+ Free (CC0 license) Ferris illustrations with different emotions, poses and situations in PNG and SVG 🦀](https://github.com/MariaLetta/free-ferris-pack)

## Crate of the Week

This week's crate is [mitsein](https://github.com/olson-sean-k/mitsein), a library of non-empty collections.

Thanks to [Nik Revenco](https://users.rust-lang.org/t/crate-of-the-week/2704/1481) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

* *No calls for testing were issued this week by
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

* [Diesel - View Support - Show me your view definitions](https://github.com/diesel-rs/diesel/discussions/4805)
* [Diesel - Add `#[diagnostic::do_not_recommend]` to `impl AsExpression for T: Expression`](https://github.com/diesel-rs/diesel/issues/4760)
* [Diesel - Improve documentation for Postgres loading modes](https://github.com/diesel-rs/diesel/issues/4764)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [**TokioConf 2026**](https://tokio.rs/blog/2025-09-26-announcing-tokio-conf-cfp)| CFP closes 2025-12-08 | Portland, Oregon, USA | 2026-04-20

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

420 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-10-07..2025-10-14

#### Compiler
* [add a new `wasm32-wasip3` target to Rust](https://github.com/rust-lang/rust/pull/147205)
* [Global Variable Naming: evaluate constants lazily](https://github.com/rust-lang/rust/pull/146869)
* [`DepNodeColor` tweaks](https://github.com/rust-lang/rust/pull/147423)
* [perform InstSimplify before ReferencePropagation](https://github.com/rust-lang/rust/pull/147483)
* [refactor AddCallGuards in two loops](https://github.com/rust-lang/rust/pull/147477)
* [split `overlapping_{inherent,trait}_impls`](https://github.com/rust-lang/rust/pull/147502)
* [validate `CopyForDeref` and `DerefTemps` better and remove them from runtime MIR](https://github.com/rust-lang/rust/pull/145513)

#### Library
* [move more code to `RawVec::finish_grow`](https://github.com/rust-lang/rust/pull/147124)
* [port the implemention of SIMD intrinsics from Miri to const-eval](https://github.com/rust-lang/rust/pull/146568)
* [specialize `slice::fill` to use memset when possible](https://github.com/rust-lang/rust/pull/147457)
* [stabilize `NonZero<u*>::div_ceil`](https://github.com/rust-lang/rust/pull/147562)

#### Cargo
* [Reorganize build-dir layout](https://github.com/rust-lang/cargo/pull/15947)
* [add: Report a missing source error for workspace dependencies](https://github.com/rust-lang/cargo/pull/16063)
* [script: Default bin.name to package.name](https://github.com/rust-lang/cargo/pull/16064)
* [script: Store cargo script lockfiles in build-dir](https://github.com/rust-lang/cargo/pull/16087)
* [tree: Switch from `--depth public` to `--edges public`](https://github.com/rust-lang/cargo/pull/16081)
* [allow to rustfix `unused_variables` lint](https://github.com/rust-lang/cargo/pull/16082)
* [fix regression that swallowed json diagnostic explanations](https://github.com/rust-lang/cargo/pull/16075)

#### Rustdoc
* [rustdoc: Don't serialize & deserialize data that doesn't go over the wire](https://github.com/rust-lang/rust/pull/147402)
* [rustdoc: a small performance improvement: only allocate new string if there are DOS backlines in highlight.rs](https://github.com/rust-lang/rust/pull/147443)

#### Clippy
* [`multiple_inherent_impl`: Add config option to target specific scope](https://github.com/rust-lang/rust-clippy/pull/15843)
* [`zero_repeat_side_effects`: don't suggest unnecessary braces around stmts](https://github.com/rust-lang/rust-clippy/pull/15826)
* [`clone_on_ref_ptr`: only name the generic type if possible](https://github.com/rust-lang/rust-clippy/pull/15740)
* [`collapsible_match`: exclude binding modes from `struct` field pattern suggestions](https://github.com/rust-lang/rust-clippy/pull/15608)
* [`zero_repeat_side_effects`: don't suggest unsuggestable types](https://github.com/rust-lang/rust-clippy/pull/15815)
* [`legacy_numeric_constants`: add ctxt check for internal macro](https://github.com/rust-lang/rust-clippy/pull/15816)
* [`manual_unwrap_or`: fix false positive edge case](https://github.com/rust-lang/rust-clippy/pull/15812)
* [`get_unwrap`: avoid calling `is_type_diagnostic_item` multiple times](https://github.com/rust-lang/rust-clippy/pull/15847)
* [add `replace_box` lint](https://github.com/rust-lang/rust-clippy/pull/14953)
* [add lint `unnecessary_option_map_or_else`](https://github.com/rust-lang/rust-clippy/pull/14662)
* [check structs and enums for `use_self`](https://github.com/rust-lang/rust-clippy/pull/15566)
* [fix `needless_continue` false positive when match type is not unit or never](https://github.com/rust-lang/rust-clippy/pull/15547)
* [honor `allow`/`expect` attributes on ADT and `impl Clone` nodes](https://github.com/rust-lang/rust-clippy/pull/15849)

#### Rust-Analyzer
* [add ide-assist: generate blanket trait impl](https://github.com/rust-lang/rust-analyzer/pull/19771)
* [add self param completions for trait assoc fn](https://github.com/rust-lang/rust-analyzer/pull/20812)
* [build rust-analyzer with `--target` for install/pgo xtask](https://github.com/rust-lang/rust-analyzer/pull/20804)
* [fix .let completion not working for let-chain](https://github.com/rust-lang/rust-analyzer/pull/20526)
* [fix closure coerced return type for `add_return_type`](https://github.com/rust-lang/rust-analyzer/pull/20816)
* [fix empty closure completion analysis](https://github.com/rust-lang/rust-analyzer/pull/20824)
* [fix not applicable c-str and byte-str for `raw_string`](https://github.com/rust-lang/rust-analyzer/pull/20788)
* [fix not applicable on param in let-stmt for `add_explicit_type`](https://github.com/rust-lang/rust-analyzer/pull/20817)
* [improve parsing error for `static` and `const`](https://github.com/rust-lang/rust-analyzer/pull/20805)
* [replace `--show-output` task defaults with `--nocapture`](https://github.com/rust-lang/rust-analyzer/pull/20803)

### Rust Compiler Performance Triage

This week saw small wins across the board from some microoptimizations of the incremental query
system ([#147423](https://github.com/rust-lang/rust/pull/147423)). There have also been a couple of
regressions. [#142390](https://github.com/rust-lang/rust/pull/142390) introduced regressions of `check`
builds across the board. The largest regression (18%) is from an incremental opt build of a secondary
artificial stress test, so we deemed it acceptable.

Triage done by **@kobzol**.

Revision range: [1a3cdd34..956f47c3](https://perf.rust-lang.org/?start=1a3cdd34629306fa67624eaa60d73687e7fcf855&end=956f47c32f1bd97b22cd702d7ccf78f0f0d42c34&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.7%  | [0.1%, 2.0%]   | 65    |
| Regressions ❌ <br /> (secondary)  | 0.8%  | [0.1%, 18.6%]  | 65    |
| Improvements ✅ <br /> (primary)   | -0.6% | [-1.6%, -0.1%] | 119   |
| Improvements ✅ <br /> (secondary) | -0.4% | [-1.6%, -0.1%] | 76    |
| All ❌✅ (primary)                 | -0.1% | [-1.6%, 2.0%]  | 184   |

2 Regressions, 7 Improvements, 3 Mixed; 3 of them in rollups
35 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/b4b30e719c7083141669f79edfdf20e685cf918f/triage/2025/2025-10-13.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [iter repeat: panic on last](https://github.com/rust-lang/rust/pull/147258)
* [Tracking Issue (take 2) for `more_float_constants`](https://github.com/rust-lang/rust/issues/146939)
* [Temporary lifetime extension for blocks](https://github.com/rust-lang/rust/pull/146098)
* [Document MaybeUninit bit validity](https://github.com/rust-lang/rust/pull/140463)
* [unused_must_use: Don't warn on `Result<(), Uninhabited>` or `ControlFlow<Uninhabited, ()>`](https://github.com/rust-lang/rust/pull/147382)
* [Allow passing `expr` metavariable to `cfg`](https://github.com/rust-lang/rust/pull/146961)
* [Remove current code for embedding command-line args in PDB](https://github.com/rust-lang/rust/pull/147022)
* [`-Znext-solver` instantiate predicate binder without recanonicalizing goal](https://github.com/rust-lang/rust/pull/146725)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [Should build-plans be deleted?](https://github.com/rust-lang/cargo/issues/7614)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Pass pointers to `const` in assembly](https://github.com/rust-lang/rfcs/pull/3848)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period)

* [Delegate GSoC money spending to the t-mentorship team](https://github.com/rust-lang/leadership-council/issues/232)

*No Items entered Final Comment Period this week for
  [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) or
  [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2025-10-15 - 2025-11-12 🦀

### Virtual
* 2025-10-15 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**indielinks**](https://www.meetup.com/vancouver-rust/events/307731034/)
* 2025-10-16 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/o8fh3fh7)
* 2025-10-16 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/306046668/)
* 2025-10-18 | Virtual (Gdansk/Kraków/Wroclaw, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto/)
    * [**[BEZPŁATNIE] Programowanie w języku Rust**](https://www.meetup.com/stacja-it-trojmiasto/events/310935164/) | [Kraków Mirror](https://www.meetup.com/stacja-it-krakow/events/310935157/) | [Wroclaw Mirror](https://www.meetup.com/stacja-it-wroclaw/events/310935159/)
* 2025-10-19 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109167/)
* 2025-10-21 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/311068625/)
* 2025-10-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/310002307/)
* 2025-10-22 | Virtual (Boulder, CO, US) | [Boulder Elixir](https://www.meetup.com/boulder-elixir/events/)
    * [**Integrating Elixir and Apache DataFusion with Rustler**](https://www.meetup.com/boulder-elixir/events/310996627/)
* 2025-10-22 | Virtual (Buenos Aires, AR) | [[Net-Baires] Comunidad de .NET en Buenos Aires](https://www.meetup.com/es-ES/net-baires/)
    * [**Rust para devs .NET | Community Standup #10**](https://www.meetup.com/es-ES/net-baires/events/311365783/)
* 2025-10-23 | Hybrid (Seattle/Bellevue, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**October, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351020/)
* 2025-10-23 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046641/)
* 2025-10-23 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/zyc3touy)
* 2025-10-26 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109171/)
* 2025-10-28 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361444/)
* 2025-10-30 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/t8yovmmm)
* 2025-11-01 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763868657)
* 2025-11-02 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109173/)
* 2025-11-05 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 2025-11-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyhcpbhb/)
* 2025-11-06 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305646021/)
* 2025-11-06 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/xkd84gfz)
* 2025-11-09 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109175/)
* 2025-11-11 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361536/)
* 2025-11-11 | Virtual (London, GB) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/311068632/)

### Asia
* 2025-10-20 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust October 2025 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/310628902/)

### Europe
* 2025-10-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/311035141/)
* 2025-10-21 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Meetup #01 @ Zrch**](https://www.meetup.com/bergen-rust-new-technology/events/311153821/)
* 2025-10-21 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592252/)
* 2025-10-21 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group)
    * [**Rust in Surgery: Powering the Data Pipelines**](https://www.meetup.com/london-rust-project-group/events/310813952/)
* 2025-10-23 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (evening pub)**](https://www.meetup.com/rust-and-friends/events/311501254/)
* 2025-10-24 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/311501249/)
* 2025-10-28 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester October Code Night**](https://www.meetup.com/rust-manchester/events/307919171/)
* 2025-10-29 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund/events/)
    * [**Rust Dortmund Meetup October 2025**](https://www.meetup.com/rust-dortmund/events/311251545/)
* 2025-10-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #62 sponsored by Google!**](https://www.meetup.com/copenhagen-rust-community/events/311405044/)
* 2025-10-30 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Prague (October 2025)**](https://www.meetup.com/rust-prague/events/310967094/)
* 2025-11-02 - 2025-11-04 | Florence, IT | [Rustlab 2025](https://rustlab.it/)
    * [**Rustlab 2025**](https://rustlab.it/)
* 2025-11-04 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester November Talk**](https://www.meetup.com/rust-manchester/events/310921632/)
* 2025-11-05 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Rust Girona Hack & Learn 11 2025**](https://luma.com/xl8ob0tn)
* 2025-11-05 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310601872/)
* 2025-11-05 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Rust/ACCU meetup.**](https://www.meetup.com/oxford-rust-meetup-group/events/nnrkttyhcpbhb/)
* 2025-11-06 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #11**](https://www.meetup.com/rust-gdansk/events/310924266/)
* 2025-11-12 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944050/)

### North America
* 2025-10-15 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**indielinks**](https://www.meetup.com/vancouver-rust/events/307731034/)
* 2025-10-16 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/311012947/)
* 2025-10-16 | San Francisco, CA, US | [Svix](https://luma.com/calendar/cal-Cnmn4RR2n4fRUNZ)
    * [**San Francisco Rust Meetup**](https://luma.com/tp6w7tc9)
* 2025-10-21 | San Francisco, CA, US | [Vara & Gear](https://luma.com/events-by-vara-gear)
    * [**Rust Workshop by Vara Network**](https://luma.com/kbs2os1c)
* 2025-10-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/308284343/)
* 2025-10-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310457307/)
* 2025-10-23 | Hybrid (Seattle/Bellevue, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**October, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351020/)
* 2025-10-23 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Year In Review**](https://www.meetup.com/music-city-rust-developers/events/304333267/)
* 2025-10-23 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust)
    * [**October Rust Meetup: A Special Presentation and Monthly Meetups are Back!**](https://www.meetup.com/spokane-rust/events/311346444/)
* 2025-10-25 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Porter Square Rust Lunch, Oct 25**](https://www.meetup.com/bostonrust/events/310983712/)
* 2025-10-29 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Scalable static analysis: confronting the halting problem**](https://www.meetup.com/rust-nyc/events/311541108/)
* 2025-10-30 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/308675988/)
* 2025-10-30 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/311273832/)
* 2025-11-01 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Chinatown Rust Lunch, Nov 1**](https://www.meetup.com/bostonrust/events/311039492/)
* 2025-11-06 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**SIUE students on wasm 3D animations**](https://www.meetup.com/stl-rust/events/307251982/)
* 2025-11-08 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Winter Hill Rust Lunch, Nov 8**](https://www.meetup.com/bostonrust/events/311039501/)

### Oceania
* 2025-10-22 | Perth, AU | [Rust Perth Meetup Group](https://www.meetup.com/perth-rust-meetup-group)
    * [**October Meetup**](https://www.meetup.com/perth-rust-meetup-group/events/310847099/)
* 2025-10-28 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**October Meetup**](https://www.meetup.com/rust-canberra/events/311234237/)

### South America
* 2025-10-22 | Montevideo, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay/events/)
    * [**Rust Uruguay meetup de Octubre**](https://www.meetup.com/rust-uruguay/events/311475675/)
* 2025-10-25 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP na Amazon Web Services**](https://www.meetup.com/rust-sao-paulo-meetup/events/311084440/)
* 2025-10-30 | Florianopolis, BR | [Rust Brasil](https://luma.com/calendar/cal-iOloL5ZqswCO5Mm)
    * [**Rust Floripa**](https://luma.com/lky7an18)

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

> Pointers are quite hard.

– Tim McNamara

> And, as the name implies, pointy.

– [llogiq on LinkedIn](https://www.linkedin.com/feed/update/urn:li:activity:7381109081857724416?commentUrn=urn%3Ali%3Acomment%3A%28activity%3A7381109081857724416%2C7381113605926166528%29&dashCommentUrn=urn%3Ali%3Afsd_comment%3A%287381113605926166528%2Curn%3Ali%3Aactivity%3A7381109081857724416%29)

Thanks to [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1721) for the self-suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
