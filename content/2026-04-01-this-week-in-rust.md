Title: This Week in Rust 645
Number: 645
Date: 2026-04-01
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
* [Announcing Rust 1.94.1](https://blog.rust-lang.org/2026/03/26/1.94.1-release/)
* [Program management update — February 2026 ](https://blog.rust-lang.org/inside-rust/2026/03/27/program-management-update-2026-02/)

### Foundation
* [Rust Foundation Joins Datadog’s Open Source Program](https://rustfoundation.org/media/rust-foundation-joins-datadogs-open-source-program/)

### Project/Tooling Updates
* [Ntpd-rs: it's about time!](https://discourse.ubuntu.com/t/ntpd-rs-its-about-time/79154)
* [octopos: OS for risc-v in rust](https://www.boranseckin.com/projects/octopos)
* [Building a guitar trainer with embedded Rust](https://blog.orhun.dev/introducing-tuitar/)
* [blogr v0.5.0 - blog without from your terminal](https://github.com/bahdotsh/blogr/releases/tag/v0.5.0)
* [feedr v0.7.0 - terminal-based RSS/Atom feed reader](https://github.com/bahdotsh/feedr/releases/tag/v0.7.0)
* [mdterm v2.0.0 - terminal-based Markdown browser](https://github.com/bahdotsh/mdterm/releases/tag/v2.0.0)
* [RustGrep: simple search for 114 Rust blogs](https://rustgrep.dev/)
* [Rust's next-generation trait solver](https://lwn.net/SubscriberLink/1063124/fcf747e51a5974f2/)
* [Portable Async Rust](https://aimdb.dev/blog/building-aimdb-one-async-api)
* [jsongrep faster than {jq, jmespath, jsonpath-rust, jql}](https://micahkepe.com/blog/jsongrep/)
* [SeqPacker: 11 bin-packing algorithms in Rust](https://alphakhaw.com/blog/seqpacker-bin-packing-algorithms-rust-llm)
* [flodl v0.2.2: PyTorch parity in Rust](https://flodl.dev/blog/pytorch-parity)

### Observations/Thoughts
* [filtra.io | Breaking The AI Infra Monopoly With Rust- Tracel AI](https://filtra.io/rust/interviews/tracel-mar-26)
* [Rust: Memory safety in kernel space | OSHub](https://oshub.org/users/OSHub/posts/rust-memory-safety-in-kernel-space-9178dd)
* [Fixing our own problems in the Rust compiler](https://trifectatech.org/blog/fixing-our-own-problems-in-the-rust-compiler/)
* [Bugs that the Rust compiler catches for you: The revolution of compiler-enforced correctness](https://kerkour.com/rust-compiler-correctness-bugs)
* [I ported the OpenAI Python SDK to Rust in 5 days with Claude Code](https://dev.to/fortunto2/squeezing-every-millisecond-from-the-openai-api-in-rust-4b11)
* [video] [🦀 Rust (mir) compiler bites: Closures — thou shalt not name this struct](https://www.youtube.com/watch?v=OxK5pNvC20Y)
* [video] [How C++ Finally Beats Rust at JSON Serialization](https://www.youtube.com/watch?v=Mcgk3CxHYMs)

### Rust Walkthroughs
* [Adding WASM Plugins to Your App](https://blog.ar-ms.me/thoughts/adding-wasm-plugins-to-your-app/)
* [ZK snarks for rust developer part 3/8](https://rustarians.com/execution-trace/)
* [Building a Crash-Safe Email Queue in Rust](https://ferax564.github.io/rustqueue/blog/crash-safe-email-queue.html)
* [Adding a Scripting Engine to a Rust CLI with Rhai](https://dev.to/ayarotsky/adding-a-scripting-engine-to-a-rust-cli-with-rhai-56g1)

## Crate of the Week

This week's crate is [tsastat](https://github.com/AnkurRathore/tsastat), a high-resolution Thread State Analysis (TSA) tool for Linux.

Thanks to [Ankur Rathore](https://users.rust-lang.org/t/crate-of-the-week/2704/1574) for the self-suggestion!

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
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [**NDC Techtown**](https://ndctechtown.com/call-for-papers) | CFP open until 2024-04-14 | Kongsberg, Norway | 2024-09-09 - 2026-09-12.
* [**EuroRust**](https://sessionize.com/eurorust-2026/) | CFP open until 2026-04-27 | Barcelona, Spain | 2026-10-14 - 2026-10-17

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

487 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-03-24..2026-03-31

#### Compiler
* [add `x86_64-unknown-linux-gnu{m,t}san` target which enables {M,T}San by default](https://github.com/rust-lang/rust/pull/152757)
* [add `-Zsanitize=kernel-hwaddress`](https://github.com/rust-lang/rust/pull/153049)

#### Library
* [constify `Step` trait and all of its `impl`ementations](https://github.com/rust-lang/rust/pull/153821)
* [constify comparisons and `Clone` for `core::mem::Alignment`](https://github.com/rust-lang/rust/pull/154512)
* [constify const Fn*: Destruct](https://github.com/rust-lang/rust/pull/153874)
* [don't drop arguments' temporaries in `dbg!`](https://github.com/rust-lang/rust/pull/154074)
* [don't fuse in `MapWindows`](https://github.com/rust-lang/rust/pull/154190)
* [implement `unchecked_funnel_{shl,shr}`](https://github.com/rust-lang/rust/pull/154153)
* [reimplement `hash_map!` macro](https://github.com/rust-lang/rust/pull/154322)
* [make `PinCoerceUnsized` require `Deref`](https://github.com/rust-lang/rust/pull/149218)
* [stabilize new RangeFrom type and iterator](https://github.com/rust-lang/rust/pull/153380)
* [`trim_prefix` for paths](https://github.com/rust-lang/rust/pull/154320)

#### Cargo
* [`resolver`: better match rustc in error styling](https://github.com/rust-lang/cargo/pull/16795)
* [`build`: cover more behavior of `build.warnings`](https://github.com/rust-lang/cargo/pull/16785)
* [`build`: make it easier to review `build.warnings` behavior](https://github.com/rust-lang/cargo/pull/16788)

#### Rustdoc
* [rustdoc rejects html emits with json output](https://github.com/rust-lang/rust/pull/154421)

#### Rustfmt
* [prevent panic when rewritng associated item delegations](https://github.com/rust-lang/rust/pull/154454)

#### Clippy
* [add `manual_option_zip` lint (`a.and_then(|x| b.map(|y| (x, y)))`)](https://github.com/rust-lang/rust-clippy/pull/16600)
* [impl `manual_noop_waker` lint](https://github.com/rust-lang/rust-clippy/pull/16687)
* [`explicit_counter_loop`: suggest `.take(n)` for `for _ in 0..n` co…](https://github.com/rust-lang/rust-clippy/pull/16658)
* [`iter_kv_map`: handle identity map for `map` and `flat_map`](https://github.com/rust-lang/rust-clippy/pull/16743)
* [`manual_pop_if`: lint more cases, even if we do not provide a suggestion](https://github.com/rust-lang/rust-clippy/pull/16683)
* [fix `collapsible_if` false positive when the inner if contains cfg](https://github.com/rust-lang/rust-clippy/pull/16757)
* [preserve parentheses in suggestion in presence of cascaded casts](https://github.com/rust-lang/rust-clippy/pull/16483)
* [perf: reduce `matching_root_macro_call` usage (23b → 22.24b)](https://github.com/rust-lang/rust-clippy/pull/16756)

#### Rust-Analyzer
* [fix not applicable on ambiguous ident pat for `merge_match_arms`](https://github.com/rust-lang/rust-analyzer/pull/21411)
* [complete envs in nested `env!()`](https://github.com/rust-lang/rust-analyzer/pull/21902)
* [correct `type_or_const` param index bound in `debug_assert`](https://github.com/rust-lang/rust-analyzer/pull/21879)
* [correct missing-args messages for `sched_getaffinity` and getenv shims](https://github.com/rust-lang/rust-analyzer/pull/21881)
* [don't panic unmerge arm on trailing pipe](https://github.com/rust-lang/rust-analyzer/pull/21904)
* [fix block lowering in ast id map](https://github.com/rust-lang/rust-analyzer/pull/21907)
* [keep comments for 'Fill match arms'](https://github.com/rust-lang/rust-analyzer/pull/21744)
* [postfix completions include nots prefix-expr](https://github.com/rust-lang/rust-analyzer/pull/21903)
* [skip usages inside macro expansions in destructure struct/tuple binding](https://github.com/rust-lang/rust-analyzer/pull/21838)
* [turn back `TyLoweringContext.store` to self after lowering parent defaults](https://github.com/rust-lang/rust-analyzer/pull/21871)
* [wrap `Option<>` for `desugar_try_expr_let_else`](https://github.com/rust-lang/rust-analyzer/pull/21860)
* [wrap `Result<>` for `desugar_try_expr_let_else`](https://github.com/rust-lang/rust-analyzer/pull/21865)
* [wrap ty-anchor in non-path type constuctor](https://github.com/rust-lang/rust-analyzer/pull/21876)
* [fully implement `VariantFields expression support`](https://github.com/rust-lang/rust-analyzer/pull/21900)
* [hookup Signature Inference in more places](https://github.com/rust-lang/rust-analyzer/pull/21859)
* [only allocate item blocks if they actually contain items or statement macros](https://github.com/rust-lang/rust-analyzer/pull/21901)
* [remove `Arc` from `GenericParams` and `AstIdMap`](https://github.com/rust-lang/rust-analyzer/pull/21897)
* [remove generate trait impl text intransitive from utils](https://github.com/rust-lang/rust-analyzer/pull/21870)

### Rust Compiler Performance Triage

We had some infrastructure troubles this week which prevented some rollup PRs from generating their
"unrolled" builds, which made rollup regression investigation more complicated, although we were
able to locate and revert the largest rollup regressions in the end. [#154304](https://github.com/rust-lang/rust/pull/154304) brought some nice improvements by optimizing the query system.

Triage done by **@kobzol**.
Revision range: [6f22f613..cf7da0b7](https://perf.rust-lang.org/?start=6f22f61305478df09f9a4523743f85d9f558c3d7&end=cf7da0b7277cad05b79f91b60c290aa08a17a6f0&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.4%  | [0.1%, 1.2%]   | 4     |
| Regressions ❌ <br /> (secondary)  | 0.3%  | [0.1%, 0.5%]   | 12    |
| Improvements ✅ <br /> (primary)   | -0.8% | [-6.2%, -0.2%] | 58    |
| Improvements ✅ <br /> (secondary) | -0.4% | [-1.9%, -0.1%] | 28    |
| All ❌✅ (primary)                 | -0.8% | [-6.2%, 1.2%]  | 62    |

3 Regressions, 4 Improvements, 2 Mixed; 2 of them in rollups
35 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/23c7a57ddd710830f9ae14d2676718587e9dc412/triage/2026/2026-03-31.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Mitigation enforcement](https://github.com/rust-lang/rfcs/pull/3855)
* [Add `homogeneous_try_blocks`](https://github.com/rust-lang/rfcs/pull/3721)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Tracking Issue for `isolate_most_least_significant_one`](https://github.com/rust-lang/rust/issues/136909)
* [Tracking Issue for `int_lowest_highest_one`](https://github.com/rust-lang/rust/issues/145203)
* [Tracking Issue for `uint_bit_width`](https://github.com/rust-lang/rust/issues/142326)
* [Tracking Issue for #138068: Add `Result::map_or_default` and `Option::map_or_default`](https://github.com/rust-lang/rust/issues/138099)
* [Do not use non-wf input expectations from fudge when checking function calls](https://github.com/rust-lang/rust/pull/150316)
* [Syntactically reject equality predicates](https://github.com/rust-lang/rust/pull/153513)
* [Tracking Issue for tcp_deferaccept](https://github.com/rust-lang/rust/issues/119639)
* [stabilize s390x vector registers](https://github.com/rust-lang/rust/pull/154184)
* [Replacing self overwriting with proper resolution](https://github.com/rust-lang/rust/pull/152996)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)

* [build-std: always](https://github.com/rust-lang/rfcs/pull/3874)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [RFC: Associated traits](https://github.com/rust-lang/rfcs/pull/3938)
* [Add \*be present\* contribution policy](https://github.com/rust-lang/rfcs/pull/3936)
* [initial draft for 2026 project goals](https://github.com/rust-lang/rfcs/pull/3935)

## Upcoming Events

Rusty Events between 2026-04-01 - 2026-04-29 🦀

### Virtual
* 2026-03-26 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455925/)
* 2026-03-31 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens)
    * [**Web development using axum in Rust - part 1**](https://www.meetup.com/code-mavens/events/313944077/)
* 2026-04-01 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/me4jwgxu)
* 2026-04-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/313656388/)
* 2026-04-02 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345237/)
* 2026-04-04 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-04-05 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/313846136/)
* 2026-04-07 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens)
    * [**Web development using axum in Rust - part 2**](https://www.meetup.com/code-mavens/events/313944233/)
* 2026-04-09 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455926/)
* 2026-04-14 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens)
    * [**Web development using axum in Rust - part 3**](https://www.meetup.com/code-mavens/events/314072969/)
* 2026-04-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254784/)
* 2026-04-14 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/313506013/)
* 2026-04-15 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/jia7wtfv)
* 2026-04-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Nushell**](https://www.meetup.com/vancouver-rust/events/313471712/)
* 2026-04-16 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**April, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873177/)
* 2026-04-19 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/313846195/)
* 2026-04-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/314007434/)
* 2026-04-22 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/26dvwb85)
* 2026-04-23 | Virtual (Amsterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development)
    * [**Bevy Meetup #13**](https://www.meetup.com/bevy-game-development/events/313842977/)
* 2026-04-23 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455927/)
* 2026-04-28 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254783/)
* 2026-04-28 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: From Protobuf to Production - A Guide to gRPC in Rust**](https://www.meetup.com/women-in-rust/events/313505777/)
* 2026-04-29 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/8hi2xywi)

### Asia
* 2026-04-11 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**April 2026/Pre-Conference Rustacean meetup**](https://hasgeek.com/rustbangalore/april-2026-pre-conference-rustacean-meetup/)
* 2026-04-17 | Bangalore, IN | [Rust India](https://rustindia.org/)
    * [**Rust India Workshop**](https://rustindia.org/schedule)
* 2026-04-18 | Bangalore, IN | [Rust India](https://rustindia.org/)
    * [**Rust India Conference**](https://rustindia.org/schedule)

### Europe
* 2026-04-01 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/313783250/)
* 2026-04-01 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (evening pub)**](https://www.meetup.com/rust-and-friends/events/313898254/)
* 2026-04-01 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn)
    * [**Rust in April: From Zero to Rust**](https://www.meetup.com/rustcologne/events/313947839/)
* 2026-04-01 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**It came from another dimension!**](https://www.meetup.com/oxford-rust-meetup-group/events/312664491/)
* 2026-04-02 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group)
    * [**LDN Talks Spring Community Showcase**](https://www.meetup.com/rust-london-user-group/events/313816694/)
* 2026-04-02 | Toulouse, FR | [Rust Toulouse](https://www.meetup.com/rust-community-toulouse)
    * [**Rust Toulouse Meetup - Release, Servers & Ray Tracing Demystified**](https://www.meetup.com/rust-community-toulouse/events/313650892/)
* 2026-04-03 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/313898258/)
* 2026-04-07 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #15 @ letsboot**](https://www.meetup.com/rust-basel/events/313765547/)
* 2026-04-07 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Building a drone from scratch with Rust (and some hardware)**](https://www.meetup.com/rust-rhein-main/events/314051660/)
* 2026-04-08 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 04 2026**](https://luma.com/z8aoscr9)
* 2026-04-09 | Geneva, CH | [Rust Meetup Geneva](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-04-09 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust talks @ AutoStore – "Patterns for Event Driven Systems" and "Rust + WASM"**](https://www.meetup.com/rust-oslo/events/313806765/)
* 2026-04-21 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Native GUIs with Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813853/)
* 2026-04-23 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Talk Night and Birthday Party at MFT Energy**](https://www.meetup.com/rust-aarhus/events/313910468/)

### North America
* 2026-04-02 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/313543900/)
* 2026-04-02 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**SIUE Cruft Crawler with LLM**](https://www.meetup.com/stl-rust/events/313482094/)
* 2026-04-04 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Winter Hill Rust Lunch, Apr 4**](https://www.meetup.com/bostonrust/events/313883689/)
* 2026-04-07 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: The Open Source Special**](https://www.meetup.com/rust-nyc/events/313946458/)
* 2026-04-09 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/313987321/)
* 2026-04-09 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust April Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/313721879/)
* 2026-04-11 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Brookline Rust Lunch, Apr 11**](https://www.meetup.com/bostonrust/events/313883710/)
* 2026-04-14 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Sharpening Your Rust Skills for Job Interviews**](https://www.meetup.com/charlottesville-rust-meetup/events/313262215/)
* 2026-04-15 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Nushell**](https://www.meetup.com/vancouver-rust/events/313471712/)
* 2026-04-16 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**April, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873177/)
* 2026-04-18 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Harvard Square Rust Lunch, Apr 18**](https://www.meetup.com/bostonrust/events/313883701/)
* 2026-04-20 - 2026-04-22 | Portland, OR | [Tokio](https://tokio.rs/)
    * [**TokioConf 2026**](https://www.tokioconf.com/)
* 2026-04-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/313918531/)
* 2026-04-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/314000435/)
* 2026-04-23 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA April!**](https://www.meetup.com/rust-los-angeles/events/313542139/)
* 2026-04-25 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**South Station Rust Lunch, Apr 25**](https://www.meetup.com/bostonrust/events/313883704/)

### Oceania
* 2026-04-09 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Rust Brisbane Apr 2026**](https://www.meetup.com/rust-brisbane/events/313975190/)

### South America
* 2026-04-11 | Buenos Aires, AR | [Oxidar Org](https://luma.com/user/oxidar)
    * [**Oxidar.org Hackaton - Snakear - ¡Veni a hackear con Rust!**](https://luma.com/5f51ey45)
* 2026-04-17 | Rio de Janeiro, BR | [Meetups Rust RJ](https://luma.com/calendar/cal-z65k0aMSe7DaqKv)
    * [**Meetup Rust RJ**](https://luma.com/ce46pl7z)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1rmra27/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> When you do cursed things, problems find you.

– [Folkert de Vries on the trifecta tech blog](https://trifectatech.org/blog/fixing-our-own-problems-in-the-rust-compiler)

We have gone four weeks bare of suggestions for quotes. llogiq is still fine with his choice, but he'd be much more happy if any of you would help him in his search.

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

<small>[Discuss on r/rust](https://this-week-in-rust.org/blog/2026/04/01/this-week-in-rust-645/)</small>
