Title: This Week in Rust 603
Number: 603
Date: 2025-06-11
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

- [A glance at Compiler Team Operations](https://blog.rust-lang.org/inside-rust/2025/06/05/a-glance-at-the-team-compiler-operations)

### Foundation

### Newsletters
* [The Embedded Rustacean Issue #47](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-47)
* [Rust Trends Issue #66](https://rust-trends.com/newsletter/rust-rewinded-debug-smarter-build-faster/)

### Project/Tooling Updates
* [Are We Embedded Yet? #2](https://jslazak.com/are-we-embedded-yet-2/)

* [Meilisearch 1.15 - new typo tolerance setting, comparison operators for string filters, and improved support for Chinese](https://www.meilisearch.com/blog/meilisearch-1-15)

- [cctx: A Context Switcher for Claude Code](https://syu-m-5151.hatenablog.com/entry/2025/06/05/232126)

### Observations/Thoughts
* [Rewriting SymCrypt in Rust to modernize Microsoft’s cryptographic library](https://www.microsoft.com/en-us/research/blog/rewriting-symcrypt-in-rust-to-modernize-microsofts-cryptographic-library/)
* [Why doesn’t Rust care more about compiler performance?](https://kobzol.github.io/rust/rustc/2025/06/09/why-doesnt-rust-care-more-about-compiler-performance.html)
* [Hedge funds are replacing a programming language with Rust, but it's not C++](https://www.efinancialcareers.com/news/rust-replacing-c-programming-language-hedge-fund)
* [The Concurrency Trap: How An Atomic Counter Stalled A Pipeline](https://www.conviva.com/platform/the-concurrency-trap-how-an-atomic-counter-stalled-a-pipeline/)
* [Rust For Foundational Software](https://corrode.dev/blog/foundational-software/)
* [10 years of betting on Rust](https://tably.com/tably/10-years-of-betting-on-rust)
* [Report on variadic generics discussions at RustWeek 2025.](https://poignardazur.github.io/2025/06/07/report-on-variadics-rustweek/)
* [Zero-cost Functional Records in Rust](https://ecency.com/rust-lang/@jonwolski/zero-cost-functional-records-in-rust)
* [A plan for SIMD](https://linebender.org/blog/a-plan-for-simd/)
* [When is a Rust function "unsafe"?](https://crescentro.se/posts/when-unsafe/)
* [video playlist] [RustWeek 2025](https://www.youtube.com/playlist?list=PL8Q1w7Ff68DCEXiGidlM0DMn8ztjlUlez)
* [audio] [What's New in Rust 1.79 and 1.80](https://rustacean-station.org/episode/rust-1.79-1.80/)
* [audio] [Rust at Work with Ran Reichman Co-Founder and CEO of Flarion](https://rustacean-station.org/episode/ran-reichman/)

* [Nine Rules for Scientific Libraries in Rust](https://medium.com/@carlmkadie/nine-rules-for-scientific-libraries-in-rust-6e5e33a6405b)

* [Rust on a Diet](https://blog.veeso.dev/blog/en/rust-on-a-diet/)

### Rust Walkthroughs
* [Introduction to embedded development with Rust: Overview of the ecosystem](https://kerkour.com/introduction-to-embedded-development-with-rust)
* [Achieving <100 ms Latency for Remote Control with WebRTC](https://gethopp.app/blog/latency-exploration)
* [Patterns for Modeling Overlapping Variant Data in Rust](https://mcmah309.github.io/posts/patterns-for-modeling-overlapping-variant-data-in-rust/)
* [Is Rust faster than C?](https://steveklabnik.com/writing/is-rust-faster-than-c/)
* [video] [Introducing facet: Reflection for Rust](https://www.youtube.com/watch?v=0mqFCqw_XvI)

* [video] [Combining Swift and Rust with UniFFI: Have Your Cake & Eat it Too](https://www.youtube.com/watch?v=DfSBBOlFTeE)

### Research

* [Kernel Memory Safety: Mission Accomplished](https://asterinas.github.io/2025/06/04/kernel-memory-safety-mission-accomplished.html)

### Miscellaneous
* [Getting A Read On Rust With Trainer, Consultant, and Author Herbert Wolverson](https://filtra.io/rust/interviews/ardan-jun-25)
* [video] [Julian Hofer - Pixi: the missing companion to cargo](https://www.youtube.com/watch?v=Hso3TQx13b0)

## Crate of the Week

This week's crate is [optics](https://crates.io/crates/optics), a typesafe, fully featured lens library.

Thanks to [Akos Vandra](https://users.rust-lang.org/t/crate-of-the-week/2704/1442) for the self-suggestion!

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

516 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-06-03..2025-06-10

#### Compiler

* [add (back) `unsupported_calling_conventions` lint to reject more invalid calling conventions](https://github.com/rust-lang/rust/pull/141435)
* [add a new `mismatched-lifetime-syntaxes` lint](https://github.com/rust-lang/rust/pull/138677)
* [fast path for stalled obligations on self ty](https://github.com/rust-lang/rust/pull/141681)
* [simplify and optimize `VecCache`'s `SlotIndex::from_index`](https://github.com/rust-lang/rust/pull/142095)
* [rework `collect_and_apply` to not rely on size hint for optimization](https://github.com/rust-lang/rust/pull/141652)
* [miri: TB: add flag to disable the more precise interior mutability tracking](https://github.com/rust-lang/miri/pull/4376)
* [miri: native-lib: allow multiple libraries and/or dirs](https://github.com/rust-lang/miri/pull/4372)

#### Library

* [stabilise `os_string_pathbuf_leak`](https://github.com/rust-lang/rust/pull/137992)
* [stabilize `const_eq_ignore_ascii_case`](https://github.com/rust-lang/rust/pull/142065)
* [stabilize `nonnull_provenance`](https://github.com/rust-lang/rust/pull/142238)
* [stabilize `sha512`, `sm3` and `sm4` for x86](https://github.com/rust-lang/rust/pull/140767)
* [stabilize `tcp_quickack`](https://github.com/rust-lang/rust/pull/129121)
* [bootstrap: build std sans leaf frame pointers](https://github.com/rust-lang/rust/pull/141800)
* [make `NonZero<char>` possible](https://github.com/rust-lang/rust/pull/141001)
* [optimize `Seek::stream_len` impl for `File`](https://github.com/rust-lang/rust/pull/125087)

#### Clippy

* [`doc_suspicious_footnotes`: lint text that looks like a footnote](https://github.com/rust-lang/rust-clippy/pull/14708)
* [`missing_const_for_fn`: consider constness of instance](https://github.com/rust-lang/rust-clippy/pull/14759)
* [`zombie_processes`: do not complain about early early returns](https://github.com/rust-lang/rust-clippy/pull/14912)
* [add new lint: `ip_constant`](https://github.com/rust-lang/rust-clippy/pull/14878)
* [do not lint macro generated codes](https://github.com/rust-lang/rust-clippy/pull/14976)
* [do not recurse indefinitely while checking for inner mutability](https://github.com/rust-lang/rust-clippy/pull/14965)
* [fix `branches_sharing_code` suggests wrongly when dealing with macros](https://github.com/rust-lang/rust-clippy/pull/14907)
* [fix `create_dir` ignores paths in suggestions](https://github.com/rust-lang/rust-clippy/pull/15011)
* [fix `match_single_binding` misses curlies on type signatures](https://github.com/rust-lang/rust-clippy/pull/15017)
* [fix `std_instead_of_core` FP when part of the `use` cannot be replaced](https://github.com/rust-lang/rust-clippy/pull/15016)
* [fix `unnecessary_debug_formatting` FP inside `Debug` impl](https://github.com/rust-lang/rust-clippy/pull/14955)
* [fix false positive for `unused_unit`](https://github.com/rust-lang/rust-clippy/pull/14962)
* [fix suggestion-causes-error of `print_literal` and `write_literal`](https://github.com/rust-lang/rust-clippy/pull/14961)
* [introduce `coerce_container_to_any`](https://github.com/rust-lang/rust-clippy/pull/14812)
* [invert suggestion if pointer is tested for non-nullness](https://github.com/rust-lang/rust-clippy/pull/15015)
* [lint reversed ordering in partial ord impl](https://github.com/rust-lang/rust-clippy/pull/14945)
* [use interned strings when possible, for efficiency purposes](https://github.com/rust-lang/rust-clippy/pull/14963)

#### Rust-Analyzer

* [better parser recovery for macro calls in type bound position](https://github.com/rust-lang/rust-analyzer/pull/19933)
* [add `dyn` keyword inlay hints](https://github.com/rust-lang/rust-analyzer/pull/19922)
* [implement attribute completions for diagnostics module](https://github.com/rust-lang/rust-analyzer/pull/19908)
* [always include quickfixes for diagnostics, even when diagnostics are disabled](https://github.com/rust-lang/rust-analyzer/pull/19935)
* [do not error at impls for unsized types that do not include `where Self: Sized` items](https://github.com/rust-lang/rust-analyzer/pull/19963)
* [record macro calls for fields in `ChildBySource` impls](https://github.com/rust-lang/rust-analyzer/pull/19937)
* [record macro calls in signatures in `ChildBySource` impls](https://github.com/rust-lang/rust-analyzer/pull/19932)
* [stabilize the "JSON is not Rust" diagnostic](https://github.com/rust-lang/rust-analyzer/pull/19949)
* [stabilize unlinked file diagnostic](https://github.com/rust-lang/rust-analyzer/pull/19936)
* [hir-ty: add incremental tests checking for `infer` invalidation](https://github.com/rust-lang/rust-analyzer/pull/19914)
* [make `Semantics<'db, DB>` support `Semantics<'db, dyn HirDatabase>`, take two](https://github.com/rust-lang/rust-analyzer/pull/19930)

### Rust Compiler Performance Triage

Mostly positive week, with a lot of improvements in the type system, especially in new solver and one big win in caching code. Regressions come from new warnings, with outsized impact on one benchmark with a lot of generated code.

Triage done by **@panstromek**.
Revision range: [2fc3deed..c31cccb7](https://perf.rust-lang.org/?start=2fc3deed9fcb8762ad57191e0195f06f7543e4a5&end=c31cccb7b5cc098b1a8c1794ed38d7fdbec0ccb0&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 3.1%  | [0.3%, 8.5%]    | 22    |
| Regressions ❌ <br /> (secondary)  | 0.6%  | [0.2%, 0.9%]    | 3     |
| Improvements ✅ <br /> (primary)   | -1.0% | [-3.4%, -0.2%]  | 151   |
| Improvements ✅ <br /> (secondary) | -3.5% | [-66.5%, -0.2%] | 146   |
| All ❌✅ (primary)                 | -0.4% | [-3.4%, 8.5%]   | 173   |

[Full report here](https://github.com/rust-lang/rustc-perf/blob/7a1e00ae0a30c783bdfa8e3c35e3b49ce88b58e9/triage/2025-06-09.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Tracking Issue for `mixed_integer_ops_unsigned_sub`](https://github.com/rust-lang/rust/issues/126043)
* [Allow storing `format_args!()` in variable](https://github.com/rust-lang/rust/pull/140748)
* [Tracking Issue for File lock API](https://github.com/rust-lang/rust/issues/130994)
* [Sized Hierarchy: Part I](https://github.com/rust-lang/rust/pull/137944)
* [Allow volatile access to non-Rust memory, including address 0](https://github.com/rust-lang/rust/pull/141260)
* [const-eval: allow constants to refer to mutable/external memory, but reject such constants as patterns](https://github.com/rust-lang/rust/pull/140942)
* [Report never type lints in dependencies](https://github.com/rust-lang/rust/pull/141937)
* [builtin dyn impl no guide inference](https://github.com/rust-lang/rust/pull/141352)
* [Change `core::iter::Fuse`'s Default impl to do what its docs say it does](https://github.com/rust-lang/rust/pull/140985)
* [Stabilize derive(CoercePointee)](https://github.com/rust-lang/rust/pull/133820)
* [impl `Default` for `array::IntoIter`](https://github.com/rust-lang/rust/pull/141574)
* [Added `Clone` implementation for `ChunkBy`](https://github.com/rust-lang/rust/pull/138016)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC: Dedented String Literals](https://github.com/rust-lang/rfcs/pull/3830)

## Upcoming Events

Rusty Events between 2025-06-11 - 2025-07-09 🦀

### Virtual
* 2025-06-04 | Virtual | [Scientific Computing in Rust](https://scientificcomputing.rs)
    * [**Scientific Computing in Rust 2025**](https://scientificcomputing.rs/2025)
* 2025-06-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031665)
* 2025-06-05 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820301)
* 2025-06-07 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2025-06-08 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/307927093)
* 2025-06-10 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305020417)
* 2025-06-10 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/307560326)
* 2025-06-11 | Virtual (Tel Aviv, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/)
    * [**Rust at Work - conversation with Herbert Wolverson of Ardan Labs & LibreQoS**](https://www.meetup.com/code-mavens/events/308234298/)
* 2025-06-12 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Meet, swap, and learn!**](https://www.meetup.com/charlottesville-rust-meetup/events/307767236)
* 2025-06-15 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/308074808)
* 2025-06-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/305170853)
* 2025-06-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307730493)
* 2025-06-19 | Hybrid (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**June, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 2025-06-19 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820303)
* 2025-06-22 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/308246353)
* 2025-06-24 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361436)
* 2025-06-24 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Building Efficient Web Scrapers: Rust vs. Python for Data Ingestion**](https://www.meetup.com/women-in-rust/events/306683025)
* 2025-06-26 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567869)
* 2025-06-29 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/kvqfrtyhcjbmc)
* 2025-07-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031667)

### Asia
* 2025-06-08 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust June 2025 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/306414888)

### Europe
* 2025-06-04 | Ghent, BE | [Systems Programming Ghent](https://www.sysghent.be/)
    * [**Grow smarter with embedded Rust**](https://www.meetup.com/systems-programming-ghent/events/307269551)
* 2025-06-04 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn)
    * [**Rust in June: A Decade of Rust**](https://www.meetup.com/rustcologne/events/308158241)
* 2025-06-04 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Risc V - the new challenger for cpus in AI and embedded systems.**](https://www.meetup.com/oxford-rust-meetup-group/events/307673867)
* 2025-06-05 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2025 / 2 - Hacking Evening**](https://www.meetup.com/rust-munich/events/307105443)
* 2025-06-10 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/308080874)
* 2025-06-10 | Warsaw, PL | [Rust Warsaw](https://www.meetup.com/rust-warsaw)
    * [**Rust Warsaw Meetup #5**](https://www.meetup.com/rust-warsaw/events/307955051)
* 2025-06-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305045448)
* 2025-06-12 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin on location 🏳️‍🌈 - Edition 003**](https://www.meetup.com/rust-berlin/events/308131380)
* 2025-06-17 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/305741641)
* 2025-06-18 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Rust Meetup @Magello**](https://www.meetup.com/stockholm-rust/events/308129156)
* 2025-06-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Rust Aarhus meetup at Trifork**](https://www.meetup.com/rust-aarhus/events/308060489)
* 2025-06-19 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (evening pub)**](https://www.meetup.com/rust-and-friends/events/308023524)
* 2025-06-20 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/308023512)
* 2025-06-24 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester June Code Night**](https://www.meetup.com/rust-manchester/events/307919158)
* 2025-06-25 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group)
    * [**Lessons learnt from making a tiny game in nostd Rust**](https://www.meetup.com/london-rust-project-group/events/306809962)
* 2025-06-26 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #58**](https://www.meetup.com/copenhagen-rust-community/events/308161212)
* 2025-07-02 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #12 @ kHaus**](https://www.meetup.com/rust-basel/events/307567391)

### North America
* 2025-06-05 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/308091592)
* 2025-06-05 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Leptos web framework**](https://www.meetup.com/stl-rust/events/305534867)
* 2025-06-08 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Boston University Rust Lunch, June 8**](https://www.meetup.com/bostonrust/events/307936165)
* 2025-06-11 | Phoenix, AZ, US | [Desert Rust](https://www.meetup.com/desert-rustaceans)
    * [**Rust <> Security**](https://www.meetup.com/desert-rustaceans/events/308010023)
* 2025-06-12 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/308019627)
* 2025-06-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/307595021)
* 2025-06-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307730493)
* 2025-06-19 | Hybrid (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**June, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 2025-06-19 | México City, MX | [Rust MX](https://www.meetup.com/rust-mx)
    * [**programación asíncrona en Rust usando Tokio**](https://www.meetup.com/rust-mx/events/308248260)
* 2025-06-19 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Using Rust For Web Series 3 : Final Presentations and Community Social**](https://www.meetup.com/music-city-rust-developers/events/304333108)
* 2025-06-20 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Lechmere Rust Lunch, June 20**](https://www.meetup.com/bostonrust/events/307936242)
* 2025-06-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcjbhc)
* 2025-06-26 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust)
    * [**Monthly Meetup: Making a CRUD API with Rust!**](https://www.meetup.com/spokane-rust/events/307969600)
* 2025-06-28 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Back Bay Rust Lunch, June 28**](https://www.meetup.com/bostonrust/events/307936269)

### Oceania
* 2025-06-16 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group)
    * [**Christchurch Rust Meetup**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/307808896)
* 2025-06-24 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/307520854)

### South America
* 2025-06-04 | Montevideo, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay)
    * [**Primera meetup de Rust de 2025!**](https://www.meetup.com/rust-uruguay/events/307341567)
* 2025-06-12 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Junio de WebAssembly!**](https://www.meetup.com/rust-argentina/events/307990465)

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

> Gaze not into the abyss, lest you become recognized as an ***abyss domain expert***, and they expect you keep gazing into the damn thing.

– [Nick Mathewson on twitter](https://x.com/nickm_tor/status/860234274842324993?lang=en)

Thanks to [robin](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1696) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
