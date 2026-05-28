Title: This Week in Rust 653
Number: 653
Date: 2026-05-27
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

* [Scientific Computing in Rust #18 (May 2026)](https://scientificcomputing.rs/monthly/2026-05)

### Project/Tooling Updates

* [gitoxide - May 26](https://github.com/GitoxideLabs/gitoxide/discussions/2621)
* [hyper User Survey 2025 Results](https://seanmonstar.com/blog/hyper-user-survey-2025-results/)
* [Rust Update: gRPC Welcomes Tonic!](https://grpc.io/blog/grpc-welcomes-tonic/)
* [serde-const-default v0.1: Removes boilerplate when using const values as field defaults](https://github.com/ifsheldon/serde-const-default/releases/tag/v0.1)
* [BoquilaHUB 0.5: AIs for Nature. Now it includes SOTA AI bioacoustics models and embeddings models](https://github.com/boquila/boquilahub/releases/tag/v0.5)
* [splog: a log viewer TUI with automatic tag categorization](https://www.sextianbytes.fr/blog/imperfect-by-design/)
* [rgx v0.12.3 — Building a regex debugger for the terminal in Rust](https://dev.to/brevity1swos/building-a-regex-debugger-for-the-terminal-in-rust-977)
* [UI tests are the guardrails an AI needs: the story of clipboardwire](https://davefx.com/en/2026/05/clipboardwire-construction-story/)
* [slintcn 0.22: shadcn/ui-style copy-paste components for Slint native apps](https://github.com/stevekwon211/slintcn/blob/main/docs/INTRODUCING_SLINTCN.md)
* [Releasing dtact v0.2.2 and rssn-advanced v0.1.0: the next generation async concurrent engine and scientific computing engine](https://users.rust-lang.org/t/releasing-dtact-v0-2-2-and-rssn-advanced-v0-1-0/140278)

### Observations/Thoughts

* [Noroboto: Lying Fonts and Mitigation in Rust](https://tritium.legal/blog/noroboto)
* [Erasing Existentials](https://wolfgirl.dev/blog/2026-05-20-erasing-existentials/)
* [libwce: the entropy layer of a wavelet codec, on its own](https://yogthos.net/posts/2026-05-24-libwce.html)
* [Tech Notes: Theseus: translating win32 to wasm](https://neugierig.org/software/blog/2026/05/theseus-wasm.html)
* [audio] [Rust for Linux Live with Alice Ryhl and Greg Kroah-Hartman](https://corrode.dev/podcast/s06e04-rust4linux/)
* [video] [Can a QR code be made of stars?](https://www.youtube.com/watch?v=RbmkNSqMvZY)
* [Bevy Game Engine Explained Visually](https://aibodh.com/posts/bevy-game-engine/)

* [The reflex of deriving `serde` traits](https://verrchu.github.io/blog/3-the-reflex-of-deriving-serde-traits/)

* [audio] [Netstack.FM episode 38 — Building and testing network stacks with Rama](https://netstack.fm/#episode-38)

* [Physical AI Needs a Typed World Model, Not a Vector DB](https://aimdb.dev/blog/typed-world-model)

* [Keep calm and use (Rust) monorepos](https://kerkour.com/rust-monorepos)

### Rust Walkthroughs

* [Rust Patterns & Engineering How-Tos](https://microsoft.github.io/RustTraining/rust-patterns-book/)
* [Laissez-Faire Errors](https://hemomorphic.alexblood.net/posts/laissez-faire-errors/)
* [Learn Rust HashMap and Iterators by Building a Git Object Store Reader](https://blog.sheerluck.dev/posts/learn-hashmap-iterators-by-building-a-git-object-store-reader/)
* [Learn the Basics of Bevy by Building and Deploying Pong to Itch.io](https://blog.sheerluck.dev/posts/learn-the-basics-of-bevy-by-building-and-deploying-pong-to-itch-io/)
* [The Slowdown That Doesn't Show Up in Profiles](https://cong-or.xyz/false-sharing-cache-lines.html)
* [Video] [Nine Ways to do Inheritance in Rust, a Language without Inheritance](https://www.youtube.com/watch?v=3IyKC5EtNkM)
* [Building an AsyncIO executor for the 3DS](https://blog.cat-girl.gay/3ds-async-part-one/)

### Research

### Miscellaneous

* [Content-addressed Rust builds (or, what kache actually caches)](https://kunobi.ninja/blog/what-kache-actually-caches)

## Crate of the Week

This week's crate is [inline\_tweak](https://docs.rs/inline_tweak), a crate to embed tweakable constants inside your Rust application without full recompilation.

Thanks to [Kill The Mule](https://users.rust-lang.org/t/crate-of-the-week/2704/1607) for the suggestion!

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

* [rust cookbook - Expand Command Line section with clap derive, subcommands, and env vars](https://github.com/rust-lang-nursery/rust-cookbook/issues/760)
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

352 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-05-19..2026-05-26

#### Compiler
* [`rustc_on_unimplemented`: introduce format specifiers](https://github.com/rust-lang/rust/pull/156161)
* [account for proc macro spans in `do_not_recommend` diagnostics](https://github.com/rust-lang/rust/pull/156763)
* [implement fast path for `derive(PartialOrd)` when deriving `Ord`](https://github.com/rust-lang/rust/pull/155598)
* [make bitset `would_modify_words` more vectorzer-friendly](https://github.com/rust-lang/rust/pull/153640)
* [parse `mut` restrictions](https://github.com/rust-lang/rust/pull/156824)
* [stop needing materialized places for most intrinsics](https://github.com/rust-lang/rust/pull/156116)

#### Library
* [add unstable Share trait](https://github.com/rust-lang/rust/pull/156828)
* [stabilize `bool_to_result`](https://github.com/rust-lang/rust/pull/156594)
* [use strongly typed wrapped indices in `VecDeque`](https://github.com/rust-lang/rust/pull/152112)

#### Cargo
* [compiler: forward verbose flag to rustc for local crates](https://github.com/rust-lang/cargo/pull/17006)
* [don't use the network for a publish dry-run test](https://github.com/rust-lang/cargo/pull/17027)
* [break out `RegistryConfig` and `crate_url` for interpreting `RegistryConfig::dl`](https://github.com/rust-lang/cargo/pull/17011)
* [fix CVE-2026-5222 and CVE-2026-5223](https://github.com/rust-lang/cargo/pull/17031)
* [artifact: remove compat mode from artifacts](https://github.com/rust-lang/cargo/pull/17016)

#### Rustdoc
* [stabilize `--remap-path-prefix` in rustdoc](https://github.com/rust-lang/rust/pull/155307)

#### Clippy
* [`useless_format`: fire on wrapped in a block-producing macro](https://github.com/rust-lang/rust-clippy/pull/17060)
* [`return` can be removed from the last stmt of a block if it has an expr](https://github.com/rust-lang/rust-clippy/pull/16959)
* [add check for midpoint using multiplication by `0.5` and `>> 1`](https://github.com/rust-lang/rust-clippy/pull/17025)
* [avoid unnecessary `String` allocations in `MinifyingSugg` arithmetic ops](https://github.com/rust-lang/rust-clippy/pull/17057)
* [extend `clippy::missing_safety_doc` to unsafe fields](https://github.com/rust-lang/rust-clippy/pull/16767)
* [fix `manual_range_contains` NAN handling](https://github.com/rust-lang/rust-clippy/pull/17065)
* [fix error message for `useless_borrows_in_formatting` for mutable borrows](https://github.com/rust-lang/rust-clippy/pull/17036)
* [move `unnecessary_get_then_check` to `complexity`](https://github.com/rust-lang/rust-clippy/pull/16998)
* [simplify `is_some() && …unwrap()` to `is_some_and` in `unit_arg`](https://github.com/rust-lang/rust-clippy/pull/17055)

#### Rust-Analyzer
* [`diagnostics: mut_ref` binding feature diagnostic](https://github.com/rust-lang/rust-analyzer/pull/22406)
* [`assists/add_reference_here: _modify_` the reference type when dealing with `&T->&mut T`](https://github.com/rust-lang/rust-analyzer/pull/22342)
* [`cfg`: correct separator index in CfgDiff disable loop](https://github.com/rust-lang/rust-analyzer/pull/22426)
* [`hir-ty`: saturate float-to-uint cast in const eval](https://github.com/rust-lang/rust-analyzer/pull/22430)
* [`test-utils`: drain `inactive_regions` by `inactive_line_region`](https://github.com/rust-lang/rust-analyzer/pull/22427)
* [add diagnostic for E0033](https://github.com/rust-lang/rust-analyzer/pull/22411)
* [add diagnostic for E0608](https://github.com/rust-lang/rust-analyzer/pull/22404)
* [completions imports exclude supports sub items](https://github.com/rust-lang/rust-analyzer/pull/22416)
* [filter package-scoped features](https://github.com/rust-lang/rust-analyzer/pull/22432)
* [`extract_module` missing import for macro calls](https://github.com/rust-lang/rust-analyzer/pull/22437)
* [add `type_match` score for `struct_pat`](https://github.com/rust-lang/rust-analyzer/pull/22452)
* [allow wildcard params in foreign fn declarations](https://github.com/rust-lang/rust-analyzer/pull/22415)
* [analysis expected ty in `enum` variant](https://github.com/rust-lang/rust-analyzer/pull/22449)
* [autoimport `enum` variants](https://github.com/rust-lang/rust-analyzer/pull/22385)
* [do not autoref in method probe in path mode](https://github.com/rust-lang/rust-analyzer/pull/22392)
* [do not complete semicolon in match-expr place](https://github.com/rust-lang/rust-analyzer/pull/22408)
* [do not consider the path of the macro in a macro call to be inside a macro call](https://github.com/rust-lang/rust-analyzer/pull/22397)
* [emit diagnostic for rest array patterns without fixed-length arrays](https://github.com/rust-lang/rust-analyzer/pull/22424)
* [fix `SyntaxContext::root`s technically overlapping valid interneds](https://github.com/rust-lang/rust-analyzer/pull/21566)
* [flip `coerce_never type_mismatch` tys](https://github.com/rust-lang/rust-analyzer/pull/22451)
* [have a specific error for unimplemented builtin macros](https://github.com/rust-lang/rust-analyzer/pull/22383)
* [no suggest ref match when expected generic ref](https://github.com/rust-lang/rust-analyzer/pull/22409)
* [no use sad pattern on happy arm with guard](https://github.com/rust-lang/rust-analyzer/pull/22369)
* [normalize expected tuple `struct` pat field](https://github.com/rust-lang/rust-analyzer/pull/22425)
* [refactor handling of generic params in `hir::Type`](https://github.com/rust-lang/rust-analyzer/pull/22252)
* [support named consts in range pattern types](https://github.com/rust-lang/rust-analyzer/pull/22396)
* [use grouped annotation for `add_label_to_loop`](https://github.com/rust-lang/rust-analyzer/pull/22419)
* [provide better incrementality for modules](https://github.com/rust-lang/rust-analyzer/pull/22322)

### Rust Compiler Performance Triage

This week was largely positive, with most of the improvements coming from algorithm change in visibility checking: [#156228](https://github.com/rust-lang/rust/pull/156228).

Triage done by **@panstromek**.
Revision range: [281c97c3..783eb8c8](https://perf.rust-lang.org/?start=281c97c3240a9abd984ca0c6a2cd7389115e80d5&end=783eb8c8682ddde0807c60ed8293670ef523794f&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.4%  | [0.1%, 0.7%]   | 5     |
| Regressions ❌ <br /> (secondary)  | 0.5%  | [0.1%, 1.1%]   | 16    |
| Improvements ✅ <br /> (primary)   | -0.9% | [-6.6%, -0.1%] | 164   |
| Improvements ✅ <br /> (secondary) | -0.4% | [-1.3%, -0.1%] | 51    |
| All ❌✅ (primary)                 | -0.9% | [-6.6%, 0.7%]  | 169   |


2 Regressions, 2 Improvements, 5 Mixed; 2 of them in rollups
34 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/4e9e90ee6ec008cadd1f351541185eff56319998/triage/2026/2026-05-25.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Propose the concept of a crates.io username for identity](https://github.com/rust-lang/rfcs/pull/3946)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Promotes 5 Thumb-mode bare-metal Arm targets to Tier 2](https://github.com/rust-lang/compiler-team/issues/985)
* [Add -Z dead-fn-elimination to skip codegen of BFS-unreachable functions](https://github.com/rust-lang/compiler-team/issues/976)

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Update `transmute_copy` to ub_checks and `?Sized`](https://github.com/rust-lang/rust/pull/155989)
* [Tracking Issue for NEON dot product intrinsics](https://github.com/rust-lang/rust/issues/117224)
* [Never break between empty parens](https://github.com/rust-lang/rust/issues/152761)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Avoid linting `unreachable_code` on `todo!()`](https://github.com/rust-lang/rfcs/pull/3928)

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [What are the values of a union type? (in particular, what is the validity invariant of a union)](https://github.com/rust-lang/unsafe-code-guidelines/issues/438)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2026-05-27 - 2026-06-24 🦀

### Virtual
* 2026-05-27 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/9v7hv2g1)
* 2026-06-02 | Virtual | [libp2p Events](https://luma.com/libp2p)
    * [**rust-libp2p Open Maintainers Call**](https://luma.com/ukfh0mcf)
* 2026-06-02 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**‎שיחה חופשית ווירטואלית על ראסט**](https://www.meetup.com/rust-tlv/events/314871990/)
* 2026-06-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/314691782/)
* 2026-06-04 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455930/)
* 2026-06-04 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345241/)
* 2026-06-04 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/)
    * [**Exploring FalkorDB - Learning to use a Graph Database in Rust**](https://www.meetup.com/code-mavens/events/314979560/) 
* 2026-06-06 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-06-07 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/314095285/)
* 2026-06-09 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254780/)
* 2026-06-10 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/3bcnx1jb)
* 2026-06-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/rdhhptyjcjbvb/)
* 2026-06-17 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314000478/)
* 2026-06-17 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/ekws5nr4)
* 2026-06-18 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**June, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314236370/)
* 2026-06-18 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455931/)
* 2026-06-21 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/314329044/)
* 2026-06-23 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254779/)
* 2026-06-23 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: What the heck are monads - and how do we fake them in Rust**](https://www.meetup.com/women-in-rust/events/313767883/)

### Asia
* 2026-06-02 | Beijing, CN | [Voice AI and Rust Meetup (Rust for AI, lowcoderust.com)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**AI Agents and Open Source LLM (Call for Speakers)**](https://www.meetup.com/wasm-rust-meetup/events/314750465/)

### Europe
* 2026-05-28 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #68**](https://www.meetup.com/copenhagen-rust-community/events/314868448/)
* 2026-05-28 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group)
    * [**LDN Talks May Community Showcase**](https://www.meetup.com/rust-london-user-group/events/314846861/)
* 2026-05-29 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/314396588/)
* 2026-05-30 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #26**](https://www.meetup.com/stockholm-rust/events/314926826/)
* 2026-06-02 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**gRPC with Rust and Tonic**](https://www.meetup.com/rust-rhein-main/events/314051727/)
* 2026-06-03 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Join us live and INPERSON for Rust 261**](https://www.meetup.com/rust-dublin/events/314689875/)
* 2026-06-03 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 06 2026**](https://luma.com/4bmlc7qd)
* 2026-06-10 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 2 - Hacking Evening**](https://www.meetup.com/rust-munich/events/313791798/)
* 2026-06-11 | Switzerland, CH | [PostTenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-06-12 - 2026-06-14 | Kraków, PL | [Rustmeet](https://rustmeet.eu/)
    * [**Rustmeet**](https://rustmeet.eu/)
* 2026-06-16 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Interactive: Everything is Open Source**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813937/)
* 2026-06-16 | Milano, IT | [Rust Language Milan](https://www.meetup.com/rust-language-milano)
    * [**Real-time planning in Rust: SolverForge & SERIO**](https://www.meetup.com/rust-language-milan/events/314766950/)
* 2026-06-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Talk Night at Danske Commodities**](https://www.meetup.com/rust-aarhus/events/314965238/)

### North America
* 2026-05-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/314209662/)
* 2026-05-28 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539319/)
* 2026-05-28 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust in Embedded & Autonomous Systems at Parallel Systems in DTLA**](https://www.meetup.com/rust-los-angeles/events/314218564/)
* 2026-05-28 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/314716463/)
* 2026-05-30 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Central Cambridge Rust Lunch, May 30**](https://www.meetup.com/bostonrust/events/314480537/)
* 2026-06-04 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Testing, Coverage, Tracey & Mutations**](https://www.meetup.com/stl-rust/events/314106244/)
* 2026-06-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Boston Common Rust Lunch, June 6**](https://www.meetup.com/bostonrust/events/314480539/)
* 2026-06-11 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Utah Rust June Meetup**](https://www.meetup.com/utah-rust/events/314696643/)
* 2026-06-11 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/314825006/)
* 2026-06-11 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust June Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/313721899/)
* 2026-06-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcjbvb/)
* 2026-06-17 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314000478/)
* 2026-06-18 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**June, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314236370/)
* 2026-06-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyjcjbgc/)
* 2026-06-24 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust-Based Constraint Solvers in 2D Sketching with Zoo Technologies**](https://www.meetup.com/rust-los-angeles/events/314386080/)

### South America
* 2026-06-18 | Florianópolis, BR | [Rust SC](https://luma.com/rust-sc)
    * [**Rust Floripa**](https://luma.com/acinctdf)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> This overflows the trait solver today as well as my brain

– [Nadrieril on their blog](https://nadrieril.github.io/blog/2026/05/14/when-can-traits-depend-on-themselves.html)

Thanks to [Theemathas](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1774) for the suggestion!

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
