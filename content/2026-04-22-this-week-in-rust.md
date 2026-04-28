Title: This Week in Rust 648
Number: 648
Date: 2026-04-22
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
* [crates.io: Help test our new web frontend](https://blog.rust-lang.org/inside-rust/2026/04/17/crates-io-svelte-public-testing/)
* [Announcing Rust 1.95.0 | Rust Blog](https://blog.rust-lang.org/2026/04/16/Rust-1.95.0/)

### Foundation
* RustConf 2026 [schedule](https://rustconf.com/schedule/) and [registration](https://rustconf.com/register) are live! Early bird ticket prices are available through April 29.

### Project/Tooling Updates
* [axum-harness: agent-native backend architecture template for Axum — semantic-first, topology-late, multi-agent harness](https://github.com/openclosed-org/axum-harness/blob/main/docs/launch.md)
* [lean-decimal: 2~6X faster than `rust_decimal`](https://github.com/WuBingzheng/lean-decimal/blob/main/benches/README.md)
* [Building Semantic Version Control in Rust](https://therohansharma.com/semantic-version-control-rust)
* [Oxanus v1.0 - Job processing library](https://github.com/pragmaplatform/oxanus/releases/tag/v1.0)
* [flodl 0.5.2: HuggingFace, in Rust](https://flodl.dev/blog/huggingface)
* [One Sized trait does not fit all](https://lwn.net/SubscriberLink/1067220/f4b7acbc7ce7d1fa/)
* [tinyboot v0.4.0 Released — The API is Stable](https://aaronqian.com/log/2026-04-22-tinyboot-v040-released/)
* [Slint 1.16 Released](https://slint.dev/blog/slint-1.16-released)
* [Danube Messaging adds Key-Shared subscriptions](https://danube-docs.dev-state.com/architecture/key_shared_architecture/)
* [Announcing mtp-mount: pure-Rust FUSE mount for MTP devices](https://www.veszelovszki.com/a/mtp-mount/)
* [wrkflw v0.8.0 - Validate and Run GitHub Actions locally.](https://github.com/bahdotsh/wrkflw/releases/tag/v0.8.0)
* [lean-ctx v3.4.4 - Context runtime for AI coding agents with 46 MCP tools, reducing LLM token costs by 60-99%](https://github.com/yvgude/lean-ctx)

### Observations/Thoughts
* [Cryptographic Right Answers: Post Quantum and Rust Edition](https://kerkour.com/post-quantum-cryptography-recommendations-rust)
* [Learning rust through an LLM to develop a TUI RSS reader (and what I tell my students)](https://github.com/christo-auer/eilmeldung/blob/main/docs/llm-development.md)
* [What Happens When You Build an Inode-Style Vector in Rust](https://sot.dev/inode-style-vector-in-rust.html)
* [Ownership & Borrowing
versus Reference Counting](https://slicker.me/rust/ownership_and_borrowing_vs_reference_counting.html)
* [The Edge of Safe Rust](https://kyju.org/blog/tokioconf-2026/)
* [video] [Third Online Func Prog Sweden 2026](https://www.youtube.com/watch?v=fboHzVVfknU&t=340s)

### Rust Walkthroughs
* [video] [Build a Full Stack Twitter Clone web application in Rust (Axum & Leptos)](https://www.youtube.com/watch?v=WmGv-lZgr7M)
* [The Impatient Programmer's Guide to Bevy and Rust: Chapter 12 - Let There Be Networking](https://aibodh.com/posts/bevy-rust-game-development-chapter-12/)
* [video] [RustCurious lesson 6: Enums and Polymorphism](https://www.youtube.com/watch?v=7dXQLr014JU)
* [A minimal VMM in Rust with Apple Hypervisor](https://gigapotential.dev/blog/minimal-vmm-in-rust-with-apple-hypervisor/)

* [Caching Expensive Functions in Rust with `cached`](https://kocharhook.com/post/5/caching-expensive-functions-in-rust/)

## Crate of the Week
This week's crate is [farben](https://github.com/razkar-studio/farben), a German-named macro crate for terminal colors.

Thanks to [Nik Revenco](https://users.rust-lang.org/t/crate-of-the-week/2704/1597) for the suggestion!

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

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->

* [rust-cookbook - Add Asynchronous section with tokio runtime recipes](https://github.com/rust-lang-nursery/rust-cookbook/issues/759) ([other high impact examples](https://github.com/rust-lang-nursery/rust-cookbook/issues?q=is%3Aissue%20state%3Aopen%20label%3Aexample))
* [wacp-platform - Fix test-only clippy drifts in `wacp-runtime/tests.rs` + `console-db/queries/tests.rs`](https://github.com/AAkil98/wacp-platform/issues/2) ([other good first issues](https://github.com/AAkil98/wacp-platform/labels/good%20first%20issue))

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [**EuroRust**](https://sessionize.com/eurorust-2026/) | 2026-04-27 | Barcelona, Spain | 2026-10-14 - 2026-10-17
* [**NDC Techtown**](https://ndctechtown.com/call-for-papers) | 2026-05-03 | Kongsberg, Norway | 2026-09-21 to 23.

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

542 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-04-14..2026-04-21

#### Compiler
* [don't hash `DelayedLints`](https://github.com/rust-lang/rust/pull/155248)
* [refactor FnDecl and FnSig non-type fields into a new wrapper type](https://github.com/rust-lang/rust/pull/155223)
* [suggest removing `&` when awaiting a reference to a future](https://github.com/rust-lang/rust/pull/154933)
* [suggest returning a reference for unsized place from a closure](https://github.com/rust-lang/rust/pull/152162)

#### Library
* [abort in core](https://github.com/rust-lang/rust/pull/154604)
* [constify `Index`(`Mut`), `Deref`(`Mut`) for `Vec`](https://github.com/rust-lang/rust/pull/155054)
* [core/num: implement feature `integer_cast_extras`](https://github.com/rust-lang/rust/pull/154664)
* [`core::unicode`: Replace `Cased` table with `Lt`](https://github.com/rust-lang/rust/pull/154699)
* [libtest: use binary search for --exact test filtering](https://github.com/rust-lang/rust/pull/154865)
* [move `std::io::ErrorKind` to `core::io`](https://github.com/rust-lang/rust/pull/154654)

#### Rustdoc
* [fix `redundant_explicit_links` incorrectly firing (or not firing) under certain scenarios](https://github.com/rust-lang/rust/pull/155435)
* [preserve `doc(cfg)` on locally re-exported type aliases](https://github.com/rust-lang/rust/pull/154970)

#### Clippy
* [add MSRV check for `manual_noop_waker`](https://github.com/rust-lang/rust-clippy/pull/16850)
* [add `useless_borrows_in_formatting` lint](https://github.com/rust-lang/rust-clippy/pull/16523)
* [do not propose to refactor when no variant constructor is used](https://github.com/rust-lang/rust-clippy/pull/16867)
* [do not trigger `let_and_return` on `let else`](https://github.com/rust-lang/rust-clippy/pull/16829)
* [extend `byte_char_slices` to cover arrays](https://github.com/rust-lang/rust-clippy/pull/16770)
* [extend `zst_offset` lint to detect `NonNull<T>` offset calculations](https://github.com/rust-lang/rust-clippy/pull/16888)
* [fix a case where `collapsible_match` suggested a transformation that changes runtime behavior](https://github.com/rust-lang/rust-clippy/pull/16878)
* [fix `cloned_ref_to_slice_refs` false negative on `to_owned()`](https://github.com/rust-lang/rust-clippy/pull/16329)
* [fix `expect_fun_call` suggests wrongly for string slicing](https://github.com/rust-lang/rust-clippy/pull/16752)
* [fix `for_kv_map` false negative when using `iter` and `iter_mut`](https://github.com/rust-lang/rust-clippy/pull/16830)
* [parenthesize `AssocOp::Cast` in suggestion when replacement operator is `<` to avoid parse error](https://github.com/rust-lang/rust-clippy/pull/16848)
* [`useless_conversion`: do not lint `(a..b).into_iter()` (for edition migration)](https://github.com/rust-lang/rust-clippy/pull/16891)

#### Rust-Analyzer
* [`completion`: reduce relevance for deprecated items](https://github.com/rust-lang/rust-analyzer/pull/22085)
* [remove duplicate lints](https://github.com/rust-lang/rust-analyzer/pull/22054)
* [allow crate authors to declare that their trait prefers to be imported `as _`](https://github.com/rust-lang/rust-analyzer/pull/21740)
* [do not complete unstable items that use an internal feature](https://github.com/rust-lang/rust-analyzer/pull/22044)
* [exclude refs(find all refs) from deps and stdlib](https://github.com/rust-lang/rust-analyzer/pull/21906)
* [support extract variable in macro call](https://github.com/rust-lang/rust-analyzer/pull/21487)
* [add parentheses on record expr for `replace_let_with_if_let`](https://github.com/rust-lang/rust-analyzer/pull/22067)
* [adjust name of `extract_type_alias`](https://github.com/rust-lang/rust-analyzer/pull/22070)
* [allow ambiguity in assoc type shorthand if they resolve to the same assoc type, between supertraits this time](https://github.com/rust-lang/rust-analyzer/pull/22032)
* [port call expr type checking and closure upvar inference from rustc](https://github.com/rust-lang/rust-analyzer/pull/22101)
* [respect `#[deprecated]` attr when deciding if a `ModuleDef` completion is `deprecated`](https://github.com/rust-lang/rust-analyzer/pull/22083)
* [some fixes for `upvars_mentioned()`](https://github.com/rust-lang/rust-analyzer/pull/22055)
* [use `ProofTreeVisitor` for unsized coercion](https://github.com/rust-lang/rust-analyzer/pull/22096)
* [parse `type const` items](https://github.com/rust-lang/rust-analyzer/pull/22046)
* [perf: do not check solver's cache validity on every access](https://github.com/rust-lang/rust-analyzer/pull/22104)
* [sync function call args check fudging with rustc](https://github.com/rust-lang/rust-analyzer/pull/22092)

### Rust Compiler Performance Triage

This week was a bit all over the place, but the largest regressions were either
already fixed or they are being investigated. There were also a couple of nice perf. wins.

Triage done by **@Kobzol**.
Revision range: [dab8d9d1..9ab01ae5](https://perf.rust-lang.org/?start=dab8d9d1066c4c95008163c7babf275106ce3f32&end=9ab01ae53c416f89fe256b79588a76dcbcdc9290&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.7%  | [0.2%, 4.6%]   | 39    |
| Regressions ❌ <br /> (secondary)  | 0.6%  | [0.2%, 1.4%]   | 31    |
| Improvements ✅ <br /> (primary)   | -0.6% | [-4.8%, -0.1%] | 70    |
| Improvements ✅ <br /> (secondary) | -0.7% | [-4.1%, -0.0%] | 93    |
| All ❌✅ (primary)                 | -0.1% | [-4.8%, 4.6%]  | 109   |

3 Regressions, 4 Improvements, 6 Mixed; 4 of them in rollups
41 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/9c2cedf21859ce1404fe1265ab518ca243d1d20b/triage/2026/2026-04-21.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Error on invalid macho section specifier](https://github.com/rust-lang/rust/pull/155065)
* [Allow trailing `self` in more contexts](https://github.com/rust-lang/rust/pull/155137)
* [Add FCW to disallow `$crate` in macro matcher](https://github.com/rust-lang/rust/pull/155121)
* [Lint unused pub items in binary crates](https://github.com/rust-lang/rust/pull/149509)
* [const-stabilize `char::is_control()`](https://github.com/rust-lang/rust/pull/155528)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Stabilize `build-dir` layout v2](https://github.com/rust-lang/cargo/pull/16807)
* [feat(compile): Stabilize `build.warnings`](https://github.com/rust-lang/cargo/pull/16796)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)

* [Promote riscv64gc-unknown-linux-musl to Tier 2 (with Tools)](https://github.com/rust-lang/compiler-team/issues/982)
* [Make stable hashing names consistent](https://github.com/rust-lang/compiler-team/issues/983)

*No Items entered Final Comment Period this week for
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [Add contribution policy for AI-generated work](https://github.com/rust-lang/rfcs/pull/3950)
* [Bounded Trait Casting](https://github.com/rust-lang/rfcs/pull/3952)
* [Support heterogeneous try blocks (`try_blocks_heterogeneous`) RFC](https://github.com/rust-lang/rfcs/pull/3953)

## Upcoming Events

Rusty Events between 2026-04-22 - 2026-05-20 🦀

### Virtual
* 2026-04-22 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/26dvwb85)
* 2026-04-23 | Virtual (Amsterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development)
    * [**Bevy Meetup #13**](https://www.meetup.com/bevy-game-development/events/313842977/)
* 2026-04-23 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455927/)
* 2026-04-24 | Virtual (Nairobi, KE) | [RustaceansKenya](http://luma.com/RustaceansKenya)
    * [**Transitioning To Rust: The Learning Curve**](https://luma.com/f4qezpay)
* 2026-04-28 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254783/)
* 2026-04-28 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: From Protobuf to Production - A Guide to gRPC in Rust**](https://www.meetup.com/women-in-rust/events/313505777/)
* 2026-04-28 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens)
    * [**Web development using axum in Rust - part 4**](https://www.meetup.com/code-mavens/events/314401473/)
* 2026-04-29 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/8hi2xywi)
* 2026-05-01 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Hacker's Hike 0x1**](https://www.meetup.com/rust-noris/events/312788983/)
* 2026-05-02 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763928837)
* 2026-05-03 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/314036479/)
* 2026-05-06 | Virtual (Cardiff, GB) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Practical introduction to SIMD**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/314301861/)
* 2026-05-06 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/rd05z3vo)
* 2026-05-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/314323890/)
* 2026-05-07 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455928/)
* 2026-05-07 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345240/)
* 2026-05-12 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254782/)
* 2026-05-12 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/313506068/)
* 2026-05-13 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/ooub1kt0)
* 2026-05-17 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/314329043/)
* 2026-05-19 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/rdhhptyjchbzb/)
* 2026-05-20 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/548kbqhl)
* 2026-05-20 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Mouse Control with Rust**](https://www.meetup.com/vancouver-rust/events/313572925/)
    
### Asia
* 2026-05-13 | Malaysia, MY | [Rust Meetup Malaysia](https://docs.google.com/forms/d/e/1FAIpQLSfMh6PA05ujl3lS59tJU3DcLHGVZ1zjzJhl49hXEHU7e6vsQA/viewform)
    * [**Rust Meetup May 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfMh6PA05ujl3lS59tJU3DcLHGVZ1zjzJhl49hXEHU7e6vsQA/viewform)

### Europe
* 2026-04-23 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Talk Night and Birthday Party at MFT Energy**](https://www.meetup.com/rust-aarhus/events/313910468/)
* 2026-04-23 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #85**](https://www.meetup.com/rust-paris/events/314283634/)
* 2026-04-24 - 2026-04-26 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Future Week Augsburg: Road to Game Jam – Spielend Bevy und Rust lernen bei Tuxedo Computers**](https://www.tuxedocomputers.com/de/Road-to-Game-Jam-2026-Bevy-Workshop.tuxedo)
* 2026-04-25 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #26**](https://www.meetup.com/stockholm-rust/events/314227099/)
* 2026-04-29 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust meetup #67**](https://www.meetup.com/copenhagen-rust-community/events/314279730/)
* 2026-04-29 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.fr/o/74289178383)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1984135342220)
* 2026-04-30 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/314292918/)
* 2026-04-30 | Manchester, GB | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester April Talk**](https://www.meetup.com/rust-manchester/events/314229892/)
* 2026-05-02 | Augsburg, DE | [Rust Munich](https://rust-munich.de/) and [Rust Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Augsburger Linux-Infotag 2026: Gemeinschaftsstand Rust Augsburg und Rust München**](https://www.luga.de/static/LIT-2026/)
* 2026-05-04 | Amsterdam, NH, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ JetBrains**](https://www.meetup.com/rust-amsterdam-group/events/314268909/)
* 2026-05-04 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Writing a stock portfolio simulation in Rust with Leptos**](https://www.meetup.com/rust-rhein-main/events/314051688/)
* 2026-05-05 | Olomouc, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**5. Rust Moravia Meetup (Ukaž testy!)**](https://www.meetup.com/rust-moravia/events/314218493/)
* 2026-05-07 | Edinburgh, GB | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust May Talks: Aetherus + TBA**](https://www.meetup.com/rust-and-friends/events/314300802/)
* 2026-05-13 | Girona, ES | [Rust Girona](https://luma.com/rust-girona)
    * [**Rust Girona Hack & Learn 05 2026**](https://luma.com/ooub1kt0)
* 2026-05-14 | Switzerland, CH | [PostTenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-05-18 | Milano, MI, IT | [Rust Language Milan](https://www.meetup.com/rust-language-milano/events/)
    * [**RustWeek 2026**](https://www.meetup.com/rust-language-milan/events/314329200/)
* 2026-05-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/314129975/)
* 2026-05-19 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam/events/)
    * [**RustWeek 2026 announcement**](https://www.meetup.com/rust-nederland/events/312861992/)
* 2026-05-19 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Cross-Building & Cross-Testing**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813902/)
* 2026-05-19 | London, UK | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**RustWeek lunch meetup**](https://www.meetup.com/women-in-rust/events/314313054/)

### North America
* 2026-04-20 - 2026-04-22 | Portland, OR | [Tokio](https://tokio.rs/)
    * [**TokioConf 2026**](https://www.tokioconf.com/)
* 2026-04-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/314000435/)
* 2026-04-22 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Formally Verified Rust & SAT Solvers**](https://www.meetup.com/rust-nyc/events/314167944/)
* 2026-04-22 | Portland, OR | [**Apache DataFusion Meetup**](https://luma.com/dsp3ud82)
    * [**Portland Apache DataFusion Meetup**](https://luma.com/dsp3ud82)
* 2026-04-23 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA April!**](https://www.meetup.com/rust-los-angeles/events/313542139/)
* 2026-04-25 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**South Station Rust Lunch, Apr 25**](https://www.meetup.com/bostonrust/events/313883704/)
* 2026-04-28 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC x OpenAI: Safer 'unsafe' & Barnum: The agentic workflow engine.**](https://www.meetup.com/rust-nyc/events/314180711/)
* 2026-04-30 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/311228662/)
* 2026-05-07 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Open Project Night**](https://www.meetup.com/stl-rust/events/313807225/)
* 2026-05-14 | Portland, OR, US | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**From Radio Waves to Pixels - Real-Time Visualizations with Rust and WebAssembly**](https://www.meetup.com/pdxrust/events/314256732/)
* 2026-05-14 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust/events/)
    * [**San Diego Rust May Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/313721886/)
* 2026-05-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/314154841/)
* 2026-05-20 | San Francisco, CA, US | [Bay Area Rust Meetup](https://luma.com/bayarearust)
    * [**Bay Area Rust Meetup**](https://luma.com/9j3q5ejl)

### Oceania
* 2026-05-14 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/events/)
    * [**Rust Melbourne - May 2026**](https://www.meetup.com/rust-melbourne/events/314260890/)
    
If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1sobu1s/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> in Rust we pay the price of composition up-front

– [Nadieril on rust zulip](https://rust-lang.zulipchat.com/#narrow/channel/213817-t-lang/topic/broken.20and.20un-fixable.20parts.20of.20Rust/near/587758938)

Thanks to [Nadieril](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1763) for the self-suggestion!

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1su40pd/this_week_in_rust_648/)</small>
