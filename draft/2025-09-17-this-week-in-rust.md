Title: This Week in Rust 617
Number: 617
Date: 2025-09-17
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

* [Title of the linked Page](https://example.com/my_article)

If you add a link to a non-text content please prefix it with `[video]` or `[audio]`:

* [video] [Title of the linked video](https://example.com/my_video_article)
* [audio] [Title of the linked audio file](https://example.com/my_podcast)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official
* [crates.io phishing campaign](https://blog.rust-lang.org/2025/09/12/crates-io-phishing-campaign/)

### Foundation

### Newsletters
* [The Embedded Rustacean Issue #54](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-54)
* [Rust Trends Issue #70: Foundation Moves and Performance Breakthroughs](https://rust-trends.com/newsletter/rust-foundation-performance-breakthroughs-2025)

### Project/Tooling Updates
* [Cot v0.4: Particularly Lazy](https://mackow.ski/blog/cot-v04-particularly-lazy/)
* [Announcing Diesel 2.3](https://diesel.rs/news/2_3_0_release.html)
* [Rerun 0.25.0 - Syntax highlighting, table filtering, transparent objects](https://github.com/rerun-io/rerun/releases/tag/0.25.0)
* [Introducing Obelisk 0.24.1](https://obeli.sk/blog/introducing-obelisk-0-24-1/)
* [Introducing CurveForge](https://smartnets.etrovub.be/posts/introducing-curveforge/)
* [hotpath 0.2.5 - a simple performance and memory profiler](https://github.com/pawurb/hotpath)
* [Swiftide 0.31](https://blog.bosun.ai/swiftide-0-31/)

### Observations/Thoughts
* [Embedded async debugging and inspect-embassy](https://tweedegolf.nl/en/blog/161/embedded-async-debugging-and-inspect-embassy)
* [Be Simple](https://corrode.dev/blog/simple/)
* [Why We Built Our Own SQL Parser From Scratch: A Rust Implementation Story](https://www.databend.com/blog/category-engineering/2025-09-10-query-parser/)
* [Comparing transitive dependency version resolution in Rust and Java](https://blog.frankel.ch/dependency-version-resolution-rust-java/)
* [Trade-offs in designing DSLs](https://forgestream.idverse.com/blog/20250916-dsl-trade-offs/)
* [Rust Algorithm Bites – Validating a Binary Search Tree](https://d34dl0ck.me/rust-algorithm-bites-validate-bst/index.html)

### Rust Walkthroughs
* [Axum Backend Series: Docker, Database and Connection Pooling](https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-database-setup-using-docker/)
* [The Hidden Rules Behind Rust Functions & Closures](https://blog.cuongle.dev/p/the-hidden-rules-behind-rust-functions)
* [Rust unit testing: asynchronous code](https://jorgeortiz.dev/posts/rust_unit_testing_async_code/)
* [Ray Tracing in One Weekend - In Rust](https://buttondown.com/dabeaz/archive/new-video-ray-tracing-in-one-weekend-in-rust/)
* [New Schematic - How to save $327.6 million using Rust](https://newschematic.org/blog/how-to-save-327-6-million-using-rust/)

### Research

### Miscellaneous
* [audio] [Netstack.FM — A Podcast About Networking and Rust](https://netstack.fm/#episode-5)
* [video] [Jan David Nose Interview, Rust Infrastructure Team (Rust Project Content @ RustConf 2025)](https://www.youtube.com/watch?v=r7i-2wHtNjw)
* [August 2025 Rust Jobs Report](https://filtra.io/rust/jobs-report/aug-25)
* [The Symbiosis Of Rust And Arm: A Conversation With David Wood](https://filtra.io/rust/interviews/arm-sep-25)

## Crate of the Week

This week's crate is [asciinema](https://crates.io/crates/asciinema), a well-known command-line tool for recording, replaying and streaming terminal sessions recently rewritten in Rust.

Despite a lack of suggestions, llogiq is plenty happy with his choice.

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
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Diesel - Automatically infer enum definitions](https://github.com/diesel-rs/diesel/issues/4759)
<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
*No Calls for papers or presentations were submitted this week.*

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

379 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-09-09..2025-09-16

#### Compiler
* [implement `#[rustc_align_static(N)]` on `static`s](https://github.com/rust-lang/rust/pull/146178)
* [move more early buffered lints to dyn lint diagnostics](https://github.com/rust-lang/rust/pull/145881)
* [make `AssocItem` aware of its impl kind](https://github.com/rust-lang/rust/pull/145186)
* [match clang's `va_arg` assembly on arm targets](https://github.com/rust-lang/rust/pull/144549)
* [reject invalid literal suffixes in tuple indexing, tuple `struct` indexing, and `struct` field name position](https://github.com/rust-lang/rust/pull/145463)
* [sort array trait implementation suggestions correctly](https://github.com/rust-lang/rust/pull/146403)
* [strip frontmatter in fewer places](https://github.com/rust-lang/rust/pull/146340)
* [miri: fix release/scquire synchonization for loads from the store buffer](https://github.com/rust-lang/miri/pull/4577)
* [miri: make a basic hello world work on wasip2](https://github.com/rust-lang/miri/pull/4582)

#### Library
* [constify Eq, Ord, PartialOrd](https://github.com/rust-lang/rust/pull/144847)
* [implement `Sum` and `Product` for `f16` and `f128`](https://github.com/rust-lang/rust/pull/146300)
* [inclusive `Range`s: change `end` to `last`](https://github.com/rust-lang/rust/pull/144765)
* [make `Barrier` `RefUnwindSafe` again](https://github.com/rust-lang/rust/pull/146322)
* [stabilize `BTree{Map,Set}::extract_if`](https://github.com/rust-lang/rust/pull/145471)
* [support integer literals in `${concat()}`](https://github.com/rust-lang/rust/pull/146308)

#### Cargo
* [cli: Allow completions for third-party subcommand names](https://github.com/rust-lang/cargo/pull/15961)
* [completer: Added completion for `--features` flag](https://github.com/rust-lang/cargo/pull/15309)
* [complete: Show local crates/features over other members](https://github.com/rust-lang/cargo/pull/15956)
* [frontmatter: Try alternative len code fences](https://github.com/rust-lang/cargo/pull/15952)
* [manifest: Show error source to users](https://github.com/rust-lang/cargo/pull/15939)
* [publish: Switch the 'ctrl-c on wait' line to a help message](https://github.com/rust-lang/cargo/pull/15942)

#### Rustdoc
* [Correctly handle literal search on paths](https://github.com/rust-lang/rust/pull/146448)

#### Clippy
* [`elidable_lifetime_names`: avoid overlapping spans in suggestions](https://github.com/rust-lang/rust-clippy/pull/15667)
* [`len_zero`: don't eagerly call `GenericArgs::type_at`](https://github.com/rust-lang/rust-clippy/pull/15660)
* [`multiple_unsafe_ops_per_block`: ignore unsafe ops from `.await` desugaring](https://github.com/rust-lang/rust-clippy/pull/15654)
* [`needless_closure`: don't lint on `AsyncFn*`s](https://github.com/rust-lang/rust-clippy/pull/15649)
* [`needless_return`: fix false positive with `cfg`d code after return](https://github.com/rust-lang/rust-clippy/pull/15669)
* [`ref_option`: don't lint in external and proc-macros](https://github.com/rust-lang/rust-clippy/pull/15668)
* [`semicolon_inside_block`: don't lint if block is in parens](https://github.com/rust-lang/rust-clippy/pull/15626)
* [`use_self`: don't early-return if the outer type has no lifetimes](https://github.com/rust-lang/rust-clippy/pull/15611)
* [add suggestion to `cast_sign_loss` and `cast_possible_wrap` using the `cast_{un,}signed()` methods](https://github.com/rust-lang/rust-clippy/pull/15384)
* [fix `as_underscore` to only suggest when it's suggestable](https://github.com/rust-lang/rust-clippy/pull/15652)
* [fix `invalid_upcast_comparisons` wrongly unmangled macros](https://github.com/rust-lang/rust-clippy/pull/15663)
* [fix `useless_attribute` false positive on `deprecated_in_future`](https://github.com/rust-lang/rust-clippy/pull/15645)
* [recognize canonical `?` pattern with `Result`](https://github.com/rust-lang/rust-clippy/pull/15680)

#### Rust-Analyzer
* [add more workaround hacks for incorrect startup diagnostics](https://github.com/rust-lang/rust-analyzer/pull/20402)
* [fix `LifetimeParam::lifetime_bounds` invalid implement](https://github.com/rust-lang/rust-analyzer/pull/20624)
* [fix extra semicolon before else in let-stmt](https://github.com/rust-lang/rust-analyzer/pull/20657)
* [fix indent for `unresolved_field` fixes](https://github.com/rust-lang/rust-analyzer/pull/20613)
* [always coerce in a cast, even when there are unknown types](https://github.com/rust-lang/rust-analyzer/pull/20649)
* [don't mark unknown type as implementing every notable trait](https://github.com/rust-lang/rust-analyzer/pull/20665)
* [don't output an empty generic parameters list in `generate_function`](https://github.com/rust-lang/rust-analyzer/pull/20653)
* [don't trigger two flychecks when saving files that are part of targets](https://github.com/rust-lang/rust-analyzer/pull/20635)
* [fix expand macro recursively not working correctly for nested macro calls](https://github.com/rust-lang/rust-analyzer/pull/20612)
* [fix normalization in the new solver](https://github.com/rust-lang/rust-analyzer/pull/20647)
* [infinite loop while elaborting predicates](https://github.com/rust-lang/rust-analyzer/pull/20654)
* [make `#[target_feature]` always safe on WASM](https://github.com/rust-lang/rust-analyzer/pull/20642)
* [more precise clause filtering for `explicit_*_predicates_of`](https://github.com/rust-lang/rust-analyzer/pull/20671)
* [only compute unstable paths on nightly toolchains for IDE features](https://github.com/rust-lang/rust-analyzer/pull/20517)
* [resolve paths to snapshot test libraries absolutely](https://github.com/rust-lang/rust-analyzer/pull/20639)
* [migrate `InferenceTable` into next-solver](https://github.com/rust-lang/rust-analyzer/pull/20578)

### Rust Compiler Performance Triage

Difficult week to interpret, because a positive change in [#145910](https://github.com/rust-lang/rust/pull/145910) performs a bit worse in our benchmarks than it would in the real world. Overall result is probably still slightly negative, because there's more work from added features. On the other hand, we also have a nice improvement in reducing the number of query dependencies in compiler's incremental system in [#145186](https://github.com/rust-lang/rust/pull/145186).

Triage done by **@panstromek**.
Revision range: [f13ef0d7..52618eb3](https://perf.rust-lang.org/?start=f13ef0d75d834c826c9479a5d244bcfb9891df45&end=52618eb338609df44978b0ca4451ab7941fd1c7a&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.5%  | [0.2%, 2.7%]   | 72    |
| Regressions ❌ <br /> (secondary)  | 0.7%  | [0.0%, 3.5%]   | 96    |
| Improvements ✅ <br /> (primary)   | -0.5% | [-0.9%, -0.1%] | 10    |
| Improvements ✅ <br /> (secondary) | -0.8% | [-2.9%, -0.1%] | 41    |
| All ❌✅ (primary)                 | 0.4%  | [-0.9%, 2.7%]  | 82    |


1 Regression, 1 Improvement, 6 Mixed; 3 of them in rollups
36 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/888c0a24417c3883373ae0844f760f8300176b90/triage/2025/2025-09-15.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Allow &raw `[mut | const]` for union field in safe code](https://github.com/rust-lang/rust/pull/141469)
* [Deny-by-default never type lints](https://github.com/rust-lang/rust/pull/146167)
* [Opportunistically split `!=` to successfully parse never type](https://github.com/rust-lang/rust/pull/145536)
* [Allow specifying multiple bounds for same associated item, except in trait objects](https://github.com/rust-lang/rust/pull/146593)
* [rustdoc: hide `#[repr]` if it isn't part of the public ABI](https://github.com/rust-lang/rust/pull/116882)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: `#[cfg(since(rust, "1.95"))]` for Rust-version conditional compilation](https://github.com/rust-lang/rfcs/pull/3857)
* [Mitigation enforcement](https://github.com/rust-lang/rfcs/pull/3855)
* [RFC for `#[stable(since)]`](https://github.com/rust-lang/rfcs/pull/3854)

## Upcoming Events

Rusty Events between 2025-09-17 - 2025-10-15 🦀

### Virtual
* 2025-09-11 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305646019)
* 2025-09-11 | Virtual (San Diego, CA, US) | [San Diego Rust](https://www.meetup.com/san-diego-rust/events/)
    * [**San Diego Rust September 2025 Online Meetup**](https://www.meetup.com/san-diego-rust/events/310326567)
* 2025-09-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002480)
* 2025-09-15 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Setup Tock OS in a virtual environment (online) - prep for Sep 17**](https://www.meetup.com/charlottesville-rust-meetup/events/310706165/)
* 2025-09-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/306757758)
* 2025-09-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731033)
* 2025-09-18 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/305646039/)
* 2025-09-23 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361443)
* 2025-09-25 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046637)
* 2025-10-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyhcnbcb)

### Asia
* 2025-09-13 | Hangzhou, CN | [WebAssembly and Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**GOSIM AI Hangzhou 2025 (CFP is still open)**](https://www.meetup.com/wasm-rust-meetup/events/309987624)
* 2025-09-13 - 2025-09-14 | Hangzhou, CN | [GOSIM](https://hangzhou2025.gosim.org/schedule/)
    * [**GOSIM Hangzhou 2025**](https://dev.events/conferences/rust-global-china-and-rust-china-conf-2025-dscrf0e1)
* 2025-09-17 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**In person Rust September 2025 at Varonis in Herzeliya**](https://www.meetup.com/rust-tlv/events/310708628)
* 2025-10-02 | Seoul, KR | [Seoul Rust (Programming Language) Meetup](https://www.meetup.com/rust-seoul-meetup/events/)
    * [**Seoul Rust Meetup**](https://www.meetup.com/rust-seoul-meetup/events/310824483)

### Europe
* 2025-09-10 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust in September: Atomic Rust**](https://www.meetup.com/rustcologne/events/310858679)
* 2025-09-10 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944038)
* 2025-09-11 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #4 @Zühlke**](https://www.meetup.com/rust-bern/events/309903540)
* 2025-09-16 - 2025-09-18 | Berlin, DE | [Oxidize Conference](https://oxidizeconf.com/)
    * [**Oxidize Conference**](https://oxidizeconf.com/)
* 2025-09-16 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592250)
* 2025-09-17 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Rust Girona Hack & Learn 09 2025**](https://lu.ma/ql3u6q5u)
* 2025-09-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Talk Night at Mjølner Informatics**](https://www.meetup.com/rust-aarhus/events/310562343)
* 2025-09-23 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #78**](https://www.meetup.com/rust-paris/events/310935603)
* 2025-09-24 | Göteborg, SE | [Rust Göteborg](https://www.meetup.com/rustgbg/events/)
    * [**Rust Gbg — September 2025**](https://www.meetup.com/rustgbg/events/310866773)
* 2025-09-24 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2025 / 3 - hybrid**](https://www.meetup.com/rust-munich/events/307105978)
* 2025-09-25 | Augsburg, DE | [Rust Augsburg](https://rust-augsburg.github.io/meetup/introduction.html)
    * [**Augsburg Rust Meetup #15**](https://rust-augsburg.github.io/meetup/Meetup_15.html)
* 2025-10-01 | Olomouc, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**4. Rust Moravia Meetup (In the capital!)**](https://www.meetup.com/rust-moravia/events/310743282)
* 2025-10-02 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310062134)
* 2025-10-08 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #79**](https://www.meetup.com/rust-paris/events/310424476)
* 2025-10-08 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944041)

### North America
* 2025-09-10 | Phoenix, AZ, US | [Desert Rust](https://www.meetup.com/desert-rustaceans/events/)
    * [**Rust <> JS**](https://www.meetup.com/desert-rustaceans/events/310669989)
* 2025-09-11 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/310840020)
* 2025-09-11 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Mazes and Graphs in Rust**](https://www.meetup.com/utah-rust/events/310674937)
* 2025-09-11 | México City, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Polars para análisis y manipulación de datos**](https://www.meetup.com/rust-mx/events/310408223)
* 2025-09-14 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Davis Square Rust Lunch, Sep 14**](https://www.meetup.com/bostonrust/events/310106317)
* 2025-09-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/308284339)
* 2025-09-16 | San Francisco, CA, US | [Vara Network](https://lu.ma/events-by-vara-gear)
    * [**Rust Workshop by Vara Network**](https://luma.com/1bii0kv7)
* 2025-09-18 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust on Bare Metal Series 3 : Place Holder**](https://www.meetup.com/music-city-rust-developers/events/304333261)
* 2025-09-18 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**September, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/308677324)
* 2025-09-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310287849)
* 2025-09-24 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Tick, Tock, talk—find out how Rust secures embedded devices**](https://www.meetup.com/charlottesville-rust-meetup/events/310603587)
* 2025-09-25 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Rust-Atl At Manuels Tavern**](https://www.meetup.com/rust-atl/events/308675983)
* 2025-10-02 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**🚁 Rust in Flight: Lessons from Designing a 3D‑Printed Quadcopter with Embedded**](https://www.meetup.com/stl-rust/events/310279407)

### Oceania:
* 2025-10-01 | Perth, WA, AU | [Rust Perth Meetup Group](https://www.meetup.com/perth-rust-meetup-group/events/)
    * [**October Meetup**](https://www.meetup.com/perth-rust-meetup-group/events/310847099)

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

> **Real Question:** is an array a struct/tuple, or is it an enum?

– [Lokathor on github](https://github.com/rust-lang/rust/pull/146509#discussion_r2346807413)

Thanks to [Theemathas](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1716) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
