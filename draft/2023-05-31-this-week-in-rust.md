Title: This Week in Rust 497
Number: 497
Date: 2023-05-31
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

### Foundation

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [progenitor](https://github.com/oxidecomputer/progenitor), an OpenAPI client generator with support for strongly typed mock tests.

Thanks to [John Vandenberg](https://users.rust-lang.org/t/crate-of-the-week/2704/1200) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Ockam - Don't show a secure channel listener if it does not exist 2](https://github.com/build-trust/ockam/issues/4974)
* [Ockam - `ockam secure-channel-listener list --node n1` is inconsistent 2](https://github.com/build-trust/ockam/issues/4973)
* [Ockam - Do not print that a secure channel listener was successfully deleted when it is not the case 2](https://github.com/build-trust/ockam/issues/4972)
* [Ockam - `tcp-connection show` command should iter all the available nodes to find the requested ID](https://github.com/build-trust/ockam/issues/4976)
* [Ockam - Define json output for `ockam node create`](https://github.com/build-trust/ockam/issues/4967)
* [velo - Implement Copy and Paste Functionality for `bevy_cosmic_edit` - Issue #128 - StaffEngineer/velo - GitHub 2](https://github.com/StaffEngineer/velo/issues/128)
* [velo - Fix text height calculation for proper text alignment - Issue #131 - StaffEngineer/velo - GitHub 1](https://github.com/StaffEngineer/velo/issues/131)
* [send-file - create WiFi hotspot on Linux Operating system 1](https://github.com/opeolluwa/send-file/issues/83)
* [send-file - read the device default documents directory and return an array of documents files path 1](https://github.com/opeolluwa/send-file/issues/84)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

325 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-05-22..2023-05-29

* [promote loongarch64-unknown-linux-gnu to Tier 2 with host tools](https://github.com/rust-lang/rust/pull/110936)
* [enable sanitizers and profiler for aarch64-unknown-linux-musl](https://github.com/rust-lang/rust/pull/111575)
* [add support for LLVM SafeStack](https://github.com/rust-lang/rust/pull/112000)
* [Expand more expressions](https://github.com/rust-lang/rust/pull/111928) (RFC [#2011](https://rust-lang.github.io/rfcs/2011-generic-assert.html))
* [parse: return unpected when current token is EOF](https://github.com/rust-lang/rust/pull/111181)
* [resolve: not defined `extern crate shadow_name`](https://github.com/rust-lang/rust/pull/111761)
* [`rustc_privacy`: Cleanups and perf improvements to `EmbargoVisitor`](https://github.com/rust-lang/rust/pull/111260)
* [add warn-by-default lint when local binding shadows exported glob re-export item](https://github.com/rust-lang/rust/pull/111378)
* [always require closure parameters to be `Sized`](https://github.com/rust-lang/rust/pull/111864)
* [check opaques for mismatch during writeback](https://github.com/rust-lang/rust/pull/111853)
* [consider lint check attributes on match arms](https://github.com/rust-lang/rust/pull/111757)
* [deal with unnormalized projections when structurally resolving types with new solver](https://github.com/rust-lang/rust/pull/110204)
* [do not prefer substs relate during coherence](https://github.com/rust-lang/rust/pull/111987)
* [don't ICE if method receiver fails to unify with `arbitrary_self_types`](https://github.com/rust-lang/rust/pull/111860)
* [don't ICE on return-type notation when promoting trait preds to associated type bounds](https://github.com/rust-lang/rust/pull/111861)
* [don't ICE when computing PointerLike trait when region vars are in param-env](https://github.com/rust-lang/rust/pull/111880)
* [don't check for misaligned raw pointer derefs inside `Rvalue::AddressOf`](https://github.com/rust-lang/rust/pull/112026)
* [don't skip mir typeck if body has errors](https://github.com/rust-lang/rust/pull/111863)
* [enable `MatchBranchSimplification`](https://github.com/rust-lang/rust/pull/112001)
* [expose more information in `get_body_with_borrowck_facts`](https://github.com/rust-lang/rust/pull/111840)
* [fix for `Self` not respecting tuple Ctor privacy](https://github.com/rust-lang/rust/pull/111245)
* [fix re-export of doc hidden macro not showing up](https://github.com/rust-lang/rust/pull/111997)
* [fix some issues with folded AArch64 features](https://github.com/rust-lang/rust/pull/107294)
* [fix symbol conflict diagnostic mistakenly being shown instead of missing crate diagnostic](https://github.com/rust-lang/rust/pull/111461)
* [handle opaques in the new solver](https://github.com/rust-lang/rust/pull/111473)
* [improve error message for calling a method on a raw pointer with an unknown pointee](https://github.com/rust-lang/rust/pull/111954)
* [inline derived `hash`](https://github.com/rust-lang/rust/pull/111963)
* [leverage the interval property to precompute borrow kill points](https://github.com/rust-lang/rust/pull/111759)
* [libtest: improve error when missing `-Zunstable-options`](https://github.com/rust-lang/rust/pull/111915)
* [perform MIR type ops locally in new solver](https://github.com/rust-lang/rust/pull/111983)
* [preprocess and cache dominator tree](https://github.com/rust-lang/rust/pull/111673)
* [pretty-print inherent projections correctly](https://github.com/rust-lang/rust/pull/111486)
* [prevent crash when a path is not resolved in intra-doc link](https://github.com/rust-lang/rust/pull/111195)
* [rework handling of recursive panics](https://github.com/rust-lang/rust/pull/110975)
* [split out opaque collection from from `type_of`](https://github.com/rust-lang/rust/pull/111862)
* [stop confusing specification levels when computing expectations](https://github.com/rust-lang/rust/pull/111714)
* [use `ErrorGuaranteed` more in MIR type ops](https://github.com/rust-lang/rust/pull/111918)
* [MIR: opt-in normalization of `BasicBlock` and `Local` numbering](https://github.com/rust-lang/rust/pull/111813)
* [support `#[global_allocator]` without the allocator shim](https://github.com/rust-lang/rust/pull/86844)
* [stabilize `BuildHasher::hash_one`](https://github.com/rust-lang/rust/pull/111934)
* [use an unbounded lifetime in `String::leak`](https://github.com/rust-lang/rust/pull/111656)
* [add Median of Medians fallback to introselect](https://github.com/rust-lang/rust/pull/107522)
* [add `#[inline]` to array TryFrom impls](https://github.com/rust-lang/rust/pull/111966)
* [add `slice::`{`split_`, }{`first`, `last`}`_chunk`{, `_mut`}](https://github.com/rust-lang/rust/pull/95198)
* [stdarch: stabilize AArch64 AES/SHA1/SHA2 intrinsics](https://github.com/rust-lang/stdarch/pull/1399)
* [regex: fix complete literal optimization issue](https://github.com/rust-lang/regex/pull/1000)
* [cargo: add: Reduce the chance we re-format the user's `[features]` table](https://github.com/rust-lang/cargo/pull/12191)
* [cargo: lints: Switch to -Zlints so stable projects can experiment](https://github.com/rust-lang/cargo/pull/12168)
* [cargo: add a description of `Cargo.lock` conflicts in the Cargo FAQ](https://github.com/rust-lang/cargo/pull/12185)
* [cargo: automatically inherit workspace fields when running cargo new/init](https://github.com/rust-lang/cargo/pull/12069)
* [cargo: automatically inherit workspace lints when running cargo new/init](https://github.com/rust-lang/cargo/pull/12174)
* [cargo: consider rust-version when selecting packages for cargo add](https://github.com/rust-lang/cargo/pull/12078)
* [cargo: deps: remove unused features from windows-sys](https://github.com/rust-lang/cargo/pull/12176)
* [cargo: warn when an edition 2021 crate is in a virtual workspace with default resolver](https://github.com/rust-lang/cargo/pull/10910)
* [rustdoc: get unnormalized link destination for suggestions](https://github.com/rust-lang/rust/pull/112014)
* [rustdoc-json: Use exclusively externally tagged enums in the JSON representation](https://github.com/rust-lang/rust/pull/111427)
* [new tool `rustdoc-gui-test`](https://github.com/rust-lang/rust/pull/111348)
* [clippy: `default_constructed_unit_structs`: do not lint on type alias paths](https://github.com/rust-lang/rust-clippy/pull/10813)
* [clippy: `large_stack_arrays`: check array initializer expressions](https://github.com/rust-lang/rust-clippy/pull/10806)
* [clippy: add `needless_else` lint to check for empty `else` clauses](https://github.com/rust-lang/rust-clippy/pull/10810)
* [clippy: `unused_async`: do not consider `await` in nested `async` blocks as used](https://github.com/rust-lang/rust-clippy/pull/10807)
* [clippy: add new lint `ptr_cast_constness`](https://github.com/rust-lang/rust-clippy/pull/10779)
* [clippy: display the `needless_return` suggestion](https://github.com/rust-lang/rust-clippy/pull/10819)
* [clippy: fix `redundant_pattern_match` on matches! macro](https://github.com/rust-lang/rust-clippy/pull/10831)
* [clippy: fix missing block for unsafe code](https://github.com/rust-lang/rust-clippy/pull/10809)
* [clippy: fixing `invalid_regex` with invalid UTF8. Also, adding more test cases](https://github.com/rust-lang/rust-clippy/pull/10839)
* [clippy: ignore `#[cfg]`'d out code in `needless_else`](https://github.com/rust-lang/rust-clippy/pull/10822)
* [clippy: improve pattern printing for `manual_let_else`](https://github.com/rust-lang/rust-clippy/pull/10797)
* [rust-analyzer: editors/code: add markdown syntax highlighting to doc comments](https://github.com/rust-lang/rust-analyzer/pull/14866)
* [rust-analyzer: allow users to override the .scip output file path](https://github.com/rust-lang/rust-analyzer/pull/14894)
* [rust-analyzer: add diagnostic for incorrect `_` expressions (typed holes)](https://github.com/rust-lang/rust-analyzer/pull/14916)
* [rust-analyzer: assist to replace generic with impl trait](https://github.com/rust-lang/rust-analyzer/pull/14816)
* [rust-analyzer: using doc aliases to search workspace symbols](https://github.com/rust-lang/rust-analyzer/pull/14849)
* [rust-analyzer: fix `need-mut` false positive in closure capture of match scrutinee](https://github.com/rust-lang/rust-analyzer/pull/14893)
* [rust-analyzer: add a toggle to disable the dependency explorer](https://github.com/rust-lang/rust-analyzer/pull/14906)
* [rust-analyzer: assists no longer break indentation](https://github.com/rust-lang/rust-analyzer/pull/14752)
* [rust-analyzer: change how `#![cfg(FALSE)]` behaves on crate root](https://github.com/rust-lang/rust-analyzer/pull/14874)
* [rust-analyzer: don't try determining type of token inside macro calls](https://github.com/rust-lang/rust-analyzer/pull/14895)
* [rust-analyzer: evaluate `UnevaluatedConst` in unify](https://github.com/rust-lang/rust-analyzer/pull/14891)
* [rust-analyzer: evaluate `UnevaluatedConst` before trait solving](https://github.com/rust-lang/rust-analyzer/pull/14913)
* [rust-analyzer: filter out unused cargo features from config](https://github.com/rust-lang/rust-analyzer/pull/14910)
* [rust-analyzer: insert type vars in function arguments](https://github.com/rust-lang/rust-analyzer/pull/14897)
* [rust-analyzer: use `::core` instead of `$crate` in `option_env!`](https://github.com/rust-lang/rust-analyzer/pull/14890)
* [rust-analyzer: implement `${count()}` metavariable expression](https://github.com/rust-lang/rust-analyzer/pull/14878)

### Rust Compiler Performance Triage

A good week overall, with a broad set of improvements to many primary benchmarks.
The main single source of primary regressions is from rollup PR #111869; we are
in the process of narrowing that down to see if there is a root cause.

Triage done by **@pnkfelix**.
Revision range: [cda5becc..1221e43b](https://perf.rust-lang.org/?start=cda5becc27cbc7106646fbc40aacea5e7896d954&end=1221e43bdf413f7c405e9b17ef19d76c88222098&absolute=false&stat=instructions%3Au)

3 Regressions, 3 Improvements, 3 Mixed; 4 of them in rollups
26 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-05-30.md)

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

Rusty Events between 2023-05-31 - 2023-06-28 🦀

### Virtual

* 2023-05-31 | Virtual (Chicago, IL, US) | [Chicago Healthcare Cloud Technology Community](https://www.meetup.com/chicago-healthcare-tech-and-ai/)
    * [**Rust for Mission-Critical AI: A Journey into Healthcare's Safest Language**](https://www.meetup.com/chicago-healthcare-tech-and-ai/events/293278396/?chapterContext=true&regToRsvp=true&isFromReg=true)
* 2023-06-06 | Virtual (Austin, TX, US) | [WebAssembly and WasmEdge](https://www.meetup.com/webassembly-and-wasmedge/)
    * [**Monthly WasmEdge Community Meeting**](https://www.meetup.com/webassembly-and-wasmedge/events/293014949)
* 2023-06-06 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/293485509)
* 2023-06-06 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/293296995)
* 2023-06-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/293309294)
* 2023-06-07 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/293616568)
* 2023-06-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732649)
* 2023-06-13 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/293014938)
* 2023-06-14 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/293014897)
* 2023-06-15 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/293786806)
* 2023-06-20 | Virtual (Berlin, DE) | [Berline.rs / OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/293485510)
* 2023-06-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/jkxsctyfcjbbc/)
* 2023-06-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763484)
* 2023-06-25 | Virtual (Auckland, NZ) | [ResBaz Aotearoa 2023](https://resbaz.auckland.ac.nz/)
    * [**Research Computing With The Rust Programming Language - Tim McNamara**](https://www.eventbrite.com/e/research-computing-with-the-rust-programming-language-tickets-637648623197?aff=ebdssbdestsearch)
* 2023-06-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfcjbkc/)

### Asia

* 2023-06-10 | Kuala Lumpur, MY | [GoLang Malaysia](https://t.me/golangmalaysia)
    * [**Rust Workshop/Hack and Learn Malaysia June 2023**](https://forms.gle/2fvbCG77HXCkWLfe6) | [Event updates Telegram](https://t.me/+dF46Fly4A_BjOTJl) | [Event group chat](https://t.me/golangmalaysia)

### Europe

* 2023-06-03 | Plovidv, BG | [AeroRust](https://aerorust.org)
    * [**Space Conference - Plovdiv**](https://space-conference-plovdiv.eventbrite.com)
* 2023-06-04 | Plovidv, BG | [AeroRust](https://aerorust.org)
    * [**Space Conference : Nanosatellite embedded workshop**](https://forms.gle/rnFx5wCbrYA9qN7m6)
* 2023-06-08 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust Aarhus meetup #2 sponsored by BRØLSTÆRK**](https://www.meetup.com/rust-aarhus/events/292865970/)
* 2023-06-08 | Paris, FR | [Stockly.ai](https://www.eventbrite.fr/o/stockly-42274765293)
    * [**Rust Meetup in Paris - hosted by Stockly**](https://www.eventbrite.fr/e/rust-meetup-in-paris-hosted-by-stockly-tickets-630742055467)
* 2023-06-08 | Zurich, CH | [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**Unsafe, Miri, SIMD - June Meetup**](https://www.meetup.com/de-DE/rust-zurich/events/293322792/)
* 2023-06-16 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/pdhvctyfcjbvb/)
* 2023-06-28 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake**](https://www.meetup.com/bratislava-rust-meetup-group/events/293732916)

### North America

* 2023-06-01 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/293763494/)
* 2023-06-01 | Pasadena, CA, US | [Pasadena Thursday Go / Rust](https://www.meetup.com/thursday-go/)
    * [**Weekly leetcode group (Go and Rust)**](https://www.meetup.com/thursday-go/events/293777753)
* 2023-06-07 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/293730065)
* 2023-06-08 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Rust 1.70.0, Module System Deep Dive & Pizza**](https://www.meetup.com/utah-rust/events/293849386/)

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

> Panics are overgrown ASSERTs, not an underbuilt exception system.

– [Stephan Sokolow on hacker news](https://news.ycombinator.com/item?id=36104811)

Thanks to [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1430) for the self-suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
