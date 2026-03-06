Title: This Week in Rust 641
Number: 641
Date: 2026-03-04
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
* [2025 State of Rust Survey Results](https://blog.rust-lang.org/2026/03/02/2025-State-Of-Rust-Survey-results/)

### Foundation

### Newsletters
* [The Embedded Rustacean Issue #66](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-66)

### Project/Tooling Updates
* [Compendium: Adding eBPF for Kernel-Level Visibility](https://pker.xyz/posts/compendium-ebpf)
* [Danube Messaging migration from ETCD](https://dev-state.com/posts/migrate_danube_etcd_to_raft/) 
* [Feedr v0.4.0 - Terminal-based RSS feed reader](https://github.com/bahdotsh/feedr/releases/tag/v0.4.0)
* [dag_exec: DAG executor for CPU-heavy pipelines](https://www.reymom.xyz/blog/rust/2026-03-03-exec_dag-official-release)
* [Supercharge Rust functions with implicit arguments using CGP v0.7.0](https://contextgeneric.dev/blog/v0.7.0-release/)
* [vscreen: AI agents browser](https://dev.to/lowjax/i-built-a-tool-that-lets-ai-agents-browse-the-real-internet-and-you-can-watch-them-do-it-2fff)
* [Ply 1.0: Building apps in Rust shouldn't be this hard](https://plyx.iz.rs/blog/introducing-ply/)

### Observations/Thoughts
* [Using Rust and Postgres for everything: patterns learned over the years](https://kerkour.com/rust-postgres-everything)
* [Kovan: From Production MVCC Systems to Wait-Free Memory Reclamation](https://vertexclique.com/blog/kovan-from-prod-to-mr/)
* [Never snooze a future](https://jacko.io/snooze.html)
* [Rust zero-cost abstractions vs. SIMD](https://turbopuffer.com/blog/zero-cost)
* [Nobody ever got fired for using a struct](https://www.feldera.com/blog/nobody-ever-got-fired-for-using-a-struct)
* [Debugging Reproducibility Issues in Rust Software](https://notes.8pit.net/notes/iqfs.html)

* [Designing Backpressure in a Parallel DAG Executor](https://www.reymom.xyz/blog/rust/2026-02-21-backpressure-in-parallel-executor)
* [Testing Concurrency Invariants in a Parallel Executor](https://www.reymom.xyz/blog/rust/2026-02-24-testing-invariants-atomics)

[audio] [Netstack.FM episode 29 — Hyper With Sean McArthur (Ep 2 Remastered)](https://netstack.fm/#episode-29)

### Rust Walkthroughs
* [Tutorial: let's make a resumable Pi Spigot with SQLite](https://www.sea-ql.org/blog/2026-02-28-sea-orm-sync/)
* [Apache Iggy's migration journey to thread-per-core architecture powered by io_uring](https://iggy.apache.org/blogs/2026/02/27/thread-per-core-io_uring/)
* [Formal methods for the unsafe side of the Force](https://antithesis.com/blog/2026/rust_formal_methods/)
* [Quantifying the Swiss marriage tax](https://gendx.dev/blog/2026/03/02/swiss-marriage-tax.html)
* [video] [Rust: compiling to WASM to make a browser-based game using canvas](https://artificialworlds.net/blog/2026/02/27/wasm-game/)
* [video] [Daniel Almeida Interview, Writing a Linux GPU Kernel Driver in Rust](https://youtu.be/rgjTPBRae6I)

* [Fast Python with Rust: a data-oriented approach](https://hackeryarn.com/post/fast-python-with-rust/)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [office2pdf](office2pdf), a standalone library or binary to generate PDF from OOXML (docx, xlsx, etc.) files.

Thanks to [One](https://users.rust-lang.org/t/crate-of-the-week/2704/1562) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

*No calls for testing were issued this week by
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Rustup](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) or
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

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

* [**Rust India Conference 2026**](https://hasgeek.com/rustbangalore/cfp-rust-india-conference-2026/) | CFP open until 2026-03-14 | Bangalore, IN | 2026-04-18
* [**Oxidize Conference**](https://pretalx.com/oxidize-conference-2026-2025/cfp) | CFP open until 2026-03-23 | Berlin, Germany | 2026-09-14 - 2026-09-16
* [**EuroRust**](https://sessionize.com/eurorust-2026/) | CFP open until 2026-04-27 | Barcelona, Spain | 2026-10-14 - 2026-10-17

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

414 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-02-24..2026-03-03

#### Compiler
* [improve the forcing/promotion functions in `DepKindVTable`](https://github.com/rust-lang/rust/pull/153122)
* [codegen: Restore `noundef` On `PassMode::Cast` Args In Rust ABI](https://github.com/rust-lang/rust/pull/152864)

#### Library
* [`BTreeMap::merge` optimized](https://github.com/rust-lang/rust/pull/152418)
* [make atomic primitives type aliases of `Atomic<T>`](https://github.com/rust-lang/rust/pull/153015)
* [neon fast path for `str::contains`](https://github.com/rust-lang/rust/pull/152176)
* [prepare `NonNull` for pattern types](https://github.com/rust-lang/rust/pull/152702)
* [re-add `#[inline]` to `Eq::assert_fields_are_eq`](https://github.com/rust-lang/rust/pull/153157)
* [stabilize new `RangeToInclusive` type](https://github.com/rust-lang/rust/pull/152304)

#### Cargo
* [fix: Inject an edition into scripts](https://github.com/rust-lang/cargo/pull/16678)
* [help: display manpage for nested commands](https://github.com/rust-lang/cargo/pull/16432)
* [host-config: fix panic when cross-compiling with host-config](https://github.com/rust-lang/cargo/pull/16674)
* [toml: show required rust-version in unstable edition error](https://github.com/rust-lang/cargo/pull/16653)
* [improve parent workspace search error msg](https://github.com/rust-lang/cargo/pull/16669)

#### Clippy
* [fix `cmp_owned` suggests wrongly on `PathBuf`](https://github.com/rust-lang/rust-clippy/pull/16628)
* [fix `explicit_counter_loop` false positive when the initializer is not integral](https://github.com/rust-lang/rust-clippy/pull/16647)
* [fix `suboptimal_flops` false negative on add and sub assign](https://github.com/rust-lang/rust-clippy/pull/16625)
* [handle core panics in all format lints](https://github.com/rust-lang/rust-clippy/pull/16597)

#### Rust-Analyzer
* [detect E0804 when casting raw ptr-to-dyn adds auto traits](https://github.com/rust-lang/rust-analyzer/pull/21699)
* [don't panic on invalid LSP notifications](https://github.com/rust-lang/rust-analyzer/pull/21708)
* [fix scrutinee expr indent for `replace_if_let_with_match`](https://github.com/rust-lang/rust-analyzer/pull/21698)
* [no complete `enum` variant qualifier in pat](https://github.com/rust-lang/rust-analyzer/pull/21706)
* [use `ExprIsRead::Yes` for rhs of binary operators](https://github.com/rust-lang/rust-analyzer/pull/21654)
* [implement `Span::SpanParent` for proc-macro-srv](https://github.com/rust-lang/rust-analyzer/pull/21669)

### Rust Compiler Performance Triage


A positive week with a few nice improvements coming from query system cleanups.

Triage done by **@panstromek**.
Revision range: [eeb94be7..ddd36bd5](https://perf.rust-lang.org/?start=eeb94be79adc9df7a09ad0b2421f16e60e6d932c&end=ddd36bd57051f796850345b76c17e9402e28a9e4&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.3%  | [0.3%, 0.3%]   | 1     |
| Regressions ❌ <br /> (secondary)  | 0.2%  | [0.0%, 0.3%]   | 3     |
| Improvements ✅ <br /> (primary)   | -0.8% | [-2.1%, -0.1%] | 141   |
| Improvements ✅ <br /> (secondary) | -1.1% | [-6.6%, -0.1%] | 90    |
| All ❌✅ (primary)                 | -0.8% | [-2.1%, 0.3%]  | 142   |


2 Regressions, 5 Improvements, 5 Mixed; 4 of them in rollups
30 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/06a788cbc715e02d77e998eefe5ad6d20bf95855/triage/2026/2026-03-02.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Always check `ConstArgHasType` even when otherwise ignoring](https://github.com/rust-lang/rust/pull/152931)
* [Always make tuple elements a coercion site](https://github.com/rust-lang/rust/pull/147834)
* [deny-by-default & report in deps `uninhabited_static`](https://github.com/rust-lang/rust/pull/152853)
* [Never break between empty parens](https://github.com/rust-lang/rust/issues/152761)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)

* [Remove soft_unstable](https://github.com/rust-lang/compiler-team/issues/972)
* [Parse unstable keywords for experimental syntax](https://github.com/rust-lang/compiler-team/issues/945)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Mitigation enforcement](https://github.com/rust-lang/rfcs/pull/3855)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2026-03-04 - 2026-04-01 🦀

### Virtual
* 2026-03-04 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Getting Started with Rust Part 4: Module Handling in a Project**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/313526020/)
* 2026-03-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/313303094/)
* 2026-03-05 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Presentation:  Tock OS Part #3 - Capsules and lower-level hardware drivers**](https://www.meetup.com/charlottesville-rust-meetup/events/313264830/)
* 2026-03-05 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313293173/)
* 2026-03-07 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763908777)
* 2026-03-10 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254786/)
* 2026-03-10 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/312799450/)
* 2026-03-11 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/cgzfpzcp)
* 2026-03-12 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455924/)
* 2026-03-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/313490537/)
* 2026-03-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Embedded Rust**](https://www.meetup.com/vancouver-rust/events/313471716/)
* 2026-03-18 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Hybrid event with Rust Dortmund!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/313621933/)
* 2026-03-18 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/45qqc2eo)
* 2026-03-19 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**March, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274882/)
* 2026-03-20 | Virtual | [Packt Publishing Limited](https://www.eventbrite.com/o/70306584013)
    * [**Rust Adoption, Safety, and Cloud with Francesco Ciulla**](https://www.eventbrite.com/e/rust-adoption-safety-and-cloud-with-francesco-ciulla-registration-1981847709850)
* 2026-03-24 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254785/)
* 2026-03-24 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Crates, Tips & Tricks Lightning Talks - Bring your ideas!**](https://www.meetup.com/women-in-rust/events/312799496/)
* 2026-03-25 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 03 2026**](https://luma.com/vq9w8q0w)
* 2026-03-26 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455925/)
* 2026-04-01 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/me4jwgxu)
* 2026-04-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcgbcb/)

### Asia
* 2026-03-22 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust March 2026 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/312862609/)

### Europe
* 2026-03-04 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust)
    * [**Rust at MWC Talent Arena — Workshops + Community Meetup**](https://www.meetup.com/bcnrust/events/313263086/)
* 2026-03-04 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg)
    * [**Rust Hack & Learn March 2026**](https://www.meetup.com/rust-meetup-hamburg/events/311942636/)
* 2026-03-04 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn)
    * [**Rust in March: Abstractions, but at what cost?**](https://www.meetup.com/rustcologne/events/313532986/)
* 2026-03-04 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Records, Shredded on Ice: A Primer on Parquet and Iceberg**](https://www.meetup.com/oxford-rust-meetup-group/events/312664488/)
* 2026-03-04 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #83**](https://www.meetup.com/rust-paris/events/313493454/)
* 2026-03-05 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/313464558/)
* 2026-03-11 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group)
    * [**Meetup @ Instruqt**](https://www.meetup.com/rust-amsterdam-group/events/313426708/)
* 2026-03-11 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Writing a Python compiler in Rust**](https://www.meetup.com/rust-rhein-main/events/313617138/)
* 2026-03-12 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern)
    * [**2026 Rust Talks Bern #1 @bespinian**](https://www.meetup.com/rust-bern/events/313443248/)
* 2026-03-12 | Geneva, CH | [Post Tenebras Lab](https://www.posttenebraslab.ch/)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-03-18 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - Intro to Embedded Rust - March**](https://www.meetup.com/rust-dortmund/events/313338784/)
* 2026-03-19 - 2026-03-20 | Warsaw, PL | [Rustikon](https://www.rustikon.dev/)
    * [**Rustikon Conference**](https://www.rustikon.dev/)
* 2026-03-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night - Advent of Code**](https://www.meetup.com/rust-aarhus/events/313284304/)
* 2026-03-24 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester March Code Night**](https://www.meetup.com/rust-manchester/events/313495449/)
* 2026-03-27 | Paris, FR | [Rust in Paris](https://www.rustinparis.com/)
    * [**Rust in Paris**](https://www.rustinparis.com/)
* 2026-04-01 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Rust/ACCU meetup.**](https://www.meetup.com/oxford-rust-meetup-group/events/312664491/)

### North America
* 2026-03-04 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Custom Metrics Collector & Embedded Rust!**](https://www.meetup.com/rust-nyc/events/313499010/)
* 2026-03-05 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/313529755/)
* 2026-03-05 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/313305800/)
* 2026-03-05 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Rust Project Night**](https://www.meetup.com/stl-rust/events/312654992/)
* 2026-03-07 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**MIT Rust Lunch, Mar 7**](https://www.meetup.com/bostonrust/events/313208584/)
* 2026-03-12 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**An Interpreter for Computability theory, Written the Hard Way**](https://www.meetup.com/utah-rust/events/313506767/)
* 2026-03-14 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**North End Rust Lunch, Mar 14**](https://www.meetup.com/bostonrust/events/313208587/)
* 2026-03-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/313404095/)
* 2026-03-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Embedded Rust**](https://www.meetup.com/vancouver-rust/events/313471716/)
* 2026-03-19 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**March, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274882/)
* 2026-03-19 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Applied Rust - Building Rust Applictions**](https://www.meetup.com/music-city-rust-developers/events/313576317/)
* 2026-03-21 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Porter Square Rust Lunch, Mar 21**](https://www.meetup.com/bostonrust/events/313208597/)
* 2026-03-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyjcfbhc/)
* 2026-03-26 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/311228658/)

### Oceania
* 2026-03-12 | Brisbane City, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Rust Brisbane Mar 2026**](https://www.meetup.com/rust-brisbane/events/313596218/)
* 2026-03-26 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**TBD March Meetup**](https://www.meetup.com/rust-melbourne/events/313471749/)

### South America
* 2026-03-21 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP (migrado pro Lumma)**](https://www.meetup.com/rust-sao-paulo-meetup/events/313446400/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> After all, Rust only became as good as it is by going through a rather drastic transformation. At one point it had a GC and Green Threads, famously. There's no substitute for making it exist and seeing how it does on a real problem.

– [scottmcm on rust-users](https://users.rust-lang.org/t/aliased-xor-mutable-core-for-a-high-level-language/138482/22)

Thanks to [Jonas Fassbender](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1755) for the suggestion!

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
