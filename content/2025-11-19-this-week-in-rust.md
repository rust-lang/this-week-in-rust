Title: This Week in Rust 626
Number: 626
Date: 2025-11-19
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
* [Launching the 2025 State of Rust Survey](https://blog.rust-lang.org/2025/11/17/launching-the-2025-state-of-rust-survey)
* [Google Summer of Code 2025 results](https://blog.rust-lang.org/2025/11/18/gsoc-2025-results/)
* [Project goals update — October 2025](https://blog.rust-lang.org/2025/11/19/project-goals-update-october-2025/)
* [Project goals update — September 2025](https://blog.rust-lang.org/2025/11/19/Project-Goals-2025-September-Update/)

### Newsletters
* [Scientific Computing in Rust #12 (November 2025)](https://scientificcomputing.rs/monthly/2025-11)
* [Secure-by-design firmware development with Wasefire](https://opensource.googleblog.com/2025/11/secure-by-design-firmware-development-with-wasefire.html)
* [Rust Trends Issue #72: From Experimental to Enterprise: Rust's Production Moment](https://rust-trends.com/newsletter/experimental-to-enterprise-rust-production)

### Project/Tooling Updates
* [GuardianDB v0.11.28](https://www.willsearch.com.br/blog/2025/11/18/whats-new-in-guardiandb-v0-11-18/)
* [The current state of Linux architecture support](https://lwn.net/SubscriberLink/1045363/60611dc5ec3f7099/)

### Observations/Thoughts
* [audio] [Netstack.FM Episode 14 –  Roto And Cascade with Terts and Arya from NLnet Labs](https://netstack.fm/#episode-14)
* [Improving the Incremental System in the Rust Compiler](https://blog.goose.love/posts/improving-the-incremental-system-in-the-rust-compiler/)
* [Truly First-Class Custom Smart Pointers](https://nadrieril.github.io/blog/2025/11/11/truly-first-class-custom-smart-pointers.html)
* [Pinning is a kind of static borrow](https://nadrieril.github.io/blog/2025/11/12/pinning-is-a-kind-of-static-borrow.html)
* [Rust in Android: move fast and fix things](https://security.googleblog.com/2025/11/rust-in-android-move-fast-fix-things.html)
* [Match it again Sam](https://www.sminez.dev/match-it-again-sam/)
* [Humanity is stained by the sins of C and no LLM can rewrite them away to Rust](https://kirancodes.me/posts/log-sins-of-c.html)
* [UV and Ruff: Turbocharging Python Development with Rust-Powered Tools](https://www.devtoolsacademy.com/blog/uv-and-ruff-turbocharging-python-development-with-rust-powered-tools/)
* [A Function Inliner for Wasmtime and Cranelift](https://fitzgen.com/2025/11/19/inliner.html)

### Rust Walkthroughs
- [Rust Unit Tests: Assertion libraries](https://jorgeortiz.dev/posts/rust_unit_testing_assertion_libraries/)
- [Rust Unit Tests: Using a mocking library](https://jorgeortiz.dev/posts/rust_unit_testing_mocking_library/)
* [A Practical Guide to Transitioning to Memory-Safe Languages](https://queue.acm.org/detail.cfm?id=3773096)
* [Building WebSocket Protocol in Apache Iggy using io_uring and Completion Based I/O Architecture](https://iggy.apache.org/blogs/2025/11/17/websocket-io-uring/)
* [Building serverless applications with Rust on AWS Lambda](https://aws.amazon.com/blogs/compute/building-serverless-applications-with-rust-on-aws-lambda/)
* [Disallow code usage with a custom `clippy.toml`](https://www.schneems.com/2025/11/19/find-accidental-code-usage-with-a-custom-clippytoml/)

### Miscellaneous
* [Absurd Rust? Never!](https://academy.fpblock.com/blog/absurd-rust-never/?share=1)
* [video] [Linus Torvalds — Speaks up on the Rust Divide and saying NO](https://www.youtube.com/watch?v=amyKC9lJe3Q)
* [October 2025 Rust Jobs Report](https://filtra.io/rust/jobs-report/oct-25)
* [Rust’s Strategic Advantage](https://sysid.github.io/rusts-strategic-advantage/)

## Crate of the Week

This week's crate is [cargo cat](crates.io/crates/cat-ascii-faces), a cargo-subcommand to put a random ascii cat face on your terminal.

Thanks to [Alejandra Gonzáles](https://users.rust-lang.org/t/crate-of-the-week/2704/1490) for the self-suggestion!

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
  [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) or
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

* [GuardianDB - Create and translate documentation to English](https://github.com/wmaslonek/guardian-db/issues/3)
* [GuardianDB - Increase test coverage (currently 13%)](https://github.com/wmaslonek/guardian-db/issues/4)
* [GuardianDB - Create cohesive usage examples](https://github.com/wmaslonek/guardian-db/issues/5)
* [GuardianDB - Backend Iroh IPFS Node](https://github.com/wmaslonek/guardian-db/issues/6)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.
* [**Rustikon 2026**](https://sessionize.com/rustikon-2026/) \| CFP closes: 2025-11-24 23:59 \| Warsaw, Poland \| Event: 2025-03-19–2025-03-20 [Event website](https://www.rustikon.dev/)<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [**TokioConf 2026**](https://tokio.rs/blog/2025-09-26-announcing-tokio-conf-cfp)| CFP closes 2025-12-08 | Portland, Oregon, USA | 2026-04-20
* [**RustWeek 2026**](https://sessionize.com/rustweek-2026/)| CFP closes 2025-12-31 | Utrecht, The Netherlands | 2026-05-19 - 2026-05-20

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

427 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-11-11..2025-11-18

#### Compiler
* [add new `function_casts_as_integer` lint](https://github.com/rust-lang/rust/pull/141470)
* [miri: initial implementation of wildcard provenence for tree borrows](https://github.com/rust-lang/miri/pull/4630)
#### Library
* [new `format_args!()` and `fmt::Arguments` implementation](https://github.com/rust-lang/rust/pull/148789)
* [`vec_recycle`: implementation](https://github.com/rust-lang/rust/pull/148416)
* [implement `Read::read_array`](https://github.com/rust-lang/rust/pull/148850)
* [stabilize `char_max_len`](https://github.com/rust-lang/rust/pull/145610)
* [stabilize `duration_from_nanos_u128`](https://github.com/rust-lang/rust/pull/148587)
* [stabilize `extern_system_varargs`](https://github.com/rust-lang/rust/pull/145954)
* [stabilize `vec_into_raw_parts`](https://github.com/rust-lang/rust/pull/148827)
* [constify `ManuallyDrop::take`](https://github.com/rust-lang/rust/pull/148752)
* [constify `mem::take`](https://github.com/rust-lang/rust/pull/148757)
* [remove `rustc_inherit_overflow_checks` from `position()` in slice iterators](https://github.com/rust-lang/rust/pull/148944)
#### Cargo
* [`cli`: add support for completing `--config` values in Bash](https://github.com/rust-lang/cargo/pull/16245)
* [`tree`: support long forms for --format variables](https://github.com/rust-lang/cargo/pull/16204)
* [`config`: fallback to non-canonical path for workspace-path-hash](https://github.com/rust-lang/cargo/pull/16248)
* [`manifest`: point out when a key belongs to config](https://github.com/rust-lang/cargo/pull/16256)
* [`package`: all tar entries timestamp be the same](https://github.com/rust-lang/cargo/pull/16242)
* [do not lock the artifact-dir for check builds](https://github.com/rust-lang/cargo/pull/16230)
* [add unstable rustc-unicode flag](https://github.com/rust-lang/cargo/pull/16243)
#### Rustdoc
* [Fix invalid jump to def macro link generation](https://github.com/rust-lang/rust/pull/148080)
* [don't ignore path distance for doc aliases](https://github.com/rust-lang/rust/pull/147701)
* [don't pass `RenderOptions` to `DocContext`](https://github.com/rust-lang/rust/pull/147832)
* [microoptimize `render_item,` move stuff out of common path](https://github.com/rust-lang/rust/pull/148877)
* [quality of life changes](https://github.com/rust-lang/rust/pull/148466)
#### Clippy
* [`ok_expect`: add autofix](https://github.com/rust-lang/rust-clippy/pull/15867)
* [{`unnecessary`, `panicking`}`_unwrap`: lint field accesses](https://github.com/rust-lang/rust-clippy/pull/15949)
* [`equatable_if_let`: don't suggest `=` in const context](https://github.com/rust-lang/rust-clippy/pull/16092)
* [`rc_buffer`: don't touch the path to `Rc`/`Arc` in the suggestion](https://github.com/rust-lang/rust-clippy/pull/15803)
* [`incompatible_msrv`: don't check the contents of any `std` macro](https://github.com/rust-lang/rust-clippy/pull/16083)
* [add a `doc_paragraphs_missing_punctuation` lint](https://github.com/rust-lang/rust-clippy/pull/15758)
* [fix `single_range_in_vec_init` false positive for explicit `Range`](https://github.com/rust-lang/rust-clippy/pull/16043)
* [fix `sliced_string_as_bytes` false positive with a `RangeFull`](https://github.com/rust-lang/rust-clippy/pull/15873)
* [fix website history interactions](https://github.com/rust-lang/rust-clippy/pull/16060)
* [rework `missing_docs_in_private_items`](https://github.com/rust-lang/rust-clippy/pull/14741)
#### Rust-Analyzer
* [fix removed feature `doc_auto_cfg` for `smol_str` lib](https://github.com/rust-lang/rust-analyzer/pull/21021)

### Rust Compiler Performance Triage

Positive week, most notably because of the new format_args!() and fmt::Arguments implementation from [#148789](https://github.com/rust-lang/rust/pull/148789). Another notable improvement came from moving some computations from one compiler stage to another to save memory and unnecessary tree traversals in [#148706](https://github.com/rust-lang/rust/pull/148706)

Triage done by **@panstromek**.
Revision range: [055d0d6a..6159a440](https://perf.rust-lang.org/?start=055d0d6aaf937cc11b3d2a5b5725972723b7f3c6&end=6159a44067ebce42b38f062cc7df267a1348e092&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 1.6%  | [0.2%, 5.6%]    | 11    |
| Regressions ❌ <br /> (secondary)  | 0.3%  | [0.1%, 1.1%]    | 26    |
| Improvements ✅ <br /> (primary)   | -0.8% | [-4.5%, -0.1%]  | 161   |
| Improvements ✅ <br /> (secondary) | -1.4% | [-38.1%, -0.1%] | 168   |
| All ❌✅ (primary)                 | -0.6% | [-4.5%, 5.6%]   | 172   |


2 Regressions, 4 Improvements, 10 Mixed; 4 of them in rollups
48 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/8cb481daaea8c43b1d694184b0a58fa93001ece6/triage/2025/2025-11-19.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [Stabilize `-Zremap-path-scope`](https://github.com/rust-lang/rust/pull/147611)
* [misc coercion cleanups and handle safety correctly](https://github.com/rust-lang/rust/pull/148602)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)

* [Contracts: primitive ownership assertions: `owned` and `block`](https://github.com/rust-lang/compiler-team/issues/942)

*No Items entered Final Comment Period this week for
  [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) or 
  [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2025-11-19 - 2025-12-17 🦀

### Virtual
* 2025-11-19 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/309926564/)
* 2025-11-19 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/l2xukapz)
* 2025-11-20 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**November, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351673/)
* 2025-11-20 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046642/)
* 2025-11-20 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock, a Rust based Operating System Part #1**](https://www.meetup.com/charlottesville-rust-meetup/events/311705915/)
* 2025-11-23 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109183/)
* 2025-11-25 | Virtual (Boulder, CO, US) | [Boulder Elixir](https://www.meetup.com/boulder-elixir/events/)
    * [**Integrating Elixir and Apache DataFusion with Rustler**](https://www.meetup.com/boulder-elixir/events/310996627/)
* 2025-11-25 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361446/)
* 2025-11-25 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Data-Intensive Systems in Rust: Safety, Speed, Concurrency**](https://www.meetup.com/women-in-rust/events/311534469/)
* 2025-11-26 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.github.io)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/q5tjirkt)
* 2025-11-27 | Virtual (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Meet de Noviembre - Async Runtimes, parte 2!**](https://www.meetup.com/rust-argentina/events/312061828/)
* 2025-11-30 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109188/)
* 2025-12-02 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Advent of Code - Kick Off!**](https://www.meetup.com/women-in-rust/events/311068648/)
* 2025-12-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 2025-12-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/311886445/)
* 2025-12-04 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046643/)
* 2025-12-05 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Kick-Off!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/311103307/)
* 2025-12-06 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763878687)
* 2025-12-07 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Finale**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/311103329/)
* 2025-12-09 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361537/)
* 2025-12-10 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/li5de4ts)
* 2025-12-11 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**December, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351054/)
* 2025-12-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/310728572/)
* 2025-12-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/310002338/)
* 2025-12-17 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/309926569/)
* 2025-12-17 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/6v2rorp3)

### Asia
* 2025-11-20 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup/)
    * [**Tracing Rust at Scale**](https://www.meetup.com/tokyo-rust-meetup/events/311817069/)

### Europe
* 2025-11-19 | Ostrava, CZ | [TechMeetup Ostrava](https://www.meetup.com/techmeetupostrava/)
    * [**QA Circle**](https://www.meetup.com/techmeetupostrava/events/311581090/)
* 2025-11-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Social Night**](https://www.meetup.com/rust-aarhus/events/311502123/)
* 2025-11-20 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group)
    * [**Rust Meetup @ Monumental X Zed**](https://www.meetup.com/rust-amsterdam-group/events/311829267/)
* 2025-11-20 | Luzern, CH | [Rust Luzern](https://www.meetup.com/rust-luzern/)
    * [**2025 Rust Talks Luzern #3: Crate Walkthroughs @ Noser Engineering AG**](https://www.meetup.com/rust-luzern/events/311410681/)
* 2025-11-26 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern)
    * [**2025 Rust Talks Bern #5 @Source Engineers**](https://www.meetup.com/rust-bern/events/311792568/)
* 2025-11-27 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #16: Christian Meusel - Oxidizing Step by Step**](https://rust-augsburg.github.io/meetup/Meetup_16.html)
* 2025-11-27 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust)
    * [**19th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/311787736/)
* 2025-11-27 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Exotically Sized Types, and Rust in Space at Spire!**](https://www.meetup.com/rust-and-friends/events/311753411/)
* 2025-11-28 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Prague @ Barclays**](https://www.meetup.com/rust-prague/events/311846118/)
* 2025-12-03 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 12 2025**](https://luma.com/8ncu1p8l)
* 2025-12-03 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Rust/ACCU meetup.**](https://www.meetup.com/oxford-rust-meetup-group/events/311994790/)
* 2025-12-08 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #81**](https://www.meetup.com/rust-paris/events/312004357/)
* 2025-12-10 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2025 / 4 - Hacking Evening**](https://www.meetup.com/rust-munich/events/307105932/)
* 2025-12-10 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944053/)
* 2025-12-16 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592258/)

### North America
* 2025-11-19 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/309926564/)
* 2025-11-20 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**November, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351673/)
* 2025-11-20 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust)
    * [**Monthly Rust Meetup: Embedded Primer & Free-Coding**](https://www.meetup.com/spokane-rust/events/311863560/)
* 2025-11-23 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Inman Rust Lunch & Hackathon, Nov 23**](https://www.meetup.com/bostonrust/events/311917854/)
* 2025-11-26 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310457310/)
* 2025-11-26 | Phoenix, AZ, US | [Desert Rust](https://www.meetup.com/desert-rustaceans)
    * [**Booze.rs**](https://www.meetup.com/desert-rustaceans/events/312000222/)
* 2025-11-27 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/jqvvttyhcpbkc/)
* 2025-11-29 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Harvard Square Rust Lunch, Nov 29**](https://www.meetup.com/bostonrust/events/311917256/)
* 2025-12-02 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Talk December**](https://www.meetup.com/chicago-rust-meetup/events/311736848/)
* 2025-12-04 | México City, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Optimizando rendimiento de Python con Rust**](https://www.meetup.com/rust-mx/events/312052780/)
* 2025-12-04 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Actix Web Unleashed: Mastering State, Security, and Scalable Handlers in Rust**](https://www.meetup.com/stl-rust/events/311396006/)
* 2025-12-05 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC Unconf 2025: Our Biggest Event Yet!**](https://www.meetup.com/rust-nyc/events/311757146/)
* 2025-12-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Downtown Rust Lunch, Dec 6**](https://www.meetup.com/bostonrust/events/311917263/)
* 2025-12-11 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**December, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351054/)
* 2025-12-11 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Competetive Robotics with Rust**](https://www.meetup.com/utah-rust/events/311613704/)
* 2025-12-11 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust December Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/312009598/)
* 2025-12-13 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Alewife Rust Lunch, Dec 13**](https://www.meetup.com/bostonrust/events/311917267/)
* 2025-12-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/308865807/)
* 2025-12-17 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/309926569/)

### Oceania
* 2025-12-11 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Rust Meetup Dec 2025**](https://www.meetup.com/rust-brisbane/events/312027415/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1ow6s90/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> We adopted Rust for its security and are seeing a 1000x reduction in memory safety vulnerability density compared to Android’s C and C++ code. But the biggest surprise was Rust's impact on software delivery. With Rust changes having a 4x lower rollback rate and spending 25% less time in code review, the safer path is now also the faster one.

– [Jeff Vander Stoep on the Google Android blog](https://security.googleblog.com/2025/11/rust-in-android-move-fast-fix-things.html)

Thanks to [binarycat](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1728) for the suggestion!

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1p1q0zt/this_week_in_rust_626/)</small>
