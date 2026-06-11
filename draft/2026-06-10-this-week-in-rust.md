Title: This Week in Rust 655
Number: 655
Date: 2026-06-10
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
* [The Embedded Rustacean Issue #73](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-73)

### Project/Tooling Updates
* [OmniScope 0.2.0 released:FFI static detection tool based on LLVM IR](https://medium.com/p/dc57a4631f8b?postPublishedType=initial)
* [Announcing Asterinas 0.18.0](https://asterinas.github.io/2026/06/04/announcing-asterinas-0.18.0.html)
* [Oryxis SSH 0.8: split panes](https://github.com/wilsonglasser/oryxis/releases/tag/v0.8.0)
* [Ratatui 0.30.1 is released - a Rust library for cooking up terminal user interfaces](https://ratatui.rs/highlights/v0301/)
* [@utoo/pack: A Next-Generation Build Tool Based on Turbopack](https://utoo.land/en/docs/blog/utoopack-intro)
* [Pico de Gallo - a USB-attached protocol bridge for developing embedded-hal drivers on your laptop](https://felipebalbi.github.io/pico-de-gallo/)
* [kache 0.5.0: designing a correct compile-cache key](https://kunobi.ninja/blog/kache-v0-5-0)

### Observations/Thoughts

* [Decimal Crates Comparison and Benchmark](https://wubingzheng.github.io/en/Decimal-Crates-Comparison.html) | [Chinese version](https://wubingzheng.github.io/zh/Decimal-Crates-Comparison.html)
* [TeaQL Robot Task Board: a Rust TUI showcase for auditable business workflows](https://teaql.io/blog/robot-task-board-showcase/)

### Rust Walkthroughs

* [series] [Who Runs Your Rust Future? Hands-On Intro to Async Rust](https://aibodh.com/posts/async-rust-chapter-1-hands-on-intro-to-async-rust/)
* [Extend MySQL Using Rust](https://villagesql.com/blog/rust/)
* [Learn Rust Smart Pointers and Interior Mutability by Building Git Commit Graph Viewer](https://blog.sheerluck.dev/posts/learn-rust-smart-pointers-and-interior-mutability-by-building-git-commit-graph-viewer/)
* * [heap underflow: classic algorithm solutions in idiomatic Rust, runnable in the browser](https://rustarians.com/heap-underflow/)

### Research

### Miscellaneous

## Crate of the Week

<!-- COTW goes here -->

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
* [cuda-oxide Windows fork - test the Windows MSVC release on more CUDA/Windows setups](https://github.com/ansidium/cuda-oxide-windows/issues/1)
* [openslate - add unit tests for slugify() in api/src/notes.rs](https://github.com/MrSheerluck/openslate/issues/38)
* [openslate - add integration tests for notes CRUD in api/src/notes.rs](https://github.com/MrSheerluck/openslate/issues/70)
* [openslate - add integration tests for auth flow in api/src/users.rs](https://github.com/MrSheerluck/openslate/issues/96)
* [openslate - add unit tests for build_fts_query() in api/src/search.rs](https://github.com/MrSheerluck/openslate/issues/89)
* [openslate - add integration tests for auth middleware and logout in api/src/auth.rs](https://github.com/MrSheerluck/openslate/issues/106)
* [openslate - add integration tests for media endpoints (DB layer) in api/src/media.rs](https://github.com/MrSheerluck/openslate/issues/85)
* [openslate - add unit tests for ext_from_mime() and filename_from_url() in api/src/media.rs](https://github.com/MrSheerluck/openslate/issues/40)
* [reliakit - add a typed_csv example to the umbrella crate](https://github.com/satyakwok/reliakit/issues/91)
* [reliakit - implement CsvField for char](https://github.com/satyakwok/reliakit/issues/92)
* [reliakit - implement CsvField for the core::net address types](https://github.com/satyakwok/reliakit/issues/107)
* [reliakit - write a short "which resilience block do I use?" guide](https://github.com/satyakwok/reliakit/issues/95)
* [reliakit - extract a reusable rolling-window counter from RollingBreaker](https://github.com/satyakwok/reliakit/issues/94)
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

<!-- Rust updates go here -->

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [`#![register_{attribute,lint}_tool]`](https://github.com/rust-lang/rfcs/pull/3808)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Document panic in `RangeInclusive::from(legacy::RangeInclusive)`](https://github.com/rust-lang/rust/pull/155421)
* [Tracking Issue for explicit-endian String::from_utf16](https://github.com/rust-lang/rust/issues/116258)
* [Tracking Issue for `substr_range` and related methods](https://github.com/rust-lang/rust/issues/126769)
* [Decide and document where stdarch intrinsics are allowed to diverge from asm behavior](https://github.com/rust-lang/rust/issues/153990)
* [Document that `ManuallyDrop`'s Box interaction has been fixed](https://github.com/rust-lang/rust/pull/155750)
* [Add temporary scope to assert_eq and assert_ne](https://github.com/rust-lang/rust/pull/155739)
* [Clean up crate type names to fix dylib vs staticlib confusion](https://github.com/rust-lang/rust/issues/153863)
* [Add `T: PartialEq` bounds to derived `StructuralPartialEq` impls.](https://github.com/rust-lang/rust/pull/156807)
* [stabilize feature `float_algebraic`](https://github.com/rust-lang/rust/pull/157029)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Deny todo!() in tidy](https://github.com/rust-lang/compiler-team/issues/999)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Rust All Hands 2027](https://github.com/rust-lang/leadership-council/issues/300)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC for convenient, explicit closure capture using move($expr) expressions](https://github.com/rust-lang/rfcs/pull/3968)

## Upcoming Events

Rusty Events between 2026-06-10 - 2026-07-08 🦀

### Virtual
* 2026-06-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/314691782/)
* 2026-06-04 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455930/)
* 2026-06-04 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345241/)
* 2026-06-04 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/)
    * [**Exploring FalkorDB - Learning to use a Graph Database in Rust**](https://www.meetup.com/code-mavens/events/314979560/)
* 2026-06-06 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-06-07 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/314095285/)
* 2026-06-08 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**RustWeek Reflections**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/315009040/)
* 2026-06-09 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254780/)
* 2026-06-09 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315060947/)
* 2026-06-10 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/3bcnx1jb)
* 2026-06-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/rdhhptyjcjbvb/)
* 2026-06-17 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/314000478/)
* 2026-06-17 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/ekws5nr4)
* 2026-06-18 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**June, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314236370/)
* 2026-06-18 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455931/)
* 2026-06-21 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/314329044/)
* 2026-06-23 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254779/)
* 2026-06-23 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: What the heck are monads - and how do we fake them in Rust**](https://www.meetup.com/women-in-rust/events/313767883/)
* 2026-07-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjckbcb/)

### Africa
* 2026-06-09 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup)
    * [**Rust by Example - Lifetimes**](https://www.meetup.com/johannesburg-rust-meetup/events/315070733/)

### Europe
* 2026-06-03 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Join us live and INPERSON for Rust 261**](https://www.meetup.com/rust-dublin/events/314689875/)
* 2026-06-03 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 06 2026**](https://luma.com/4bmlc7qd)
* 2026-06-10 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 2 - Hacking Evening**](https://www.meetup.com/rust-munich/events/313791798/)
* 2026-06-11 | Switzerland, CH | [PostTenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-06-12 - 2026-06-14 | Kraków, PL | [Rustmeet](https://rustmeet.eu/)
    * [**Rustmeet**](https://rustmeet.eu/)
* 2026-06-16 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Interactive: Everything is Open Source**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813937/)
* 2026-06-16 | Milano, IT | [Rust Language Milan](https://www.meetup.com/rust-language-milano)
    * [**Real-time planning in Rust: SolverForge & SERIO**](https://www.meetup.com/rust-language-milan/events/314766950/)
* 2026-06-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Talk Night at Danske Commodities**](https://www.meetup.com/rust-aarhus/events/314965238/)
* 2026-06-19 | Dresden, DE | [Rust Dresden](https://github.com/rust-dresden)
    * [**Second Meetup**](https://pretix.eu/rust-dresden/on-location-2)
* 2026-06-23 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #86**](https://www.meetup.com/rust-paris/events/315040676/)
* 2026-06-25 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/314396600/)

### North America
* 2026-06-04 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/314983693/)
* 2026-06-04 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Testing, Coverage, Tracey & Mutations**](https://www.meetup.com/stl-rust/events/314106244/)
* 2026-06-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Boston Common Rust Lunch, June 6**](https://www.meetup.com/bostonrust/events/314480539/)
* 2026-06-11 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Utah Rust June Meetup**](https://www.meetup.com/utah-rust/events/314696643/)
* 2026-06-11 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/314825006/)
* 2026-06-11 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust June Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/313721899/)
* 2026-06-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/314989012/)
* 2026-06-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcjbvb/)
* 2026-06-17 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/314000478/)
* 2026-06-18 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**June, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314236370/)
* 2026-06-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyjcjbgc/)
* 2026-06-24 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust-Based Constraint Solvers in 2D Sketching with Zoo Technologies**](https://www.meetup.com/rust-los-angeles/events/314386080/)
* 2026-06-25 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539326/)
* 2026-06-26 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC's Big Summer Social**](https://www.meetup.com/rust-nyc/events/315014582/)

### Oceania
* 2026-06-25 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne June 2026**](https://www.meetup.com/rust-melbourne/events/315039461/)

### South America
* 2026-06-18 | Florianópolis, BR | [Rust SC](https://luma.com/rust-sc)
    * [**Rust Floripa**](https://luma.com/acinctdf)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

<!-- QOTW goes here -->

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
