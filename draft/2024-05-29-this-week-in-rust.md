Title: This Week in Rust 549
Number: 549
Date: 2024-05-29
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://x.com/ThisWeekInRust) on X(formerly Twitter) or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).

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

This week's crate is [pulso](https://github.com/guapodero/pulso), a simple metrics collector for TCP/IP.

Thanks to [guapodero](https://users.rust-lang.org/t/crate-of-the-week/2704/1312) for the self-suggestion!

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

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](link to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

397 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-05-21..2024-05-28

* [MIR validation: ensure that downcast projection is followed by field projection](https://github.com/rust-lang/rust/pull/125616)
* [actually use TAIT instead of emulating it](https://github.com/rust-lang/rust/pull/125362)
* [allow coercing functions whose signature differs in opaque types in their defining scope into a shared function pointer type](https://github.com/rust-lang/rust/pull/124297)
* [allow monomorphization time const eval failures if the cause is a type layout issue](https://github.com/rust-lang/rust/pull/124516)
* [an async closure may implement `FnMut`/`Fn` if it has no self-borrows](https://github.com/rust-lang/rust/pull/125259)
* [detect unused structs which implement private traits](https://github.com/rust-lang/rust/pull/122382)
* [disallow cast with trailing braced macro in let-else](https://github.com/rust-lang/rust/pull/125049)
* [don't continue probing for method if in suggestion and autoderef hits ambiguity](https://github.com/rust-lang/rust/pull/125466)
* [don't do cc detection for synthetic targets](https://github.com/rust-lang/rust/pull/125369)
* [don't eagerly monomorphize drop for types that are impossible to instantiate](https://github.com/rust-lang/rust/pull/125513)
* [don't suggest adding the unexpected cfgs to the build-script it-self](https://github.com/rust-lang/rust/pull/125412)
* [drop region constraints for ambiguous goals](https://github.com/rust-lang/rust/pull/125413)
* [exit the process a short time after entering our ctrl-c handler](https://github.com/rust-lang/rust/pull/125523)
* [expand `for_loops_over_fallibles` lint to lint on fallibles behind references](https://github.com/rust-lang/rust/pull/125156)
* [f32: use constants instead of reassigning a dummy value as PI](https://github.com/rust-lang/rust/pull/125571)
* [fail relating constants of different types](https://github.com/rust-lang/rust/pull/125451)
* [fix OutsideLoop's error suggestion: adding label `'block` for `if` block](https://github.com/rust-lang/rust/pull/123623)
* [fix `unexpected_cfgs` lint on std](https://github.com/rust-lang/rust/pull/125296)
* [fix incorrect suggestion for undeclared hrtb lifetimes in where clauses](https://github.com/rust-lang/rust/pull/123122)
* [fix parsing of erroneously placed semicolons](https://github.com/rust-lang/rust/pull/125276)
* [follow-up fixes to `report_return_mismatched_types`](https://github.com/rust-lang/rust/pull/123812)
* [force the inner coroutine of an async closure to `move` if the outer closure is `move` and `FnOnce`](https://github.com/rust-lang/rust/pull/125306)
* [handle `ReVar` in `note_and_explain_region`](https://github.com/rust-lang/rust/pull/125054)
* [make sure that the method resolution matches in `note_source_of_type_mismatch_constraint`](https://github.com/rust-lang/rust/pull/124227)
* [move `#[do_not_recommend]` to the `#[diagnostic]` namespace](https://github.com/rust-lang/rust/pull/125326)
* [only allow immutable statics with `#[linkage]`](https://github.com/rust-lang/rust/pull/125046)
* [only suppress binop error in favor of semicolon suggestion if we're in an assignment statement](https://github.com/rust-lang/rust/pull/125467)
* [panic directly in `Arguments::new*` instead of recursing](https://github.com/rust-lang/rust/pull/117804)
* [pattern types: Prohibit generic args on const params](https://github.com/rust-lang/rust/pull/125015)
* [properly deal with missing/placeholder types inside GACs](https://github.com/rust-lang/rust/pull/125457)
* [relax restrictions on multiple sanitizers](https://github.com/rust-lang/rust/pull/124676)
* [remove `DefId` from `EarlyParamRegion`](https://github.com/rust-lang/rust/pull/125468)
* [remove proof tree formatting, make em shallow](https://github.com/rust-lang/rust/pull/125510)
* [rename `FrameworkOnlyWindows` to `RawDylibOnlyWindows`](https://github.com/rust-lang/rust/pull/125409)
* [resolve anon const's parent predicates to direct parent instead of opaque's parent](https://github.com/rust-lang/rust/pull/125501)
* [stop SRoA'ing `DynMetadata` in MIR](https://github.com/rust-lang/rust/pull/125508)
* [support C23's Variadics Without a Named Parameter](https://github.com/rust-lang/rust/pull/124048)
* [tag more stuff with `WG-trait-system-refactor`](https://github.com/rust-lang/rust/pull/125519)
* [turn remaining non-structural-const-in-pattern lints into hard errors](https://github.com/rust-lang/rust/pull/124661)
* [use `Backtrace::force_capture` instead of `Backtrace::capture` in `rustc_log`](https://github.com/rust-lang/rust/pull/125355)
* [validate the special layout restriction on `DynMetadata`](https://github.com/rust-lang/rust/pull/125479)
* [warn (or error) when `Self` ctor from outer item is referenced in inner nested item](https://github.com/rust-lang/rust/pull/124187)
* [wrap Context.ext in AssertUnwindSafe](https://github.com/rust-lang/rust/pull/125392)
* [interpret: make overflowing binops just normal binops](https://github.com/rust-lang/rust/pull/125359)
* [miri: add back some tokio features](https://github.com/rust-lang/miri/pull/3628)
* [miri: bugfix `MiriAllocBytes` to guarantee different addresses](https://github.com/rust-lang/miri/pull/3625)
* [miri: completely refactor how we manage blocking and unblocking threads](https://github.com/rust-lang/miri/pull/3631)
* [perf: Delay the construction of early lint diag structs](https://github.com/rust-lang/rust/pull/125410)
* [stabilize `LazyCell` and `LazyLock`](https://github.com/rust-lang/rust/pull/121377)
* [stabilize `div_duration`](https://github.com/rust-lang/rust/pull/124667)
* [stabilize `slice_flatten`](https://github.com/rust-lang/rust/pull/125561)
* [rewrite native thread-local storage](https://github.com/rust-lang/rust/pull/116123)
* [rewrite TLS on platforms without threads](https://github.com/rust-lang/rust/pull/123724)
* [simplify key-based thread locals](https://github.com/rust-lang/rust/pull/122494)
* [add opt-for-size core lib feature flag](https://github.com/rust-lang/rust/pull/125011)
* [always use the general case char count with `optimize_for_size`](https://github.com/rust-lang/rust/pull/125609)
* [add `IntoIterator` for `Box<[T]>` + edition 2024-specific lints](https://github.com/rust-lang/rust/pull/124097)
* [add `assert_unsafe_precondition` to `unchecked_{add,sub,neg,mul,shl,shr}` methods](https://github.com/rust-lang/rust/pull/121571)
* [add a fast-path to `Debug` ASCII `&str`](https://github.com/rust-lang/rust/pull/121150)
* [add manual `Sync` impl for `ReentrantLockGuard`](https://github.com/rust-lang/rust/pull/125527)
* [fix `VecDeque::shrink_to` UB when `handle_alloc_error` unwinds](https://github.com/rust-lang/rust/pull/123803)
* [simplify the `unchecked_sh[lr]` ub-checks a bit](https://github.com/rust-lang/rust/pull/125559)
* [less syscalls for the `copy_file_range` probe](https://github.com/rust-lang/rust/pull/122079)
* [make `clamp` inline](https://github.com/rust-lang/rust/pull/125455)
* [fix `c_char` on AIX](https://github.com/rust-lang/rust/pull/122986)
* [panic if `PathBuf::set_extension` would add a path separator](https://github.com/rust-lang/rust/pull/125070)
* [codegen\_llvm: add support for writing summary bitcode](https://github.com/rust-lang/rust/pull/125345)
* [codegen\_gcc: simd: implement pointer provenance intrinsics](https://github.com/rust-lang/rustc_codegen_gcc/pull/519)
* [rust-lld: fallback to rustc's sysroot if there's no path to the linker in the target sysroot](https://github.com/rust-lang/rust/pull/125263)
* [self-contained linker: retry linking without `-fuse-ld=lld` on CCs that don't support it](https://github.com/rust-lang/rust/pull/125417)
* [cargo: add more high level traces](https://github.com/rust-lang/cargo/pull/13951)
* [cargo: fetch specific commits even if the github fast path fails](https://github.com/rust-lang/cargo/pull/13946)
* [cargo: fix: check if rev is full commit sha for github fast path](https://github.com/rust-lang/cargo/pull/13969)
* [cargo: fix: remove symlink dir on Windows](https://github.com/rust-lang/cargo/pull/13910)
* [cargo: improve error description when deserializing partial field `struct`](https://github.com/rust-lang/cargo/pull/13956)
* [cargo: test: switch from `drop` to `let _` due to nightly rustc change](https://github.com/rust-lang/cargo/pull/13964)
* [cargo: upgrade gix from 0.62 to 0.63](https://github.com/rust-lang/cargo/pull/13948)
* [cargo: use `i32` rather than `usize` as "default integer" in library template](https://github.com/rust-lang/cargo/pull/13939)
* [clippy: `significant_drop_in_scrutinee`: Trigger lint only if lifetime allows early significant drop](https://github.com/rust-lang/rust-clippy/pull/12740)
* [clippy: add new lint `while_float`](https://github.com/rust-lang/rust-clippy/pull/12765)
* [clippy: add parentheses to `let_and_return`'s suggestion](https://github.com/rust-lang/rust-clippy/pull/12842)
* [clippy: bug fix: lint `numbered_fields` message error](https://github.com/rust-lang/rust-clippy/pull/12398)
* [clippy: correctly handle closing parens in `missing_backticks` doc lint](https://github.com/rust-lang/rust-clippy/pull/12809)
* [clippy: Quick Fix for bare URLs](https://github.com/rust-lang/rust-clippy/pull/12836)
* [clippy: fix `unnecessary_to_owned` interaction with macro expansion](https://github.com/rust-lang/rust-clippy/pull/12843)
* [clippy: fulfill expectations in `check_partial_eq_without_eq`](https://github.com/rust-lang/rust-clippy/pull/12841)
* [clippy: fulfill expectations in `check_unsafe_derive_deserialize`](https://github.com/rust-lang/rust-clippy/pull/12804)
* [clippy: suppress `iter_on_empty_collections` if the iterator's concrete type is relied upon](https://github.com/rust-lang/rust-clippy/pull/12823)
* [rust-analyzer: add `toggle_async_sugar` assist code action](https://github.com/rust-lang/rust-analyzer/pull/17258)
* [rust-analyzer: allow sysroots to only consist of the source root dir](https://github.com/rust-lang/rust-analyzer/pull/17287)
* [rust-analyzer: clear diagnostics only after new ones were received](https://github.com/rust-lang/rust-analyzer/pull/17248)
* [rust-analyzer: more callable info](https://github.com/rust-lang/rust-analyzer/pull/17268)
* [rust-analyzer: fix `data_constructor` ignoring generics for `struct`](https://github.com/rust-lang/rust-analyzer/pull/17291)
* [rust-analyzer: fix inconsistent cwd of `run` and `debug` command in client](https://github.com/rust-lang/rust-analyzer/pull/17275)
* [rust-analyzer: ensure implied bounds from associated types are considered in autocomplete](https://github.com/rust-lang/rust-analyzer/pull/17270)
* [rust-analyzer: fix `format_args` lowering passing incorrect parameters to `rustc_parse_format`](https://github.com/rust-lang/rust-analyzer/pull/17279)
* [rust-analyzer: infer type of async block with tail return expr](https://github.com/rust-lang/rust-analyzer/pull/17174)
* [rust-analyzer: resolve extern prelude for local mods in block modules](https://github.com/rust-lang/rust-analyzer/pull/17251)
* [rust-analyzer: use correct toolchain channel when generating builtin type doc links](https://github.com/rust-lang/rust-analyzer/pull/17284)
* [rust-analyzer: various find path fixes](https://github.com/rust-lang/rust-analyzer/pull/17277)
* [rust-analyzer: handle `{self}` when removing unused imports](https://github.com/rust-lang/rust-analyzer/pull/17140)
* [rust-analyzer: implement assist to switch between doc and normal comments](https://github.com/rust-lang/rust-analyzer/pull/17253)

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

Rusty Events between 2024-05-29 - 2024-06-26 🦀

### Virtual

* 2024-05-23 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477699/)
* 2024-05-23 | Virtual (Israel) | [Rust in Israel](https://rust.org.il/)
    * [**Web development in Rust using Rocket (Hebrew)**](https://www.meetup.com/code-mavens/events/300974367/)
* 2024-05-24 | Virtual (Rotterdam, NL)| [Bevy Game Development](https://www.meetup.com/bevy-game-development/)
    * [**Bevy Meetup #4**](https://www.meetup.com/bevy-game-development/events/300919821/)
* 2024-05-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/300533392/)
* 2024-05-28 & 2024-05-28 | Virtual | [Mainmatter](https://mainmatter.com/)
    * [**Remote Workshop: Telemetry for Rust APIs – you can't fix what you can't see (fee)**](https://ti.to/mainmatter/rust-telemetry-may-2024)
* 2024-05-29 | Virtual | [Training 4 Programmers LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Enums, Structs, and Traits - Essential Building Blocks of Rust Programming**](https://www.eventbrite.com/e/enums-structs-and-traits-essential-building-blocks-of-rust-programming-tickets-904696681127)
* 2024-05-30 | Virtual + In Person (Barcelona, ES) | [Mainmatter](https://mainmatter.com/) & [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**Rust for the web, Barcelona 2024**](https://www.meetup.com/es-ES/bcnrust/events/300765894/) 
* 2024-05-30 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298542326/)
* 2024-06-04 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Lunch & Learn: A Creative Thinker's Programming Language**](https://www.meetup.com/women-in-rust/events/300918713/)
* 2024-06-04 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300191681/)
* 2024-06-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047896/)
* 2024-06-06 | Virtual | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Rust Maven Workshop: Your first contribution to an Open Source Rust project**](https://www.meetup.com/code-mavens/events/301156302/)
* 2024-06-06 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477702/)
* 2024-06-11 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341709/)
* 2024-06-13 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897800/)
* 2024-06-13 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945258/)
* 2024-06-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346963/)
* 2024-06-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631733/)

### Africa

* 2024-06-01 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia

* 2024-05-22 | Singapore, SG | [SG Rust Meetup](https://www.meetup.com/rust-singapore/)
    * [**SG Rustaceans! Updated - SG Rust Meetup at CraftsforGreen Whole Studio**](https://www.meetup.com/rust-singapore/events/300988123/)

### Europe

* 2024-05-22 | Leiden, NL | [Future-proof Software Development by FreshMinds](https://www.meetup.com/freshminds-future-proof-software-development/)
    * [**Coding Dojo Session**](https://www.meetup.com/freshminds-future-proof-software-development/events/300566391/)
* 2024-05-23 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**2024 Rust Talks Bern #2**](https://www.meetup.com/rust-bern/events/300286917/)
* 2024-05-23 | Łodz, PL | [Mobica](https://www.linkedin.com/posts/mobica_rust-programming-embeddedsoftware-activity-7193232853717946369-CK68/)
    * [**Zapisz się na warsztat Rust / Embedded w Łodzi! / What's all the fuss about Rust?**](https://www.interankiety.pl/f/b4D7G7xO)
* 2024-05-23 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester May Code Night**](https://www.meetup.com/rust-manchester/events/300923207/)
* 2024-05-23 | Salzburg, AT | [SRUG: Salzburg Rust User Group]()
    * [**SRUG: Salzburg Rust User Group**](https://data.local.cccsbg.at/apps/calendar/p/jrFa8wz3zZQESKrQ/dayGridMonth/now/view/sidebar/L3JlbW90ZS5waHAvZGF2L3B1YmxpYy1jYWxlbmRhcnMvanJGYTh3ejN6WlFFU0tyUS8yOTRBQTkzMS1GQjFCLTRDODItQTM0My0yMzU5NEE0RDMyNEYuaWNz/1716480000)
* 2024-05-24 | Bordeaux, FR | [Rust Bordeaux](https://www.meetup.com/bordeaux-rust/)
    * [**Rust Bordeaux #3: Discussions**](https://www.meetup.com/bordeaux-rust/events/300723854/)
* 2024-05-25 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/)
    * [**Ferris' Fika Forum #3 [Embedded lab edition]**](https://www.meetup.com/stockholm-rust/events/301014982/)
* 2024-05-25 | Tampere, FI | [Finland Rust-lang Group](https://www.meetup.com/finland-rust-meetup/)
    * [**May Meetup**](https://www.meetup.com/finland-rust-meetup/events/301056235/)
* 2024-05-28 - 2024-05-30 | Berlin, DE | [Oxidize](https://oxidizeconf.com/)
    * [**Oxidize Conf 2024**](https://oxidizeconf.com/)
* 2024-05-30 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Developer Meetup @ Avalor AI**](https://www.meetup.com/rust-amsterdam-group/events/301065548/)
* 2024-05-30 | Barcelona, ES | [Mainmatter](https://mainmatter.com/) & [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**Rust for the web, Barcelona 2024**](https://www.meetup.com/es-ES/bcnrust/events/300765894/) 
* 2024-05-30 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299288963/)
* 2024-05-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #47 sponsored by Microsoft!**](https://www.meetup.com/copenhagen-rust-community/events/300458222/)
* 2024-05-30 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/300453310/)
* 2024-05-30 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/)
    * [**Rust Vienna Meetup - May - Rust Backend 101**](https://www.meetup.com/rust-vienna/events/301162548/)
* 2024-06-05 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn June 2024**](https://www.meetup.com/rust-meetup-hamburg/events/299235215/)
* 2025-06-06 | Vilnius, LT | [Rust Vilnius](https://www.meetup.com/rust-in-vilnius/)
    * [**Enjoy our second Rust and ZIG event**](https://www.meetup.com/rust-in-vilnius/events/301012097/)
* 2024-06-19 - 2024-06-24 | Zürich, CH | [RustFest Zürich](https://rustfest.ch/)
    * [**RustFest Zürich 2024**](https://rustfest.ch/)

### North America

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
* 2024-06-13 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/)
    * [**Monthly Meetup: Topic TBD!**](https://www.meetup.com/spokane-rust/events/300020010/)
* 2024-06-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186953/)

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

> I’ve said it before and I’ll say it again: as a child of OCaml and C++, Rust currently is the best language for production compiler-shaped things.

– [Alex Kladov on lobste.rs](https://lobste.rs/s/hjmrl1/how_we_migrated_our_static_analyzer_from#c_amxgiq)

Thanks to [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1570) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
