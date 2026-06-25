Title: This Week in Rust 657
Number: 657
Date: 2026-06-24
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

### Project/Tooling Updates
* [kache 0.7.0: caching real-world C/C++ trees](https://github.com/kunobi-ninja/kache/releases/tag/v0.7.0)
* [New Feature in GuardianDB: Introducing the ODM (Object Document Mapper) Layer](https://www.willsearch.com.br/blog/2026/06/23/new-feature-in-guardiandb-introducing-the-odm-object-document-mapper-layer/)
* [Safe SIMD in Rust, even on the inside](https://shnatsel.medium.com/safe-simd-in-rust-even-on-the-inside-c6f1ff381828)
* [Ratatui 0.30.2 is released - a Rust library for cooking up terminal user interfaces](https://ratatui.rs/highlights/v0302/)
* [Adding a post-quantum hybrid handshake to a Rust VPN](https://dev.to/alexandr_litvinov/adding-a-post-quantum-hybrid-handshake-to-a-rust-vpn-pk8)
* [From Julia to Rust: a differentiable tensor stack for scientific computing in the agentic AI era](https://tensor4all.org/blog/introducing-tenferro-rs/)

* [hotpath-rs 0.18: Profiling Async and Concurrent Rust - Channels and Lock Contention](https://hotpath.rs/blog/profiling-async-rust)

### Observations/Thoughts
* [Deep dive into iroh: A replacement for WireGuard or a peer-to-peer layer for your application?](https://kerkour.com/iroh-v1-p2p)

* [Optimizing #\[sqlx::test\] rebuild time](https://kobzol.github.io/rust/2026/06/21/optimizing-sqlx-test-rebuild-time.html)

* [Rewriting the world in Rust](https://bitfieldconsulting.com/posts/rewrite-in-rust)

### Rust Walkthroughs
* [Learn Rust Async/Await, Tokio, and TCP Networking by Building an HTTP/1.1 Server](https://blog.sheerluck.dev/posts/learn-rust-async-await-by-building-an-http-server/)
* [Building Breakout in Bevy: Step by Step](https://blog.sheerluck.dev/posts/build-breakout-in-bevy-step-by-step/)
* [Porting 300,000 Lines of C++ and Perl to Rust: A Dual-Oracle Media Metadata Engine](https://medium.com/@vbasky/porting-200-000-lines-of-c-to-rust-building-a-byte-identical-mediainfo-replacement-8e9b587d469a)
* [video] [RustCurious lesson 9: Traits are Interfaces](https://www.youtube.com/watch?v=RKojTb9IVJc)

* [A data race that doesn't compile](https://corentin-core.github.io/posts/ruxe-type-level-disjointness/)

* [Video] [BAML: a new programming language (created in Rust)](https://www.youtube.com/watch?v=X8GDc2AtbG8)
* [Video] [The Future of Version Control](https://www.youtube.com/watch?v=O3YWQvNqwHc)
* [Video] [Borrowing Beauty: My Beginner's Quest to Create Approachable Bevy & Rust Code](https://www.youtube.com/watch?v=1Xz1E_27Uqc)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [cargo-rdme](https://github.com/orium/cargo-rdme), a 

Thanks to [Diogo Sousa](https://users.rust-lang.org/t/crate-of-the-week/2704/1616) for the self-suggestion!

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
<!-- or if none - *No Calls for participation were submitted this week.* -->

* [AimDB - Non-blocking fallible `try_produce` for bounded / non-overwriting buffers](https://github.com/aimdb-dev/aimdb/issues/116)
* [AimDB - Add minimal example: hello-mailbox-async](https://github.com/aimdb-dev/aimdb/issues/99)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

515 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-06-16..2026-06-23

#### Compiler
* [implement `#[diagnostic::on_unknown]` for modules](https://github.com/rust-lang/rust/pull/157926)
* [outline part of `evaluate_goal_raw` into its own `#[cold]` function](https://github.com/rust-lang/rust/pull/158042)
* [preserve `track_caller` for by-value dyn vtable shims](https://github.com/rust-lang/rust/pull/157967)

#### Library
* [add `io::Read::read_le` and `io::Read::read_be`](https://github.com/rust-lang/rust/pull/156983)
* [constify `TryFrom<Vec>` for array](https://github.com/rust-lang/rust/pull/155616)
* [`impl [const] Default for BTreeMap`](https://github.com/rust-lang/rust/pull/157878)
* [stabilize `str_from_utf16_endian`](https://github.com/rust-lang/rust/pull/157912)
* [stabilize `strip_circumfix`](https://github.com/rust-lang/rust/pull/158012)
* [stabilize `substr_range` and `subslice_range`](https://github.com/rust-lang/rust/pull/141266)

#### Cargo
* [`diag`: Support `build.warnings` for cargo lints](https://github.com/rust-lang/cargo/pull/17112)
* [`add`: list too-new versions and how to override](https://github.com/rust-lang/cargo/pull/17117)
* [`host-config`: dont apply target config to host artifacts](https://github.com/rust-lang/cargo/pull/17123)
* [`install`: Run cargo lints like rustc lints](https://github.com/rust-lang/cargo/pull/17107)
* [`resolver`: hint how to resolve too-new versions](https://github.com/rust-lang/cargo/pull/17118)
* [`test`: skip dwp uplift test without packed debuginfo](https://github.com/rust-lang/cargo/pull/17127)
* [add Solaris fcntl file locking](https://github.com/rust-lang/cargo/pull/17110)
* [`-Zmin-publish-age`](https://github.com/rust-lang/cargo/pull/17012) (RFC [#3923](https://rust-lang.github.io/rfcs/3923-cargo-min-publish-age.html))
* [improved the test error messages when 'rustc -V' fails](https://github.com/rust-lang/cargo/pull/17108)
* [remove windows-sys dependencies older than 0.61](https://github.com/rust-lang/cargo/pull/17115)

#### Clippy
* [add lint to suggest `as_chunks` over `chunks_exact` with constant](https://github.com/rust-lang/rust-clippy/pull/16931)
* [new `unnecessary_unwrap_unchecked`: lint](https://github.com/rust-lang/rust-clippy/pull/16252)
* [`extra_unused_type_parameters`: don't suggest an autofix](https://github.com/rust-lang/rust-clippy/pull/15907)
* [`let_underscore_future`: skip bindings with an explicit type annotation](https://github.com/rust-lang/rust-clippy/pull/17001)
* [avoid ICE when evaluating constants containing unsized type args](https://github.com/rust-lang/rust-clippy/pull/16976)
* [avoid `map_unwrap_or` fix when default is adjusted](https://github.com/rust-lang/rust-clippy/pull/16928)
* [do not check for unused lifetimes in expanded code](https://github.com/rust-lang/rust-clippy/pull/17256)
* [don't trigger `unnecessary_box_returns` when the size depends on generics](https://github.com/rust-lang/rust-clippy/pull/17249)
* [find a shared context for the format string and the `format!` call](https://github.com/rust-lang/rust-clippy/pull/17243)
* [fix OOM panic for large types on uninit check](https://github.com/rust-lang/rust-clippy/pull/17205)
* [fix `std_instead_of_core`: false positives for `core::io`/MSRV](https://github.com/rust-lang/rust-clippy/pull/16964)
* [`manual_slice_fill` detect for in loops over `&mut [T; N]` slices](https://github.com/rust-lang/rust-clippy/pull/16926)
* [merge comment and cfg checking in `matches` lint pass](https://github.com/rust-lang/rust-clippy/pull/17239)
* [perf: check the method name first in `or_fun_call`](https://github.com/rust-lang/rust-clippy/pull/17266)
* [perf: compare method names before type queries in three lint passes](https://github.com/rust-lang/rust-clippy/pull/17265)
* [perf: run structural checks before const context queries in `question_mark, manual_clamp` and ranges](https://github.com/rust-lang/rust-clippy/pull/17275)
* [perf: skip `match_same_arms` work when the lint is allowed](https://github.com/rust-lang/rust-clippy/pull/17272)
* [perf: skip tokenizing in `span_contains_cfg` when no '#' is present](https://github.com/rust-lang/rust-clippy/pull/17226)
* [treat `!` the same as `-` in `unnecessary_cast`](https://github.com/rust-lang/rust-clippy/pull/17278)

#### Rust-Analyzer
* [`assists/replace_match_with_if_let`: don't parenthesize if-let guards](https://github.com/rust-lang/rust-analyzer/pull/22618)
* [`implements_trait_unique_with_infcx`: only forbid the self type from being an error type](https://github.com/rust-lang/rust-analyzer/pull/22617)
* [bye bye ted](https://github.com/rust-lang/rust-analyzer/pull/22516)
* [do not visit nodes in GC multiple times](https://github.com/rust-lang/rust-analyzer/pull/22627)
* [MIR eval mixed bit and byte sizes](https://github.com/rust-lang/rust-analyzer/pull/22594)
* [check for `#[cfg]s` in tail expression macros](https://github.com/rust-lang/rust-analyzer/pull/22599)
* [crash on static constants in array length positions](https://github.com/rust-lang/rust-analyzer/pull/22601)
* [don't complete `.await` on receivers of unknown type](https://github.com/rust-lang/rust-analyzer/pull/22486)
* [don't panic on out-of-range integer literals in const positions](https://github.com/rust-lang/rust-analyzer/pull/22621)
* [migrate merge imports to editor](https://github.com/rust-lang/rust-analyzer/pull/22351)

### Rust Compiler Performance Triage

This week had a lot of big swings, with two significant perf regressions that are accepted
because they unlock future features and perf improvements.
We also saw large improvements in the next trait solver due to the performance optimization work happening there.

Triage done by **@JonathanBrouwer** with help from **@Kobzol**.
Revision range: [b5d46ecb..8b6558a0](https://perf.rust-lang.org/?start=b5d46ecb51c3e4134b82570cfe718f093daa6390&end=8b6558a02b2774acfb25cf15e199467c37ba7490&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean   | range           | count |
|:----------------------------------:|:------:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.9%   | [0.2%, 2.7%]    | 184   |
| Regressions ❌ <br /> (secondary)  | 1.0%   | [0.1%, 4.2%]    | 160   |
| Improvements ✅ <br /> (primary)   | -0.3%  | [-0.3%, -0.2%]  | 2     |
| Improvements ✅ <br /> (secondary) | -11.8% | [-69.9%, -0.2%] | 25    |
| All ❌✅ (primary)                 | 0.8%   | [-0.3%, 2.7%]   | 186   |


5 Regressions, 3 Improvements, 2 Mixed; 4 of them in rollups
30 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/660052c17ccde865dff7c7ffd525affa0550c846/triage/2026/2026-06-21.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [rustc_lint: Allow scoped `non_ascii_idents` lint levels](https://github.com/rust-lang/rust/pull/157497)
* [Stabilize `#[my_macro] mod foo;` (part of `proc_macro_hygiene`)](https://github.com/rust-lang/rust/pull/157857)
* [Implement `IntoIterator` for `[&[mut]] Box<[T; N], A>`](https://github.com/rust-lang/rust/pull/134021)
* [Tracking Issue for `string_from_utf8_lossy_owned`](https://github.com/rust-lang/rust/issues/129436)
* [Infer all anonymous lifetimes in assoc consts as `'static`](https://github.com/rust-lang/rust/pull/156508)
* [consider subtyping when checking if an infer var is sized](https://github.com/rust-lang/rust/pull/157820)
* [remove `box_patterns`](https://github.com/rust-lang/rust/pull/156749)
* [enable eager `param_env` norm in new solver](https://github.com/rust-lang/rust/pull/156976)
* [Lint against iterator functions that panic when `N` is zero](https://github.com/rust-lang/rust/pull/153563)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Start a t-project-structure/t-comprehensibility](https://github.com/rust-lang/leadership-council/issues/298)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2026-06-24 - 2026-07-22 🦀

### Virtual
* 2026-06-25 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Weekly Session**](https://luma.com/rust-girona?e=evt-rgneLvX1H85AmjV)
* 2026-07-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/315210366/)
* 2026-07-02 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455932/)
* 2026-07-02 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Learning Game Development the Hard Way with Rust and Bevy**](https://www.meetup.com/charlottesville-rust-meetup/events/315211402/)
* 2026-07-02 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345243/)
* 2026-07-04 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-07-05 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/314095287/)
* 2026-07-07 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315060981/)
* 2026-07-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254778/)
* 2026-07-15 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314233743/)
* 2026-07-16 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**July, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314520812/)
* 2026-07-16 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/312045926/)
* 2026-07-19 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/314329045/)
* 2026-07-21 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Learning Rust as First Programming Language**](https://www.meetup.com/women-in-rust/events/315102297/)
* 2026-07-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/315279653/)

### Asia
* 2026-07-18 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**July 2026 Rustacean Meetup**](https://hasgeek.com/rustbangalore/july-2026-rustacean-meetup/)

### Europe
* 2026-06-24 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester June Talks**](https://www.meetup.com/rust-manchester/events/315200163/)
* 2026-06-24 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim)
    * [**The Chaos of Time and Time Intervals**](https://www.meetup.com/rust-trondheim/events/315298357/)
* 2026-06-25 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/314396600/)
* 2026-06-25 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #69**](https://www.meetup.com/copenhagen-rust-community/events/315214426/)
* 2026-06-25 | Toulouse, FR | [Rust Toulouse](https://www.meetup.com/rust-community-toulouse/)
    * [**Rust Toulouse Meetup - Bevy & ESP32**](https://www.meetup.com/rust-community-toulouse/events/314947457/)
* 2026-06-27 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #27**](https://www.meetup.com/stockholm-rust/events/315371143/)
* 2026-07-02 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Bevy, Bits, & Cats (Rust July Talks)**](https://www.meetup.com/rust-and-friends/events/314941098/)
* 2026-07-02 | Enschede, NL | [Baseflow Tech Meetups](https://www.meetup.com/dutch-rust-meetup)
    * [**AI Summit**](https://www.meetup.com/baseflow-tech-meetups/events/315099547/)
* 2026-07-08 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Join us live and INPERSON for Rust 262**](https://www.meetup.com/rust-dublin/events/315150327/)
* 2026-07-09 | Switzerland, CH | [PostTenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)

### North America
* 2026-06-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/315105633/)
* 2026-06-24 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust-Based Constraint Solvers in 2D Sketching with Zoo Technologies**](https://www.meetup.com/rust-los-angeles/events/314386080/)
* 2026-06-25 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539326/)
* 2026-06-25 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/314825008/)
* 2026-06-26 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC's Big Summer Social**](https://www.meetup.com/rust-nyc/events/315014582/)
* 2026-06-27 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Somerville Union Square Rust Lunch, June 27**](https://www.meetup.com/bostonrust/events/315225857/)
* 2026-07-02 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Git is easy?**](https://www.meetup.com/stl-rust/events/315103359/)
* 2026-07-04 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Boston University Rust Lunch, July 4**](https://www.meetup.com/bostonrust/events/315225861/)
* 2026-07-09 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Utah Rust July Meetup**](https://www.meetup.com/utah-rust/events/314696647/)
* 2026-07-11 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**MIT Rust Lunch, July 11**](https://www.meetup.com/bostonrust/events/315225865/)
* 2026-07-15 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314233743/)
* 2026-07-16 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**July, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314520812/)
* 2026-07-18 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**North End Rust Lunch, July 18**](https://www.meetup.com/bostonrust/events/315225872/)
* 2026-07-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/314997214/)
* 2026-07-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyjckbdc/)
* 2026-07-22 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust in Distributed Systems with Flight Science!**](https://www.meetup.com/rust-los-angeles/events/315376271/)

### Oceania
* 2026-06-25 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne June 2026**](https://www.meetup.com/rust-melbourne/events/315039461/)
* 2026-07-21 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**July Meetup**](https://www.meetup.com/rust-canberra/events/315307280/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> I think this is the wrong decision, and I wish the lang team had stabilized the Late type instead.
Better Late than Never.

– [/u/CouteauBleu on /r/rust](https://www.reddit.com/r/rust/comments/1u1v53c/the_never_type_is_likely_to_stabilize_soon/oqsxf3v/)

Thanks to [Theemathas](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1782) for the suggestion!

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
