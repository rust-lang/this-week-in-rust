Title: This Week in Rust 506
Number: 506
Date: 2023-08-02
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

This week's crate is [allocator-api2](https://docs.rs/allocator-api2), a forward-compatibility shim to use the nightly allocator API on stable Rust.

Thanks to [Zicklag](https://users.rust-lang.org/t/crate-of-the-week/2704/1215) for the suggestion!

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

404 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-07-24..2023-07-31

* [add `x86_64-unikraft-linux-musl` target](https://github.com/rust-lang/rust/pull/113411)
* [CFI: fix ICE: `encode_const`: unexpected type usize](https://github.com/rust-lang/rust/pull/113708)
* [data\_structures: Simplify `binary_search_slice`](https://github.com/rust-lang/rust/pull/114152)
* [`builtin_macros`: expect raw strings too](https://github.com/rust-lang/rust/pull/114014)
* [`codegen_ssa` cleanups](https://github.com/rust-lang/rust/pull/113879)
* [`desugar_doc_comments` cleanups](https://github.com/rust-lang/rust/pull/114081)
* [resolve: report unresolved imports firstly](https://github.com/rust-lang/rust/pull/113920)
* [resolve: skip panic when resolution is dummy](https://github.com/rust-lang/rust/pull/113980)
* [abi: unsized field in union - assert to delay bug](https://github.com/rust-lang/rust/pull/114060)
* [add Param and Bound ty to SMIR](https://github.com/rust-lang/rust/pull/113930)
* [add `simd_bswap, simd_bitreverse, simd_ctlz,` and `simd_cttz` intrinsics](https://github.com/rust-lang/rust/pull/114156)
* [adjust spans correctly for fn → method suggestion](https://github.com/rust-lang/rust/pull/114138)
* [change LLVM BOLT flags for nice space savings](https://github.com/rust-lang/rust/pull/114141)
* [check for `<&NotClone as Clone>::clone()` calls and suggest to add Clone trait appropriately](https://github.com/rust-lang/rust/pull/112995)
* [check lazy type aliases for well-formedness](https://github.com/rust-lang/rust/pull/114228)
* [define `CMAKE_SYSTEM_NAME` on a cross build targeting DragonFly](https://github.com/rust-lang/rust/pull/113996)
* [diagnostic namespace](https://github.com/rust-lang/rust/pull/111780)
* [discard default auto trait impls if explicit ones exist](https://github.com/rust-lang/rust/pull/113312)
* [don't attempt to compute layout of type referencing error](https://github.com/rust-lang/rust/pull/113773)
* [don't install default projection bound for return-position `impl Trait` in trait methods with no body](https://github.com/rust-lang/rust/pull/113741)
* [don't say that a type is uncallable if its fn signature has errors in it](https://github.com/rust-lang/rust/pull/113578)
* [don't treat negative trait predicates as always knowable](https://github.com/rust-lang/rust/pull/114080)
* [dont pass `-Zwrite-long-types-to-disk=no` for `ui-fulldeps --stage=1`](https://github.com/rust-lang/rust/pull/114102)
* [double check that hidden types match the expected hidden type](https://github.com/rust-lang/rust/pull/113661)
* [effects: don't print `host` param in diagnostics](https://github.com/rust-lang/rust/pull/114203)
* [fix `simd_bswap` for i8/u8](https://github.com/rust-lang/rust/pull/114266)
* [fix intra-doc links on nested `use` and `extern crate` items](https://github.com/rust-lang/rust/pull/113958)
* [fix invalid suggestion for mismatched types in closure arguments](https://github.com/rust-lang/rust/pull/114256)
* [fix missing attribute merge on glob foreign re-exports](https://github.com/rust-lang/rust/pull/114012)
* [gracefully handle ternary operator](https://github.com/rust-lang/rust/pull/114028)
* [hide `ToString` implementations that specialize the default one](https://github.com/rust-lang/rust/pull/114020)
* [implement diagnostic translation for rustc-errors](https://github.com/rust-lang/rust/pull/113281)
* [implement generic const items](https://github.com/rust-lang/rust/pull/113522)
* [improve test case for experimental API `remove_matches`](https://github.com/rust-lang/rust/pull/114111)
* [insert RPITITs that were shadowed by missing ADTs that resolve to type error](https://github.com/rust-lang/rust/pull/114147)
* [interpret: make read/write methods generic](https://github.com/rust-lang/rust/pull/114071)
* [interpret: unify projections for MPlaceTy, PlaceTy, OpTy](https://github.com/rust-lang/rust/pull/114011)
* [less `TokenTree` cloning](https://github.com/rust-lang/rust/pull/114115)
* [lint/ctypes: fix `()` return type checks](https://github.com/rust-lang/rust/pull/113457)
* [make RPITITs inherit the `assumed_wf_types` of their parent method](https://github.com/rust-lang/rust/pull/113704)
* [make `--error-format human-annotate-rs` handle multiple files](https://github.com/rust-lang/rust/pull/114018)
* [make `--print` with path unstable](https://github.com/rust-lang/rust/pull/114139)
* [make `noop_method_call` warn by default](https://github.com/rust-lang/rust/pull/111916)
* [make std tests pass on newer Android](https://github.com/rust-lang/rust/pull/102757)
* [map RPITIT's opaque type bounds back from projections to opaques](https://github.com/rust-lang/rust/pull/114267)
* [mark `lazy_type_alias` as incomplete](https://github.com/rust-lang/rust/pull/114222)
* [new unstable option: -Zwrite-long-types-to-disk](https://github.com/rust-lang/rust/pull/113893)
* [normalize the RHS of an `Unsize` goal in the new solver](https://github.com/rust-lang/rust/pull/113393)
* [only golden arches](https://github.com/rust-lang/rust/pull/114225)
* [optimize `TokenKind::clone`](https://github.com/rust-lang/rust/pull/114119)
* [print omitted frames count for short backtrace mode](https://github.com/rust-lang/rust/pull/112843)
* [privacy: no nominal visibility for assoc fns](https://github.com/rust-lang/rust/pull/114099)
* [reimplement C-str literals](https://github.com/rust-lang/rust/pull/113476)
* [remove `constness` from `ParamEnv`](https://github.com/rust-lang/rust/pull/114134)
* [replace atty crate with std's IsTerminal](https://github.com/rust-lang/rust/pull/114098)
* [restore region uniquification in the new solver 🎉](https://github.com/rust-lang/rust/pull/114117)
* [skip reporting item name when checking RPITIT GAT's associated type bounds hold](https://github.com/rust-lang/rust/pull/114146)
* [split some functions with many arguments into builder pattern functions](https://github.com/rust-lang/rust/pull/114054)
* [suggest {`Option`,`Result`}`::as_ref()` instead of `cloned()` in some cases](https://github.com/rust-lang/rust/pull/114052)
* [turns out opaque types can have hidden types registered during mir validation](https://github.com/rust-lang/rust/pull/114123)
* [weaken `unnameable_types` lint](https://github.com/rust-lang/rust/pull/114246)
* [when flushing delayed span bugs, write to the ICE dump file even if it doesn't exist](https://github.com/rust-lang/rust/pull/114128)
* [miri: TB: redefine trigger condition for protectors](https://github.com/rust-lang/miri/pull/2993)
* [miri: add some interesting tests for alignment corner cases](https://github.com/rust-lang/miri/pull/3001)
* [miri: fix oversight from new miri-script](https://github.com/rust-lang/miri/pull/3002)
* [miri: refactor tests/utils a bit, and move some FS functions there](https://github.com/rust-lang/miri/pull/2997)
* [miri: rewrite miri script in rust](https://github.com/rust-lang/miri/pull/2909)
* [stabilize `const-weak-new`](https://github.com/rust-lang/rust/pull/95965)
* [const-stablilize `NonNull::as_ref`](https://github.com/rust-lang/rust/pull/102198)
* [optimize `AtomicBool` for target that don't support byte-sized atomics](https://github.com/rust-lang/rust/pull/114034)
* [mark `map_or` as `#[must_use]`](https://github.com/rust-lang/rust/pull/112655)
* [merge functionality of `io::Sink` into `io::Empty`](https://github.com/rust-lang/rust/pull/98154)
* [futures: add `len` method for `UnboundedSender`](https://github.com/rust-lang/futures-rs/pull/2750)
* [cargo: normalize relative git submodule urls with `ssh://`](https://github.com/rust-lang/cargo/pull/12411)
* [cargo: use thiserror for credential provider errors](https://github.com/rust-lang/cargo/pull/12424)
* [rustdoc: fix cross-crate `impl Sized` & `impl ?Sized`](https://github.com/rust-lang/rust/pull/114059)
* [rustdoc: If re-export is private, get the next item until a public one is found or expose the private item directly](https://github.com/rust-lang/rust/pull/113374)
* [rustdoc: Remove unneeded `clone()` calls for `derive_id`](https://github.com/rust-lang/rust/pull/114204)
* [rustfmt: handle `dyn*` syntax when rewriting `ast::TyKind::TraitObject`](https://github.com/rust-lang/rustfmt/pull/5552)
* [clippy: `arc_with_non_send_sync`: no longer lints macro-generated code](https://github.com/rust-lang/rust-clippy/pull/11233)
* [clippy: `arithmetic_side_effects`: ignore paths referring to literals](https://github.com/rust-lang/rust-clippy/pull/11263)
* [clippy: `needless_pass_by_ref_mut`: do not lint if passed as a fn-like argument](https://github.com/rust-lang/rust-clippy/pull/11207)
* [clippy: various `redundant_closure` fixes](https://github.com/rust-lang/rust-clippy/pull/8685)
* [clippy: `slow_vector_initialization`: catch `Vec::new()` followed by `.resize(len, 0)`](https://github.com/rust-lang/rust-clippy/pull/11198)
* [clippy: `unnecessary_find_map`: look for `then_some`](https://github.com/rust-lang/rust-clippy/pull/11261)
* [clippy: `needless_pass_by_ref_mut`: emit note if function is behind a `#[cfg]`](https://github.com/rust-lang/rust-clippy/pull/11226)
* [clippy: new lint `filter_map_bool_then`](https://github.com/rust-lang/rust-clippy/pull/11115)
* [clippy: new lint `readonly_write_lock`](https://github.com/rust-lang/rust-clippy/pull/11210)
* [clippy: now `option_env_unwrap` warns even if a variable isn't set at compiletime](https://github.com/rust-lang/rust-clippy/pull/10759)
* [rust-analyzer: add manual implementation of clone for tuples in mir interpreter](https://github.com/rust-lang/rust-analyzer/pull/15353)
* [rust-analyzer: fix: do not create fn macro calls with non-fn expanders](https://github.com/rust-lang/rust-analyzer/pull/15357)
* [rust-analyzer: fix: expand eager macros to delimited comma separated expression list](https://github.com/rust-lang/rust-analyzer/pull/15361)
* [rust-analyzer: fix: fix bad unwrap in `eager_macro_recur`](https://github.com/rust-lang/rust-analyzer/pull/15356)
* [rust-analyzer: fix: remove another faulty unwrap (expect)](https://github.com/rust-lang/rust-analyzer/pull/15366)
* [rust-analyzer: fixup path fragments upon MBE transcription](https://github.com/rust-lang/rust-analyzer/pull/15360)
* [rust-analyzer: properly infer types with type casts](https://github.com/rust-lang/rust-analyzer/pull/15271)
* [rust-analyzer: runnable env per platform](https://github.com/rust-lang/rust-analyzer/pull/15308)
* [rust-analyzer: show anonymous fn def type as a fn pointer in source code](https://github.com/rust-lang/rust-analyzer/pull/15349)
* [rust-analyzer: support `Self` without field in mir lowering](https://github.com/rust-lang/rust-analyzer/pull/15363)
* [rust-analyzer: write proc-macro server spawn errors to the status text](https://github.com/rust-lang/rust-analyzer/pull/15359)

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

Rusty Events between 2023-08-02 - 2023-08-30 🦀

### Virtual

* 2023-07-26 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**The unreasonable power of combinator APIs**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/294748626)
* 2023-07-27 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/ngnwftyfckbkc/)
* 2023-07-28 | Virtual (Tunis, TN) | [Rust Meetup Tunisia](https://www.meetup.com/rust-tunisia/)
    * [**Rust Meetup Tunisia - Volume I, Number IV**](https://www.meetup.com/rust-tunisia/events/294664236/)
* 2023-07-31 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 31st Edition**](https://www.meetup.com/rust-linz/events/294718541/)
* 2023-08-01 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/zdrzpsyfclbcb/)
* 2023-08-01 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfclbcb/)
* 2023-08-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/294768155)
* 2023-08-02 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/294788420)
* 2023-08-08 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsyfclblb/)
* 2023-08-10 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732653)
* 2023-08-15 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/zdrzpsyfclbtb/)
* 2023-08-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/293553331/)
* 2023-08-16 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/kmhpftyfclbvb/)
* 2023-08-22 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/Rust-Dublin/)
    * [**Rust, Serverless and AWS**](https://www.meetup.com/Rust-Dublin/events/294587280/)

### Asia

* 2023-07-27 | Seoul, KR | [Rust Programming Meetup Seoul](https://www.meetup.com/rust-seoul-meetup/)
    * [**Seoul Rust Meetup**](https://www.meetup.com/rust-seoul-meetup/events/294832771/)
* 2023-07-27 | Seoul, KR | [T-RUST meetup](https://www.meetup.com/t-rust-meetup/)
    * [**🦀 7/27(목) T-RUST Meetup 🦀 (+ 현직자 네트워킹 세션)**](https://www.meetup.com/t-rust-meetup/events/295001681/)

### Europe

* 2023-07-27 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #38 at Deloitte!**](https://www.meetup.com/copenhagen-rust-community/events/294780997)
* 2023-07-27 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna)
    * [**Rust Dojo**](https://www.meetup.com/rust-vienna/events/294974346/)
* 2023-08-17 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/294201562/)
* 2023-08-22 | Helsinki, FI | [Finland Rust Meetup](https://www.meetup.com/helsinki-rust-meetup-group)
    * [**Helsink Rustaceans First Gathering**](https://www.meetup.com/helsinki-rust-meetup-group/events/294616573/)
* 2023-08-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust Aarhus Hack and Learn at Trifork**](https://www.meetup.com/rust-aarhus/events/293950871/)

### North America

* 2023-07-27 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/294911462/)
* 2023-08-01 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Happy Hour**](https://www.meetup.com/deep-dish-rust/events/294951373/)
* 2023-08-07 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/293561660/)
* 2023-08-10 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/294911475/)
* 2023-08-10 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Building a simplified JVM in Rust**](https://www.meetup.com/utah-rust/events/294972766/)
* 2023-08-15 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfclbtb)
* 2023-08-15 | Seattle, WA, US | [Seattle Rust User Group Meetup](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group - August Meetup**](https://www.meetup.com/seattle-rust-user-group/events/294804636/)
* 2023-08-17 | Nashville, TN, US | [Seattle Rust User Group Meetup](https://www.meetup.com/music-city-rust-developers/)
    * [**Rust goes where it pleases. Rust on the web and embedded**](https://www.meetup.com/music-city-rust-developers/events/294805470/)
* 2023-08-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/295008514)

### Oceania

* 2023-07-27 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**July Meetup**](https://www.meetup.com/rust-brisbane/events/294857868/)
* 2023-08-08 | Aukland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**A Peek into GPU Computing + Safer Code with Code Generation**](https://www.meetup.com/rust-akl/events/294858251/)
* 2023-08-09 | Perth, WA, AU | [Rust Perth](https://www.linkedin.com/groups/7439562/)
    * [**August Meetup**](https://www.tickettailor.com/events/perthrustusergroup/970279)

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

> Writing return \<something\>; at the end of a function in Rust is a bit like answering the question "Do you like potatoes?" with "Yes, I like potatoes" instead of simple "Yes".

– [Artem Borisovskiy on rust-users](https://users.rust-lang.org/t/enable-linting-for-implicit-returns/97170/15)

Thanks to [Todd Fleming](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1453) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
