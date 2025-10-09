Title: This Week in Rust 620
Number: 620
Date: 2025-10-08
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

### Foundation

### Newsletters

### Project/Tooling Updates
* [Seaography 2.0: A Powerful and Extensible GraphQL Framework](https://www.sea-ql.org/blog/2025-10-08-seaography/)
* [Announcing redis-rs 1.0.0 release candidate](https://github.com/redis-rs/redis-rs/blob/redis-1.0.0-rc.0/version1.md)
* [blazesym 0.2 stable release: batteries included address symbolization](https://github.com/libbpf/blazesym/discussions/1318)
* [Kernel hackers at Cauldron, 2025 edition](https://lwn.net/SubscriberLink/1039784/d2548814efb78046/)
* [Progress on defeating lifetime-end pointer zapping](https://lwn.net/SubscriberLink/1038757/d613acbb48f20a20/)
* [Upcoming Rust language features for kernel development](https://lwn.net/SubscriberLink/1039073/abf96f38b178f988/)
* [utsuru: "Go Live" on Discord using OBS, FFmpeg, or anything that supports WHIP.](https://github.com/VincentVerdynanta/utsuru/releases/tag/v0.2.1)

### Observations/Thoughts
* [The Handle trait](https://smallcultfollowing.com/babysteps/blog/2025/10/07/the-handle-trait/)
* [Variadic generics](https://www.wakunguma.com/blog/variadic-generics)
* [Thoughts on the Word Spec in Rust](https://tritium.legal/blog/word)
* [Why We Bet on Rust to Supercharge Feature Store at Agoda](https://medium.com/agoda-engineering/why-we-bet-on-rust-to-supercharge-feature-store-at-agoda-ed4a70d2efb7)
* [walrus: ingesting data at memory speeds](https://nubskr.com/2025/10/06/walrus.html)
* [Realtime concurrency primitives](https://blog.inkreas.ing/realtime-concurrency)
* [Why we didn't rewrite our feed handler in Rust](https://databento.com/blog/why-we-didnt-rewrite-our-feed-handler-in-rust)
* [audio] [Netstack.FM Episode 7 – Rustls with Dirkjan Ochtman](https://netstack.fm/#episode-7)
* [audio] [Netstack.FM Episode 8 – Fuchsia's Netstack3 with Bruno Dal Bo Silva](https://netstack.fm/#episode-8)

### Rust Walkthroughs
* [Axum Backend Series: Implementing Password Reset](https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-reset-password/)
* [Axum Backend Series: Email Verification After Registration](https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-email-verification/)
* [Fun With HyperLogLog and SIMD](https://vaktibabat.github.io/posts/hyperloglog/)
* [How to automatically Profile Performance of Rust Applications](https://pawelurbanek.com/rust-optimize-performance)
* [Async Rust with Tokio I/O Streams: Backpressure, Concurrency, and Ergonomics](https://biriukov.dev/docs/async-rust-tokio-io/1-async-rust-with-tokio-io-streams-backpressure-concurrency-and-ergonomics/)
* [Preventing Invalid Database Access At Compile Time](https://www.svix.com/blog/preventing-db-misuse-at-compile-time/)
* [Rust unit testing: test doubles & stubs](https://jorgeortiz.dev/posts/rust_unit_testing_test_doubles_stub/)
* [Cancelling async Rust](https://sunshowers.io/posts/cancelling-async-rust/)
* [Behind the Scenes of Pingoo: Slashing allocations with mimalloc and heapless to build the fastest reverse proxy](https://kerkour.com/rust-pingoo-high-performance-allocations-mimalloc-heapless)
* [Generic Numeric Computations in Rust ](https://michaelmauderer.com/blog/generic-numeric-computations/)
* [Let's write a macro in Rust - Part 3](https://hackeryarn.com/post/rust-macros-3/)
* [Rust/C++ Interop Part 5 - Interop in 2025: A Complete Example](https://tylerjw.dev/posts/20251003-rust-cpp-interop-2025-update/)

### Research

### Miscellaneous
* [videos] [RustConf 2025](https://www.youtube.com/playlist?list=PL2b0df3jKKiRFEuVNk76ufXagOgEJ9sBZ)
* [audio] [Prime Video - Alexandru Ene, Principal Engineer](https://corrode.dev/podcast/s05e01-prime-video/)

## Crate of the Week

This week's crate is [tokio-netem](https://crates.io/crates/tokio-netem), a toolbox of Tokio AsyncRead /AsyncWrite adapters to emulate latency, throttling, slicing, termination, forced shutdown, data injection and data corruption.

Thanks to [Viacheslav Biriukov](https://users.rust-lang.org/t/crate-of-the-week/2704/1478) for the self-suggestion!

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
*No Calls for participation were submitted this week.*

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

* [**TokioConf 2026**](https://tokio.rs/blog/2025-09-26-announcing-tokio-conf-cfp)| CFP closes 2025-12-08 | Portland, Oregon, USA | 2026-04-20

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

398 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-09-30..2025-10-07

 #### Compiler
* [add a dummy codegen backend](https://github.com/rust-lang/rust/pull/146596)
* [don't normalize higher-ranked assumptions if they're not used](https://github.com/rust-lang/rust/pull/147299)
* [extending `#[rustc_force_inline]` to be applicable to inherent methods](https://github.com/rust-lang/rust/pull/147231)
* [fix the bevy implied bounds hack for the next solver](https://github.com/rust-lang/rust/pull/147184)
* [Global Value Numbering: support unions](https://github.com/rust-lang/rust/pull/146355)
* [Global Value Numbering: use a VnIndex in Address projection](https://github.com/rust-lang/rust/pull/144477)
* [miri: add support for temporal mixing of atomic and non-atomic accesses in GenMC mode](https://github.com/rust-lang/miri/pull/4611)
 #### Library
* [add `CloneFromCell` and `Cell::get_cloned`](https://github.com/rust-lang/rust/pull/145685)
* [add `Path::has_trailing_sep` and related methods](https://github.com/rust-lang/rust/pull/142506)
* [add `mem::conjure_zst`](https://github.com/rust-lang/rust/pull/146479)
* [add fast-path for accessing the current thread id](https://github.com/rust-lang/rust/pull/143069)
* [implement `Box::take`](https://github.com/rust-lang/rust/pull/147227)
* [implement non-poisoning `Mutex::with_mut`, `RwLock::with` and `RwLock::with_mut`](https://github.com/rust-lang/rust/pull/147328)
* [hashbrown: recognize and use over sized allocations](https://github.com/rust-lang/hashbrown/pull/523)
 #### Cargo
* [`fix(run)`: Override arg0 for cargo scripts](https://github.com/rust-lang/cargo/pull/16027)
* [`fix(timings)`: compute codegen start time to draw dep lines](https://github.com/rust-lang/cargo/pull/16055)
* [`fix(toml)`: Prevent non-script fields in Cargo scripts](https://github.com/rust-lang/cargo/pull/16026)
* [accessing each build script's `OUT_DIR`](https://github.com/rust-lang/cargo/pull/15891)
* [add panic=immediate-abort support to Cargo](https://github.com/rust-lang/cargo/pull/16041)
* [consider public dependencies when choosing a version in cargo add (#1…](https://github.com/rust-lang/cargo/pull/15966)
* [convert a multi-part diagnostic to a report](https://github.com/rust-lang/cargo/pull/16035)
* [feat (publish): deprecate `--token` option](https://github.com/rust-lang/cargo/pull/16046)
* [fix FileLock path tracking after rename in package operation](https://github.com/rust-lang/cargo/pull/16036)
* [fix `unsafe_op_in_unsafe_fn` for Windows](https://github.com/rust-lang/cargo/pull/16058)
* [fix: remove FIXME comment that's no longer a problem](https://github.com/rust-lang/cargo/pull/16025)
* [lockfile schemas error cleanup](https://github.com/rust-lang/cargo/pull/16039)
* [public in private manifest errors](https://github.com/rust-lang/cargo/pull/16002)
* [recommend `package.rust-version` in the Rust version section of `reference/semver.md`](https://github.com/rust-lang/cargo/pull/15806)
* [test: null-terminated path for reserved windows name detection](https://github.com/rust-lang/cargo/pull/16052)
 #### Rustdoc
* [replace `rustc_span::Span` with a stripped down version for librustdoc's highlighter](https://github.com/rust-lang/rust/pull/147189)
 #### Clippy
* [`assertions_on_constants`: Suggest using a const block when using a named constant](https://github.com/rust-lang/rust-clippy/pull/15774)
* [`zero_repeat_side_effects`: better identify exprs with side effects](https://github.com/rust-lang/rust-clippy/pull/15814)
* [const eval changes](https://github.com/rust-lang/rust-clippy/pull/15773)
* [do not suggest using a `if let` chain if it is not supported](https://github.com/rust-lang/rust-clippy/pull/15746)
* [do not trigger `inefficient_to_string` after Rust 1.82](https://github.com/rust-lang/rust-clippy/pull/15729)
* [extend `while_let_loop` to `loop { let else }`](https://github.com/rust-lang/rust-clippy/pull/15701)
* [fix `if_then_some_else_none` false positive when return exists in block expr](https://github.com/rust-lang/rust-clippy/pull/15783)
* [fix `let_unit_value` suggests wrongly for field init shorthand](https://github.com/rust-lang/rust-clippy/pull/15791)
* [fix `mem_replace_with_default` wrongly unmangled macros](https://github.com/rust-lang/rust-clippy/pull/15786)
* [implement `volatile_composites` lint](https://github.com/rust-lang/rust-clippy/pull/15686)
 #### Rust-Analyzer
* [make rust-analyzer use a dedicated build directory](https://github.com/rust-lang/rust/pull/141839)
* [deduplicate sort+dedup calls](https://github.com/rust-lang/rust-analyzer/pull/20794)
* [log flycheck stdout and stderr to files](https://github.com/rust-lang/rust-analyzer/pull/20806)
* [fix missing parentheses for `missing_unsafe`](https://github.com/rust-lang/rust-analyzer/pull/20793)
* [fix panic when using analysis-stats](https://github.com/rust-lang/rust-analyzer/pull/20777)
* [fix erroneous diagnostic `incorrect_generics_len` when there are generics on `enum` variant used through type alias](https://github.com/rust-lang/rust-analyzer/pull/20787)
* [ignore impl trait safety errors when the trait is unresolved](https://github.com/rust-lang/rust-analyzer/pull/20770)

### Rust Compiler Performance Triage

Largely a positive week. Big win coming from avoiding unnecessary work for debug log in [#147293](https://github.com/rust-lang/rust/pull/147293), and another one for rustdoc from optimized span representation for highlighter [#147189](https://github.com/rust-lang/rust/pull/147189). Lots of noisy results otherwise.

Triage done by **@panstromek**.
Revision range: [8d72d3e1..1a3cdd34](https://perf.rust-lang.org/?start=8d72d3e1e96f58ca10059a6bb6e8aecba4a0e9cd&end=1a3cdd34629306fa67624eaa60d73687e7fcf855&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.5%  | [0.2%, 2.0%]    | 10    |
| Regressions ❌ <br /> (secondary)  | 0.4%  | [0.0%, 0.8%]    | 50    |
| Improvements ✅ <br /> (primary)   | -1.3% | [-5.3%, -0.2%]  | 147   |
| Improvements ✅ <br /> (secondary) | -1.3% | [-12.7%, -0.1%] | 111   |
| All ❌✅ (primary)                 | -1.2% | [-5.3%, 2.0%]   | 157   |


6 Regressions, 3 Improvements, 6 Mixed; 8 of them in rollups
40 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/33d1d2f6103c22772c45562aa159d1e1257c228e/triage/2025/2025-10-06.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [Uplifts and extends `clippy::needless-maybe-sized` into rustc](https://github.com/rust-lang/rust/pull/145924)
* [prefer alias candidates for sizedness + auto trait goals](https://github.com/rust-lang/rust/pull/144064)
* [implement Extend<{Group, Literal, Punct, Ident}> for TokenStream](https://github.com/rust-lang/rust/pull/145722)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Add `target="host"` meaning the current host](https://github.com/rust-lang/cargo/issues/13051)

*No Items entered Final Comment Period this week for
  [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) or
  [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [RFC: Allow `#[ignore(Trait)]` on field to ignore it when deriving `Trait`](https://github.com/rust-lang/rfcs/pull/3869)

## Upcoming Events

Rusty Events between 2025-10-08 - 2025-11-05 🦀

### Virtual
* 2025-10-08 | Virtual (Boulder, CO, US) | [Boulder Elixir](https://www.meetup.com/boulder-elixir/events/)
    * [**Integrating Elixir and Apache DataFusion with Rustler**](https://www.meetup.com/boulder-elixir/events/310996627/)
* 2025-10-09 - 2025-10-10 | Hybrid (Paris, FR) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2025**](https://eurorust.eu/schedule/)
* 2025-10-09 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046639/)
* 2025-10-09 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/jotnli2g)
* 2025-10-09 | Virtual (San Diego, CA, US) | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust October 2025 Online Meetup**](https://www.meetup.com/san-diego-rust/events/311359525/)
* 2025-10-12 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109160/)
* 2025-10-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361534/)
* 2025-10-15 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**indielinks**](https://www.meetup.com/vancouver-rust/events/307731034/)
* 2025-10-16 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/o8fh3fh7)
* 2025-10-16 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/306046668/)
* 2025-10-18 | Virtual (Gdansk, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto/)
    * [**[BEZPŁATNIE] Programowanie w języku Rust**](https://www.meetup.com/stacja-it-trojmiasto/events/310935164/)
* 2025-10-19 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109167/)
* 2025-10-21 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/311068625/)
* 2025-10-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/310002307/)
* 2025-10-23 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**October, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351020/)
* 2025-10-23 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046641/)
* 2025-10-23 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/zyc3touy)
* 2025-10-26 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109171/)
* 2025-10-28 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361444/)
* 2025-10-30 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/t8yovmmm)
* 2025-11-01 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763868657)
* 2025-11-02 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109173/)
* 2025-11-05 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 2025-11-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyhcpbhb/)

### Asia
* 2025-10-08 | Kuala Lumpur, MY | [Rust Malaysia](https://t.me/rustlangmalaysia)
    * [**Malaysia Rust Meetup**](https://docs.google.com/forms/d/e/1FAIpQLScESY4eHc5lzZznAHZmFxI85CYaOKCYTQASRwXxC2y0KpI6zw/viewform)
* 2025-10-09 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup)
    * [**Building Pocket-Sized Terminal UIs with Rust**](https://www.meetup.com/tokyo-rust-meetup/events/310899137/)
* 2025-10-20 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust October 2025 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/310628902/)

### Europe
* 2025-10-08 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 10 2025**](https://luma.com/8u55jo0h)
* 2025-10-08 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #79**](https://www.meetup.com/rust-paris/events/310424476/)
* 2025-10-08 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944041/)
* 2025-10-09 - 2025-10-10 | Hybrid (Paris, FR) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2025**](https://eurorust.eu/schedule/)
* 2025-10-14 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #13 @ letsboot**](https://www.meetup.com/rust-basel/events/310827834/)
* 2025-10-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/311035141/)
* 2025-10-21 | Bergen, No | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Meetup #01 @ Zrch**](https://www.meetup.com/bergen-rust-new-technology/events/311153821/)
* 2025-10-21 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592252/)
* 2025-10-21 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group)
    * [**Rust in Surgery: Powering the Data Pipelines**](https://www.meetup.com/london-rust-project-group/events/310813952/)
* 2025-10-28 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester October Code Night**](https://www.meetup.com/rust-manchester/events/307919171/)
* 2025-10-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #62 sponsored by Google!**](https://www.meetup.com/copenhagen-rust-community/events/311405044/)
* 2025-10-30 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Prague (October 2025)**](https://www.meetup.com/rust-prague/events/310967094/)
* 2025-11-02 - 2025-11-04 | Florence, IT | [Rustlab 2025](https://rustlab.it/)
    * [**Rustlab 2025**](https://rustlab.it/)
* 2025-11-04 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester November Talk**](https://www.meetup.com/rust-manchester/events/310921632/)
* 2025-11-05 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310601872/)
* 2025-11-05 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Rust/ACCU meetup.**](https://www.meetup.com/oxford-rust-meetup-group/events/nnrkttyhcpbhb/)

### North America
* 2025-10-08 | Phoenix, AZ, US | [Desert Rust](https://www.meetup.com/desert-rustaceans)
    * [**Rust <> C++**](https://www.meetup.com/desert-rustaceans/events/311365791/)
* 2025-10-09 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Aya the Beholder: Writing an eBPF Firewall with the Aya Crate**](https://www.meetup.com/utah-rust/events/311145663/)
* 2025-10-14 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Rust in Robotics & Isograph**](https://www.meetup.com/rust-nyc/events/311328171/)
* 2025-10-15 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**indielinks**](https://www.meetup.com/vancouver-rust/events/307731034/)
* 2025-10-16 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/311012947/)
* 2025-10-16 | San Francisco, CA, US | [Svix](https://luma.com/calendar/cal-Cnmn4RR2n4fRUNZ)
    * [**San Francisco Rust Meetup**](https://luma.com/tp6w7tc9)
* 2025-10-21 | San Francisco, CA, US | [Vara & Gear](https://luma.com/events-by-vara-gear)
    * [**Rust Workshop by Vara Network**](https://luma.com/kbs2os1c)
* 2025-10-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/308284343/)
* 2025-10-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310457307/)
* 2025-10-23 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**October, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351020/)
* 2025-10-23 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Year In Review**](https://www.meetup.com/music-city-rust-developers/events/304333267/)
* 2025-10-23 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust)
    * [**October Rust Meetup: A Special Presentation and Monthly Meetups are Back!**](https://www.meetup.com/spokane-rust/events/311346444/)
* 2025-10-25 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Porter Square Rust Lunch, Oct 25**](https://www.meetup.com/bostonrust/events/310983712/)
* 2025-10-30 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/308675988/)
* 2025-11-01 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Chinatown Rust Lunch, Nov 1**](https://www.meetup.com/bostonrust/events/311039492/)

### Oceania
* 2025-10-22 | Perth, AU | [Rust Perth Meetup Group](https://www.meetup.com/perth-rust-meetup-group)
    * [**October Meetup**](https://www.meetup.com/perth-rust-meetup-group/events/310847099/)
* 2025-10-28 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**October Meetup**](https://www.meetup.com/rust-canberra/events/311234237/)

### South America
* 2025-10-08 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Octubre Async - Escribimos un Runtime desde Cero!**](https://www.meetup.com/rust-argentina/events/311276950/)
* 2025-10-25 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP na Amazon Web Services**](https://www.meetup.com/rust-sao-paulo-meetup/events/311084440/)
* 2025-10-30 | Florianopolis, BR | [Rust Brasil](https://luma.com/calendar/cal-iOloL5ZqswCO5Mm)
    * [**Rust Floripa**](https://luma.com/lky7an18)

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

> For me personally, the best thing about becoming successful at anything is you gain the ability to lift others up.

– [Nell Shamrell-Harrington at RustConf](https://youtu.be/nEHLIUWO78I?t=1175) (youtube video link, the rest of the talk is great, too!)

Thanks to [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1720) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
