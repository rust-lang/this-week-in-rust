Title: This Week in Rust 643
Number: 643
Date: 2026-03-18
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

* [Announcing rustup 1.29.0 | Rust Blog](https://blog.rust-lang.org/2026/03/12/Rustup-1.29.0/)

* [Call for Testing: Build Dir Layout v2 | Rust Blog](https://blog.rust-lang.org/2026/03/13/call-for-testing-build-dir-layout-v2/)

### Foundation

### Newsletters

* [The Embedded Rustacean Issue #67](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-67)
* [This Month in Rust OSDev: February 2026](https://rust-osdev.com/this-month/2026-02/)

### Project/Tooling Updates

* [loadgen-rs - h2load-compatible HTTP benchmark client written in Rust, supporting HTTP/1.1, HTTP/2, and HTTP/3 (QUIC)](https://blog.none.at/blog/2026/2026-03-01-loadgen-rs/)
* [Introducing pgtui, a Postgres TUI client](https://kdwarn.net/programming/blog/227)
* [Avian Physics 0.6](https://joonaa.dev/blog/12/avian-0-6)
* [Vite 8.0 is out!](https://vite.dev/blog/announcing-vite8)
* [Building Rust Procedural Macros Without quote!: Introducing zyn](https://aacebo.hashnode.dev/building-rust-procedural-macros-without-quote-introducing-zyn)
* [bnum v0.14.0: a lot of big improvements!](https://github.com/isaacholt100/bnum/releases/tag/v0.14.0)
* [ClawShell: Secure the OpenClaw using OS-level primitives](https://runta.com/blog/introducing-clawshell/)

### Observations/Thoughts

* [Summary - Rust Project Perspectives on AI](https://nikomatsakis.github.io/rust-project-perspectives-on-ai/feb27-summary.html)
* [How to use storytelling to fit inline assembly into Rust](https://www.ralfj.de/blog/2026/03/13/inline-asm.html)
* [Why WebAssembly components](https://blog.yoshuawuyts.com/why-webassembly-components/)
* [yes, all longest regex matches in linear time is possible](https://iev.ee/blog/all-longest-regex-matches-in-linear-time/)
* [Accessing Hardware in Rust](https://ferrous-systems.com/blog/hardware-access-rust/)
* [audio] [Netstack.FM episode 31 — Protocol Shorts: MITM Proxies and Transparent L4 Interception](https://netstack.fm/#episode-31)
* [video] [Rust-powered SpacetimeDB is 1000x Faster? Founder Explains](https://www.youtube.com/watch?v=qfKBv3A0CVs)

### Rust Walkthroughs

* [Patching LMDB: How We Made Meilisearch’s Vector Store 333% Faster](https://blog.kerollmops.com/patching-lmdb-how-we-made-meilisearch-s-vector-store-333-faster)
* [Creating a DAW in Rust - Playing Audio](https://whoisryosuke.com/blog/2026/creating-a-daw-in-rust/)
* [How to Check Code Coverage in Rust](https://barretts.club/posts/how-to-test-code-coverage-rust-2026/)
* [video] [RustCurious lesson 4: Structs and Resources – Copy vs Clone vs Move](https://www.youtube.com/watch?v=r-Ag_21CKBI)

### Research

### Miscellaneous
* [Free TokioConf tickets for contributors and open source maintainers](https://tokio.rs/blog/2026-03-12-tokioconf-oss-tickets)

## Crate of the Week

This week's crate is [grab](https://github.com/anwitars/grab), a command-line tool to quickly convert CSV to JSON.

Thanks to [Gábor Maksa](https://users.rust-lang.org/t/crate-of-the-week/2704/1565) for the self-suggestion!

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

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

427 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-03-10..2026-03-17

#### Compiler
* [provide better suggestions for inference errors on `.collect()?`](https://github.com/rust-lang/rust/pull/153925)

#### Library
* [add `From` impls for wrapper types](https://github.com/rust-lang/rust/pull/146013)
* [in `Option::get_or_insert_with()`, forget the `None` instead of dropping it](https://github.com/rust-lang/rust/pull/148562)
* [fixed `VecDeque::splice()` not filling the buffer correctly when resizing the buffer on start = end range](https://github.com/rust-lang/rust/pull/152258)

#### Cargo
* [`CARGO_TARGET_DIR` doesn't have to be relative](https://github.com/rust-lang/cargo/pull/16735)
* [`shell`: Support OSC 9;4 progress on ptyxis](https://github.com/rust-lang/cargo/pull/16730)
* [`compile`: Stop on denying warnings without --keep-going](https://github.com/rust-lang/cargo/pull/16725)
* [avoid panic for package specs with an empty fragment](https://github.com/rust-lang/cargo/pull/16754)
* [util: exclude from iCloud Drive sync on macOS](https://github.com/rust-lang/cargo/pull/16728)

#### Rustdoc
* [`rustdoc-json`: Add optional support for rkyv (de)serialization](https://github.com/rust-lang/rust/pull/153283)

#### Clippy
* [fix `match_same_arms` false positive with associated consts](https://github.com/rust-lang/rust-clippy/pull/16701)
* [fix: `question_mark` suggestion caused error](https://github.com/rust-lang/rust-clippy/pull/16656)
* [refactor implementation of `unnecessary_{option,result}_map_or_else`](https://github.com/rust-lang/rust-clippy/pull/15889)

#### Rust-Analyzer
* [don't trigger GC on slow tests](https://github.com/rust-lang/rust-analyzer/pull/21827)
* [SCIP generation should prime caches in parallel](https://github.com/rust-lang/rust-analyzer/pull/21828)
* [add naming convention validation for `union` types](https://github.com/rust-lang/rust-analyzer/pull/21794)
* [handle multi-byte UTF-8 identifiers in `NameGenerator::suggest_name`](https://github.com/rust-lang/rust-analyzer/pull/21793)
* [infer generic args for trait ref and its assoc type](https://github.com/rust-lang/rust-analyzer/pull/21820)
* [remove angle brackets if all lifetime args removed in inline type alias code assist](https://github.com/rust-lang/rust-analyzer/pull/21784)
* [replace make usage with SyntaxFactory in few ide-assists utils methods](https://github.com/rust-lang/rust-analyzer/pull/21826)

### Rust Compiler Performance Triage

Another fairly quiet week, with few changes and overall neutral performance.

Triage done by **@simulacrum**.
Revision range: [3945997a..5b61449e](https://perf.rust-lang.org/?start=3945997aabf6165261ef3419534c1ad59d9dc5c6&end=5b61449ed85a670f1dd3fca6a8c759ee0b451b66&absolute=false&stat=instructions%3Au)

1 Regression, 1 Improvement, 2 Mixed; 3 of them in rollups
35 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2026/2026-03-16.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Improvements to match formatting](https://github.com/rust-lang/rust/issues/152763)
* [Fix SGX delayed host lookup via ToSocketAddr](https://github.com/rust-lang/rust/pull/152851)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Add `homogeneous_try_blocks` RFC](https://github.com/rust-lang/rfcs/pull/3721)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [allow `incomplete_features` in UI tests](https://github.com/rust-lang/compiler-team/issues/974)
* [Add `-Zsanitizer=kernel-hwaddress`](https://github.com/rust-lang/compiler-team/issues/975)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [[type layout] usize and isize have the same size and alignment](https://github.com/rust-lang/reference/pull/2200)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Discuss travel grants 2026 projections](https://github.com/rust-lang/leadership-council/issues/276)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Crate deletion allowances](https://github.com/rust-lang/rfcs/pull/3927)
* [Avoid linting `unreachable_code` on `todo!()`](https://github.com/rust-lang/rfcs/pull/3928)
* [Propose the Rust Foundation Maintainer fund](https://github.com/rust-lang/rfcs/pull/3931)

## Upcoming Events

Rusty Events between 2026-03-18 - 2026-04-15 🦀

### Virtual
* 2026-03-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Embedded Rust**](https://www.meetup.com/vancouver-rust/events/313471716/)
* 2026-03-18 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Hybrid event with Rust Dortmund!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/313621933/)
* 2026-03-18 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/45qqc2eo)
* 2026-03-19 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**March, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274882/)
* 2026-03-20 | Virtual | [Packt Publishing Limited](https://www.eventbrite.com/o/70306584013)
    * [**Rust Adoption, Safety, and Cloud with Francesco Ciulla**](https://www.eventbrite.com/e/rust-adoption-safety-and-cloud-with-francesco-ciulla-registration-1981847709850)
* 2026-03-24 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254785/)
* 2026-03-24 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Crates, Tips & Tricks Lightning Talks - Bring your ideas!**](https://www.meetup.com/women-in-rust/events/312799496/)
* 2026-03-25 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 03 2026**](https://luma.com/vq9w8q0w)
* 2026-03-26 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455925/)
* 2026-04-01 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/me4jwgxu)
* 2026-04-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/313656388/)
* 2026-04-02 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345237/)
* 2026-04-04 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-04-09 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455926/)
* 2026-04-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254784/)
* 2026-04-14 | Virtual (London, GB) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/313506013/)
* 2026-04-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Nushell**](https://www.meetup.com/vancouver-rust/events/313471712/)

### Asia
* 2026-03-19 | Seoul, KR | [Seoul Rust (Programming Language) Meetup](https://www.meetup.com/rust-seoul-meetup/events/)
    * [**Seoul Rust Meetup**](https://www.meetup.com/rust-seoul-meetup/events/313764176/)
* 2026-03-22 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust March 2026 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/312862609/)
* 2026-03-28 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/events/)
    * [**Rust Delhi Meetup #13**](https://www.meetup.com/rustdelhi/events/313777790/)

### Europe
* 2026-03-18 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - Intro to Embedded Rust - March**](https://www.meetup.com/rust-dortmund/events/313338784/)
* 2026-03-19 - 2026-03-20 | Warsaw, PL | [Rustikon](https://www.rustikon.dev/)
    * [**Rustikon Conference**](https://www.rustikon.dev/)
* 2026-03-23 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #18**: Ludwig Weinzierl - Bevy: Spielentwicklung mit Rust](https://rust-augsburg.github.io/meetup/Meetup_18.html)
* 2026-03-23 | Amsterdam, NL | [Open Source SecurityCon](https://events.linuxfoundation.org/kubecon-cloudnativecon-europe/co-located-events/open-source-securitycon/)
    * [**Open Source SecurityCon EU 2026**](https://rustfoundation.org/event/open-source-securitycon-eu-2026/)
* 2026-03-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night - Advent of Code**](https://www.meetup.com/rust-aarhus/events/313284304/)
* 2026-03-24 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester March Code Night**](https://www.meetup.com/rust-manchester/events/313495449/)
* 2026-03-24 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim)
    * [**Rust projects - show and tell in March**](https://www.meetup.com/rust-trondheim/events/313537618/)
* 2026-03-25 | Dresden, DE | [Rust Dresden](https://github.com/rust-dresden)
    * [**First Meetup**](https://github.com/rust-dresden/rust-dresden/discussions/7)
* 2026-03-26 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #84**](https://www.meetup.com/rust-paris/events/313646981/)
* 2026-03-27 | Paris, FR | [Rust in Paris](https://www.rustinparis.com/)
    * [**Rust in Paris**](https://www.rustinparis.com/)
* 2026-03-28 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #25**](https://www.meetup.com/stockholm-rust/events/313749232/)
* 2026-04-01 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/313783250/)
* 2026-04-01 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Rust/ACCU meetup.**](https://www.meetup.com/oxford-rust-meetup-group/events/312664491/)
* 2026-04-02 | London, GB | [Rust London User Group](https://www.meetup.com/rust-london-user-group/events/)
    * [**LDN Talks Spring Community Showcase**](https://www.meetup.com/rust-london-user-group/events/313816694/)
* 2026-04-07 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #15 @ letsboot**](https://www.meetup.com/rust-basel/events/313765547/)
* 2026-04-09 | Geneva, CH | [Rust Meetup Geneva](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-04-09 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust talks @ AutoStore – "Patterns for Event Driven Systems" and "Rust + WASM"**](https://www.meetup.com/rust-oslo/events/313806765/)

### North America
* 2026-03-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Embedded Rust**](https://www.meetup.com/vancouver-rust/events/313471716/)
* 2026-03-19 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**March, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274882/)
* 2026-03-19 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/313569258/)
* 2026-03-19 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Applied Rust - Building Rust Applictions**](https://www.meetup.com/music-city-rust-developers/events/313576317/)
* 2026-03-19 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Social Interoperability - Rust, C++ and The Greater Good**](https://www.meetup.com/rust-nyc/events/313639707/)
* 2026-03-21 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Porter Square Rust Lunch, Mar 21**](https://www.meetup.com/bostonrust/events/313208597/)
* 2026-03-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/313653030/)
* 2026-03-25 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC's Digital Asset Adoption Special**](https://www.meetup.com/rust-nyc/events/313713085/)
* 2026-03-26 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/311228658/)
* 2026-04-02 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**SIUE Cruft Crawler with LLM**](https://www.meetup.com/stl-rust/events/313482094/)
* 2026-04-09 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust/events/)
    * [**San Diego Rust April Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/313721879/)
* 2026-04-14 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Sharpening Your Rust Skills for Job Interviews**](https://www.meetup.com/charlottesville-rust-meetup/events/313262215/)

### Oceania
* 2026-03-26 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**TBD March Meetup**](https://www.meetup.com/rust-melbourne/events/313471749/)

### South America
* 2026-03-21 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP (migrado pro Lumma)**](https://www.meetup.com/rust-sao-paulo-meetup/events/313446400/)
* 2026-04-11 | Argentina, AR | [Oxidar Org](https://luma.com/user/oxidar)
    * [**Oxidar.org Hackaton - Snakear - ¡Veni a hackear con Rust!**](https://luma.com/5f51ey45)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> What we collectively build, beyond the code artifacts that the compiler+tools are, is a group of people who come back, who learn, who share their understanding, who align their tastes, who take input from the community, etc etc. Merging an LLM-generated PR feeds only the “we have code that works” part of the Project; it’s not participating in all the other feedback cycles that make the project alive.

– [Nadrieril on the Rust Project Perspectives on AI](https://nikomatsakis.github.io/rust-project-perspectives-on-ai/feb27-summary.html#codebases-are-more-than-code)

Despite another week without a suggestion, llogiq is pleased with his choice.

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
