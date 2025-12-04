Title: This Week in Rust 628
Number: 628
Date: 2025-12-03
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
* [Fall 2025 Project Director Update](https://blog.rust-lang.org/inside-rust/2025/12/02/project-director-update/)

### Foundation
* [Rustls Shortlisted for Two 2025 OpenUK Awards](https://rustfoundation.org/media/rustls-shortlisted-for-two-2025-openuk-awards/)

### Newsletters
* [Rust Trends Issue #73: From Lab to Factory: Rust in Production](https://rust-trends.com/newsletter/from-lab-to-factory-rust-in-production/)

### Project/Tooling Updates
* [Wasmi 1.0 — WebAssembly Interpreter Stable At Last](https://wasmi-labs.github.io/blog/posts/wasmi-v1.0/)
* [hyper-util Composable Pools](https://seanmonstar.com/blog/hyper-util-composable-pools/)
* [Fall Updates: Standard Library Support with vexide 0.8.0!](https://vexide.dev/blog/posts/thanksgiving-update-25/)
* [3DCF/doc2dataset v0.1.0 – Rust document-to-dataset pipeline for RAG & LLM fine-tuning](https://github.com/3DCF-Labs/doc2dataset/releases/tag/v0.1.0)
* [PGM-Extra: High-performance learned index structures for Rust](https://github.com/itsfoxstudio/pgm-extra-rs/releases/tag/v1.2.2)

### Observations/Thoughts
* [In defense of lock poisoning in Rust](https://sunshowers.io/posts/on-poisoning/)
* [How CRDTs and Rust are revolutionizing distributed systems and real-time applications](https://kerkour.com/rust-crdt)
* [KCL part 1: units](https://www.ncameron.org/blog/kcl-part-1-units/)
* [New rust lint: function_casts_as_integer](https://blog.guillaume-gomez.fr/articles/2025-11-28+New+rust+lint%3A+function_casts_as_integer)
* [audio] [Netstack.FM episode 16 — WebRTC and Sans IO with Martin Algesten](https://netstack.fm/#episode-16)
* [audio] [Canonical with Jon Seager - Rust in Production Podcast](https://corrode.dev/podcast/s05e05-canonical/)

### Rust Walkthroughs
* [The Impatient Programmer's Guide to Bevy and Rust: Chapter 3 - Let The Data Flow](https://aibodh.com/posts/bevy-rust-game-development-chapter-3/)
* [Cross-Compiling Rust for Raspberry Pi and making CI](https://sysdev.me/2025/11/27/cross-compiling-rust-for-raspberry-pi/)
* [Rootless pings in Rust](https://bou.ke/blog/rust-ping/)
* [Mutation testing for librsvg](https://viruta.org/mutation-testing-librsvg.html)
* [video] [impl Rust: One Billion Row Challenge](https://www.youtube.com/watch?v=tCY7p6dVAGE)

### Research

### Miscellaneous
* [The Rust Africa Hackathon 2026](https://rustafrica.org/the-future-is-written-in-rust-rust-africa-hackathon-2026/)
* [Ferrous Systems Achieves IEC 61508 (SIL 2) Certification for Rust Core Library Subset](https://ferrous-systems.com/blog/ferrocene-libcore-news-release/)

## Crate of the Week

This week's crate is [corosensei](https://github.com/Amanieu/corosensei), a crate that allows you to write stackful coroutines on stable Rust.

Thanks to [Christiaan](https://users.rust-lang.org/t/crate-of-the-week/2704/1497) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

* *No calls for testing were issued this week by
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects
  
Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
* [**hyper - User Survey 2025**](https://www.surveyhero.com/c/vvnhc7j7)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [**Rustikon 2026**](https://sessionize.com/rustikon-2026/) | CFP closes 2025-11-24 | Warsaw, Poland | 2025-03-19 - 2025-03-20 | [Event Website](https://www.rustikon.dev/)
* [**TokioConf 2026**](https://tokio.rs/blog/2025-09-26-announcing-tokio-conf-cfp) | CFP closes 2025-12-08 | Portland, Oregon, USA | 2026-04-20
* [**RustWeek 2026**](https://sessionize.com/rustweek-2026/) | CFP closes 2025-12-31 | Utrecht, The Netherlands | 2026-05-19 - 2026-05-20

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

509 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-11-25..2025-12-02

#### Compiler
* [add `Box::clone_from_ref` and similar under `feature(clone_from_ref)`](https://github.com/rust-lang/rust/pull/149079)
* [add `Command::get_env_clear`](https://github.com/rust-lang/rust/pull/149074)
* [add a diagnostic attribute for special casing const bound errors for non-const impls](https://github.com/rust-lang/rust/pull/148641)
* [collapse `constness` query `match` logic](https://github.com/rust-lang/rust/pull/149444)

#### Library
* [add `impl TrustedLen` on `BTree{Map,Set}` iterators](https://github.com/rust-lang/rust/pull/149381)
* [constify `from_fn, try_from_fn, try_map,` map](https://github.com/rust-lang/rust/pull/147071)
* [implement `Iterator::{exactly_one, collect_array}`](https://github.com/rust-lang/rust/pull/149270)
* [implement `clamp_magnitude` method for primitive floats & signed integers](https://github.com/rust-lang/rust/pull/148690)
* [in `BTreeMap::eq`, do not compare the elements if the sizes are different](https://github.com/rust-lang/rust/pull/149125)
* [num: implement `uint_gather_scatter_bits` feature for unsigned integers](https://github.com/rust-lang/rust/pull/149097)
* [offload intrinsic](https://github.com/rust-lang/rust/pull/147936)
* [optimize `slice::Iter::next_chunk`](https://github.com/rust-lang/rust/pull/149131)
* [stabilize `asm_cfg`](https://github.com/rust-lang/rust/pull/147736)
* [stabilize `maybe_uninit_slice`](https://github.com/rust-lang/rust/pull/149102)
* [stabilize `maybe_uninit_write_slice`](https://github.com/rust-lang/rust/pull/148048)
* [stabilize `unchecked_neg` and `unchecked_shifts`](https://github.com/rust-lang/rust/pull/149087)

#### Cargo
* [`clean`: Clean hosts builds with new layout](https://github.com/rust-lang/cargo/pull/16300)
* [`completion`: Put host-tuple before actual tuples](https://github.com/rust-lang/cargo/pull/16327)
* [`completions`: include `all` in `cargo tree --target` candidates](https://github.com/rust-lang/cargo/pull/16322)
* [`config-include`: remove support of single string shorthand](https://github.com/rust-lang/cargo/pull/16298)
* [`lints`: show lint error number](https://github.com/rust-lang/cargo/pull/16320)
* [`clean`: add --workspace support](https://github.com/rust-lang/cargo/pull/16263)
* [do not lock the artifact-dir for check builds + fix uplifting](https://github.com/rust-lang/cargo/pull/16307)
* [properly validate crate names in `cargo install`](https://github.com/rust-lang/cargo/pull/16314)

#### Rustdoc
* [fix bad intra-doc-link preprocessing](https://github.com/rust-lang/rust/pull/148169)
* [fix invalid link generation for type alias methods](https://github.com/rust-lang/rust/pull/149274)
* [fix rustdoc search says “Consider searching for "null" instead.” #149324](https://github.com/rust-lang/rust/pull/149332)

#### Clippy
* [`manual_ilog2`: new lint](https://github.com/rust-lang/rust-clippy/pull/15865)
* [`equatable_if_let`: don't lint if pattern or initializer come from expansion](https://github.com/rust-lang/rust-clippy/pull/15958)
* [add `ptr_offset_by_literal` lint](https://github.com/rust-lang/rust-clippy/pull/15606)
* [clippy lints page improvements and cleanups](https://github.com/rust-lang/rust-clippy/pull/16112)
* [fix `implicit_hasher` wrongly unmangled macros](https://github.com/rust-lang/rust-clippy/pull/16129)
* [fix `large_stack_frames` false positive on compiler generated targets](https://github.com/rust-lang/rust-clippy/pull/15101)
* [fix display of dropdown menu "buttons"](https://github.com/rust-lang/rust-clippy/pull/16151)
* [fix: `zero_repeat_side_effects` misses curlies](https://github.com/rust-lang/rust-clippy/pull/15853)
* [new lint: `decimal_bitwise_operands`](https://github.com/rust-lang/rust-clippy/pull/15215)
* [stop inserting redundant parenthesis around desugared match expressions](https://github.com/rust-lang/rust-clippy/pull/16102)

#### Rust-Analyzer
* [add multiple generate for `enum` generate is, as, `try_into`](https://github.com/rust-lang/rust-analyzer/pull/20685)
* [build releases with static CRT for `-windows-msvc` targets](https://github.com/rust-lang/rust-analyzer/pull/21027)
* [completions: fix completions disregarding snippet capabilities](https://github.com/rust-lang/rust-analyzer/pull/21131)
* [feature: set `enclosing_range` field on SCIP output](https://github.com/rust-lang/rust-analyzer/pull/21141)
* [fix Display scope inlay hints after closing brace for more types of blocks #18833](https://github.com/rust-lang/rust-analyzer/pull/21077)
* [fix `syntax_editor` duplicated changed element](https://github.com/rust-lang/rust-analyzer/pull/21023)
* [fix complete after `extern`, add `crate` completion](https://github.com/rust-lang/rust-analyzer/pull/21144)
* [fix not complete after inner-attr in source-file](https://github.com/rust-lang/rust-analyzer/pull/20976)
* [fix not complete type alias in pattern](https://github.com/rust-lang/rust-analyzer/pull/21028)
* [fix skipiter not applicable in autoderef](https://github.com/rust-lang/rust-analyzer/pull/21095)
* [do not try to connect via postcard to proc-macro-srv](https://github.com/rust-lang/rust-analyzer/pull/21133)
* [don't run cache priming when disabled in settings](https://github.com/rust-lang/rust-analyzer/pull/21151)
* [fix proc-macro-srv passing invalid extra none group to proc-macros](https://github.com/rust-lang/rust-analyzer/pull/21190)
* [fix proc-macro-srv protocol read implementation](https://github.com/rust-lang/rust-analyzer/pull/21135)
* [pass the correct per-token (not global) edition when expanding `macro_rules`](https://github.com/rust-lang/rust-analyzer/pull/20164)
* [rewrite dyn trait lowering to follow rustc](https://github.com/rust-lang/rust-analyzer/pull/21159)
* [support multiple `enable` in `#[target_feature]`](https://github.com/rust-lang/rust-analyzer/pull/21170)
* [use per-token, not global, edition in the parser](https://github.com/rust-lang/rust-analyzer/pull/20163)
* [use root hygiene for speculative resolution](https://github.com/rust-lang/rust-analyzer/pull/20217)
* [perf: use one query per crate for lang items, not one per lang item](https://github.com/rust-lang/rust-analyzer/pull/21149)
* [proc-macro-srv: fix `<TokenStream as Display>::fmt` impl producing trailing whitespace](https://github.com/rust-lang/rust-analyzer/pull/21145)
* [proc-macro-srv: fix `<TokenStream as Display>::fmt` impl rendering puncts as u8](https://github.com/rust-lang/rust-analyzer/pull/21146)
* [proc-macro-srv: fix unnecessary subtree wrapping in protocol](https://github.com/rust-lang/rust-analyzer/pull/21154)
* [re-introduce attribute rewrite](https://github.com/rust-lang/rust-analyzer/pull/20892)

### Rust Compiler Performance Triage

A fairly quiet week overall, despite a slightly higher than usual amount of merged PRs.

Triage done by **@simulacrum**.
Revision range: [b64df9d1..eca9d93f](https://perf.rust-lang.org/?start=b64df9d1012f2482b54a4d959548cf8fc67e820c&end=eca9d93f9057f9a48ff691bd65e7daf2f94c1b67&absolute=false&stat=instructions%3Au)

3 Regressions, 1 Improvement, 4 Mixed; 3 of them in rollups
43 artifact comparisons made in total

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025/2025-12-02.md] for details.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs entered Final Comment Period this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [don't normalize where-clauses when checking well-formedness](https://github.com/rust-lang/rust/pull/148477)
* [Stabilize `const_mul_add`](https://github.com/rust-lang/rust/pull/148052)
* [Do not propogate unnecessary closure constraints.](https://github.com/rust-lang/rust/pull/148329)
* [Not linting `irrefutable_let_patterns` on let chains](https://github.com/rust-lang/rust/pull/146832)
* [Make closure capturing have consistent and correct behaviour around patterns](https://github.com/rust-lang/rust/pull/138961)

[Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Use `annotate-snippets` as the default emitter](https://github.com/rust-lang/compiler-team/issues/947)
* [Promote powerpc64-unknown-linux-musl to tier 2 with host tools](https://github.com/rust-lang/compiler-team/issues/946)
* [Proposal for a dedicated test suite for the parallel frontend](https://github.com/rust-lang/compiler-team/issues/906)
* [Promote tier 3 riscv32 ESP-IDF targets to tier 2](https://github.com/rust-lang/compiler-team/issues/864)
* [Proposal for Adapt Stack Protector for Rust](https://github.com/rust-lang/compiler-team/issues/841)
* [Give integer literals a sign instead of relying on negation expressions](https://github.com/rust-lang/compiler-team/issues/835)
* [Also enable ICE file dumps on stable](https://github.com/rust-lang/compiler-team/issues/809)
* [New Tier-3 target proposal: `loongarch64-linux-android`](https://github.com/rust-lang/compiler-team/issues/806)

[Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [Adding a crates.io Security tab](https://github.com/rust-lang/rfcs/pull/3872)

[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [feat: stabilize `-Zconfig-include`](https://github.com/rust-lang/cargo/pull/16284)

*No Items entered Final Comment Period this week for
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Complex numbers](https://github.com/rust-lang/rfcs/pull/3892)
* [RFC: Const self fields](https://github.com/rust-lang/rfcs/pull/3888)
* [Create 0000-pub_use_pub_glob.md](https://github.com/rust-lang/rfcs/pull/3887)

## Upcoming Events

Rusty Events between 2025-12-03 - 2025-12-31 🦀

### Virtual
* 2025-11-26 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.github.io)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/q5tjirkt)
* 2025-11-27 | Virtual (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Meet de Noviembre - Async Runtimes, parte 2!**](https://www.meetup.com/rust-argentina/events/312061828/)
* 2025-11-30 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109188/)
* 2025-12-02 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Advent of Code - Kick Off!**](https://www.meetup.com/women-in-rust/events/311068648/)
* 2025-12-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 2025-12-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/311886445/)
* 2025-12-04 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046643/)
* 2025-12-05 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Kick-Off!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/311103307/)
* 2025-12-06 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763878687)
* 2025-12-07 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Finale**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/311103329/)
* 2025-12-09 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361537/)
* 2025-12-10 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/li5de4ts)
* 2025-12-11 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**December, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351054/)
* 2025-12-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/306046672/)
* 2025-12-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/310002338/)
* 2025-12-17 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/309926569/)
* 2025-12-17 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/6v2rorp3)
* 2025-12-18 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046644/)
* 2025-12-23 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361448/)

### Asia
* 2025-12-08 | Tokyo, JP | [Rust Global: Tokyo](https://rustfoundation.org/event/rust-global-tokyo/)
    * [**Rust Global: Tokyo**](https://rustfoundation.org/event/rust-global-tokyo/)
* 2025-12-20 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**December 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/december-2025-rustacean-meetup/)

### Europe
* 2025-11-26 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern)
    * [**2025 Rust Talks Bern #5 @Source Engineers**](https://www.meetup.com/rust-bern/events/311792568/)
* 2025-11-27 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #16: Christian Meusel - Oxidizing Step by Step**](https://rust-augsburg.github.io/meetup/Meetup_16.html)
* 2025-11-27 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust)
    * [**19th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/311787736/)
* 2025-11-27 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin on location 🏳️‍🌈 - Edition 009**](https://www.meetup.com/rust-berlin/events/312169727/)
* 2025-11-27 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #63 sponsored by Adapt!**](https://www.meetup.com/copenhagen-rust-community/events/312070502/)
* 2025-11-27 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Exotically Sized Types, and Rust in Space at Spire!**](https://www.meetup.com/rust-and-friends/events/311753411/)
* 2025-11-28 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Prague @ Barclays**](https://www.meetup.com/rust-prague/events/311846118/)
* 2025-12-03 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 12 2025**](https://luma.com/8ncu1p8l)
* 2025-12-03 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Rust/ACCU meetup.**](https://www.meetup.com/oxford-rust-meetup-group/events/311994790/)
* 2025-12-04 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna)
    * [**Rust Vienna S2E2 - December | at metalab 🦀**](https://www.meetup.com/rust-vienna/events/311680386/)
* 2025-12-08 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - December 2025**](https://www.meetup.com/rust-dortmund/events/312165912/)
* 2025-12-08 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #81**](https://www.meetup.com/rust-paris/events/312004357/)
* 2025-12-10 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2025 / 4 - Hacking Evening**](https://www.meetup.com/rust-munich/events/307105932/)
* 2025-12-10 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944053/)
* 2025-12-16 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Bergen #3 @ Zrch**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/312037597)
* 2025-12-16 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592258/)

### North America
* 2025-11-26 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310457310/)
* 2025-11-26 | Phoenix, AZ, US | [Desert Rust](https://www.meetup.com/desert-rustaceans)
    * [**Booze.rs**](https://www.meetup.com/desert-rustaceans/events/312000222/)
* 2025-11-27 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/jqvvttyhcpbkc/)
* 2025-11-29 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Harvard Square Rust Lunch, Nov 29**](https://www.meetup.com/bostonrust/events/311917256/)
* 2025-12-02 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Talk December**](https://www.meetup.com/chicago-rust-meetup/events/311736848/)
* 2025-12-04 | México City, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Optimizando rendimiento de Python con Rust**](https://www.meetup.com/rust-mx/events/312052780/)
* 2025-12-04 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Actix Web Unleashed: Mastering State, Security, and Scalable Handlers in Rust**](https://www.meetup.com/stl-rust/events/311396006/)
* 2025-12-05 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC Unconf 2025: Our Biggest Event Yet!**](https://www.meetup.com/rust-nyc/events/311757146/)
* 2025-12-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Downtown Rust Lunch, Dec 6**](https://www.meetup.com/bostonrust/events/311917263/)
* 2025-12-11 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**December, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351054/)
* 2025-12-11 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Competetive Robotics with Rust**](https://www.meetup.com/utah-rust/events/311613704/)
* 2025-12-11 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/312103517/)
* 2025-12-11 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust December Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/312009598/)
* 2025-12-13 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Alewife Rust Lunch, Dec 13**](https://www.meetup.com/bostonrust/events/311917267/)
* 2025-12-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/308865807/)
* 2025-12-17 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/309926569/)
* 2025-12-20 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Back Bay Rust Lunch, Dec 20**](https://www.meetup.com/bostonrust/events/311917280/)
* 2025-12-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/312076080/)

### Oceania
* 2025-12-11 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Rust Meetup Dec 2025**](https://www.meetup.com/rust-brisbane/events/312027415/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> \[...\] just returning an error is not *error handling*, it is just user space unwinding.

– [Ddystopia on Rust-internals](https://internals.rust-lang.org/t/re-opening-deprecating-option-unwrap-and-result-unwrap/23734/45)

Thanks to [Aleksander Krauze](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1734) for the suggestion!

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
