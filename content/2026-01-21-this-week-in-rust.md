Title: This Week in Rust 635
Number: 635
Date: 2026-01-21
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
* [What does it take to ship Rust in safety-critical?](https://blog.rust-lang.org/2026/01/14/what-does-it-take-to-ship-rust-in-safety-critical/)

### Newsletters
* [The Embedded Rustacean Issue #63](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-63)
* [Scientific Computing in Rust #14 (January 2026)](https://scientificcomputing.rs/monthly/2026-01)

### Project/Tooling Updates
* [Creusot 0.9.0](https://creusot-rs.github.io/devlog/2026-01-19/)
* [diesel-guard v0.5.0 released](https://github.com/ayarotsky/diesel-guard/releases/tag/v0.5.0)
* [skim v1.0.0: fuzzy-finder TUI now uses Ratatui](https://github.com/skim-rs/skim/releases/tag/v1.0.0)
* [d-engine - A Lightweight Distributed Coordination Engine for Rust](https://dev.to/joshua_c/d-engine-a-lightweight-distributed-coordination-engine-for-rust-210j)
* [govctl: Opinionated CLI tool to enforce RFC-driven AI coding](https://dev.to/lucifer1004/govctl-opinionated-cli-tool-to-enforce-rfc-driven-ai-coding-2ngi)
* [Burn 0.20.0 Release: Unifying CPU & GPU kernels with CubeCL](https://burn.dev/blog/release-0.20.0/)
* [git-cliff 2.12.0 is released! (a highly customizable changelog generator)](https://git-cliff.org/blog/2.12.0)
* [Cot v0.5 Released: New Features for Lazy Web Developers](https://mackow.ski/blog/cot-v05-new-features-for-lazy-web-developers/)

### Observations/Thoughts
* [Stop Allocating Per Label: A Data‑Driven Rust SymbolTable for OTLP/TSDB](https://baarse.substack.com/p/stop-allocating-per-label-a-datadriven)
* [Profile a Parser Implementation in Rust](https://blog.wybxc.cc/blog/profile-cgrammar/)
* [Rust's Culture of Semantic Precision](https://www.alilleybrinker.com/mini/rusts-culture-of-semantic-precision/)
* [video] [Rust is not about memory safety](https://www.youtube.com/watch?v=ngTZN09poqk)

### Rust Walkthroughs
* [Structuring a Gtk4 Rust App](https://w-graj.net/posts/rust-gtk4-mvpvm/)
* [Rust's standard library on the GPU](https://www.vectorware.com/blog/rust-std-on-gpu/)
* [Elegant and safe concurrency in Rust with async combinators](https://kerkour.com/rust-async-combinators-concurrency)
* [AWS Lambda From Scratch](https://forgestream.idverse.com/blog/20260119-lambda-from-scratch/)
* [Using Oracle db26ai from Rust with the oracle crate - Queries](https://jorgeortiz.dev/posts/rust_use_oracle_db26ai_with_oracle_crate_1/)
* [Using Oracle db26ai from Rust with the sibyl crate - Queries](https://jorgeortiz.dev/posts/rust_use_oracle_db26ai_with_sibyl_crate_1/)

### Miscellaneous
* [December '25 Rust Jobs Report](https://filtra.io/rust/jobs-report/dec-25)

## Crate of the Week

This week's crate is [throttled-tracing](https://crates.io/crates/throttled-tracing), a crate of periodic and throttled logging macros.

Thanks to [Paperinik](https://users.rust-lang.org/t/crate-of-the-week/2704/1522) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

##### [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing)
* [Tracking Issue for timing report SVG render backend](https://github.com/rust-lang/cargo/issues/16440)

* *No calls for testing were issued this week by
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing) or
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing).*

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

* [**RustConf 2026**](https://sessionize.com/rustconf-2026/) | CFP closes 2026-02-16 | Montreal, Quebec, Canada | 2026-09-08 - 2026-09-11

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

464 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-01-13..2026-01-20

#### Compiler
* [`rustc_errors`: Add (heuristic) Syntax Highlighting for `rustc --explain`](https://github.com/rust-lang/rust/pull/150895)
* [cache derive proc macro expansion with incremental query](https://github.com/rust-lang/rust/pull/145354)
* [feat: support references in reflection type info](https://github.com/rust-lang/rust/pull/151222)

#### Library
* [make `Type::of` support unsized types](https://github.com/rust-lang/rust/pull/151019)

#### Cargo
* [`git`: avoid partial oid got zero padded](https://github.com/rust-lang/cargo/pull/16511)
* [`lockfile`: switch to `resolver.lockfile-path` config](https://github.com/rust-lang/cargo/pull/16510)
* [invalidate the whole build cache when `-Zno-embed-metadata` changes](https://github.com/rust-lang/cargo/pull/16513)
* [moved build-script bins to `deps` directory](https://github.com/rust-lang/cargo/pull/16515)
* [optimize `cargo locate-project --workspace`](https://github.com/rust-lang/cargo/pull/16423)
* [store artifact deps in build unit dir](https://github.com/rust-lang/cargo/pull/16519)

#### Rustdoc
* [fix intra-doc link bugs involving type aliases and associated items](https://github.com/rust-lang/rust/pull/150586)
* [stop unconditionally evaluating the initializer of associated consts](https://github.com/rust-lang/rust/pull/151232)

#### Clippy
* [`double_comparisons`: check for expressions such as `x != y && x >= y`](https://github.com/rust-lang/rust-clippy/pull/16033)
* [`collapsible_span_lint_calls`: use `snippet_with_context` for spans that are likely to contain macro expns](https://github.com/rust-lang/rust-clippy/pull/15881)
* [`unnecessary_sort_by`: reduce suggestion diffs](https://github.com/rust-lang/rust-clippy/pull/16417)
* [add `manual_checked_ops` lint](https://github.com/rust-lang/rust-clippy/pull/16149)
* [add `manual_take` lint](https://github.com/rust-lang/rust-clippy/pull/16368)
* [do not consider binary operators as commutative by default](https://github.com/rust-lang/rust-clippy/pull/16420)
* [do not output an error if standard output is full on --help/--version](https://github.com/rust-lang/rust-clippy/pull/16412)
* [fix `unnecessary_sort_by` false negative on field access](https://github.com/rust-lang/rust-clippy/pull/16406)
* [post `needless_continue` diagnostic on the right node](https://github.com/rust-lang/rust-clippy/pull/16265)
* [skip `elidable_lifetime_names` lint for proc-macro generated code](https://github.com/rust-lang/rust-clippy/pull/16402)
* [suggest `Cstr::count_bytes` in `strlen_on_c_strings`](https://github.com/rust-lang/rust-clippy/pull/16323)

#### Rust-Analyzer
* [trigger flycheck if non-workspace files get modified](https://github.com/rust-lang/rust-analyzer/pull/21483)
* [fix false positive precedence in `(2 as i32) < 3`](https://github.com/rust-lang/rust-analyzer/pull/21465)
* [do not show sysroot dependencies in symbol search](https://github.com/rust-lang/rust-analyzer/pull/21484)
* [don't produce redundant block in `move_guard`](https://github.com/rust-lang/rust-analyzer/pull/21485)
* [ensure correct capturing of async fn params even when they use weird patterns](https://github.com/rust-lang/rust-analyzer/pull/21492)
* [fix path symbol search not respecting re-exports](https://github.com/rust-lang/rust-analyzer/pull/21464)
* [insert type vars and normalize for the type of a used `static`](https://github.com/rust-lang/rust-analyzer/pull/21491)
* [lookup flycheck by ID instead of vector index](https://github.com/rust-lang/rust-analyzer/pull/21475)
* [migrate `unwrap_block` assist to use SyntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/21458)
* [remove postcard from legacy](https://github.com/rust-lang/rust-analyzer/pull/21466)

### Rust Compiler Performance Triage

Various changes in both direction, but not much has changed overall.

Triage done by **@panstromek**.
Revision range: [840245e9..3d087e60](https://perf.rust-lang.org/?start=840245e91b90f22adf9f26d0a0cd98c2416cdef3&end=3d087e6044bddc65723bf42c76fe4cc33a0076b0&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.6%  | [0.1%, 1.6%]    | 21    |
| Regressions ❌ <br /> (secondary)  | 0.6%  | [0.0%, 2.6%]    | 113   |
| Improvements ✅ <br /> (primary)   | -0.3% | [-2.1%, -0.2%]  | 37    |
| Improvements ✅ <br /> (secondary) | -1.2% | [-29.6%, -0.1%] | 37    |
| All ❌✅ (primary)                 | 0.0%  | [-2.1%, 1.6%]   | 58    |


3 Regressions, 4 Improvements, 7 Mixed; 6 of them in rollups
40 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/98f432f4bae9972f8f320bb0df52c80546cae724/triage/2026/2026-01-19.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: `#[export_visibility = ...]` attribute](https://github.com/rust-lang/rfcs/pull/3834)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Tracking Issue for AArch64 FEAT_JSCVT](https://github.com/rust-lang/rust/issues/147555)
* [thread::scope: document how join interacts with TLS destructors](https://github.com/rust-lang/rust/pull/149482)A
* [Don't try to evaluate const blocks during constant promotion](https://github.com/rust-lang/rust/pull/150557)
* [implement PartialEq\<Vec\<U\>\> for [T; N] and &[T; N]](https://github.com/rust-lang/rust/pull/149045)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [docs(report): enhance man pages for `cargo report *`](https://github.com/rust-lang/cargo/pull/16430)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period)
* [project directors: responsibilities](https://github.com/rust-lang/leadership-council/pull/250)

*No Items entered Final Comment Period this week for
  [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html),
  [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) or
  [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Cargo mTLS registry authentication](https://github.com/rust-lang/rfcs/pull/3907)
* [RFC: obj-action style method disambiguation](https://github.com/rust-lang/rfcs/pull/3908)


## Upcoming Events

Rusty Events between 2026-01-21 - 2026-02-18 🦀

### Virtual
* 2026-01-21 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Stack Safety**](https://www.meetup.com/vancouver-rust/events/310619449/)
* 2026-01-21 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/e2ea7hxo)
* 2026-01-26 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Rust code reading and open source contribution (UTC 18:00; English)**](https://www.meetup.com/code-mavens/events/312782592/)
* 2026-01-27 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254790/)
* 2026-01-27 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & learn: Error Handling in Rust**](https://www.meetup.com/women-in-rust/events/312799344/)
* 2026-01-28 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/9h9n0dau)
* 2026-01-28 | Virtual (Lima-Perú, PE) | [Rust Perú](https://rust.pe)
    * [**Meetup: Rust from Hardware to Web (Embedded + Backend from Scratch)**](https://calendar.app.google/jc9DziLWVTUn1qNVA)    
* 2026-01-29 | Virtual (Amsterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development)
    * [**Bevy Meetup #12**](https://www.meetup.com/bevy-game-development/events/312681343/)
* 2026-01-29 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455921/)
* 2026-01-29 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Part #2 - Rusty processes, memory limits, and basic capsules**](https://www.meetup.com/charlottesville-rust-meetup/events/312326112/)
* 2026-02-04 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Getting started with Rust Part 1: Common Programming Concepts**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/312946936/)
* 2026-02-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/312187422/)
* 2026-02-07 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-02-09 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens)
    * [**Rust code reading and open source contribution (UTC 18:00; English)**](https://www.meetup.com/code-mavens/events/312985189/)
* 2026-02-10 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254789/)
* 2026-02-10 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/312799368/)
* 2026-02-11 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Getting Started with Rust Part 2: Ownership and Structs**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/312947249/)
* 2026-02-11 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/5bu9kas1)
* 2026-02-12 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455922/)
* 2026-02-12 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/312385179/)
* 2026-02-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/312951859/)
* 2026-02-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/310619456/)
* 2026-02-18 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/ir8s81ec)

### Asia
* 2026-02-05 | Seoul, KR | [Seoul Rust (Programming Language) Meetup](https://www.meetup.com/rust-seoul-meetup)
    * [**Seoul Rust Meetup**](https://www.meetup.com/rust-seoul-meetup/events/312799833/)
* 2026-02-11 | Kuala Lumpur, MY | [Rust Malaysia](https://t.me/rustlangmalaysia)
    * [**Malaysia Rust Meetup February 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfSCWkaD3LeQFleGcGsO4flR3mDKaEQknOTamGg7J7Pw9RoLw/viewform?usp=send_form)

### Europe
* 2026-01-21 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/312749221/)
* 2026-01-22 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin on location 🏳️‍🌈 - Edition 010**](https://www.meetup.com/rust-berlin/events/312962219/)
* 2026-01-26 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #17: Emily Coaca - Entwicklung des Kernels Update für TockOS**](https://rust-augsburg.github.io/meetup/Meetup_17.html)
* 2026-01-27 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Bergen #4 @ Zrch**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/312851079)
* 2026-01-27 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester January Code Night**](https://www.meetup.com/rust-manchester/events/312848708/)
* 2026-01-28 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - January 2026**](https://www.meetup.com/rust-dortmund/events/312485262/)
* 2026-01-28 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Prague (January 2026)**](https://www.meetup.com/rust-prague/events/312895761/)
* 2026-01-28 | Toulouse, FR | [Rust Toulouse](https://www.meetup.com/rust-community-toulouse)
    * [**Rust Toulouse Meetup - WASM & Elegant CLI**](https://www.meetup.com/rust-community-toulouse/events/312782796/)
* 2026-01-29 | Ostrava, CZ | [MeetUpdate Ostrava](https://www.meetup.com/meetupdate-ostrava)
    * [**MeetUpdate Ostrava #28: Rust**](https://www.meetup.com/meetupdate-ostrava/events/312747904/)
* 2026-01-31 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #23**](https://www.meetup.com/stockholm-rust/events/312919934/)
* 2026-02-04 | Darmstadt, HE, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Writing a newsletter subscription service with axum**](https://www.meetup.com/rust-rhein-main/events/312798996/)
* 2026-02-04 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 1**](https://www.meetup.com/rust-munich/events/312844145/)
* 2026-02-04 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Paul Grenyer: Beyond the Code: Designing Services That Stand the Test of Time**](https://www.meetup.com/oxford-rust-meetup-group/events/311744940/)
* 2026-02-05 | Karlsruhe, DE | [Rust Hack & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe)
    * [**Karlsruhe Rust Hack and Learn Meetup bei BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/312679714/)
* 2026-02-11 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #14 @ Optravis LLC**](https://www.meetup.com/rust-basel/events/312849882/)
* 2026-02-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/312954164/)
* 2026-02-12 | Geneva, CH | [Post Tenebras Lab](https://www.posttenebraslab.ch)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/312954164/)
* 2026-02-12 | Geneva, CH | [Post Tenebras Lab](https://www.posttenebraslab.ch)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-02-14 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 02 2026**](https://luma.com/e0uay6q5)
* 2026-02-18 - 2026-02-19 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation UK 2026**](https://www.rustnationuk.com/)

### North America
* 2026-01-21 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/312185794/)
* 2026-01-21 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Stack Safety**](https://www.meetup.com/vancouver-rust/events/310619449/)
* 2026-01-22 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Evening Boston Rust Meetup with Jiff, January 22**](https://www.meetup.com/bostonrust/events/312598080/)
* 2026-01-22 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/312692728/)
* 2026-01-24 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Davis Square Rust Lunch, Jan 24**](https://www.meetup.com/bostonrust/events/312483715/)
* 2026-01-28 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust Los Angeles: Building Git-LFS Replacements in Rust**](https://www.meetup.com/rust-los-angeles/events/312267194/)
* 2026-01-29 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/308676002/)
* 2026-01-29 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Rust Programming 101**](https://www.meetup.com/music-city-rust-developers/events/312038621/)
* 2026-01-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Northeastern Rust Lunch, Jan 31**](https://www.meetup.com/bostonrust/events/312483767/)
* 2026-02-03 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Bevy Rendering & Build Times at Amazon**](https://www.meetup.com/rust-nyc/events/312871242/)
* 2026-02-05 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Rendering the Mandelbrot set in Rust**](https://www.meetup.com/stl-rust/events/312614666/)
* 2026-02-07 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Allston Rust Lunch, Feb 7**](https://www.meetup.com/bostonrust/events/312483562/)
* 2026-02-12 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Full Stack Web Development in Rust**](https://www.meetup.com/utah-rust/events/312565489/)
* 2026-02-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcdbwb/)
* 2026-02-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/310619456/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1plbecs/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> I might suspect that if you are lumping all statically-typed languages into a single bucket without making particular distinction among them, then you might not have fully internalized the implications of union (aka Rust enum aka sum) typed data structures combined with exhaustive pattern matching.
>
> I like to call it getting "union-pilled" and it's really hard to accept otherwise statically-typed languages once you become familiar.

– [arwhatever on hacker news](https://news.ycombinator.com/item?id=45043148)

Thanks to [Colin Bennett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1748) for the suggestion!

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1qjkhiv/this_week_in_rust_635/)</small>
