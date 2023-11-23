Title: This Week in Rust 522
Number: 522
Date: 2023-11-22
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

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
* [Announcing Rust 1.74.0](https://blog.rust-lang.org/2023/11/16/Rust-1.74.0.html)

### Foundation

### Newsletters

### Project/Tooling Updates
* [hyper v1](https://seanmonstar.com/blog/hyper-v1/)
* [Rocket v0.5: Stable, Async, Sentinels, Streams, SSE, Forms, WebSockets, & So Much More](https://rocket.rs/v0.5/news/2023-11-17-version-0.5/)
* [GlueSQL v0.15 - Python, Redis and CSV support](https://github.com/gluesql/gluesql/releases/tag/v0.15.0)
* [Meilisearch releases v1.5 - faster indexing, on-demand snapshots, and Puffin reports](https://blog.meilisearch.com/meilisearch-1-5/)
* [Nutype v0.4.0 - newtype with constraints](https://github.com/greyblake/nutype/releases/tag/v0.4.0)
* [Announcing open-ai-safe: a typesafe OpenAI API Rust client](https://youtu.be/x11tBhokFNc)

### Observations/Thoughts
* [A JVM in Rust part 8 - Retrospective](https://andreabergia.com/blog/2023/11/a-jvm-in-rust-part-8-retrospective/)
* [Rust 1.74.0: All 45 changes in 19 minutes!](https://www.youtube.com/watch?v=MOzuShpnUm8)
* [Fun with lexical analysis and Rust](https://blog.blotato.com/fun-with-lexical-analysis-rust/)
* [Wasmtime and Cranelift in 2023](https://bytecodealliance.org/articles/wasmtime-and-cranelift-in-2023)
* [Signals vs. Servers](https://blog.adamchalmers.com/signals-vs-servers/)
* [Function interposition in Rust with upgrayedd](https://blog.yossarian.net/2023/11/19/Function-interposition-in-Rust-with-upgrayedd)
* [A close encounter with false sharing](https://morestina.net/blog/1976/a-close-encounter-with-false-sharing)
* [Edge IoT with Rust on ESP: MQTT Publisher](https://apollolabsblog.hashnode.dev/edge-iot-with-rust-on-esp-mqtt-publisher)
* [Checking semver in the presence of doc(hidden) items](https://predr.ag/blog/checking-semver-for-doc-hidden-items/)
* [Push Ifs Up And Fors Down](https://matklad.github.io/2023/11/15/push-ifs-up-and-fors-down.html)
* [Building Segmented Logs in Rust: From Theory to Production!](https://arindas.github.io/blog/segmented-log-rust/)
* [Writing an async runtime generic library](https://www.sea-ql.org/blog/2023-11-22-async-runtime-generic/)
* [Ferrostar: Building a Cross-Platform Navigation SDK in Rust (Part 1)](https://stadiamaps.com/news/ferrostar-building-a-cross-platform-navigation-sdk-in-rust-part-1/)

### Rust Walkthroughs
* [This is how I made the runtime this website runs on!](https://aandreba.com/article/this-is-how-i-made-the-runtime-this-website-runs-on)
* [video] [Tokenizing and parsing a programming language in Rust, by Adam Chalmers](https://www.youtube.com/watch?v=LUcI6KkM-PE)

### Research

### Miscellaneous
* [Building a Better Foundation for Rocket's Future](https://rocket.rs/v0.5/news/2023-11-17-rwf2-prelaunch/)
* [audio] [Rust Digger with Gabor Szabo](https://rustacean-station.org/episode/gabor-szabo/)
* [video] [Making the (partial) Rust BitTorrent client more reasonable](https://www.youtube.com/watch?v=r0srf3kfZbs)
* [video] [Rust Release Train 1.74](https://www.youtube.com/watch?v=Ciuhriopc00)
* [video] [EuroRust 2023](https://www.youtube.com/playlist?list=PLH6-VpZ3SvUUKFSEPEWiHQi4JqebBj9Tq)

## Crate of the Week

This week's crate is [rocket](https://rocket.rs), an opinionated web framework that aims to be really ergonomic while still being fast.

Thanks to [David Mason](https://users.rust-lang.org/t/crate-of-the-week/2704/1265) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [ Hyperswitch - [REFACTOR]: [Zen] MCA metadata validation ](https://github.com/juspay/hyperswitch/issues/2913)
* [ Hyperswitch - [FEATURE] Standardize state field in billing and shipping address ](https://github.com/juspay/hyperswitch/issues/2939)
* [ Hyperswitch - [BUG]: MCA metadata deserialization failures should be 4xx ](https://github.com/juspay/hyperswitch/issues/2899)
* [ Hyperswitch - [Feature]: [NMI] Sync with Hyperswitch Reference](https://github.com/juspay/hyperswitch/issues/2905)
* [ Hyperswitch - [Feature]: [Zen] Sync with Hyperswitch Reference](https://github.com/juspay/hyperswitch/issues/2908)
* [Ockam - `Cargo.toml` feature management should allow building individual crates with default feature set](https://github.com/build-trust/ockam/issues/5491)
* [Ockam - Improve error handling of multiple `ockam tcp-outlet create` calls](https://github.com/build-trust/ockam/issues/5897)
* [Ockam - Library - Slim down the `NodeManagerWorker` for `node / tcp`](https://github.com/build-trust/ockam/issues/6708)
* [Ockam - Library - Slim down the `NodeManagerWorker` for `node / credentials`](https://github.com/build-trust/ockam/issues/6709)
* [Ockam - Command - refactor to use typed interfaces to implement commands for `secure channel` and `secure channel listener`](https://github.com/build-trust/ockam/issues/6699)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

369 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-11-13..2023-11-20

* [add arm64e-apple-ios & arm64e-apple-darwin targets](https://github.com/rust-lang/rust/pull/115526)
* [remove asmjs](https://github.com/rust-lang/rust/pull/117338)
* [add -Z `llvm_module_flag`](https://github.com/rust-lang/rust/pull/116555)
* [add richer structure for Stable MIR Projections](https://github.com/rust-lang/rust/pull/117517)
* [adjust frame IP in backtraces relative to image base for SGX target](https://github.com/rust-lang/rust/pull/117895)
* [always point at index span on index obligation failure](https://github.com/rust-lang/rust/pull/117856)
* [avoid iterating over hashmaps in astconv](https://github.com/rust-lang/rust/pull/117828)
* [build pre-coroutine-transform coroutine body on error](https://github.com/rust-lang/rust/pull/117686)
* [ensure sanity of all computed ABIs](https://github.com/rust-lang/rust/pull/117500)
* [fix insertion of statements to be executed along return edge in inlining](https://github.com/rust-lang/rust/pull/117783)
* [handle attempts to have multiple `cfg`d tail expressions](https://github.com/rust-lang/rust/pull/117988)
* [ignore but do not assume region obligations from unifying headers in negative coherence](https://github.com/rust-lang/rust/pull/117994)
* [ignore implied bounds with placeholders](https://github.com/rust-lang/rust/pull/112422)
* [lint pinned `#[must_use]` pointers (in particular, `Box<T>` where `T` is `#[must_use]`) in `unused_must_use`](https://github.com/rust-lang/rust/pull/118054)
* [make `LayoutError::Cycle` carry `ErrorGuaranteed`](https://github.com/rust-lang/rust/pull/117849)
* [make regionck care about placeholders in outlives components](https://github.com/rust-lang/rust/pull/118000)
* [more detail when expecting expression but encountering bad macro argument](https://github.com/rust-lang/rust/pull/114292)
* [new solver normalization improvements](https://github.com/rust-lang/rust/pull/117278)
* [patterns: don't ICE when encountering a raw str slice](https://github.com/rust-lang/rust/pull/117807)
* [recover `dyn` and `impl` after `for<...>`](https://github.com/rust-lang/rust/pull/117891)
* [remove incorrect transformation from RemoveZsts](https://github.com/rust-lang/rust/pull/117801)
* [remove legacy bitcode defaults from all Apple specs](https://github.com/rust-lang/rust/pull/117364)
* [better handle type errors involving `Self` literals](https://github.com/rust-lang/rust/pull/117959)
* [add some additional warnings for duplicated diagnostic items](https://github.com/rust-lang/rust/pull/117742)
* [suggest dereferencing the LHS for binops such as `&T == T`](https://github.com/rust-lang/rust/pull/117893)
* [on resolve error of `[rest..]`, suggest `[rest @ ..]`](https://github.com/rust-lang/rust/pull/117998)
* [try to use approximate placeholder regions when outputting an AscribeUserType error in borrowck](https://github.com/rust-lang/rust/pull/116097)
* [when a local binding shadows a fn, point at fn def in call failure](https://github.com/rust-lang/rust/pull/117924)
* [when encountering `struct` fn call literal with private fields, suggest all builders](https://github.com/rust-lang/rust/pull/117683)
* [when using existing fn as module, don't claim it doesn't exist](https://github.com/rust-lang/rust/pull/117964)
* [interpret: simplify handling of shifts by no longer trying to handle signed and unsigned shift amounts in the same branch](https://github.com/rust-lang/rust/pull/117832)
* [custom MIR: Support cleanup blocks](https://github.com/rust-lang/rust/pull/117330)
* [emit smir](https://github.com/rust-lang/rust/pull/117745)
* [add CoroutineWitness to covered types in smir](https://github.com/rust-lang/rust/pull/117787)
* [miri: cargo-miri: when verbose, print where the sysroot is being built](https://github.com/rust-lang/miri/pull/3175)
* [miri: get rid of our last uses of `set_var`](https://github.com/rust-lang/miri/pull/3168)
* [miri: implement all 16 AVX compare operators for 128-bit SIMD vectors](https://github.com/rust-lang/miri/pull/3176)
* [miri: reallocarray shim linux/freebsd support proposal](https://github.com/rust-lang/miri/pull/3166)
* [reenable effects in libcore](https://github.com/rust-lang/rust/pull/117825)
* [if available use a Child's pidfd for kill/wait](https://github.com/rust-lang/rust/pull/117957)
* [fix rounding issue with exponents in `fmt`](https://github.com/rust-lang/rust/pull/116301)
* [add `Seek::seek_relative`](https://github.com/rust-lang/rust/pull/116750)
* [impl more traits for `ptr::Alignment,` add mask method](https://github.com/rust-lang/rust/pull/115249)
* [feat: implement `DoubleEndedSearcher` for `CharArray[Ref]Searcher`](https://github.com/rust-lang/rust/pull/111922)
* [hashbrown: avoid using unstable `ptr::invalid_mut`](https://github.com/rust-lang/hashbrown/pull/481)
* [futures: fillBuf: don't poll a second time on EOF](https://github.com/rust-lang/futures-rs/pull/2801)
* [futures: remove redundant `impl Unpin`](https://github.com/rust-lang/futures-rs/pull/2800)
* [cargo-credential-1password: add missing `--account` argument to `op signin` command](https://github.com/rust-lang/cargo/pull/12985)
* [cargo: add color output for `cargo --list`](https://github.com/rust-lang/cargo/pull/12992)
* [cargo resolver: Don't do git fetches when updating workspace members](https://github.com/rust-lang/cargo/pull/12975)
* [cargo resolver: Prefer MSRV, rather than ignore incompatible](https://github.com/rust-lang/cargo/pull/12950)
* [cargo: fix `--check-cfg` invocations with zero features](https://github.com/rust-lang/cargo/pull/13011)
* [cargo: fix error message for duplicate links](https://github.com/rust-lang/cargo/pull/12973)
* [cargo: handle `$message_type` in JSON diagnostics](https://github.com/rust-lang/cargo/pull/13016)
* [cargo: if the only path is a loop then counted as the shortest path](https://github.com/rust-lang/cargo/pull/12977)
* [cargo: ignore `changing_spec_relearns_crate_types` on windows-gnu](https://github.com/rust-lang/cargo/pull/12972)
* [cargo: only filter out target if its in the package root](https://github.com/rust-lang/cargo/pull/12944)
* [remove unneeded `unknown` variable and `Symbol` creation when iterating over items in rustdoc rendering](https://github.com/rust-lang/rust/pull/118051)
* [rustdoc-search: optimize unifyFunctionTypes](https://github.com/rust-lang/rust/pull/118024)
* [rustdoc-search: simplify the checkTypes fast path](https://github.com/rust-lang/rust/pull/117955)
* [rustfix: fix insert at beginning](https://github.com/rust-lang/rustfix/pull/224)
* [clippy: `impl_trait_in_params`: avoid ICE when function with `impl Trait` type has no parameters](https://github.com/rust-lang/rust-clippy/pull/11804)
* [clippy: `needless_return_with_question_mark` ignore let-else](https://github.com/rust-lang/rust-clippy/pull/11802)
* [clippy: change `if_same_then_else` to be a `style` lint](https://github.com/rust-lang/rust-clippy/pull/11809)
* [clippy: extend `maybe_misused_cfg` lint over `cfg(test)`](https://github.com/rust-lang/rust-clippy/pull/11821)
* [clippy: `manual_memcpy` reduce indexing suggestions when array length is equal to loop range](https://github.com/rust-lang/rust-clippy/pull/11778)
* [clippy: implement new lint `iter_over_hash_type`](https://github.com/rust-lang/rust-clippy/pull/11791)
* [clippy: improve maybe misused cfg](https://github.com/rust-lang/rust-clippy/pull/11840)
* [clippy: lint `flatten()` under `lines_filter_map_ok`](https://github.com/rust-lang/rust-clippy/pull/11691)
* [clippy: new lint `clippy::join_absolute_paths`](https://github.com/rust-lang/rust-clippy/pull/11453)
* [clippy: teach `eager_or_lazy` about panicky arithmetic operations](https://github.com/rust-lang/rust-clippy/pull/11002)
* [clippy: verify `Borrow<T>` semantics for types that implement Hash, `Borrow<str>` and `Borrow<[u8]>`](https://github.com/rust-lang/rust-clippy/pull/11781)
* [rust-analyzer: diagnose incorrect unsafety for trait impls](https://github.com/rust-lang/rust-analyzer/pull/15893)
* [rust-analyzer: diagnose missing assoc items in trait impls](https://github.com/rust-lang/rust-analyzer/pull/15895)
* [rust-analyzer: diagnose some orphan trait impl cases](https://github.com/rust-lang/rust-analyzer/pull/15891)
* [rust-analyzer: fix `PathSegment` grammar](https://github.com/rust-lang/rust-analyzer/pull/15875)
* [rust-analyzer: fix builtin line! expansion](https://github.com/rust-lang/rust-analyzer/pull/15903)
* [rust-analyzer: diagnose everything in nested items, not just def diagnostics](https://github.com/rust-lang/rust-analyzer/pull/15901)
* [rust-analyzer: handle default constant values in `trait_impl_missing_assoc_item` diagnostic](https://github.com/rust-lang/rust-analyzer/pull/15911)

### Rust Compiler Performance Triage

Pretty quiet week, with only a small number of statistically significant changes landing.

Triage done by **@simulacrum**.
Revision range: [173b6e68..4f3da90](https://perf.rust-lang.org/?start=173b6e686b158dbad7d072c64bef3ced2052312b&end=4f3da903a43f22ea33d2ca4435a24b42fc1f842a&absolute=false&stat=instructions%3Au)

1 Regressions, 1 Improvements, 1 Mixed; 0 of them in rollups
60 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-11-21.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Superseding public/private dependencies](https://github.com/rust-lang/rfcs/pull/3516)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [[style edition 2024] Combine all delimited exprs as last argument](https://github.com/rust-lang/rust/pull/114764)
* [disposition: merge] [Tracking Issue for `ptr::addr_eq`](https://github.com/rust-lang/rust/issues/116324)
* [disposition: merge] [Stabilize C string literals](https://github.com/rust-lang/rust/pull/117472)
* [disposition: merge] [Tracking Issue for mutex_unpoison](https://github.com/rust-lang/rust/issues/96469)

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Macro fragment specifiers edition policy](https://github.com/rust-lang/rfcs/pull/3531)
* [eRFC: Implement function delegation in rustc](https://github.com/rust-lang/rfcs/pull/3530)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-11-22 - 2023-12-20 🦀

### Virtual

* 2023-11-23 | Virtual (Edmonton, AB, CA) | [Edmonton R User Group - Yegrug](https://www.meetup.com/edmonton-r-user-group-yegrug/)
    * [**Edmonton R User Group Meetup: R and Rust, like a match made in heaven**](https://www.meetup.com/edmonton-r-user-group-yegrug/events/296605221/)
* 2023-11-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtyfcpblc/)
* 2023-11-28 | Virtual (Europe / Africa) | [Rust for Lunch](https://lunch.rs/)
    * [**Rust Meet-up**](https://lunch.rs/meetups/2023-11-28/)
* 2023-11-29 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Atomics & Locks Book Club Final Chapter! (Chapter 10)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583091/)
* 2023-11-30 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296833665/)
* 2023-11-30 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/rust-dublin/)
    * [**Automating expertise with cargo-semver-checks**](https://www.meetup.com/rust-dublin/events/296346693/)
* 2023-12-01 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Kick-Off!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583626/)
* 2023-12-02 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdssbdestsearch)
* 2023-12-05 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679797/) | [**Mirror**](https://berline.rs/)
* 2023-12-05 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/297021574/)
* 2023-12-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/297172140)
* 2023-12-12 | Virtual | [Mainmatter](https://mainmatter.com)
    * [**Workshop: Telemetry for Rust applications**](https://rust-telemetry-workshop.mainmatter.com)
* 2023-12-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/fvdtgtyfcqbqb/)
* 2023-12-14 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/295679660/)
* 2023-12-18 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - hybrid**](https://www.meetup.com/rust-munich/events/296429053/)
* 2023-12-20 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763506/)

### Europe

* 2023-11-23 | Biel/Bienne, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Talks Bern @ Biel: Embedded Edition**](https://www.meetup.com/rust-bern/events/296556498/)
* 2023-11-28 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**LDN Talks November 2023 with Helsing.ai**](https://www.meetup.com/rust-london-user-group/events/297257712/)
* 2023-11-30 | Brussels, BE | [Belgium Rust user group](https://www.meetup.com/fr-FR/belgium-rust-user-group/events/297538601/)
    * [**Lambda Brussels**](https://lambda-brussels.glitch.me/)
* 2023-11-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #42 sponsored by Nine A/S**](https://www.meetup.com/copenhagen-rust-community/events/297405705/)
* 2023-11-30 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/)
    * [**Rust Vienna Meetup - November**](https://www.meetup.com/rust-vienna/events/297382145/)
* 2023-11-30 | Zurich, CH| [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**November Meetup**](https://www.meetup.com/rust-zurich/events/297312190/)
* 2023-12-06 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/events)
    * [**December Meetup**](https://www.meetup.com/rustcologne/events/297100007/)
* 2023-12-07 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Danske Commodities**](https://www.meetup.com/rust-aarhus/events/296223513/)
* 2023-12-07 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #5**](https://www.meetup.com/meetup-group-zgphbyet/events/297477578/)
* 2023-12-14 | Augsburg, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Augsburg Rust Meetup #4**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/297025700/)
* 2023-12-18 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - hybrid**](https://www.meetup.com/rust-munich/events/296429053/)
* 2023-12-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Tauri, an Electron-alternative**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504276/)

### North America

* 2023-11-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcpbdc/)
* 2023-11-28 | Pasadena, CA, US | [Pasadena Thursday Go / Rust](https://www.meetup.com/thursday-go/)
    * [**Monthly Rust group**](https://www.meetup.com/thursday-go/events/297062186/)
* 2023-11-29 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/296657831/)
* 2023-12-12 | Seattle, WA, US | [Cap Hill Rust Coding/Hacking/Learning](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564619/)
* 2023-12-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfcqbzb/)

### Oceania

* 2023-11-28 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**November Meetup**](https://www.meetup.com/rust-canberra/events/296391733/)
* 2023-12-05 | Aukland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Advanced Async Tricks + Interruptible Software**](https://www.meetup.com/rust-akl/events/297271684/)
* 2023-12-11 | Perth, WA, AU | [Rust Perth Meetup Group](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Rust End of Year Event**](https://www.meetup.com/perth-rust-meetup-group/events/297191089/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/163w6fl/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> If you require it, measure it. That's the simple answer. Everything else is guesswork.

– [Johannes Lade on rust-users](https://users.rust-lang.org/t/rusts-forcing-of-using-pointers-when-writing-a-variable-printing-it/102627/12)

Thanks to [Michael Bryan](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1489) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
