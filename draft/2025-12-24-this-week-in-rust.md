Title: This Week in Rust 631
Number: 631
Date: 2025-12-24
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

### Foundation

### Newsletters
* [The Embedded Rustacean Issue #61](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-61)
* [Scientific Computing in Rust #13 (December 2025)](https://scientificcomputing.rs/monthly/2025-12)

### Project/Tooling Updates

* [cargo-coupling: Visualizing Coupling in Rust Projects](https://syu-m-5151.hatenablog.com/entry/2025/12/21/152559)
* [Announcing Asterinas 0.17.0](https://asterinas.github.io/2025/12/19/announcing-asterinas-0.17.0.html)
* [Tuitar - A portable guitar training tool & DIY kit](https://github.com/orhun/tuitar/releases/tag/v0.1.0)

### Observations/Thoughts

* [audio] [Netstack.FM episode 19 — Firezone and Zero-Trust Network Security with Thomas Eizinger](https://netstack.fm/#episode-19)

### Rust Walkthroughs
* [Rust Unit Testing: Basic HTTP Server](https://jorgeortiz.dev/posts/rust_unit_testing_basic_http_srvr/)
* [Async Rust Bluetooth Plumbing: Where the Throughput Goes](https://medium.com/@potto_94870/async-rust-bluetooth-plumbing-where-the-throughput-goes-d2cf21430a90)
* [series] [Part 2: Tensor Operations, Building an LLM from Scratch in Rust](https://www.tag1.com/how-to/part2-tensor-operations-building-an-llm-from-scratch/)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [arcshift](https://docs.rs/arcshift), an Arc replacement for read-heavy workloads that supports lock-free atomic replacement.

Thanks to [rustkins](https://users.rust-lang.org/t/crate-of-the-week/2704/1510) for the suggestion!

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

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
* [**RustWeek 2026**](https://sessionize.com/rustweek-2026/) | CFP closes 2026-01-18 | Utrecht, The Netherlands | 2026-05-19 - 2026-05-20
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

475 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-12-16..2025-12-23

#### Compiler
* [add `target_feature = "gc"` for Wasm](https://github.com/rust-lang/rust/pull/150111)
* [better closure requirement propagation](https://github.com/rust-lang/rust/pull/148329)
* [correctly encode doc attribute metadata](https://github.com/rust-lang/rust/pull/149919)
* [don't treat asserts as a call in cross-crate inlining](https://github.com/rust-lang/rust/pull/117192)
* [improve filenames encoding and misc](https://github.com/rust-lang/rust/pull/149989)
* [make closure capturing have consistent and correct behaviour around patterns](https://github.com/rust-lang/rust/pull/138961)
* [support recursive delegation](https://github.com/rust-lang/rust/pull/150024)

#### Library
* [add `try_as_dyn` and `try_as_dyn_mut`](https://github.com/rust-lang/rust/pull/150033)
* [add const default for OnceCell and OnceLock](https://github.com/rust-lang/rust/pull/149812)
* [expand `str_as_str` to more types](https://github.com/rust-lang/rust/pull/145933)
* [make `const BorrowMut` require `const Borrow` and make `const Fn` require `const FnMut`](https://github.com/rust-lang/rust/pull/147939)
* [hashbrown: add `hash_map::{OccupiedEntry::into_entry`, `VacantEntryRef::insert_entry_with_key}`, make `EntryRef` use `ToOwned` again](https://github.com/rust-lang/hashbrown/pull/670)
* [hashbrown: add `hash_table::OccupiedEntry::replace_entry_with` to mirror HashMap API](https://github.com/rust-lang/hashbrown/pull/669)
* [hashbrown: add `hash_table::UnsafeIter`, `iter()` method to various iterators](https://github.com/rust-lang/hashbrown/pull/667)

#### Rustdoc
* [Add missing close tags in extern crate reexports](https://github.com/rust-lang/rust/pull/150185)
* [Fix invalid handling of field followed by negated macro call](https://github.com/rust-lang/rust/pull/150099)
* [generate macro expansion for rust compiler crates docs](https://github.com/rust-lang/rust/pull/150022)
* [handle macro expansions in types](https://github.com/rust-lang/rust/pull/150221)

#### Clippy
* [`transmuting_null`: Check const integer casts](https://github.com/rust-lang/rust-clippy/pull/16227)
* [allow multiline suggestions in `map-unwrap-or`](https://github.com/rust-lang/rust-clippy/pull/16114)
* [do not attempt to use `nth` with non-usize argument](https://github.com/rust-lang/rust-clippy/pull/16272)
* [don't emit `collapsible_else_if` lint when all arms contain only `if {} else {}` expressions](https://github.com/rust-lang/rust-clippy/pull/16286)
* [fix `cmp_null` missing parens in the example](https://github.com/rust-lang/rust-clippy/pull/16282)
* [fix `empty_enum_variants_with_brackets` misses removing brackets in patterns](https://github.com/rust-lang/rust-clippy/pull/16160)
* [fix `if_then_some_else_none` suggests wrongly when then ends with comment](https://github.com/rust-lang/rust-clippy/pull/16278)
* [fix `needless_type_cast` suggesting invalid code for non-literal initializers](https://github.com/rust-lang/rust-clippy/pull/16248)
* [fix `println_empty_string` suggestion caused error](https://github.com/rust-lang/rust-clippy/pull/16201)
* [fix `use_self` false positive on type in const generics](https://github.com/rust-lang/rust-clippy/pull/16172)
* [fix an incorrect error message regarding the size of `usize` and `isize` in `cast_precision_loss`](https://github.com/rust-lang/rust-clippy/pull/14966)
* [move `collapsible_else_if` to `pedantic`](https://github.com/rust-lang/rust-clippy/pull/16211)
* [new lint - `same_length_and_capacity`](https://github.com/rust-lang/rust-clippy/pull/15656)

#### Rust-Analyzer
* [add 'Use of AI tools' section to CONTRIBUTING.md](https://github.com/rust-lang/rust-analyzer/pull/21314)
* [add BreakExpr completion suggest](https://github.com/rust-lang/rust-analyzer/pull/20521)
* [add an lsp extension to get failed obligations for a given function](https://github.com/rust-lang/rust-analyzer/pull/21309)
* [add default varname for TryEnum postfix completion](https://github.com/rust-lang/rust-analyzer/pull/21212)
* [add guess braces doc `T![]` for `T_`](https://github.com/rust-lang/rust-analyzer/pull/20439)
* [add ide-assist: `add_explicit_method_call_deref`](https://github.com/rust-lang/rust-analyzer/pull/20996)
* [complete reference `&T` → `&&T`](https://github.com/rust-lang/rust-analyzer/pull/21289)
* [introduce `crate_attrs` field in `rust-project.json`](https://github.com/rust-lang/rust-analyzer/pull/21282)
* [pretty print attributes up to `cfg(false)`](https://github.com/rust-lang/rust-analyzer/pull/21298)
* [fix applicable on non naked if for `move_guard` assist](https://github.com/rust-lang/rust-analyzer/pull/21293)
* [fix guess renamed macro braces](https://github.com/rust-lang/rust-analyzer/pull/20438)
* [fix indent for `convert_iter_for_each_to_for`](https://github.com/rust-lang/rust-analyzer/pull/20595)
* [fix indent for `merge_nested_if`](https://github.com/rust-lang/rust-analyzer/pull/20577)
* [fix match arm nested body invalid expected type](https://github.com/rust-lang/rust-analyzer/pull/21291)
* [fix nested if-let for `merge_nested_if`](https://github.com/rust-lang/rust-analyzer/pull/20576)
* [fix flycheck generations not being synced for multiple workspaces](https://github.com/rust-lang/rust-analyzer/pull/21326)
* [more perf improvements, made possible after non-Salsa interneds](https://github.com/rust-lang/rust-analyzer/pull/21307)
* [non-Salsa-interned solver types - with GC for them](https://github.com/rust-lang/rust-analyzer/pull/21295)
* [remove conflicting advice](https://github.com/rust-lang/rust-analyzer/pull/20472)
* [support undotted-self for `this` param closure](https://github.com/rust-lang/rust-analyzer/pull/21166)

### Rust Compiler Performance Triage

Very quiet week, with essentially no change in performance.

Triage done by **@simulacrum**.
Revision range: [21ff67df..e1212ea7](https://perf.rust-lang.org/?start=21ff67df15329dd7548ccba54b6c6ae9a562124f&end=e1212ea79b38d51954625291c04d2797c4bb8ec5&absolute=false&stat=instructions%3Au)

1 Regression, 1 Improvement, 3 Mixed; 2 of them in rollups
36 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025/2025-12-22.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:
* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Explicitly export core and std macros](https://github.com/rust-lang/rust/pull/139493)
* [Stabilize 29 RISC-V target features (`riscv_ratified_v2`)](https://github.com/rust-lang/rust/pull/145948)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [feat(toml): TOML 1.1 parse support](https://github.com/rust-lang/cargo/pull/16415)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Allow combining `--help -C help -Z help -W help` in one invocation](https://github.com/rust-lang/compiler-team/issues/954)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period)
* [Raise travel grant limit to $100,000 for 2026](https://github.com/rust-lang/leadership-council/pull/254)
* [Fund program management program for 2026](https://github.com/rust-lang/leadership-council/issues/255)
* [Raise automatic travel grant to $2000](https://github.com/rust-lang/leadership-council/pull/257)

*No Items entered Final Comment Period this week for
  [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) or
  [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Thin pointers with inline metadata](https://github.com/rust-lang/rfcs/pull/3898)

## Upcoming Events

Rusty Events between 2025-12-24 - 2026-01-21 🦀

### Virtual
* 2025-12-30 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/)
    * [**Live Open Source Rust project contribution**](https://www.meetup.com/code-mavens/events/312554028/)
* 2026-01-03 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763888717)
* 2026-01-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/312102790/)
* 2026-01-08 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Meet, swap, and learn!**](https://www.meetup.com/charlottesville-rust-meetup/events/312321120/)
* 2026-01-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/312379275/)
* 2026-01-13 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254791/)
* 2026-01-13 | Virtual | [libp2p Events](https://luma.com/libp2p)
    * [**rust-libp2p Open Maintainers Call**](https://luma.com/xov10pef)
* 2026-01-15 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305646023/)
* 2026-01-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/312489197/)
* 2026-01-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Stack Safety**](https://www.meetup.com/vancouver-rust/events/310619449/)

### Asia
* 2026-01-07 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**In person Rust January 2026 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/311759516/)

### Europe
* 2026-01-07 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Meetup @ Instruqt**](https://www.meetup.com/rust-amsterdam-group/events/312497150/)
* 2026-01-07 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 01 2026**](https://luma.com/mdymp686)
* 2026-01-08 | Geneva, CH | [Post Tenebras Lab](https://www.posttenebraslab.ch)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-01-14 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/csvcvtyjccbsb/)
* 2026-01-20 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592260/)
* 2026-01-20 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #82**](https://www.meetup.com/rust-paris/events/312364675/)

### North America
* 2025-12-27 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Lechmere Rust Lunch, Dec 27**](https://www.meetup.com/bostonrust/events/312483556/)
* 2026-01-03 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Allston Rust Lunch, Jan 3**](https://www.meetup.com/bostonrust/events/312483562/)
* 2026-01-08 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/jqvvttyjccblb/)
* 2026-01-10 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Central Cambridge Rust Lunch, Jan 10**](https://www.meetup.com/bostonrust/events/312483605/)
* 2026-01-15 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**Janurary, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311814876/)
* 2026-01-17 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Boston Common Rust Lunch, Jan 17**](https://www.meetup.com/bostonrust/events/312483677/)
* 2026-01-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/310403081/)
* 2026-01-21 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/312185794/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> they should just rename `unsafe` to `C` so people can shut up

– [/u/thisismyfavoritename on /r/rust](https://www.reddit.com/r/rust/comments/1pp3y9e/comment/nukdfn4/)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1739) for the suggestion!

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
