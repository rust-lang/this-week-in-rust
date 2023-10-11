Title: This Week in Rust 516
Number: 516
Date: 2023-10-11
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
* [Announcing Rust 1.73.0](https://blog.rust-lang.org/2023/10/05/Rust-1.73.0.html)
* [Polonius update](https://blog.rust-lang.org/inside-rust/2023/10/06/polonius-update.html)

### Foundation

### Newsletters

### Project/Tooling Updates
* [rust-analyzer changelog #202](https://rust-analyzer.github.io/thisweek/2023/10/09/changelog-202.html)
* [Announcing: pid1 Crate for Easier Rust Docker Images - FP Complete](https://www.fpcomplete.com/blog/announcing-pid1-crate-for-easier-rust-docker-images/)
* [bit_seq in Rust: A Procedural Macro for Bit Sequence Generation](https://medium.com/@jozott/bit-seq-in-rust-a-procedural-macro-for-bit-sequence-generation-5a4b856d629a)
* [tcpproxy 0.4 released](https://neosmart.net/blog/tcpproxy-0-4-released/)
* [Rune 0.13](https://rune-rs.github.io/posts/rune-0-13-0/)
* [Rust on Espressif chips - September 29 2023](https://mabez.dev/blog/posts/esp-rust-29-09-2023/)
* [esp-rs quarterly planning: Q4 2023](https://beta7.io/posts/esp-rs-quarterly-planning-q4-2023/)
* [Implementing the `#[diagnostic]` namespace to improve rustc error messages in complex crates](https://blog.weiznich.de/blog/diagnostic-namespace-10-23/)

### Observations/Thoughts
* [Safety vs Performance. A case study of C, C++ and Rust sort implementations](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/sort_safety/text.md)
* [Raw SQL in Rust with SQLx](https://www.shuttle.rs/blog/2023/10/04/sql-in-rust)
* [Thread-per-core](https://without.boats/blog/thread-per-core/)
* [Edge IoT with Rust on ESP: HTTP Client](https://apollolabsblog.hashnode.dev/edge-iot-with-rust-on-esp-http-client)
* [The Ultimate Data Engineering Chadstack. Running Rust inside Apache Airflow](https://www.confessionsofadataguy.com/the-ultimate-data-engineering-chadstack-running-rust-inside-apache-airflow/)
* [Why Rust doesn't need a standard div_rem: An LLVM tale - CodSpeed](https://codspeed.io/blog/why-rust-doesnt-need-a-standard-divrem)
* [Making Rust supply chain attacks harder with Cackle](https://davidlattimore.github.io/making-supply-chain-attacks-harder.html)
* [video] [Rust 1.73.0: Everything Revealed in 16 Minutes](https://youtu.be/xXdfLNIHkPk)

### Rust Walkthroughs
* [Let's Build A Cargo Compatible Build Tool - Part 5](https://ductile.systems/freight-part-5/)
* [How we reduced the memory usage of our Rust extension by 4x](https://blog.kolo.app/optimising-rust-memory.html)
* [Calling Rust from Python](https://blog.frankel.ch/rust-from-python/)
* [Acceptance Testing embedded-hal Drivers](https://tommy-gilligan.github.io/driver-acceptance.html)
* [5 ways to instantiate Rust structs in tests](https://jmmv.dev/2023/10/rust-test-structs.html)

### Research
* [Looking for Bad Apples in Rust Dependency Trees Using GraphQL and Trustfall](https://medium.com/volvo-cars-engineering/looking-for-bad-apples-in-rust-dependency-trees-using-graphql-and-trustfall-cb88b835f652)

### Miscellaneous
* [Rust, Open Source, Consulting - Interview with Matthias Endler](https://filtra.io/rust-corrode-oct-23)
* [Edge IoT with Rust on ESP: Connecting WiFi](https://apollolabsblog.hashnode.dev/edge-iot-with-rust-on-esp-connecting-wifi)
* [Bare-metal Rust in Android](https://security.googleblog.com/2023/10/bare-metal-rust-in-android.html)
* [audio] [Learn Rust in a Month of Lunches with Dave MacLeod](https://rustacean-station.org/episode/dave-macleod/)
* [video] [Rust 1.73.0: Everything Revealed in 16 Minutes](https://www.youtube.com/watch?v=xXdfLNIHkPk)
* [video] [Rust 1.73 Release Train](https://www.youtube.com/watch?v=oMYKGhPw9Sc)
* [video] [Why is the JavaScript ecosystem switching to Rust?](https://www.youtube.com/watch?v=dZQMoEWe5uY)

## Crate of the Week

This week's crate is [yarer](https://crates.io/crates/yarer), a library and command-line tool to evaluate mathematical expressions.

Thanks to [Gianluigi Davassi](https://users.rust-lang.org/t/crate-of-the-week/2704/1246) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Ockam - Make `ockam node delete` (no args) interactive by asking the user to choose from a list of nodes to delete (tuify)](https://github.com/build-trust/ockam/issues/6450)
* [Ockam - Improve `ockam enroll ----help` text by adding doc comment for `identity` flag (`clap` command)](https://github.com/build-trust/ockam/issues/6054)
* [Ockam - Enroll "email: '+' character not allowed"](https://github.com/build-trust/ockam/issues/6095)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

384 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-10-02..2023-10-09

* [formally demote tier 2 MIPS targets to tier 3](https://github.com/rust-lang/rust/pull/115238)
* [add tvOS to `target_os` for `register_dtor`](https://github.com/rust-lang/rust/pull/116500)
* [linker: remove `-Zgcc-ld` option](https://github.com/rust-lang/rust/pull/116514)
* [linker: remove unstable legacy CLI linker flavors](https://github.com/rust-lang/rust/pull/116515)
* [`non_lifetime_binders`: fix ICE in lint opaque-hidden-inferred-bound](https://github.com/rust-lang/rust/pull/116379)
* [add `async_fn_in_trait` lint](https://github.com/rust-lang/rust/pull/116184)
* [add a note to duplicate diagnostics](https://github.com/rust-lang/rust/pull/116428)
* [always preserve DebugInfo in DeadStoreElimination](https://github.com/rust-lang/rust/pull/116183)
* [bring back generic parameters for indices in `rustc_abi` and make it compile on stable](https://github.com/rust-lang/rust/pull/116269)
* [coverage: allow each coverage statement to have multiple code regions](https://github.com/rust-lang/rust/pull/115301)
* [detect missing `=>` after match guard during parsing](https://github.com/rust-lang/rust/pull/116400)
* [diagnostics: be more careful when suggesting `struct` fields](https://github.com/rust-lang/rust/pull/116429)
* [don't suggest nonsense suggestions for unconstrained type vars in `note_source_of_type_mismatch_constraint`](https://github.com/rust-lang/rust/pull/116158)
* [dont call `mir.post_mono_checks` in codegen](https://github.com/rust-lang/rust/pull/116277)
* [emit feature gate *warning* for `auto` traits pre-expansion](https://github.com/rust-lang/rust/pull/116393)
* [ensure that `~const` trait bounds on associated functions are in const traits or impls](https://github.com/rust-lang/rust/pull/116210)
* [extend `impl`'s `def_span` to include its where clauses](https://github.com/rust-lang/rust/pull/116497)
* [fix detecting references to packed unsized fields](https://github.com/rust-lang/rust/pull/115583)
* [fix fast-path for `try_eval_scalar_int`](https://github.com/rust-lang/rust/pull/116457)
* [fix to register analysis passes with -Zllvm-plugins at link-time](https://github.com/rust-lang/rust/pull/116486)
* [for a single impl candidate, try to unify it with error trait ref](https://github.com/rust-lang/rust/pull/115726)
* [generalize small dominators optimization](https://github.com/rust-lang/rust/pull/116454)
* [improve the suggestion of `generic_bound_failure`](https://github.com/rust-lang/rust/pull/115882)
* [make FnDef 1-ZST in LLVM debuginfo](https://github.com/rust-lang/rust/pull/116096)
* [more accurately point to where default return type should go](https://github.com/rust-lang/rust/pull/116296)
* [move subtyper below `reveal_all` and change `reveal_all`](https://github.com/rust-lang/rust/pull/116415)
* [only trigger `refining_impl_trait` lint on reachable traits](https://github.com/rust-lang/rust/pull/116273)
* [point to full `async fn` for future](https://github.com/rust-lang/rust/pull/116360)
* [print normalized ty](https://github.com/rust-lang/rust/pull/116374)
* [properly export function defined in test which uses `global_asm!()`](https://github.com/rust-lang/rust/pull/116458)
* [remove Key impls for types that involve an AllocId](https://github.com/rust-lang/rust/pull/116330)
* [remove is global hack](https://github.com/rust-lang/rust/pull/116417)
* [remove the `TypedArena::alloc_from_iter` specialization](https://github.com/rust-lang/rust/pull/116370)
* [show more information when multiple `impl`s apply](https://github.com/rust-lang/rust/pull/114811)
* [suggest `pin!()` instead of `Pin::new()` when appropriate](https://github.com/rust-lang/rust/pull/114654)
* [make subtyping explicit in MIR](https://github.com/rust-lang/rust/pull/115025)
* [do not run optimizations on trivial MIR](https://github.com/rust-lang/rust/pull/116533)
* [in smir `find_crates` returns `Vec<Crate>` instead of `Option<Crate>`](https://github.com/rust-lang/rust/pull/116543)
* [add Span to various smir types](https://github.com/rust-lang/rust/pull/116288)
* [miri-script: print which sysroot target we are building](https://github.com/rust-lang/miri/pull/3105)
* [miri: auto-detect `no_std` where possible](https://github.com/rust-lang/miri/pull/3000)
* [miri: continuation of #3054: enable spurious reads in TB](https://github.com/rust-lang/miri/pull/3067)
* [miri: do not use host floats in `simd_{ceil,floor,round,trunc}`](https://github.com/rust-lang/miri/pull/3110)
* [miri: ensure RET assignments do not get propagated on unwinding](https://github.com/rust-lang/miri/pull/3114)
* [miri: implement `llvm.x86.aesni.*` intrinsics](https://github.com/rust-lang/miri/pull/3101)
* [miri: refactor dlsym: dispatch symbols via the normal shim mechanism](https://github.com/rust-lang/miri/pull/3108)
* [miri: support getentropy on macOS as a foreign item](https://github.com/rust-lang/miri/pull/3098)
* [miri: tree Borrows: do not create new tags as 'Active'](https://github.com/rust-lang/miri/pull/3106)
* [add missing inline attributes to Duration trait impls](https://github.com/rust-lang/rust/pull/116386)
* [stabilize `Option::as_`(`mut_`)`slice`](https://github.com/rust-lang/rust/pull/116220)
* [reuse existing `Some`s in `Option::(x)or`](https://github.com/rust-lang/rust/pull/116481)
* [fix generic bound of `str::SplitInclusive`'s `DoubleEndedIterator` impl](https://github.com/rust-lang/rust/pull/100806)
* [cargo: `refactor(toml)`: Make manifest file layout more consitent](https://github.com/rust-lang/cargo/pull/12768)
* [cargo: add new package cache lock modes](https://github.com/rust-lang/cargo/pull/12706)
* [cargo: add unsupported short suggestion for --out-dir flag](https://github.com/rust-lang/cargo/pull/12755)
* [cargo: crates-io: add doc comment for `NewCrate struct`](https://github.com/rust-lang/cargo/pull/12782)
* [cargo: feat: add `Edition2024`](https://github.com/rust-lang/cargo/pull/12771)
* [cargo: prep for automating MSRV management](https://github.com/rust-lang/cargo/pull/12767)
* [cargo: set and verify all MSRVs in CI](https://github.com/rust-lang/cargo/pull/12654)
* [rustdoc-search: fix bug with multi-item impl trait](https://github.com/rust-lang/rust/pull/116475)
* [rustdoc: rename `issue-\d+.rs` tests to have meaningful names (part 2)](https://github.com/rust-lang/rust/pull/116432)
* [rustdoc: Show `enum` discrimant if it is a C-like variant](https://github.com/rust-lang/rust/pull/116142)
* [rustfmt: adjust span derivation for const generics](https://github.com/rust-lang/rustfmt/pull/5936)
* [clippy: `impl_trait_in_params` now supports impls and traits](https://github.com/rust-lang/rust-clippy/pull/11550)
* [clippy: `into_iter_without_iter`: walk up deref impl chain to find `iter` methods](https://github.com/rust-lang/rust-clippy/pull/11639)
* [clippy: `std_instead_of_core`: avoid lint inside of proc-macro](https://github.com/rust-lang/rust-clippy/pull/11589)
* [clippy: avoid invoking `ignored_unit_patterns` in macro definition](https://github.com/rust-lang/rust-clippy/pull/11602)
* [clippy: fix `items_after_test_module` for non root modules, add applicable suggestion](https://github.com/rust-lang/rust-clippy/pull/11611)
* [clippy: fix ICE in `redundant_locals`](https://github.com/rust-lang/rust-clippy/pull/11623)
* [clippy: fix: avoid changing drop order](https://github.com/rust-lang/rust-clippy/pull/11603)
* [clippy: improve `redundant_locals` help message](https://github.com/rust-lang/rust-clippy/pull/11628)
* [rust-analyzer: add config option to use `rust-analyzer` specific target dir](https://github.com/rust-lang/rust-analyzer/pull/15681)
* [rust-analyzer: add configuration for the default action of the status bar click action in VSCode](https://github.com/rust-lang/rust-analyzer/pull/15707)
* [rust-analyzer: do flyimport completions by prefix search for short paths](https://github.com/rust-lang/rust-analyzer/pull/15713)
* [rust-analyzer: add assist for applying De Morgan's law to `Iterator::all` and `Iterator::any`](https://github.com/rust-lang/rust-analyzer/pull/15700)
* [rust-analyzer: add backtick to surrounding and auto-closing pairs](https://github.com/rust-lang/rust-analyzer/pull/15668)
* [rust-analyzer: implement tuple return type to tuple `struct` assist](https://github.com/rust-lang/rust-analyzer/pull/15696)
* [rust-analyzer: ensure `rustfmt` runs when configured with `./`](https://github.com/rust-lang/rust-analyzer/pull/15600)
* [rust-analyzer: fix path syntax produced by the `into_to_qualified_from` assist](https://github.com/rust-lang/rust-analyzer/pull/15641)
* [rust-analyzer: recognize custom main function as binary entrypoint for runnables](https://github.com/rust-lang/rust-analyzer/pull/15709)

### Rust Compiler Performance Triage

A quiet week, with few regressions and improvements.

Triage done by **@simulacrum**.
Revision range: [9998f4add..84d44dd](https://perf.rust-lang.org/?start=9998f4add08c3d09c82e00975cf3a293b30160ec&end=84d44dd1d8ec1e98fff94272ba4f96b2a1f044ca&absolute=false&stat=instructions%3Au)

1 Regressions, 2 Improvements, 4 Mixed; 1 of them in rollups

68 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-10-10.md)

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

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

<!-- New or updated RFCs go here, use this format: * [new|updated] [Topic](URL) -->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- * [new|updated] []() -->

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
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

## Upcoming Events

Rusty Events between 2023-10-11 - 2023-11-08 🦀

### Virtual

* 2023-10-04 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcnbgb/)
* 2023-10-04 | Virtual (Various) | [Ferrous Systems](https://www.eventbrite.com/o/ferrous-systems-gmbh-68735392123)
    * [**A Decade of Rust with Ferrous Systems**](https://www.eventbrite.com/e/a-decade-of-rust-with-ferrous-systems-tickets-680492891557?aff=ebdssbdestsearch)
* 2023-10-04 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Understanding the Processor (Atomics & Locks Chapter 7)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296278202/)
* 2023-10-05 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296135640/)
* 2023-10-07 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup: Mentorship (First Saturday)**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763617907?aff=erelpanelorg)
* 2023-10-10 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679773/) | [**Mirror**](https://berline.rs/)
* 2023-10-10 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/fvdtgtyfcnbnb/)
* 2023-10-11| Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsryfcnbpb/)
* 2023-10-12 - 2023-10-13 | Virtual (Brussels, BE) | [EuroRust](https://eurorust.eu)
    * [**EuroRust 2023**](https://eurorust.eu)
* 2023-10-12 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732662/)
* 2023-10-18 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/295057159/)
* 2023-10-19 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/ngnwftyfcnbzb/)
* 2023-10-19 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcnbgb/)
* 2023-10-24 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679778/) | [**Mirror**](https://berline.rs/)
* 2023-10-31 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtyfcnbpc/)

### Asia

* 2023-10-11 | Kuala Lumpur, MY | [GoLang Malaysia](https://t.me/golangmalaysia)
    * [**Rust Meetup Malaysia October 2023**](https://forms.gle/wwJAEipFgwQtEfJB9) | [Event updates Telegram](https://t.me/+dF46Fly4A_BjOTJl) | [Event group chat](https://t.me/golangmalaysia)

### Europe

* 2023-10-04 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #6**](https://www.meetup.com/fr-FR/rust-lyon/events/296186641/)
* 2023-10-10 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679773/)
* 2023-10-11 | Brussels, BE | [BeCode Brussels Meetup](https://www.eventbrite.be/e/becode-brussels-meetup-rust-on-web-tickets-728375238947)
    * [**Rust on Web - EuroRust Conference**](https://rust-on-web.glitch.me/)
* 2023-10-12 - 2023-10-13 | Brussels, BE | [EuroRust](https://eurorust.eu)
    * [**EuroRust 2023**](https://eurorust.eu)
* 2023-10-12 | Brussels, BE | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Rust Aarhus - EuroRust Conference**](https://www.meetup.com/rust-aarhus/events/295673220/)
* 2023-10-12 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/295955356/)
* 2023-10-17 | Helsinki, FI | [Finland Rust-lang Group](https://www.meetup.com/finland-rust-meetup/)
    * [**Helsinki Rustaceans Meetup**](https://www.meetup.com/finland-rust-meetup/events/295680333/)
* 2023-10-17 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**SIMD in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504251/)
* 2023-10-19 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Amsterdam Meetup @ Terraform**](https://www.meetup.com/rust-amsterdam-group/events/296495570/)
* 2023-10-19 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Meetup #35**](https://www.meetup.com/rust-wroclaw/events/296507983/)
* 2023-10-25 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin/)
    * [**Biome, web development tooling with Rust**](https://www.meetup.com/rust-dublin/events/295179534/)
* 2023-10-26 | Augsburg, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Augsburg Rust Meetup #3**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/296183126/)

### North America

* 2023-10-05 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/296369949/)
* 2023-10-09 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/296346749/)
* 2023-10-09 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/296497475/)
* 2023-10-11 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**First Meetup - Demo Day and Office Hours**](https://www.meetup.com/boulder-rust-meetup/events/296193722/)
* 2023-10-12 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**The Actor Model: Fearless Concurrency, Made Easy w/Chris Mena**](https://www.meetup.com/utah-rust/events/295771376/)
* 2023-10-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfcnbwb/)
* 2023-10-18 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston University Rust Lunch**](https://www.meetup.com/bostonrust/events/296223807/)
* 2023-10-19 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/296369976/)
* 2023-10-19 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Rust Goes Where It Pleases Pt2 - Rust on the front end!**](https://www.meetup.com/music-city-rust-developers/events/296254420/)
* 2023-10-19 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group - October Meetup**](https://www.meetup.com/seattle-rust-user-group/events/296110729)
* 2023-10-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/296495790)

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

> The Rust mission -- let you write software that's fast and correct, productively -- has never been more alive. So next Rustconf, I plan to celebrate:
>
> * All the buffer overflows I didn't create, thanks to Rust
> * All the unit tests I didn't have to write, thanks to its type system
> * All the null checks I didn't have to write thanks to Option and Result
> * All the JS I didn't have to write thanks to WebAssembly
> * All the impossible states I didn't have to assert "This can never actually happen"
> * All the JSON field keys I didn't have to manually type in thanks to Serde
> * All the missing SQL column bugs I caught at compiletime thanks to Diesel
> * All the race conditions I never had to worry about thanks to the borrow checker
> * All the connections I can accept concurrently thanks to Tokio
> * All the formatting comments I didn't have to leave on PRs thanks to Rustfmt
> * All the performance footguns I didn't create thanks to Clippy

– [Adam Chalmers in their RustConf 2023 recap](https://blog.adamchalmers.com/rustconf-2023-recap/)

Thanks to [robin](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1469) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
