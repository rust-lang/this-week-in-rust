Title: This Week in Rust 666
Number: 666
Date: 2026-08-26
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
* [Announcing our first Maintainers in Residence](https://blog.rust-lang.org/2026/08/26/announcing-our-first-maintainers-in-residence/)
* [Enabling the next-generation trait solver on nightly](https://blog.rust-lang.org/2026/08/21/enabling-next-solver-on-nightly/)
* [Supply chain attack on arrayref](https://blog.rust-lang.org/2026/08/20/supply-chain-attack-on-arrayref/)
* [Rust Function Overloading - Call for Experimentation](https://blog.rust-lang.org/inside-rust/2026/08/19/overloading-experiment/)

### Project/Tooling Updates
*  [Intent to Ship: JPEG XL – Mozilla Hacks - the Web developer blog](https://hacks.mozilla.org/2026/08/intent-to-ship-jpeg-xl/)

### Observations/Thoughts
* [Scaling Memory Safety: AI-Assisted Rewrites of C/C++ Dependencies to Rust](https://bughunters.google.com/blog/scaling-memory-safety)
* [Replacing a Rust Enum with a 64-bit Word Made My Interpreter 17% Faster](https://pointersgonewild.com/2026-08-25-replacing-a-rust-enum-with-a-64-bit-word/)
* [3 Seconds of compilation shaved by metadata analysis](https://blog.goose.love/posts/three-seconds-of-compilation-shaved-by-metadata-analysis/)
* [Your E-Paper Panel Isn't Broken: How Retained State Makes Drivers Look Buggy](https://msj.prose.sh/epaper-retained-state)
* [To Async or Not to Async: Building a Rust MCP Server for rust-analyzer](https://developerlife.com/2026/08/22/to-async-or-not-to-async-rust-mcp-server/)
* [One trie, three jobs, zero benchmarks won](https://akesson.io/wordtree/)
* [Fixing Rust's supply chain security: The good, the bad and the ugly](https://kerkour.com/fixing-rust-supply-chain-security)

### Rust Walkthroughs
* [Rust errors every beginner hits](https://dev.to/yetmike/the-22-rust-errors-every-beginner-hits-in-the-order-they-hit-them-13gi)
* [Build a Scientific Calculator in Rust - Understanding Variables and Types](https://blog.sheerluck.dev/posts/understanding-rust-variables-and-types-by-building-a-scientific-calculator/)
* [Proving SQLx’s Statement Cache with bpftrace](https://flakm.com/posts/sqlx_caches_til/)
* [Beyond WASI: Rust applications in-browser](https://labs.leaningtech.com/blog/browserpod-rust)

### Miscellaneous
* [JetBrains Partners with the Rust Foundation for an AI Livestream](https://rustfoundation.org/media/jetbrains-partners-with-the-rust-foundation-for-an-ai-livestream-series/)

## Crate of the Week

This week's crate is [swift-topomap](https://github.com/swiftlogicsystems/swifttopology), a microarchitectural observability tool.

Thanks to [Ankur Rathore](https://users.rust-lang.org/t/crate-of-the-week/2704/1658) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

##### [Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen)
* [Tracking Issue for `--remap-path-scope` in rustdoc](https://github.com/rust-lang/rust/issues/155451)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen)
* [Tracking Issue for `-Zembed-metadata`](https://github.com/rust-lang/cargo/issues/15495)

*No calls for testing were issued this week by
[Rustup](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) or
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
* [sysknife - Split the Ubuntu-only actions out of DEBIAN_ONLY_ACTIONS](https://github.com/lacs-project/sysknife/issues/237)
* [sysknife - Make Debian eligible: a version floor of 12, and a reason in is_supported](https://github.com/lacs-project/sysknife/issues/238)
* [sysknife - Debian's default firewall is nftables, and the catalogue has no nftables vocabulary](https://github.com/lacs-project/sysknife/issues/239)
<!-- or if none - *No Calls for participation were submitted this week.* -->

* [stomatopod - Add a Docker Compose healthcheck on /health](https://github.com/kkir/stomatopod/issues/40)
* [stomatopod - Add a custom GitHub social preview image](https://github.com/kkir/stomatopod/issues/41)
* [stomatopod - Document the v0.1.0 GHCR tag next to :latest](https://github.com/kkir/stomatopod/issues/42)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

593 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-08-18..2026-08-25

#### Compiler
* [add a cache to the `WfPredicates` visitor](https://github.com/rust-lang/rust/pull/161274)
* [allow self in const generics](https://github.com/rust-lang/rust/pull/157949)
* [eliminate some buggy `unreachable!()`s in `expand_`(`option_`)`env()`](https://github.com/rust-lang/rust/pull/159940)
* [enable `-Znext-solver` on nightly by default](https://github.com/rust-lang/rust/pull/160619)
* [optimize `DeepRejectCtxt`](https://github.com/rust-lang/rust/pull/161211)

#### Library
* [add `Arc/Rc::strong_count_from_raw`](https://github.com/rust-lang/rust/pull/159098)
* [add `Default` implementation for `std::sync::Once`](https://github.com/rust-lang/rust/pull/160136)
* [add symmetric PartialEq impls for `Vec`, `&[T]`, `&mut [T]` versus `Cow<'_, [T]>`](https://github.com/rust-lang/rust/pull/156160)
* [core: implement float conversion methods](https://github.com/rust-lang/rust/pull/159954)
* [make `BorrowedCursor<'a, T>` covariant in `'a` and drop an indirection](https://github.com/rust-lang/rust/pull/160563)
* [rework `div_ceil` for nonzero integers](https://github.com/rust-lang/rust/pull/160819)
* [stabilize `bool::toggle`](https://github.com/rust-lang/rust/pull/160299)
* [stabilize never type](https://github.com/rust-lang/rust/pull/155499)

#### Cargo
* [`config`: Add build.fingerprint](https://github.com/rust-lang/cargo/pull/17382)
* [fix `git gc` with `safe.bareRepository=explicit`](https://github.com/rust-lang/cargo/pull/17370)
* [install cargo tools with locked dependencies](https://github.com/rust-lang/cargo/pull/17377)

#### Rustdoc
* [add new `invalid_markdown_table` rustdoc lint](https://github.com/rust-lang/rust/pull/159583)
* [only generate search DOM elements if the search is actually needed](https://github.com/rust-lang/rust/pull/160639)
* [enable scrolling only on table/code](https://github.com/rust-lang/rust/pull/161340)
* [fix issue preventing "read more" links from generating](https://github.com/rust-lang/rust/pull/161553)

#### Rustfmt
* [fix ICE on `for await` loops with separated keyword tokens](https://github.com/rust-lang/rustfmt/pull/7063)
* [fix brace placement for multiline control flow](https://github.com/rust-lang/rustfmt/pull/7005)
* [fix comments rewritten too long](https://github.com/rust-lang/rustfmt/pull/6802)
* [correct the span used when rewriting `ast::TyKind::FnPtr`](https://github.com/rust-lang/rustfmt/pull/7066)
* [correct visibility and defaultness order on associated impl type alias](https://github.com/rust-lang/rustfmt/pull/7064)
* [inconsistent formatting of doc comments in macros](https://github.com/rust-lang/rustfmt/pull/7042)

#### Clippy
* [optimize Clippy with PGO](https://github.com/rust-lang/rust/pull/159642)
* [`unnecessary_fold`: lint folding over an Option's iterator](https://github.com/rust-lang/rust-clippy/pull/17445)
* [`unused_trait_names`: make the suggestion nicer](https://github.com/rust-lang/rust-clippy/pull/17589)
* [avoid `manual_assert_eq` for byte slice-like types](https://github.com/rust-lang/rust-clippy/pull/17575)
* [don't fire `manual_contains` when both sides use the slice element](https://github.com/rust-lang/rust-clippy/pull/17564)
* [fix `large_futures` ICE with the next solver](https://github.com/rust-lang/rust-clippy/pull/17601)
* [avoid `double_must_use` in macro-generated code](https://github.com/rust-lang/rust-clippy/pull/17547)
* [make `needless_bool` less aggressive for chained `if`s](https://github.com/rust-lang/rust-clippy/pull/17598)
* [perf: check `first_node_in_macro` before the root macro walk in `useless_format`](https://github.com/rust-lang/rust-clippy/pull/17584)
* [remove broken suggestion for `blocks_in_conditions`](https://github.com/rust-lang/rust-clippy/pull/17127)
* [suggest `hypot` for `x.mul_add(x, y * y).sqrt()`](https://github.com/rust-lang/rust-clippy/pull/17600)
* [suggest `is_ok/is_err` for boolean Result mappings](https://github.com/rust-lang/rust-clippy/pull/17537)
* [trigger `integer_division_remainder_used` on `DivAssign`/`RemAssign`](https://github.com/rust-lang/rust-clippy/pull/16493)

#### Rust-Analyzer
* [`hir`: Use expression store of parent body if available](https://github.com/rust-lang/rust-analyzer/pull/23202)
* [adds-arrow unmap ranges when fn inside macro](https://github.com/rust-lang/rust-analyzer/pull/23216)
* [allow `asm!` label blocks to diverge](https://github.com/rust-lang/rust-analyzer/pull/23186)
* [prevent stack overflow for recursive ADT layouts](https://github.com/rust-lang/rust-analyzer/pull/23201)
* [optimize the heck out of the storage of token trees](https://github.com/rust-lang/rust-analyzer/pull/23079)
* [use Cargo build directory for flycheck logs](https://github.com/rust-lang/rust-analyzer/pull/23214)

### Rust Compiler Performance Triage

A busy week, with a continued stream of improvements to the next trait solver
and next borrow check implementations. Other than those changes, the week was
pretty quiet for performance.

Triage done by **@simulacrum**.
Revision range: [8fa1c96c..9a4ad59a](https://perf.rust-lang.org/?start=8fa1c96cfd489e4c27654c144ae871ce2c4db6c6&end=9a4ad59ae3073b013cd62f53f8349ddc61a012e8&absolute=false&stat=instructions%3Au)

2 Regressions, 4 Improvements, 2 Mixed; 2 of them in rollups. 28 artifact comparisons made in total.

[Full report here](https://github.com/rust-lang/rustc-perf/blob/main/triage/2026/2026-08-23.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [make target feature ABI check a hard error on ARM](https://github.com/rust-lang/rust/pull/161280)
* [stabilize smart pointer map functions](https://github.com/rust-lang/rust/pull/160534)
* [volatile: allow accesses to non-AM memory to trap](https://github.com/rust-lang/rust/pull/160564)
* [Stabilize the `supertrait_item_shadowing` feature](https://github.com/rust-lang/rust/pull/148605)
* [Add intrinsics for integer minimum and maximum](https://github.com/rust-lang/rust/pull/161081)
* [Always escape grapheme extenders in `str::escape_debug`](https://github.com/rust-lang/rust/pull/158303)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Change `i686-pc-windows-msvc` from Tier 1 with host tools => Tier 1 without host tools](https://github.com/rust-lang/rfcs/pull/3999)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [feat(resolver): Stabilize min-publish-age](https://github.com/rust-lang/cargo/pull/17335)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Add `codeview_annotation` intrinsic](https://github.com/rust-lang/compiler-team/issues/1026)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Update PD election process based on 2025 feedback](https://github.com/rust-lang/leadership-council/pull/286)
* [Create an LLM policy team](https://github.com/rust-lang/leadership-council/issues/308)

*No Items entered Final Comment Period this week for
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Add `core::ffi::c_longdouble`](https://github.com/rust-lang/rfcs/pull/4003)
* [RFC: add `freeze` operation](https://github.com/rust-lang/rfcs/pull/4001)

## Upcoming Events

Rusty Events between 2026-08-26 - 2026-09-23 🦀

### Virtual
* 2026-08-26 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Operating Systems Book Club: Lottery and Multi-CPU Scheduling**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316083375/)
* 2026-08-27 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/313345334/)
* 2026-08-28 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/arkkrcj5)
* 2026-08-31 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Workshop: Add tests to an open source Rust project**](https://luma.com/nwfmsdtf)
* 2026-09-01 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Tauri: Cross-Platform desktop applications with Rust and web technologies**](https://luma.com/d9w26vav)
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
* 2026-09-10 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Solving Real-World Planning Problems in Rust with SolverForge**](https://luma.com/rfbzk3ae)
* 2026-09-10 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/315691423/)
* 2026-09-10 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619611/)
* 2026-09-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/fhvsztyjcmbtb/)
* 2026-09-16 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/314233757/)
* 2026-09-17 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**September, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/315635881/)
* 2026-09-18 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/ibaxicxv)
* 2026-09-20 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/316133974/)
* 2026-09-22 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday Rust Bookclub**](https://www.meetup.com/dallasrust/events/310254773/)

### Africa
* 2026-09-08 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup)
    * [**Rust's extended standard library**](https://www.meetup.com/johannesburg-rust-meetup/events/315750593/)

### Asia
* 2026-08-29 | Pune, IN | [Rust Pune](https://hasgeek.com/rustpune/)
    * [**Rust Pune Meetup: August 2026**](https://hasgeek.com/rustpune/meetup-august-2026/)

### Europe
* 2026-08-26 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #71 Sponsored by Factbird**](https://www.meetup.com/copenhagen-rust-community/events/316180984/)
* 2026-08-26 | Dresden, DE | [Rust Dresden](https://github.com/rust-dresden)
    * [**Third Meetup**](https://pretix.eu/rust-dresden/on-location-3)
* 2026-08-27 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group)
    * [**LDN Talks August Community Showcase**](https://www.meetup.com/rust-london-user-group/events/316197176/)
* 2026-08-27 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester August Talks**](https://www.meetup.com/rust-manchester/events/315891530/)
* 2026-08-29 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #29**](https://www.meetup.com/stockholm-rust/events/316130996/)
* 2026-09-08 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #87**](https://www.meetup.com/rust-paris/events/316169040/)
* 2026-09-14 - 2026-09-16 | Berlin, DE | [Oxidize 2026](https://oxidizeconf.com/)
    * [**Oxidize 2026**](https://oxidizeconf.com/)
* 2026-09-15 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Reproducing scientific papers - with Rust & "AI"**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816477/)
* 2026-09-22 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Prague @ Rockwell Automation**](https://www.meetup.com/rust-prague/events/316070376/)

### North America
* 2026-08-26 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/315171660/)
* 2026-08-26 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA August! Rust in Quantum Computing**](https://www.meetup.com/rust-los-angeles/events/315963062/)
* 2026-08-27 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539331/)
* 2026-09-03 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/316124372/)
* 2026-09-03 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Cryptography + Quantum Computers**](https://www.meetup.com/stl-rust/events/315603673/)
* 2026-09-08 - 2026-09-11 | Hybrid (Montreal, CA) | [RustConf 2026](https://rustconf.com/)
    * [**RustConf 2026**](https://rustconf.com/)
* 2026-09-09 | Montreal, CA | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**RustConf Coffee Break Meetup**](https://www.meetup.com/women-in-rust/events/315773005/)
* 2026-09-10 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Hands-on Embedded Rust**](https://www.meetup.com/utah-rust/events/316198046/)
* 2026-09-10 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust September Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/315601104/)
* 2026-09-15 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/314997217/)
* 2026-09-16 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/314233757/)
* 2026-09-17 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**September, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/315635881/)
* 2026-09-17 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/316176445/)
* 2026-09-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyjcmbfc/)

### Oceania
* 2026-08-27 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne August 2026**](https://www.meetup.com/rust-melbourne/events/315039490/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1vtuq1b/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> I care about this community, including its human and social nature. I want others to appreciate those qualities, and I don't want to see them compromised and replaced by excessive machine-generated content.

– [Quine Dot on rust-users](https://users.rust-lang.org/t/use-of-ai-assitance-to-solve-issues-and-validate-to-reply/142029/11)

Thanks to [Jonas Fassbender](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1791) for the suggestion!

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1vzh4xx/this_week_in_rust_666/)</small>
