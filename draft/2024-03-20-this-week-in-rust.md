Title: This Week in Rust 539
Number: 539
Date: 2024-03-20
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

This week's crate is [heck](https://docs.rs/heck), a `no_std` crate to perform case conversions.

Thanks to [Edoardo Morandi](https://users.rust-lang.org/t/crate-of-the-week/2704/1295) for the suggestion!

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
* [RustFest Zürich 2024](https://rustfest.ch/cfp/) | Closes 2024-03-31 | Zürich, Switzerland | Event date: 2024-06-19 - 2024-06-24
* [Oxidize 2024](https://pretalx.com/oxidize-berlin-2024/cfp) | Closes 2024-03-24 | Berlin, Germany | Event date: 2024-05-28 - 2024-05-30
* [RustConf 2024](https://foundation.rust-lang.org/news/the-rustconf-2024-call-for-talk-proposals-is-open/) | Closes 2024-04-25 | Montreal, Canada | Event date: 2024-09-10
* [EuroRust 2024](https://www.papercall.io/eurorust-2024)| Closes 2024-06-03 | Vienna, Austria & online | Event on 2024-10-10
* [Scientific Computing in Rust 2024](https://scientificcomputing.rs/)| Closes 2024-06-14 | online | Event date: 2024-07-17 - 2024-07-19

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

498 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-03-12..2024-03-19

* [BOLT Use CDSort and CDSplit](https://github.com/rust-lang/rust/pull/119418)
* [`NormalizesTo`: return nested goals to caller](https://github.com/rust-lang/rust/pull/122687)
* [`add_retag`: ensure box-to-raw-ptr casts are preserved for Miri](https://github.com/rust-lang/rust/pull/122647)
* [`f16` and `f128` step 3: compiler support & feature gate](https://github.com/rust-lang/rust/pull/121926)
* [add `-Z external-clangrt`](https://github.com/rust-lang/rust/pull/121207)
* [add `wasm_c_abi future-incompat` lint](https://github.com/rust-lang/rust/pull/117918)
* [add missing `try_visit` calls in visitors](https://github.com/rust-lang/rust/pull/122689)
* [check library crates for all tier 1 targets in PR CI](https://github.com/rust-lang/rust/pull/122401)
* [copy byval argument to alloca if alignment is insufficient](https://github.com/rust-lang/rust/pull/122212)
* [coverage: initial support for branch coverage instrumentation](https://github.com/rust-lang/rust/pull/122322)
* [create some minimal HIR for associated opaque types](https://github.com/rust-lang/rust/pull/120943)
* [delay expand macro bang when there has indeterminate path](https://github.com/rust-lang/rust/pull/121589)
* [delegation: fix ICE on duplicated associative items](https://github.com/rust-lang/rust/pull/122564)
* [detect allocator for box in `must_not_suspend` lint](https://github.com/rust-lang/rust/pull/122701)
* [detect calls to `.clone()` on T: !Clone types on borrowck errors](https://github.com/rust-lang/rust/pull/122254)
* [detect when move of !Copy value occurs within loop and should likely not be cloned](https://github.com/rust-lang/rust/pull/121652)
* [diagnostics: suggest `Clone` bounds when noop `clone()`](https://github.com/rust-lang/rust/pull/122174)
* [do not eat nested expressions' results in `MayContainYieldPoint` format args visitor](https://github.com/rust-lang/rust/pull/122680)
* [don't create `ParamCandidate` when obligation contains errors](https://github.com/rust-lang/rust/pull/122360)
* [don't ICE when non-self part of trait goal is constrained in new solver](https://github.com/rust-lang/rust/pull/122319)
* [don't show suggestion if slice pattern is not top-level](https://github.com/rust-lang/rust/pull/121236)
* [downgrade const eval dangling ptr in final to future incompat lint](https://github.com/rust-lang/rust/pull/122204)
* [enable PR tracking review assignment for rust-lang/rust](https://github.com/rust-lang/rust/pull/122383)
* [enable creating backtraces via -Ztreat-err-as-bug when stashing errors](https://github.com/rust-lang/rust/pull/122194)
* [enable frame pointers for the standard library](https://github.com/rust-lang/rust/pull/122646)
* [ensure RPITITs are created before def-id freezing](https://github.com/rust-lang/rust/pull/122523)
* [fix 32-bit overflows in LLVM composite constants](https://github.com/rust-lang/rust/pull/122000)
* [fix ICE in diagnostics for parenthesized type arguments](https://github.com/rust-lang/rust/pull/122400)
* [fix `long-linker-command-lines` failure caused by `rust.rpath=false`](https://github.com/rust-lang/rust/pull/122270)
* [fix attribute validation on associated items in traits](https://github.com/rust-lang/rust/pull/121545)
* [fix stack overflow with recursive associated types](https://github.com/rust-lang/rust/pull/122366)
* [interpret: ensure that Place is never used for a different frame](https://github.com/rust-lang/rust/pull/122243)
* [make incremental sessions identity no longer depend on the crate names provided by source code](https://github.com/rust-lang/rust/pull/121764)
* [match lowering: don't collect test alternatives ahead of time](https://github.com/rust-lang/rust/pull/121908)
* [more eagerly instantiate binders](https://github.com/rust-lang/rust/pull/119849)
* [never patterns: suggest `!` patterns on non-exhaustive matches](https://github.com/rust-lang/rust/pull/121823)
* [only generate a ptrtoint in AtomicPtr codegen when absolutely necessary](https://github.com/rust-lang/rust/pull/122220)
* [only invoke `decorate` if the diag can eventually be emitted](https://github.com/rust-lang/rust/pull/122578)
* [pass the correct DefId when suggesting writing the aliased Self type out](https://github.com/rust-lang/rust/pull/122515)
* [pattern analysis: Store field indices in `DeconstructedPat` to avoid virtual wildcards](https://github.com/rust-lang/rust/pull/121820)
* [provide structured suggestion for `#![feature(foo)]`](https://github.com/rust-lang/rust/pull/122158)
* [register LLVM handlers for bad-alloc / OOM](https://github.com/rust-lang/rust/pull/122574)
* [reject overly generic assoc const binding types](https://github.com/rust-lang/rust/pull/121258)
* [represent `Result<usize, Box<T>>` as ScalarPair(i64, ptr)](https://github.com/rust-lang/rust/pull/121668)
* [split `refining_impl_trait` lint into `_reachable, _internal` variants](https://github.com/rust-lang/rust/pull/121720)
* [stabilize `imported_main`](https://github.com/rust-lang/rust/pull/122060)
* [stabilize associated type bounds](https://github.com/rust-lang/rust/pull/122055) (RFC [#2289](https://rust-lang.github.io/rfcs/2289-associated-type-bounds.html))
* [stop walking the bodies of statics for reachability, and evaluate them instead](https://github.com/rust-lang/rust/pull/122371)
* [ungate the `UNKNOWN_OR_MALFORMED_DIAGNOSTIC_ATTRIBUTES` lint](https://github.com/rust-lang/rust/pull/122482)
* [unix time module now return result](https://github.com/rust-lang/rust/pull/114038)
* [validate `builder::PATH_REMAP`](https://github.com/rust-lang/rust/pull/122081)
* [miri: add some chance to reuse addresses of previously freed allocations](https://github.com/rust-lang/rust/pull/122240)
* [avoid lowering code under dead SwitchInt targets](https://github.com/rust-lang/rust/pull/121421)
* [use `UnsafeCell` for fast constant thread locals](https://github.com/rust-lang/rust/pull/122583)
* [add `CStr::bytes` iterator](https://github.com/rust-lang/rust/pull/104353)
* [add `as_(mut_)ptr` and `as_(mut_)slice` to raw array pointers](https://github.com/rust-lang/rust/pull/119411)
* [implement `{Div,Rem}Assign<NonZero<X>>` on `X`](https://github.com/rust-lang/rust/pull/121952)
* [fix unsoundness in `Step::forward_unchecked` for signed integers](https://github.com/rust-lang/rust/pull/122461)
* [implement `Duration::as_millis_{f64,f32}`](https://github.com/rust-lang/rust/pull/122479)
* [optimize `ptr::replace`](https://github.com/rust-lang/rust/pull/122601)
* [safe Transmute: Require that source referent is smaller than destination](https://github.com/rust-lang/rust/pull/122438)
* [safe Transmute: Use 'not yet supported', not 'unspecified' in errors](https://github.com/rust-lang/rust/pull/122560)
* [hashbrown: fix index calculation in panic guard of `clone_from_impl`](https://github.com/rust-lang/hashbrown/pull/511)
* [cargo tree: Control `--charset` via auto-detecting config value](https://github.com/rust-lang/cargo/pull/13337)
* [cargo toml: Flatten manifest parsing](https://github.com/rust-lang/cargo/pull/13589)
* [cargo: add 'open-namespaces' feature](https://github.com/rust-lang/cargo/pull/13591)
* [cargo fix: strip feature dep when dep is dev dep](https://github.com/rust-lang/cargo/pull/13518)
* [cargo: prevent dashes in lib.name](https://github.com/rust-lang/cargo/pull/12783)
* [cargo: expose source/spans to Manifest for emitting lints](https://github.com/rust-lang/cargo/pull/13593)
* [rustdoc-search: depth limit `T<U>` → `U` unboxing](https://github.com/rust-lang/rust/pull/122247)
* [rustdoc-search: search types by higher-order functions](https://github.com/rust-lang/rust/pull/119676)
* [rustdoc: add `--test-builder-wrapper` arg to support wrappers such as `RUSTC_WRAPPER` when building doctests](https://github.com/rust-lang/rust/pull/114651)
* [rustdoc: do not preload fonts when browsing locally](https://github.com/rust-lang/rust/pull/122410)
* [rustfmt: fix: ICE with expanded code](https://github.com/rust-lang/rustfmt/pull/6112)
* [rustfmt: initial work on formatting headers](https://github.com/rust-lang/rustfmt/pull/5847)
* [clippy: `cast_lossless`: Suggest type alias instead of the aliased type](https://github.com/rust-lang/rust-clippy/pull/11287)
* [clippy: `else_if_without_else`: Fix duplicate diagnostics](https://github.com/rust-lang/rust-clippy/pull/12441)
* [clippy: `map_entry`: call the visitor on the local's `else` block](https://github.com/rust-lang/rust-clippy/pull/12498)
* [clippy: `option_option`: Fix duplicate diagnostics](https://github.com/rust-lang/rust-clippy/pull/12450)
* [clippy: `unused_enumerate_index`: trigger on method calls](https://github.com/rust-lang/rust-clippy/pull/12432)
* [clippy: `use_self`: Make it aware of lifetimes](https://github.com/rust-lang/rust-clippy/pull/12386)
* [clippy: don't emit `doc_markdown` lint for missing backticks if it's inside a quote](https://github.com/rust-lang/rust-clippy/pull/12472)
* [clippy: fix `dbg_macro` false negative when dbg is inside some complex macros](https://github.com/rust-lang/rust-clippy/pull/12482)
* [clippy: fix `empty_docs` trigger in proc-macro](https://github.com/rust-lang/rust-clippy/pull/12466)
* [clippy: fix span calculation for non-ascii in `needless_return`](https://github.com/rust-lang/rust-clippy/pull/12493)
* [clippy: handle false positive with `map_clone` lint](https://github.com/rust-lang/rust-clippy/pull/12282)
* [clippy: lint when calling the blanket `Into` impl from a `From` impl](https://github.com/rust-lang/rust-clippy/pull/12459)
* [clippy: move `iter_nth` to `style`, add machine applicable suggestion](https://github.com/rust-lang/rust-clippy/pull/12417)
* [clippy: move `readonly_write_lock` to perf](https://github.com/rust-lang/rust-clippy/pull/12479)
* [clippy: new restriction lint: `integer_division_remainder_used`](https://github.com/rust-lang/rust-clippy/pull/12451)
* [rust-analyzer: distinguish integration tests from crates in test explorer](https://github.com/rust-lang/rust-analyzer/pull/16847)
* [rust-analyzer: apply `#[cfg]` to proc macro inputs](https://github.com/rust-lang/rust-analyzer/pull/16789)
* [rust-analyzer: implement ATPIT](https://github.com/rust-lang/rust-analyzer/pull/16852)
* [rust-analyzer: support macro calls in eager macros for IDE features](https://github.com/rust-lang/rust-analyzer/pull/16834)
* [rust-analyzer: syntax highlighting improvements](https://github.com/rust-lang/rust-analyzer/pull/16860)
* [rust-analyzer: fix panic with impl trait associated types in where clause](https://github.com/rust-lang/rust-analyzer/pull/16830)
* [rust-analyzer: don't auto-close block comments in strings](https://github.com/rust-lang/rust-analyzer/pull/16820)
* [rust-analyzer: fix wrong where clause rendering on hover](https://github.com/rust-lang/rust-analyzer/pull/16856)
* [rust-analyzer: handle attributes when typing curly bracket](https://github.com/rust-lang/rust-analyzer/pull/16868)
* [rust-analyzer: ignore some warnings if they originate from within macro expansions](https://github.com/rust-lang/rust-analyzer/pull/16861)
* [rust-analyzer: incorrect handling of `use` and panic issue in `extract_module`](https://github.com/rust-lang/rust-analyzer/pull/16846)
* [rust-analyzer: make inlay hint resolving work better for inlays targetting the same position](https://github.com/rust-lang/rust-analyzer/pull/16822)
* [rust-analyzer: refactor extension to support arbitrary shell command runnables](https://github.com/rust-lang/rust-analyzer/pull/16839)
* [rust-analyzer: show compilation progress in test explorer](https://github.com/rust-lang/rust-analyzer/pull/16845)
* [rust-analyzer: use `--workspace` and `--no-fail-fast` in test explorer](https://github.com/rust-lang/rust-analyzer/pull/16880)

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

Rusty Events between 2024-03-20 - 2024-04-17 🦀

### Virtual

* 2024-03-14 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Web Frontend Co-Learning (online)**](https://www.meetup.com/opentechschool-berlin/events/298406445/)
* 2024-03-14 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298457903/) | [**Mirror: Berline.rs page**](https://berline.rs/2024/03/14/rust-hack-and-learn.html)
* 2024-03-14 | Virtual (Nürnberg, DE) | [Rust Nüremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945252/)
* 2024-03-14 | Virtual (San Diego, CA, US) | [San Diego Rust](https://www.meetup.com/san-diego-rust/)
    * [**San Diego Rust March 2024 Tele-Meetup**](https://www.meetup.com/san-diego-rust/events/299743034/)
* 2024-03-19 | Virtual | [Formal Land](https://github.com/formal-land/)
    * [**Rust for Lunch: Formal verification for Rust with coq-of-rust. Speaker: Guillaume Claret**](https://lecture.senfcall.de/hay-gmh-wox-mru) | [Docs](https://github.com/formal-land/coq-of-rust)
* 2024-03-19 | Virtual (Washinigton, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299335006/)
* 2024-03-20 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 3 - Designing Interfaces**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/299505703/)
* 2024-03-20 | Virtual (Vancouver, BC, CA)| [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763494/)
* 2024-03-21 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368793/)
* 2024-03-26 | Virtual + In Person (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**13th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/299223178/) - [Stream](https://www.youtube.com/@bcnrust)
* 2024-03-26 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/299644917/)
* 2024-03-28 | Virtual + In Person (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298457904/)
* 2024-04-02 | Virtual (Buffalo, NY, US) | [Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/mrnrktygcgbdb/)
* 2024-04-03 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 4 - Error Handling**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/299507234/)
* 2024-04-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047892/)
* 2024-04-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341660/)

### Africa

* 2024-04-05 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia

* 2024-03-30 | New Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/)
    * [**Rust Delhi Meetup #6**](https://www.meetup.com/rustdelhi/events/299771772/)

### Europe

* 2024-03-13 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.com/o/paris-rustaceans-74289178383)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-830340830777)
* 2024-03-14 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/298533419/)
* 2024-03-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/299028814/)
* 2024-03-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Rust Interactive Session**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/299309224/)
* 2024-03-19 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/events/299515169/)
    * [**Rust Meetup @ Charles University**](https://www.meetup.com/rust-prague/events/299515169/)
* 2024-03-20 | Girona, ES | [Rust Girona](https://www.meetup.com/rust-girona/)
    * [**Introduction to programming Microcontrollers with Rust**](https://www.meetup.com/rust-girona/events/299172343/)
* 2024-03-20 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #9**](https://www.meetup.com/fr-FR/rust-lyon/events/299527560/)
* 2024-03-21 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/de-DE/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #6**](https://www.meetup.com/de-DE/rust-meetup-augsburg/events/299354449/)
* 2024-03-21 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #6: Du RSS et de L'ECS !**](https://www.meetup.com/meetup-group-zgphbyet/events/299295547/)
* 2024-03-21 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/)
    * [**Rust Vienna Meetup - March - Unsafe Rust**](https://www.meetup.com/rust-vienna/events/299682390/)
* 2024-03-23 | Stockholm, SE | [Ferris' Fika Forum](https://www.google.com/calendar/event?eid=NWMzaDNqZDZjcG5oZXNwYzJycHRzMXI5djUgYXBkOXZtYmMyMmVnZW5tdHU1bDZjNWpiZmNAZw&ctz=America/Los_Angeles)
    * [**Ferris' Fika Forum**](https://www.google.com/calendar/event?eid=NWMzaDNqZDZjcG5oZXNwYzJycHRzMXI5djUgYXBkOXZtYmMyMmVnZW5tdHU1bDZjNWpiZmNAZw&ctz=America/Los_Angeles) | [Map](https://maps.google.com/maps?hl=en&q=Starbucks%2C%20Hamngatan%2033%2C%20111%2047%20Stockholm%2C%20Sweden)
* 2024-03-26 | Barcelona, ES + Virtual | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**13th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/299223178/)
* 2024-03-26 - 2024-03-28 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation 2024 - Conference**](https://www.rustnationuk.com/)
* 2024-03-28 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell**](https://www.meetup.com/rust-berlin/events/299288961/)
* 2024-04-10 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Rust Meetup Reboot 3**](https://www.meetup.com/cambridge-rust-meetup/events/299730322/)
* 2024-04-10 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/299488225/)

### North America

* 2024-03-13 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Northeastern Rust Lunch**](https://www.meetup.com/bostonrust/events/299262009/)
* 2024-03-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186823/)
* 2024-03-21 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/298631832/)
* 2024-03-22 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Somerville Union Square Rust Lunch, Mar 22**](https://www.meetup.com/bostonrust/events/299262036/)
* 2024-03-26 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust: Getting started with Rust!**](https://www.meetup.com/minneapolis-rust-meetup/events/299489274/)
* 2024-03-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/299220136/)
* 2024-03-27 | Hawthorne, CA, US | [Freeform](https://freeform.co/)
    * [**Rust in the Physical World 🦀 Tech Talk Event at Freeform**](https://freeformxrust.rsvpify.com/)
* 2024-03-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Beacon Hill Rust Lunch, Mar 31**](https://www.meetup.com/bostonrust/events/299262047/)

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

> In 10 years we went from “Rust will never replace C and C++” to “New C/C++ should not be written anymore, and you should use Rust”. Good job.

– [dpc_pw on lobste.rs](https://lobste.rs/s/dbwn98/secure_by_design_google_s_perspective_on#c_f6j9ok)

Thanks to [Dennis Luxen](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1547) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
