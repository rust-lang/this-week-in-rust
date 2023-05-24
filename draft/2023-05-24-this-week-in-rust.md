Title: This Week in Rust 496
Number: 496
Date: 2023-05-24
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

* [Update-informer v1.0.0](https://github.com/mgrachev/update-informer/releases/tag/v1.0.0)
* [Pavex, a new Rust web framework - #4: Performance is a feature](https://www.lpalmieri.com/posts/pavex-progress-report-04/)
* [Opensourcing Whichlang, a fast language detection library for Rust](https://quickwit.io/blog/whichlang-language-detection-library)

### Observations/Thoughts

### Rust Walkthroughs

* [A guide to closures in Rust](https://hashrust.com/blog/a-guide-to-closures-in-rust/)
* [Guide to parsing with nom](https://developerlife.com/2023/02/20/guide-to-nom-parsing/)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [whichlang](https://github.com/quickwit-oss/whichlang), a fast no-dependencies OSS natural language detector.

Thanks to [Brian Kung](https://users.rust-lang.org/t/crate-of-the-week/2704/1199) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [bilge - Allow others to define their own `-Bits` derives](https://github.com/hecatia-elegua/bilge/issues/11)
* [Hyperswitch - Implement `CardsInfoInterface` for `MockDb`](https://github.com/juspay/hyperswitch/issues/1256)
* [Hyperswitch - Implement `DisputeInterface` for `MockDb`](https://github.com/juspay/hyperswitch/issues/1257)
* [Hyperswitch - Unite payment intent and setup intent in stripe compatibility](https://github.com/juspay/hyperswitch/issues/1242)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

314 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-05-15..2023-05-22

* [CFI: fix `encode_ty`: unexpected `Param(B/#1)`](https://github.com/rust-lang/rust/pull/111697)
* [`diagnostic`: wrap parens for ref impl trait param](https://github.com/rust-lang/rust/pull/111610)
* [resolve: only disambiguate binding key during define](https://github.com/rust-lang/rust/pull/110493)
* [add `--remote-time` flag to curl for bootstrap](https://github.com/rust-lang/rust/pull/111771)
* [add a conversion from `&mut T` to `&mut UnsafeCell<T>`](https://github.com/rust-lang/rust/pull/111654)
* [add creation time support to `FileTimes` on apple and windows](https://github.com/rust-lang/rust/pull/109773)
* [add derive for `core::marker::ConstParamTy`](https://github.com/rust-lang/rust/pull/111649)
* [always fall back to PartialEq when a constant in a pattern is not recursively structural-eq](https://github.com/rust-lang/rust/pull/105750)
* [better diagnostic for `use Self::..`](https://github.com/rust-lang/rust/pull/111652)
* [debug format `Const`'s less verbosely](https://github.com/rust-lang/rust/pull/111630)
* [do not recover when parsing stmt in cfg-eval](https://github.com/rust-lang/rust/pull/111054)
* [don't expect normalization to succeed in `elaborate_drops`](https://github.com/rust-lang/rust/pull/110930)
* [don't inline functions with unsized args](https://github.com/rust-lang/rust/pull/111424)
* [don't skip inference for type in `offset_of!`](https://github.com/rust-lang/rust/pull/111696)
* [don't use inner macro in `marker_impls`](https://github.com/rust-lang/rust/pull/111810)
* [dont check `must_use` on nested `impl Future` from fn](https://github.com/rust-lang/rust/pull/111491)
* [erase `ReError` properly](https://github.com/rust-lang/rust/pull/111573)
* [erase regions of type in `offset_of!`](https://github.com/rust-lang/rust/pull/111661)
* [exclude inherent projections from some alias type `match`es](https://github.com/rust-lang/rust/pull/111695)
* [fix dependency tracking for debugger visualizers](https://github.com/rust-lang/rust/pull/111641)
* [fix doc comment for `ConstParamTy` derive](https://github.com/rust-lang/rust/pull/111715)
* [fix duplicate `arcinner_layout_for_value_layout` calls when using the uninit `Arc` constructors](https://github.com/rust-lang/rust/pull/111634)
* [fix local libs not included when printing native static libs](https://github.com/rust-lang/rust/pull/111675)
* [fix overflow in error emitter](https://github.com/rust-lang/rust/pull/111745)
* [fix recursion depth handling after confirmation](https://github.com/rust-lang/rust/pull/111754)
* [fix: emit error when fragment is `MethodReceiverExpr` and items is empty](https://github.com/rust-lang/rust/pull/111762)
* [get current target config from `--print=cfg`](https://github.com/rust-lang/rust/pull/111472)
* [give a more useful location for where a `span_bug` was delayed](https://github.com/rust-lang/rust/pull/111708)
* [give better error when collecting into `&[T]`](https://github.com/rust-lang/rust/pull/111612)
* [handle error body in generator layout](https://github.com/rust-lang/rust/pull/111533)
* [improve cgu merging debug output](https://github.com/rust-lang/rust/pull/111743)
* [keep only the trait when emitting the error for `MyTrait + 'a`](https://github.com/rust-lang/rust/pull/111731)
* [merge return place with other locals in CopyProp](https://github.com/rust-lang/rust/pull/111556)
* [merge some query impl modules into one](https://github.com/rust-lang/rust/pull/111746)
* [move `rustc_middle/src/ty/query.rs` to `rustc_middle/src/query/plumbing.rs`](https://github.com/rust-lang/rust/pull/111625)
* [only depend on `CFG_VERSION` in `rustc_interface`](https://github.com/rust-lang/rust/pull/111345)
* [optimize `next_chunk` impls for Filter and FilterMap](https://github.com/rust-lang/rust/pull/111781)
* [process current bucket instead of parent's bucket when starting loop for dominators](https://github.com/rust-lang/rust/pull/111596)
* [recover `impl<T ?Sized>` correctly](https://github.com/rust-lang/rust/pull/111449)
* [rename `{drop,forget}_{copy,ref}` lints to more consistent naming](https://github.com/rust-lang/rust/pull/111756)
* [replace `QueryStruct` with arrays local to `rustc_query_impl`](https://github.com/rust-lang/rust/pull/111808)
* [shorten backtraces for queries in ICEs](https://github.com/rust-lang/rust/pull/108938)
* [shorten even more panic temporary lifetimes](https://github.com/rust-lang/rust/pull/111590)
* [specialize `ToString` implementation for `fmt::Arguments`](https://github.com/rust-lang/rust/pull/111168)
* [specialize query execution for incremental and non-incremental](https://github.com/rust-lang/rust/pull/108062)
* [support PGO on custom project](https://github.com/rust-lang/rust/pull/110605)
* [support RISC-V unaligned-scalar-mem target feature](https://github.com/rust-lang/rust/pull/110884)
* [suppress "erroneous constant used" for constants tainted by errors](https://github.com/rust-lang/rust/pull/111602)
* [use error term in projection if missing associated item in new solver](https://github.com/rust-lang/rust/pull/111488)
* [add the weak-intrinsics feature](https://github.com/rust-lang/compiler-builtins/pull/526)
* [stabilize feature `cstr_is_empty`](https://github.com/rust-lang/rust/pull/111043)
* [stabilize feature `nonzero_negation_ops`](https://github.com/rust-lang/rust/pull/111044)
* [constify `slice_as_chunks` (unstable)](https://github.com/rust-lang/rust/pull/111453)
* [use code with reliable branchless code-gen for `slice::sort` merge](https://github.com/rust-lang/rust/pull/111646)
* [`ascii::Char`-ify the escaping code in core](https://github.com/rust-lang/rust/pull/111524)
* [hashbrown: add NEON backend for RawTable](https://github.com/rust-lang/hashbrown/pull/430)
* [hashbrown: add support for allocator-api2](https://github.com/rust-lang/hashbrown/pull/417)
* [regex syntax: fix overflow for big counted repetitions](https://github.com/rust-lang/regex/pull/996)
* [cargo: `lints` feature](https://github.com/rust-lang/cargo/pull/12148)
* [cargo: pass `-C debuginfo` after weakening if explicitly set](https://github.com/rust-lang/cargo/pull/12165)
* [rustdoc: hide repr attribute from doc of types without guaranteed repr](https://github.com/rust-lang/rust/pull/107680)
* [rustdoc: include strikethrough in item summary](https://github.com/rust-lang/rust/pull/111824)
* [rustdoc: Only keep impl blocks from bodies](https://github.com/rust-lang/rust/pull/111642)
* [clippy: add `minimal_cfg_condition` lint](https://github.com/rust-lang/rust-clippy/pull/10763)
* [clippy: `SpanlessEq` improvements](https://github.com/rust-lang/rust-clippy/pull/10478)
* [clippy: `match_wild_err_arm`: do not lint in const contexts](https://github.com/rust-lang/rust-clippy/pull/10811)
* [clippy: `redundant_pattern_matching`: check for single-arm match](https://github.com/rust-lang/rust-clippy/pull/10753)
* [clippy: `dbg_macro`: don't remove `dbg!` in arbitrary expressions](https://github.com/rust-lang/rust-clippy/pull/10725)
* [clippy: don't suggest unnameable types in `box_default`, `let_underscore_untyped`](https://github.com/rust-lang/rust-clippy/pull/10798)
* [clippy: enhance `needless_collect`: lint in method/function arguments that take an `IntoIterator`](https://github.com/rust-lang/rust-clippy/pull/10777)
* [clippy: fix `invalid_regex` not recognizing new syntax introduced after regex-1.8.0](https://github.com/rust-lang/rust-clippy/pull/10682)
* [clippy: fix some suggestions generated by the `option_if_let_else` lint](https://github.com/rust-lang/rust-clippy/pull/10337)
* [clippy: ignoring `let_underscore_untyped` warnings in code from proc macros](https://github.com/rust-lang/rust-clippy/pull/10775)
* [clippy: rename `integer_arithmetic`](https://github.com/rust-lang/rust-clippy/pull/10674)
* [rust-analyzer: consider block impls in `lookup_impl_assoc_item_for_trait_ref`](https://github.com/rust-lang/rust-analyzer/pull/14855)
* [rust-analyzer: expand `format_args!` with more details](https://github.com/rust-lang/rust-analyzer/pull/14820)
* [rust-analyzer: add `moved-out-of-ref` diagnostic](https://github.com/rust-lang/rust-analyzer/pull/14789)
* [rust-analyzer: highlight used trait assoc items when cursor is on trait import or trait bound](https://github.com/rust-lang/rust-analyzer/pull/14812)
* [rust-analyzer: render hover actions for closure captures and sig](https://github.com/rust-lang/rust-analyzer/pull/14811)
* [rust-analyzer: support C string literals](https://github.com/rust-lang/rust-analyzer/pull/14837)
* [rust-analyzer: consider all tokens in macro expr when analyzing locals](https://github.com/rust-lang/rust-analyzer/pull/14863)
* [rust-analyzer: fix `preorder_expr` skipping the `else` block of let-else statements](https://github.com/rust-lang/rust-analyzer/pull/14848)
* [rust-analyzer: fix evaluating negation for floating point types](https://github.com/rust-lang/rust-analyzer/pull/14825)
* [rust-analyzer: handle match scrutinee in closure captures](https://github.com/rust-lang/rust-analyzer/pull/14851)
* [rust-analyzer: introduce new type var when expectation for ref pat is not ref](https://github.com/rust-lang/rust-analyzer/pull/14872)
* [rust-analyzer: place type inlay hints after the item and without left-padding](https://github.com/rust-lang/rust-analyzer/pull/14818)
* [rust-analyzer: process `macro_use` prelude in semantic scope resolver](https://github.com/rust-lang/rust-analyzer/pull/14828)

### Rust Compiler Performance Triage

There were a few regressions, but most were expected, and one in particular (PR
#111807) is expected yield gains in object code performance at the expense of a
slight compile-time hit. There are a couple PR's that need future followup,
namely PRs #111364 and #111524.

Triage done by **@pnkfelix**.
Revision range: [3ea9ad53..cda5becc](https://perf.rust-lang.org/?start=3ea9ad532474343426e564b997891e459cda89a6&end=cda5becc27cbc7106646fbc40aacea5e7896d954&absolute=false&stat=instructions%3Au)
 
3 Regressions, 2 Improvements, 5 Mixed; 2 of them in rollups
51 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-05-23.md)

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

Rusty Events between 2023-05-24 - 2023-06-21 🦀

### Virtual

* 2023-05-25 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Practical Monads**](https://www.meetup.com/charlottesville-rust-meetup/events/293384348)
* 2023-05-25 | Virtual (Ciudad de México, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Proyecto "Taller de Rust"**](https://www.meetup.com/rust-mx/events/293332410)
* 2023-05-25 | Virtual (Karlsruhe, DE) | [The Karlsruhe Functional Programmers Meetup Group](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/)
    * [**Stammtisch (gemeinsam mit der C++ UG KA) - various topics, from C++ to Rust**](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/events/293349464)
* 2023-05-25 | Virtual (Raleigh, NC, US) | [Triangle BitDevs](https://www.meetup.com/triangle-bitdevs/)
    * [**Rust for Bitcoiners**](https://www.meetup.com/triangle-bitdevs/events/293547054)
* 2023-05-25 | Virtual (San Francisco, CA, US) | [Data + AI Online Meetup](https://www.meetup.com/data-ai-online/)
    * [**D3L2: Discussing Rust, Ballista, Ray SQL, DataFusion with Andy Grove**](https://www.meetup.com/data-ai-online/events/293432877)
* 2023-05-30 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/293014934)
* 2023-05-31 | Virtual (Chicago, IL, US) | [Chicago Healthcare Cloud Technology Community](https://www.meetup.com/chicago-healthcare-tech-and-ai/)
    * [**Rust for Mission-Critical AI: A Journey into Healthcare's Safest Language**](https://www.meetup.com/chicago-healthcare-tech-and-ai/events/293278396/?chapterContext=true&regToRsvp=true&isFromReg=true)
* 2023-06-06 | Virtual (Austin, TX, US) | [WebAssembly and WasmEdge](https://www.meetup.com/webassembly-and-wasmedge/)
    * [**Monthly WasmEdge Community Meeting - Run Rust Warp in WasmEdge -- Alan, Poon Yong Quan**](https://www.meetup.com/webassembly-and-wasmedge/events/293014949)
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
* 2023-06-20 | Virtual (Berlin, DE) | [Berline.rs / OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/293485510)
* 2023-06-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/jkxsctyfcjbbc/)
* 2023-06-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763484)

### Asia

* 2023-05-25 | Amsterdam, NL | [Frontend Developer Meetup Amsterdam](https://www.meetup.com/frontend-developer-meetup-amsterdam/)
    * [**Svelte Frontend Meetup (signup required) - Building a Svelte-Rust app using Tauri**](https://www.meetup.com/frontend-developer-meetup-amsterdam/events/293272364)
* 2023-06-10 | Kuala Lumpur, MY | [GoLang Malaysia](https://t.me/golangmalaysia)
    * [**Rust Workshop/Hack and Learn Malaysia June 2023**](https://forms.gle/2fvbCG77HXCkWLfe6) | [Event updates Telegram](https://t.me/+dF46Fly4A_BjOTJl) | [Event group chat](https://t.me/golangmalaysia)

### Europe

* 2023-05-24 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #4**](https://www.meetup.com/fr-FR/rust-lyon/events/293322211)
* 2023-05-25 | Barcelona, ES | [C++ Programmer Meetup.](https://www.meetup.com/c-programmer-meetup/)
    * [**Rust for C++ Developers.**](https://www.meetup.com/c-programmer-meetup/events/292816507)
* 2023-05-25 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #36 at Adapt Agency!**](https://www.meetup.com/copenhagen-rust-community/events/293293863)
* 2023-05-25 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #59**](https://www.meetup.com/rust-paris/events/293191172)
* 2023-05-30 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/)
    * [**10th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/293363107)
* 2023-06-03 | Plovidv, BG | [AeroRust](https://aerorust.org)
    * [**Space Conference - Plovdiv**](https://space-conference-plovdiv.eventbrite.com)
* 2023-06-04 | Plovidv, BG | [AeroRust](https://aerorust.org)
    * [**Space Conference : Nanosatellite embedded workshop**](https://forms.gle/rnFx5wCbrYA9qN7m6)
* 2023-06-08 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [****](https://www.meetup.com/rust-aarhus/events/292865970/)
* 2023-06-08 | Zurich, CH | [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**Unsafe, Miri, SIMD - June Meetup**](https://www.meetup.com/de-DE/rust-zurich/events/293322792/)

### Oceania

* 2023-05-30 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**May Meetup**](https://www.meetup.com/rust-canberra/events/292717772/)

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

> I guess the nicest example of this phenomenon is shared mutability. Programmers have been arguing for decades whether it is sharing xor mutability that causes memory safety bugs:
>
> * "It's threads!" – shouted JavaScript and Python, and JS remained single-threaded, and Python introduced the GIL.
> * "It's mutability!" – screamed Haskell and Erlang, and they made (almost) everything immutable.
>
> And then along came Rust, and said: "you are fools! You can have both sharing and mutability in the same language, as long as you isolate them from each other."

– [H2CO3 on rust-users](https://users.rust-lang.org/t/is-copy-on-enums-ok/94128/12)

Thanks to [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1425) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
