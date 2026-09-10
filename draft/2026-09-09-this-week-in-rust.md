Title: This Week in Rust 668
Number: 668
Date: 2026-09-09
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

* [Rust/C++ Interop Initiative: Progress Update, September 2026](https://rustfoundation.org/media/rust-cpp-interop-initiative-progress-update/)

### Newsletters
[Rust Trends Issue 81 - Rust 1.98 Ships as the Supply Chain Gets Tested](https://rust-trends.com/newsletter/rust-1-98-ships-as-the-supply-chain-gets-tested/)

### Project/Tooling Updates

### Observations/Thoughts

* [Microcontrollers with good support for Rust](https://kerkour.com/rust-microcontrollers)
* [What Does a Governed Data Runtime Cost? TeaQL vs Diesel and SeaORM on MusicBrainz](https://teaql.io/blog/musicbrainz-rust-orm-benchmark/)
* [Stabilizing Rust's never type](https://lwn.net/SubscriberLink/1091015/5009546caa744c57/)
* [Searching through 150 GiB of Text per Second with SIMD](https://pid7.com/blog/searching-150gb-text-per-second/)
* [Nine Rules for Compile-Time Work with Rust `const fn`: Parse files, build tables, and catch mistakes … without a build script (Part 2)](https://levelup.gitconnected.com/nine-rules-for-compile-time-work-with-rust-const-fn-part-2-76ccd0e8a965)

### Rust Walkthroughs

* [What Rust's +simd128 Actually Changed in My WebAssembly](https://www.debugdiary.dev/log/rust-simd128-what-changed-in-webassembly)
* [Rust Control Flow in Practice - Build a Number Guessing Game](https://blog.sheerluck.dev/posts/understanding-rust-control-flow-by-building-a-number-guessing-game/)
* [Gloo + Yew for persistent webapp state](https://hemomorphic.alexblood.net/posts/gloo-yew-for-persistent-webapp-state/)

### Research

### Miscellaneous

* [Awesome Rust Migrations](https://github.com/kevincouton/awesome-rust-migrations)

## Crate of the Week

This week's crate is [tokio-rcu](https://github.com/roeeshoshani/tokio_rcu), a user-space RCU implementation specifically built around the semantics of async rust and tokio.

Thanks to [Roee Shoshani](https://users.rust-lang.org/t/crate-of-the-week/2704/1662) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature need testing.

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen)
* [Tracking Issue for checksum freshness](https://github.com/rust-lang/cargo/issues/14136)

*No calls for testing were issued this week by
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Rustup](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) or
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.


## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
* [sysknife - action_reference_doc_is_current prints two 44 KB documents instead of the line that differs](https://github.com/lacs-project/sysknife/issues/345)
* [sysknife - packages/setup claims Node 18 support, and Node 18 has been end-of-life since 2025-04-30](https://github.com/lacs-project/sysknife/issues/327)
* [sysknife - cargo test fails intermittently on main: a test sets a process-global env var](https://github.com/lacs-project/sysknife/issues/356)
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

613 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-09-01..2026-09-08

#### Compiler
* [always rerun if we normalize local opaques](https://github.com/rust-lang/rust/pull/161795)
* [optimize empty token streams](https://github.com/rust-lang/rust/pull/162047)
* [store LiveLoans more densely packed](https://github.com/rust-lang/rust/pull/161850)
* [use IndexVec instead of BTreeMap for polonius variances](https://github.com/rust-lang/rust/pull/162422)

#### Library
* [add custom allocator support to `(try_)map` on `UniqueArc` and `UniqueRc`](https://github.com/rust-lang/rust/pull/161893)
* [box: fixup `map/try_map` deallocate calls](https://github.com/rust-lang/rust/pull/162285)
* [stabilize smart pointer map functions](https://github.com/rust-lang/rust/pull/160534)

#### Cargo
* [`docs(lints)`: how to configure Cargo lints](https://github.com/rust-lang/cargo/pull/17441)
* [`docs(trim-paths)`: add limitations and polish](https://github.com/rust-lang/cargo/pull/17425)
* [`docs(trim-paths)`: workspace remap begins with `.`](https://github.com/rust-lang/cargo/pull/17433)
* [`fix(git)`: Apply pr hint to git-fetch-with-cli](https://github.com/rust-lang/cargo/pull/17437)
* [`fix(git)`: Make PR dep note cloer to our style guide](https://github.com/rust-lang/cargo/pull/17436)
* [`fix(git)`: Simplify error message](https://github.com/rust-lang/cargo/pull/17429)
* [`fix(git)`: Use git's 429 retry, when available](https://github.com/rust-lang/cargo/pull/17422)
* [`fix(parser)`: Resolve theoretical use-after-free](https://github.com/rust-lang/cargo/pull/17428)
* [avoid passing search path (-L) args when they are passed as --extern](https://github.com/rust-lang/cargo/pull/17410)
* [docs: switch from "target triple" to "target tuple"](https://github.com/rust-lang/cargo/pull/17430)
* [fix relative symlink handling in `write_atomic`](https://github.com/rust-lang/cargo/pull/17362)
* [fix(trim-paths)!: limit options to `none|object|all`](https://github.com/rust-lang/cargo/pull/17432)
* [fix(trim-paths)!: remove default scope from release profile](https://github.com/rust-lang/cargo/pull/17424)
* [fixed stale comment about fingerprint checking method](https://github.com/rust-lang/cargo/pull/17450)

#### Rustdoc
* [add `--print` option](https://github.com/rust-lang/rust/pull/151618)

#### Rustfmt
* [fix non-idempotent block doc comment closer rewrite](https://github.com/rust-lang/rustfmt/pull/7017)
* [prevent infinite loops when parsing items from `cfg_select!` arms](https://github.com/rust-lang/rustfmt/pull/7089)

#### Clippy
* [`unnecessary_self_imports`: lint nested imports](https://github.com/rust-lang/rust-clippy/pull/17653)
* [`legacy_numeric_constants`: make fixes machine-applicable](https://github.com/rust-lang/rust-clippy/pull/17490)
* [`std_instead_of_core`: don't suggest a path that does not resolve](https://github.com/rust-lang/rust-clippy/pull/17648)
* [`useless_conversion`: ignore `From::from` in generated code](https://github.com/rust-lang/rust-clippy/pull/17583)
* [`useless_format`: improve suggestion](https://github.com/rust-lang/rust-clippy/pull/16595)
* [`regex_creation_in_loops`: check MIR loop structure](https://github.com/rust-lang/rust-clippy/pull/17681)
* [check that intra-doc links are not broken](https://github.com/rust-lang/rust-clippy/pull/17504)
* [detect integration tests in `is_in_test`](https://github.com/rust-lang/rust-clippy/pull/16786)
* [do not trigger `integer_division_remainder_used` in macros](https://github.com/rust-lang/rust-clippy/pull/17049)
* [improve `map_unwrap_or` lint to support `map(f).unwrap_or_default()`](https://github.com/rust-lang/rust-clippy/pull/17644)
* [move the `clippy_ci_panic_test` integration into a regular test](https://github.com/rust-lang/rust-clippy/pull/17502)
* [respect inline allows in `needless_pass_by_value`](https://github.com/rust-lang/rust-clippy/pull/17665)
* [soft rename `clippy::all` to `clippy::default`](https://github.com/rust-lang/rust-clippy/pull/14689)

#### Rust-Analyzer
* [add diagnostics for missing bodies for free and associated items](https://github.com/rust-lang/rust-analyzer/pull/23262)
* [fix `NamedTempFile` constructors](https://github.com/rust-lang/rust-analyzer/pull/23292)
* [accept Self as non-leading path segment in attribute paths](https://github.com/rust-lang/rust-analyzer/pull/23249)
* [allow inner attributes on blocks in tuple expressions](https://github.com/rust-lang/rust-analyzer/pull/23246)
* [avoid type unification errors in term search](https://github.com/rust-lang/rust-analyzer/pull/22662)
* [fix handling of `#[unsafe()]` attrs without inner meta](https://github.com/rust-lang/rust-analyzer/pull/23270)
* [fix parsing of `self:`: in fn param list](https://github.com/rust-lang/rust-analyzer/pull/23163)
* [hover `1f64` use float instead of integer](https://github.com/rust-lang/rust-analyzer/pull/23279)
* [follow symlinks when scanning the sysroot for proc-macro dylibs](https://github.com/rust-lang/rust-analyzer/pull/23297)
* [install cargo tools with locked dependencies](https://github.com/rust-lang/rust-analyzer/pull/23248)
* [merge `hir_def::hir::Expr::Unsafe` into `Expr::Block`](https://github.com/rust-lang/rust-analyzer/pull/23271)
* [render const value in completions label details](https://github.com/rust-lang/rust-analyzer/pull/23266)

### Rust Compiler Performance Triage

This week we've hit quite a few regressions, both expected and unexpected.
One of them has already been fixed, with fixes for a few others being discussed.
One big improvement comes from caching the sanitizer set in `Session`, which fixes a large regression from last week.
A few minor improvements landed, including a 75% reduction in memory usage while compiling `bevy_render` with the next trait solver.

Triage done by **@JonathanBrouwer**.
Revision range: [5321a4f4..656a9da1](https://perf.rust-lang.org/?start=5321a4f40c957cf3587c055e77461febc2ebc865&end=656a9da186dacaf3bf8f7f7296a825d256cb4ae3&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.5%  | [0.1%, 1.3%]   | 121   |
| Regressions ❌ <br /> (secondary)  | 0.6%  | [0.1%, 10.3%]  | 106   |
| Improvements ✅ <br /> (primary)   | -0.6% | [-1.9%, -0.1%] | 63    |
| Improvements ✅ <br /> (secondary) | -0.6% | [-2.4%, -0.1%] | 65    |
| All ❌✅ (primary)                 | 0.1%  | [-1.9%, 1.3%]  | 184   |


3 Regressions, 2 Improvements, 8 Mixed; 6 of them in rollups
33 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/a953e9c59b18feffa9dc06bee4ab30ad5a5700e8/triage/2026/2026-09-07.md)

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature need testing.

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen)
* [Tracking Issue for checksum freshness](https://github.com/rust-lang/cargo/issues/14136)

*No calls for testing were issued this week by
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Rustup](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) or
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

---

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Rustdoc LaTeX math](https://github.com/rust-lang/rfcs/pull/3958)
* [RFC: Cargo feature descriptions](https://github.com/rust-lang/rfcs/pull/3485)
* [Change `i686-pc-windows-msvc` from Tier 1 with host tools => Tier 1 without host tools](https://github.com/rust-lang/rfcs/pull/3999)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [riscv: stabilize 'd' and 'f' target features](https://github.com/rust-lang/rust/pull/161385)
* [x86: on targets that requires SSE, use those registers for ABI](https://github.com/rust-lang/rust/pull/161583)
* [Re-export `core::fmt::NumBuffer` in `alloc` (and `std`)](https://github.com/rust-lang/rust/pull/161430)
* [fix: unfulfilled nested dead code lint](https://github.com/rust-lang/rust/pull/161005)
* [turn aligned-in-packed error into lint](https://github.com/rust-lang/rust/pull/162160)
* [Guarantee 8 bytes of alignment of RawWakerVTable](https://github.com/rust-lang/rust/pull/158186)
* [libtest: Allow passing --test-threads and --color multiple times, with later arguments overriding earlier](https://github.com/rust-lang/rust/pull/161312)
* [Stabilize `core::mem::DropGuard`](https://github.com/rust-lang/rust/pull/161520)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [fix(install): use packaged lockfile by default](https://github.com/rust-lang/cargo/pull/17388)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [UB does not time travel](https://github.com/rust-lang/reference/pull/2320)

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Can references inside an enum transiently set an invalid enum discriminant?](https://github.com/rust-lang/unsafe-code-guidelines/issues/621)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2026-09-09 - 2026-10-07 🦀

### Virtual
* 2026-09-09 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Operating Systems Book Club: Address spaces and Memory API**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316283651/)
* 2026-09-10 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Solving Real-World Planning Problems in Rust with SolverForge**](https://luma.com/rfbzk3ae)
* 2026-09-10 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/315691423/)
* 2026-09-10 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619611/)
* 2026-09-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/fhvsztyjcmbtb/)
* 2026-09-16 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Building a Rust GPU driver in the Linux kernel**](https://www.meetup.com/vancouver-rust/events/314233757/)
* 2026-09-17 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**September, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/315635881/)
* 2026-09-18 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/ibaxicxv)
* 2026-09-20 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/316133974/)
* 2026-09-22 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday Rust Bookclub**](https://www.meetup.com/dallasrust/events/310254773/)
* 2026-09-24 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/315907979/)
* 2026-09-24 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Rust Cells — Interior Mutability from Rust Core to Tock OS**](https://www.meetup.com/charlottesville-rust-meetup/events/316460694/)
* 2026-09-29 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: Crates, Tips & Tricks Lightning Talks - Bring your ideas!**](https://www.meetup.com/women-in-rust/events/315691730/)
* 2026-10-02 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/yqxvguts)
* 2026-10-04 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/316134009/)
* 2026-10-06 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315773044/)
* 2026-10-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcnbkb/)

### Africa

### Europe
* 2026-10-10 | Geneva, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-09-14 - 2026-09-16 | Berlin, DE | [Oxidize 2026](https://oxidizeconf.com/)
    * [**Oxidize 2026**](https://oxidizeconf.com/)
* 2026-09-15 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Reproducing scientific papers - with Rust & "AI"**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816477/)
* 2026-09-15 | Madrid, ES | [MadRust](https://www.meetup.com/madrust/events/)
    * [**Tras la Máscara de Async Rust**](https://www.meetup.com/madrust/events/316361267/)
* 2026-09-17 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund/events/)
    * [**Rust Dortmund Meetup - From Segfault to Safety @DiWoDo**](https://www.meetup.com/rust-dortmund/events/316428507/)
* 2026-09-22 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Prague @ Rockwell Automation**](https://www.meetup.com/rust-prague/events/316070376/)
* 2026-09-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Talk Night at SkyTEM**](https://www.meetup.com/rust-aarhus/events/316236528/)
* 2026-09-24 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ BlockTech**](https://www.meetup.com/rust-amsterdam-group/events/316162802/)
* 2026-09-24 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**AI Agentic Coding**](https://www.meetup.com/rust-rhein-main/events/316328297/)
* 2026-09-29 | Manchester, GB | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester September Code Night**](https://www.meetup.com/rust-manchester/events/316200964/)
* 2026-09-30 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #16 @ ERNI**](https://www.meetup.com/rust-basel/events/315986893/)
* 2026-10-05 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2026 / 3**](https://www.meetup.com/rust-munich/events/316244709/)

### North America
* 2026-09-08 - 2026-09-11 | Hybrid (Montreal, CA) | [RustConf 2026](https://rustconf.com/)
    * [**RustConf 2026**](https://rustconf.com/)
* 2026-09-09 | Montreal, CA | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**RustConf Coffee Break Meetup**](https://www.meetup.com/women-in-rust/events/315773005/)
* 2026-09-10 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Hands-on Embedded Rust**](https://www.meetup.com/utah-rust/events/316198046/)
* 2026-09-10 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust September Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/315601104/)
* 2026-09-12 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Somerville Union Square Rust Lunch, Sep 12**](https://www.meetup.com/bostonrust/events/310983699/)
* 2026-09-15 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/314997217/)
* 2026-09-16 | San Francisco, CA, US | [Bay Area Rust](https://luma.com/bayarearust)
    * [**Bay Area Rust - Graphics Meetup**](https://luma.com/9oiujuyw)
* 2026-09-16 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Building a Rust GPU driver in the Linux kernel**](https://www.meetup.com/vancouver-rust/events/314233757/)
* 2026-09-17 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**September, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/315635881/)
* 2026-09-17 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/316176445/)
* 2026-09-19 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Boston Common Rust Lunch, Sep 19**](https://www.meetup.com/bostonrust/events/316378813/)
* 2026-09-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyjcmbfc/)
* 2026-09-24 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539333/)
* 2026-09-26 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Harvard Rust Lunch, Sep 26**](https://www.meetup.com/bostonrust/events/316378817/)
* 2026-10-01 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Building a Minimal, Rootless Container in Rust**](https://www.meetup.com/stl-rust/events/316410027/)
* 2026-10-03 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Alewife Rust Lunch, Oct 3**](https://www.meetup.com/bostonrust/events/316378820/)

### Oceania:
* 2026-09-29 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/events/)
    * [**September Meetup**](https://www.meetup.com/rust-canberra/events/316398052/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> The Demon has access to a Super Turing Machine.

– [Connor Horman on rust zulip](https://rust-lang.zulipchat.com/#narrow/channel/136281-t-opsem/topic/.E2.9C.94.20Can.20IO.20provide.20angelic.20choice/near/621828055)

Thanks to [Theemathas](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1798) for the suggestion!

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
