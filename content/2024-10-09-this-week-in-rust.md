Title: This Week in Rust 568
Number: 568
Date: 2024-10-09
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

### Foundation
* [Thanks For Joining Us at RustConf 2024](https://foundation.rust-lang.org/news/thanks-for-joining-us-at-rustconf-2024/)

### Newsletters
* [This Month in Rust OSDev: September 2024](https://rust-osdev.com/this-month/2024-09/)

### Project/Tooling Updates
* [Announcing zerocopy 0.8!](https://github.com/google/zerocopy/discussions/1680)
* [Introduction to durable execution](https://flawless.dev/docs/)
* [Pursuit of Performance on Building a JavaScript Compiler](https://oxc.rs/docs/learn/performance.html)
* [gitoxide September 2024](https://github.com/Byron/gitoxide/discussions/1614)
* [Tauri 2.0 Stable Release](https://v2.tauri.app/blog/tauri-20/)
* [A new version of modversions](https://lwn.net/Articles/986892/)
* [Smart pointers for the kernel](https://lwn.net/SubscriberLink/992055/104fe7d0d355faba/)
* [Efficient Rust tracepoints](https://lwn.net/SubscriberLink/992455/6c61de6764f17830/)
* [Improving bindgen for the kernel](https://lwn.net/SubscriberLink/992693/d4d6587f6faaf524/)
* [termscp 0.15.0](https://blog.veeso.dev/blog/en/announcing-termscp-015/)

### Observations/Thoughts
* [regalloc III](https://d-sonuga.netlify.app/gsoc/regalloc-iii/)
* [On Ousterhout's Dichotomy](https://matklad.github.io/2024/10/06/ousterhouts-dichotomy.html)
* [Rust is rolling off the Volvo assembly line](https://tweedegolf.nl/en/blog/137/rust-is-rolling-off-the-volvo-assembly-line)
* [Three Kinds Of Unwrap](https://zkrising.com/writing/three-unwraps/)
* [5 Awesome (and less known) Rust projects](https://kerkour.com/awesome-rust-projects-2024)
* [Nine Rules for Running Rust in the Browser: Practical lessons from porting range-set-blaze to WASM](https://towardsdatascience.com/nine-rules-for-running-rust-in-the-browser-8228353649d1)
* [video] [A Legendary Web Framework is Reborn... In Rust](https://www.youtube.com/watch?v=7utPutDORb4)
* [audio] [Fixing build times with rubicon](https://sdr-podcast.com/episodes/dynamic-linking/)

### Rust Walkthroughs
* [Building Async I/O in Rust: How Futures, Wakers, and Thread Pools Work Together](https://www.spaghetti-coder.com/building-async-io-in-rust-how-futures-wakers-and-thread-pools-work-together)
* [Index Trait, Pinned Elements and Immutable Push Vector](https://orxfun.github.io/orxfun-notes/#/imp-vec-motivation-2024-10-03)
* [series] [Serde for Trait Object 2: Serialization](https://voelklmichael.github.io/Blog/2024/10/08/serde-trait-part2.html)
* [video] [Build with Naz : Create an async shell in Rust](https://www.youtube.com/watch?v=jXzFCDIJQag)

### Miscellaneous
* [September 2024 Rust Jobs Report](https://filtra.io/rust-sep-24)

## Crate of the Week

This week's crate is [float8](https://crates.io/crates/float8), an 8-bit float implementation.

llogiq is still pleased with his choice, but increasingly unhappy about the lack of suggestions.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No calls for testing were issued this week.*

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No calls for testing were issued this week.*

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No calls for testing were issued this week.*

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

437 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-10-01..2024-10-08

* [add `x86_64-unknown-trusty` as tier 3 target](https://github.com/rust-lang/rust/pull/130453)
* [initial support for `riscv32{e|em|emc}_unknown_none_elf`](https://github.com/rust-lang/rust/pull/130555)
* [increase Stack Size for AIX](https://github.com/rust-lang/rust/pull/131116)
* [allow boolean literals as cfg predicates](https://github.com/rust-lang/rust/pull/131034) (RFC [#3695](https://rust-lang.github.io/rfcs/3695-cfg-boolean-literals.html))
* [account for `impl Trait {` when `impl Trait for Type {` was intended](https://github.com/rust-lang/rust/pull/131273)
* [add `naked_asm!` macro for use in `#[naked]` functions](https://github.com/rust-lang/rust/pull/128651)
* [add a lint for pointer to integer transmutes in consts](https://github.com/rust-lang/rust/pull/130540)
* [add caching to most type folders, rm region uniquification](https://github.com/rust-lang/rust/pull/130821)
* [add missing module flags for `-Zfunction-return=thunk-extern`](https://github.com/rust-lang/rust/pull/130824)
* [add support for reborrowing pinned method receivers](https://github.com/rust-lang/rust/pull/130633)
* [add unstable support for outputting file checksums for use in cargo](https://github.com/rust-lang/rust/pull/126930)
* [avoid ICE in coverage builds with bad `#[coverage(..)]` attributes](https://github.com/rust-lang/rust/pull/131187)
* [check elaborated projections from dyn don't mention unconstrained late bound lifetimes](https://github.com/rust-lang/rust/pull/130367)
* [compute array length from type for unconditional panic lint](https://github.com/rust-lang/rust/pull/129517)
* [couple of changes to make it easier to compile rustc for wasm](https://github.com/rust-lang/rust/pull/130899)
* [coverage: multiple small tweaks to counter creation](https://github.com/rust-lang/rust/pull/131325)
* [disable jump threading `UnOp::Not` for non-bool](https://github.com/rust-lang/rust/pull/131201)
* [do not consider match/let/ref of place that evaluates to `!` to diverge, disallow coercions from them too](https://github.com/rust-lang/rust/pull/129392)
* [don't allow the `#[pointee]` attribute where it doesn't belong](https://github.com/rust-lang/rust/pull/128721)
* [don't give method suggestions when method probe fails due to bad implementation of `Deref`](https://github.com/rust-lang/rust/pull/131024)
* [improve const traits diagnostics for new desugaring](https://github.com/rust-lang/rust/pull/131152)
* [instantiate binders in `supertrait_vtable_slot`](https://github.com/rust-lang/rust/pull/131042)
* [make `deprecated_cfg_attr_crate_type_name` a hard error](https://github.com/rust-lang/rust/pull/129670)
* [make `test_lots_of_insertions` test take less long in Miri](https://github.com/rust-lang/rust/pull/131085)
* [make opaque types regular HIR nodes](https://github.com/rust-lang/rust/pull/129244)
* [only query `params_in_repr` if def kind is adt](https://github.com/rust-lang/rust/pull/131150)
* [panic when an interpreter error gets unintentionally discarded](https://github.com/rust-lang/rust/pull/130885)
* [parser: better error messages for `@` in `struct` patterns](https://github.com/rust-lang/rust/pull/130725)
* [replace -Z default-hidden-visibility with -Z default-visibility](https://github.com/rust-lang/rust/pull/130005)
* [restrict `ignore-mode-*` directives](https://github.com/rust-lang/rust/pull/131346)
* [support `clobber_abi` and vector/access registers (clobber-only) in s390x inline assembly](https://github.com/rust-lang/rust/pull/130630)
* [interpret: always enable `write_immediate` sanity checks](https://github.com/rust-lang/rust/pull/131006)
* [miri: add vector clock to epoll ready lists](https://github.com/rust-lang/miri/pull/3932)
* [miri: added rust-analyzer instructions for Helix](https://github.com/rust-lang/miri/pull/3936)
* [miri: avoid `pthread_attr_t` in tests](https://github.com/rust-lang/miri/pull/3945)
* [miri: implement LLVM x86 gfni intrinsics](https://github.com/rust-lang/miri/pull/3895)
* [miri: prefer refutable slice patterns over len check + index op](https://github.com/rust-lang/miri/pull/3940)
* [miri: pthread mutex: better error in reentrant-locking-UB](https://github.com/rust-lang/miri/pull/3943)
* [`rustc_infer` cleanups](https://github.com/rust-lang/rust/pull/131226)
* [stabilize 5 `const_mut_refs`-dependent API](https://github.com/rust-lang/rust/pull/131177)
* [stabilize `BufRead::skip_until`](https://github.com/rust-lang/rust/pull/131267)
* [stabilize `const_float_classify`](https://github.com/rust-lang/rust/pull/130157)
* [stabilize `const_slice_from_raw_parts_mut`](https://github.com/rust-lang/rust/pull/130403)
* [stabilize `const_slice_split_at_mut` and `const_slice_first_last_chunk`](https://github.com/rust-lang/rust/pull/130428)
* [stabilize `expr_2021` fragment specifier in all editions](https://github.com/rust-lang/rust/pull/129972)
* [stabilize the `map`/`value` methods on `ControlFlow`](https://github.com/rust-lang/rust/pull/130518)
* [liballoc: introduce String, Vec const-slicing](https://github.com/rust-lang/rust/pull/128399)
* [make Cell unstably const](https://github.com/rust-lang/rust/pull/131281)
* [enable f16 and f128 on windows-gnullvm targets](https://github.com/rust-lang/rust/pull/131308)
* [transmuteFrom: gracefully handle unnormalized types and normalization errors](https://github.com/rust-lang/rust/pull/131112)
* [small optimization for integers Display implementation](https://github.com/rust-lang/rust/pull/128204)
* [add `[Option<T>; N]::transpose`](https://github.com/rust-lang/rust/pull/130829)
* [add precondition checks to `ptr::offset, ptr::add, ptr::sub`](https://github.com/rust-lang/rust/pull/130251)
* [avoid emptiness check in `PeekMut::pop`](https://github.com/rust-lang/rust/pull/131197)
* [don't use `Immediate::offset` to transmute pointers to integers](https://github.com/rust-lang/rust/pull/131068)
* [add multi-producer, multi-consumer channel (mpmc)](https://github.com/rust-lang/rust/pull/126839)
* [impl `Default` for `HashMap`/`HashSet` iterators that don't already have it](https://github.com/rust-lang/rust/pull/128711)
* [std: make `thread::current` available in all `thread_local!` destructors](https://github.com/rust-lang/rust/pull/127912)
* [std: replace `LazyBox` with `OnceBox`](https://github.com/rust-lang/rust/pull/131094)
* [futures: fix use after free of task in FuturesUnordered when dropped future panics](https://github.com/rust-lang/futures-rs/pull/2886)
* [hashbrown: add `Tag(u8)` newtype in an attempt to stop using byte-pointers for everything](https://github.com/rust-lang/hashbrown/pull/565)
* [hashbrown: change the default hasher to foldhash](https://github.com/rust-lang/hashbrown/pull/563)
* [cargo: fix `cargo:version_number` - has only one `:`](https://github.com/rust-lang/cargo/pull/14637)
* [cargo: fix: remove implicit feature removal](https://github.com/rust-lang/cargo/pull/14630)
* [cargo: improve error reporting when feature not found in `activated_features`](https://github.com/rust-lang/cargo/pull/14647)
* [rustdoc: cleaner errors on disambiguator/namespace mismatches](https://github.com/rust-lang/rust/pull/131260)
* [rustdoc: improve `<wbr>`-insertion for `SCREAMING_CAMEL_CASE`](https://github.com/rust-lang/rust/pull/131370)
* [rustdoc: lists items that contain multiple paragraphs are more clear](https://github.com/rust-lang/rust/pull/130933)
* [rustdoc: prevent ctors from resolving](https://github.com/rust-lang/rust/pull/131224)
* [clippy: `infinite_loop`: continuing an outer loop leaves the inner loop](https://github.com/rust-lang/rust-clippy/pull/13512)
* [clippy: `rustc_tools_util`: rerun when git commit changes](https://github.com/rust-lang/rust-clippy/pull/13329)
* [clippy: `zombie_processes`: consider `wait()` calls in nested bodies](https://github.com/rust-lang/rust-clippy/pull/13462)
* [clippy: compare trait references in `trait_duplication_in_bounds` correctly](https://github.com/rust-lang/rust-clippy/pull/13493)
* [clippy: fix `mut_mutex_lock` when reference not ultimately mutable](https://github.com/rust-lang/rust-clippy/pull/13122)
* [clippy: implement lint for `regex::Regex` compilation inside a loop](https://github.com/rust-lang/rust-clippy/pull/13412)
* [clippy: reduce default 'large array' threshold](https://github.com/rust-lang/rust-clippy/pull/13485)
* [clippy: show interior mutability chain in `mutable_key_type`](https://github.com/rust-lang/rust-clippy/pull/13496)
* [clippy: simplify negative `Option::{is_some_and,is_none_or}`](https://github.com/rust-lang/rust-clippy/pull/13443)
* [clippy: style: do not defensively use `saturating_sub()`](https://github.com/rust-lang/rust-clippy/pull/13513)
* [rust-analyzer: fix: fix bootstrap error message being incorrect](https://github.com/rust-lang/rust-analyzer/pull/18219)
* [rust-analyzer: use external stack in borrowck DFS](https://github.com/rust-lang/rust-analyzer/pull/18255)

### Rust Compiler Performance Triage

One regression dominated this week (dealing with a correctness fix around type system caching that was deemed necessary), but it luckily did not produce large regressions in any benchmarks. Overall, performance still ended up relatively in the same place as the beginning of the week.

Triage done by **@rylev**.
Revision range: [c87004a1..e6c46db4](https://perf.rust-lang.org/?start=c87004a1f5be671e3f03f69fb13d8915bdbb6a52&end=e6c46db4e9fd11e3183c397a59d946731034ede6&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.3%  | [0.1%, 1.0%]   | 63    |
| Regressions ❌ <br /> (secondary)  | 1.1%  | [0.1%, 3.4%]   | 81    |
| Improvements ✅ <br /> (primary)   | -0.5% | [-3.0%, -0.1%] | 19    |
| Improvements ✅ <br /> (secondary) | -0.5% | [-1.5%, -0.1%] | 46    |
| All ❌✅ (primary)                 | 0.1%  | [-3.0%, 1.0%]  | 82    |


2 Regressions, 3 Improvements, 7 Mixed; 3 of them in rollups
57 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/e6fcc69d6b3483f737140ff5c9fdba1ccac44776/triage/2024-10-08.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Allow boolean literals as `cfg` predicates](https://github.com/rust-lang/rfcs/pull/3695)
* [Move `rustdoc-types` crate to T-Rustdoc ownership.](https://github.com/rust-lang/rfcs/pull/3505)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *No RFCs entered Final Comment Period this week.*

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Stabilize const `ptr::write*` and `mem::replace`](https://github.com/rust-lang/rust/pull/130954)
* [disposition: merge] [Check ABI target compatibility for function pointers](https://github.com/rust-lang/rust/pull/128784)
* [disposition: merge] [Proposal: stabilize `if_let_rescope` for Edition 2024](https://github.com/rust-lang/rust/issues/131154)
* [disposition: merge] [Fixup Windows verbatim paths when used with the `include!` macro](https://github.com/rust-lang/rust/pull/125205)
* [disposition: merge] [Implemented `FromStr` for `CString` and `TryFrom<CString>` for `String`](https://github.com/rust-lang/rust/pull/130608)
* [disposition: merge] [Tracking Issue for `debug_more_non_exhaustive`](https://github.com/rust-lang/rust/issues/127942)
* [disposition: merge] [Tracking Issue for `const_make_ascii`](https://github.com/rust-lang/rust/issues/130698)
* [disposition: merge] [Tracking Issue for `const_char_encode_utf8`](https://github.com/rust-lang/rust/issues/130512)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [feat: Stabilize MSRV-aware resolver config](https://github.com/rust-lang/cargo/pull/14639)
* [disposition: merge] [Official API for build scripts](https://github.com/rust-lang/cargo/issues/12432)

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* [disposition: merge] [Meeting proposal: rename "object safety" to "dyn compatibility"](https://github.com/rust-lang/lang-team/issues/286)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC: Promote riscv64gc-unknown-linux-gnu to Tier-1 (without host tools)](https://github.com/rust-lang/rfcs/pull/3707)
* [new] [[RFC] Add Option::todo and Result::todo](https://github.com/rust-lang/rfcs/pull/3706)

## Upcoming Events

Rusty Events between 2024-10-09 - 2024-11-06 🦀

### Virtual
* 2024-10-10 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Part 2 of 4 - Navigating Rust Web Frameworks: Axum, Actix, and Rocket**](https://www.meetup.com/women-in-rust/events/303213792/)
* 2024-10-10 | Virtual (Barcelona, ES) | [BcnRust](https://bcnrust.github.io) + [Codurance](https://www.codurance.com/) + [Heavy Duty Builders](https://heavyduty.builders/)
    * [**15th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/303443195/)
* 2024-10-10 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633270/)
* 2024-10-10 | Virtual (Girona, ES) | [Rust Girona](https://www.meetup.com/rust-girona/)
    * [**Leveraging Rust to Improve Your Programming Fundamentals & De Rust A Solana**](https://www.meetup.com/rust-girona/events/303484509/)
* 2024-10-10 - 2024-10-11 | Virtual and In-Person (Vienna, AT) | [Euro Rust](eurorust)
    * [**Euro Rust 2024**](https://eurorust.eu/)
* 2024-10-14 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/302828025/)
* 2024-10-14 | Virtual | [Rust for Lunch](https://lunch.rs/)
    * [**Rust for Lunch - Getting started with embedded Rust (Speaker: Sandro Stikić)**](https://lunch.rs/meetups/2024-07-09/) | [Online meeting link](https://lecture.senfcall.de/hay-gmh-wox-mru)
* 2024-10-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346970/)
* 2024-10-16 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631737/)
* 2024-10-17 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Part 3 of 4 - Hackathon Ideation Lab**](https://www.meetup.com/women-in-rust/events/303213817/)
* 2024-10-17| Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898023)
* 2024-10-22 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/fkmcntygcnbdc/)
* 2024-10-24 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Part 4 of 4 - Hackathon Showcase: Final Projects and Presentations**](https://www.meetup.com/women-in-rust/events/303213835/)
* 2024-10-24 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633271/)
* 2024-10-26 | Virtual (Gdansk, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-trojmiasto/events/303550643/)
* 2024-10-29 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/301585671/)
* 2022-10-31 | Virtual (Nürnberg, DE) | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820274/)

### Africa
* 2024-11-02 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia
* 2024-10-09 | Subang Jaya / Kuala Lumpur, Selangor, MY | [Rust Malaysia](https://rust-malaysia.github.io/meetup/)
    * [**Rust Malaysia Meetup - Traits and How to Read Trait (October 2024)**](https://docs.google.com/forms/d/e/1FAIpQLScNS5IWmnzTTJAOw-RIxdj4_BWbxB5NVmAVO30XHr_viMbLqQ/viewform)
* 2024-10-17 - 2024-10-18 | Beijing, CN | [Global Open-Source Innovation Meetup (GOSIM)](https://www.gosim.org/)
    * [**GOSIM 2024**](https://china2024.gosim.org/)
* 2024-10-19 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**October 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/october-2024-rustacean-meetup/)

### Europe
* 2024-10-09 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtygcnbmb/)
* 2024-10-10 - 2024-10-11 | Virtual and In-Person (Vienna, AT) | [Euro Rust](eurorust)
    * [**Euro Rust 2024**](https://eurorust.eu/)
* 2024-10-15 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425056/)
* 2024-10-17 | Darmstadr, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Rust Code Together**](https://www.meetup.com/rust-rhein-main/events/303240000/)
* 2024-10-15 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/303606799/)
* 2024-10-15 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425056/)
* 2024-10-15 | Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 3 - hybrid**](https://www.meetup.com/rust-munich/events/303273953/)
* 2024-10-16 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester Talks October - Leptos and Crux**](https://www.meetup.com/rust-manchester/events/303658240/)
* 2024-10-17 | Barcelona, ES | [BcnRust](https://bcnrust.github.io)
    * [**16th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/303792888/)
* 2024-10-17 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**2024 Rust Talks Bern #3**](https://www.meetup.com/rust-bern/events/303617330/)
* 2024-10-22 | Warsaw, PL | [Rust Warsaw](https://www.meetup.com/rust-warsaw/)
    * [**New Rust Warsaw Meetup #2**](https://www.meetup.com/rust-warsaw/events/303619536/)
* 2024-10-26 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Ferris' Fika Forum #6**](https://www.meetup.com/stockholm-rust/events/303918943/)
* 2024-10-28 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust Meetup #71**](https://www.meetup.com/rust-paris/events/303663366/)
* 2024-10-29 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/303479865)
* 2024-10-30 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn October 2024**](https://www.meetup.com/rust-meetup-hamburg/events/303373054/)
* 2024-10-31 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/300820289/)
* 2024-11-06 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.fr/o/paris-rustaceans-74289178383)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1037795553437)

### North America
* 2024-10-15 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638250/)
* 2024-10-16 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631737/)
* 2024-10-17 | Virtual and In-Person (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**October Meetup**](https://www.meetup.com/join-srug/events/303545170/)
* 2024-10-19 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**North End Rust Lunch, Oct 19**](https://www.meetup.com/bostonrust/events/303708335/)
* 2024-10-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcnbfc/)
* 2024-10-27 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Kendall Rust Lunch, Oct 27**](https://www.meetup.com/bostonrust/events/303708359/)
* 2024-11-04 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, Nov 4**](https://www.meetup.com/bostonrust/events/303708387/)

### Oceania
* 2024-10-29 | Canberra, ACT, AU | [Canberra Rust User Group (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/303128131/)

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

> I'm the wrong side of 45. I have zero interest in wasting any time that I might have left writing C from scratch. Writing Rust is pure joy. I can go from an idea to a working, tested, robust, published and packaged implementation in the time it would take me to even begin the first few lines of a C version. The tooling is beautiful, makes programming fun, and the end result usually outperforms the equivalent C. Once it builds I know it will run perfectly on all of the platforms I care about, and I don't have to go around manually testing on them to find all of the various platform and compiler quirks that will break it.

– [Jonathan Perkins on the NetBSD mailing list](http://mail-index.netbsd.org/pkgsrc-users/2024/08/25/msg040053.html)

Thanks to [blonk](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1617) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1g0bncp/this_week_in_rust_568/)</small>
