Title: This Week in Rust 548
Number: 548
Date: 2024-05-22
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

### Foundation

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [ralertsinua](https://github.com/voiceapiai/ralertsinua), a text user interface for getting information about Russian air raids in Ukraine.

Thanks to [Vladyslav Batyrenko](https://users.rust-lang.org/t/crate-of-the-week/2704/1311) for the suggestion!

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

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](link to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

364 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-05-14..2024-05-21

* [add `x86_64-unknown-linux-none` target](https://github.com/rust-lang/rust/pull/125023)
* [enable `rust-lld` on nightly `x86_64-unknown-linux-gnu`](https://github.com/rust-lang/rust/pull/124129)
* [`rustc_hir_typeck`: Account for `skipped_ref_pats` in `expr_use_visitor`](https://github.com/rust-lang/rust/pull/125084)
* [`rustc_resolve` cleanups](https://github.com/rust-lang/rust/pull/125105)
* [actually use the `#[do_not_recommend]` attribute if present](https://github.com/rust-lang/rust/pull/124708)
* [add `#[inline]` to float `Debug` fallback used by `cfg(no_fp_fmt_parse)`](https://github.com/rust-lang/rust/pull/125252)
* [add `fn into_raw_with_allocator` to Rc/Arc/Weak](https://github.com/rust-lang/rust/pull/125093)
* [add an experimental feature gate for global registration](https://github.com/rust-lang/rust/pull/125314)
* [add and use `generics.is_empty()` and `generics.is_own_empty,` rather than using generics' attributes](https://github.com/rust-lang/rust/pull/125006)
* [also expand weak alias tys inside consts inside `expand_weak_alias_tys`](https://github.com/rust-lang/rust/pull/124990)
* [android: use `posix_memalign` for aligned allocations](https://github.com/rust-lang/rust/pull/125244)
* [avoid using `aligned_alloc`; `posix_memalign` is better-behaved](https://github.com/rust-lang/rust/pull/125003)
* [check whether the `next_node` is else-less if in `get_return_block`](https://github.com/rust-lang/rust/pull/124917)
* [coverage: `CoverageIdsInfo::mcdc_bitmap_bytes` is never needed](https://github.com/rust-lang/rust/pull/125108)
* [coverage: memoize and simplify counter expressions](https://github.com/rust-lang/rust/pull/125106)
* [defrost `RUST_MIN_STACK=ice rustc hello.rs`](https://github.com/rust-lang/rust/pull/125302)
* [delegation: implement list delegation](https://github.com/rust-lang/rust/pull/123413)
* [don't ICE because recomputing overflow goals during `find_best_leaf_obligation` causes inference side-effects](https://github.com/rust-lang/rust/pull/124871)
* [don't do post-method-probe error reporting steps if we're in a suggestion](https://github.com/rust-lang/rust/pull/125100)
* [fix ICE in non-operand `aggregate_raw_ptr` intrinsic codegen](https://github.com/rust-lang/rust/pull/125184)
* [fix println! ICE when parsing percent prefix number](https://github.com/rust-lang/rust/pull/125004)
* [fix suggestion in E0373 for `!Unpin` coroutines](https://github.com/rust-lang/rust/pull/125301)
* [fix the dedup error because of spans from suggestion](https://github.com/rust-lang/rust/pull/125135)
* [improve error message: missing `;` in `macro_rules`](https://github.com/rust-lang/rust/pull/125180)
* [include line tables in compiler profile](https://github.com/rust-lang/rust/pull/123337)
* [lldb-formatters: use StdSliceSyntheticProvider for &str](https://github.com/rust-lang/rust/pull/124500)
* [make `EvalCtxt` generic over `InferCtxtLike`](https://github.com/rust-lang/rust/pull/125255)
* [make crashes dump mir to build dir](https://github.com/rust-lang/rust/pull/125032)
* [never type unsafe lint improvements](https://github.com/rust-lang/rust/pull/125282)
* [note for E0599 if shadowed bindings has the method](https://github.com/rust-lang/rust/pull/124283)
* [only find segs chain for missing methods when no available candidates](https://github.com/rust-lang/rust/pull/125080)
* [only make GAT ambiguous in `match_projection_projections` considering shallow resolvability](https://github.com/rust-lang/rust/pull/125214)
* [refactor: add rustc-perf submodule to src/tools](https://github.com/rust-lang/rust/pull/125166)
* [report better WF obligation leaf obligations in new solver](https://github.com/rust-lang/rust/pull/125191)
* [style-guide: when breaking binops handle multi-line first operand better](https://github.com/rust-lang/rust/pull/119838)
* [suggest setting lifetime in borrowck error involving types with elided lifetimes](https://github.com/rust-lang/rust/pull/124682)
* [temporarily revert to NonZeroUsize in rustc-abi to fix building on stable](https://github.com/rust-lang/rust/pull/125240)
* [track cycle participants per root](https://github.com/rust-lang/rust/pull/125308)
* [use a proper probe for shadowing impl](https://github.com/rust-lang/rust/pull/124844)
* [use a single static for all default slice Arcs](https://github.com/rust-lang/rust/pull/125283)
* [warn against changes in opaque lifetime captures in 2024](https://github.com/rust-lang/rust/pull/124228)
* [miri: adjust Allocation Bytes used by Miri to custom MiriAllocBytes](https://github.com/rust-lang/miri/pull/3526)
* [miri: directly implement native exception raise methods in miri](https://github.com/rust-lang/miri/pull/3319)
* [miri: give `FileDescription::{read, write}` access to the `MiriInterpCx `](https://github.com/rust-lang/miri/pull/3603)
* [miri: ignore the Helix configuration directory](https://github.com/rust-lang/miri/pull/3611)
* [miri: make basic things work on Android](https://github.com/rust-lang/miri/pull/3616)
* [miri: properly print error in 'cargo miri setup --print-sysroot'](https://github.com/rust-lang/miri/pull/3619)
* [miri: support `aligned_alloc` for unixes](https://github.com/rust-lang/miri/pull/3585)
* [miri: use `throw_unsup_format!` instead of returning `ENOTSUP` in the mmap shim](https://github.com/rust-lang/miri/pull/3610)
* [miri: use a little arg-parsing helper for miri-script](https://github.com/rust-lang/miri/pull/3621)
* [remove libc from MSVC targets](https://github.com/rust-lang/rust/pull/124050)
* [optimize character escaping](https://github.com/rust-lang/rust/pull/124307)
* [alloc: implement FromIterator for `Box<str>`](https://github.com/rust-lang/rust/pull/99969)
* [implemented Default for `Arc<str>`](https://github.com/rust-lang/rust/pull/124640)
* [fix `read_exact` and `read_buf_exact` for `&[u8]` and `io:Cursor`](https://github.com/rust-lang/rust/pull/125123)
* [fix assertion when attempting to convert `f16` and `f128` with `as`](https://github.com/rust-lang/rust/pull/125172)
* [inline `Duration` construction into `Duration::from_`{`secs`, `millis`, `micros`, `nanos`}](https://github.com/rust-lang/rust/pull/125232)
* [remove bound checks from `BorrowedBuf` and `BorrowedCursor` methods](https://github.com/rust-lang/rust/pull/123786)
* [optimize inplace collection of `Vec`](https://github.com/rust-lang/rust/pull/123878)
* [make `Debug` impl for `Term` simpler](https://github.com/rust-lang/rust/pull/125279)
* [invert comparison in `u`N`::checked_sub`](https://github.com/rust-lang/rust/pull/125038)
* [add `f128` float to integer conversion functions](https://github.com/rust-lang/compiler-builtins/pull/613)
* [add addition, subtraction, multiplication, and compare operations for `f128`](https://github.com/rust-lang/compiler-builtins/pull/606)
* [add `powi` fo `f16` and `f128`](https://github.com/rust-lang/rust/pull/125188)
* [add v0 symbol mangling for `f16` and `f128`](https://github.com/rust-lang/rust/pull/123816)
* [re-add `From<f16> for f64`](https://github.com/rust-lang/rust/pull/124728)
* [cargo: add special `check-cfg` lint config for the `unexpected_cfgs` lint](https://github.com/rust-lang/cargo/pull/13913)
* [cargo: fix warning about unused Permissions](https://github.com/rust-lang/cargo/pull/13938)
* [cargo: fix warning output in `build_with_symlink_to_path_dependency_with_build_script_in_git`](https://github.com/rust-lang/cargo/pull/13930)
* [cargo: fix: make path dependencies with the same name stays locked](https://github.com/rust-lang/cargo/pull/13572)
* [cargo: fix: support IPv6-only network for cargo fix](https://github.com/rust-lang/cargo/pull/13907)
* [cargo: load `libsecret` by its `SONAME`, `libsecret-1.so.0`](https://github.com/rust-lang/cargo/pull/13927)
* [cargo: preserve file permissions on unix during `write_atomic`](https://github.com/rust-lang/cargo/pull/13898)
* [cargo: silence warnings running embedded unittests](https://github.com/rust-lang/cargo/pull/13929)
* [rustdoc: don't strip items with inherited visibility in `AliasedNonLocalStripper`](https://github.com/rust-lang/rust/pull/125300)
* [rustdoc: negative impls are not notable](https://github.com/rust-lang/rust/pull/125134)
* [add `-` (stdin) support in rustdoc](https://github.com/rust-lang/rust/pull/124611)
* [clippy: `assigning_clones`: move to `pedantic` so it is allow by default](https://github.com/rust-lang/rust-clippy/pull/12779)
* [clippy: `doc_lazy_continuation`: do not warn on `End` events](https://github.com/rust-lang/rust-clippy/pull/12818)
* [clippy: add configuration option for ignoring `panic!()` in tests](https://github.com/rust-lang/rust-clippy/pull/12803)
* [clippy: don't lint path statements in `no_effect`](https://github.com/rust-lang/rust-clippy/pull/12798)
* [clippy: don't lint `missing_panic_docs` for panics in `const` environments](https://github.com/rust-lang/rust-clippy/pull/12790)
* [clippy: improve `match_same_arms` messages, enable rustfix test](https://github.com/rust-lang/rust-clippy/pull/12794)
* [clippy: less aggressive `needless_borrows_for_generic_args`](https://github.com/rust-lang/rust-clippy/pull/12706)
* [clippy: make sure the msrv for `const_raw_ptr_deref` is met when linting `missing_const_for_fn`](https://github.com/rust-lang/rust-clippy/pull/12713)
* [clippy: manually set library paths in .github/driver.sh](https://github.com/rust-lang/rust-clippy/pull/12812)
* [rust-analyzer: fix metrics workflow not actually updating the toolchain](https://github.com/rust-lang/rust-analyzer/pull/17238)
* [rust-analyzer: fix: don't emit --keep-going for custom build script commands](https://github.com/rust-lang/rust-analyzer/pull/17232)
* [rust-analyzer: fix: expand macro recursively expands both fp-like and attribute macros when intertwined](https://github.com/rust-lang/rust-analyzer/pull/17225)
* [rust-analyzer: fix: extract mod to file should respect path attribute](https://github.com/rust-lang/rust-analyzer/pull/17216)
* [rust-analyzer: fix: hash file contents to verify whether file actually changed](https://github.com/rust-lang/rust-analyzer/pull/17227)

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

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
<!-- Remove this section if empty>

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
<!-- Remove this section if empty>

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

<!-- New or updated RFCs go here, use this format: * [new|updated] [Topic](URL) -->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- * [new|updated] []() -->

## Upcoming Events

Rusty Events between 2024-05-22 - 2024-06-19 🦀

### Virtual

* 2024-05-15 | Virtual (Ankara, TR) | [Türkiye Rust Community](https://kommunity.com/turkiye-rust-community/events/)
    * [**#RustSemineri - 7**](https://kommunity.com/turkiye-rust-community/events/rustsemineri-7-0a97e784)
* 2024-05-15 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 6 - Testing**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/300819214/)
* 2024-05-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**NativeLink**](https://www.meetup.com/vancouver-rust/events/298542331/)
* 2024-05-16 | Virtual (Ankara, TR) | [Türkiye Rust Community](https://kommunity.com/turkiye-rust-community/events)
    * [**#RustSemineri - 8**](https://kommunity.com/turkiye-rust-community/events/rustsemineri-8-ddfe6b15)
* 2024-05-16 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298312423/)
* 2024-05-17 | Virtual | [Training 4 Programmers LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Rust at Full Speed: Harnessing Concurrency with Confidence**](https://www.eventbrite.com/e/rust-at-full-speed-harnessing-concurrency-with-confidence-tickets-884842296127)
* 2024-05-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—forensic parsing via Artemis**](https://www.meetup.com/rustdc/events/299346490/)
* 2024-05-23 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477699/)
* 2024-05-23 | Virtual (Israel) | [Rust in Israel](https://rust.org.il/)
    * [**Web development in Rust using Rocket (Hebrew)**](https://www.meetup.com/code-mavens/events/300974367/)
* 2024-05-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/300533392/)
* 2024-05-28 & 2024-05-28 | Virtual | [Mainmatter](https://mainmatter.com/)
    * [**Remote Workshop: Telemetry for Rust APIs – you can't fix what you can't see (fee)**](https://ti.to/mainmatter/rust-telemetry-may-2024)
* 2024-05-30 | Virtual + In Person (Barcelona, ES) | [Mainmatter](https://mainmatter.com/) & [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**Rust for the web, Barcelona 2024**](https://www.meetup.com/es-ES/bcnrust/events/300765894/) 
* 2024-05-30 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298542326/)
* 2024-06-04 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300191681/)
* 2024-06-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047896/)
* 2024-06-06 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477702/)
* 2024-06-11 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341709/)

### Africa

* 2024-06-01 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia

* 2024-05-22 | Singapore, SG | [SG Rust Meetup](https://www.meetup.com/rust-singapore/)
    * [**SG Rustaceans! Updated - SG Rust Meetup at CraftsforGreen Whole Studio**](https://www.meetup.com/rust-singapore/events/300988123/)

### Europe

* 2024-05-16 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #7**](https://www.meetup.com/rust-meetup-augsburg/events/300174327/)
* 2024-05-16 | Paris, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [**Paris Rust Meetup #68**](https://mobilizon.fr/events/14b51ccc-211f-400f-9615-707d9d871e78)
* 2024-05-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/300307155/)
* 2024-05-21 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**Save the date - Mai Meetup**](https://www.meetup.com/rust-zurich/events/300513957/)
* 2024-05-22 | Leiden, NL | [Future-proof Software Development by FreshMinds](https://www.meetup.com/freshminds-future-proof-software-development/)
    * [**Coding Dojo Session**](https://www.meetup.com/freshminds-future-proof-software-development/events/300566391/)
* 2024-05-23 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**2024 Rust Talks Bern #2**](https://www.meetup.com/rust-bern/events/300286917/)
* 2024-05-23 | Łodz, PL | [Mobica](https://www.linkedin.com/posts/mobica_rust-programming-embeddedsoftware-activity-7193232853717946369-CK68/)
    * [**Zapisz się na warsztat Rust / Embedded w Łodzi! / What's all the fuss about Rust?**](https://www.interankiety.pl/f/b4D7G7xO)
* 2024-05-23 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester May Code Night**](https://www.meetup.com/rust-manchester/events/300923207/)
* 2024-05-24 | Bordeaux, FR | [Rust Bordeaux](https://www.meetup.com/bordeaux-rust/)
    * [**Rust Bordeaux #3: Discussions**](https://www.meetup.com/bordeaux-rust/events/300723854/)
* 2024-05-25 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/)
    * [**Ferris' Fika Forum #3 [Embedded lab edition]**](https://www.meetup.com/stockholm-rust/events/301014982/)
* 2024-05-28 - 2024-05-30 | Berlin, DE | [Oxidize](https://oxidizeconf.com/)
    * [**Oxidize Conf 2024**](https://oxidizeconf.com/)
* 2024-05-30 | Barcelona, ES | [Mainmatter](https://mainmatter.com/) & [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**Rust for the web, Barcelona 2024**](https://www.meetup.com/es-ES/bcnrust/events/300765894/) 
* 2024-05-30 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299288963/)
* 2024-05-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #47 sponsored by Microsoft!**](https://www.meetup.com/copenhagen-rust-community/events/300458222/)
* 2024-05-30 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/300453310/)
* 2024-06-05 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn June 2024**](https://www.meetup.com/rust-meetup-hamburg/events/299235215/)
* 2025-06-06 | Vilnius, LT | [Rust Vilnius](https://www.meetup.com/rust-in-vilnius/)
    * [**Enjoy our second Rust and ZIG event**](https://www.meetup.com/rust-in-vilnius/events/301012097/)

### North America

* 2024-05-16 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/300775539/)
* 2024-05-16 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/299509369/)
* 2024-05-20 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch, May 20**](https://www.meetup.com/bostonrust/events/300116765/)
* 2024-05-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186931/)
* 2024-05-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygchbdc/)
* 2024-05-25 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Talk Double Feature**](https://www.meetup.com/deep-dish-rust/events/300665520/)
* 2024-05-30 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/300775547/)
* 2024-05-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston Common Rust Lunch, May 31**](https://www.meetup.com/bostonrust/events/300116786/)
* 2024-06-08 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Porter Square Rust Lunch, Jun 8**](https://www.meetup.com/bostonrust/events/300116799/)

### Oceania

* 2024-05-28 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**a demo 🤯 & a lightning ⚡show ✨**](https://www.meetup.com/rust-sydney/events/300854266/)

### South America

* 2024-06-06 | Buenos Aires, AR | [Rust en Español | Rust Argentina](https://www.meetup.com/rust-argentina/)
    * [**Juntada de Junio**](https://www.meetup.com/rust-argentina/events/299740249)

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

> In other words, I do not want the compiler to just insert code to uphold the bare minimum guarantees, I want the compiler to check my work for me and assist me in developing an algorithm I can confidently assert is right.

– [without boats](https://without.boats/blog/references-are-like-jumps/)

Thanks to [scottmcm](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1568) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
