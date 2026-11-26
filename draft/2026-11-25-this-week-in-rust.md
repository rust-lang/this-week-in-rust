Title: This Week in Rust 679
Number: 679
Date: 2026-11-25
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/main/CONTRIBUTING.md).

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

* [Announcing Rust 1.100.0](https://blog.rust-lang.org/2026/11/12/Rust-1.100.0/)
* [Rust All Hands 2027](https://blog.rust-lang.org/inside-rust/2026/11/23/all-hands-2027/)

### Foundation

* [Needles, Haystacks, and Numbers Between 0 and 1](https://rustfoundation.org/media/needles-haystacks-and-numbers-between-0-and-1/)

### Newsletters

* [Scientific Computing in Rust #24 (November 2026)](https://scientificcomputing.rs/monthly/2026-11)
* [The Embedded Rustacean Issue #85](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-85)

### Project/Tooling Updates

<!-- IMPORTANT NOTE: We are no longer accepting pull request submissions for the Project/Tooling Updates section.
See here for details: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

* [Release: Roto v0.14.0!](https://community.nlnetlabs.nl/t/release-roto-v0-14-0/3511)
* [October in Servo: video captions and inert content](https://servo.org/blog/2026/11/25/october-in-servo/)
* [A new name for ntpd-rs: Statime](https://trifectatech.org/blog/new-name-for-ntpd-rs-statime/)

### Observations/Thoughts

* [Optimizing Rustc's next trait solver](https://donsz.nl/blog/optimizing-rustcs-next-trait-solver)
* [Advice for the next 100 Rust GUIs](https://linebender.org/blog/next-100-guis/)
* [Abolish Lock Poisoning](https://lucumr.pocoo.org/2026/11/25/poisoning/)
* [Compile time improvements for derive(Debug)](https://yoyo-code.com/derive-debug-compile-time-improvements/)
* [The semver trick is not backwards compatible if you wait](https://sgued.fr/blog/semver-trick-wait/)
* [How to Stop Generic Type Explosion in Rust] (https://edvong.com/stop-generic-type-explosion/)
* [How Proton (Mail / VPN...) are rewriting their cross-platform apps in Rust to reduces costs and improve security for millions of people](https://kerkour.com/rust-at-proton)
* [video] [Rust Release Changelog - 1.100](https://www.youtube.com/watch?v=fP2FiYfyzlk)
* [video] [From Hardware to Human Interface: Building HID Devices in Rust - Frank Lyaruu](https://www.youtube.com/watch?v=Hhtdf0bQBks)
* [video] [Tales from the Embedded Rust Frontier - Jonathan Pallant](https://www.youtube.com/watch?v=0M72XNQzkRU)
* [audio] [Axum with David Pedersen](https://netstack.fm/episodes/axum-with-david-pedersen)
* [audio] [Embedded Software with Rust with David Cabanis](https://rustacean-station.org/episode/david-cabanis/)

### Rust Walkthroughs

* [The State of Allocators in 2026 - Finally Stabilised!](https://cetra3.github.io/blog/state-of-allocators-2026-finally-stabilised/)
* [Rust Projects - Write an Interpreter - Version 1.0.1](https://rust-projects-write-an-interpreter.github.io/)
* [RSPHP Part 2: Build Tools](https://scherzer.dev/Blog/20261120-rsphp-build-tools)
* [Rolling upgrades in tellus: old code cannot learn a new shape](https://heikoseeberger.de/2026-11-24-rolling-upgrades/)
* [Build your own Paint](https://www.buildyourownpaint.com/en)
* [video] [Nine Rules for Compile-Time Work with const fn by Carl Kadie](https://www.youtube.com/watch?v=KznFsjlPCbo)
* [video] [series] [Repository Pattern in Rust: async fn in traits, dyn vs generics, and the Send problem (Axum + SQLx)](https://www.youtube.com/watch?v=zbglU7Ppv10)
* [video] [Code Safari: uutils cp](https://www.youtube.com/watch?v=Yn4i4tRhOdU)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [i_overlay](https://crates.io/crates/i_overlay), providing a set of geometrical operations on polygons and line segment chains running completely on integer math.

Thanks to [Mikolaj Wielgus](https://users.rust-lang.org/t/crate-of-the-week/2704/1725) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

*No calls for testing were issued this week by
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Rustup](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) or
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

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
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

724 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-11-17..2026-11-24

#### Compiler
* [add lint against duplicate parameter names in fns without body](https://github.com/rust-lang/rust/pull/165366)
* [add wide pointer visualizer for LLDB](https://github.com/rust-lang/rust/pull/164975)
* [avoid instantiating `FnSig` when not necessary](https://github.com/rust-lang/rust/pull/165863)
* [avoid unnecessary LLVM passes when thin LTO buffer is null](https://github.com/rust-lang/rust/pull/166084)
* [compressed `ParamEnv`s for common marker traits](https://github.com/rust-lang/rust/pull/165460)
* [`hir_typeck`: propagate async closure output into its body](https://github.com/rust-lang/rust/pull/161708)
* [inline some attribute parsing code](https://github.com/rust-lang/rust/pull/166385)
* [perf: remove cache in `DeepResolverIgnoringRegions`](https://github.com/rust-lang/rust/pull/166384)
* [perf: separate flag computation & intern functions per `TyKind`](https://github.com/rust-lang/rust/pull/165396)
* [share more debuginfo for inlined functions](https://github.com/rust-lang/rust/pull/166140)

#### Library
* [core: stabilize `float_bits_const`](https://github.com/rust-lang/rust/pull/166421)
* [fmt: improve `Debug` for `FormattingOptions`](https://github.com/rust-lang/rust/pull/166259)
* [implement RangeIntersect on `core::range` types](https://github.com/rust-lang/rust/pull/165618)
* [stabilize `char::is_case_ignorable`, `char::is_cased`, and the `titlecase` feature](https://github.com/rust-lang/rust/pull/165434)
* [stabilize unchecked `nonzero_ops`](https://github.com/rust-lang/rust/pull/164485)
* [unstably constify `std::process::abort`](https://github.com/rust-lang/rust/pull/166194)

#### Cargo
* [config: Add IncludeConfig](https://github.com/rust-lang/cargo/pull/17782)
* [config: `--config` use same layers merge as config files](https://github.com/rust-lang/cargo/pull/17780)
* [config: Handle parsing dotted keys using TOML keys](https://github.com/rust-lang/cargo/pull/17770)
* [config: Improve config schema by implementing Serialize](https://github.com/rust-lang/cargo/pull/17762)
* [config: drop redundant config merge error context](https://github.com/rust-lang/cargo/pull/17769)
* [config: keep winning definition for non-mergeable list](https://github.com/rust-lang/cargo/pull/17766)
* [config: layer loads to work the same as merged load](https://github.com/rust-lang/cargo/pull/17761)
* [config: list unmerged config in precedence order](https://github.com/rust-lang/cargo/pull/17773)
* [context: merge files and `--config` as layers](https://github.com/rust-lang/cargo/pull/17775)
* [fix git checkout long paths](https://github.com/rust-lang/cargo/pull/17752)
* [config: really ensure no comments are present](https://github.com/rust-lang/cargo/pull/17774)
* [support wasm64-unknown-emscripten](https://github.com/rust-lang/cargo/pull/17620)

#### Rustdoc
* [add emoji icons for unstable and deprecated to search results](https://github.com/rust-lang/rust/pull/153746)
* [clean inlined assoc items in their own param-env](https://github.com/rust-lang/rust/pull/165699)
* [fix search index generation: if crate content didn't change, it should generate the same search index](https://github.com/rust-lang/rust/pull/166420)

#### Rustfmt
* [don't duplicate type bounds when a comment precedes the where clause](https://github.com/rust-lang/rustfmt/pull/6980)
* [have `cfg_select!` double-brace concise formatting respect `match_block_trailing_comma`](https://github.com/rust-lang/rustfmt/pull/7213)

#### Clippy
* [add a recursion limit to avoid hang with recursive associated types](https://github.com/rust-lang/rust-clippy/pull/18086)
* [fix `single_match` suggests wrongly for bool comparisons](https://github.com/rust-lang/rust-clippy/pull/16880)
* [fix `unused_async_trait_impl` suggestion for implicit unit return type](https://github.com/rust-lang/rust-clippy/pull/18101)
* [`unusual_byte_groupings`: Don't lint binary literals](https://github.com/rust-lang/rust-clippy/pull/18104)

#### Rust-Analyzer
* [add `r? @ghost` Josh push PR suffix](https://github.com/rust-lang/rust-analyzer/pull/23681)
* [show custom iter adapter in chaining inlayHints](https://github.com/rust-lang/rust-analyzer/pull/23549)
* [fix `extract_type_alias` for const `_` placeholders](https://github.com/rust-lang/rust-analyzer/pull/22211)
* [accidentally quadratic performance in type inference](https://github.com/rust-lang/rust-analyzer/pull/23678)
* [allow `notable_trait` attr args](https://github.com/rust-lang/rust-analyzer/pull/23685)
* [exclude proc macro crates from lang item computations for their dependencies](https://github.com/rust-lang/rust-analyzer/pull/23689)
* [make sure copied lock files are writable](https://github.com/rust-lang/rust-analyzer/pull/23688)
* [respect `is_workspace_member` for all crates in rust-project.json](https://github.com/rust-lang/rust-analyzer/pull/23694)
* [introduce a hack for `PartialEq` lang item attr change in core lib](https://github.com/rust-lang/rust-analyzer/pull/23691)
* [ops: resolve `artipacked` and `template-injection`](https://github.com/rust-lang/rust-analyzer/pull/23594)
* [perf: cache the mark when expanding MBE](https://github.com/rust-lang/rust-analyzer/pull/23709)

### Rust Compiler Performance Triage


Very positive week with many improvements in the type system, codegen, debuginfo and encoding. The biggest improvement came
perhaps surprisingly from Cargo submodule update, because Cargo recently changed
the default debuginfo level for `dev` profile to `line-tables-only`.

Triage done by **@panstromek**.
Revision range: [6ca2b346..5debe6a1](https://perf.rust-lang.org/?start=6ca2b34602bf2f3591ffa7359c166b6409a44b0a&end=5debe6a1244452beced762385f88a51857a915fc&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 1.3%  | [0.2%, 11.1%]   | 65    |
| Regressions ❌ <br /> (secondary)  | 4.1%  | [0.2%, 12.1%]   | 79    |
| Improvements ✅ <br /> (primary)   | -5.2% | [-38.3%, -0.2%] | 136   |
| Improvements ✅ <br /> (secondary) | -3.4% | [-24.3%, -0.1%] | 172   |
| All ❌✅ (primary)                 | -3.1% | [-38.3%, 11.1%] | 201   |


7 Regressions, 8 Improvements, 19 Mixed; 1 of them in rollups
235 artifact comparisons made in total

[Full report here](https://github.com/panstromek/rustc-perf/blob/e92593db8a8ea7c091e1b6e673349297dd4d6f23/triage/2026/2026-11-23.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/main)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [emit fake reads for indexing after every bounds-check](https://github.com/rust-lang/rust/pull/161857)
* [Don't emit diagnostics during pre-expansion lint attribute parsing](https://github.com/rust-lang/rust/pull/164898)
* [Types FCP v2: Supertrait item shadowing stabilization](https://github.com/rust-lang/rust/issues/162130)
* [don't ignore ambiguity during MIR type check](https://github.com/rust-lang/rust/pull/164055)
* [debugger: stabilize trim-paths unremap file loading](https://github.com/rust-lang/rust/pull/163910)
* [Add #[rustc_edition_redirect]](https://github.com/rust-lang/rust/pull/160227)
* [Tracking Issue for NEON FCMA intrinsics](https://github.com/rust-lang/rust/issues/117222)
* [Remove `T: Send + Sync` bound from `Arc::downcast`](https://github.com/rust-lang/rust/pull/161531)
* [Stabilization of the `documentation` scope in `rustc`](https://github.com/rust-lang/rust/issues/163896)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [RFC: Registry withholding](https://github.com/rust-lang/rfcs/pull/4016)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [fix!: stop uplifting rlibs ](https://github.com/rust-lang/cargo/pull/17715)
* [Tracking Issue for `-Zmem-stats`](https://github.com/rust-lang/cargo/issues/17577)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Store C-compatible types in the target specification](https://github.com/rust-lang/compiler-team/issues/1050)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Allocate budget for paying for zulip features we need](https://github.com/rust-lang/leadership-council/issues/343)

*No Items entered Final Comment Period this week for
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Security team charter](https://github.com/rust-lang/rfcs/pull/4032)
* [ci: Resolve most `zizmor` findings as of `zizmor@1.32.0`](https://github.com/rust-lang/rfcs/pull/4031)

## Upcoming Events

Rusty Events between 2026-11-25 - 2026-12-23 🦀

### Virtual
* 2026-11-26 | Virtual | [betterCode() Rust](https://rust.bettercode.eu/)
    * [**betterCode() Rust 2026**](https://rust.bettercode.eu/)
* 2026-11-28 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Getting Started with Game Development in Rust and Bevy**](https://www.meetup.com/charlottesville-rust-meetup/events/317399210/)
* 2026-12-01 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Cursed Rust**](https://luma.com/omlooqv8)
* 2026-12-01 | Virtual | [ERNI](https://www.eventbrite.es/o/erni-30130841744)
    * [**Memory-safe by design: How Rust strengthens security in medtech**](https://www.eventbrite.es/e/memory-safe-by-design-how-rust-strengthens-security-in-medtech-tickets-2003651953931)
* 2026-12-01 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Christmas Coding Challenge - Kick Off!**](https://www.meetup.com/women-in-rust/events/316993134/)
* 2026-12-02 | Virtual (Cardiff, GB) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Operating Systems Book Club: Swapping Policies & Complete VM Systems**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/317485117/)
* 2026-12-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/317404144/)
* 2026-12-03 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/316907782/)
* 2026-12-04 | Hybrid (Moscow, RU) | [RustCon RU](https://rustcon.ru/)
    * [**RustCon 2026**](https://rustcon.ru/)
* 2026-12-05 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Entities, Components, and Systems: How Bevy Games Work**](https://www.meetup.com/charlottesville-rust-meetup/events/317399385/)
* 2026-12-06 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/316331278/)
* 2026-12-08 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254767/)
* 2026-12-08 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/316993086/)
* 2026-12-12 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Building a Game with Rust and Bevy: Putting It All Together**](https://www.meetup.com/charlottesville-rust-meetup/events/317399435/)
* 2026-12-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/316916636/)
* 2026-12-16 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/315210235/)
* 2026-12-17 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/317328457/)
* 2026-12-20 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/316563036/)
* 2026-12-22 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Fourth Tuesday Rust Bookclub**](https://www.meetup.com/dallasrust/events/310254766/)

### Asia
* 2026-12-12 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**December 2026 Rustacean Meetup**](https://hasgeek.com/rustbangalore/december-2026-rustacean-meetup/)
* 2026-12-12 | Shah Alam, MY | [Rust Malaysia Meetup](https://discord.gg/Uz88bnZA3B)
    * [**Year End Hack (Rust Meetup December 2026)**](https://forms.gle/2WPUZC3RVLXh21zP9)

### Europe
* 2026-11-25 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group)
    * [**Rust Meetup @ Reclamefolder**](https://www.meetup.com/rust-amsterdam-group/events/317381204/)
* 2026-11-25 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**On-site Rust Ratatui Workshop in Dortmund**](https://www.meetup.com/rust-dortmund/events/317393392/)
* 2026-11-26 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/317497055/)
* 2026-11-26 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin on location 🏳️‍🌈 – Edition 020**](https://www.meetup.com/rust-berlin/events/317543387/)
* 2026-11-26 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #73 sponsored by Adapt!**](https://www.meetup.com/copenhagen-rust-community/events/317181792/)
* 2026-11-26 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group)
    * [**Rust London x Zed**](https://www.meetup.com/rust-london-user-group/events/317253958/)
* 2026-11-26 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester November Code Night**](https://www.meetup.com/rust-manchester/events/317222338/)
* 2026-11-26 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Rust Meetup @Lovable**](https://www.meetup.com/stockholm-rust/events/317161063/)
* 2026-11-28 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #32**](https://www.meetup.com/stockholm-rust/events/317458938/)
* 2026-11-30 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #22**: Mert Yildiz: Sleepless in Tokio - Channel your inner actor with Rust](https://rust-augsburg.github.io/meetup/Meetup_22.html)
* 2026-12-01 | Milano, MI, IT | [Rust Language Milan](https://www.meetup.com/rust-language-milano/events/)
    * [**Web Applications in Rust with Dioxus & Axum**](https://www.meetup.com/rust-language-milan/events/317485676/)
* 2026-12-01 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #90**](https://www.meetup.com/rust-paris/events/316865676/)
* 2026-12-02 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**From Curiosity to Capability: Build Your Own Device Swarm**](https://www.meetup.com/rust-rhein-main/events/317186779/)
* 2026-12-02 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust in December: Streaming Glühwein**](https://www.meetup.com/rustcologne/events/317564826/)
* 2026-12-04 | Hybrid (Moscow, RU) | [RustCon RU](https://rustcon.ru/)
    * [**RustCon 2026**](https://rustcon.ru/)
* 2026-12-08 | Dresden, DE | [Rust Dresden](https://github.com/rust-dresden)
    * [**Meetup 4**](https://pretix.eu/rust-dresden/on-location-4/)
* 2026-12-08 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2026 / 4 - Hacking Evening**](https://www.meetup.com/rust-munich/events/317565563/)
* 2026-12-10 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust Hack Night #13: Advent of Code, sponsored by PROSA**](https://www.meetup.com/copenhagen-rust-community/events/317468202/)
* 2025-11-13 | Geneva, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-12-22 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816504/)

### North America
* 2026-11-28 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**South Station Rust Lunch, Nov 28**](https://www.meetup.com/bostonrust/events/316924412/)
* 2026-12-03 | Montréal, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
    * [**December Monthly Social**](https://www.meetup.com/rust-montreal/events/317415310/)
* 2026-12-04 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: The UnConf 2026.**](https://www.meetup.com/rust-nyc/events/317259308/)
* 2026-12-04 | New York, NY, US | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋Women in Rust x Rust NYC UnConf 2026**](https://www.meetup.com/women-in-rust/events/317383331/)
* 2026-12-15 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/315783990/)
* 2026-12-16 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/315210235/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> Good news bad news situation.
>
> Good news: I have 3 blog posts' worth of work in cargo-semver-checks to publish.
> Bad news: It means Rust is *way more cursed* than we all ever knew.'

– [Predrag Gruevski on Mastodon](https://hachyderm.io/@predrag/117672539151294123)

Despite a lamentable lack of suggestions, llogiq loves cursed Rust is pretty pleased with his choice.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

This Week in Rust is edited by:

* [nellshamrell](https://github.com/nellshamrell)
* [llogiq](https://github.com/llogiq)
* [ericseppanen](https://github.com/ericseppanen)
* [extrawurst](https://github.com/extrawurst)
* [U007D](https://github.com/U007D)
* [mariannegoldin](https://github.com/mariannegoldin)
* [bdillo](https://github.com/bdillo)
* [opeolluwa](https://github.com/opeolluwa)
* [bnchi](https://github.com/bnchi)
* [KannanPalani57](https://github.com/KannanPalani57)
* [tzilist](https://github.com/tzilist)

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
