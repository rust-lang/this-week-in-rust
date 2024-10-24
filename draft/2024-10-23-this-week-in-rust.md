Title: This Week in Rust 570
Number: 570
Date: 2024-10-23
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://x.com/ThisWeekInRust) on X (formerly Twitter) or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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
* [Announcing Rust 1.82.0](https://blog.rust-lang.org/2024/10/17/Rust-1.82.0.html)

### Foundation

### Newsletters

### Project/Tooling Updates
* [Ratatui 0.29.0](https://ratatui.rs/highlights/v029/)
* [Life of a Zed Extension: Rust, WIT, Wasm](https://zed.dev/blog/zed-decoded-extensions)
* [Shuttle’s New Platform — Redefining Backend Development](https://www.shuttle.dev/blog/2024/10/10/shuttle-redefining-backend-development)
* [Hifitime version 4.0.0: A Leap Forward in Time Management](https://nyxspace.com/blog/2024/10/17/hifitime-version-400-a-leap-forward-in-time-management/)
* [Fjall 2.2 - now supports serializable snapshot isolation (multi-writer transactions)](https://fjall-rs.github.io/post/announcing-fjall-22/)

### Observations/Thoughts
* [Rust's design goals should be about code](https://tmandry.gitlab.io/blog/posts/the-main-thing/)
* [UnpinCell](https://without.boats/blog/unpin-cell/)
* [Blocking code is a leaky abstraction](https://notgull.net/blocking-leaky/)
* [Life as a Rust Project Contributor](https://yaah.dev/staying-involved)
* [Rustls Outperforms OpenSSL and BoringSSL](https://www.memorysafety.org/blog/rustls-performance-outperforms/)
* [Using libgdx texture atlases in Bevy](https://rustunit.com/blog/2024/10-21-bevy-libgdx-atlas/)
* [audio] [Rust in Production - Zed with Conrad Irwin](https://corrode.dev/podcast/s03e01-zed/)
* [audio] [Async Allocators](https://sdr-podcast.com/episodes/async-allocators/)
* [audio] [PubNub with Stephen Blum](https://rustacean-station.org/episode/stephen-blum/)

### Rust Walkthroughs
* [Demystifying Alignment and Memory Layout in Rust](https://garden.christophertee.dev/blogs/Memory-Alignment-and-Layout/Part-1)
* [Using Rust in Non-Rust Servers to Improve Performance](https://github.com/pretzelhammer/rust-blog/blob/master/posts/rust-in-non-rust-servers.md)
* [Async Rust in Three Parts](https://jacko.io/async_intro.html)
* [ When should I use String vs &str? ](https://steveklabnik.com/writing/when-should-i-use-string-vs-str/)
* [Using Web Workers in Rust Webapps](https://allwright.io/#/blog/20241016-using-web-workers.md)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [trait-gen](https://crates.io/crates/trait-gen), an attribute macro to generate the trait implementations for several types without needing custom declarative macros, code repetition, or blanket implementations.

Thanks to [Luke Peterson](https://users.rust-lang.org/t/crate-of-the-week/2704/1358) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Calls for Testing go here, use this format:
    * [<RFC Topic>](<RFC URL>)
        * [Tracking Issue](<Tracking Issue URL>)
        * [Testing steps](<Testing Steps URL>)
-->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- Remember to remove the `call-for-testing` label from the RFC so that the maintainer can signal for testers again, if desired. -->

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

* [Rama — add "Deny All" Dns Resolver](https://github.com/plabayo/rama/issues/329)
* [Rama — expand support to hijack based on context data](https://github.com/plabayo/rama/issues/328)
* [Rama — support vec/array impl for DnsResolver](https://github.com/plabayo/rama/issues/332)
* [Rama — Support IP modes in connector and resolver](https://github.com/plabayo/rama/issues/331)



If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.


<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

464 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-10-15..2024-10-22

* [make `rustc_abi` compile on stable again](https://github.com/rust-lang/rust/pull/131997)
* [`optimize` attribute applied to things other than methods/functions/c…](https://github.com/rust-lang/rust/pull/131814)
* [`rust_for_linux: -Zregparm=<N>` commandline flag for X86](https://github.com/rust-lang/rust/pull/130432)
* [`rustc_llvm`: Fix flattened CLI args](https://github.com/rust-lang/rust/pull/131805)
* [add `&pin (mut|const) T` type position sugar](https://github.com/rust-lang/rust/pull/130635)
* [add getentropy for RTEMS](https://github.com/rust-lang/rust/pull/131774)
* [added more scenarios where comma to be removed in the function arg](https://github.com/rust-lang/rust/pull/126588)
* [allow `#[deny]` inside `#[forbid]` as a no-op](https://github.com/rust-lang/rust/pull/121560)
* [allow dropping dyn principal](https://github.com/rust-lang/rust/pull/131857)
* [always specify `llvm_abiname` for RISC-V targets](https://github.com/rust-lang/rust/pull/131807)
* [autodiff Upstreaming - enzyme frontend](https://github.com/rust-lang/rust/pull/129458)
* [change orphan hint from "only" to "any uncovered type inside..."](https://github.com/rust-lang/rust/pull/128391)
* [check for filecheck directives in files marked `skip-filecheck`](https://github.com/rust-lang/rust/pull/131927)
* [compiler: adopt rust-analyzer impls for `LayoutCalculatorError`](https://github.com/rust-lang/rust/pull/131942)
* [compiler: error on layout of enums with invalid reprs](https://github.com/rust-lang/rust/pull/131843)
* [compiler: use LLVM's Comdat support](https://github.com/rust-lang/rust/pull/131876)
* [continue to get rid of `ty::Const::{try_}eval*`](https://github.com/rust-lang/rust/pull/130950)
* [coverage: make counter creation handle node/edge counters more uniformly](https://github.com/rust-lang/rust/pull/131918)
* [default to the medium code model on OpenHarmony LoongArch target](https://github.com/rust-lang/rust/pull/131874)
* [delay ambiguous intra-doc link resolution after `Cache` has been populated](https://github.com/rust-lang/rust/pull/131691)
* [do not run test where it cannot run](https://github.com/rust-lang/rust/pull/131835)
* [don't check unsize goal in MIR validation when opaques remain](https://github.com/rust-lang/rust/pull/130989)
* [don't report `on_unimplemented` message for negative traits](https://github.com/rust-lang/rust/pull/131701)
* [don't report bivariance error when nesting a `struct` with field errors into another `struct`](https://github.com/rust-lang/rust/pull/131754)
* [dont ICE when computing coverage of synthetic async closure body](https://github.com/rust-lang/rust/pull/131802)
* [dont consider predicates that may hold as impossible in `is_impossible_associated_item`](https://github.com/rust-lang/rust/pull/131840)
* [enable XRay instrumentation for LoongArch Linux targets](https://github.com/rust-lang/rust/pull/131818)
* [fix coherence error for very large tuples™](https://github.com/rust-lang/rust/pull/132001)
* [fix range misleading field access](https://github.com/rust-lang/rust/pull/131537)
* [handle gracefully true/false in `cfg(target(..))` compact](https://github.com/rust-lang/rust/pull/131771)
* [implement edition 2024 match ergonomics restrictions](https://github.com/rust-lang/rust/pull/131381)
* [make `unsupported_calling_conventions` a hard error](https://github.com/rust-lang/rust/pull/129935)
* [make destructors on `extern "C"` frames to be executed](https://github.com/rust-lang/rust/pull/129582)
* [make some float methods unstable `const fn`](https://github.com/rust-lang/rust/pull/130568)
* [make sure that outer opaques capture inner opaques's lifetimes even with precise capturing syntax](https://github.com/rust-lang/rust/pull/131789)
* [never emit `vptr` for empty/auto traits](https://github.com/rust-lang/rust/pull/131864)
* [register `src/tools/unicode-table-generator` as a runnable tool](https://github.com/rust-lang/rust/pull/131647)
* [remove invalid help diagnostics for const pointer](https://github.com/rust-lang/rust/pull/127675)
* [return values larger than 2 registers using a return area pointer](https://github.com/rust-lang/rust/pull/131211)
* [setting up indirect access to external data for loongarch64-linux-{musl,ohos}](https://github.com/rust-lang/rust/pull/131583)
* [try to improve error messages involving aliases in the solver](https://github.com/rust-lang/rust/pull/131699)
* [warn less about non-exhaustive in ffi](https://github.com/rust-lang/rust/pull/116863)
* [miri: `epoll_ctl`: throw unsupported error on unsupported opcode](https://github.com/rust-lang/miri/pull/3982)
* [miri: android: added support for prctl handling thread names](https://github.com/rust-lang/miri/pull/3899)
* [miri: improve support for `f16` and `f128`](https://github.com/rust-lang/miri/pull/3977)
* [add fast-path when computing the default visibility](https://github.com/rust-lang/rust/pull/131686)
* [use `ThinVec` for PredicateObligation storage](https://github.com/rust-lang/rust/pull/131422)
* [finish stabilization of `result_ffi_guarantees`](https://github.com/rust-lang/rust/pull/130628)
* [stabilize Strict Provenance and Exposed Provenance APIs](https://github.com/rust-lang/rust/pull/130350)
* [stabilize `-Znext-solver=coherence` again](https://github.com/rust-lang/rust/pull/130654)
* [add `from_ref` and `from_mut` constructors to `core::ptr::NonNull`](https://github.com/rust-lang/rust/pull/130822)
* [add `must_use` to `CommandExt::exec`](https://github.com/rust-lang/rust/pull/131833)
* [avoid use imports in `thread_local_inner!`](https://github.com/rust-lang/rust/pull/131866)
* [mark the unstable `LazyCell::into_inner` const](https://github.com/rust-lang/rust/pull/131712)
* [optimize `Box::default` and `Arc::default` to construct more types in place](https://github.com/rust-lang/rust/pull/131460)
* [optimize str.replace](https://github.com/rust-lang/rust/pull/130223)
* [partially stabilize `const_pin`](https://github.com/rust-lang/rust/pull/130136)
* [refactor some `core::fmt` macros](https://github.com/rust-lang/rust/pull/131730)
* [avoid superfluous UB checks in `IndexRange`](https://github.com/rust-lang/rust/pull/131572)
* [relax a memory order in `once_box`](https://github.com/rust-lang/rust/pull/131746)
* [speedup directory traversal on windows](https://github.com/rust-lang/rust/pull/131972)
* [std: uefi: add basic Env variables](https://github.com/rust-lang/rust/pull/127462)
* [uefi: implement getcwd and chdir](https://github.com/rust-lang/rust/pull/129794)
* [cargo: registry: HttpRegistry `block_until_ready` returns early when work is still pending](https://github.com/rust-lang/cargo/pull/14694)
* [cargo: resolver: avoid cloning when iterating using RcVecIter](https://github.com/rust-lang/cargo/pull/14690)
* [cargo: stabilize MSRV-aware resolver config](https://github.com/rust-lang/cargo/pull/14639)
* [rustdoc-json-types: introduce rustc-hash feature](https://github.com/rust-lang/rust/pull/131936)
* [rustdoc-json-types: mark simple enums as copy](https://github.com/rust-lang/rust/pull/131976)
* [rustdoc: switch from FxHash to sha256 for static file hashing](https://github.com/rust-lang/rust/pull/131908)
* [rustfmt `for<'a> async` correctly](https://github.com/rust-lang/rust/pull/131657)
* [rustfmt: `compile_rustfmt` rewrite](https://github.com/rust-lang/rustfmt/pull/6275)
* [rustfmt: apply 2024 version sort algorithm to mods](https://github.com/rust-lang/rustfmt/pull/6368)
* [rustfmt: defer changes for zero argument functions until `style_edition=2027`](https://github.com/rust-lang/rustfmt/pull/6362)
* [clippy: add lint for unnecessary lifetime bounded `&str` return](https://github.com/rust-lang/rust-clippy/pull/13395)
* [clippy: allow to go through clippy lints page without javascript](https://github.com/rust-lang/rust-clippy/pull/13539)
* [clippy: change the category of `manual_is_power_of_two` to `pedantic`](https://github.com/rust-lang/rust-clippy/pull/13553)
* [clippy: stop linting `manual_bits` in any macro invocation](https://github.com/rust-lang/rust-clippy/pull/13564)
* [rust-analyzer: add wrap/unwrap return type in Option](https://github.com/rust-lang/rust-analyzer/pull/18294)
* [rust-analyzer: clamp `Position::character` to line length](https://github.com/rust-lang/rust-analyzer/pull/18243)
* [rust-analyzer: do not consider match/let/ref of place that evaluates to ! to diverge, disallow coercions from them too](https://github.com/rust-lang/rust-analyzer/pull/18278)
* [rust-analyzer: better completions for extern blocks](https://github.com/rust-lang/rust-analyzer/pull/18360)
* [rust-analyzer: goto definition on range operators](https://github.com/rust-lang/rust-analyzer/pull/18362)
* [rust-analyzer: initial support for `safe_kw` in extern blocks](https://github.com/rust-lang/rust-analyzer/pull/18350)
* [rust-analyzer: support initializeStopped setting](https://github.com/rust-lang/rust-analyzer/pull/18359)
* [rust-analyzer: fix status bar message not being marked markdown](https://github.com/rust-lang/rust-analyzer/pull/18366)
* [rust-analyzer: classify `safe` as a contextual keyword](https://github.com/rust-lang/rust-analyzer/pull/18354)
* [rust-analyzer: fix token downmapping failing for include! inputs](https://github.com/rust-lang/rust-analyzer/pull/18361)
* [rust-analyzer: private items are shown in completions for modules in fn body](https://github.com/rust-lang/rust-analyzer/pull/18337)

### Rust Compiler Performance Triage

Some tidy improvements from switching to next generation trait solver (solely for coherence checking) and from simplifying our dataflow analysis framework. There were some binary size regressions associated with PR 126557 (adding `#[track_caller]` to allocating methods of `Vec` and `VecDeque`), which I have handed off to T-libs to choose whether to investigate further.

Triage done by **@pnkfelix**.
Revision range: [5ceb623a..3e33bda0](https://perf.rust-lang.org/?start=5ceb623a4abd66e91e7959d25caaf0523f1a7f7c&end=3e33bda0326586a6e1e34d0f5c060ca6d116e6a4&absolute=false&stat=instructions%3Au)

0 Regressions, 3 Improvements, 6 Mixed; 3 of them in rollups
47 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/81de3d5e2cc599cc49bc11c64f9a5b911f3a83dd/triage/2024-10-21.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

<!-- Approved RFCs go here, use this format: * [Topic](URL) -->
<!-- or if none were approved this week, use: * *No RFCs were approved this week.* -->
<!-- * []() -->

<!--
### [Approved Major Change Proposals (MCP)](https://forge.rust-lang.org/compiler/mcp.html)
<!~~ MCPs occur infrequently, so this section is commented out by default. ~~>
<!~~ MCPs which have been approved or rejected this week go here, use this format: * [major change accepted|rejected] [Topic](URL) ~~>
-->

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

<!-- RFCs which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No RFCs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

<!-- Tracking Issues which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No Tracking Issues or PRs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
<!-- Remove this section if empty>

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
<!-- Remove this section if empty>

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

<!-- New or updated RFCs go here, use this format: * [new|updated] [Topic](URL) -->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- * [new|updated] []() -->

## Upcoming Events

Rusty Events between 2024-10-23 - 2024-11-20 🦀

### Virtual
* 2024-10-24 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Part 4 of 4 - Hackathon Showcase: Final Projects and Presentations**](https://www.meetup.com/women-in-rust/events/303213835/)
* 2024-10-24 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633271/)
* 2024-10-25 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304100127/)
* 2024-10-26 | Virtual (Gdansk, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-trojmiasto/events/303550643/)
* 2024-10-29 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/301585671/)
* 2024-10-31 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898048/)
* 2024-10-31 | Virtual (Nürnberg, DE) | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820274/)
* 2024-11-01 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcpbcb/)
* 2024-11-02 | Virtual( Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2024-11-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031651/)
* 2024-11-07 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633272/)
* 2024-11-08 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304099245/)
* 2024-11-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346985/)
* 2024-11-14 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898070/)
* 2024-11-14 | Virtual and In-Person (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**November Meetup**](https://www.meetup.com/join-srug/events/304166747/)
* 2024-11-15 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcpbtb/)
* 2024-11-19 | Virtual (Los Angeles, CA, US) | [DevTalk LA](https://www.meetup.com/lajugstudygroup/)
    * [**Discussion - Topic: Rust for UI**](https://www.meetup.com/lajugstudygroup/events/302952703/)
* 2024-11-19 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346971/)
* 2024-11-20 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Embedded Rust Workshop**](https://www.meetup.com/vancouver-rust/events/304047664/)

### Asia
* 2024-10-29 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup/)
    * [**Bug-Free Concurrency in Rust**](https://www.meetup.com/tokyo-rust-meetup/events/304107583/)

### Europe
* 2024-10-26 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Ferris' Fika Forum #6**](https://www.meetup.com/stockholm-rust/events/303918943/)
* 2024-10-28 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust Meetup #71**](https://www.meetup.com/rust-paris/events/303663366/)
* 2024-10-29 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/303479865)
* 2024-10-30 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn October 2024**](https://www.meetup.com/rust-meetup-hamburg/events/303373054/)
* 2024-10-31 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/300820289/)
* 2024-11-06 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123398/)
* 2024-11-06 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.fr/o/paris-rustaceans-74289178383)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1037795553437)
* 2024-11-14 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @UXStream**](https://www.meetup.com/stockholm-rust/events/304124737/)
* 2024-11-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Daten sichern mit ZFS (und Rust)**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425200/)

### North America
* 2024-10-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcnbfc/)
* 2024-10-26 | Newark, NJ, US | [NJ Code & Coffee](https://www.meetup.com/nj-code-coffee/)
    * [**Intro to Rust: Build a Text Adventure Game x NJ Code & Coffee**](https://www.meetup.com/nj-code-coffee/events/304149949/)
* 2024-10-27 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Kendall Rust Lunch, Oct 27**](https://www.meetup.com/bostonrust/events/303708359/)
* 2024-10-28 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**Generics from the Ground Up**](https://www.meetup.com/boulder-rust-meetup/events/303766925/)
* 2024-10-28 | Ferndale, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ferndale**](https://www.meetup.com/detroitrust/events/303909299/)
* 2024-10-28 | Minneapolis, MN US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/303884468/)
* 2024-10-29 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers : State of the Group and Expectations for 2025**](https://www.meetup.com/music-city-rust-developers/events/301425524/)
* 2024-10-30 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Workshop: deploying your code**](https://www.meetup.com/deep-dish-rust/events/304071348/)
* 2024-11-04 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, Nov 4**](https://www.meetup.com/bostonrust/events/303708387/)
* 2024-11-07 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Game development with Rust and the Bevy engine**](https://www.meetup.com/stl-rust/events/302371464/)
* 2024-11-12 | Ann Arbor, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcpbqb/)
* 2024-11-15 | Mexico City, DF, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Multi threading y Async en Rust parte 2 - Smart Pointes y Closures**](https://www.meetup.com/rust-mx/events/304150412/)
* 2024-11-15 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch, Nov 15**](https://www.meetup.com/bostonrust/events/303708398/)

### Oceania
* 2024-10-28 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**October 2024 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/304034898/)
* 2024-10-29 | Canberra, ACT, AU | [Canberra Rust User Group (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/303128131/)
* 2024-10-31 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Rust on AWS: Sustainability + Peace: Zero Stress Automation**](https://www.meetup.com/rust-akl/events/303824919/)

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

> Your problem is that you’re trying to borrow from the dead.

– [/u/masklinn on /r/rust](https://old.reddit.com/r/rust/comments/1g3a2ul/hey_rustaceans_got_a_question_ask_here_422024/lrzqed7/)

Thanks to [Maciej Dziardziel](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1622) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
