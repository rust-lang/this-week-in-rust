Title: This Week in Rust 609
Number: 609
Date: 2025-07-23
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

### Official
* [Sunsetting the rustwasm GitHub org](https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org/)

### Foundation

### Newsletters
* [The Embedded Rustacean Issue #50](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-50)
* [Bioinformatics in Rust #1](https://dawnandrew100.github.io/seq.rs/posts/2025/july/)

### Project/Tooling Updates
* [GCC Front-End For Rust- June 2025 Monthly report](https://rust-gcc.github.io/2025/07/17/2025-06-monthly-report.html)
* [gitoxide - July 25](https://github.com/GitoxideLabs/gitoxide/discussions/2084)
* [Announcing NAPI-RS v3](https://napi.rs/blog/announce-v3)
* [Rust: Clippy performance status update](https://blog.goose.love/posts/clippy-performance-status-update/)
* [How to write Rust in the kernel: part 3](https://lwn.net/SubscriberLink/1026694/ec6db2459bc68907/)
* [State of the Art Dynamic Matrix Multiplication](https://burn.dev/blog/sota-multiplatform-matmul/)
* [Tako: A Lightweight Async Web Framework on Tokio and Hyper](https://rust-dd.com/post/tako-a-lightweight-async-web-framework-on-tokio-and-hyper)
* [stochastic-rs: High-Performance Stochastic Process Simulation in Rust](https://rust-dd.com/post/stochastic-rs-high-performance-stochastic-process-simulation-in-rust)

### Observations/Thoughts
* ["Bypassing" specialization in Rust or How I Learned to Stop Worrying and Love Function Pointers](https://oakchris1955.eu/posts/bypassing_specialization/)
* [The borrowchecker is what I like the least about Rust](https://viralinstruction.com/posts/borrowchecker/)
* [On Reifying Nested Closures in Rust](https://radekmie.dev/blog/on-reifying-nested-closures-in-rust/)
* [Supporting faster file load times with memory optimizations in Rust](https://www.figma.com/blog/supporting-faster-file-load-times-with-memory-optimizations-in-rust/)
* [Rickrolling Turso DB (SQLite rewrite in Rust)](https://avi.im/blag/2025/rickrolling-turso/)
* [Rust unit testing: simplify your tests](https://jorgeortiz.dev/posts/rust_unit_testing_simplify_tests/)

### Rust Walkthroughs
* [The Signal Protocol Explained #1: Implementing the Post-Quantum Extended Diffie-Hellman (PQXDH) protocol in Rust](https://kerkour.com/signal-protocol-pqxdh-rust)

* [Structured GCP logging in Rust using tracing](https://medium.com/@ludo.vp/structured-gcp-logging-in-rust-using-the-tracing-and-tracing-subscriber-crates-356fcb38e46e)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [index\_permute](https://crates.io/crates/index_permute), a library for in-place clone-less permutation on mutable slices.

Despite a lack of suggestions this week, llogiq is fairly pleased with his choice.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

* *No calls for testing were issued this week by [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

425 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-07-15..2025-07-22

#### Compiler
* [parse `const trait Trait`](https://github.com/rust-lang/rust/pull/143879)
* [unify `CoroutineWitness` sooner in typeck, and stall coroutine obligations based off of `TypingEnv`](https://github.com/rust-lang/rust/pull/141762)
* [miri: add support for global constructors (i.e. life before main)](https://github.com/rust-lang/miri/pull/4459)
* [miri: non-deterministically truncate reads/writes](https://github.com/rust-lang/miri/pull/4481)

#### Library
* [add experimental `backtrace-trace-only` std feature](https://github.com/rust-lang/rust/pull/143910)
* [add `uX::strict_sub_signed`](https://github.com/rust-lang/rust/pull/143282)
* [constify Try, From, TryFrom and relevant traits](https://github.com/rust-lang/rust/pull/143768)
* [constify `Index` traits](https://github.com/rust-lang/rust/pull/143921)
* [constify `Option` methods](https://github.com/rust-lang/rust/pull/143967)
* [make slice comparisons const](https://github.com/rust-lang/rust/pull/143925)
* [stabilize `const_float_round_methods`](https://github.com/rust-lang/rust/pull/143604)
* [stabilize `const_slice_reverse`](https://github.com/rust-lang/rust/pull/143382)

#### Cargo
* [expose artifact dependency getters in cargo-as-a-library](https://github.com/rust-lang/cargo/pull/15753)

#### Rustdoc
* [Make aliases search support partial matching](https://github.com/rust-lang/rust/pull/143988)

#### Clippy
* [`unsafe_derive_deserialize`: do not consider `pin!()` as `unsafe`](https://github.com/rust-lang/rust-clippy/pull/15137)
* [don't trigger `unused_trait_names` in macros](https://github.com/rust-lang/rust-clippy/pull/14947)
* [fix `empty_with_brackets` span handling](https://github.com/rust-lang/rust-clippy/pull/15311)
* [fix `filter_map_bool_then` wrongly suggests macro definitions](https://github.com/rust-lang/rust-clippy/pull/15048)
* [fix `missing_inline_in_public_items` false positive on functions with `extern`](https://github.com/rust-lang/rust-clippy/pull/15313)
* [fix `needless_range_loop` false positive on array literals](https://github.com/rust-lang/rust-clippy/pull/15314)
* [fix `never_loop` forget to remove `break` in suggestion](https://github.com/rust-lang/rust-clippy/pull/15064)
* [fix `ptr_arg` suggests changes when it's actually better not to bother](https://github.com/rust-lang/rust-clippy/pull/15105)
* [fix `ptr_as_ptr` suggests wrongly with turbo fish](https://github.com/rust-lang/rust-clippy/pull/15289)
* [fix capacity overflow in `single_match` with deref patterns](https://github.com/rust-lang/rust-clippy/pull/15124)
* [fix false positive in `useless_attribute` with `redundant_imports`](https://github.com/rust-lang/rust-clippy/pull/15318)
* [fix: ignore `pattern_type_mismatch` when external macro owns the match](https://github.com/rust-lang/rust-clippy/pull/15306)
* [`large_enum_variant`: dont suggest `Box` in `no_std` mode](https://github.com/rust-lang/rust-clippy/pull/15241)
* [propose to exchange ranges only when it is safe to do so](https://github.com/rust-lang/rust-clippy/pull/14432)
* [various improvements to the `incompatible_msrv` lint](https://github.com/rust-lang/rust-clippy/pull/14433)
* [warn about `const` instability wrt MSRV](https://github.com/rust-lang/rust-clippy/pull/15297)
* [warn about types not meeting MSRV](https://github.com/rust-lang/rust-clippy/pull/15296)

#### Rust-Analyzer
* [add AsMut to minicore `prelude::v1`](https://github.com/rust-lang/rust-analyzer/pull/20246)
* [add `AsRef` and `Borrow` for `generate_mut_trait_impl`](https://github.com/rust-lang/rust-analyzer/pull/19917)
* [add `Deref` → `DerefMut` for `generate_mut_trait_impl`](https://github.com/rust-lang/rust-analyzer/pull/20256)
* [add ide-assist, generate single field `struct` `From`](https://github.com/rust-lang/rust-analyzer/pull/19783)
* [add tailexpr `&` → `&mut` for `generate_mut_trait_impl`](https://github.com/rust-lang/rust-analyzer/pull/20247)
* [support `cfg_select!`](https://github.com/rust-lang/rust-analyzer/pull/20265)
* [ide-assist: generate Deref transitive](https://github.com/rust-lang/rust-analyzer/pull/20255)
* [apply adjusts to pats and exprs when doing pat analysis](https://github.com/rust-lang/rust-analyzer/pull/20273)
* [disable tests in flycheck if `cfg.setTest` is set to false](https://github.com/rust-lang/rust-analyzer/pull/20271)
* [fix search of raw labels and lifetimes](https://github.com/rust-lang/rust-analyzer/pull/20262)
* [infer lifetimes for GATs in expression/pattern position](https://github.com/rust-lang/rust-analyzer/pull/20238)

### Rust Compiler Performance Triage

Fairly busy week with improvements outweighing regressions. Most of the regressions were considered acceptable given the circumstances (such as landing a long awaited feature). By far the biggest win comes from being a bit smarter about hashing certain information inside of `DefPathHash`. Since hashing happens quite a lot in th compiler's query system, optimizing when hashing happens can have large performance impacts.

Triage done by **@rylev**.
Revision range: [a9fb6103..3f9f20f7](https://perf.rust-lang.org/?start=a9fb6103b05c6ad6eee6bed4c0bb5a2e8e1024c6&end=3f9f20f71dd945fe7d044e274094a53c90788269&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.4%  | [0.1%, 0.9%]   | 47    |
| Regressions ❌ <br /> (secondary)  | 0.8%  | [0.1%, 2.7%]   | 69    |
| Improvements ✅ <br /> (primary)   | -0.8% | [-4.1%, -0.2%] | 122   |
| Improvements ✅ <br /> (secondary) | -0.7% | [-2.5%, -0.0%] | 143   |
| All ❌✅ (primary)                 | -0.5% | [-4.1%, 0.9%]  | 169   |


3 Regressions, 8 Improvements, 8 Mixed; 8 of them in rollups
35 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/3c5e96856ae3c9e34f08e4b9bb3ef0fe75709db2/triage/2025/2025-07-22.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [[rustdoc] Display unsafe attrs with edition 2024 unsafe() wrappers.](https://github.com/rust-lang/rust/pull/143662)
* [disposition: close] [take into account the num of processes by ulimit](https://github.com/rust-lang/rust/pull/143614)
* [Port the proc macro attributes to the new attribute parsing infrastructure](https://github.com/rust-lang/rust/pull/143607)
* [stop specializing on `Copy`](https://github.com/rust-lang/rust/pull/135634)
* [Remove `[T]::array_chunks(_mut)`](https://github.com/rust-lang/rust/pull/143289)
* [Port #[macro_export] to the new attribute parsing infrastructure](https://github.com/rust-lang/rust/pull/143857)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
* [RFC: enable derive(From) for single-field structs](https://github.com/rust-lang/rfcs/pull/3809)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Redefine `CARGO_TARGET_DIR` to be only an artifacts directory](https://github.com/rust-lang/cargo/issues/14125)

*No Items entered Final Comment Period this week for
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC: ID_Compat_Math characters allowed in identifiers](https://github.com/rust-lang/rfcs/pull/3840)

## Upcoming Events

Rusty Events between 2025-07-23 - 2025-08-20 🦀

### Virtual
* 2025-07-16 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731031)
* 2025-07-17 | Hybrid (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**July, 2025 Computer Programming Language Panel (Special Event)**](https://www.meetup.com/seattle-rust-user-group/events/307698855)
* 2025-07-17 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820305)
* 2025-07-20 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/308383001)
* 2025-07-22 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/tgctrtyhckbdc)
* 2025-07-22 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Crates, Tips & Tricks Lightning Talks - Bring your ideas!**](https://www.meetup.com/women-in-rust/events/307560304)
* 2025-07-24 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567874)
* 2025-07-27 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/bhctrtyhckbkc)
* 2025-07-31 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820306)
* 2025-08-02 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763838567)
* 2025-08-03 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/bhctrtyhclbfb)
* 2025-08-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyhclbjb)
* 2025-08-10 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/bhctrtyhclbnb)
* 2025-08-12 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361531)

### Asia
* 2025-07-19 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi)
    * [**Rust Delhi Meetup #11**](https://www.meetup.com/rustdelhi/events/308666751)
* 2025-07-26 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**July 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/july-2025-rustacean-meetup/)

### Europe
* 2025-07-23 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund/)
    * [**Rust Dortmund Meetup - Teach and Hack**](https://www.meetup.com/rust-dortmund/events/308517530/)
* 2025-07-24 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**July talks: A Crab, a Pufferfish and a State-of-the-art Chess AI**](https://www.meetup.com/rust-and-friends/events/308687848)
* 2025-07-24 | Nuremberg/Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567874/)
* 2025-07-26 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #15**](https://www.meetup.com/stockholm-rust/events/309275728)
* 2025-07-29 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Lightning Talks July 2025**](https://www.meetup.com/rust-manchester/events/308085035)
* 2025-07-29 | Prague, CZ | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic)
    * [**Nix Meetup at Braiins :)**](https://www.meetup.com/rust-czech-republic/events/308963318)
* 2025-07-30 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group)
    * [**Rust Meetup @ BlockTech**](https://www.meetup.com/rust-amsterdam-group/events/308548455)
* 2025-07-31 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #14: Prof. Dr. Claudia Meitinger - Embassy - Möglichkeiten und Herausforderungen im Modul "Interdisciplinary Project"**](https://rust-augsburg.github.io/meetup/Meetup_14.html)
* 2025-08-06 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 08 2025**](https://lu.ma/eoydaar9)
* 2025-08-13 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944036)

### North America
* 2025-07-16 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731031)
* 2025-07-17 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/308979091)
* 2025-07-17 | Hybrid (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**July, 2025 Computer Programming Language Panel (Special Event)**](https://www.meetup.com/seattle-rust-user-group/events/307698855)
* 2025-07-17 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Rust on Bare Metal Series 1 : Introduction to Embedded Development**](https://www.meetup.com/music-city-rust-developers/events/304333113)
* 2025-07-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/308791385)
* 2025-07-24 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/xdxtqtyhckbgc)
* 2025-07-24 | México City, MX | [Rust MX](https://www.meetup.com/rust-mx)
    * [**Construyendo un Runtime Asíncrono desde Cero en Rust**](https://www.meetup.com/rust-mx/events/309687971)
* 2025-07-31 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/308675947)
* 2025-08-07 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**macros!**](https://www.meetup.com/stl-rust/events/306648747)
* 2025-08-12 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/308284338)

### South America
* 2025-07-17 | Florianópolis, BR | [Rust Brazil + Rust Floripa](https://lu.ma/calendar/cal-iOloL5ZqswCO5Mm)
    * [**Rust Floripa**](https://lu.ma/p0umq6vm)

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

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> In a way, \[the\] borrow checker also makes interfaces simpler. The rules may be restrictive, but the same rules apply to everything everywhere. I can learn them once, and then know what to expect from every API using references. There are no exceptions in libraries that try to be clever. There are no exceptions for single-threaded programs. There are no exceptions for DLLs. There are no exceptions for programs built with -fpointers-go-sideways. It may be tricky like a game of chess, but I only need to consider the rules of the game, and not odd stuff like whether my opponent glued pieces to the chessboard.

– [Kornel Lesiński on hacker news](https://news.ycombinator.com/item?id=44620667)

Thanks to [danjl1100](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1707) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
