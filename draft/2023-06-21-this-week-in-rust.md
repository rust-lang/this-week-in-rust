Title: This Week in Rust 500
Number: 500
Date: 2023-06-21
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

This week's crate is [release-plz](https://github.com/MarcoIeni/release-plz), a crate to automate changelog generation, GitHub/Gitea release tagging, publishing on [crates.io](https://crates.io) and bumping the version.

Thanks to [Marco Ieni](https://users.rust-lang.org/t/crate-of-the-week/2704/1207) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

410 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-06-12..2023-06-19

* [collect VTable stats & add `-Zprint-vtable-sizes`](https://github.com/rust-lang/rust/pull/112400)
* [prevent `.eh_frame` from being emitted for `-C panic=abort`](https://github.com/rust-lang/rust/pull/112403)
* [`#[test]` function signature verification improvements](https://github.com/rust-lang/rust/pull/112366)
* [add `<meta charset="utf-8">` to `-Zdump-mir-spanview` output](https://github.com/rust-lang/rust/pull/112728)
* [add casting suggestion when assigning negative 2's complement bin or hex literal to a size compatible signed integer](https://github.com/rust-lang/rust/pull/111212)
* [continue folding in query normalizer on weak aliases](https://github.com/rust-lang/rust/pull/112777)
* [don't record adjustments twice in `note_source_of_type_mismatch_constraint`](https://github.com/rust-lang/rust/pull/112537)
* [don't ICE on bound var in `reject_fn_ptr_impls`](https://github.com/rust-lang/rust/pull/112783)
* [don't ICE on unsized `extern "rust-call"` call](https://github.com/rust-lang/rust/pull/111885)
* [don't capture `&[T; N]` when contents isn't read](https://github.com/rust-lang/rust/pull/112636)
* [extend `unused_must_use` to cover block exprs](https://github.com/rust-lang/rust/pull/112529)
* [fix `dead_code_cgu` computation](https://github.com/rust-lang/rust/pull/112639)
* [fix explicit-outlives-requirements lint span](https://github.com/rust-lang/rust/pull/112612)
* [fix suggestion for E0404 not dealing with multiple generics](https://github.com/rust-lang/rust/pull/112486)
* [fix the overflow issue for `transmute_generic_consts`](https://github.com/rust-lang/rust/pull/112520)
* [handle interpolated literal errors](https://github.com/rust-lang/rust/pull/112642)
* [non-unwinding panic for misaligned pointer deref](https://github.com/rust-lang/rust/pull/112599)
* [make `struct` layout not depend on unsizeable tail](https://github.com/rust-lang/rust/pull/112062)
* [new solver proof tree generation](https://github.com/rust-lang/rust/pull/112351)
* [opportunistically resolve regions in new solver](https://github.com/rust-lang/rust/pull/112443)
* [normalize closure output in `equate_inputs_and_outputs`](https://github.com/rust-lang/rust/pull/112654)
* [safe Transmute: Enable handling references](https://github.com/rust-lang/rust/pull/110662)
* [support 128-bit `enum` variant in debuginfo codegen](https://github.com/rust-lang/rust/pull/112474)
* [introduce a minimum CGU size in non-incremental builds](https://github.com/rust-lang/rust/pull/112448)
* [ignore the always part of `#[inline(always)]` in MIR inlining](https://github.com/rust-lang/rust/pull/112294)
* [introduce a `Stable` trait to translate MIR to SMIR](https://github.com/rust-lang/rust/pull/112600)
* [make mir dataflow graphviz dumps opt-in](https://github.com/rust-lang/rust/pull/112617)
* [miri: dereference pointers in shims as correct types](https://github.com/rust-lang/miri/pull/2661)
* [promote unchecked integer math to MIR `BinOp`s](https://github.com/rust-lang/rust/pull/112238)
* [simplify `unchecked_{shl,shr}`](https://github.com/rust-lang/rust/pull/112724)
* [implement `TryFrom<&OsStr>` for `&str`](https://github.com/rust-lang/rust/pull/98202)
* [extend `io::copy` buffer reuse to BufReader too](https://github.com/rust-lang/rust/pull/112330)
* [stabilize `String::leak`](https://github.com/rust-lang/rust/pull/109814)
* [`available_parallelism` using native netbsd api first](https://github.com/rust-lang/rust/pull/112226)
* [only depend on dlmalloc for wasm*-unknown](https://github.com/rust-lang/rust/pull/112685)
* [don't drain-on-drop in `DrainFilter` impls of various collections](https://github.com/rust-lang/rust/pull/104455)
* [make `BinaryHeap` parametric over Allocator](https://github.com/rust-lang/rust/pull/99339)
* [optimize `slice::Iter::fold`](https://github.com/rust-lang/rust/pull/106343)
* [relax implicit `T: Sized` bounds on `BufReader<T>`, `BufWriter<T>` and `LineWriter<T>`](https://github.com/rust-lang/rust/pull/111074)
* [hashbrown: fix leaking of allocator in `RawIntoIter` and `RawIntoParIter`](https://github.com/rust-lang/hashbrown/pull/439)
* [cargo: Align package name sanitization with cargo-new](https://github.com/rust-lang/cargo/pull/12255)
* [cargo: Don't auto-discover build.rs files](https://github.com/rust-lang/cargo/pull/12283)
* [cargo: Switch to `syn` for parsing doc comments](https://github.com/rust-lang/cargo/pull/12258)
* [cargo: enable `doctest-in-workspace` by default](https://github.com/rust-lang/cargo/pull/12221)
* [cargo: fix version requirement example in Dependency Resolution, SemVer compatibility section](https://github.com/rust-lang/cargo/pull/12267)
* [rustdoc: Fix URL encoding of % sign](https://github.com/rust-lang/rust/pull/112581)
* [rustdoc: Fix invalid handling of "going back in history" when "go to only search result" setting is enabled](https://github.com/rust-lang/rust/pull/112707)
* [rustdoc-gui: allow running on Windows](https://github.com/rust-lang/rust/pull/112562)
* [rustdoc-search: search never type with `!`](https://github.com/rust-lang/rust/pull/112571)
* [rustdoc: add search result item types after their name](https://github.com/rust-lang/rust/pull/110688)
* [rustfmt: adjust `enum` variant spans to exclude any explicit discriminant](https://github.com/rust-lang/rustfmt/pull/5687)
* [rustfmt: prevent ICE when calling `parse_attribute` without an attribute](https://github.com/rust-lang/rustfmt/pull/5732)
* [clippy: add lint `incorrect_clone_impl_on_copy_type`](https://github.com/rust-lang/rust-clippy/pull/10925)
* [clippy: new lint `single_call_fn`](https://github.com/rust-lang/rust-clippy/pull/10951)
* [clippy: new lint `single_range_in_vec_init`](https://github.com/rust-lang/rust-clippy/pull/10934)
* [clippy: new lint: `drain_collect`](https://github.com/rust-lang/rust-clippy/pull/10835)
* [clippy: `arithmetic_side_effects` also lint const arithmetic](https://github.com/rust-lang/rust-clippy/pull/10793)
* [clippy: `missing_panics_doc`: pickup expect method](https://github.com/rust-lang/rust-clippy/pull/10953)
* [clippy: `redundant_closure_call`: handle nested closures](https://github.com/rust-lang/rust-clippy/pull/10930)
* [clippy: fix `find_format_arg_expr` when incremental compilation is enabled](https://github.com/rust-lang/rust-clippy/pull/10980)
* [clippy: `derivable_impls`: don't lint if `default()` call expr unsize-coerces to trait object](https://github.com/rust-lang/rust-clippy/pull/10954)
* [clippy: `map_unwrap_or`: don't lint when referenced variable is moved](https://github.com/rust-lang/rust-clippy/pull/10919)
* [clippy: `match_same_arms`: don't lint if `non_exhaustive_omitted_patterns`](https://github.com/rust-lang/rust-clippy/pull/10946)
* [clippy: `missing_const_for_fn`: Ensure dropped locals are `~const Destruct`](https://github.com/rust-lang/rust-clippy/pull/10891)
* [clippy: `needless_doctest_main`: ignore `main()` in `no_test` code fences](https://github.com/rust-lang/rust-clippy/pull/10950)
* [clippy: make `missing_panics_doc` not lint for `todo!()`](https://github.com/rust-lang/rust-clippy/pull/10976)
* [clippy: don't lint non-statement/faux empty `needless_if`s](https://github.com/rust-lang/rust-clippy/pull/10935)
* [clippy: fix false positive of `self_named_module_files` and `mod_module_files`](https://github.com/rust-lang/rust-clippy/pull/10975)
* [clippy: ignore more type aliases in `unnecessary_cast`](https://github.com/rust-lang/rust-clippy/pull/10942)
* [clippy: adding configuration to allow safety comment above stmt containing unsafe block](https://github.com/rust-lang/rust-clippy/pull/10886)
* [clippy: improve suggestion for `needless_lifetimes`](https://github.com/rust-lang/rust-clippy/pull/10947)
* [clippy: `from_over_into`: Show suggestions for non-Self expanded paths](https://github.com/rust-lang/rust-clippy/pull/10840)
* [clippy: `unnecessary_fold`: suggest turbofish if necessary](https://github.com/rust-lang/rust-clippy/pull/10931)
* [clippy: `no_effect`: Suggest adding `return` if applicable](https://github.com/rust-lang/rust-clippy/pull/10945)
* [clippy: make `--explain` subcommand return 1 for missing lints](https://github.com/rust-lang/rust-clippy/pull/10965)
* [rust-analyzer: correctly handle inlining of async fn](https://github.com/rust-lang/rust-analyzer/pull/15074)
* [rust-analyzer: deduplicate tuple indices for completion](https://github.com/rust-lang/rust-analyzer/pull/15044)
* [rust-analyzer: add binding definition for for-expr iterator desugared binding](https://github.com/rust-lang/rust-analyzer/pull/15075)
* [rust-analyzer: ensure that ws loading error includes path to ws](https://github.com/rust-lang/rust-analyzer/pull/15085)
* [rust-analyzer: implement missing members doesn't transform const params and default types](https://github.com/rust-lang/rust-analyzer/pull/15054)

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

Rusty Events between 2023-06-21 - 2023-07-19 🦀

### Virtual

* 2023-06-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763484)
* 2023-06-22 | Virtual (Karlsruhe, DE) | [Karlsruhe Functional Programmers Group](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/)
    * [**Stammtisch (gemeinsam mit der C++ UG KA)**](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/events/293937801)
* 2023-06-25 | Virtual (Auckland, NZ) | [ResBaz Aotearoa 2023](https://resbaz.auckland.ac.nz/)
    * [**Research Computing With The Rust Programming Language - Tim McNamara**](https://www.eventbrite.com/e/research-computing-with-the-rust-programming-language-tickets-637648623197?aff=ebdssbdestsearch)
* 2023-06-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfcjbkc/)
* 2023-06-28 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Building Our Own 'Arc' in Rust (Atomics & Locks Chapter 6)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/294184120/)
* 2023-06-28 | Virtual (Chicago, IL, US) | [Chicago Healthcare Cloud Technology Community](https://www.meetup.com/chicago-healthcare-tech-and-ai/)
    * [**Rust for Mission-Critical AI: A Journey into Healthcare's Safest Language**](https://www.meetup.com/chicago-healthcare-tech-and-ai/events/293278396)
* 2023-06-29 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/294294905)
* 2023-06-29 | Virtual (Ciudad de México, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Rust y Haskell**](https://www.meetup.com/rust-mx/events/294152158)
* 2023-07-01 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 4: Protohackers Exercises Mob Coding (Problem II onwards)**](https://www.meetup.com/rust-noris/events/293800373)
* 2023-07-04 | Virtual (Berlin, DE) | [Berline.rs / OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/zdrzpsyfckbgb/)
* 2023-07-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfckbgb/)
* 2023-07-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/293309295)
* 2023-07-05 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfckbhb)
* 2023-07-11 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsyfckbpb)
* 2023-07-13 - 2023-07-14 | Virtual | [Scientific Computing in Rust](https://scientificcomputing.rs/)
    * [**Scientific Computing in Rust workshop**](https://scientificcomputing.rs/)
* 2023-07-13 | Virtual (Edinburgh, UK) | [Rust Edinburgh](https://www.meetup.com/rust-edi/)
    * [**Reasoning about Rust: an introduction to Rustdoc’s JSON format**](https://www.meetup.com/rust-edi/events/293820336/)

### Asia

* 2023-06-29 | Seoul, KR | [T-RUST meetup](https://www.meetup.com/t-rust-meetup/)
    * [**🦀 T-RUST Meetup 🦀**](https://www.meetup.com/t-rust-meetup/events/294280140/)

### Europe

* 2023-06-22 | Vienna, AT | [Papers We Love Vienna](https://www.meetup.com/papers-we-love-vienna/)
    * [**June: Data and Ownership in Rust**](https://www.meetup.com/papers-we-love-vienna/events/293974147)
* 2023-06-22 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**Second Rust Bern Meetup**](https://www.meetup.com/de-DE/rust-bern/events/293619228/)
* 2023-06-22 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Meetup #34**](https://www.meetup.com/rust-wroclaw/events/294176450/)
* 2023-06-27 | Bucharest, RO | [Rust Lang Bucharest Meetup](https://www.meetup.com/rust-lang-bucharest-meetup/)
    * [**Rust Bucharest Meetup #2**](https://www.meetup.com/rust-lang-bucharest-meetup/events/294204963/)
* 2023-06-27 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**LDN Talks June 2023 *Shuttle Takeover***](https://www.meetup.com/rust-london-user-group/events/294038743/)
* 2023-06-27 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #60**](https://www.meetup.com/rust-paris/events/294138477)
* 2023-06-28 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake**](https://www.meetup.com/bratislava-rust-meetup-group/events/293732916)
* 2023-06-29 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup**](https://www.meetup.com/rust-meetup-augsburg/events/293566071/)
* 2023-06-29 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #37 at Samsung!**](https://www.meetup.com/copenhagen-rust-community/events/294024476)
* 2023-06-29 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna)
    * [**Rust Vienna Meetup - June - final meetup before a summer break**](https://www.meetup.com/rust-vienna/events/294225540/)
* 2023-07-01 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**(Beginner) Rust Workshop**](https://www.meetup.com/rust-basel/events/293906330/)
* 2023-07-03 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**Rust in the Linux Kernel - July Meetup**](https://www.meetup.com/rust-zurich/events/293322905)
* 2023-07-05 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #5**](https://www.meetup.com/fr-FR/rust-lyon/events/294325808)
* 2023-07-11 | Breda, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Rust: Advanced Graphics and User Interfaces**](https://www.meetup.com/rust-nederland/events/294199533/)
* 2023-07-13 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns***](https://www.meetup.com/reading-rust-workshop/events/mstlftyfckbrb/)

### North America

* 2023-06-21 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Pizza sponsored by JFrog**](https://www.meetup.com/rust-atx/events/294049756)
* 2023-06-21 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/BostonRust/)
    * [**Ball Square Rust Lunch, June 21**](https://www.meetup.com/BostonRust/events/293725119)
* 2023-06-22 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Learn How to Use cargo-semver-checks and Closure Traits to Write Better Code**](https://www.meetup.com/rust-nyc/events/294123104)
* 2023-06-24 | San Jose, CA, US | [Rust Breakfast & Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/jnmgftyfcjbgc/)
* 2023-06-28 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/BostonRust/)
    * [**Harvard Square Rust Lunch**](https://www.meetup.com/BostonRust/events/293725559)
* 2023-06-29 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/294223607)
* 2023-07-01 | San Jose, CA, US | [Rust Breakfast & Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/jnmgftyfckbcb/)
* 2023-07-07 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Lunch**](https://www.meetup.com/deep-dish-rust/events/293794930/)
* 2023-07-13 | Seattle, WA, US | [Seattle Rust User Group Meetup](https://www.meetup.com/seattle-rust-user-group/)
    * [**July Meetup**](https://www.meetup.com/seattle-rust-user-group/events/294191599/)
* 2023-07-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfckbxb)

### Oceania

* 2023-07-11 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Hybrid - in person & online) July 2023 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/294274774/)

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

> rust programmers when they see each other again:
> > Long time no C

– [ciscoffeine on mond-basis.eu](https://mond-basis.eu/@transcaffeine/110538051681033551)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1435) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
