Title: This Week in Rust 624
Number: 624
Date: 2025-11-05
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
* [Project goals for 2025H2 | Rust Blog](https://blog.rust-lang.org/2025/10/28/project-goals-2025h2/)
* [Announcing Rust 1.91.0 | Rust Blog](https://blog.rust-lang.org/2025/10/30/Rust-1.91.0/)

### Foundation
* [Announcing the Rust Foundation Maintainers Fund - The Rust Foundation](https://rustfoundation.org/media/announcing-the-rust-foundation-maintainers-fund/)

### Newsletters
* [Rust Trends Issue #71: Production Rust at Internet Scale](https://rust-trends.com/newsletter/production-rust-internet-scale)
* [This Month in Redox - October 2025 - Redox - Your Next(Gen) OS](https://www.redox-os.org/news/this-month-251031/)

### Project/Tooling Updates
* [Developing UEFI in Rust with Patina](https://opendevicepartnership.github.io/patina/introduction.html)
* [Announcing cgp-serde: A modular serialization library for Serde powered by CGP](https://contextgeneric.dev/blog/cgp-serde-release/)
* [CGP v0.6.0 Release - Major ergonomic improvements for provider and context implementations](https://contextgeneric.dev/blog/v0-6-0-release/)
* [`esp-hal` 1.0.0 release announcement](https://developer.espressif.com/blog/2025/10/esp-hal-1/)

### Observations/Thoughts
* [Ghosts in the Compilation](https://predr.ag/blog/ghosts-in-the-compilation/)
* [Patterns for Defensive Programming in Rust | corrode Rust Consulting](https://corrode.dev/blog/defensive-programming/)
* [Cloudflare with Edward Wang & Kevin Guthrie - Rust in Production](https://corrode.dev/podcast/s05e03-cloudflare/)
* [Neural Networks with Candle](https://pranitha.dev/posts/neural-networks-with-candle/)
* [Async Rust - Part 18 of Idiomatic Rust in Simple Steps](https://www.youtube.com/watch?v=_x61dSP4ZKM)
* [The state of SIMD in Rust in 2025](https://shnatsel.medium.com/the-state-of-simd-in-rust-in-2025-32c263e5f53d)
* [Rust is eating the world: From embedded firmware to cross-platform applications, databases and big servers](https://kerkour.com/rust-is-eating-the-world)
* [video] [Building Next Generation Rail Systems With Rust: Tom Praderio of Parallel](https://www.youtube.com/watch?v=Ga2YDNDHQsM)
* [video] [Are we desktop yet? - Victoria Brekenfeld | EuroRust 2025](https://www.youtube.com/watch?v=0ZrD2oYabn4)
* [audio] [Netstack.FM Episode 12 – Oxide Networking with Ryan Goodfellow](https://netstack.fm/#episode-12)

### Rust Walkthroughs
* [Rust Unit Testing Test Doubles: Fakes](https://jorgeortiz.dev/posts/rust_unit_testing_test_doubles_fake/)
* [Building a Coding Agent in Rust: Implementing Chat Feature](https://blog.0xshadow.dev/posts/coding-agent-in-rust/coding-agent-in-rust-chat/)
* [Image Classification in Rust with Tch-rs (Torch bindings)](https://www.djamware.com/post/690864cde87a290bcfebeebe/image-classification-in-rust-with-tchrs-torch-bindings)
* [Inside Rust's std and parking_lot mutexes - who wins?](https://blog.cuongle.dev/p/inside-rusts-std-and-parking-lot-mutexes-who-win)
* [Positron - Only the Future Is Certain](https://positron.solutions/articles/building-nicely-with-rust-and-nix)
* [Getting Started with Rust and ClickHouse](https://www.svix.com/blog/getting-started-with-rust-and-clickhouse/)
* [`SocketAddrV6` is not roundtrip serializable · sunshowers](https://sunshowers.io/posts/socketaddrv6-not-roundtrip/)
* [Building Next Generation Rail Systems With Rust: Tom Praderio of Parallel](https://filtra.io/rust/interviews/parallel-nov-25)
* [Diesel Workshop Slides from RustWeek2025](https://blog.weiznich.de/rustweek_2025.html#/title-slide)
* [video] [Building Coding Agent in Rust | Implement Chat CLI | Part-2](https://www.youtube.com/watch?v=N21aCBICHLU)

### Miscellaneous
* [Can-t stop till you get enough](https://cant.bearblog.dev/can-t-stop-till-you-get-enough/)

## Crate of the Week

This week's crate is [dioxus](https://docs.rs/dioxus), a framework for building cross-platform apps.

Thanks to [llogiq](https://users.rust-lang.org/t/crate-of-the-week/2704/1484) for the suggestion!

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

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
* [Motor OS - Improve rush (the shell in Motor OS)](https://github.com/moturus/motor-os/issues/33)
* [Motor OS - Make imager configurable](https://github.com/moturus/motor-os/issues/24)
* [Motor OS - Port libc/llvm/rustc](https://github.com/moturus/motor-os/issues/26)
* [Diesel - Improve documentation for Postgres loading modes](https://github.com/diesel-rs/diesel/issues/4764)
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [**TokioConf 2026**](https://tokio.rs/blog/2025-09-26-announcing-tokio-conf-cfp)| CFP closes 2025-12-08 | Portland, Oregon, USA | 2026-04-20

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

480 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-10-28..2025-11-04

#### Compiler
* [`rustc_codegen`: fix musttail returns for cast/indirect ABIs](https://github.com/rust-lang/rust/pull/148240)
* [accept trivial consts based on trivial consts](https://github.com/rust-lang/rust/pull/148182)
* [add LLVM range attributes to slice length parameters](https://github.com/rust-lang/rust/pull/148350)
* [adjust successor iterators](https://github.com/rust-lang/rust/pull/148157)
* [allow check builds with binaries for the dummy codegen backend](https://github.com/rust-lang/rust/pull/148299)
* [allow codegen backends to indicate which crate types they support](https://github.com/rust-lang/rust/pull/148177)
* [better warning message for crate type unsupported by codegen backend](https://github.com/rust-lang/rust/pull/148400)
* [contract variable declarations](https://github.com/rust-lang/rust/pull/144444)
* [fix deferred cast checks using the wrong body for determining constness](https://github.com/rust-lang/rust/pull/148287)
* [fix types being marked as dead when they are inferred generic arguments](https://github.com/rust-lang/rust/pull/148262)
* [implement pin-project in pattern matching for `&pin mut|const T`](https://github.com/rust-lang/rust/pull/139751)
* [miscellaneous const-generics-related fixes](https://github.com/rust-lang/rust/pull/147642)
* [remove `QPath::LangItem`](https://github.com/rust-lang/rust/pull/148193)
* [stabilize -Zno-jump-tables into -Cjump-tables=bool](https://github.com/rust-lang/rust/pull/145974)
* [when a trait isn't implemented, but another similar impl is found, point at it](https://github.com/rust-lang/rust/pull/145640)

#### Library
* [add `from_fn_ptr` to `Waker` and `LocalWaker`](https://github.com/rust-lang/rust/pull/146057)
* [add SliceIndex wrapper types Last and `Clamp<Idx>`](https://github.com/rust-lang/rust/pull/146260)
* [constify Range functions](https://github.com/rust-lang/rust/pull/146573)
* [constify trait aliases](https://github.com/rust-lang/rust/pull/144291)
* [implement VecDeque `extend_from_within` and `prepend_from_within`](https://github.com/rust-lang/rust/pull/147161)
* [implement `VecDeque::extract_if`](https://github.com/rust-lang/rust/pull/147780)
* [implement `strip_circumfix` lib feature](https://github.com/rust-lang/rust/pull/147947)
* [smart pointer `(try_)map`](https://github.com/rust-lang/rust/pull/144420)
* [stabilize `fmt::from_fn`](https://github.com/rust-lang/rust/pull/145915)

#### Cargo
* [`build-analysis`: JSONL-based logging infra](https://github.com/rust-lang/cargo/pull/16150)
* [`build-analysis`: emit timing-info log](https://github.com/rust-lang/cargo/pull/16179)
* [`config-include`: add optional field support](https://github.com/rust-lang/cargo/pull/16180)
* [`config-include`: support inline and array of tables](https://github.com/rust-lang/cargo/pull/16174)
* [support array of any types in Cargo config](https://github.com/rust-lang/cargo/pull/16103)

#### Rustdoc
* [search: Include extern crates when filtering on `import`](https://github.com/rust-lang/rust/pull/148301)
* [Include attribute and derive macros when filtering on "macros"](https://github.com/rust-lang/rust/pull/148176)
* [use configured target modifiers when collecting doctests](https://github.com/rust-lang/rust/pull/148068)

#### Clippy
* [`search_is_some`: Fix when the closure spans multiple lines](https://github.com/rust-lang/rust-clippy/pull/15902)
* [`double_parens`: don't lint in proc-macros](https://github.com/rust-lang/rust-clippy/pull/15939)
* [`let_and_return`: disallow `_any_` text between let and return](https://github.com/rust-lang/rust-clippy/pull/16006)
* [`use_debug`: don't get confused by nested `Debug` impls](https://github.com/rust-lang/rust-clippy/pull/15946)
* [`incompatible_msrv`: Don't check the const MSRV for uncalled functions](https://github.com/rust-lang/rust-clippy/pull/15795)
* [`manual_unwrap_or(_default)`: don't lint if not safe to move scrutinee](https://github.com/rust-lang/rust-clippy/pull/15817)
* [extend `needless_collect`](https://github.com/rust-lang/rust-clippy/pull/14361)
* [fix `replace_box` false positive when the box is moved](https://github.com/rust-lang/rust-clippy/pull/15984)
* [improve doc comment code language tag parsing, don't use a full parser](https://github.com/rust-lang/rust-clippy/pull/15967)

#### Rust-Analyzer
* [add ide-assist: `convert_range_for_to_while`](https://github.com/rust-lang/rust-analyzer/pull/20565)
* [support memory profiling with dhat](https://github.com/rust-lang/rust-analyzer/pull/20927)
* [fix missing other assoc items for `generate_blanket_trait_impl`](https://github.com/rust-lang/rust-analyzer/pull/20957)
* [fix not applicable on while for `replace_is_method_with_if_let_method`](https://github.com/rust-lang/rust-analyzer/pull/20915)
* [canonicalize `custom-target.json` paths when fetching sysroot metadata](https://github.com/rust-lang/rust-analyzer/pull/20964)
* [consider more expression types as `in_value`](https://github.com/rust-lang/rust-analyzer/pull/20961)
* [expand literals with wrong suffixes into `LitKind::Err`](https://github.com/rust-lang/rust-analyzer/pull/20963)
* [false positive syntax errors on frontmatter](https://github.com/rust-lang/rust-analyzer/pull/20942)
* [fix handling of blocks modules that are not the root module](https://github.com/rust-lang/rust-analyzer/pull/20930)
* [improve error recovery when parsing malformed function return types](https://github.com/rust-lang/rust-analyzer/pull/20934)
* [properly support opaques](https://github.com/rust-lang/rust-analyzer/pull/20906)
* [resolve `target-dir` more precisely](https://github.com/rust-lang/rust-analyzer/pull/20920)
* [show proper async function signatures in the signature help](https://github.com/rust-lang/rust-analyzer/pull/20931)

### Rust Compiler Performance Triage

Mostly positive week. We saw a great performance win implemented by [#148040](https://github.com/rust-lang/rust/pull/148040) and [#148182](https://github.com/rust-lang/rust/pull/148182), which optimizes crates with a lot of trivial constants.

Triage done by **@kobzol**.

Revision range: [23fced0f..35ebdf9b](https://perf.rust-lang.org/?start=23fced0fcc5e0ec260d25f04a8b78b269e5e90f0&end=35ebdf9ba1414456dfe1cb6a6b13ebae80e99734&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.8%  | [0.1%, 2.9%]    | 22    |
| Regressions ❌ <br /> (secondary)  | 0.5%  | [0.1%, 1.7%]    | 48    |
| Improvements ✅ <br /> (primary)   | -2.8% | [-16.4%, -0.1%] | 102   |
| Improvements ✅ <br /> (secondary) | -1.9% | [-8.0%, -0.1%]  | 51    |
| All ❌✅ (primary)                 | -2.1% | [-16.4%, 2.9%]  | 124   |

4 Regressions, 6 Improvements, 7 Mixed; 7 of them in rollups
36 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/057eaab3021d6bc301bba06b69e7e1cfdb4f9c3d/triage/2025/2025-11-03.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [Emit error when using path-segment keyword as cfg pred](https://github.com/rust-lang/rust/pull/146978)
* [stabilize extern_system_varargs](https://github.com/rust-lang/rust/pull/145954)
* [Tracking issue for `vec_into_raw_parts`](https://github.com/rust-lang/rust/issues/65816)
* [rustdoc: Erase `#![doc(document_private_items)]`](https://github.com/rust-lang/rust/pull/146495)
* [Add new `function_casts_as_integer` lint](https://github.com/rust-lang/rust/pull/141470)
* [resolve: Preserve ambiguous glob reexports in crate metadata](https://github.com/rust-lang/rust/pull/147984)
* [Make deref_nullptr deny by default instead of warn](https://github.com/rust-lang/rust/pull/148122)
* [Tracking Issue for `const_mul_add`](https://github.com/rust-lang/rust/issues/146724)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)

* [remove support for `typeof`](https://github.com/rust-lang/compiler-team/issues/940)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [Guarantee the binary representation of `isize` explicitly](https://github.com/rust-lang/reference/pull/2064)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period)

* [Proposal: Require all project team members to have Zulip IDs](https://github.com/rust-lang/leadership-council/issues/228)

*No Items entered Final Comment Period this week for
  [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) or
  [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [build-std: explicit dependencies](https://github.com/rust-lang/rfcs/pull/3875)
* [build-std: always](https://github.com/rust-lang/rfcs/pull/3874)
* [build-std: context](https://github.com/rust-lang/rfcs/pull/3873)

## Upcoming Events

Rusty Events between 2025-11-05 - 2025-12-03 🦀

### Virtual
* 2025-11-05 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 2025-11-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/311574520/)
* 2025-11-05 | Virtual | [Ardan Labs](https://www.eventbrite.com/o/ardan-labs-7092394651)
    * [**Mastering Error Handling in Rust: From Panics to thiserror & anyhow**](https://www.eventbrite.com/e/mastering-error-handling-in-rust-from-panics-to-thiserror-anyhow-tickets-1849030121869)
* 2025-11-06 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305646021/)
* 2025-11-09 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109175/)
* 2025-11-10 || [BetterCode](https://www.bettercode.eu/)
    * $[**betterCode() Industrielle Anwendungen mit Rust**](https://dev.events/conferences/better-code-rust-i6inve6t)
* 2025-11-11 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361536/)
* 2025-11-11 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/311068632/)
* 2025-11-12 | Virtual (Boulder, CO, US) | [Boulder Elixir](https://www.meetup.com/boulder-elixir/events/)
    * [**Integrating Elixir and Apache DataFusion with Rustler**](https://www.meetup.com/boulder-elixir/events/310996627/)
* 2025-11-12 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/yhe1xrhe)
* 2025-11-13 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/310849154/)
* 2025-11-16 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109181/)
* 2025-11-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/310002262/)
* 2025-11-19 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/l2xukapz)
* 2025-11-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/309926564/)
* 2025-11-20 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046642/)
* 2025-11-20 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Tock, a Rust based Operating System Part #1**](https://www.meetup.com/charlottesville-rust-meetup/events/311705915/)
* 2025-11-23 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109183/)
* 2025-11-25 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361446/)
* 2025-11-25 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Data-Intensive Systems in Rust: Safety, Speed, Concurrency**](https://www.meetup.com/women-in-rust/events/311534469/)
* 2025-11-26 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/q5tjirkt)
* 2025-11-30 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109188/)
* 2025-12-02 | Virtual (London, GB) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Advent of Code - Kick Off!**](https://www.meetup.com/women-in-rust/events/311068648/)
* 2025-12-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 2025-12-03 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/yf2t878c)
* 2025-12-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyhcqbfb/)

### Africa
* 2025-11-11 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**More on Rust types**](https://www.meetup.com/johannesburg-rust-meetup/events/311726345/)

### Asia
* 2025-11-15 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**November 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/november-2025-rustacean-meetup//)

### Europe
* 2025-11-05 | Bergen, NO | [Hubbel kodeklubb](https://www.meetup.com/bergen-html-css-meetup-group/events/)
    * [**Workshop c# - nr 1 av 2 - grunnleggende nivå**](https://www.meetup.com/bergen-html-css-meetup-group/events/311784113/)
* 2025-11-05 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Rust Girona Hack & Learn 11 2025**](https://luma.com/xl8ob0tn)
* 2025-11-05 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust in November: Small Crates Cult**](https://www.meetup.com/rustcologne/events/311767026/)
* 2025-11-05 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310601872/)
* 2025-11-05 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Halloween special.**](https://www.meetup.com/oxford-rust-meetup-group/events/311569796/)
* 2025-11-06 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #11**](https://www.meetup.com/rust-gdansk/events/310924266/)
* 2025-11-06 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/events/)
    * [**Season 2 Kickoff | at metalab 🦀**](https://www.meetup.com/rust-vienna/events/311679397/)
* 2025-11-07 | Ostrava, CZ | [TechMeetup Ostrava](https://www.meetup.com/techmeetupostrava/events/)
    * [**TechMeetup Ostrava Conference**](https://www.meetup.com/techmeetupostrava/events/306776025/)
* 2025-11-11 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/events/)
    * [**Rust London x Zed Meetup**](https://www.meetup.com/rust-london-user-group/events/311737021/)
* 2025-11-11 | Stockholm, SE | [Func Prog Sweden](https://www.meetup.com/func-prog-sweden/events/)
    * [**Func Prog Sweden 2025 at Kivra**](https://www.meetup.com/func-prog-sweden/events/308526518/)
* 2025-11-12 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/events/)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/311583721/)
* 2025-11-12 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944050/)
* 2025-11-13 | Geneva, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2025-11-13 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #80**](https://www.meetup.com/rust-paris/events/311461594/)
* 2025-11-14 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Rust Meetup @Magello**](https://www.meetup.com/stockholm-rust/events/311844163/)
* 2025-11-18 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592257/)
* 2025-11-19 | Ostrava, CZ | [TechMeetup Ostrava](https://www.meetup.com/techmeetupostrava/events/)
    * [**QA Circle**](https://www.meetup.com/techmeetupostrava/events/311581090/)
* 2025-11-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Social Night**](https://www.meetup.com/rust-aarhus/events/311502123/)
* 2025-11-20 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ Monumental X Zed**](https://www.meetup.com/rust-amsterdam-group/events/311829267/)
* 2025-11-20 | Luzern, CH | [Rust Luzern]((https://www.meetup.com/rust-luzern/)
    * [**2025 Rust Talks Luzern #3: Crate Walkthroughs @ Noser Engineering AG**](https://www.meetup.com/rust-luzern/events/311410681/)
* 2025-11-26 | Bergen, NO | [Hubbel kodeklubb](https://www.meetup.com/bergen-html-css-meetup-group/events/)
    * [**Workshop c# - nr 2 av 2 - grunnleggende nivå**](https://www.meetup.com/bergen-html-css-meetup-group/events/311784127/)
* 2025-11-26 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #5 @Source Engineers**](https://www.meetup.com/rust-bern/events/311792568/)
* 2025-11-27 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**19th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/311787736/)
* 2025-11-27 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**November Talks: Exotically Sized Types and ...**](https://www.meetup.com/rust-and-friends/events/311753411/)
* 2025-11-28 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/events/)
    * [**Rust Meetup Prague @ Barclays**](https://www.meetup.com/rust-prague/events/311846118/)
* 2025-12-03 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Rust/ACCU meetup.**](https://www.meetup.com/oxford-rust-meetup-group/events/nnrkttyhcqbfb/)

### North America
* 2025-11-06 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal/events/)
    * [**November Monthly Social**](https://www.meetup.com/rust-montreal/events/311762040/)
* 2025-11-06 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Surprise**](https://www.meetup.com/stl-rust/events/307251982/)
* 2025-11-08 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Winter Hill Rust Lunch, Nov 8**](https://www.meetup.com/bostonrust/events/311039501/)
* 2025-11-13 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Ipmap: Building Desktop Apps with Tauri**](https://www.meetup.com/utah-rust/events/311613658/)
* 2025-11-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/308865806/)
* 2025-11-20 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**November, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351673/)
* 2025-11-20 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/events/)
    * [**Monthly Rust Meetup: November**](https://www.meetup.com/spokane-rust/events/311863560/)
* 2025-11-26 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310457310/)
* 2025-12-02 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Rust Talk December**](https://www.meetup.com/chicago-rust-meetup/events/311736848/)

### Oceania
* 2025-11-11 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/events/)
    * [**Christchurch Rust Meetup**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/311685331/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> If someone opens a PR introducing C++ to your Rust project, that code is free as in "use after"

– [Predrag Gruevski on Mastodon]()

Thanks to [Brett Witty](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1726) for the suggestion!

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
