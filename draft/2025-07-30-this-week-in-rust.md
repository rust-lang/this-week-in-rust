Title: This Week in Rust 610
Number: 610
Date: 2025-07-30
Category: This Week in Rust

Hello and welcome to another issue of _This Week in Rust_!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

_This Week in Rust_ is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
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

### Foundation

- [RustConf 2025 is Almost Here — Register Now!](https://rustfoundation.org/media/rustconf-2025-is-almost-here-register-now/)

### Project/Tooling Updates

- [Embedded DHT Sensor Library](https://rust-dd.com/post/building-a-rust-library-for-dht11-sensor-a-step-by-step-guide)
- [Rust eBooks Nightly: always up-to-date Rust books in EPUB, AZW3, MOBI, PDF](https://artur-sulej.github.io/rust-ebooks/)
- [git-cliff 2.10.0 is released! (a highly customizable changelog generator)](https://git-cliff.org/blog/2.10.0)
- [Nutype 0.6.2](https://github.com/greyblake/nutype/releases/tag/v0.6.2) - supports `derive_unsafe` attribute to derive arbitrary third party traits.

### Observations/Thoughts

- [Rust Async Web Framework Performance Breakthrough(9247)](https://dev.to/member_8c78b76f/rust-async-web-framework-performance-breakthrough9247-4l22)

### Rust Walkthroughs

- [Using ESP32 as a Rust Beginner](https://rust-dd.com/post/introduction-to-embedded-systems-with-rust-a-beginner-s-guide-using-esp32)
- [The Signal Protocol Explained #2: Implementing the Double Ratchet Algorithm in Rust](https://kerkour.com/signal-protocol-double-ratchet-rust)
- [Vibe coding a Rust MCP proxy in VSCode with GitHub Copilot](https://awakecoding.com/posts/vibe-coding-a-rust-mcp-proxy-in-vscode-with-github-copilot/)
- [Programming Extensible Data Types in Rust with CGP - Part 4](https://contextgeneric.dev/blog/extensible-datatypes-part-4/)

### Miscellaneous

- [100 Exercises to Learn Rust: RustRover Edition](https://blog.jetbrains.com/education/2025/07/28/rust-exercises-rustrover/)
- [Stackoverflow Survey: Rust is yet again the most admired programming language ](https://survey.stackoverflow.co/2025/technology/#admired-and-desired)

## Crate of the Week

This week's crate is [qop](https://github.com/cchexcode/qop), a standalone SQL migration tool.

Thanks to [Alexander Weber](https://users.rust-lang.org/t/crate-of-the-week/2704/1454) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

- _No calls for testing were issued this week by [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)._

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

[guidelines]: https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

428 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-07-22..2025-07-29

#### Compiler

- [avoid unnecessary `new_adt`/`new_fn_def` calls](https://github.com/rust-lang/rust/pull/144425)
- [`loop_match`: suggest extracting to a `const` item](https://github.com/rust-lang/rust/pull/143585)

#### Library

- [add `Rev::into_inner`](https://github.com/rust-lang/rust/pull/144278)
- [str: mark unstable `round_char_boundary` feature functions as const](https://github.com/rust-lang/rust/pull/144472)

#### Cargo

- [schema: Expose `IndexPackage`, the description of a package within a Registry Index](https://github.com/rust-lang/cargo/pull/15770)
- [allow using Cargo-as-a-library with gix's reqwest backend](https://github.com/rust-lang/cargo/pull/15653)
- [fix: `no-proc-macro` is overridden by subsequent edges](https://github.com/rust-lang/cargo/pull/15764)
- [timings: make graphs scalable to user's window](https://github.com/rust-lang/cargo/pull/15766)
- [use `gix` for `cargo package`](https://github.com/rust-lang/cargo/pull/15534)

#### Rustdoc

- [rustdoc: add ways of collapsing all impl blocks](https://github.com/rust-lang/rust/pull/141663)

#### Clippy

- [`cast-lossless` should not suggest when casting type is from macro input](https://github.com/rust-lang/rust-clippy/pull/15358)
- [correct help message for `arc_with_non_send_sync`](https://github.com/rust-lang/rust-clippy/pull/15332)
- [detect prefixed attributes as duplicated](https://github.com/rust-lang/rust-clippy/pull/15212)
- [fix `empty_structs_with_brackets` suggesting wrongly on generics](https://github.com/rust-lang/rust-clippy/pull/15355)
- [fix `if_then_some_else_none` false positive when require type coercion](https://github.com/rust-lang/rust-clippy/pull/15267)
- [fix `ip_constant` when call wrapped in extra parens](https://github.com/rust-lang/rust-clippy/pull/15339)
- [fix `let_unit_value` suggesting wrongly for format macros](https://github.com/rust-lang/rust-clippy/pull/15085)
- [fix `match_single_binding` wrongly handling scope](https://github.com/rust-lang/rust-clippy/pull/15060)
- [fix `module_name_repetitions` false positive on exported macros](https://github.com/rust-lang/rust-clippy/pull/15319)
- [fix `unused_async` false positive on function with `todo!`](https://github.com/rust-lang/rust-clippy/pull/15308)
- [`unnecessary_map_or`: don't add parens if the parent expr…](https://github.com/rust-lang/rust-clippy/pull/15345)

#### Rust-Analyzer

- [add ide-assist: `generate_impl_trait` for `generate_impl`](https://github.com/rust-lang/rust-analyzer/pull/19938)
- [change rename self to parameter use `Self` type](https://github.com/rust-lang/rust-analyzer/pull/20285)
- [fix `generate_trait_from_impl` whitespace after vis](https://github.com/rust-lang/rust-analyzer/pull/20297)
- [fix fold doc comment for multiline param list fn](https://github.com/rust-lang/rust-analyzer/pull/20302)
- [consider all produced artifacts for proc-macro dylib search](https://github.com/rust-lang/rust-analyzer/pull/20319)
- [fix incorrect build script version check](https://github.com/rust-lang/rust-analyzer/pull/20317)
- [fix runnables extra env not substituting env vars](https://github.com/rust-lang/rust-analyzer/pull/20313)
- [ignore `Destruct` bounds again](https://github.com/rust-lang/rust-analyzer/pull/20318)
- [parse `for<'a> [const]`](https://github.com/rust-lang/rust-analyzer/pull/20281)
- [use `TempDir` for copied lockfiles](https://github.com/rust-lang/rust-analyzer/pull/20290)

### Rust Compiler Performance Triage

A week with lots of mixed results, including a few benchmarks that appear to be
newly bimodal, which made some of the results look closer to noise than signal.
Overall, however, the week ended as a slight improvement.

Triage done by **@simulacrum**.
Revision range: [3f9f20f7..e3514bde](https://perf.rust-lang.org/?start=3f9f20f71dd945fe7d044e274094a53c90788269&end=e3514bde96d2d13586337a48db77fa64b850d249&absolute=false&stat=instructions%3Au)

1 Regressions, 2 Improvements, 9 Mixed; 2 of them in rollups
38 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025/2025-07-28.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

- _No RFCs were approved this week._

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

- [Allow the global allocator to use thread-local storage and std::thread::current()](https://github.com/rust-lang/rust/pull/144465)
- [Tracking Issue for str::{floor, ceil}\_char_boundary](https://github.com/rust-lang/rust/issues/93743)
- [Check coroutine upvars in dtorck constraint](https://github.com/rust-lang/rust/pull/144156)
- [Tracking Issue for arithmetic and certain bitwise ops on `AtomicPtr`](https://github.com/rust-lang/rust/issues/99108)
- [Add lint against dangling pointers from local variables](https://github.com/rust-lang/rust/pull/144322)
- [`apply_member_constraints`: fix placeholder check](https://github.com/rust-lang/rust/pull/142071)
- [Remove the `#[no_sanitize]` attribute in favor of `#[sanitize(xyz = "on|off")`]](https://github.com/rust-lang/rust/pull/142681)
- [Port `#[should_panic]` to the new attribute parsing infrastructure](https://github.com/rust-lang/rust/pull/143808)
- [emit `StorageLive` and schedule `StorageDead` for `let` - `else`'s bindings after matching](https://github.com/rust-lang/rust/pull/143028)
- [lower pattern bindings in the order they're written and base drop order on primary bindings' order](https://github.com/rust-lang/rust/pull/143764)
- [Upgrade semicolon_in_expressions_from_macros from warn to deny](https://github.com/rust-lang/rust/pull/144369)
- [Stabilize const TypeId::of](https://github.com/rust-lang/rust/pull/144133)
- [Mark all deprecation lints in name resolution as deny-by-default and report-in-deps](https://github.com/rust-lang/rust/pull/143929)
- [Tracking Issue for arithmetic that panics on overflow ( `strict_*` operations)](https://github.com/rust-lang/rust/issues/118260)
- [[rustdoc] Display unsafe attrs with edition 2024 `unsafe()` wrappers.](https://github.com/rust-lang/rust/pull/143662)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),

- [Demote `x86_64-apple-darwin` from Tier 1 to Tier 2 with host tools](https://github.com/rust-lang/rfcs/pull/3841)
- [RFC: enable `derive(From)` for single-field structs](https://github.com/rust-lang/rfcs/pull/3809)

_No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)._

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

- [new] [Demote x86_64-apple-darwin from Tier 1 to Tier 2 with host tools](https://github.com/rust-lang/rfcs/pull/3841)

## Upcoming Events

Rusty Events between 2025-07-30 - 2025-08-27 🦀

### Virtual

- 2025-07-30 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
  - [**Jan Arends: How to turn Spaghetti Code into a Gourmet Architecture**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/310006696)
- 2025-07-31 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
  - [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820306)
- 2025-08-02 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
  - [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763838567)
- 2025-08-03 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
  - [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/bhctrtyhclbfb)
- 2025-08-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
  - [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyhclbjb)
- 2025-08-10 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
  - [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/bhctrtyhclbnb)
- 2025-08-12 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
  - [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361531)
- 2025-08-14 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
  - [**August, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/307698880)
- 2025-08-14 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
  - [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820307)
- 2025-08-17 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
  - [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002458)
- 2025-08-19 | Virtual (Santa Clara, CA, US) | [UCSC Extension Community](https://www.meetup.com/ucsc-extension-community/events/)
  - [**Programming with Rust**](https://www.meetup.com/ucsc-extension-community/events/310108013)
- 2025-08-19 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
  - [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/306757756)
- 2025-08-20 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
  - [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731032)
- 2025-08-21 | Virtual (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina)
  - [**Agosto de Protocol Buffers!**](https://www.meetup.com/rust-argentina/events/310019573)
- 2025-08-21 | Virtual (London, UK) | [Conf42: Online Tech Events](https://www.meetup.com/conf42/events/)
  - [**Conf42 Rustlang 2025**](https://www.meetup.com/conf42/events/305437705)
- 2025-08-21 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
  - [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567875)
- 2025-08-24 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
  - [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002461)
- 2025-08-26 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
  - [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361442)
- 2025-09-02 - 2025-09-05 | Hybrid (Seattle, WA, US) | [RustConf](https://rustconf.com/)
  - [**RustConf 2025**](https://rustconf.com/)

### Europe

- 2025-07-30 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group)
  - [**Rust Meetup @ BlockTech**](https://www.meetup.com/rust-amsterdam-group/events/308548455)
- 2025-07-31 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
  - [**Rust Meetup #14: Prof. Dr. Claudia Meitinger - Embassy - Möglichkeiten und Herausforderungen im Modul "Interdisciplinary Project"**](https://rust-augsburg.github.io/meetup/Meetup_14.html)
- 2025-07-31 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
  - [**Rust meetup #59**](https://www.meetup.com/copenhagen-rust-community/events/310147999)
- 2025-08-06 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
  - [**Rust Girona Hack & Learn 08 2025**](https://lu.ma/eoydaar9)
- 2025-08-06 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
  - [**Alastair Harrison: Version control for the agentic age.**](https://www.meetup.com/oxford-rust-meetup-group/events/310101048)
- 2025-08-13 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup)
  - [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/310014719)
- 2025-08-13 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
  - [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944036)
- 2025-08-16 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
  - [**Rust Embedded - Workshop #4 @letsboot**](https://www.meetup.com/rust-basel/events/309894848)
- 2025-08-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
  - [**Hack Night - Robot Edition**](https://www.meetup.com/rust-aarhus/events/310039453)
- 2025-08-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
  - [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592249)
- 2025-08-21 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
  - [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310062129)

### North America

- 2025-07-31 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
  - [**Rust-Atl**](https://www.meetup.com/rust-atl/events/308675947)
- 2025-07-31 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
  - [**Rust in Web3 Developer Group**](https://www.meetup.com/rust-los-angeles/events/310240265)
- 2025-08-01 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup)
  - [**Rust Lunch - Loop Edition**](https://www.meetup.com/chicago-rust-meetup/events/310199297)
- 2025-08-02 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
  - [**Central Cambridge Rust Lunch, Aug 2**](https://www.meetup.com/bostonrust/events/310106288)
- 2025-08-05 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
  - [**Rust NYC: Validating/Optimizing DB Queries w/Types & Rust in Enterprise AI**](https://www.meetup.com/rust-nyc/events/310107945)
- 2025-08-07 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
  - [**August Monthly Social**](https://www.meetup.com/rust-montreal/events/310259905)
- 2025-08-07 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
  - [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/310030338)
- 2025-08-07 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
  - [**macros!**](https://www.meetup.com/stl-rust/events/306648747)
- 2025-08-08 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
  - [**Northeastern Rust Lunch, Aug 8**](https://www.meetup.com/bostonrust/events/310106298)
- 2025-08-12 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
  - [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/308284338)
- 2025-08-14 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
  - [**August, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/307698880)
- 2025-08-14 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
  - [**Programming a Fighting Robot in Rust with Rex Magana**](https://www.meetup.com/utah-rust/events/310053631)
- 2025-08-18 | Denver, CO, US | [FOSS Rust Colorado](https://mobilizon.us/@foss_rust_colorado/events)
  - [**FOSS Rust Hack Night**](https://mobilizon.us/events/9092695a-89f0-40fa-b3d0-50072827b0ec)
- 2025-08-20 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
  - [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731032)
- 2025-08-21 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
  - [**Rust on Bare Metal Series 2 : Place Holder**](https://www.meetup.com/music-city-rust-developers/events/304333117)
- 2025-08-23 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
  - [**Somerville Union Square Rust Lunch, Aug 23**](https://www.meetup.com/bostonrust/events/310106302)
- 2025-08-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
  - [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310205991)
- 2025-09-02 - 2025-09-05 | Hybrid (Seattle, WA, US) | [RustConf](https://rustconf.com/)
  - [**RustConf 2025**](https://rustconf.com/)

### Oceania

- 2025-08-11 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group)
  - [**Christchurch Rust Meetup**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/308880707)
- 2025-08-26 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
  - [**August Meetup**](https://www.meetup.com/rust-canberra/events/308746519)
- 2025-08-27 - 2025-08-30 | Wellington, NZ | [Rust Forge](https://rustforgeconf.com/)
  - [**Rust Forge**](https://rustforgeconf.com/)

### South America

- 2025-08-07 | Montevideo, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay)
  - [**Rust Uruguay meetup de Agosto**](https://www.meetup.com/rust-uruguay/events/310004109)

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

> The same compute logic runs on all targets, written entirely in regular Rust. No shader or kernel languages are used.

– [Christian Legnitto on the rust-gpu blog](https://rust-gpu.github.io/blog/2025/07/25/rust-on-every-gpu/) showing off a demo compiling Rust to all major GPU platforms + web.

Despite a lack of suggestions, llogiq is remarkably pleased with his choice.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

_This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)_

_Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)_

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
