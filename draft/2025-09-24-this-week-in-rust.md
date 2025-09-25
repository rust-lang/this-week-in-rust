Title: This Week in Rust 618
Number: 618
Date: 2025-09-24
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

* [Variadic Generics Micro Survey](https://blog.rust-lang.org/inside-rust/2025/09/22/variadic-generics-micro-survey/)

### Foundation

### Newsletters

### Project/Tooling Updates
* [Fighting human trafficking with self-contained applications](https://lwn.net/SubscriberLink/1036916/8fa1fd58807543b6/)
* [CHERI with a Linux on top](https://lwn.net/SubscriberLink/1037974/7860e9a3612d70fb/)
* [SeaORM 2.0: A closer look](https://www.sea-ql.org/blog/2025-09-24-sea-orm-2.0/)
* [GuardianDB: The Rust Implementation of OrbitDB. A peer-to-peer database for the Decentralized Web.](https://www.willsearch.com.br/)
* [Styx Emulator: A new emulation framework for DSPs, weird SoCs and embedded systems](https://stumbl.ing/posts/styx-emulator-release/)
* [GlueSQL v0.18.0 adds Send/Sync support and a new derive macro for typed row mapping](https://github.com/gluesql/gluesql/releases/tag/v0.18.0)

### Observations/Thoughts
* [Reducing binary size of (Rust) programs with debuginfo](https://kobzol.github.io/rust/2025/09/22/reducing-binary-size-of-rust-programs-with-debuginfo.html)
* [A more stable Rust Ecosystem](https://ranger-ross.github.io/blog/more-stable-ecosystem/)
* [Comparing Rust to Carbon](https://lwn.net/SubscriberLink/1036912/ecf2235a9ef774d9/)
* [Canceling asynchronous Rust](https://lwn.net/SubscriberLink/1036924/83af62ecb5f74c06/)
* [Rust Contribution Experience: From a Curious Outsider to a GreptimeDB Advocator: My Journey into Open Source Contribution](https://greptime.com/blogs/2025-09-23-greptimedb-submission-rust-contribute-guide)

### Rust Walkthroughs
* [Axum Backend Series: Models, Migration, DTOs and Repository Pattern](https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-model-setup/)
* [video] [(Kernel) Task Switching in Rust](https://www.youtube.com/watch?v=JP4-JJefY_A)

### Research

### Miscellaneous
* [Leading The Way For Safety Certified Rust: A Conversation With Espen Albrektsen Of Sonair](https://filtra.io/rust/interviews/sonair-sep-25)

## Crate of the Week

This week's crate is [faer](https://docs.rs/faer), a eneral-purpose linear algebra library for rust, with a focus on high performance for algebraic operations on medium/large matrices, as well as matrix decompositions.

Despite another week going by without a suggested weekly crate, llogiq is pleased with his choice.

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

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

- Good First Issue:
  - [Supports profile signal for OLTP](https://github.com/GreptimeTeam/greptimedb/issues/6760)
  - [Currently KILL cannot terminate queries like INSERT INTO SELECT](https://github.com/GreptimeTeam/greptimedb/issues/6334)
  - [Supports exporting compressed CSV or JSON files](https://github.com/GreptimeTeam/greptimedb/issues/6286)

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

430 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-09-16..2025-09-23

 #### Compiler
* [`-Znext-solver` allow `ExprKind::Call` for not-yet defined opaques](https://github.com/rust-lang/rust/pull/145993)
* [destinationPropagation: avoid creating overlapping assignments](https://github.com/rust-lang/rust/pull/146516)
* [detect attempt to use var-args in closure](https://github.com/rust-lang/rust/pull/146581)
* [don't apply temporary lifetime extension rules to non-extended `super let`](https://github.com/rust-lang/rust/pull/145838)
* [enable DestinationPropagation by default](https://github.com/rust-lang/rust/pull/142915)
* [lint more overlapping assignments in MIR](https://github.com/rust-lang/rust/pull/146566)
* [remove `Rvalue::Len` again](https://github.com/rust-lang/rust/pull/146564)
* [suggest removing `Box::new` instead of unboxing it](https://github.com/rust-lang/rust/pull/146259)
 #### Library
* [add `[const] PartialEq` bound to `PartialOrd`](https://github.com/rust-lang/rust/pull/146690)
* [iterator repeat: no infinite loop for `last` and `count`](https://github.com/rust-lang/rust/pull/146410)
* [make `PeekMut` generic over the allocator](https://github.com/rust-lang/rust/pull/146621)
* [specialize `Iterator::eq{_by}` for `TrustedLen` iterators](https://github.com/rust-lang/rust/pull/137122)
* [stabilize `btree_entry_insert` feature](https://github.com/rust-lang/rust/pull/144871)
* [stabilize `new_zeroed_alloc`](https://github.com/rust-lang/rust/pull/144091)
* [stabilize `std::panic::Location::file_as_c_str`](https://github.com/rust-lang/rust/pull/145664)
* [fix WASI implementation of `remove_dir_all`](https://github.com/rust-lang/rust/pull/146691)
* [merge definitions of `StdioPipes`](https://github.com/rust-lang/rust/pull/146639)
* [simplify host lookup](https://github.com/rust-lang/rust/pull/146541)
 #### Cargo
* [`fix(frontmatter)`: Improve error quality](https://github.com/rust-lang/cargo/pull/15972)
* [feat: add lint for global use of `hint-mostly-unused`](https://github.com/rust-lang/cargo/pull/15995)
 #### Rustdoc
* [rustdoc-search: javaScript optimization based on Firefox Profiler output](https://github.com/rust-lang/rust/pull/146484)
 #### Clippy
* [`match_as_ref`: do not lint if other arm is not `None => None`](https://github.com/rust-lang/rust-clippy/pull/15693)
* [`redundant_clone`: split iterator checks into `redundant_iter_cloned`](https://github.com/rust-lang/rust-clippy/pull/15277)
* [`transmute_ptr_to_ref`: don't suggest `.cast` when to-type is DST](https://github.com/rust-lang/rust-clippy/pull/15621)
* [add `clippy::self_only_used_in_recursion` lint](https://github.com/rust-lang/rust-clippy/pull/14787)
* [do not replace `.unwrap_or(vec![])` by `.unwrap_or_default()`](https://github.com/rust-lang/rust-clippy/pull/15699)
* [`nonstandard_macro_braces`: suggest trailing semicolon when needed](https://github.com/rust-lang/rust-clippy/pull/15593)
* [fix `option_if_let_else` when `Err` variant is ignored](https://github.com/rust-lang/rust-clippy/pull/14429)
* [fix `question_mark` false positive on variables used after](https://github.com/rust-lang/rust-clippy/pull/15644)
* [fix `unnecessary_semicolon` false negative on `#[feature(stmt_expr_attributes)]`](https://github.com/rust-lang/rust-clippy/pull/15481)
* [fix `unnecessary_unwrap` false negative](https://github.com/rust-lang/rust-clippy/pull/15689)
* [note that using `enumerate()` will swap the arguments](https://github.com/rust-lang/rust-clippy/pull/14969)
* [rework `module_inception`](https://github.com/rust-lang/rust-clippy/pull/14753)
* [suggestion for `rest_pat_in_fully_bound_structs`](https://github.com/rust-lang/rust-clippy/pull/15648)
 #### Rust-Analyzer
* [`hover`: unify horizontal rule formatting to `---`](https://github.com/rust-lang/rust-analyzer/pull/20379)
* [add `rust-analyzer.semanticHighlighting.comments.enable`](https://github.com/rust-lang/rust-analyzer/pull/20583)
* [fix `IfExpr` branches suggests](https://github.com/rust-lang/rust-analyzer/pull/20661)
* [fix `else` completion before `else` keyword](https://github.com/rust-lang/rust-analyzer/pull/20702)
* [fix `extract_variable` on `LetExpr`](https://github.com/rust-lang/rust-analyzer/pull/20700)
* [fix `unused_variables` shorthand record field](https://github.com/rust-lang/rust-analyzer/pull/20710)
* [fix apply in inner if for `pull_assignment_up`](https://github.com/rust-lang/rust-analyzer/pull/20722)
* [fix negative const generic integer literals](https://github.com/rust-lang/rust-analyzer/pull/20697)
* [fix not applicable on trailing comma for `remove_dbg`](https://github.com/rust-lang/rust-analyzer/pull/20714)
* [fix panics on `Foo{mut x}` for `destructure_struct_binding`](https://github.com/rust-lang/rust-analyzer/pull/20708)
* [fix to implement in-place `stdx::replace`](https://github.com/rust-lang/rust-analyzer/pull/20706)
* [fix lifetime elision handling for `Fn`-style trait bounds](https://github.com/rust-lang/rust-analyzer/pull/20725)
* [make flycheck clearing dependency-aware](https://github.com/rust-lang/rust-analyzer/pull/20689)
* [port a bunch of stuff from rustc and fix a bunch of type mismatches/diagnostics](https://github.com/rust-lang/rust-analyzer/pull/20664)

### Rust Compiler Performance Triage

Moving command-line argument quoting from C++ to Rust ([#146700](https://github.com/rust-lang/rust/pull/146700)) resulted in a nice performance
win when dealing with many dependencies and large workspaces. A somewhat costly destination propagation
compiler pass was enabled by default ([#142915](https://github.com/rust-lang/rust/pull/142915)), which resulted in some build time regressions,
but should result in improved runtime performance. The rest of changes were small.

Triage done by **@kobzol**.
Revision range: [52618eb3..ce4beebe](https://perf.rust-lang.org/?start=52618eb338609df44978b0ca4451ab7941fd1c7a&end=ce4beebecb77821734079cff47d8af08f9f27f11&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.3%  | [0.1%, 1.9%]    | 61    |
| Regressions ❌ <br /> (secondary)  | 0.6%  | [0.1%, 3.4%]    | 90    |
| Improvements ✅ <br /> (primary)   | -0.5% | [-1.9%, -0.2%]  | 29    |
| Improvements ✅ <br /> (secondary) | -1.3% | [-22.8%, -0.1%] | 71    |
| All ❌✅ (primary)                 | 0.0%  | [-1.9%, 1.9%]   | 90    |


1 Regression, 4 Improvements, 4 Mixed; 4 of them in rollups
37 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/8e7c75c12a21eb9c8c86cbfc75eff144a017f6b2/triage/2025/2025-09-23.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [the `#[track_caller]` shim should not inherit `#[no_mangle]`](https://github.com/rust-lang/rust/pull/145724)
* [Allow borrowing array elements from packed structs with ABI align <= packed align](https://github.com/rust-lang/rust/pull/145419)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Add iter! macro](https://github.com/rust-lang/rfcs/pull/3861)

## Upcoming Events

Rusty Events between 2025-09-24 - 2025-10-22 🦀

### Virtual
* 2025-09-25 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046637)
* 2025-09-28 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311046301/)
* 2025-10-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyhcnbcb)
* 2025-10-02 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/ekgdex6j)
* 2025-10-04 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763858627)
* 2025-10-05 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311062530/)
* 2025-10-07 | Virtual (Beijing, CN) | [WebAssembly and Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**Monthly WasmEdge Community Meeting, the runtime for LLM/AGI**](https://www.meetup.com/wasm-rust-meetup/events/310831771/)
* 2025-10-09 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046639/)
* 2025-10-09 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/jotnli2g)
* 2025-10-09 - 2025-10-10 | Hybrid (Paris, FR) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2025**](https://eurorust.eu/schedule/)
* 2025-10-12 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/tsjcttyhcnbqb/)
* 2025-10-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361534/)
* 2025-10-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731034/)
* 2025-10-16 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/o8fh3fh7)
* 2025-10-16 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/305646039/)
* 2025-10-19 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109167)
* 2025-10-21 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/311068625)
* 2025-10-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/310002307)

### Asia
* 2025-10-02 | Seoul, KR | [Seoul Rust (Programming Language) Meetup](https://www.meetup.com/rust-seoul-meetup/events/)
    * [**Seoul Rust Meetup**](https://www.meetup.com/rust-seoul-meetup/events/310824483)
* 2025-10-04 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**October 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/october-2025-rustacean-meetup/)
* 2025-10-08 | Kuala Lumpur, MY | [Rust Malaysia](https://t.me/rustlangmalaysia)
    * [**Malaysia Rust Meetup**](https://docs.google.com/forms/d/e/1FAIpQLScESY4eHc5lzZznAHZmFxI85CYaOKCYTQASRwXxC2y0KpI6zw/viewform)
* 2025-10-09 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup/events/)
    * [**Building Pocket-Sized Terminal UIs with Rust**](https://www.meetup.com/tokyo-rust-meetup/events/310899137/)
* 2025-10-20 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**In person Rust October 2025 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/310628902)

### Europe
* 2025-09-24 | Göteborg, SE | [Rust Göteborg](https://www.meetup.com/rustgbg/events/)
    * [**Rust Gbg — September 2025**](https://www.meetup.com/rustgbg/events/310866773)
* 2025-09-24 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2025 / 3**](https://www.meetup.com/rust-munich/events/307105978)
* 2025-09-25 | Augsburg, DE | [Rust Augsburg](https://rust-augsburg.github.io/meetup/introduction.html)
    * [**Augsburg Rust Meetup #15**](https://rust-augsburg.github.io/meetup/Meetup_15.html)
* 2025-09-25 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust meetup #61**](https://www.meetup.com/copenhagen-rust-community/events/311100221)
* 2025-09-25 | London, UK | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Women in Rust x Scala: Functional Programming in Rust & Streams with Aquascape**](https://www.meetup.com/women-in-rust/events/311056499/)
* 2025-09-27 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #18**](https://www.meetup.com/stockholm-rust/events/311027118/)
* 2025-09-30 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/events/)
    * [**LDN Talks September 2025 Community Showcase**](https://www.meetup.com/rust-london-user-group/events/311070068/)
* 2025-10-01 | Olomouc, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**4. Rust Moravia Meetup (In the capital!)**](https://www.meetup.com/rust-moravia/events/310743282)
* 2025-10-01 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Building AI chatbots with Webassembly, Rust, and Leptos**](https://www.meetup.com/oxford-rust-meetup-group/events/311170808)
* 2025-10-01 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1686673127729)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1686673127729)
* 2025-10-02 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310062134)
* 2025-10-08 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Rust Girona Hack & Learn 10 2025**](https://luma.com/8u55jo0h)
* 2025-10-08 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #79**](https://www.meetup.com/rust-paris/events/310424476)
* 2025-10-08 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944041)
* 2025-10-09 - 2025-10-10 | Hybrid (Paris, FR) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2025**](https://eurorust.eu/schedule/)
* 2025-10-14 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #13 @ letsboot**](https://www.meetup.com/rust-basel/events/310827834/)
* 2025-10-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/311035141)
* 2025-10-21 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592252)
* 2025-10-21 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group/events/)
    * [**Rust in Surgery: Powering the Data Pipelines**](https://www.meetup.com/london-rust-project-group/events/310813952)

### North America
* 2025-09-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310287849)
* 2025-09-24 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Tick, Tock, talk—find out how Rust secures embedded devices**](https://www.meetup.com/charlottesville-rust-meetup/events/310603587)
* 2025-09-24 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**RustConf Recap - The Inside Scoop!**](https://www.meetup.com/chicago-rust-meetup/events/311006846)
* 2025-09-24 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Rust/Python Interop & DB Design**](https://www.meetup.com/rust-nyc/events/311006867/)
* 2025-09-25 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Rust-Atl At Manuels Tavern**](https://www.meetup.com/rust-atl/events/308675983)
* 2025-09-25 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust on Bare Metal Series 3 : Series Finale**](https://www.meetup.com/music-city-rust-developers/events/304333261/)
* 2025-09-27 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**MIT Rust Lunch, Sep 27**](https://www.meetup.com/bostonrust/events/311038485/)
* 2025-09-30 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Evening Boston Rust Meetup with Bevy and Isograph, September 30**](https://www.meetup.com/bostonrust/events/310907806/)
* 2025-10-02 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/311004898)
* 2025-10-02 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**🚁 Rust in Flight: Lessons from Designing a 3D‑Printed Quadcopter with Embedded**](https://www.meetup.com/stl-rust/events/310279407)
* 2025-10-04 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**North End Rust Lunch, Oct 4**](https://www.meetup.com/bostonrust/events/310983705/)
* 2025-10-09 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Aya the Beholder: Writing an eBPF Firewall with the Aya Crate**](https://www.meetup.com/utah-rust/events/311145663)
* 2025-10-16 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Year In Review**](https://www.meetup.com/music-city-rust-developers/events/304333267)
* 2025-10-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/308284343)
* 2025-10-21 | San Francisco, CA, US | [Vara & Gear](https://luma.com/events-by-vara-gear)
    * [**Rust Workshop by Vara Network**](https://luma.com/kbs2os1c)
* 2025-10-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310457307)

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

> We're here to learn. We will do so relentlessly.

– [Jon Gjengset on YouTube](https://youtu.be/Wnb_n5YktO8?feature=shared&t=5645)

Thanks to [John Arundel](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1718) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
