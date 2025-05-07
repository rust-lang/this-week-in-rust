Title: This Week in Rust 597
Number: 597
Date: 2025-04-30
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

### Newsletters
* [The Embedded Rustacean Issue #44](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-44)

### Project/Tooling Updates
* [Bevy 0.16](https://bevyengine.org/news/bevy-0-16/)
* [Yelken Alpha Release Announcement](https://bwqr.github.io/yelken-blog/first-announcement/)
* [BugStalker v0.3.0 - Modern debugger for Linux x86-64. Written in Rust for Rust programs.](https://www.reddit.com/r/rust/comments/1k6vni5/bugstalker_v030_released_async_debugging_new/)

### Observations/Thoughts
* [Audit of the Rust p256 Crate](https://reports.zksecurity.xyz/reports/near-p256/)
* [We have polymorphism at home 🦀!](https://medium.com/@alighahremani1377/we-have-polymorphism-at-home-d9f21f5565bf)
* [Migrating away from Rust](https://deadmoney.gg/news/articles/migrating-away-from-rust)
* [Syntactic musings on match expressions](https://blog.yoshuawuyts.com/syntactic-musings-on-match-expressions/)

### Rust Walkthroughs
* [Freeing Up Gigabytes: Reclaiming Disk Space from Rust Cargo Builds](https://thisdavej.com/freeing-up-gigabytes-reclaiming-disk-space-from-rust-cargo-builds/)
* [A Visual Journey Through Async Rust](https://github.com/alexpusch/rust-magic-patterns/blob/master/visual-journey-through-async-rust/Readme.md)
* [Generating 1 Million PDFs in 10 Minutes](https://www.ersteiger.com/posts/rendering-one-million-pdfs/)
* [Video] [Shipping Rust to Python, Typescript, and Ruby](https://www.youtube.com/watch?v=Zs6Uer3VAyQ)
* [Video] [From Rust to C and Back Again: an introduction to "foreign functions"](https://www.youtube.com/watch?v=B4yNqR0WgYQ) ([Long Version](https://www.youtube.com/watch?v=LLAUzghhNHg))
* [Video] [Internals of SlateDB: An embedded key-value store built on object storage](https://www.youtube.com/watch?v=qqF_zFWqFYk)
* [Video] [RefinedRust - High-Assurance Verification of Rust Programs](https://www.youtube.com/watch?v=XR8p9R1cPC4)

### Miscellaneous
* [The Company That Is All-In On Rust For Robotics](https://filtra.io/rust/interviews/matic-apr-25)
* [How Rolldown Works: Module Loading, Dependency Graphs, and Optimization Explained](https://www.atriiy.dev/blog/rolldown-module-loader-and-dependency-graph)
* [Video] [Why Learning Rust Could Change Your Career](https://www.youtube.com/watch?v=_7va24sawyg)

## Crate of the Week

This week's crate is [rust-sel4](https://github.com/seL4/rust-sel4/), a no\_std crate to bind to the Se4L microkernel APIs.

Thanks to [Robbie VanVossen](https://users.rust-lang.org/t/crate-of-the-week/2704/1432) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No calls for testing were issued this week by [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

* [rama - add a SubdomainTrieMatcher](https://github.com/plabayo/rama/issues/534)
* [rama - add ffi/rama-rhai: support ability to use services and layers written in rhai](https://github.com/plabayo/rama/issues/533)
* [rama - support (TLS) peetprint in rama-net fingerprinting](https://github.com/plabayo/rama/issues/518)
* [rama - support akamai h2 passive fingerprint and expose in echo + fp services](https://github.com/plabayo/rama/issues/517)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->
*No Calls for papers or presentations were submitted this week.* 

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

389 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-04-22..2025-04-29

#### Compiler

* [`rc""` more clear error message](https://github.com/rust-lang/rust/pull/140175)
* [allow deref patterns to move out of boxes](https://github.com/rust-lang/rust/pull/140022)
* [async drop codegen](https://github.com/rust-lang/rust/pull/123948)
* [avoid re-interning in `LateContext::get_def_path`](https://github.com/rust-lang/rust/pull/140345)
* [implement a lint for implicit autoref of raw pointer dereference - take 2](https://github.com/rust-lang/rust/pull/123239)
* [improve error message for `||` (or) in let chains](https://github.com/rust-lang/rust/pull/140272)
* [stabilize let chains in the 2024 edition](https://github.com/rust-lang/rust/pull/132833)
* [deny `unsafe_op_in_unsafe_fn` by default](https://github.com/rust-lang/compiler-builtins/pull/801)

#### Library

* [add `Arc::is_unique`](https://github.com/rust-lang/rust/pull/138939)
* [stabilise `std::ffi::c_str`](https://github.com/rust-lang/rust/pull/137439)
* [stabilize `proc_macro::Span::{start,end,line,column}`](https://github.com/rust-lang/rust/pull/139865)
* [stabilize `slice_as_chunks` library feature](https://github.com/rust-lang/rust/pull/139656)
* [transmutability: support char, NonZeroXxx](https://github.com/rust-lang/rust/pull/140215)

#### Cargo

* [implement RFC3695: Allow boolean literals as cfg predicates](https://github.com/rust-lang/cargo/pull/14649)
* [stabilize automatic garbage collection](https://github.com/rust-lang/cargo/pull/14287)
* [`feat(add/install)`: check if given crate argument would be valid with inserted @ symbol](https://github.com/rust-lang/cargo/pull/15441)

#### Rustdoc

* [correctly display stdout and stderr in case a doctest is failing](https://github.com/rust-lang/rust/pull/140291)
* [stabilize flags for doctest cross compilation](https://github.com/rust-lang/rust/pull/137096)

#### Clippy

* [`manual_div_ceil`: fix suggestions when macro is involved](https://github.com/rust-lang/rust-clippy/pull/14666)
* [consider side effects when rewriting iterator behaviors](https://github.com/rust-lang/rust-clippy/pull/14490)
* [fix `zombie_processes` false positive inside closures](https://github.com/rust-lang/rust-clippy/pull/14696)
* [fix: `equatable_if_let` suggests wrongly when involving reference](https://github.com/rust-lang/rust-clippy/pull/14504)
* [fix: `unnecessary_cast` suggests extra brackets when in macro](https://github.com/rust-lang/rust-clippy/pull/14643)
* [fix: `unused_unit` suggests wrongly on unit never type fallback](https://github.com/rust-lang/rust-clippy/pull/14609)
* [restrict the cases where `ptr_eq` triggers](https://github.com/rust-lang/rust-clippy/pull/14526)

#### Rust-Analyzer

* [add expression fill mode variant for filling with underscore expressions](https://github.com/rust-lang/rust-analyzer/pull/19704)
* [always error when failed to parse DiscoverProjectMessage](https://github.com/rust-lang/rust-analyzer/pull/19684)
* [arena allocate `LifetimeRef`s](https://github.com/rust-lang/rust-analyzer/pull/19678)
* [base-db: add more details to panic](https://github.com/rust-lang/rust-analyzer/pull/19710)
* [add two new diagnostics: one for mismatch in generic arguments count, and another for mismatch in their kind](https://github.com/rust-lang/rust-analyzer/pull/19479)
* [adds an assist to remove underscores from used variables](https://github.com/rust-lang/rust-analyzer/pull/19692)
* [better support `offset_of!()`](https://github.com/rust-lang/rust-analyzer/pull/19657)
* [properly handle lifetimes when checking generic arguments len](https://github.com/rust-lang/rust-analyzer/pull/19676)
* [fix ide-assists `raw_string` suffix fail](https://github.com/rust-lang/rust-analyzer/pull/19622)
* [escape raw names in labels properly](https://github.com/rust-lang/rust-analyzer/pull/19699)
* [fix incorrect diagnostic for lifetime parameter count mismatch](https://github.com/rust-lang/rust-analyzer/pull/19672)
* [fix type argument mismatch incorrectly triggering on inferred trait args](https://github.com/rust-lang/rust-analyzer/pull/19675)
* [panics in inlay hints that produce empty text edits for closure return types](https://github.com/rust-lang/rust-analyzer/pull/19647)

### Rust Compiler Performance Triage

Strange week with lots of noise peeking through the performance runs. The only really significant change was a performance improvement that comes from allowing out of order encoding of the dep graph.

Triage done by **@rylev**.
Revision range: [8f2819b0..25cdf1f6](https://perf.rust-lang.org/?start=8f2819b0e3428d0aee05fa60e91e0211c2aea053&end=25cdf1f67463c9365d8d83778c933ec7480e940b&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.5%  | [0.1%, 3.0%]   | 77    |
| Regressions ❌ <br /> (secondary)  | 0.6%  | [0.1%, 2.4%]   | 77    |
| Improvements ✅ <br /> (primary)   | -0.7% | [-1.3%, -0.2%] | 106   |
| Improvements ✅ <br /> (secondary) | -0.7% | [-1.2%, -0.2%] | 29    |
| All ❌✅ (primary)                 | -0.2% | [-1.3%, 3.0%]  | 183   |


4 Regressions, 2 Improvements, 4 Mixed; 2 of them in rollups
38 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/748534344dceab1e8001a925cf84fa04a2c1c752/triage/2025-04-29.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Finalize repeat expr inference behaviour with inferred repeat counts](https://github.com/rust-lang/rust/pull/139635)
* [Partially stabilize LoongArch target features](https://github.com/rust-lang/rust/pull/135015)
* [Unify sidebar buttons to use the same image](https://github.com/rust-lang/rust/pull/140135)
* [de-stabilize bench attribute](https://github.com/rust-lang/rust/pull/134273)
* [Remove some unsized tuple impls now that we don't support unsizing tuples anymore](https://github.com/rust-lang/rust/pull/138340)

#### Other Areas

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Remove apologies about the Reference](https://github.com/rust-lang/reference/pull/1792)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: naming groups of configuration with cfg_alias](https://github.com/rust-lang/rfcs/pull/3804)

## Upcoming Events

Rusty Events between 2025-04-30 - 2025-05-28 🦀

### Virtual
* 2025-05-01 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/clzsjmn0)
* 2025-05-03 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2025-05-04 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/307556965)
* 2025-05-05 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Tauri: Cross-Platform desktop applications with Rust and web technologies**](https://www.meetup.com/rust-tlv/events/307178592/)
* 2025-05-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031663)
* 2025-05-07 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #10**](https://www.meetup.com/bevy-game-development/events/307354690)
* 2025-05-08 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/2lmcm9iq)
* 2025-05-08 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820300)
* 2025-05-11 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/kvqfrtyhchbpb)
* 2025-05-13 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305020415)
* 2025-05-15 | Virtual (Joint Meetup, Europe + Israel) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/), [Rust Paris](https://www.meetup.com/de-DE/rust-paris/), [London Rust Project Group](https://www.meetup.com/de-DE/london-rust-project-group/), [Rust Zürisee](https://www.meetup.com/de-DE/rust-zurich/), [Rust TLV](https://www.meetup.com/de-DE/rust-tlv/), [Rust Nürnberg](https://www.meetup.com/de-DE/rust-noris/), [Rust Munich](https://www.meetup.com/de-DE/rust-munich/), [Rust Aarhus](https://www.meetup.com/de-de/rust-aarhus/), [lunch.rs](http://lunch.rs/)
    * [**🦀 Celebrating 10 years of Rust 1.0 🦀**](https://www.meetup.com/rust-berlin/events/307293317)
* 2025-05-15 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/eeqmobhz)
* 2025-05-18 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/kvqfrtyhchbxb)
* 2025-05-20 | Virtual (London, GB) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Threading through lifetimes of borrowing - the Rust way**](https://www.meetup.com/women-in-rust/events/307522540)
* 2025-05-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/305170826)
* 2025-05-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307184332)
* 2025-05-22 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820302)
* 2025-05-22 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/8zabmc3w)
* 2025-05-25 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/kvqfrtyhchbhc)
* 2025-05-27 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361435)

### Asia
* 2025-05-24 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**May 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/may-2025-rustacean-meetup/)

### Europe
* 2025-04-30 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**Kubernetes Operator in Rust**](https://www.meetup.com/rust-rhein-main/events/306772838)
* 2025-05-01 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Hackers Hike 0x0**](https://www.meetup.com/rust-noris/events/305522254)
* 2025-05-04 | Istanbul, TR | [Türkiye Rust Community](https://kommunity.com/turkiye-rust-community/events)
    * [**Rust Connect**](https://kommunity.com/turkiyerust/events/rust-connect-91f7b3a6)
* 2025-05-06 | Cambridge, GB | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/events/)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/307477191)
* 2025-05-06 - 2025-05-07 | Paris, FR | [WebAssembly and Rust Meetup](https://www.meetup.com/wasm-rust-meetup/)
    * [**GOSIM AI Paris 2025**](https://www.meetup.com/wasm-rust-meetup/events/306530699/)
* 2025-05-06 | Paris, FR | [WebAssembly and Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**GOSIM AI Paris 2025 (Discount available)**](https://www.meetup.com/wasm-rust-meetup/events/306530699)
* 2025-05-07 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 05 2025**](https://lu.ma/k4nscy5q)
* 2025-05-07 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust in May: FFI**](https://www.meetup.com/rustcologne/events/307548402)
* 2025-05-07 | Madrid, ES | [MadRust](https://www.meetup.com/madrust/events/)
    * [**VII Lenguajes, VII Perspectivas, I Problema**](https://www.meetup.com/madrust/events/307030185)
* 2025-05-07 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/306541571)
* 2025-05-08 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #8**](https://www.meetup.com/rust-gdansk/events/307281434)
* 2025-05-08 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group/events/)
    * [**Adopting Rust (Hosted by Lloyds bank)**](https://www.meetup.com/london-rust-project-group/events/307085179)
* 2025-05-13 - 2025-05-17 | Utrecht, NL | [Rust NL](https://rustweek.org/about)
    * [**RustWeek 2025**](https://rustweek.org)
* 2025-05-14 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305045447)
* 2025-05-15 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust 10-year anniversary @ Appear**](https://www.meetup.com/rust-oslo/events/307427014)
* 2025-05-16 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam/events/)
    * [**Rust Week Hackathon**](https://www.meetup.com/rust-nederland/events/307107584)
* 2025-05-16 | Utrecht, NL | [Rust NL Meetup Group](https://www.meetup.com/rust-nederland/)
    * [**RustWeek Hackathon**](https://www.meetup.com/rust-nederland/events/307107584/)
* 2025-05-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Hack Night - Robot Edition**](https://www.meetup.com/rust-aarhus/events/307289572)
* 2025-05-20 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/305741635)
* 2025-05-22 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #3 @zentroom**](https://www.meetup.com/rust-bern/events/307559606)
* 2025-05-22 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #77**](https://www.meetup.com/rust-paris/events/307565141)
* 2025-05-27 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #11 @ Letsboot Basel**](https://www.meetup.com/rust-basel/events/307567083)

### North America
* 2025-05-01 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**SIUE Capstone Project reflections on Rust**](https://www.meetup.com/stl-rust/events/304026152)
* 2025-05-03 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Boston Common Rust Lunch, May 3**](https://www.meetup.com/bostonrust/events/306845368)
* 2025-05-07 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/307557852)
* 2025-05-08 | México City, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Calculando con el compilador: Compiler time vs Run time. Introducción a uv**](https://www.meetup.com/rust-mx/events/307015601)
* 2025-05-08 | Portland, OR, US | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**Apache DataFusion: A Fast, Extensible, Modular Analytic Query Engine in Rust**](https://www.meetup.com/pdxrust/events/307288436)
* 2025-05-11 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Porter Square Rust Lunch, May 11**](https://www.meetup.com/bostonrust/events/306845728)
* 2025-05-15 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Using Rust For Web Series 2 : Why you, Yes You. Should use Hyperscript!**](https://www.meetup.com/music-city-rust-developers/events/304333101)
* 2025-05-15 | Redmond, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**May, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658468)
* 2025-05-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/307337307)
* 2025-05-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhchblc)

### South America
* 2025-05-28 | Montevideo, DE, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay/events/)
    * [**Primera meetup de Rust en 2025!**](https://www.meetup.com/rust-uruguay/events/307341567)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1jttzz4/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> With Bevy clearly being an extended test suite for Rust's trait solver, how did you get the idea to also turn it into a game engine?

> Every sufficiently advanced test is indistinguishable from a game engine 🙂

– [/u/0x564A00 and /u/_cart on /r/rust](https://www.reddit.com/r/rust/comments/1k721w1/comment/moumd91)

Thanks to [Ludwig Stecher](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1681) and [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1682) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1kbx9de/this_week_in_rust_597/)</small>
