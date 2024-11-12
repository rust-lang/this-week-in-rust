Title: This Week in Rust 573
Number: 573
Date: 2024-11-13
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://x.com/ThisWeekInRust) on X (formerly Twitter) or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

Want TWIR in your inbox? [Subscribe here](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

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

This week's crate is [struct-split](https://github.com/wdanilo/struct-split), a proc macro to implement partial borrows.

Thanks to [Felix](https://users.rust-lang.org/t/crate-of-the-week/2704/1374) for the suggestion!

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

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

403 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-11-05..2024-11-12

* [remove the `wasm32-wasi` target from rustc](https://github.com/rust-lang/rust/pull/132562)
* [add a new `wide-arithmetic` feature for WebAssembly](https://github.com/rust-lang/rust/pull/132077)
* [add Unicode block-drawing compiler output support](https://github.com/rust-lang/rust/pull/126597)
* [add `{ignore,needs}-{rustc,std}-debug-assertions` directive support](https://github.com/rust-lang/rust/pull/131913)
* [add a default implementation for `CodegenBackend::link`](https://github.com/rust-lang/rust/pull/132820)
* [add discriminators to DILocations when multiple functions are inlined into a single point](https://github.com/rust-lang/rust/pull/132613)
* [add v9, v8plus, and leoncasa target feature to sparc and use v8plus in `create_object_file`](https://github.com/rust-lang/rust/pull/132552)
* [additional tests to ensure let is rejected during parsing](https://github.com/rust-lang/rust/pull/132828)
* [arbitrary self types v2: (unused) Receiver trait](https://github.com/rust-lang/rust/pull/132144)
* [basic inline assembly support for SPARC and SPARC64](https://github.com/rust-lang/rust/pull/132472)
* [coverage: extract safe FFI wrapper functions to `llvm_cov`](https://github.com/rust-lang/rust/pull/132452)
* [coverage: restrict empty-span expansion to only cover `{` and `}`](https://github.com/rust-lang/rust/pull/132675)
* [coverage: simplify parts of coverage graph creation](https://github.com/rust-lang/rust/pull/132389)
* [do not filter empty lint passes & re-do CTFE pass](https://github.com/rust-lang/rust/pull/132637)
* [do not reveal opaques in the param-env, we got lazy norm instead](https://github.com/rust-lang/rust/pull/132755)
* [do not trust download-rustc=if-unchanged on CI for now](https://github.com/rust-lang/rust/pull/132852)
* [don't suggest `.into_iter()` on iterators](https://github.com/rust-lang/rust/pull/132760)
* [don't use `maybe_unwrap_block` when checking for macro calls in a block expr](https://github.com/rust-lang/rust/pull/132653)
* [dont suggest `use<impl Trait>` when we have an edition-2024-related borrowck issue](https://github.com/rust-lang/rust/pull/132816)
* [drop "gnu" in the target env for FreeBSD armv6/7](https://github.com/rust-lang/rust/pull/132764)
* [emit warning when calling/declaring functions with unavailable vectors](https://github.com/rust-lang/rust/pull/132173)
* [enforce that raw lifetimes must be valid raw identifiers](https://github.com/rust-lang/rust/pull/132363)
* [ensure that tail expr receive lifetime extension](https://github.com/rust-lang/rust/pull/129627)
* [fix parens mangled in shared mut static lint suggestion](https://github.com/rust-lang/rust/pull/132095)
* [get rid of `check_opaque_type_well_formed`](https://github.com/rust-lang/rust/pull/132757)
* [make `RustString` an extern type to avoid `improper_ctypes` warnings](https://github.com/rust-lang/rust/pull/132549)
* [make `Ty::primitive_symbol` recognize `str`](https://github.com/rust-lang/rust/pull/132799)
* [make `fn_abi_sanity_check` a bit stricter](https://github.com/rust-lang/rust/pull/132729)
* [make sure that we suggest turbofishing the right type arg for never suggestion](https://github.com/rust-lang/rust/pull/132933)
* [mark some target features as 'forbidden' so they cannot be (un)set with -Ctarget-feature](https://github.com/rust-lang/rust/pull/129884)
* [only disable cache if predicate has opaques within it](https://github.com/rust-lang/rust/pull/132625)
* [passWrapper: adapt for new parameter in LLVM](https://github.com/rust-lang/rust/pull/132600)
* [prefer `pub(super)` in `unreachable_pub` lint suggestion](https://github.com/rust-lang/rust/pull/132426)
* [properly suggest `E::assoc` when we encounter `E::Variant::assoc`](https://github.com/rust-lang/rust/pull/132567)
* [provide placeholder generics for traits in "no method found for type parameter" suggestions](https://github.com/rust-lang/rust/pull/132487)
* [reject raw lifetime followed by `'`, like regular lifetimes do](https://github.com/rust-lang/rust/pull/132341)
* [remove 'platform-intrinsic' ABI leftovers](https://github.com/rust-lang/rust/pull/132734)
* [remove `rustc_session::config::rustc_short_optgroups`](https://github.com/rust-lang/rust/pull/132891)
* [remove support for `rustc_safe_intrinsic` attribute; use `rustc_intrinsic` functions instead](https://github.com/rust-lang/rust/pull/132717)
* [remove unnecessary pub `enum` glob-imports from `rustc_middle::ty`](https://github.com/rust-lang/rust/pull/132580)
* [require `const_impl_trait` gate for all conditional and trait const calls](https://github.com/rust-lang/rust/pull/132823)
* [revert using `HEAP` static in Windows alloc](https://github.com/rust-lang/rust/pull/131888)
* [set "symbol name" in raw-dylib import libraries to the decorated name](https://github.com/rust-lang/rust/pull/130586)
* [simplify FFI calls for `-Ztime-llvm-passes` and `-Zprint-codegen-stats`](https://github.com/rust-lang/rust/pull/132590)
* [simplify some places that deal with generic parameter defaults](https://github.com/rust-lang/rust/pull/132912)
* [simplify the internal API for declaring command-line options](https://github.com/rust-lang/rust/pull/132754)
* [suggest swapping LHS and RHS when RHS impls `PartialEq<lhs_ty>`](https://github.com/rust-lang/rust/pull/132404)
* [tweak E0320 overflow error wording](https://github.com/rust-lang/rust/pull/132663)
* [tweak detection of multiple crate versions to be more encompassing](https://github.com/rust-lang/rust/pull/128849)
* [use `download-rustc="if-unchanged"` as a global default](https://github.com/rust-lang/rust/pull/132772)
* [use a separate dir for r-a builds consistently in helix config](https://github.com/rust-lang/rust/pull/132794)
* [use verbose for path separator suggestion](https://github.com/rust-lang/rust/pull/132780)
* [`pointee_info_at`: fix logic for recursing into enums](https://github.com/rust-lang/rust/pull/132745)
* [`rustc_codegen_llvm`: Add a new 'pc' option to branch-protection](https://github.com/rust-lang/rust/pull/132259)
* [`rustc_target`: more target string fixes for LLVM 20](https://github.com/rust-lang/rust/pull/132785)
* [interpret: `get_alloc_info`: also return mutability](https://github.com/rust-lang/rust/pull/132801)
* [StableMIR: A few fixes to pretty printing](https://github.com/rust-lang/rust/pull/132161)
* [StableMIR: API to retrieve definitions from crates](https://github.com/rust-lang/rust/pull/132131)
* [miri: fix linux-futex test being accidentally disabled](https://github.com/rust-lang/miri/pull/4022)
* [miri: get/set thread name shims return errors for invalid handles](https://github.com/rust-lang/miri/pull/4004)
* [miri: preparing for merge from rustc](https://github.com/rust-lang/miri/pull/4023)
* [miri: pthread-sync test: avoid confusing error when running with preemption](https://github.com/rust-lang/miri/pull/4020)
* [miri: remove MutexID list](https://github.com/rust-lang/miri/pull/4002)
* [miri: renamed this arguments to ecx](https://github.com/rust-lang/miri/pull/4029)
* [miri: stacked borrows tests: add those that fail under TB](https://github.com/rust-lang/miri/pull/4028)
* [miri: standardized variable names for InterpCx](https://github.com/rust-lang/miri/pull/4018)
* [miri: store futexes in per-allocation data rather than globally](https://github.com/rust-lang/miri/pull/3971)
* [miri: sync support: dont implicitly clone inside the general sync machinery](https://github.com/rust-lang/miri/pull/4027)
* [stabilise `const_char_encode_utf16`](https://github.com/rust-lang/rust/pull/132153)
* [stabilize Arm64EC inline assembly](https://github.com/rust-lang/rust/pull/131781)
* [stabilize WebAssembly `multivalue`, reference-types`, and tail-call` target features](https://github.com/rust-lang/rust/pull/131080)
* [stabilize `UnsafeCell::from_mut`](https://github.com/rust-lang/rust/pull/131261)
* [stabilize s390x inline assembly](https://github.com/rust-lang/rust/pull/131258)
* [add new unstable feature `const_eq_ignore_ascii_case`](https://github.com/rust-lang/rust/pull/131721)
* [make `char::is_whitespace` unstably const](https://github.com/rust-lang/rust/pull/132500)
* [inline `str::repeat`](https://github.com/rust-lang/rust/pull/132705)
* [core/fmt: Replace checked slice indexing by unchecked to support panic-free code](https://github.com/rust-lang/rust/pull/132473)
* [add Set entry API](https://github.com/rust-lang/rust/pull/120077)
* [implement `div_ceil` for `NonZero<unsigned>`](https://github.com/rust-lang/rust/pull/132665)
* [implement `file_lock` feature](https://github.com/rust-lang/rust/pull/130999)
* [initialize channel `Block`s directly on the heap](https://github.com/rust-lang/rust/pull/132738)
* [disable `f16` on platforms that have recursion problems](https://github.com/rust-lang/compiler-builtins/pull/730)
* [cargo: warnings: add build.warnings option](https://github.com/rust-lang/cargo/pull/14388)
* [cargo: test: Make redactions consistent with snapbox](https://github.com/rust-lang/cargo/pull/14790)
* [cargo: git: do not validate submodules of fresh checkouts](https://github.com/rust-lang/cargo/pull/14605)
* [cargo: normalize the `target` paths](https://github.com/rust-lang/cargo/pull/14497)
* [cargo: refactor: clone-on-write when needed for InternedString](https://github.com/rust-lang/cargo/pull/14808)
* [cargo: rustfix: replace special-case duplicate handling with error](https://github.com/rust-lang/cargo/pull/14782)
* [rustdoc-search: show type signature on type-driven SERP](https://github.com/rust-lang/rust/pull/124544)
* [rustdoc-search: simplify rules for generics and type params](https://github.com/rust-lang/rust/pull/127589)
* [bindgen: fix `field_visibility` not called for new-type aliases](https://github.com/rust-lang/rust-bindgen/pull/2967)
* [bindgen: fix `unsafe_op_in_unsafe_fn` when using dynamic librarys and `wrap_unsafe_ops`](https://github.com/rust-lang/rust-bindgen/pull/2961)
* [handle separate prefixes in clippy rules](https://github.com/rust-lang/rust/pull/132873)
* [clippy: `no_mangle_with_rust_abi`: properly position the suggested ABI](https://github.com/rust-lang/rust-clippy/pull/13659)
* [clippy: add match-based manual try to `clippy::question_mark`](https://github.com/rust-lang/rust-clippy/pull/13627)
* [clippy: collect attribute spans early for disallowed macros](https://github.com/rust-lang/rust-clippy/pull/13657)
* [clippy: fix `large_include_file` lint being triggered all the time by doc comments](https://github.com/rust-lang/rust-clippy/pull/13672)
* [clippy: fix: `identity_op` suggestions use correct parenthesis](https://github.com/rust-lang/rust-clippy/pull/13647)
* [rust-analyzer: editors/code: change minimum VS Code from 1.78 to 1.83](https://github.com/rust-lang/rust-analyzer/pull/18486)
* [rust-analyzer: use completion item indices instead of property matching when searching for the completion item to resolve](https://github.com/rust-lang/rust-analyzer/pull/18503)

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

Rusty Events between 2024-11-13 - 2024-12-11 🦀

### Virtual
* 2024-11-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031651/)
* 2024-11-07 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633272/)
* 2024-11-08 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304099245/)
* 2024-11-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346985/)
* 2024-11-14 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898070/)
* 2024-11-14 | Virtual and In-Person (Lehi, UT, US) | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Green Thumb: Building a Bluetooth-Enabled Plant Waterer with Rust and Microbit**](https://www.meetup.com/utah-rust/events/304206130/)
* 2024-11-14 | Virtual and In-Person (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**November Meetup**](https://www.meetup.com/join-srug/events/304166747/)
* 2024-11-15 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcpbtb/)
* 2024-11-19 | Virtual (Los Angeles, CA, US) | [DevTalk LA](https://www.meetup.com/lajugstudygroup/)
    * [**Discussion - Topic: Rust for UI**](https://www.meetup.com/lajugstudygroup/events/302952703/)
* 2024-11-19 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346971/)
* 2024-11-20 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Embedded Rust Workshop**](https://www.meetup.com/vancouver-rust/events/304047664/)
* 2024-11-21 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633273/)
* 2024-11-21 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Trustworthy IoT with Rust--and passwords!**](https://www.meetup.com/charlottesville-rust-meetup/events/304216847/)
* 2024-11-21 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #7**](https://www.meetup.com/bevy-game-development/events/304078762/)
* 2024-11-25 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**ONLINE Talk, sponsored by Sonalake - Bratislava Rust Meetup**](https://www.meetup.com/bratislava-rust-meetup-group/events/304373224/)
* 2024-11-26 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/fkmcntygcpbjc/)
* 2024-11-28 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898099/)
* 2024-11-28 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820276/)
* 2024-12-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/302007374/)

### Asia
* 2024-11-28 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**RustTechX Summit 2024 BOSCH**](https://hasgeek.com/rustbangalore/rusttechx-summit-2024-bosch/)
* 2024-11-30 | Tokyo, JP | [Rust Tokyo](https://rust.tokyo/)
    * [**Rust.Tokyo 2024**](https://rust.tokyo/lineup)

### Europe
* 2024-11-06 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123398/)
* 2024-11-06 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.fr/o/paris-rustaceans-74289178383)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1037795553437)
* 2024-11-09 - 2024-11-11 | Florence, IT | [Rust Lab](https://rustlab.it/)
    * [**Rust Lab 2024: The International Conference on Rust in Florence**](https://rustlab.it/schedule)
* 2024-11-12 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/events/)
    * [**LDN Talks November 2024 RustRover Takeover with JetBrains**](https://www.meetup.com/rust-london-user-group/events/304378534/)
* 2024-11-12 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/events/)
    * [**Encrypted/distributed filesystems, wasm-bindgen**](https://www.meetup.com/rust-zurich/events/304162840)
* 2024-11-13 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/303915771/)
* 2024-11-14 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @UXStream**](https://www.meetup.com/stockholm-rust/events/304124737/)
* 2024-11-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Daten sichern mit ZFS (und Rust)**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425200/)
* 2024-11-19 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #72**](https://www.meetup.com/rust-paris/events/304396616/)
* 2024-11-21 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (pub)**](https://www.meetup.com/rust-and-friends/events/304110922/)
* 2024-11-21 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154277/)
* 2024-11-23 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust + HTMX - Workshop #3**](https://www.meetup.com/rust-basel/events/303714372/)
* 2024-11-26 | Warsaw, PL | [Rust Warsaw](https://www.meetup.com/rust-warsaw/events/)
    * [**New Rust Warsaw Meetup #3**](https://www.meetup.com/rust-warsaw/events/304379707/)
* 2024-11-27 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund**](https://www.meetup.com/rust-dortmund/events/304290556)
* 2024-11-28 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at Lind Capital**](https://www.meetup.com/rust-aarhus/events/304005322/)
* 2024-11-28 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #10**](https://www.meetup.com/rust-meetup-augsburg/events/304002691/)
* 2024-11-28 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299421381/)
* 2024-11-28 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/events/)
    * [**Rust Hack & Learn with Mainmatter & Otto**](https://www.meetup.com/rust-meetup-hamburg/events/303898286/)
* 2024-11-28 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/events/)
    * [**Rust/C++ Meetup Prague (November 2024)**](https://www.meetup.com/rust-prague/events/304002733/)
* 2024-12-04 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123399/)

### North America
* 2024-11-07 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/)
    * [**Chicago Rust Meetup**](https://www.meetup.com/chicago-rust-meetup/events/304327595/)
* 2024-11-07 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal/)
    * [**November Monthly Social**](https://www.meetup.com/rust-montreal/events/304248702/)
* 2024-11-07 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Game development with Rust and the Bevy engine**](https://www.meetup.com/stl-rust/events/302371464/)
* 2024-11-12 | Ann Arbor, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcpbqb/)
* 2024-11-12 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC Monthly Meetup**](https://www.meetup.com/rust-nyc/events/304358109/)
* 2024-11-14 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/hackerdojo/events/304211045/)
* 2024-11-15 | Mexico City, DF, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Multi threading y Async en Rust parte 2 - Smart Pointes y Closures**](https://www.meetup.com/rust-mx/events/304150412/)
* 2024-11-15 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch, Nov 15**](https://www.meetup.com/bostonrust/events/303708398/)
* 2024-11-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638252/)
* 2024-11-23 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston Common Rust Lunch, Nov 23**](https://www.meetup.com/bostonrust/events/303708407/)
* 2024-11-25 | Ferndale, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ferndale**](https://www.meetup.com/detroitrust/events/dmgjntygcpbhc/)
* 2024-11-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcpbkc/)

### Oceania
* 2024-11-12 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/events/)
    * [**Christchurch Rust Meetup**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/304029765/)

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

> Netstack3 encompasses 63 crates and 60 developer-years of code. It contains more code than the top ten crates on [crates.io](https://crates.io/) combined. ... For the past eleven months, they have been running the new networking stack on 60 devices, full time. In that time, Liebow-Feeser said, most code would have been expected to show "mountains of bugs". Netstack3 had only three; he attributed that low number to the team's approach of encoding as many important invariants in the type system as possible.

– [Joshua Liebow-Feeser at RustConf, as reported by Daroc Alden on Linux Weekly News](https://lwn.net/SubscriberLink/995814/17e451bcb3015920/)

Thanks to [Anton Fetisov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1630) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
