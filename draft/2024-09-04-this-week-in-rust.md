Title: This Week in Rust 563
Number: 563
Date: 2024-09-04
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

This week's crate is [vimania-uri-rs](https://github.com/sysid/vimania-uri-rs), a VIM plugin for file and URI handling.

Thanks to [sysid](https://users.rust-lang.org/t/crate-of-the-week/2704/1334) for the self-suggestion!

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

416 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-08-27..2024-09-03

* [ABI compat check: detect unadjusted ABI mismatches](https://github.com/rust-lang/rust/pull/129649)
* [`rustc_target`: Add various aarch64 features](https://github.com/rust-lang/rust/pull/128192)
* [`ub_checks` intrinsics: fall back to `cfg(ub_checks)`](https://github.com/rust-lang/rust/pull/129551)
* [add `aarch64_unknown_nto_qnx700` target - QNX 7.0 support for aarch64le](https://github.com/rust-lang/rust/pull/127897)
* [add `needs-unwind` compiletest directive to `libtest-thread-limit` and replace some `Path` with `path` in `run-make`](https://github.com/rust-lang/rust/pull/129690)
* [add an ability to convert between `Span` and `visit::Location`](https://github.com/rust-lang/rust/pull/129170)
* [add missing `needs-llvm-components` directives for run-make tests that need target-specific codegen](https://github.com/rust-lang/rust/pull/129605)
* [add repr to the allowlist for naked functions](https://github.com/rust-lang/rust/pull/129421)
* [const fn stability checking: also check declared language features](https://github.com/rust-lang/rust/pull/129659)
* [const-eval: do not make UbChecks behavior depend on current crate's flags](https://github.com/rust-lang/rust/pull/129608)
* [coverage: rename `CodeRegion` to `SourceRegion`](https://github.com/rust-lang/rust/pull/129686)
* [create opaque definitions in resolver](https://github.com/rust-lang/rust/pull/129493)
* [debug-fmt-detail option](https://github.com/rust-lang/rust/pull/123940)
* [deny `wasm_c_abi` lint to nudge the last 25%](https://github.com/rust-lang/rust/pull/129534)
* [deny imports of `rustc_type_ir::inherent` outside of type ir + new trait solver](https://github.com/rust-lang/rust/pull/129678)
* [do not call `source_span` when not tracking dependencies](https://github.com/rust-lang/rust/pull/129513)
* [don't make statement nonterminals match pattern nonterminals](https://github.com/rust-lang/rust/pull/120221)
* [don't use `TyKind` in a lint](https://github.com/rust-lang/rust/pull/129527)
* [emit specific message for `time<=0.3.35`](https://github.com/rust-lang/rust/pull/129343)
* [enable Miri to pass pointers through FFI](https://github.com/rust-lang/rust/pull/129684)
* [exit: explain our expectations for the exit handlers registered in a Rust program](https://github.com/rust-lang/rust/pull/129581)
* [expand NLL MIR dumps](https://github.com/rust-lang/rust/pull/129711)
* [fix LLVM ABI NAME for riscv64imac-unknown-nuttx-elf](https://github.com/rust-lang/rust/pull/129842)
* [get rid of `predicates_defined_on`](https://github.com/rust-lang/rust/pull/129546)
* [implement a first version of RFC 3525: `struct` target features](https://github.com/rust-lang/rust/pull/127537)
* [interpret, codegen: tweak some comments and checks regarding Box with custom allocator](https://github.com/rust-lang/rust/pull/129812)
* [interpret/visitor: make memory order iteration slightly more efficient](https://github.com/rust-lang/rust/pull/129751)
* [interpret: add missing alignment check in `raw_eq`](https://github.com/rust-lang/rust/pull/129666)
* [interpret: do not make const-eval query result depend on tcx.sess](https://github.com/rust-lang/rust/pull/129613)
* [linker: synchronize native library search in rustc and linker](https://github.com/rust-lang/rust/pull/129366)
* [lint that warns when an elided lifetime ends up being a named lifetime (`elided_named_lifetimes`)](https://github.com/rust-lang/rust/pull/129207)
* [llvm-wrapper: adapt for LLVM API changes](https://github.com/rust-lang/rust/pull/129749)
* [make decoding non-optional `LazyArray` panic if not set](https://github.com/rust-lang/rust/pull/129829)
* [make it possible to enable `const_precise_live_drops` per-function](https://github.com/rust-lang/rust/pull/129507)
* [make the "detect-old-time" UI test more representative](https://github.com/rust-lang/rust/pull/129760)
* [make the const-unstable-in-stable error more clear](https://github.com/rust-lang/rust/pull/129818)
* [more `unreachable_pub`](https://github.com/rust-lang/rust/pull/129648)
* [move `'tcx` lifetime off of impl and onto methods for `CrateMetadataRef`](https://github.com/rust-lang/rust/pull/129689)
* [move the Windows `remove_dir_all` impl into a module and make it more race resistant](https://github.com/rust-lang/rust/pull/129800)
* [process.rs: remove "Basic usage" text where not useful](https://github.com/rust-lang/rust/pull/129916)
* [re-enable android tests/benches in alloc/core](https://github.com/rust-lang/rust/pull/129640)
* [refactor: standardize duplicate processes in parser](https://github.com/rust-lang/rust/pull/128641)
* [rename `BikeshedIntrinsicFrom` to `TransmuteFrom`](https://github.com/rust-lang/rust/pull/129657)
* [replace walk with visit so we dont skip outermost expr kind in def collector](https://github.com/rust-lang/rust/pull/129858)
* [rewrite `lint_expectations` in a single pass](https://github.com/rust-lang/rust/pull/127313)
* [riscv64imac: allow shadow call stack sanitizer](https://github.com/rust-lang/rust/pull/129316)
* [separate core search logic with search ui](https://github.com/rust-lang/rust/pull/126183)
* [simplify some extern providers](https://github.com/rust-lang/rust/pull/129723)
* [std: move allocators to `sys`](https://github.com/rust-lang/rust/pull/128134)
* [stop storing a special inner body for the coroutine by-move body for async closures](https://github.com/rust-lang/rust/pull/128506)
* [stop using `ty::GenericPredicates` for `non-predicates_of` queries](https://github.com/rust-lang/rust/pull/129725)
* [tweak some attributes to improve `panic_immediate_abort`](https://github.com/rust-lang/rust/pull/129589)
* [use a reduced recursion limit in the MIR inliner's cycle breaker](https://github.com/rust-lang/rust/pull/129714)
* [use equality when relating formal and expected type in arg checking](https://github.com/rust-lang/rust/pull/129317)
* [use unsafe extern blocks throughout the compiler](https://github.com/rust-lang/rust/pull/129635)
* [wasi: fix sleeping for `Duration::MAX`](https://github.com/rust-lang/rust/pull/129754)
* [miri: add tokio io test](https://github.com/rust-lang/miri/pull/3848)
* [miri: make TB tree traversal bottom-up](https://github.com/rust-lang/miri/pull/3843)
* [miri: make Tree Borrows Provenance GC compact the tree](https://github.com/rust-lang/miri/pull/3837)
* [miri: support blocking for epoll](https://github.com/rust-lang/miri/pull/3804)
* [apply size optimizations to panic machinery and some cold functions](https://github.com/rust-lang/rust/pull/129063)
* [`derive(SmartPointer)`: assume pointee from the single generic and better error messages](https://github.com/rust-lang/rust/pull/129467)
* [add `fmt::Debug` to `sync::Weak<T, A>`](https://github.com/rust-lang/rust/pull/129673)
* [add missing `read_buf` stub for `x86_64-unknown-l4re-uclibc`](https://github.com/rust-lang/rust/pull/129913)
* [allow `BufReader::peek` to be called on unsized types](https://github.com/rust-lang/rust/pull/129675)
* [core: use `compare_bytes` for more slice element types](https://github.com/rust-lang/rust/pull/128495)
* [fix `Pin::set` bounds regression](https://github.com/rust-lang/rust/pull/129668)
* [improved `checked_isqrt` and `isqrt` methods](https://github.com/rust-lang/rust/pull/128166)
* [partially stabilize `feature(new_uninit)`](https://github.com/rust-lang/rust/pull/129401)
* [hashbrown: add `HashTable::iter_hash`, `HashTable::iter_hash_mut`](https://github.com/rust-lang/hashbrown/pull/549)
* [cargo: resolve: Report incompatible-with-rustc when MSRV-resolver is disabled](https://github.com/rust-lang/cargo/pull/14459)
* [cargo: resolve: Report incompatible packages with precise Rust version](https://github.com/rust-lang/cargo/pull/14457)
* [cargo: pkgid: Allow open namespaces in PackageIdSpec's](https://github.com/rust-lang/cargo/pull/14467)
* [cargo: fix elided lifetime](https://github.com/rust-lang/cargo/pull/14487)
* [rustfmt: implement 2024 expression overflowing](https://github.com/rust-lang/rustfmt/pull/6260)
* [clippy: extend `implicit_saturating_sub` lint](https://github.com/rust-lang/rust-clippy/pull/12476)
* [clippy: new lint: `zombie_processes`](https://github.com/rust-lang/rust-clippy/pull/11476)
* [clippy: remove `feature=cargo-clippy` argument](https://github.com/rust-lang/rust-clippy/pull/13246)
* [rust-analyzer: extra sugar auto-completion `async fn ...` in `impl trait` for `async fn in trait` that's defined in desugar form](https://github.com/rust-lang/rust-analyzer/pull/17737)
* [rust-analyzer: fix handling of `for` in `impl T for A` in function body](https://github.com/rust-lang/rust-analyzer/pull/18005)
* [rust-analyzer: add explicit `enum` discriminant assist](https://github.com/rust-lang/rust-analyzer/pull/17985)
* [rust-analyzer: do not report missing unsafe on `addr_of[_mut]!(EXTERN_OR_MUT_STATIC)`](https://github.com/rust-lang/rust-analyzer/pull/18003)
* [rust-analyzer: create an assist to convert closure to freestanding fn](https://github.com/rust-lang/rust-analyzer/pull/17940)
* [rust-analyzer: implement cast typecheck and diagnostics](https://github.com/rust-lang/rust-analyzer/pull/17984)
* [rust-analyzer: implement object safety and its hovering hint](https://github.com/rust-lang/rust-analyzer/pull/17814)
* [rust-analyzer: suggest name in completion for `let_stmt` and `fn_param`](https://github.com/rust-lang/rust-analyzer/pull/18031)
* [rust-analyzer: support fn-ptr and fn-path types for lifetime elision hints](https://github.com/rust-lang/rust-analyzer/pull/18010)
* [rust-analyzer: fix incorrect symbol definitions in SCIP output](https://github.com/rust-lang/rust-analyzer/pull/17988)
* [rust-analyzer: `std::error::Error` is object unsafe](https://github.com/rust-lang/rust-analyzer/pull/17999)
* [rust-analyzer: consider field attributes when converting from tuple to named `struct` and the opposite](https://github.com/rust-lang/rust-analyzer/pull/17993)
* [rust-analyzer: consider indentation in the "Generate impl" and "Generate trait impl" assists](https://github.com/rust-lang/rust-analyzer/pull/17982)
* [rust-analyzer: don't add reference when it isn't needed for the "Extract variable" assist](https://github.com/rust-lang/rust-analyzer/pull/17991)
* [rust-analyzer: fix `TokenStream::to_string` implementation dropping quotation marks](https://github.com/rust-lang/rust-analyzer/pull/17994)
* [rust-analyzer: fix lifetime elision inlay hints breaking for ranged requests](https://github.com/rust-lang/rust-analyzer/pull/18012)
* [rust-analyzer: fix name resolution of shadowed builtin macro](https://github.com/rust-lang/rust-analyzer/pull/17987)
* [rust-analyzer: handle attributes correctly in "Flip comma"](https://github.com/rust-lang/rust-analyzer/pull/18015)
* [rust-analyzer: lifetime hint panic in non generic defs](https://github.com/rust-lang/rust-analyzer/pull/18028)
* [rust-analyzer: use Result type aliases in "Wrap return type in Result" assist](https://github.com/rust-lang/rust-analyzer/pull/18016)
* [rust-analyzer: provide an option to hide deprecated items from completion](https://github.com/rust-lang/rust-analyzer/pull/18006)
* [rust-analyzer: recategorize config classes](https://github.com/rust-lang/rust-analyzer/pull/17945)

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

Rusty Events between 2024-09-04 - 2024-10-02 🦀

### Virtual
* 2024-08-28 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Command Line Tools: Implementing wc in Rust (English, Virtual)**](https://www.meetup.com/code-mavens/events/302151487/)
* 2024-08-29 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633267/)
* 2024-08-29 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Rust Source Code Reading: The thousands crate (English)**](https://www.meetup.com/code-mavens/events/302391142/)
* 2024-09-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/302007365/)
* 2024-09-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - Typestate Pattern in Rust: With a Strict Builder Example**](https://www.meetup.com/indyrs/events/300328029/)
* 2024-09-05 | Virtual | [LambdaClass](https://lu.ma/user/usr-dkk9KnFvsvZEb7k)
    * [**Meetup Rust Septiembre [Spanish]**](https://lu.ma/uh1qpox0)
* 2024-09-05 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897957/)
* 2024-09-05 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820268/)
* 2024-09-10 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346981/)
* 2024-09-10 - 2024-09-13 | Hybrid: Virtual and In-Person (Montreal, QC, CA) | [Rust Conf](https://rustconf.com/)
    * [**Rust Conf 2024**](https://foundation.rust-lang.org/events/rustconf-2024/)
* 2024-09-12 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633268/)
* 2024-09-12 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/)
    * [**Bevy Meetup #6**](https://www.meetup.com/bevy-game-development/events/302841892/)
* 2024-09-16 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/302827971/)
* 2024-09-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346969/)
* 2024-09-18 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Cells**](https://www.meetup.com/vancouver-rust/events/298631736/)
* 2024-09-18 - 2024-09-20 | Hybrid - Virtual and In-Person (Vienna, AT) | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2024**](https://lpc.events/event/18/sessions/186/)
* 2024-09-19 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897973/)
* 2024-09-19 | Virtual and In-Person (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**September Meetup**](https://www.meetup.com/join-srug/events/303067835/)
* 2024-09-24 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/301585670/)

### Africa
* 2024-09-06 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587/)

### Asia
* 2024-09-09 | Ramat Gan, IL | [Coralogix](https://coralogix.com/)
    * [**Rust as Scale**](https://coralogix.com/rust-coralogix-meetup/)
* 2024-09-14 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**September 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/september-2024-rustacean-meetup/)

### Europe
* 2024-08-28 | Frankfurt (Main), DE | [Rust Rhein Main](https://www.meetup.com/rust-rhein-main)
    * [**Rust Frankfurt WebAssembly**](https://www.meetup.com/rust-rhein-main/events/302738445/)
* 2024-08-29 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - The social Beergarden.**](https://www.meetup.com/rust-berlin/events/299421378/)
* 2024-08-29 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #50 sponsored by Adapt Agency**](https://www.meetup.com/copenhagen-rust-community/events/303040544/)
* 2024-09-04 | Oxford, UK | [Oxfrod Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**More Rust - Generics, constraints, safety.**](https://www.meetup.com/oxford-rust-meetup-group/events/302848276/)
* 2024-09-11 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/302833564/)
* 2024-09-17 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425049/)
* 2024-09-17 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/)
    * [**Making AI-models perform tasks, in Rust!**](https://www.meetup.com/rust-trondheim/events/302957040/)
* 2024-09-18 | Moravia, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/)
    * [**Rust Moravia Meetup (September 2024)**](https://www.meetup.com/rust-moravia/events/301360936)
* 2024-09-18 | Vienna, AT + Virtual | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2024 (Sep 18-20)**](https://lpc.events/event/18/sessions/186/)
* 2024-09-23 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake #6**](https://www.meetup.com/bratislava-rust-meetup-group/events/302916594/)

### North America
* 2024-08-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygclblc/)
* 2024-08-29 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers : placeholder**](https://www.meetup.com/music-city-rust-developers/events/301420110/)
* 2024-08-29 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night/Happy Hour**](https://www.meetup.com/deep-dish-rust/events/302940777/)
* 2024-08-31 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Central Cambridge Rust Lunch, Aug 31**](https://www.meetup.com/bostonrust/events/302498706/)
* 2024-09-05 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/302723843/)
* 2024-09-05 | Portland, OR, US | [PDX Rust](https://www.meetup.com/pdxrust/)
    * [**PDX Rust September!**](https://www.meetup.com/pdxrust/events/302588479/)
* 2024-09-05 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Lifetimes**](https://www.meetup.com/stl-rust/events/hdzdmtygcmbhb/)
* 2024-09-07 | Longview, TX, US | [Longview Code and Coffee](https://www.meetup.com/longview-code-and-coffee/)
    * [**Longview Code and Coffee**](https://www.meetup.com/longview-code-and-coffee/events/301976293/)
* 2024-09-08 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/302498734/)
    * [**Northeastern Rust Lunch, Sep 8**](https://www.meetup.com/bostonrust/events/302498706/)
* 2024-09-10 - 2024-09-13 | Hybrid: Virtual and In-Person (Montreal, QC, CA) | [Rust Conf](https://rustconf.com/)
    * [**Rust Conf 2024**](https://foundation.rust-lang.org/events/rustconf-2024/)
* 2024-09-11 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**Boulder Elixir Meetup**](hhttps://www.meetup.com/boulder-elixir/events/302991078/)
* 2024-09-16 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Somerville Union Square Rust Lunch, Sep 16**](https://www.meetup.com/bostonrust/events/302498750/)
* 2024-09-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638248/)
* 2024-09-18 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Cells**](https://www.meetup.com/vancouver-rust/events/298631736/)
* 2024-09-19 | Virtual and In-Person (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**September Meetup**](https://www.meetup.com/join-srug/events/303067835/)
* 2024-09-21 | Longview, TX, US | [Longview Code and Coffee](https://www.meetup.com/longview-code-and-coffee/)
    * [**Longview Code and Coffee**](https://www.meetup.com/longview-code-and-coffee/events/301976355/)
* 2024-09-24 | Detroit, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ferndale**](https://www.meetup.com/detroitrust/events/zfcbntygcmbgc/)
* 2024-09-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/302274449/)

# Oceania
* 2024-08-28 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**Stuff ⚡ And Crabs 🦀**](https://www.meetup.com/rust-sydney/events/302951173/)

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

> I'm pretty sure I'm the only person ever to single handedly write a complex GPU kernel driver that has never had a memory safety kernel panic bug (itself) in production, running on thousands of users' systems for 1.5 years now.
>
> Because I wrote it in Rust.

– [Asahi Lina on vt.social](https://vt.social/@lina/113045456734886438)

Thanks to [Ludwig Stecher](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1604) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
