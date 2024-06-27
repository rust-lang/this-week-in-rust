Title: This Week in Rust 552
Number: 552
Date: 2024-06-19
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://x.com/ThisWeekInRust) on X(formerly Twitter) or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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
* [Announcing Rust 1.79.0](https://blog.rust-lang.org/2024/06/13/Rust-1.79.0.html)
* [This Development-cycle in Cargo: 1.80](https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html)

### Foundation
* [Announcing the Safety-Critical Rust Consortium](https://foundation.rust-lang.org/news/announcing-the-safety-critical-rust-consortium/)

### Newsletters
* [This Week in Bevy: Meshlets, Stable Interpolation, and Generalized ECS Reactivity with Observers!](https://thisweekinbevy.com/issue/2024-06-17-meshlets-stable-interpolation-and-generalized-ecs-reactivity-with-observers)

### Project/Tooling Updates
* [rust-analyzer changelog #238](https://rust-analyzer.github.io/thisweek/2024/06/17/changelog-238.html)
* [Enhancing Code Completion for Rust in Cody](https://sourcegraph.com/blog/enhancing-code-completion-for-rust-in-cody)
* [Apache Datafusion-Comet- Rust-Based Apache Spark accelerator - 1.5x performance improvements in apache spark TPC-H queries, and other commodity hardware](https://github.com/apache/datafusion-comet)
* [Fluvio Distributed Streaming Engine Release 0.11.9](https://www.fluvio.io/news/this-week-in-fluvio-0062/)

### Observations/Thoughts
* [Path Generics in Rust: A Sketch Proposal for Simplicity and Generality](https://cfallin.org/blog/2024/06/12/rust-path-generics/)
* [Dioxus Labs + “High-level Rust”](https://dioxus.notion.site/Dioxus-Labs-High-level-Rust-5fe1f1c9c8334815ad488410d948f05e)
* [Making Your First Real-World Rust Project a Success](https://corrode.dev/blog/successful-rust-business-adoption-checklist/)
* [Future's liveness problem](https://skepfyr.me/blog/futures-liveness-problem/)
* [Optimizing Rust code with Flamegraph and DHAT – a practical example with Dust DDS](https://www.s2e-systems.com/2024/06/13/optimizing_rust_code/)
* [Exograph at the Edge with Cloudflare Workers](https://exograph.dev/blog/cloudflare-workers)
* [GraphQL Server in the Browser using WebAssembly](https://exograph.dev/blog/playground)
* [video] [Compiler-Driven Development in Rust](https://www.youtube.com/watch?v=Kdpfhj3VM04)
* [video] [Rust 1.79.0: Top 10 Most Interesting Things](https://youtu.be/u5WD5Ta09vs)
* [audio] [What's New in Rust 1.72 and 1.73](https://rustacean-station.org/episode/rust-1.72-1.73/)
* [audio] [Matic with Eric Seppanen](https://corrode.dev/podcast/s02e04-matic/)

### Rust Walkthroughs
* [Rust Ownership Explained: Merging Linked Lists](https://www.howtocodeit.com/articles/rust-ownership-explained-linked-lists)
* [Zero to Performance Hero: How to Benchmark and Profile Your eBPF Code in Rust](https://www.infoq.com/articles/benchmark-profile-ebpf-code/)
* [IPC in Rust - a Ping Pong Comparison](https://3tilley.github.io/posts/simple-ipc-ping-pong/)
* [I ported h2spec to Rust (also: codegen!)](https://fasterthanli.me/videos/h2spec-to-rust)
* [Parsing Python ASTs 20x Faster with Rust](https://www.gauge.sh/blog/parsing-python-asts-20x-faster-with-rust)
* [Making a const version of Rust's array::from_fn - How hard can it be?](https://gendignoux.com/blog/2024/06/17/const-array-from-fn.html)
* [Using Tauri to build a cross-platform security app](https://www.firezone.dev/blog/using-tauri)
* [Generate and package Rust client SDKs with Buf](https://buf.build/blog/bsr-generated-sdks-for-rust)
* [series] [Master Hexagonal Architecture in Rust (part 2): Separation of Concerns, Rust-Style](https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust#separation-of-concerns-rust-style)

### Research
* [SquirrelFS: using the Rust compiler to check file-system crash consistency](https://arxiv.org/abs/2406.09649)
* [Trusting code in the wild: Exploring contributor reputation measures to review dependencies in the Rust ecosystem](https://arxiv.org/abs/2406.10317)

### Miscellaneous
* [Rust social status update 2024.06](https://rust.code-maven.com/rust-update-2024-06-17)
* [Learning Material for Idiomatic Rust](https://corrode.dev/blog/idiomatic-rust-resources/)

## Crate of the Week

This week's crate is [yazi](https://yazi-rs.github.io), a blazing fast terminal file manager based on async I/O.

Despite a lamentable lack of suggestions, llogiq is content with his choice.

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
* [Dusk - Archival Nodes APIs Infrastructure RFP](https://docs.dusk.network/grants/rfps/archival_node)
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

* [Rust Ukraine 2024](https://docs.google.com/forms/d/e/1FAIpQLSc9S_95oaCsFyrULF4iBQOIiTcMlOpG07izgquYLBCKFAYTKQ/viewform) | Closes 2024-07-06 | Online + Ukraine, Kyiv | Event date: 2024-07-27
* [Conf42 Rustlang 2024](https://www.papercall.io/conf42-rustlang-2024) | Closes 2024-07-22 | online | Event date: 2024-08-22

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

470 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-06-11..2024-06-18

* [promote `arm64ec-pc-windows-msvc` to tier 2](https://github.com/rust-lang/rust/pull/126039)
* [edition 2024: Make `!` fall back to `!`](https://github.com/rust-lang/rust/pull/123508)
* [`E0229`: Suggest Moving Type Constraints to Type Parameter Declaration](https://github.com/rust-lang/rust/pull/126054)
* [`UniqueRc`: support allocators and `T: ?Sized`](https://github.com/rust-lang/rust/pull/126285)
* [account for existing bindings when suggesting `pin!()`](https://github.com/rust-lang/rust/pull/125684)
* [add a new concat metavar expr](https://github.com/rust-lang/rust/pull/118958)
* [avoid ICEs after reporting errors on erroneous patterns](https://github.com/rust-lang/rust/pull/126320)
* [build `libcxx-version` only when it doesn't exist](https://github.com/rust-lang/rust/pull/126472)
* [change method resolution to constrain hidden types instead of rejecting method candidates](https://github.com/rust-lang/rust/pull/123962)
* [check that alias-relate terms are WF if reporting an error in alias-relate](https://github.com/rust-lang/rust/pull/126404)
* [consistently use subtyping in method resolution](https://github.com/rust-lang/rust/pull/126128)
* [const validation: fix ICE on dangling ZST reference](https://github.com/rust-lang/rust/pull/126426)
* [const-eval: make lint scope computation consistent](https://github.com/rust-lang/rust/pull/126388)
* [coverage: arrange span extraction/refinement as a series of passes](https://github.com/rust-lang/rust/pull/126535)
* [coverage: replace the old span refiner with a single function](https://github.com/rust-lang/rust/pull/126294)
* [delegation: fix ICE on late diagnostics](https://github.com/rust-lang/rust/pull/126234)
* [delegation: fix ICE on recursive delegation](https://github.com/rust-lang/rust/pull/126236)
* [delegation: fix hygiene for `self`](https://github.com/rust-lang/rust/pull/126497)
* [detect pub structs never constructed even though they impl pub trait with assoc constants](https://github.com/rust-lang/rust/pull/126276)
* [do not ICE in privacy when type inference fails](https://github.com/rust-lang/rust/pull/126584)
* [do not define opaque types when selecting impls](https://github.com/rust-lang/rust/pull/126258)
* [ensure self-contained linker is only enabled on dev/nightly](https://github.com/rust-lang/rust/pull/126282)
* [expand list of trait implementers in E0277 when calling rustc with --verbose](https://github.com/rust-lang/rust/pull/126055)
* [for E0277 suggest adding `Result` return type for function when using QuestionMark `?` in the body](https://github.com/rust-lang/rust/pull/126187)
* [harmonize using root or leaf obligation in trait error reporting](https://github.com/rust-lang/rust/pull/126142)
* [honor `collapse_debuginfo` for statics](https://github.com/rust-lang/rust/pull/126365)
* [implement lint for obligations broken by never type fallback change](https://github.com/rust-lang/rust/pull/125289)
* [improve escaping of byte, byte str, and c str proc-macro literals](https://github.com/rust-lang/rust/pull/123769)
* [interpret: dyn trait metadata check: equate traits in a proper way](https://github.com/rust-lang/rust/pull/126232)
* [interpret: ensure we check bool/char for validity when they are used in a cast](https://github.com/rust-lang/rust/pull/126265)
* [make `ObligationEmittingRelation`s emit `Goal` rather than `Obligation`](https://github.com/rust-lang/rust/pull/126130)
* [make `storage-live.rs` robust against rustc internal changes](https://github.com/rust-lang/rust/pull/126286)
* [make suggestion to change `Fn` to `FnMut` work with methods as well](https://github.com/rust-lang/rust/pull/126226)
* [mark undetermined if target binding in current ns is not got](https://github.com/rust-lang/rust/pull/126568)
* [move `MatchAgainstFreshVars` to old solver](https://github.com/rust-lang/rust/pull/126353)
* [no uninitalized report in a pre-returned match arm](https://github.com/rust-lang/rust/pull/126295)
* [only compute `specializes` query if (min)specialization is enabled in the crate of the specializing impl](https://github.com/rust-lang/rust/pull/126139)
* [only compute vtable information during codegen](https://github.com/rust-lang/rust/pull/126505)
* [place explicit lifetime bound after generic param](https://github.com/rust-lang/rust/pull/124884)
* [point out failing never obligation for `DEPENDENCY_ON_UNIT_NEVER_TYPE_FALLBACK`](https://github.com/rust-lang/rust/pull/126367)
* [print `token::Interpolated` with token stream pretty printing](https://github.com/rust-lang/rust/pull/125174)
* [provide correct parent for nested anon const](https://github.com/rust-lang/rust/pull/126228)
* [resolve elided lifetimes in assoc const to static if no other lifetimes are in scope](https://github.com/rust-lang/rust/pull/125258)
* [safe transmute: support `Single` enums](https://github.com/rust-lang/rust/pull/126358)
* [walk into alias-eq nested goals even if normalization fails](https://github.com/rust-lang/rust/pull/125688)
* [simplify provider api to improve llvm ir](https://github.com/rust-lang/rust/pull/126242)
* [smir: merge identical Constant and ConstOperand types](https://github.com/rust-lang/rust/pull/126410)
* [spell out other trait diagnostic](https://github.com/rust-lang/rust/pull/126127)
* [spruce up the diagnostics of some early lints](https://github.com/rust-lang/rust/pull/125913)
* [tait must be constrained if in sig](https://github.com/rust-lang/rust/pull/113169)
* [unify guarantees about the default allocator](https://github.com/rust-lang/rust/pull/126266)
* [unify intrinsics body handling in StableMIR](https://github.com/rust-lang/rust/pull/126361)
* [use `Variance` glob imported variants everywhere](https://github.com/rust-lang/rust/pull/126354)
* [use a consistent way to filter out bounds instead of splitting it into three places](https://github.com/rust-lang/rust/pull/126471)
* [MIR Shl/Shr: the offset can be computed with `rem_euclid`](https://github.com/rust-lang/rust/pull/126469)
* [miri: cargo miri: add support for '--many-seeds'](https://github.com/rust-lang/miri/pull/3672)
* [miri: implement LLVM x86 SSE4.2 intrinsics](https://github.com/rust-lang/miri/pull/3622)
* [miri: show proper UB when making a too large allocation request](https://github.com/rust-lang/miri/pull/3682)
* [miri: tell people how to set miri flags](https://github.com/rust-lang/miri/pull/3683)
* [`rustc_span`: Optimize more hygiene operations using `Span::map_ctxt`](https://github.com/rust-lang/rust/pull/126543)
* [split core's PanicInfo and std's PanicInfo](https://github.com/rust-lang/rust/pull/115974)
* [remove superfluous UbChecks from `SliceIndex` methods](https://github.com/rust-lang/rust/pull/126299)
* [`std::unix::fs::link` using direct linkat call for Solaris](https://github.com/rust-lang/rust/pull/126351)
* [simplify `[T; N]::try_map` signature](https://github.com/rust-lang/rust/pull/126249)
* [follow up to splitting core's PanicInfo and std's PanicInfo](https://github.com/rust-lang/rust/pull/126322)
* [make `PathBuf` less Ok with adding UTF-16 then `into_string`](https://github.com/rust-lang/rust/pull/126305)
* [add `Option::is_none_or`](https://github.com/rust-lang/rust/pull/126328)
* [add `f16` and `f128` const eval for binary and unary operationations](https://github.com/rust-lang/rust/pull/126429)
* [add `f16` and `f128` inline ASM support for `x86` and `x86-64`](https://github.com/rust-lang/rust/pull/126417)
* [make `ptr::rotate` smaller when using `optimize_for_size`](https://github.com/rust-lang/rust/pull/125720)
* [hashbrown: improve Set Difference `size_hint` lower bound](https://github.com/rust-lang/hashbrown/pull/530)
* [hashbrown: make equivalent default feature](https://github.com/rust-lang/hashbrown/pull/532)
* [hashbrown: optimize Set `is_disjoint`](https://github.com/rust-lang/hashbrown/pull/531)
* [cargo fix: Address problems with implicit → explicit feature migration](https://github.com/rust-lang/cargo/pull/14018)
* [cargo: add assert redactions](https://github.com/rust-lang/cargo/pull/14054)
* [cargo: add local registry overlays](https://github.com/rust-lang/cargo/pull/13926)
* [cargo: change verification order during packaging](https://github.com/rust-lang/cargo/pull/14074)
* [cargo test: redact conditional compile-fail warning](https://github.com/rust-lang/cargo/pull/14064)
* [cargo: use `std::fs::absolute` instead of reimplementing it](https://github.com/rust-lang/cargo/pull/14075)
* [clippy: add MSRV for `manual_pattern_char_comparison`](https://github.com/rust-lang/rust-clippy/pull/12937)
* [clippy: add `field_scoped_visibility_modifiers` lint](https://github.com/rust-lang/rust-clippy/pull/12893)
* [clippy: add lint `manual_inspect`](https://github.com/rust-lang/rust-clippy/pull/12287)
* [clippy: add lint to check manual pattern char comparison](https://github.com/rust-lang/rust-clippy/pull/12849)
* [clippy: avoid emitting `assigning_clones` when cloned data borrows from the place to clone into](https://github.com/rust-lang/rust-clippy/pull/12756)
* [clippy: don't lint `indexing_slicing` lints on proc macros](https://github.com/rust-lang/rust-clippy/pull/12912)
* [clippy: fix ICE in `upper_case_acronyms`](https://github.com/rust-lang/rust-clippy/pull/12903)
* [clippy: handle single chars with `to_string()` for `single_char_add_str`](https://github.com/rust-lang/rust-clippy/pull/12915)
* [clippy: let `qualify_min_const_fn` deal with drop terminators](https://github.com/rust-lang/rust-clippy/pull/12681)
* [clippy: lint `manual_unwrap_or` for it let cases](https://github.com/rust-lang/rust-clippy/pull/12906)
* [clippy: normalize type aliases when checking significant drops](https://github.com/rust-lang/rust-clippy/pull/12904)
* [clippy: rework `octal_escapes`](https://github.com/rust-lang/rust-clippy/pull/12945)
* [rust-analyzer: allow choosing logical cores for num threads config](https://github.com/rust-lang/rust-analyzer/pull/17374)
* [rust-analyzer: allow rust-project.json to include arbitrary shell commands for runnables](https://github.com/rust-lang/rust-analyzer/pull/16840)
* [rust-analyzer: show type bounds from containers when hovering on functions](https://github.com/rust-lang/rust-analyzer/pull/17364)
* [rust-analyzer: fix and cleanup VSCode task building](https://github.com/rust-lang/rust-analyzer/pull/17440)
* [rust-analyzer: add a breaker to avoid infinite loops from source root cycles](https://github.com/rust-lang/rust-analyzer/pull/17412)
* [rust-analyzer: avoid doubling cargo args in runnables](https://github.com/rust-lang/rust-analyzer/pull/17407)
* [rust-analyzer: fix `HirDisplay` stackoverflow for parameter Self defaults](https://github.com/rust-lang/rust-analyzer/pull/17394)
* [rust-analyzer: fix pat fragment parsers choking on `<eoi>`](https://github.com/rust-lang/rust-analyzer/pull/17442)
* [rust-analyzer: properly prime all crate def maps in `parallel_prime_caches`](https://github.com/rust-lang/rust-analyzer/pull/17439)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:
* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [UnsafePinned: allow aliasing of pinned mutable references](https://github.com/rust-lang/rfcs/pull/3467)
* [disposition: postpone] [RFC: make Cargo embed dependency versions in the compiled binary](https://github.com/rust-lang/rfcs/pull/2801)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [RFC: Return Type Notation](https://github.com/rust-lang/rfcs/pull/3654)
* [disposition: merge] [RFC: Unblock Cargo feature metadata](https://github.com/rust-lang/rfcs/pull/3416)
* [disposition: postpone] [Allow requiring "at least one feature"](https://github.com/rust-lang/rfcs/pull/3347)
* [disposition: postpone] [Add a general mechanism of setting RUSTFLAGS in Cargo for the root crate only](https://github.com/rust-lang/rfcs/pull/3310)
* [disposition: close] [Allow specifying dependencies for individual artifacts](https://github.com/rust-lang/rfcs/pull/2887)
* [disposition: postpone] [RFC: make Cargo embed dependency versions in the compiled binary](https://github.com/rust-lang/rfcs/pull/2801)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team RFCs entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [repr(type) for type aliases](https://github.com/rust-lang/rfcs/pull/3659)
* [new] [Proposed 2024h2 flagship goal: Rust for Linux](https://github.com/rust-lang/rfcs/pull/3658)
* [new] [Async project goal](https://github.com/rust-lang/rfcs/pull/3657)

## Upcoming Events

Rusty Events between 2024-06-19 - 2024-07-17 🦀

### Virtual
* 2024-06-19 | Hybrid - Virtual and In-person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631733/)
* 2024-06-20 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477705/)
* 2024-06-25 | Virtual (Dallas, TX, US) | [Dallas Rust User Group](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtygcjbhc/)
* 2024-06-25 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Using the Liquid template system in Rust (English)**](https://www.meetup.com/code-mavens/events/301487547/)
* 2024-06-27 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897826/)
* 2024-07-02 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300191673/)
* 2024-07-02 | Hybrid - Virtual and In-person (Los Angeles, CA, US) | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles/)    
    * [**Rust LA Reboot**](https://www.meetup.com/rust-los-angeles/events/301645611/)
* 2024-07-03 | Virtual | [Training 4 Programmers LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Build Web Apps with Rust and Leptos**](https://www.eventbrite.com/e/build-web-apps-with-rust-and-leptos-tickets-904804503627?aff=odcleoeventsincollection)
* 2024-07-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/300328025/)
* 2024-07-04 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298488820/)
* 2024-07-06 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2024-07-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346976/)
* 2024-07-10 | Virtual | [Centre for eResearch](https://www.eventbrite.co.nz/o/centre-for-eresearch-75893560993)
    * [**Research Computing With The Rust Programming Language**](https://www.eventbrite.com/e/research-computing-with-the-rust-programming-language-tickets-908002037537?aff=ebdssbdestsearch&keep_tld=1)
* 2024-07-11 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897842/)
* 2024-07-11 | Hybrid - Virtual and In-person (Mexico City, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Programación de sistemas con Rust**](https://www.meetup.com/rust-mx/events/301740677/)
* 2024-07-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/298076822/)
* 2024-07-11 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Reading JSON files in Rust (English)**](https://www.meetup.com/code-mavens/events/301636580/)
* 2024-07-16 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Web development in Rust using Rocket - part 2 (English)**](https://www.meetup.com/code-mavens/events/301736709/)
* 2024-07-17 | Hybrid - Virtual and In-person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631734/)

### Asia
* 2024-06-22 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**June 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/june-2024-rustacean-meetup/)
* 2024-06-22 | Delhi, IN | [PyDelhi - Python Delhi User Group!](https://www.meetup.com/pydelhi/)
    * [**PyRustLin: PyDelhi x Rust Delhi x ILUG-D Meetup**](https://www.meetup.com/pydelhi/events/301520830/)
* 2024-06-23 | Tel Aviv, Israel | [Rust in Israel](https://www.meetup.com/rust-in-israel/)
    * [**Rust at Microsoft Tel Aviv in June 2024**](https://www.meetup.com/rust-in-israel/events/301670916/)
* 2024-06-30 | Kyoto, JP | [Kyoto Rust](https://www.meetup.com/kyoto-rust/)
    * [**Rust Talk: Cross Platform Apps**](https://www.meetup.com/kyoto-rust/events/301499550/)

### Europe
* 2024-06-19 - 2024-06-24 | Zürich, CH | [RustFest Zürich](https://rustfest.ch/)
    * [**RustFest Zürich 2024**](https://rustfest.ch/)
* 2024-06-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at Trifork**](https://www.meetup.com/rust-aarhus/events/300865116/)
* 2024-06-25 | Gdańsk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/)
    * [**Rust Gdansk Meetup #3**](https://www.meetup.com/rust-gdansk/events/301014697/)
* 2024-06-27 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299288965/)
* 2024-06-27 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #48 sponsored by Google!**](https://www.meetup.com/copenhagen-rust-community/events/300458252/)
* 2024-07-10 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup - July**](https://www.meetup.com/reading-rust-workshop/events/301359031/)
* 2024-07-11 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Prague (July 2024)**](https://www.meetup.com/rust-prague/events/301227195)
* 2024-07-16 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Building a REST API in Rust using Axum, SQLx and SQLite**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/301716171/)
* 2024-07-16 | Mannheim, DE | [Hackschool - Rhein-Neckar](https://www.meetup.com/hackschool-rhein-neckar)
    * [**Nix Your Bugs & Rust Your Engines #4**](https://www.meetup.com/hackschool-rhein-neckar/events/301504325/)

### North America
* 2024-06-19 | Hybrid - Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631733/)
* 2024-06-20 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/299509396/)
* 2024-06-22 | Los Angeles, CA, US | [LA Bitcoin Devs](https://www.meetup.com/bitdevsla/)
    * [**Shaan Batra on Learning Rust the Bitcoin Way and Socratic Seminar #46**](https://www.meetup.com/bitdevsla/events/301011049/)
* 2024-06-24 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Harvard Square Rust Lunch, June 24**](https://www.meetup.com/bostonrust/events/301549722/)
* 2024-06-26 | Austin, TX, US | [Rust ATC](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/301066942/)
* 2024-06-27 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/301613483/)
* 2024-06-27 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers: Holding Pattern**](https://www.meetup.com/music-city-rust-developers/events/301411746/)
* 2024-06-27 | St. Louis, MO, US | [STl Rust](https://www.meetup.com/stl-rust/)
    * [**Meet and Greet Hacker session**](https://www.meetup.com/stl-rust/events/301321974/)
* 2024-07-02 | Hybrid - Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles/)    
    * [**Rust LA Reboot**](https://www.meetup.com/rust-los-angeles/events/301645611/)
* 2024-07-05 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston University Rust Lunch, July 5**](https://www.meetup.com/bostonrust/events/301549737/)
* 2024-07-11 | Hybrid - Mexico City, DF, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Programación de sistemas con Rust**](https://www.meetup.com/rust-mx/events/301740677/)
* 2024-07-11 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/301613495/)
* 2024-07-17 | Hybrid - Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631734/)

### Oceania
* 2024-06-20 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Full Stack Rust + Writing a compiler for fun and (no) profit**](https://www.meetup.com/rust-akl/events/301193761/)
* 2024-06-25 | Canberra, ACt, AU | [Canberra Rust User Group (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/300749371/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1cixuzr/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> If there’s a backdoor attack lurking in the crates ecosystem, then it’s lurking pretty deep at present. The popular crates that we all rely on day to day generally appear to be what they say they are.

– [Adam Harvey on his blog](https://lawngno.me/blog/2024/06/10/divine-provenance.html)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1575) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1dk69nb/this_week_in_rust_552/)</small>
