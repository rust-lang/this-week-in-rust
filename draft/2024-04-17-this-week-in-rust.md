Title: This Week in Rust 543
Number: 543
Date: 2024-04-17
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

This week's crate is [venndb](https://crates.io/crates/venndb), an append-only memory DB whose tables can be build via a derive macro.

Thanks to [Glen De Cauwsemaecker](https://users.rust-lang.org/t/crate-of-the-week/2704/1303) for the self-suggestion!

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

430 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-04-09..2024-04-16

* [add support for Arm64EC inline assembly (as unstable)](https://github.com/rust-lang/rust/pull/123507)
* [`statx` probe: `ENOSYS` might come from a faulty FUSE driver](https://github.com/rust-lang/rust/pull/123928)
* [account for trait/impl difference when suggesting changing argument from ref to mut ref](https://github.com/rust-lang/rust/pull/123523)
* [add `REDUNDANT_LIFETIMES` lint to detect lifetimes which are semantically redundant](https://github.com/rust-lang/rust/pull/118391)
* [add `unsafe` to two functions with safety invariants](https://github.com/rust-lang/rust/pull/123867)
* [add const generics support for pattern types](https://github.com/rust-lang/rust/pull/123689)
* [add support to intrinsics fallback body](https://github.com/rust-lang/rust/pull/123659)
* [async closure coroutine by move body MirPass refactoring](https://github.com/rust-lang/rust/pull/123668)
* [avoid a panic in `set_output_capture` in the default panic handler](https://github.com/rust-lang/rust/pull/122882)
* [be more specific when flagging imports as redundant due to the extern prelude](https://github.com/rust-lang/rust/pull/122954)
* [call `lower_const_param` instead of duplicating the code](https://github.com/rust-lang/rust/pull/123738)
* [call the panic hook for non-unwind panics in proc-macros](https://github.com/rust-lang/rust/pull/123825)
* [detect borrow checker errors where `.clone()` would be an appropriate user action](https://github.com/rust-lang/rust/pull/122603)
* [disable Ctrl-C handling on WASM](https://github.com/rust-lang/rust/pull/123788)
* [discard overflow obligations in `impl_may_apply`](https://github.com/rust-lang/rust/pull/123618)
* [do not add prolog for variadic naked functions](https://github.com/rust-lang/rust/pull/123249)
* [do not allocate for ZST ThinBox (attempt 2 using `const_allocate`)](https://github.com/rust-lang/rust/pull/123254)
* [don't delay a bug if we suggest adding a semicolon to the RHS of an assign operator](https://github.com/rust-lang/rust/pull/123736)
* [don't do coroutine-closure-specific upvar analysis if tainted by errors](https://github.com/rust-lang/rust/pull/123834)
* [don't even parse an intrinsic unless the feature gate is enabled](https://github.com/rust-lang/rust/pull/123603)
* [don't leak unnameable types in `-> _` recover](https://github.com/rust-lang/rust/pull/123931)
* [don't rely on upvars being assigned just because coroutine-closure kind is assigned](https://github.com/rust-lang/rust/pull/123662)
* [fix UB in LLVM FFI when passing zero or \>1 bundle](https://github.com/rust-lang/rust/pull/123941)
* [fix invalid silencing of parsing error](https://github.com/rust-lang/rust/pull/123223)
* [fix various bugs in `ty_kind_suggestion`](https://github.com/rust-lang/rust/pull/123924)
* [generic associated consts: Check regions earlier when comparing impl with trait item def](https://github.com/rust-lang/rust/pull/123898)
* [improve diagnostic by suggesting to remove visibility qualifier](https://github.com/rust-lang/rust/pull/123841)
* [just use `type_dependent_def_id` to figure out what the method is for an expr](https://github.com/rust-lang/rust/pull/123989)
* [linker flavors next steps: linker features](https://github.com/rust-lang/rust/pull/123656)
* [linker: avoid some allocations in search directory iteration](https://github.com/rust-lang/rust/pull/123827)
* [linker: remove laziness and caching from native search directory walks](https://github.com/rust-lang/rust/pull/123854)
* [make `PlaceRef` and `OperandValue::Ref` share a common `PlaceValue` type](https://github.com/rust-lang/rust/pull/123775)
* [make the computation of `coroutine_captures_by_ref_ty` more sophisticated](https://github.com/rust-lang/rust/pull/123660)
* [only assert for child/parent projection compatibility AFTER checking that theyre coming from the same place](https://github.com/rust-lang/rust/pull/123701)
* [only collect mono items from reachable blocks](https://github.com/rust-lang/rust/pull/123272)
* [openBSD fix long socket addresses](https://github.com/rust-lang/rust/pull/123779)
* [panic on overflow in `BorrowedCursor::advance`](https://github.com/rust-lang/rust/pull/123806)
* [propagate temporary lifetime extension into if and match](https://github.com/rust-lang/rust/pull/121346)
* [provide suggestion to dereference closure tail if appropriate](https://github.com/rust-lang/rust/pull/122213)
* [refactor `panic_unwind/seh.rs` pointer use](https://github.com/rust-lang/rust/pull/123490)
* [remove `From` impls for unstable types that break inference](https://github.com/rust-lang/rust/pull/123830)
* [rework ptr-to-ref conversion suggestion for method calls](https://github.com/rust-lang/rust/pull/123007)
* [set target-abi module flag for RISC-V targets](https://github.com/rust-lang/rust/pull/123612)
* [skip `unused_parens` report for `Paren(Path(..))` in macro](https://github.com/rust-lang/rust/pull/123314)
* [stop making any assumption about the projections applied to the upvars in the `ByMoveBody` pass](https://github.com/rust-lang/rust/pull/123658)
* [stop using `HirId` for fn-like parents since closures are not `OwnerNode`s](https://github.com/rust-lang/rust/pull/123804)
* [stop using `PolyTraitRef` for closure/coroutine predicates already instantiated w placeholders](https://github.com/rust-lang/rust/pull/123900)
* [store all args in the unsupported Command implementation](https://github.com/rust-lang/rust/pull/123633)
* [suppress `let else` suggestion for uninitialized refutable `let`s](https://github.com/rust-lang/rust/pull/123847)
* [tweak value suggestions in `borrowck` and `hir_analysis`](https://github.com/rust-lang/rust/pull/123704)
* [typeck: fix `?` suggestion span](https://github.com/rust-lang/rust/pull/123654)
* [use `fn` ptr signature instead of `{closure@..}` in infer error](https://github.com/rust-lang/rust/pull/123703)
* [use `suggest_impl_trait` in return type suggestion on type error](https://github.com/rust-lang/rust/pull/123761)
* [miri: `MIRI_REPLACE_LIBRS_IF_NOT_TEST`: also apply to crates.io crates](https://github.com/rust-lang/miri/pull/3457)
* [miri: add some basic support for GetFullPathNameW](https://github.com/rust-lang/miri/pull/3466)
* [miri: fix error display for './miri run --dep'](https://github.com/rust-lang/miri/pull/3465)
* [miri: handle Miri sysroot entirely outside the Miri driver](https://github.com/rust-lang/miri/pull/3411)
* [miri: make `split_simd_to_128bit_chunks` take only one operand](https://github.com/rust-lang/miri/pull/3462)
* [miri on Windows: run .CRT$XLB linker section on thread-end](https://github.com/rust-lang/rust/pull/123937)
* [miri: windows: add basic support for FormatMessageW](https://github.com/rust-lang/miri/pull/3464)
* [stabilize --json `unused-externs(-silent)`](https://github.com/rust-lang/rust/pull/115717)
* [stabilize `(const_)slice_ptr_len` and `(const_)slice_ptr_is_empty_nonnull`](https://github.com/rust-lang/rust/pull/123868)
* [stabilize `cstr_count_bytes`](https://github.com/rust-lang/rust/pull/123661)
* [implement `FromIterator` for `(impl Default + Extend, impl Default + Extend)`](https://github.com/rust-lang/rust/pull/107462)
* [re-enable `has_thread_local` for i686-msvc](https://github.com/rust-lang/rust/pull/123257)
* [`std::net`: TcpListener shrinks the backlog argument to 32 for Haiku](https://github.com/rust-lang/rust/pull/123857)
* [show `mode_t` as octal in `std::fs` Debug impls](https://github.com/rust-lang/rust/pull/122812)
* [add `A: 'static` bound for `Arc/Rc::pin_in`](https://github.com/rust-lang/rust/pull/120092)
* [`f16` and `f128` step 4: basic library support](https://github.com/rust-lang/rust/pull/122470)
* [add a `Debug` impl and some basic functions to `f16` and `f128`](https://github.com/rust-lang/rust/pull/123783)
* [specialize many implementations of `Read::read_buf_exact`](https://github.com/rust-lang/rust/pull/122393)
* [windows: set main thread name without re-encoding](https://github.com/rust-lang/rust/pull/123534)
* [cargo: make sure to also wrap the initial `-vV` invocation](https://github.com/rust-lang/cargo/pull/13659)
* [cargo resolve: Respect '--ignore-rust-version'](https://github.com/rust-lang/cargo/pull/13738)
* [cargo resolve: Fallback to 'rustc -V' for MSRV resolving](https://github.com/rust-lang/cargo/pull/13743)
* [cargo fix: dont apply same suggestion twice](https://github.com/rust-lang/cargo/pull/13728)
* [cargo package: Normalize paths in `Cargo.toml`](https://github.com/rust-lang/cargo/pull/13729)
* [cargo test: don't compress test registry crates](https://github.com/rust-lang/cargo/pull/13744)
* [rustdoc: correctly handle inlining of doc hidden foreign items](https://github.com/rust-lang/rust/pull/123459)
* [rustdoc: check redundant explicit links with correct itemid](https://github.com/rust-lang/rust/pull/123905)
* [rustdoc: point at span in `include_str!`-ed md file](https://github.com/rust-lang/rust/pull/123204)
* [rustdoc: reduce per-page HTML overhead](https://github.com/rust-lang/rust/pull/123706)
* [clippy: `module_name_repetition` Recognize common prepositions](https://github.com/rust-lang/rust-clippy/pull/12573)
* [clippy: fix: incorrect suggestions when `.then` and `.then_some` is used](https://github.com/rust-lang/rust-clippy/pull/12094)
* [clippy: pin `remark-lint-maximum-line-length` version](https://github.com/rust-lang/rust-clippy/pull/12668)
* [clippy: turn `duplicated_attributes` into a late lint](https://github.com/rust-lang/rust-clippy/pull/12646)
* [clippy: use `check_attributes` in doc lints](https://github.com/rust-lang/rust-clippy/pull/12635)
* [rust-analyzer: add static and const highlighting token types](https://github.com/rust-lang/rust-analyzer/pull/17074)
* [rust-analyzer: better inline preview for postfix completion](https://github.com/rust-lang/rust-analyzer/pull/17073)
* [rust-analyzer: wrap/Unwrap `cfg_attr`](https://github.com/rust-lang/rust-analyzer/pull/16813)
* [rust-analyzer: VFS should not confuse paths with source roots that have the same prefix](https://github.com/rust-lang/rust-analyzer/pull/17019)
* [rust-analyzer: fix `impl Trait<Self>` causing stack overflows](https://github.com/rust-lang/rust-analyzer/pull/16877)
* [rust-analyzer: fix inlay hint resolution being broken](https://github.com/rust-lang/rust-analyzer/pull/17063)
* [rust-analyzer: fix: support auto-closing for triple backticks](https://github.com/rust-lang/rust-analyzer/pull/17051)
* [rust-analyzer: run cargo test per workspace in the test explorer](https://github.com/rust-lang/rust-analyzer/pull/17056)

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

Rusty Events between 2024-04-17 - 2024-05-15 🦀

### Virtual

* 2024-04-11 | Virtual + In Person (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477689/)
* 2024-04-11 | Virtual (Nürnberg, DE) | [Rust Nüremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945256/)
* 2024-04-11 | Virtual (San Diego, CA, US) | [San Diego Rust](https://www.meetup.com/san-diego-rust/)   
    * [**San Diego Rust April 2024 Tele-Meetup**](https://www.meetup.com/san-diego-rust/events/300307878/)
* 2024-04-15 & 2024-04-16 | Virtual | [Mainmatter](https://mainmatter.com/)
    * [**Remote Workshop: Testing for Rust projects – going beyond the basics**](https://ti.to/mainmatter/rust-testing-april-2024)
* 2024-04-16 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/rust-dublin/)
    * [**A reverse proxy with Tower and Hyperv1**](https://www.meetup.com/rust-dublin/events/300144192/)
* 2024-04-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—forensic parsing via Artemis**](https://www.meetup.com/rustdc/events/299346486/)
* 2024-04-17 | Virtual | [Rust for Lunch](https://lunch.rs/)
    * [**April 2024 Rust for Lunch**](https://lecture.senfcall.de/hay-gmh-wox-mru)
* 2024-04-17 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Reflections on RustNation UK 2024**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/300325512/)
* 2024-04-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298542323/)
* 2024-04-18 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368799/)
* 2024-04-21 | Virtual (Israel) | [Rust in Israel](https://rust.org.il/) 
    * [**Using AstroNvim for Rust development (in Hebrew)**](https://www.meetup.com/code-mavens/events/300265648/)
* 2024-04-25 | Virtual + In Person (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477692/)
* 2024-04-30 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtygcgbnc/)
* 2024-05-01 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 5 - Project Structure**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/300325526/)
* 2024-05-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047895/)
* 2024-05-02 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368804/)
* 2024-05-07 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300100279/)

### Africa

* 2024-05-04 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdsoporgprofile)

### Asia

* 2024-04-16 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup/)
    * [**The Good, the Bad, and the Async (RSVP by 15 Apr)**](https://www.meetup.com/tokyo-rust-meetup/events/300305613/)

### Europe

* 2024-04-10 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Rust Meetup Reboot 3**](https://www.meetup.com/cambridge-rust-meetup/events/299730322/)
* 2024-04-10 | Cologne/Köln, DE | [Rust Cologne](https://www.meetup.com/rustcologne/)
    * [**This Month in Rust, April**](https://www.meetup.com/rustcologne/events/300191375/)
* 2024-04-10 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester April 2024**](https://www.meetup.com/rust-manchester/events/299887934/)
* 2024-04-10 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/299488225/)
* 2024-04-11 | Bordeaux, FR | [Rust Bordeaux](https://www.meetup.com/bordeaux-rust/)
    * [**Rust Bordeaux #2 : Présentations**](https://www.meetup.com/bordeaux-rust/events/299628716/)
* 2024-04-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/299694473/)
* 2024-04-15 | Zagreb, HR | [impl Zagreb for Rust](https://www.meetup.com/zagreb-rust-meetup/)
    * [**Rust Meetup 2024/04: Building cargo projects with NIX**](https://www.meetup.com/zagreb-rust-meetup/events/299905015/)
* 2024-04-16 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake #5**](https://www.meetup.com/bratislava-rust-meetup-group/events/299302952/)
* 2024-04-16 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**winnow/nom**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/300024630/)
* 2024-04-16 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-04-17 | Bergen, NO | [Hubbel kodeklubb](https://www.meetup.com/bergen-html-css-meetup-group/)
    * [**Lær Rust med Conways Game of Life**](https://www.meetup.com/bergen-html-css-meetup-group/events/300031586/)
* 2024-04-17 | Ostrava, CZ | [TechMeetup Ostrava](https://www.meetup.com/techmeetupostrava/)
    * [**TechMeetup: RUST**](https://www.meetup.com/techmeetupostrava/events/299912212/)
* 2024-04-20 | Augsburg, DE | [Augsburger Linux-Infotag 2024](https://www.luga.de/static/LIT-2024/)
   * [**Augsburger Linux-Infotag 2024: Workshop Einstieg in Embedded Rust mit dem Raspberry Pico WH**](https://www.luga.de/static/LIT-2024/talks/einstieg_in_embedded_rust_mit_dem_raspberry_pico_wh/)
* 2024-04-23 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust'n'Tell - Rust for the Web**](https://www.meetup.com/rust-berlin/events/300047151/)
* 2024-04-23 | Paris, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [**Paris Rust Meetup #67**](https://mobilizon.fr/events/4ba93021-c43a-4e4a-b3e5-09c1c0d0a957)
* 2024-04-25 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at MFT Energy**](https://www.meetup.com/rust-aarhus/events/299564517/)
* 2024-04-23 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust'n'Tell - Rust for the Web**](https://www.meetup.com/rust-berlin/events/300047151/)
* 2024-04-25 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - TBD**](https://www.meetup.com/rust-berlin/events/299288960/)
* 2024-04-27 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**Fullstack Rust - Workshop #2 (Register by 23 April)**](https://www.meetup.com/rust-basel/events/299933581/)
* 2024-04-30 | Budapest, HU | [Budapest Rust Meetup Group](https://www.meetup.com/budapest-rust-meetup-group/)
    * [**Rust Meetup Budapest 2**](https://www.meetup.com/budapest-rust-meetup-group/events/300269044/)
* 2024-04-30 | Salzburg, AT | Rust Salzburg
    * [**Rust Salzburg meetup**]: 6:30pm - CCC Salzburg, 1. OG, ArgeKultur, Ulrike-Gschwandtner-Straße 5, 5020 Salzburg
* 2024-05-01 | Utrecht, NL | [NL-RSE Community](https://nl-rse.org/events/2024-05-01-meetup)
    * [**NL-RSE RUST meetup**](https://www.eventbrite.nl/e/nl-rse-rust-meetup-tickets-871056271757)
* 2024-05-06 | Delft, NL | [GOSIM](https://www.gosim.org/)
    * [**GOSIM Europe 2024**](https://europe2024.gosim.org/)
* 2024-05-07 & 2024-05-08 | Delft, NL | [RustNL](https://rustnl.org/)
    * [**RustNL 2024**](https://2024.rustnl.org/)

### North America

* 2024-04-10 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**Rust Meetup: Better Builds w/ Flox + Hangs**](https://www.meetup.com/boulder-rust-meetup/events/300019409/)
* 2024-04-11 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Interactive Storytelling using Yarn Spinner with Rex Magana**](https://www.meetup.com/utah-rust/events/300264363/)
* 2024-04-11 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/299509326/)
* 2024-04-11 | Spokane, WA, US | [Spohttps://www.meetup.com/minneapolis-rust-meetup/kane Rust](https://www.meetup.com/spokane-rust/)
    * [**Monthly Meetup: The Rust Full-Stack Experience**](https://www.meetup.com/spokane-rust/events/300019993/)
* 2024-04-15 | Minneapolis, MN, US | [Minneapolish Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust: Getting started with Rust! #2**](https://www.meetup.com/minneapolis-rust-meetup/events/300097803/)
* 2024-04-15 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Davis Square Rust Lunch, Apr 15**](https://www.meetup.com/bostonrust/events/300116673/)
* 2024-04-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186907/)
* 2024-04-16 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group: Meet Servo and Robius Open Source Projects**](https://www.meetup.com/seattle-rust-user-group/events/299908469/)
* 2024-04-18 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Talk: What Are Panics?**](https://www.meetup.com/deep-dish-rust/events/300204763/)
* 2024-04-18 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299803586/)
* 2024-04-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/299960315/)
* 2024-04-25 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers - Async Rust on Embedded**](https://www.meetup.com/music-city-rust-developers/events/299976876/)
* 2024-04-26 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**North End Rust Lunch, Apr 26**](https://www.meetup.com/bostonrust/events/300116689/)

### Oceania

* 2024-04-15 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**April 2024 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/300280391/)
* 2024-04-17 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**WMaTIR 2024 Gala & Talks**](https://www.meetup.com/rust-sydney/events/299882966/)
* 2024-04-30 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Why Rust? Convince Me!**](https://www.meetup.com/rust-akl/events/300304958/)
* 2024-04-30 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**CRUG April Meetup: Generics and Traits**](https://www.meetup.com/rust-canberra/events/300023000/)

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

> There is absolutely no way I can imagine that `Option` is causing that error. That'd be like turning on the "Hide Taskbar" setting causing your GPU to catch fire.
>
> [...]
>
> If it's not any of those, consider an exorcist because your machine *might* be haunted.

– [Daniel Keep on rust-users](https://users.rust-lang.org/t/access-is-denied-os-error-5/109515/2)

Thanks to [Hayden Brown](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1561) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
