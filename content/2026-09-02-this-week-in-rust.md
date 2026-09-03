Title: This Week in Rust 667
Number: 667
Date: 2026-09-02
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

### Official

* [Announcing rustup 1.29.1](https://blog.rust-lang.org/2026/09/01/Rustup-1.29.1/)
* [Electing new Project Directors 2026](https://blog.rust-lang.org/inside-rust/2026/08/28/electing-new-project-directors-2026/)
* [Program management in July–August 2026](https://blog.rust-lang.org/inside-rust/2026/08/31/program-management-2026-jul-aug/)

### Foundation

* [How the Rust Standard Library Verification Contest Scaled Past Manual Proof Engineering](https://rustfoundation.org/media/how-the-rust-standard-library-verification-contest-scaled-past-manual-proof-engineering/)
* [Welcoming Rust Program Manager, Tomáš Šedovič, to the Rust Foundation Team!](https://rustfoundation.org/media/welcoming-rust-program-manager-tomas-sedovic-to-the-rust-foundation-team/)
* [Welcoming Jess Izen as Engineer in Residence at the Rust Foundation](https://rustfoundation.org/media/welcoming-jess-izen-as-engineer-in-residence-at-the-rust-foundation/)

### Newsletters

* [The Embedded Rustacean Issue #79](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-79)

### Project/Tooling Updates

* [Wasmi 2.0 - Engineering of the Fastest Wasm Interpreters](https://wasmi-labs.github.io/blog/posts/wasmi-v2.0/)
* [How I made Rustdoc 33% faster in one week](https://noahlev.org/blog/2026/08/27/making-rustdoc-faster/)
* [A Self-Baked Async FFI Framework for Rust C# Interop](https://www.scylladb.com/2026/08/31/async-ffi-framework-for-rust-c-interop/)

### Observations/Thoughts

* [Could Cargo's scheduler be better?](https://spirali.github.io/blog/cargo-scheduler/)
* [How we developed the world's first safety-certified product written in Rust – and why we went bare metal](https://www.sonair.com/journal/how-we-safety-certified-the-worlds-first-rust-implementation)
* [How we saved 100 terabytes of memory by optimizing 1.1.1.1’s DNS cache](https://blog.cloudflare.com/dns-cache-memory-optimization-1111/)
* [Rust concurrency vs Go concurrency: stackless vs stackfull coroutines](https://kerkour.com/rust-vs-go-concurrency)
* [Nine Rules for Compile-Time Work with Rust const fn: Parse files, build tables, and catch mistakes … without a build script  (Part 1)](https://medium.com/@carlmkadie/nine-rules-for-compile-time-work-with-rust-const-fn-part-1-a29f7dd62b2f)
* [video] [An Agentic VM in Rust — by Cristian Sánchez](https://www.youtube.com/watch?v=d7jQXm1KcOA)
* [video] [Making Progress on AsyncIterator — by Jack O'Connor](https://www.youtube.com/watch?v=zQAia_u8WPM)

### Rust Walkthroughs

* [The 'rnull' Rust block driver](https://lwn.net/SubscriberLink/1090378/b33a6fe3f4033507/)
* [Your First GPUI App - Building a Desktop UI in Rust](https://blog.sheerluck.dev/posts/your-first-gpui-app-building-a-desktop-ui-in-rust/)
* [Welcome to the machine: emulating a CPU](https://bitfieldconsulting.com/posts/welcome-to-machine)
* [Proving my Rust NVR doesn't leak memory (it did)](https://murlet.com/blog/hunting-rust-memory-leaks/)
* [Software That Must Not Be Wrong: Property Tests for a Pediatric Dosing Calculator](https://rust-blog.github.io/post/favi-child-property-tests)
* [Drawing Shapes with GPUI's Canvas](https://hlcfan.github.io/gpui-canvas-shapes.html)
* [Pinning Down Rust’s Pin](https://gmcgoldr.github.io/2026/08/27/pin-in-rust.html)

### Research

* [Functional State Machines in Rust: Typestate and Newtype Patterns](https://dl.acm.org/doi/10.1145/3830438.3830958)

### Miscellaneous

* [Ubuntu Rust: How Canonical Is Modernizing Core System Tools](https://blog.jetbrains.com/rust/2026/08/26/ubuntu-rust/)

## Crate of the Week

This week's crate is [buf\_read\_splitter](https://crates.io/crates/buf_read_splitter), a crate to split (separate) a stream into sub-subtreams.

Thanks to [flo](https://users.rust-lang.org/t/crate-of-the-week/2704/1659) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

*No calls for testing were issued this week by
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Rustup](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) or
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Zynlex - Add case-sensitive and whole-word toggles to find-in-page](https://github.com/webtools-dotcom/Zynlex/issues/13)
* [HookEcho - High-contrast theme: colormaps and stroke widths do not respond to it](https://github.com/d4vid87/hookecho/issues/12)
* [sysknife - UfwDeleteRule needs a rule number that no action in the catalogue can produce](https://github.com/lacs-project/sysknife/issues/234)
* [sysknife - peer_pidfd cannot tell a pre-6.5 kernel from a peer that already exited, so the PID-reuse check is off in the reuse case](https://github.com/lacs-project/sysknife/issues/250)
* [sysknife - The signed trail names which account asked for a change, never which one approved it](https://github.com/lacs-project/sysknife/issues/249)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

522 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-08-25..2026-09-01

#### Compiler
* [compiler: allow safestack to be togglable via `#[sanitize(safestack = "...")]`](https://github.com/rust-lang/rust/pull/161888)
* [perf: push nominal obligations instead of returning them](https://github.com/rust-lang/rust/pull/160473)
* [reduce perf impact of scalar size checks](https://github.com/rust-lang/rust/pull/161456)
* [remove unneeded clone in macro deriving](https://github.com/rust-lang/rust/pull/162004)

#### Library
* [add custom allocators to `(try_)map` on `Box`, Rc`, Arc`](https://github.com/rust-lang/rust/pull/161617)
* [add intrinsics for integer minimum and maximum](https://github.com/rust-lang/rust/pull/161081)
* [add SVE-accelerated `Vec::retain_mut` for aarch64](https://github.com/rust-lang/rust/pull/161034)
* [alloc `String::retain` optimization](https://github.com/rust-lang/rust/pull/150067)
* [core: expose volatile atomic operations](https://github.com/rust-lang/rust/pull/161301)
* [implement `[u8]::split_ascii_whitespace`](https://github.com/rust-lang/rust/pull/161577)
* [implement `clamp_to`](https://github.com/rust-lang/rust/pull/150075)

#### Cargo
* [diag: Stabilize cargo-lints](https://github.com/rust-lang/cargo/pull/17298)
* [resolver: Stabilize min-publish-age](https://github.com/rust-lang/cargo/pull/17335)
* [run: Printing a new line to avoid overwriting error code after \r](https://github.com/rust-lang/cargo/pull/17373)
* [trim-paths: custom workspace-relative member paths remap](https://github.com/rust-lang/cargo/pull/17366)
* [`perf(git)`: Reduce extra work when using git-cli](https://github.com/rust-lang/cargo/pull/17406)
* [cargo profiling improvements](https://github.com/rust-lang/cargo/pull/17411)
* [manifest!: implement feature-metadata RFC3416](https://github.com/rust-lang/cargo/pull/15056)
* [perf: do not build SBOM if user has not set build.sbom](https://github.com/rust-lang/cargo/pull/17412)

#### Rustdoc
* [correctly handle when a macro generates multiple items in `--generate-macro-expansion`](https://github.com/rust-lang/rust/pull/161876)
* [fix lint `cargo::non_kebab_case_bins`](https://github.com/rust-lang/rust/pull/161843)
* [take into account edition information for keyword highlighting](https://github.com/rust-lang/rust/pull/161944)

#### Rustfmt
* [allow users to set the release channel when running the diff check](https://github.com/rust-lang/rustfmt/pull/7080)

#### Clippy
* [`fix(cargo_common_metadata)`: stop checking `package.readme`](https://github.com/rust-lang/rust-clippy/pull/17643)
* [`fix(redundant_clone)`: Make `visit_local_usage` analyse loop bodies instead of giving up on them](https://github.com/rust-lang/rust-clippy/pull/17495)
* [add comma to `lint_groups_priority` error message](https://github.com/rust-lang/rust-clippy/pull/17663)
* [fix `--explain` lint lookup being case-mismatched](https://github.com/rust-lang/rust-clippy/pull/17632)
* [fix false positive for never type impls](https://github.com/rust-lang/rust-clippy/pull/17163)
* [`missing_transmute_annotations` should not contain fn name in suggestion](https://github.com/rust-lang/rust-clippy/pull/17611)
* [perf: bail early in `too_many_lines` lint if rule is not enabled](https://github.com/rust-lang/rust-clippy/pull/17605)
* [perf: skip redundant clone analysis for clone-free functions](https://github.com/rust-lang/rust-clippy/pull/17486)

#### Rust-Analyzer
* [allow “Extract variable” to be invoked on field names in record expressions](https://github.com/rust-lang/rust-analyzer/pull/23213)
* [fix HIR lowering of params of trait assoc fns](https://github.com/rust-lang/rust-analyzer/pull/23162)
* [fix incorrect generic shown on hover](https://github.com/rust-lang/rust-analyzer/pull/23256)
* [fix `NamedTempFile`](https://github.com/rust-lang/rust-analyzer/pull/23257)
* [fix some subtle bugs in docs rendering](https://github.com/rust-lang/rust-analyzer/pull/23235)
* [fix unsafeck of `&raw *`](https://github.com/rust-lang/rust-analyzer/pull/23184)
* [fix panic on accessing numeric fields in unions](https://github.com/rust-lang/rust-analyzer/pull/23229)
* [fix panic on deref of unresolved aliases](https://github.com/rust-lang/rust-analyzer/pull/23212)
* [fix panic when computing `extract_variable` with macros](https://github.com/rust-lang/rust-analyzer/pull/23238)
* [fix panic when hovering a dyn trait with a binder](https://github.com/rust-lang/rust-analyzer/pull/23250)
* [print the ABI for fn pointers (if not the default)](https://github.com/rust-lang/rust-analyzer/pull/23245)
* [push a generic params scope for consts](https://github.com/rust-lang/rust-analyzer/pull/23176)
* [range pattern inside a parenthesis parsed as tuple pattern](https://github.com/rust-lang/rust-analyzer/pull/23242)
* [reinfer never type in array repeat expressions](https://github.com/rust-lang/rust-analyzer/pull/23232)
* [strip leading asterisk decoration from block doc comments](https://github.com/rust-lang/rust-analyzer/pull/22901)
* [switch from `temp-dir` to a homemade `NamedTempFile` implementation](https://github.com/rust-lang/rust-analyzer/pull/23206)

### Rust Compiler Performance Triage

This week continues a steady stream of compile time improvements. Most of the impact this week comes from type system
micro-optimization in [#160473](https://github.com/rust-lang/rust/pull/160473) and `dead_code` lint propagation 
fix in [#161571](https://github.com/rust-lang/rust/pull/161571). We've also hit unexpected regression in a standard library 
refactor, but we expect that to be addressed soon.

Triage done by **@panstromek**.
Revision range: [9a4ad59a..5321a4f4](https://perf.rust-lang.org/?start=9a4ad59ae3073b013cd62f53f8349ddc61a012e8&end=5321a4f40c957cf3587c055e77461febc2ebc865&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.6%  | [0.2%, 1.8%]   | 27    |
| Regressions ❌ <br /> (secondary)  | 0.6%  | [0.2%, 1.8%]   | 27    |
| Improvements ✅ <br /> (primary)   | -0.7% | [-2.4%, -0.1%] | 135   |
| Improvements ✅ <br /> (secondary) | -0.7% | [-2.2%, -0.1%] | 120   |
| All ❌✅ (primary)                 | -0.5% | [-2.4%, 1.8%]  | 162   |


5 Regressions, 4 Improvements, 4 Mixed; 9 of them in rollups
39 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/e1439e38ea41334d013d9566c2d20a914e3378c7/triage/2026/2026-08-31.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Change `i686-pc-windows-msvc` from Tier 1 with host tools => Tier 1 without host tools](https://github.com/rust-lang/rfcs/pull/3999)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Re-export `core::fmt::NumBuffer` in `alloc` (and `std`)](https://github.com/rust-lang/rust/pull/161430)
* [Guarantee 8 bytes of alignment of RawWakerVTable](https://github.com/rust-lang/rust/pull/158186)
* [Stabilize `core::mem::DropGuard`](https://github.com/rust-lang/rust/pull/161520)
* ["stabilize never type" T-types FCP](https://github.com/rust-lang/rust/issues/161925)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Rustdoc LaTeX math](https://github.com/rust-lang/rfcs/pull/3958)
* [RFC: Cargo feature descriptions](https://github.com/rust-lang/rfcs/pull/3485)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Extend temporary funding team charter 2026-09](https://github.com/rust-lang/leadership-council/issues/329)
* [Update PD election process based on 2025 feedback](https://github.com/rust-lang/leadership-council/pull/286)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Errata RFC 4004: Discourage locally recursive `accessible(..)`](https://github.com/rust-lang/rfcs/pull/4004)

## Upcoming Events

Rusty Events between 2026-09-02 - 2026-09-30 🦀

### Virtual
* 2026-09-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcmbdb/)
* 2026-09-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/316107210/)
* 2026-09-04 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/sqf4ux01)
* 2026-09-06 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Facts: Curated Knowledge for Humans and Agents**](https://luma.com/9lte7a58)
* 2026-09-06 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/316133872/)
* 2026-09-08 - 2026-09-11 | Hybrid (Montreal, CA) | [RustConf 2026](https://rustconf.com/)
    * [**RustConf 2026**](https://rustconf.com/)
* 2026-09-08 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254774/)
* 2026-09-08 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315859305/)
* 2026-09-09 | Virtual (Cardiff, GB) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
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
* 2026-09-29 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: Crates, Tips & Tricks Lightning Talks - Bring your ideas!**](https://www.meetup.com/women-in-rust/events/315691730/)

### Africa
* 2026-09-08 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup)
    * [**Rust's extended standard library**](https://www.meetup.com/johannesburg-rust-meetup/events/315750593/)

### Europe
* 2026-09-03 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin on location 🏳️‍🌈 - Edition 017**](https://www.meetup.com/rust-berlin/events/316311827/)
* 2026-09-03 | Oxford, GB | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Live coding a Sinclair Spectrum from scratch**](https://www.meetup.com/oxford-rust-meetup-group/events/316310712/)
* 2026-09-08 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #87**](https://www.meetup.com/rust-paris/events/316169040/)
* 2026-10-10 | Geneva, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-09-14 - 2026-09-16 | Berlin, DE | [Oxidize 2026](https://oxidizeconf.com/)
    * [**Oxidize 2026**](https://oxidizeconf.com/)
* 2026-09-15 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Reproducing scientific papers - with Rust & "AI"**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816477/)
* 2026-09-15 | Madrid, ES | [MadRust](https://www.meetup.com/madrust/events/)
    * [**Tras la Máscara de Async Rust**](https://www.meetup.com/madrust/events/316361267/)
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

### North America
* 2026-09-03 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/316124372/)
* 2026-09-03 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Cryptography + Quantum Computers**](https://www.meetup.com/stl-rust/events/315603673/)
* 2026-09-05 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Allston-Brighton Rust Lunch, Sep 5**](https://www.meetup.com/bostonrust/events/316378793/)
* 2026-09-08 | Montreal, CA| [The Rust Foundation](https://rustfoundation.org/event/rust-teams-health-summit/)
    * [**Rust Teams Health Summit**](https://rustfoundation.org/media/rust-teams-health-summit-september-8-in-montreal/)
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
* 2026-09-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyjcmbfc/)
* 2026-09-24 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539333/)
* 2026-09-26 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Harvard Rust Lunch, Sep 26**](https://www.meetup.com/bostonrust/events/316378817/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1vtuq1b/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> I think you are inventing provenance for integers. Please, let's not.

– [Ralf Jung in an RFC discussion](https://github.com/rust-lang/rfcs/pull/4001#discussion_r3825423816)

Thanks to [kleines Filmröllchen](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1792) for the suggestion!

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1w5wisv/this_week_in_rust_667/)</small>
