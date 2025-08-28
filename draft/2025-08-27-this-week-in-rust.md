Title: This Week in Rust 614
Number: 614
Date: 2025-08-27
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

* [StrongBox: Simple, Safe Data Encryption for Rust](https://www.hezmatt.org/~mpalmer/blog/2025/08/27/strong-box-simple-safe-data-encryption-for-rust.html)
* [Danube Messaging- Release 0.4.0 - Highlights](https://dev-state.com/posts/danube_update_040/)

### Observations/Thoughts

* [audio] [Netstack.FM — Episode 2: Hyper with Sean McArthur](https://netstack.fm/#episode-2)

### Rust Walkthroughs
* [Rust for JavaScript Engineers - Building Connect-4](https://www.afloat.boats/posts/rust-for-javascript-engineers-pt-1)
* [Pest gotchas](https://andreabergia.com/blog/2025/08/pest-gotchas/)

* [Shortcomings of Macros, And How To Overcome Them](https://cryptical.xyz/rust/shortcomings-of-macros)

### Research

### Miscellaneous

* [GreptimeDB Rust Client - A Comprehensive Guide to High-Throughput Bulk Stream Inserts](https://greptime.com/blogs/2025-07-30-greptimedb-rust-guide-bulk-stream-insert)

## Crate of the Week

This week's crate is [web-route](https://crates.io/crates/web-route), a library to ergonomically define and manage web server routes in Rust.

Thanks to [sidrubs](https://users.rust-lang.org/t/crate-of-the-week/2704/1463) for the self-suggestion!

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
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

553 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-08-19..2025-08-26

#### Compiler
* [demote `x86_64-apple-darwin` to Tier 2 with host tools](https://github.com/rust-lang/rust/pull/145252)
* [`rustc_expand`: ensure stack in `InvocationCollector::visit_expr`](https://github.com/rust-lang/rust/pull/145410)
* [account for impossible bounds making seemingly unsatisfyable dyn-to-dyn casts](https://github.com/rust-lang/rust/pull/145620)
* [add lint against integer to pointer transmutes](https://github.com/rust-lang/rust/pull/144531)
* [fix ICE when validating transmuting ZST to inhabited `enum`](https://github.com/rust-lang/rust/pull/145791)
* [refactor attribute parsing to improve ergonomics and some diagnostics](https://github.com/rust-lang/rust/pull/145507)
* [simplify span caches](https://github.com/rust-lang/rust/pull/145505)
* [slightly optimize reading of source files](https://github.com/rust-lang/rust/pull/145848)
* [miri: account for time spent tracing, use RDTSC for faster time](https://github.com/rust-lang/miri/pull/4524)
* [miri: support weak definitions](https://github.com/rust-lang/miri/pull/4414)

#### Library
* [`UnsafePinned::raw_get`: sync signature with get](https://github.com/rust-lang/rust/pull/145593)
* [`bufreader::Buffer::backshift`: don't move the uninit bytes](https://github.com/rust-lang/rust/pull/145538)
* [experiment: reborrow trait](https://github.com/rust-lang/rust/pull/145726)
* [fix parameter order for `_by()` variants of `min` / `max`/ `minmax` in `std::cmp`](https://github.com/rust-lang/rust/pull/139357)
* [fmt of non-decimal radix untangled](https://github.com/rust-lang/rust/pull/143730)
* [implementation: `#[feature(nonpoison_rwlock)]`](https://github.com/rust-lang/rust/pull/144648)
* [stabilize `const_array_each_ref`](https://github.com/rust-lang/rust/pull/143383)
* [stabilize `const_pathbuf_osstring_new` feature](https://github.com/rust-lang/rust/pull/145464)
* [hashbrown: un-bloat `get_inner` functions to restore lookup performance](https://github.com/rust-lang/hashbrown/pull/639)

#### Cargo
* [cargo: linting system](https://github.com/rust-lang/cargo/pull/15865)
* [cargo: suggest workspace hints for boolean dependencies](https://github.com/rust-lang/cargo/pull/15507)

#### Rustdoc
* [add support for macro expansion in rustdoc source code pages](https://github.com/rust-lang/rust/pull/137229)
* [make attributes render consistently](https://github.com/rust-lang/rust/pull/145782)
* [render attributes in Field and Variants sections](https://github.com/rust-lang/rust/pull/145812)

#### Clippy
* [clippy: `bool_comparison`: fix incorrect suggestion with `>`/`<` and macros](https://github.com/rust-lang/rust-clippy/pull/15513)
* [clippy: `bool_comparison`: no longer lint on `!x != y`](https://github.com/rust-lang/rust-clippy/pull/15498)
* [clippy: `cast_slice_from_raw_parts`: check for implicit cast to raw slice pointer](https://github.com/rust-lang/rust-clippy/pull/15437)
* [clippy: `ptr_as_ptr`: fix incorrect suggestion with `pointer::cast` and macros](https://github.com/rust-lang/rust-clippy/pull/15514)
* [clippy: `too_many_lines`: only highlight the function signature](https://github.com/rust-lang/rust-clippy/pull/15461)
* [clippy: `unnecessary_mut_passed`: add structured suggestion](https://github.com/rust-lang/rust-clippy/pull/15438)
* [clippy: `unused_unit`: don't lint on closure return types](https://github.com/rust-lang/rust-clippy/pull/15549)
* [clippy: better check for `assign_op_pattern` in `const` context](https://github.com/rust-lang/rust-clippy/pull/15532)
* [clippy: check f16 and f128 in `float_equality_without_abs`](https://github.com/rust-lang/rust-clippy/pull/15054)
* [clippy: detect infinite loop in `async fn` not returning `!`](https://github.com/rust-lang/rust-clippy/pull/15545)
* [clippy: do not replace `match` by `if` if any arm contains a binding](https://github.com/rust-lang/rust-clippy/pull/15352)
* [clippy: fix `unnecessary_safety_comment` not linting for the first line](https://github.com/rust-lang/rust-clippy/pull/15354)
* [clippy: fix `async_yields_async` wrongly unmangled macros](https://github.com/rust-lang/rust-clippy/pull/15553)
* [clippy: fix `derivable_impls` suggests wrongly on `derive_const`](https://github.com/rust-lang/rust-clippy/pull/15535)
* [clippy: fix `manual_is_ascii_check`: also add explicit type when linting `matches!`](https://github.com/rust-lang/rust-clippy/pull/15492)
* [clippy: fix `or_then_unwrap`: suggestion preserves macro calls](https://github.com/rust-lang/rust-clippy/pull/15483)
* [clippy: fix `semicolon_inside_block` false positive when attribute over expr is not enabled](https://github.com/rust-lang/rust-clippy/pull/15476)
* [clippy: fix `unnested_or_patterns` false positive on structs with only shorthand field pats](https://github.com/rust-lang/rust-clippy/pull/15343)

#### Rust-Analyzer
* [rust-analyzer: `replace_arith_op` not applicable on selected](https://github.com/rust-lang/rust-analyzer/pull/20512)
* [rust-analyzer: add `ReturnExpr` completion suggest](https://github.com/rust-lang/rust-analyzer/pull/20507)
* [rust-analyzer: add let in let-chain completion support](https://github.com/rust-lang/rust-analyzer/pull/20513)
* [rust-analyzer: add an option to hide reborrows in adjustment inlay hints](https://github.com/rust-lang/rust-analyzer/pull/20520)
* [rust-analyzer: fix `else` completion in `let _ = if x {} $0`](https://github.com/rust-lang/rust-analyzer/pull/20518)
* [rust-analyzer: fix panic in `syntax_highlighting`](https://github.com/rust-lang/rust-analyzer/pull/20506)
* [rust-analyzer: fix rust-analyzer-contributors reference](https://github.com/rust-lang/rust-analyzer/pull/20529)
* [rust-analyzer: fix indentation in `move_guard_to_arm_body`](https://github.com/rust-lang/rust-analyzer/pull/20509)
* [rust-analyzer: fix opaque generics](https://github.com/rust-lang/rust-analyzer/pull/20523)
* [rust-analyzer: improve semicolon handling in `toggle_macro_delimiter`](https://github.com/rust-lang/rust-analyzer/pull/20534)
* [rust-analyzer: infinite recursion while lowering assoc type bounds from supertraits](https://github.com/rust-lang/rust-analyzer/pull/20504)
* [rust-analyzer: make import sorting order follow 2024 edition style](https://github.com/rust-lang/rust-analyzer/pull/20423)
* [rust-analyzer: masquerade as nightly cargo when invoking flycheck with `-Zscript`](https://github.com/rust-lang/rust-analyzer/pull/20528)
* [rust-analyzer: normalize all types when finishing inference](https://github.com/rust-lang/rust-analyzer/pull/20537)
* [rust-analyzer: remove unnecessary `salsa::attach()` calls](https://github.com/rust-lang/rust-analyzer/pull/20502)

### Rust Compiler Performance Triage

Lot of regressions this week, mostly in rustdoc benchmarks from newly added features. The rest of the suite saw mostly small regressions in small benchmarks and also some improvements, notably from token tree parsing optimization in macro code, span optimization and ongoing work on new solver, which is not fully enabled, yet.

Triage done by **@panstromek**.
Revision range: [239e8b1b..ee361e8f](https://perf.rust-lang.org/?start=239e8b1b47b34120287ec36b33228c1e177f0c38&end=ee361e8fca1c30e13e7a31cc82b64c045339d3a8&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 3.7%  | [0.3%, 34.5%]  | 42    |
| Regressions ❌ <br /> (secondary)  | 2.3%  | [0.0%, 53.3%]  | 79    |
| Improvements ✅ <br /> (primary)   | -0.5% | [-0.7%, -0.3%] | 9     |
| Improvements ✅ <br /> (secondary) | -0.9% | [-2.8%, -0.0%] | 30    |
| All ❌✅ (primary)                 | 3.0%  | [-0.7%, 34.5%] | 51    |

5 Regressions, 1 Improvement, 7 Mixed; 6 of them in rollups
38 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/76b6beef3a67f4c97f61745ea510b4c4a924046f/triage/2025/2025-08-25.md)


### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Promote aarch64-pc-windows-msvc to Tier 1](https://github.com/rust-lang/rfcs/pull/3817)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Tracking Issue for NUL-terminated file names with `#[track_caller]`](https://github.com/rust-lang/rust/issues/141727)
* [style-guide: Document absence of trailing whitespace](https://github.com/rust-lang/rust/pull/145735)
* [Partial-stabilize the basics from `bigint_helper_methods`](https://github.com/rust-lang/rust/pull/144494)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Supertrait Auto-impl](https://github.com/rust-lang/rfcs/pull/3851)
* [new] [RFC: Autogenerated Attribute](https://github.com/rust-lang/rfcs/pull/3850)

## Upcoming Events

Rusty Events between 2025-08-27 - 2025-09-24 🦀

### Virtual
* 2025-08-20 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731032)
* 2025-08-21 | Hybrid (Mexico City, MX) | [Rust MX](https://www.meetup.com/rust-mx)
    * [**Polars para análisis y manipulación de datos**](https://www.meetup.com/rust-mx/events/310408223/)
* 2025-08-21 | Hybrid (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Agosto de Protocol Buffers!**](https://www.meetup.com/rust-argentina/events/310019573)
* 2025-08-21 | Virtual (London, UK) | [Conf42: Online Tech Events](https://www.meetup.com/conf42/events/)
    * [**Conf42 Rustlang 2025**](https://www.meetup.com/conf42/events/305437705)
* 2025-08-21 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567875)
* 2025-08-24 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002461)
* 2025-08-26 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361442)
* 2025-08-28 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305878943)
* 2025-08-28 | Virtual (Los Angeles, CA, US) | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles/events/)
    * [**AI-Powered Smart Contracts Workshop**](https://www.meetup.com/rust-los-angeles/events/310603465)
* 2025-08-31 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002471)
* 2025-09-02 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304234)
* 2025-09-02 - 2025-09-05 | Hybrid (Seattle, WA, US) | [RustConf](https://rustconf.com/)
    * [**RustConf 2025**](https://rustconf.com/)
* 2025-09-02 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304234)
* 2025-09-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyhcmbfb)
* 2025-09-06 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763848597)
* 2025-09-07 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002479)
* 2025-09-09 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361533)
* 2025-09-09 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**From First Lines to First Clients: Carol Nichols on Building a Career in Rust**](https://www.meetup.com/women-in-rust/events/310102318)
* 2025-09-11 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305646019)
* 2025-09-11 | Virtual (San Diego, CA, US) | [San Diego Rust](https://www.meetup.com/san-diego-rust/events/)
    * [**San Diego Rust September 2025 Online Meetup**](https://www.meetup.com/san-diego-rust/events/310326567)
* 2025-09-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002480)
* 2025-09-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/306757758)
* 2025-09-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731033)

### Asia
* 2025-08-20 | Seoul, KR | [Seoul Rust](https://www.meetup.com/rust-seoul-meetup)
    * [**Seoul Rust Meetup**](https://www.meetup.com/rust-seoul-meetup/events/310347685)
* 2025-08-21 | Kuala Lumpur, MY | [Rust Malaysia](https://www.linkedin.com/company/rustmalaysia/)
    * [**Malaysia Rust Meetup**](https://www.eventbrite.sg/e/backend-webdev-with-axum-and-diesel-rust-meetup-aug-2025-tickets-1588476137889)
* 2025-08-23 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**August 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/august-2025-rustacean-meetup/)
* 2025-09-13 | Hangzhou, ZH, CN | [WebAssembly and Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**GOSIM AI Hangzhou 2025 (CFP is still open)**](https://www.meetup.com/wasm-rust-meetup/events/309987624)

### Europe
* 2025-08-21 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310062129)
* 2025-08-28 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust meetup #60 sponsored by Bang & Olufsen**](https://www.meetup.com/copenhagen-rust-community/events/310591727)
* 2025-08-28 | Edinburgh, GB | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (evening pub)**](https://www.meetup.com/rust-and-friends/events/310438757)
* 2025-08-28 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester August Code Night**](https://www.meetup.com/rust-manchester/events/307919168)
* 2025-08-29 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/310438764)
* 2025-08-30 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #16**](https://www.meetup.com/stockholm-rust/events/310322522)
* 2025-09-03 | Edinburgh, GB | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Want a Squeezable / Modern / Helpful / Wide Language? Choose Four**](https://www.meetup.com/rust-and-friends/events/310536614)
* 2025-09-03 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**From bugs to parallelism to future-proofing: What makes Rust different**](https://www.meetup.com/rust-rhein-main/events/310322369)
* 2025-09-04 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #10**](https://www.meetup.com/rust-gdansk/events/310610993)
* 2025-09-10 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944038)
* 2025-09-11 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #4 @Zühlke**](https://www.meetup.com/rust-bern/events/309903540)
* 2025-09-16 | Berlin, DE | [Oxidize Conference] (https://oxidizeconf.com/)
    * [**Oxidize Conference**](https://oxidizeconf.com/)
* 2025-09-16 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592250)
* 2025-09-17 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Rust Girona Hack & Learn 09 2025**](https://lu.ma/ql3u6q5u)

### North America
* 2025-08-20 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731032)
* 2025-08-21 | Hybrid (Mexico City, MX) | [Rust MX](https://www.meetup.com/rust-mx)
    * [**Polars para análisis y manipulación de datos**](https://www.meetup.com/rust-mx/events/310408223/)
* 2025-08-21 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/310321250)
* 2025-08-21 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Rust on Bare Metal Series 2 : Place Holder**](https://www.meetup.com/music-city-rust-developers/events/304333117)
* 2025-08-23 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Somerville Union Square Rust Lunch, Aug 23**](https://www.meetup.com/bostonrust/events/310106302)
* 2025-08-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310205991)
* 2025-08-28 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**We're going again!**](https://www.meetup.com/rust-atl/events/308675976)
* 2025-08-28 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/310602222)
* 2025-08-28 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles/events/)
    * [**Rust in Web3 Meetup**](https://www.meetup.com/rust-los-angeles/events/310618705)
* 2025-09-02 - 2025-09-05 | Hybrid (Seattle, WA, US) | [RustConf](https://rustconf.com/)
    * [**RustConf 2025**](https://rustconf.com/)
* 2025-09-04 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/310547154)
* 2025-09-03 | Phoenix, AZ, US | [Desert Rust](https://www.meetup.com/desert-rustaceans)
    * [**RustConf 2025 Watch Party (Day 1)**](https://www.meetup.com/desert-rustaceans/events/310345446)
* 2025-09-04 | Phoenix, AZ, US | [Desert Rust](https://www.meetup.com/desert-rustaceans)
    * [**RustConf 2025 Watch Party (Day 2)**](https://www.meetup.com/desert-rustaceans/events/310345459)
* 2025-09-04 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**emulation of retro systems (NES, Gameboy) in Rust**](https://www.meetup.com/stl-rust/events/310116988)
* 2025-09-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Beacon Hill Rust Lunch, Sep 6**](https://www.meetup.com/bostonrust/events/310106310)
* 2025-09-11 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**September, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/308677324)
* 2025-09-14 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Davis Square Rust Lunch, Sep 14**](https://www.meetup.com/bostonrust/events/310106317)
* 2025-09-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/308284339)
* 2025-09-17 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Tick, Tock, talk—find out how Rust secures embedded devices**](https://www.meetup.com/charlottesville-rust-meetup/events/310603587)

### Oceania
* 2025-08-26 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**August Meetup**](https://www.meetup.com/rust-canberra/events/308746519)
* 2025-08-27 - 2025-08-30 | Wellington, NZ | [Rust Forge](https://rustforgeconf.com/)
    * [**Rust Forge**](https://rustforgeconf.com/)

### South America
* 2025-08-21 | Hybrid (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina) | [Rust Lang AR](https://rust-lang.ar) | [Oxidar](https://oxidar.org)
    * [**Agosto de Protocol Buffers!**](https://www.meetup.com/rust-argentina/events/310019573) | [Live Stream](https://meet.google.com/pfw-hrqx-zhf)

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

> I `clone()` everything now. The Borrow Checker permits this small rebellion, this inefficiency. It knows I suffer more knowing my code is not idiomatic. Every `.clone()` is a confession of my failure. Every `Arc<Mutex>` a monument to my inadequacy.

– [/u/TheEldenLorrdd on /r/rust](https://reddit.com/comments/1mwmei6)

Thanks to [Colin Terry](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1709) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
