Title: This Week in Rust 647
Number: 647
Date: 2026-04-15
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
* [Infrastructure Team 2026 Q1 Recap and Q2 Plan](https://blog.rust-lang.org/inside-rust/2026/04/14/infrastructure-team-q1-recap-and-q2-plan/)

### Project/Tooling Updates
* [pquantum.dev: Post-Quantum Cryptography in Rust](https://pquantum.dev/)
* [haproxy-spoe-rs: A Rust SPOA Agent Library for HAProxy](https://blog.none.at/blog/2026/2026-04-12-haproxy-spoa-rs/)
* [Fresh 0.2.23: Terminal IDE adds Windows-1251 encoding, customizable status bar, and faster file finder](https://github.com/sinelaw/fresh/releases/tag/v0.2.23)
* [KAIO v0.2.0: Writing GPU Kernels in Rust at 92.5% of cuBLAS](https://netviper.gr/blog/kaio-v020/)
* [RustNet: A Real-Time Network Traffic Analysis TUI](https://netbeez.net/blog/rustnet/)
* [AimDB: The Next Era of Software Architecture Is Data-First](https://aimdb.dev/blog/data-driven-design)
* [tailscale-rs v0.2.0: our new Rust library preview](https://tailscale.com/blog/tailscale-rs-rust-tsnet-library-preview)
* [Sinbo: a CLI snippet manager, store code snippets locally with fuzzy search, encryption, and shell completions](https://dev.to/opmr0/i-built-a-cli-snippet-manager-in-rust-because-i-was-tired-of-googling-the-same-things-4j4g )
* [flodl v0.4.0: heterogeneous multi-GPU DDP with faster training and better convergence than solo GPU](https://flodl.dev/blog/ddp-benchmark)

### Observations/Thoughts
* [The acyclic e-graph: Cranelift's mid-end optimizer](https://cfallin.org/blog/2026/04/09/aegraph/)
* [Rust should have stable tail calls](https://trifectatech.org/blog/tail-calls-project-goal/)
* [Flat Error Codes Are Not Enough](https://home.expurple.me/posts/flat-error-codes-are-not-enough/)
* [No one owes you supply-chain security](https://purplesyringa.moe/blog/no-one-owes-you-supply-chain-security/)
* [Everything Should Be Typed: Scalar Types Are Not Enough](https://sot.dev/everything-should-be-typed.html)
* [Borrow-checking surprises](https://www.scattered-thoughts.net/writing/borrow-checking-surprises/)
* [A Roadmap for Building an Extended Standard Library for Rust](https://kerkour.com/rust-extended-standard-library)
* [Okay, what ACTUALLY uses Rust?](https://blog.goose.love/posts/what-actually-uses-rust/)
* [audio] [Netstack.FM episode 34 — Tokio with Carl Lerche (Ep 5 Remastered)](https://netstack.fm/#episode-34)

### Rust Walkthroughs
* [Untangling Tokio and Rayon in production: From 2s latency spikes to 94ms flat](https://posthog.com/blog/untangling-rayon-and-tokio)
* [Understanding Traceroute](https://tech.stonecharioteer.com/posts/2026/traceroute/)
* [Bringing Rust to the Pixel Baseband](https://security.googleblog.com/2026/04/bringing-rust-to-pixel-baseband.html)
* [Fixing DNS tail latency with a 5-line config and a 50-line function](https://numa.rs/blog/posts/fixing-doh-tail-latency.html)
* [Debloat your async Rust](https://tweedegolf.nl/en/blog/235/debloat-your-async-rust)
* [Learn Rust Ownership and Borrowing By Building Mini Grep](https://blog.sheerluck.dev/posts/learn-rust-ownership-by-building-mini-grep/)
* [Profiling Rust: A Flamegraph vs PGO, BOLT, and Native CPU Targeting](https://alphakhaw.com/blog/seqpacker-profiling-rust-flamegraph-pgo-bolt)
* [Bulletproof Rust Web: An opinionated guide to production-grade Axum applications](https://gruberb.github.io/bulletproof-rust-web/)
* [A minimal VMM in Rust with KVM](https://gigapotential.dev/blog/minimal-vmm-in-rust-with-kvm-hypervisor/)
* [claudectl: Building a TUI Dashboard for AI Coding Agents in Rust](https://mercurialsolo.github.io/posts/claudectl-tui-dashboard/)
* [video] [Build with Naz : Eliminate busy waiting with Rust Condvar](https://www.youtube.com/watch?v=HvCptpU5r_4)

## Crate of the Week

This week's crate is [Myth Engine](https://github.com/panxinmiao/myth), a high-performance, cross-platform rendering engine.

Thanks to [Pan Xinmiao](https://users.rust-lang.org/t/crate-of-the-week/2704/1590) for the self-suggestion!

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
*No Calls for participation were submitted this week.*

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [**EuroRust**](https://sessionize.com/eurorust-2026/) | CFP open until 2026-04-27 | Barcelona, Spain | 2026-10-14 - 2026-10-17

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

519 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-04-07..2026-04-14

#### Compiler
* [add `#![unstable_removed(..)]` attribute to track removed features](https://github.com/rust-lang/rust/pull/153335)
* [add suggestion to `.to_owned()` used on `Cow` when borrowing](https://github.com/rust-lang/rust/pull/154646)
* [avoid stack overflow in FindExprBySpan](https://github.com/rust-lang/rust/pull/153888)
* [enable `#[diagnostic::on_const]` for local impls](https://github.com/rust-lang/rust/pull/154609)
* [introduce a `#[diagnostic::on_unknown]` attribute](https://github.com/rust-lang/rust/pull/152901)
* [reduce size of `ImportData`](https://github.com/rust-lang/rust/pull/155167)
* [`ty::Alias` refactor](https://github.com/rust-lang/rust/pull/154758)
* [semantic checks of `impl` restrictions](https://github.com/rust-lang/rust/pull/154661)
* [stabilize s390x vector registers](https://github.com/rust-lang/rust/pull/154184)
* [store `chunk_domain_size` explicitly in `Chunk`](https://github.com/rust-lang/rust/pull/147802)

#### Library
* [add `const Default` impls for `LazyCell` and `LazyLock`](https://github.com/rust-lang/rust/pull/154929)
* [constify some `Iterator` methods](https://github.com/rust-lang/rust/pull/154729)
* [constify DoubleEndedIterator](https://github.com/rust-lang/rust/pull/151898)
* [constify `Step for NonZero<u*>`](https://github.com/rust-lang/rust/pull/154825)
* [don't leak internal temporaries from `dbg!`](https://github.com/rust-lang/rust/pull/154994)
* [explicitly forget the zero remaining elements in `vec::IntoIter::fold()`](https://github.com/rust-lang/rust/pull/148486)
* [impl const Residual for ControlFlow](https://github.com/rust-lang/rust/pull/155142)
* [initial functions to start on transmute v2](https://github.com/rust-lang/rust/pull/155084)
* [introduce `#[diagnostic::on_move]` on `Rc`](https://github.com/rust-lang/rust/pull/154678)
* [make `Box/Rc/Arc::into_array` allocator-aware (and add doctest)](https://github.com/rust-lang/rust/pull/154925)
* [stabilize feature `int_lowest_highest_one`](https://github.com/rust-lang/rust/pull/155147)
* [stabilize feature `isolate_most_least_significant_one`](https://github.com/rust-lang/rust/pull/155130)
* [stabilize feature `uint_bit_width`](https://github.com/rust-lang/rust/pull/155131)

#### Cargo
* [clean: add target directory validation](https://github.com/rust-lang/cargo/pull/16712)
* [`manifest`: allow git dependency alongside alternate registry](https://github.com/rust-lang/cargo/pull/16810)
* [`auth`: add auth scheme hint to token rejected error for alt registries](https://github.com/rust-lang/cargo/pull/16794)
* [`core`: use `closest_msg` to suggest similar member name for mistyped `-p`](https://github.com/rust-lang/cargo/pull/16844)
* [`lints`: ignore `unused_crate_dependencies` status](https://github.com/rust-lang/cargo/pull/16877)
* [`toml`: force script edition warnings on quiet](https://github.com/rust-lang/cargo/pull/16848)
* [copy cargo clean target-dir validation tests to `clean_new_layout.rs`](https://github.com/rust-lang/cargo/pull/16878)
* [never include use extra-filename in build scripts](https://github.com/rust-lang/cargo/pull/16855)
* [support target.'cfg(..)'.rustdocflags analogously to rustflags](https://github.com/rust-lang/cargo/pull/16846)

#### Rustdoc
* [fix pattern types rendering](https://github.com/rust-lang/rust/pull/154955)
* [dep-info for standalone markdown inputs](https://github.com/rust-lang/rust/pull/154352)
* [inherit inline attributes for declarative macros](https://github.com/rust-lang/rust/pull/154902)

#### Clippy
* [`fn_to_numeric_cast_any`: do not warn cast to raw pointer](https://github.com/rust-lang/rust-clippy/pull/14109)
* [even more fixes for handling of macros](https://github.com/rust-lang/rust-clippy/pull/16443)
* [extend `manual_filter` to cover `and_then`](https://github.com/rust-lang/rust-clippy/pull/16456)
* [fix `unused_async` false positive for stubs with args](https://github.com/rust-lang/rust-clippy/pull/16832)
* [fix wrong suggestion for `println_empty_string` with non-parenthesis delimiters](https://github.com/rust-lang/rust-clippy/pull/16846)
* [truncate constants to target type in comparison](https://github.com/rust-lang/rust-clippy/pull/16782)

#### Rust-Analyzer
* [changes to build scripts and config.toml should always refresh](https://github.com/rust-lang/rust-analyzer/pull/21969)
* [demoting completion relevance when an inherent impl already exists](https://github.com/rust-lang/rust-analyzer/pull/22031)
* [enhance runnable command placeholders](https://github.com/rust-lang/rust-analyzer/pull/21978)
* [support `impl` and `mut` restrictions](https://github.com/rust-lang/rust-analyzer/pull/22022)
* [fix `[env]` in `.cargo/config.toml` overriding process environment variables](https://github.com/rust-lang/rust-analyzer/pull/21995)
* [fix rustfmt relative custom command](https://github.com/rust-lang/rust-analyzer/pull/22010)
* [MIR evaluation of sized &T with recursive const fn](https://github.com/rust-lang/rust-analyzer/pull/22030)
* [check coercion, not unification, in "Fill `struct` fields", as the criteria to use an existing local as the field's value](https://github.com/rust-lang/rust-analyzer/pull/21971)
* [complete variants of hidden enums through public aliases](https://github.com/rust-lang/rust-analyzer/pull/22003)
* [consider the context of the path for `ImportAssets`](https://github.com/rust-lang/rust-analyzer/pull/21973)
* [diagnose cfged-out crate](https://github.com/rust-lang/rust-analyzer/pull/21981)
* [disable the fix for missing-fields when the fields are private](https://github.com/rust-lang/rust-analyzer/pull/21977)
* [enable vscode suggest in strings](https://github.com/rust-lang/rust-analyzer/pull/22018)
* [fix `ref_match` position when keyword prefix](https://github.com/rust-lang/rust-analyzer/pull/21999)
* [improve add some on block like expression](https://github.com/rust-lang/rust-analyzer/pull/21953)
* [improve label on `add_missing_match_arms` assist](https://github.com/rust-lang/rust-analyzer/pull/21920)
* [no complete term expressions on qualified path](https://github.com/rust-lang/rust-analyzer/pull/22009)
* [no deref index-expr for `extract_function`](https://github.com/rust-lang/rust-analyzer/pull/22025)
* [no imports on type anchor qualified path](https://github.com/rust-lang/rust-analyzer/pull/22012)
* [parse `cfg_attr` and `cfg` specially](https://github.com/rust-lang/rust-analyzer/pull/21965)
* [handle token mutability in edit flow as well](https://github.com/rust-lang/rust-analyzer/pull/21975)
* [migrate extract `struct` from `enum` variant to new SyntaxEditor and Port whitespace heuristics to SyntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/21996)
* [replace make from generate single field `struct` from with SyntaxFactory](https://github.com/rust-lang/rust-analyzer/pull/21997)
* [unwrap unnecessary result return type in `view_crate_graph`](https://github.com/rust-lang/rust-analyzer/pull/21992)

### Rust Compiler Performance Triage

This week was negative, mainly caused by a type system fix and because we had to temporarily revert some attribute cleanups that previously improved performance.

Triage done by **@panstromek**.
Revision range: [e73c56ab..dab8d9d1](https://perf.rust-lang.org/?start=e73c56abd0baf3dbaafbdc3ce6072a416aade867&end=dab8d9d1066c4c95008163c7babf275106ce3f32&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.4%  | [0.2%, 0.7%]   | 46    |
| Regressions ❌ <br /> (secondary)  | 0.5%  | [0.1%, 2.3%]   | 102   |
| Improvements ✅ <br /> (primary)   | -0.5% | [-0.6%, -0.4%] | 4     |
| Improvements ✅ <br /> (secondary) | -0.4% | [-0.6%, -0.2%] | 5     |
| All ❌✅ (primary)                 | 0.4%  | [-0.6%, 0.7%]  | 50    |


4 Regressions, 1 Improvement, 5 Mixed; 6 of them in rollups
41 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/2112fbf8fd6dac95c7447cd62d6c0c55c413ee67/triage/2026/2026-04-13.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Verify that penultimate segment of enum variant path refers to enum if it has args](https://github.com/rust-lang/rust/pull/154971)
* [deprecate `std::char` constants and functions](https://github.com/rust-lang/rust/pull/153873)
* [`impl Default` for `RepeatN`](https://github.com/rust-lang/rust/pull/139690)
* [Make std::fs::File Send on UEFI](https://github.com/rust-lang/rust/pull/154003)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [feat(config): Stabilize `resolver.lockfile-path` config](https://github.com/rust-lang/cargo/pull/16694)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)

* [Optimize `repr(Rust)` enums by omitting tags in more cases involving uninhabited variants.](https://github.com/rust-lang/compiler-team/issues/922)
* [Proposal for a dedicated test suite for the parallel frontend](https://github.com/rust-lang/compiler-team/issues/906)
* [Promote tier 3 riscv32 ESP-IDF targets to tier 2](https://github.com/rust-lang/compiler-team/issues/864)
* [Proposal for Adapt Stack Protector for Rust](https://github.com/rust-lang/compiler-team/issues/841)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)

* [Propose the Rust Foundation Maintainer fund](https://github.com/rust-lang/rfcs/pull/3931)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)

* [Fund the Content team (2026 allocation)](https://github.com/rust-lang/leadership-council/pull/279)

*No Items entered Final Comment Period this week for
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2026-04-15 - 2026-05-13 🦀

### Virtual
* 2026-04-15 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Nushell**](https://www.meetup.com/vancouver-rust/events/313471712/)
* 2026-04-15 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/jia7wtfv)
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
* 2026-04-24 | Virtual (Nairobi, KE) | [RustaceansKenya](http://luma.com/RustaceansKenya)
    * [**Transitioning To Rust: The Learning Curve**](https://luma.com/f4qezpay)
* 2026-04-28 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254783/)
* 2026-04-28 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: From Protobuf to Production - A Guide to gRPC in Rust**](https://www.meetup.com/women-in-rust/events/313505777/)
* 2026-04-29 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/8hi2xywi)
* 2026-05-01 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Hacker's Hike 0x1**](https://www.meetup.com/rust-noris/events/312788983/)
* 2026-05-02 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763928837)
* 2026-05-03 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/314036479/)
* 2026-05-06 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/rd05z3vo)
* 2026-05-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjchbjb/)
* 2026-05-07 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455928/)
* 2026-05-07 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345240/)
* 2026-05-12 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254782/)
* 2026-05-12 | Virtual (London, GB) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/313506068/)
* 2026-05-13 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/ooub1kt0)

### Asia
* 2026-04-17 | Bangalore, IN | [Rust India](https://rustindia.org/)
    * [**Rust India Workshop**](https://rustindia.org/schedule)
* 2026-04-18 | Bangalore, IN | [Rust India](https://rustindia.org/)
    * [**Rust India Conference**](https://rustindia.org/schedule)
* 2026-05-13 | Malaysia, MY | [Rust Meetup Malaysia](https://docs.google.com/forms/d/e/1FAIpQLSfMh6PA05ujl3lS59tJU3DcLHGVZ1zjzJhl49hXEHU7e6vsQA/viewform)
    * [**Rust Meetup Malaysia**](https://docs.google.com/forms/d/e/1FAIpQLSfMh6PA05ujl3lS59tJU3DcLHGVZ1zjzJhl49hXEHU7e6vsQA/viewform)

### Europe
* 2026-04-16 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin on location 🏳️‍🌈 - Edition 013**](https://www.meetup.com/rust-berlin/events/314249809/)
* 2026-04-21 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Native GUIs with Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813853/)
* 2026-04-23 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Talk Night and Birthday Party at MFT Energy**](https://www.meetup.com/rust-aarhus/events/313910468/)
* 2026-04-24 - 2026-04-26 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Future Week Augsburg: Road to Game Jam – Spielend Bevy und Rust lernen bei Tuxedo Computers**](https://www.tuxedocomputers.com/de/Road-to-Game-Jam-2026-Bevy-Workshop.tuxedo)
* 2026-04-25 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #26**](https://www.meetup.com/stockholm-rust/events/314227099/)
* 2026-04-29 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.fr/o/74289178383)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1984135342220)
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

### North America
* 2026-04-15 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Nushell**](https://www.meetup.com/vancouver-rust/events/313471712/)
* 2026-04-16 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**April, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873177/)
* 2026-04-16 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/313995065/)
* 2026-04-16 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Community Meetup**](https://www.meetup.com/music-city-rust-developers/events/314090462/)
* 2026-04-18 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Harvard Square Rust Lunch, Apr 18**](https://www.meetup.com/bostonrust/events/313883701/)
* 2026-04-20 - 2026-04-22 | Portland, OR | [Tokio](https://tokio.rs/)
    * [**TokioConf 2026**](https://www.tokioconf.com/)
* 2026-04-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/313918531/)
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

### South America
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

> the amount of times that I spend 15 min in the docs + coding which end up in a monstrous `or().flatten().map().is_ok_and()` only to get slapped by clippy saying `replace your monster with this single function please` is way too high 😀

– [Teufelchen on RIOT off-topic matrix chat](https://matrix.to/#/!zcNfLwklXSZMQlTOLN:matrix.org/$1dc35m_KEs4r2vCu3DJ44NCjrrz3EtxWBQUdRQzlBe4)

Thanks to [chrysn](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1760) for the suggestion!

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1smskwm/this_week_in_rust_647/)</small>
