Title: This Week in Rust 669
Number: 669
Date: 2026-09-16
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/main/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

Want TWIR in your inbox? [Subscribe here](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the linked Page](https://example.com/my_article)

If you add a link to a non-text content please prefix it with `[video]` or `[audio]`:

* [video] [Title of the linked video](https://example.com/my_video_article)
* [audio] [Title of the linked audio file](https://example.com/my_podcast)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official

### Foundation

### Newsletters

* [Rust Trends Issue 82 - Even the Linker Is Getting Rewritten in Rust](https://rust-trends.com/newsletter/even-the-linker-is-getting-rewritten-in-rust/)
* [The Embedded Rustacean Issue #80](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-80)

### Project/Tooling Updates
* [Slint 1.18 Released](https://slint.dev/blog/slint-1.18-released)

<!-- IMPORTANT NOTE: We are no longer accepting pull request submissions for the Project/Tooling Updates section.
See here for details: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

### Observations/Thoughts

* [Where Does Rust Belong on Arduino? If it belongs.](https://talmondrlm.medium.com/where-does-rust-belong-on-arduino-if-it-belongs-325cbee63c1a)
* [CO3: Toward the Optimal FFI](https://mversic.github.io/co3/)
* [Why building a Rust LSP is hard · Rust Glancer](https://rust-glancer.github.io/blog/why-lsp-is-hard/)
* [Developing provably correct Rust code with Verus](https://www.amazon.science/blog/developing-provably-correct-rust-code-with-verus)
* [Principles for fast Tokio applications](https://dial9-rs.github.io/blog/principles-for-fast-tokio-applications/)

### Rust Walkthroughs

* [Trying to Make a Loop Auto-Vectorize](https://jsgroth.dev/blog/posts/trying-to-make-a-loop-auto-vectorize/)
* [Does Rust Support Inheritance? Yes, No, and Maybe, All in the Same File](https://msj.prose.sh/does-rust-support-inheritance)
* [Shipping Rust static libraries without symbol collisions](https://ai-coustics.com/blog/libpatcher)

### Rust Walkthroughs

* [A visual guide to Rust async](https://akesson.io/a-visual-guide-to-rust-async/)
* [Rust Projects - Write a Redis Clone - Version 3.0.0](https://rust-projects-write-a-redis-clone.github.io/#3.0.0)
* [Can You Use ESP32 as SWD Programmer for STM32 with Rust?](https://blog.implrust.com/posts/2026/09/swd-protocol-programmer-embedded-rust/)
* [Time and Panic Traps in WebAssembly: It Compiles, but It Crashes in the Browser](https://rust-blog.github.io/post/wasm-time-panic-traps)
* [One Lock to Rule Them All](https://flakm.com/posts/sqlx_migration_wrapper_til/)
* [video] [Your First GPUI App - Building a Desktop UI in Rust](https://www.youtube.com/watch?v=NT2XPvtof-Y)
* [Operators of death: checked arithmetic in Rust](https://bitfieldconsulting.com/posts/operators-of-death)
* [Rust generics: from Static to Dynamic dispatch](https://kerkour.com/rust-generics)

### Research

* [Optimizing a single Clippy lint by 3133X](https://blog.goose.love/posts/making-a-clippy-lint-faster-by-3133x/)

### Miscellaneous

## Crate of the Week

This week's crate is [zenjpeg](https://lib.rs/crates/zenjpeg), a pure Rust JPEG encoder and decoder.

Thanks to [Kornel](https://users.rust-lang.org/t/crate-of-the-week/2704/1669) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen)
* [Tracking Issue for trim-paths RFC 3127](https://github.com/rust-lang/cargo/issues/12137)
  * [Testing Instructions](https://github.com/rust-lang/cargo/issues/12137#issuecomment-5607218160)

*No calls for testing were issued this week by
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Rustup](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) or
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

523 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-09-08..2026-09-15

#### Compiler
* [garbage-collect old incremental compilation sessions](https://github.com/rust-lang/rust/pull/162240)
* [less clones and more cleanup for `rustc_builtin_macros`](https://github.com/rust-lang/rust/pull/162234)
* [perf: read hygiene data once when hashing syntax contexts](https://github.com/rust-lang/rust/pull/162571)
* [use `DenseBit` for `drop_live_at` in liveness tracing](https://github.com/rust-lang/rust/pull/162488)
* [use `Box<[Word]>` for word storage in `DenseBitSet`](https://github.com/rust-lang/rust/pull/161957)

#### Library
* [generalise (most) impls on `Box`](https://github.com/rust-lang/rust/pull/161946)
* [implement `Thread::os_id`](https://github.com/rust-lang/rust/pull/160219)
* [reserve items in `Extend` implementations](https://github.com/rust-lang/rust/pull/162495)
* [stabilize `Vec::from_fn`](https://github.com/rust-lang/rust/pull/162685)
* [stabilize `core::mem::DropGuard`](https://github.com/rust-lang/rust/pull/161520)
* [stabilize `unsafe_cell_access`](https://github.com/rust-lang/rust/pull/162504)

#### Cargo
* [`fix(git)`: For git cli, tell users what config we aren't forwarding on error](https://github.com/rust-lang/cargo/pull/17477)
* [`fix(install)`: use packaged lockfile by default](https://github.com/rust-lang/cargo/pull/17388)
* [fix(trim-paths)!: unremap file in one JSON doc](https://github.com/rust-lang/cargo/pull/17476)
* [lower the lint level of `manual_readme` and `non_kebab_case_bins` to `allow`](https://github.com/rust-lang/cargo/pull/17478)
* [specify `--edition` in `messages` tests](https://github.com/rust-lang/cargo/pull/17470)
* [test: add more comprehensive workspace feature unification tests](https://github.com/rust-lang/cargo/pull/17466)

#### Rustfmt
* [don't treat a raw identifier as a raw string prefix](https://github.com/rust-lang/rustfmt/pull/7110)
* [fix adjustment of `max_width` within macros](https://github.com/rust-lang/rustfmt/pull/6651)
* [reserve width for `const ` when formatting inline const blocks](https://github.com/rust-lang/rustfmt/pull/7065)

#### Clippy
* [`map_clone`: avoid suggestions after type-changing coercions](https://github.com/rust-lang/rust-clippy/pull/17671)
* [fix `collapsible_match` suggesting wrongly for conditional compiled code](https://github.com/rust-lang/rust-clippy/pull/16942)
* [enable `manual_swap` in const contexts](https://github.com/rust-lang/rust-clippy/pull/17703)

#### Rust-Analyzer
* [cache macro-expanded roots when climbing ancestors](https://github.com/rust-lang/rust-analyzer/pull/23359)
* [do not fill unstable methods in "Implement default members"](https://github.com/rust-lang/rust-analyzer/pull/23318)
* [do not panic on json with invalid field name](https://github.com/rust-lang/rust-analyzer/pull/23330)
* [don't panic on doc comments attached to literal expressions](https://github.com/rust-lang/rust-analyzer/pull/23295)
* [fix `hir::Type` owner mismatches between anon consts](https://github.com/rust-lang/rust-analyzer/pull/23315)
* [fix panic when trait solver re-enters itself](https://github.com/rust-lang/rust-analyzer/pull/23367)
* [fix panic when we call `impls_trait` for self type of builtin derive impls for generic types](https://github.com/rust-lang/rust-analyzer/pull/23352)
* [stop at eager macro recursion overflow](https://github.com/rust-lang/rust-analyzer/pull/23323)
* [ide: fix doc comment offset calculation](https://github.com/rust-lang/rust-analyzer/pull/23300)

### Rust Compiler Performance Triage

There were almost no regressions this week, and several performance improvements! Though some of them
were reverts of regressions from a previous week. [#162422](https://github.com/rust-lang/rust/pull/162422) improved the performance of Polonius, whose
performance is getting closer to the previous NLL borrow checker.

Triage done by **@Kobzol**.
Revision range: [656a9da1..20d35a3a](https://perf.rust-lang.org/?start=656a9da186dacaf3bf8f7f7296a825d256cb4ae3&end=20d35a3ae8f310f2a002e5f6e0bc583830010cd4&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | -     | -              | 0     |
| Regressions ❌ <br /> (secondary)  | 0.4%  | [0.1%, 0.9%]   | 3     |
| Improvements ✅ <br /> (primary)   | -0.7% | [-4.4%, -0.1%] | 199   |
| Improvements ✅ <br /> (secondary) | -0.9% | [-2.7%, -0.1%] | 222   |
| All ❌✅ (primary)                 | -0.7% | [-4.4%, -0.1%] | 199   |

0 Regressions, 5 Improvements, 5 Mixed; 2 of them in rollups
40 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/85651591cba06f7122ef35d459ab76b7f583bc48/triage/2026/2026-09-14.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Stabilize `debug_closure_helpers`](https://github.com/rust-lang/rust/pull/146099)
* [Additional NonZero conversions](https://github.com/rust-lang/rust/pull/129036)
* [Implement Default for NumBuffer](https://github.com/rust-lang/rust/pull/162536)
* [Allow elided ('static) lifetimes in `thread_local!`](https://github.com/rust-lang/rust/pull/159564)
* [Stabilize `funnel_shifts` (including `const`)](https://github.com/rust-lang/rust/pull/161015)
* [Stabilize `mem::conjure_zst`](https://github.com/rust-lang/rust/pull/161710)
* [Stabilize `Result::into_{ok,err}`](https://github.com/rust-lang/rust/pull/161712)
* [windows: stabilise inherit_handles](https://github.com/rust-lang/rust/pull/161163)
* [rustc: Stabilize the WebAssembly `wide-arithmetic` feature](https://github.com/rust-lang/rust/pull/160877)
* [Prevent mutating the global environment pointer in `CommandExt::exec` and opt to use execve and resolve path manually](https://github.com/rust-lang/rust/pull/157144)
* [alloc: stabilise `Allocator`](https://github.com/rust-lang/rust/pull/156882)
* [Disallow accesses through an Index projection when a sibling ConstantIndex projection has been moved out of](https://github.com/rust-lang/rust/pull/160780)
* [Allow unary operand types to be inferred later](https://github.com/rust-lang/rust/pull/159744)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [feat(config): Add build.profile, install.profile](https://github.com/rust-lang/cargo/pull/17215)
* [OUT_DIR is also set when running the program](https://github.com/rust-lang/cargo/issues/17456)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Allow observers to participate during meetings](https://github.com/rust-lang/leadership-council/pull/110)
* [Suggest discussing LC candidates with mods](https://github.com/rust-lang/leadership-council/pull/331)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Fix broken Zig link in RFC 3308 and typos across RFC texts](https://github.com/rust-lang/rfcs/pull/4006)

## Upcoming Events

Rusty Events between 2026-09-16 - 2026-10-14 🦀

### Virtual
* 2026-09-16 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Building a Rust GPU driver in the Linux kernel**](https://www.meetup.com/vancouver-rust/events/314233757/)
* 2026-09-17 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**September, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/315635881/)
* 2026-09-18 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/ibaxicxv)
* 2026-09-20 | Virtual (Bengaluru, IN) | [Embedded Rust Discord](https://discord.com/invite/pvYY69PvyS)
    * [**Silicon Sundays 3**](https://discord.gg/t9Cb2gjjq7?event=1546754977374932993)
* 2026-09-20 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/316133974/)
* 2026-09-22 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday Rust Bookclub**](https://www.meetup.com/dallasrust/events/310254773/)
* 2026-09-24 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/315907979/)
* 2026-09-24 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Rust Cells — Interior Mutability from Rust Core to Tock OS**](https://www.meetup.com/charlottesville-rust-meetup/events/316460694/)
* 2026-09-29 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Crates, Tips & Tricks Lightning Talks - Bring your ideas!**](https://www.meetup.com/women-in-rust/events/315691730/)
* 2026-09-30 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Operating Systems Book Club: Segmentation and Introduction to Paging**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316486941/)
* 2026-10-02 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/yqxvguts)
* 2026-10-04 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/316134009/)
* 2026-10-06 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315773044/)
* 2026-10-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcnbkb/)
* 2026-10-08 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/315907995/)
* 2026-10-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619617/)
* 2026-10-10 | Virtual (Gdansk, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto)
    * [**[BEZPŁATNIE] Programowanie w języku Rust**](https://www.meetup.com/stacja-it-trojmiasto/events/316381946/)
* 2026-10-13 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254772/)

### Europe
* 2026-09-14 - 2026-09-16 | Berlin, DE | [Oxidize 2026](https://oxidizeconf.com/)
    * [**Oxidize 2026**](https://oxidizeconf.com/)
* 2026-09-17 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - From Segfault to Safety @DiWoDo**](https://www.meetup.com/rust-dortmund/events/316428507/)
* 2026-09-22 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Prague @ Rockwell Automation**](https://www.meetup.com/rust-prague/events/316070376/)
* 2026-09-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Talk Night at SkyTEM**](https://www.meetup.com/rust-aarhus/events/316236528/)
* 2026-09-24 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group)
    * [**Rust Meetup @ BlockTech**](https://www.meetup.com/rust-amsterdam-group/events/316162802/)
* 2026-09-24 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**AI Agentic Coding**](https://www.meetup.com/rust-rhein-main/events/316328297/)
* 2026-09-26 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #30**](https://www.meetup.com/stockholm-rust/events/316570423/)
* 2026-09-28 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #21: Maximilian Grauvogl & Marcel Fink - From Bits to Bugs: A Rust generator for SUIT Manifests and Structure-Aware Parser Fuzzing**](https://rust-augsburg.github.io/meetup/Meetup_21.html)
* 2026-09-29 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester September Code Night**](https://www.meetup.com/rust-manchester/events/316200964/)
* 2026-09-30 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #16 @ ERNI**](https://www.meetup.com/rust-basel/events/315986893/)
* 2026-10-05 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 3**](https://www.meetup.com/rust-munich/events/316244709/)
* 2026-10-08 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/316564477/)
* 2026-10-10 | Geneva, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-10-14 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust)
    * [**22nd bcnrust session**](https://www.meetup.com/bcnrust/events/316316234/)

### North America
* 2026-09-16 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Building a Rust GPU driver in the Linux kernel**](https://www.meetup.com/vancouver-rust/events/314233757/)
* 2026-09-16 | San Francisco, CA, US | [Bay Area Rust](https://luma.com/bayarearust)
    * [**Bay Area Rust - Graphics Meetup**](https://luma.com/9oiujuyw)
* 2026-09-17 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**September, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/315635881/)
* 2026-09-17 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/316176445/)
* 2026-09-19 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Boston Common Rust Lunch, Sep 19**](https://www.meetup.com/bostonrust/events/316378813/)
* 2026-09-22 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/316572988/)
* 2026-09-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/316404827/)
* 2026-09-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyjcmbfc/)
* 2026-09-24 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539333/)
* 2026-09-26 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Harvard Rust Lunch, Sep 26**](https://www.meetup.com/bostonrust/events/316378817/)
* 2026-10-01 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Building a Minimal, Rootless Container in Rust**](https://www.meetup.com/stl-rust/events/316410027/)
* 2026-10-03 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Alewife Rust Lunch, Oct 3**](https://www.meetup.com/bostonrust/events/316378820/)
* 2026-10-08 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust October Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/316319730/)
* 2026-10-10 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Back Bay Rust Lunch, Oct 10**](https://www.meetup.com/bostonrust/events/316378823/)
* 2026-10-14 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA October: AI & Rust w/ Oxen.AI & Origin Lab!**](https://www.meetup.com/rust-los-angeles/events/315795432/)

### Oceania
* 2026-09-29 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**September Meetup**](https://www.meetup.com/rust-canberra/events/316398052/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> Every so often I am told about some maths fact that I can only assume someone went to prison for discovering

– [Simon Buchan](https://users.rust-lang.org/t/as-str-for-integers/142364/17)

Thanks to [Chayim Refael Friedman](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1799) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

This Week in Rust is edited by:

* [nellshamrell](https://github.com/nellshamrell)
* [llogiq](https://github.com/llogiq)
* [ericseppanen](https://github.com/ericseppanen)
* [extrawurst](https://github.com/extrawurst)
* [U007D](https://github.com/U007D)
* [mariannegoldin](https://github.com/mariannegoldin)
* [bdillo](https://github.com/bdillo)
* [opeolluwa](https://github.com/opeolluwa)
* [bnchi](https://github.com/bnchi)
* [KannanPalani57](https://github.com/KannanPalani57)
* [tzilist](https://github.com/tzilist)

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
