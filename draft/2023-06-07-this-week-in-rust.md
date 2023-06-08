Title: This Week in Rust 498
Number: 498
Date: 2023-06-07
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
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
* [Announcing Rust 1.70.0](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html)

### Foundation

### Newsletters
* [This Month in Rust GameDev #45 - April 2023](https://gamedev.rs/news/045/)
* [This Month in Rust OSDev: May 2023](https://rust-osdev.com/this-month/2023-05/)

### Project/Tooling Updates
* [Changelog #184](https://rust-analyzer.github.io/thisweek/2023/06/05/changelog-184.html)

* [MsgPacker: Enhancing performance and security](https://vlopes11.github.io/msgpacker-refactor/)

* [Diesel 2.1: Generated Migrations and simplified MultiBackend support](https://diesel.rs/news/2_1_0_release.html)

* [Quickwit 0.6: Elasticsearch compatible API, range & prefix phrase queries, histogram & percentiles aggregations, and more...!](https://quickwit.io/blog/quickwit-0.6)  

### Observations/Thoughts
* [A Proposal for an asynchronous Rust GUI framework](https://notgull.github.io/async-gui/)
* [A locking war story](https://sentry.engineering/blog/locking-war-story)
* [Building a Vector Database to Make Use of Vector Embeddings](https://terminusdb.com/blog/vector-database-and-vector-embeddings/)
* [Data Exfiltration through DNS with Rust](https://balwurk.com/data-exfiltration-through-dns-with-rust/)
* [The Rust I Wanted Had No Future](https://graydon2.dreamwidth.org/307291.html)
* [From Stacks to Trees: A new aliasing model for Rust](https://www.ralfj.de/blog/2023/06/02/tree-borrows.html)
* [ESP32 Embedded Rust at the HAL: GPIO Interrupts](https://apollolabsblog.hashnode.dev/esp32-embedded-rust-at-the-hal-gpio-interrupts?ref=twitter-share)
* [Introducing posh: Type-Safe Graphics Programming with Functional Shaders in Rust](https://leod.github.io/rust/gamedev/posh/2023/06/04/posh.html)
* [Demystifying trait generics in Rust](https://gruebelinchen.wordpress.com/2023/06/06/demystifying-trait-generics-in-rust/)

### Rust Walkthroughs
* [Writing universal libraries using C++ and consuming it in Rust (WASI)](https://medium.com/@shyamsundarb/writing-universal-libraries-using-c-and-consuming-it-in-rust-wasi-80ad1174e0c9)
* [Rust's cfg Attribute](https://blog.parker-codes.dev/posts/rusts-cfg-attribute)
* [Looking at Rust builtin derives](https://cohenarthur.github.io/2023/06/05/rust-derives.html)
* [What is WASI?](https://blog.yoshuawuyts.com/what-is-wasi/)

### Research

### Miscellaneous
* [audio] [Shuttle Launchpad with Stefan Baumgartner :: Rustacean Station](https://rustacean-station.org/episode/stefan-baumgartner/)
* [audio] [Episode 5: Putting Lipstick on a Pig](https://wayofthecrab.com/episodes/005/)
* [video] [Bevy Basics Timers](https://www.youtube.com/watch?v=jC7QDlNxIg8)
* [video] [We Built a Blazingly Fast Video Conferencing System in Rust & WASM](https://www.youtube.com/watch?v=LWwOSZJwEJI)
* [video] [Why Static Typing Came Back • Richard Feldman • GOTO 2022](https://www.youtube.com/watch?v=Tml94je2edk)

## Crate of the Week

This week's crate is [kanata](https://github.com/jtroo/kanata), a keyboard remapper for Linux and Windows.

Thanks to [Aleksey Kladov](https://users.rust-lang.org/t/crate-of-the-week/2704/1203) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->
* [Hyperswitch - Remove redundant heap allocation (specifically string construction) in the application](https://github.com/juspay/hyperswitch/issues/1294)
* [Hyperswitch - add domain type for client secret](https://github.com/juspay/hyperswitch/issues/1357)
* [Hyperswitch - Implement cache for `MerchantKeyStore`](https://github.com/juspay/hyperswitch/issues/1381)
* [Ockam - Add `--yes` flag to `delete` commands to prevent unintentional deletions](https://github.com/build-trust/ockam/issues/5041)
* [Ockam - Define json output for `ockam node create`](https://github.com/build-trust/ockam/issues/4967)
* [Ockam - unreported `project path not found` error](https://github.com/build-trust/ockam/issues/4868)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

392 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-05-29..2023-06-05

* [support 128-bit atomics on all `x86_64` Apple targets](https://github.com/rust-lang/rust/pull/112150)
* [support the rustc metadata for AIX](https://github.com/rust-lang/rust/pull/107583)
* [QNX Neutrino: exponential backoff when fork/spawn needs a retry](https://github.com/rust-lang/rust/pull/109432)
* [Control Flow Integrity: fix with `repr(transparent): transform_ty`: unexpected Alias(Proj](https://github.com/rust-lang/rust/pull/112182)
* [Control Flow Integrity: fix async: `transform_ty`: unexpected GeneratorWitness(Bi…](https://github.com/rust-lang/rust/pull/111914)
* [add other workspaces to `linkedProjects` in `rust_analyzer_settings`](https://github.com/rust-lang/rust/pull/111998)
* [allow limited access to `OsStr` bytes](https://github.com/rust-lang/rust/pull/109698)
* [check nested obligations during coercion unify in new solver](https://github.com/rust-lang/rust/pull/112022)
* [diagnostics: exclude indirect private deps from trait impl suggest](https://github.com/rust-lang/rust/pull/111076)
* [don't ICE in new solver when auto traits have associated types](https://github.com/rust-lang/rust/pull/112223)
* [don't compute inlining status of mono items in advance](https://github.com/rust-lang/rust/pull/112128)
* [don't require the output from libtest to be valid UTF-8](https://github.com/rust-lang/rust/pull/112277)
* [don't suggest break through nested items](https://github.com/rust-lang/rust/pull/112024)
* [don't typecheck recovered method call from suggestion](https://github.com/rust-lang/rust/pull/112100)
* [enable `ConstGoto` and `SeparateConstSwitch` passes by default](https://github.com/rust-lang/rust/pull/112040)
* [enable `ScalarReplacementOfAggregates` in optimized builds](https://github.com/rust-lang/rust/pull/112002)
* [extra context for `unreachable_pub` lint](https://github.com/rust-lang/rust/pull/111496)
* [fix bug where private item with intermediate doc hidden re-export was not inlined](https://github.com/rust-lang/rust/pull/112178)
* [fix codegen test suite for bare-metal-like targets](https://github.com/rust-lang/rust/pull/111878)
* [fix linkage for large binaries on mips64 platforms](https://github.com/rust-lang/rust/pull/111772)
* [fix re-export of doc hidden item inside private item not displayed](https://github.com/rust-lang/rust/pull/112108)
* [fix type-inference regression](https://github.com/rust-lang/rust/pull/112266)
* [fix: dedup `static_candidates` before report](https://github.com/rust-lang/rust/pull/111872)
* [improve CGU debug printing](https://github.com/rust-lang/rust/pull/112155)
* [linker: report linker flavors incompatible with the current target](https://github.com/rust-lang/rust/pull/110807)
* [normalize anon consts in new solver](https://github.com/rust-lang/rust/pull/112183)
* [only check inlining counter after recursing](https://github.com/rust-lang/rust/pull/112240)
* [only rewrite valtree-constants to patterns and keep other constants opaque](https://github.com/rust-lang/rust/pull/111913)
* [only suppress coercion error if type is definitely unsized](https://github.com/rust-lang/rust/pull/112215)
* [optimize scalar and scalar pair representations loaded from ByRef in llvm](https://github.com/rust-lang/rust/pull/111768)
* [preserve substs in opaques recorded in typeck results](https://github.com/rust-lang/rust/pull/111980)
* [refactor and cleanup the leak check, add it to new solver](https://github.com/rust-lang/rust/pull/111881)
* [remove unneeded `Buffer` allocations when `&mut fmt::Write` can be used directly](https://github.com/rust-lang/rust/pull/112243)
* [replace const eval limit by a lint and add an exponential backoff warning](https://github.com/rust-lang/rust/pull/103877)
* [require that const param tys implement `ConstParamTy`](https://github.com/rust-lang/rust/pull/111670)
* [rpath is not supported on AIX](https://github.com/rust-lang/rust/pull/109525)
* [rust-lld: add rpath entry to the correct `lib` folder](https://github.com/rust-lang/rust/pull/112247)
* [show note for type ascription on a local binding interpreted as a constant pattern and not a new variable](https://github.com/rust-lang/rust/pull/112272)
* [stop normalizing so many different prefixes](https://github.com/rust-lang/rust/pull/111975)
* [suggest `Option::as_deref(_mut)` on type mismatch in option combinator if it passes typeck](https://github.com/rust-lang/rust/pull/111659)
* [suggest correct `self_ty`](https://github.com/rust-lang/rust/pull/112057)
* [uplift `clippy::cast_ref_to_mut` lint](https://github.com/rust-lang/rust/pull/111567)
* [uplift `clippy::invalid_utf8_in_unchecked` lint](https://github.com/rust-lang/rust/pull/111543)
* [lower `unchecked_div`/`_rem` to MIR's `BinOp::Div`/`Rem`](https://github.com/rust-lang/rust/pull/112168)
* [miri: Tree Borrows (TB) diagnostics: avoid printing irrelevant events](https://github.com/rust-lang/miri/pull/2888)
* [miri: TB: improve error messages (distinguish between accesses and reborrows)](https://github.com/rust-lang/miri/pull/2918)
* [miri: remove rustc-workspace-hack](https://github.com/rust-lang/miri/pull/2916)
* [greatly decrease the size of `rustc_driver.so` when debuginfo is enabled](https://github.com/rust-lang/rust/pull/110221)
* [remove `ExtendElement`, `ExtendWith`, `extend_with`](https://github.com/rust-lang/rust/pull/112263)
* [remove `[T]::zip(_)`](https://github.com/rust-lang/rust/pull/112096)
* [make `TrustedStep` require `Copy`](https://github.com/rust-lang/rust/pull/112083)
* [`offset_of!`: don't require type to be `Sized`](https://github.com/rust-lang/rust/pull/112069)
* [check tuple elements are `Sized` in `offset_of!`](https://github.com/rust-lang/rust/pull/112193)
* [fix bug in `utf16_to_utf8` for zero length strings](https://github.com/rust-lang/rust/pull/112154)
* [hashbrown: add support for rkyv serialization and deserialization](https://github.com/rust-lang/hashbrown/pull/432)
* [regex compile: make `Regex::new(r"(?-u:\B)")` fail again](https://github.com/rust-lang/regex/pull/1007)
* [cargo: add message on reusing previous temporary path on failed cargo installs](https://github.com/rust-lang/cargo/pull/12231)
* [cargo: emit error when users try to use a toolchain via the `add` or `install` command](https://github.com/rust-lang/cargo/pull/12226)
* [cargo: support "default" option for `build.jobs`](https://github.com/rust-lang/cargo/pull/12222)
* [rustdoc: add interaction delays for tooltip popovers](https://github.com/rust-lang/rust/pull/111892)
* [rustdoc: render visibility on associated types](https://github.com/rust-lang/rust/pull/110945)
* [clippy: `allow_attributes`, `allow_attributes_without_reason`: Ignore attributes from procedural macros](https://github.com/rust-lang/rust-clippy/pull/10869)
* [clippy: `manual_let_else`: support `struct` patterns](https://github.com/rust-lang/rust-clippy/pull/10866)
* [clippy: `nonminimal_bool` fix double not](https://github.com/rust-lang/rust-clippy/pull/10845)
* [clippy: `ptr_cast_constness`: Only lint on casts which don't change type](https://github.com/rust-lang/rust-clippy/pull/10879)
* [clippy: `unnecessary_lazy_eval`: don't lint on types with deref impl](https://github.com/rust-lang/rust-clippy/pull/10864)
* [clippy: `useless_conversion`: pluralize if there are multiple `.into_iter()` calls](https://github.com/rust-lang/rust-clippy/pull/10881)
* [clippy: `wildcard_imports` Modules that contain `prelude` are also allowed](https://github.com/rust-lang/rust-clippy/pull/10848)
* [clippy: add a test that checks for old style test headers](https://github.com/rust-lang/rust-clippy/pull/10705)
* [clippy: add checking for `cfg(features = ...)`](https://github.com/rust-lang/rust-clippy/pull/10860)
* [clippy: add lints for disallowing usage of `to_xx_bytes` and `from_xx_bytes`](https://github.com/rust-lang/rust-clippy/pull/10826)
* [clippy: add spans to `clippy.toml` error messages](https://github.com/rust-lang/rust-clippy/pull/10607)
* [clippy: emit `unnecessary_cast` on raw pointers as well](https://github.com/rust-lang/rust-clippy/pull/10821)
* [clippy: fix suggestion on fully qualified syntax](https://github.com/rust-lang/rust-clippy/pull/10855)
* [clippy: ignore fix for `from_over_into` if the target type contains a `Self` reference](https://github.com/rust-lang/rust-clippy/pull/10853)
* [clippy: move `redundant_clone` to `nursery`](https://github.com/rust-lang/rust-clippy/pull/10873)
* [clippy: new lint: `explicit_into_iter_fn_arg`](https://github.com/rust-lang/rust-clippy/pull/10814)
* [clippy: new lint: `missing_fields_in_debug`](https://github.com/rust-lang/rust-clippy/pull/10616)
* [rust-analyzer: add mandatory panic contexts to all threadpool tasks](https://github.com/rust-lang/rust-analyzer/pull/14965)
* [rust-analyzer: allow setting cfgs](https://github.com/rust-lang/rust-analyzer/pull/14911)
* [rust-analyzer: don't add --all-targets to runnables for no-std crates](https://github.com/rust-lang/rust-analyzer/pull/14912)
* [rust-analyzer: add signature help for tuple patterns and expressions](https://github.com/rust-lang/rust-analyzer/pull/14938)
* [rust-analyzer: render niches on hover](https://github.com/rust-lang/rust-analyzer/pull/14905)
* [rust-analyzer: fix Assist "replace named generic type with impl trait"](https://github.com/rust-lang/rust-analyzer/pull/14945)
* [rust-analyzer: fix `unused-mut` false positive for `Box`](https://github.com/rust-lang/rust-analyzer/pull/14972)
* [rust-analyzer: fix bug in labeled for loop desugaring](https://github.com/rust-lang/rust-analyzer/pull/14942)
* [rust-analyzer: fix drop scopes problems in mir](https://github.com/rust-lang/rust-analyzer/pull/14961)
* [rust-analyzer: fix edits for `convert_named_struct_to_tuple_struct`](https://github.com/rust-lang/rust-analyzer/pull/14920)
* [rust-analyzer: fix missing terminator for slice pattern](https://github.com/rust-lang/rust-analyzer/pull/14976)
* [rust-analyzer: fix string pattern matching in mir interpreter](https://github.com/rust-lang/rust-analyzer/pull/14951)
* [rust-analyzer: fix: add enum, reference, array and slice to `render_const_scalar`](https://github.com/rust-lang/rust-analyzer/pull/14947)
* [rust-analyzer: fix: add render configs for memory layout hovers](https://github.com/rust-lang/rust-analyzer/pull/14929)
* [rust-analyzer: fix: consider outer binders when folding captured items' type](https://github.com/rust-lang/rust-analyzer/pull/14971)
* [rust-analyzer: fix: detect "bound more than once" error and suppress `need-mut` for it](https://github.com/rust-lang/rust-analyzer/pull/14970)
* [rust-analyzer: fix: don't duplicate sysroot crates in rustc workspace](https://github.com/rust-lang/rust-analyzer/pull/14935)
* [rust-analyzer: fix: emit `'_` for lifetime generics in `HirDisplay`](https://github.com/rust-lang/rust-analyzer/pull/14978)
* [rust-analyzer: fix nav target calculation discarding file ids from differing macro upmapping](https://github.com/rust-lang/rust-analyzer/pull/14939)
* [rust-analyzer: make assignment operators right associative](https://github.com/rust-lang/rust-analyzer/pull/14952)
* [rust-analyzer: prioritize threads affected by user typing](https://github.com/rust-lang/rust-analyzer/pull/14888)
* [rust-analyzer: support floating point intrinsics in const eval](https://github.com/rust-lang/rust-analyzer/pull/14950)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

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

Rusty Events between 2023-06-07 - 2023-07-05 🦀

### Virtual

* 2023-06-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/293309294)
* 2023-06-07 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/293616568)
* 2023-06-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732649)
* 2023-06-13 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/293014938)
* 2023-06-14 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Building Spin Locks and Channels - Rust Atomics & Locks Bookclub Chapters 4 & 5**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/293882628/)
* 2023-06-14 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/293014897)
* 2023-06-15 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/293786806)
* 2023-06-19 | Virtual (San Francisco, CA, US) | [Data Science on AWS - San Francisco, Global](https://www.meetup.com/data-science-on-aws/)
    * [**Generative AI Parameter Efficient Fine Tuning (PEFT), RLHF + Polars: "Polars, lightning-fast DataFrame library for Rust and Python", presented by Suman Debnath**](https://www.meetup.com/data-science-on-aws/events/289912375)
* 2023-06-20 | Virtual (Berlin, DE) | [Berline.rs / OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/293485510)
* 2023-06-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/jkxsctyfcjbbc/)
* 2023-06-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763484)
* 2023-06-22 | Virtual (Karlsruhe, DE) | [Karlsruhe Functional Programmers Group](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/)
    * [**Stammtisch (gemeinsam mit der C++ UG KA)**](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/events/293937801)
* 2023-06-25 | Virtual (Auckland, NZ) | [ResBaz Aotearoa 2023](https://resbaz.auckland.ac.nz/)
    * [**Research Computing With The Rust Programming Language - Tim McNamara**](https://www.eventbrite.com/e/research-computing-with-the-rust-programming-language-tickets-637648623197?aff=ebdssbdestsearch)
* 2023-06-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfcjbkc/)
* 2023-06-28 | Virtual (Chicago, IL, US) | [Chicago Healthcare Cloud Technology Community](https://www.meetup.com/chicago-healthcare-tech-and-ai/)
    * [**Rust for Mission-Critical AI: A Journey into Healthcare's Safest Language**](https://www.meetup.com/chicago-healthcare-tech-and-ai/events/293278396)
* 2023-07-04 | Virtual (Berlin, DE) | [Berline.rs / OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/zdrzpsyfckbgb/)
* 2023-07-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfckbgb/)
* 2023-07-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/293309295)

### Asia

* 2023-06-10 | Kuala Lumpur, MY | [GoLang Malaysia](https://t.me/golangmalaysia)
    * [**Rust Workshop/Hack and Learn Malaysia June 2023**](https://forms.gle/2fvbCG77HXCkWLfe6) | [Event updates Telegram](https://t.me/+dF46Fly4A_BjOTJl) | [Event group chat](https://t.me/golangmalaysia)
* 2023-06-10 | Pune, IN | [Rust Pune](https://www.meetup.com/rust-pune)
    * [**#1 - Meet & Greet**](https://www.meetup.com/rust-pune/events/293936676/)

### Europe

* 2023-06-08 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust Aarhus meetup #2 sponsored by BRØLSTÆRK**](https://www.meetup.com/rust-aarhus/events/292865970/)
* 2023-06-08 | Paris, FR | [Stockly.ai](https://www.eventbrite.fr/o/stockly-42274765293)
    * [**Rust Meetup in Paris - hosted by Stockly**](https://www.eventbrite.fr/e/rust-meetup-in-paris-hosted-by-stockly-tickets-630742055467)
* 2023-06-08 | Zurich, CH | [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**Unsafe, Miri, SIMD - June Meetup**](https://www.meetup.com/de-DE/rust-zurich/events/293322792/)
* 2023-06-16 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/pdhvctyfcjbvb/)
* 2023-06-22 | Vienna, AT | [Papers We Love Vienna](https://www.meetup.com/papers-we-love-vienna/)
    * [**June: Data and Ownership in Rust**](https://www.meetup.com/papers-we-love-vienna/events/293974147)
* 2023-06-28 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake**](https://www.meetup.com/bratislava-rust-meetup-group/events/293732916)
* 2023-07-03 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**Rust in the Linux Kernel - July Meetup**](https://www.meetup.com/rust-zurich/events/293322905)

### North America

* 2023-06-07 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/293730065)
* 2023-06-08 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Rust 1.70.0, Module System Deep Dive & Pizza**](https://www.meetup.com/utah-rust/events/293849386/)
* 2023-06-08 | Pasadena, CA, US | [Pasadena Thursday Go/Rust](https://www.meetup.com/thursday-go/)
    * [**Weekly leetcode group**](https://www.meetup.com/thursday-go/events/293927644)
* 2023-06-10 | San Jose, CA, US | [Rust Breakfast & Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/jnmgftyfcjbnb/)
* 2023-06-15 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/294032616)
* 2023-06-17 | San Jose, CA, US | [Rust Breakfast & Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/293825860)
* 2023-06-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/293118809)
* 2023-06-24 | San Jose, CA, US | [Rust Breakfast & Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/jnmgftyfcjbgc/)
* 2023-07-01 | San Jose, CA, US | [Rust Breakfast & Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/jnmgftyfckbcb/)

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

> (...) Rust developers usually are not just looking for "less buggy". 
>
> They are addicted to the clicky sound of legos.

– [Amirography on fosstodon](https://fosstodon.org/@Amirography/110486392650489999)

Thanks to [Jan Riemer](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1434) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
