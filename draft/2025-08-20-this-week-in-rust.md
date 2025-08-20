Title: This Week in Rust 613
Number: 613
Date: 2025-08-20
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

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [tur](https://github.com/rezigned/tur), a turing machine emulator with text-mode user interface.

Despite a lack of suggestions, llogiq is very pleased with his choice.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for feature implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your feature to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

<!-- If there are new CfT items this week, use: -->
<!--
  [Repo Name](Repo URL)
    * [<Feature name>](<Feature URL>)
        * [Testing steps](<Testing Steps URL>)
-->
<!-- where `Repo Name` and `Repo URL` are one of:
[Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
[Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing) or
[Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
-->

<!-- For all `Repo Names` with no new CfT items this week: use (removing the repos for which new
     CfT items did appear, of course) -->
<!--
* *No calls for testing were issued this week by
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*
-->
* *No calls for testing were issued this week by [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.
-->

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

390 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-08-12..2025-08-19

 #### Compiler
* [compiler: allow `extern "interrupt" fn() → !`](https://github.com/rust-lang/rust/pull/143075)
* [const-eval: full support for pointer fragments](https://github.com/rust-lang/rust/pull/144081)
* [don't warn on never to any `as` casts as unreachable](https://github.com/rust-lang/rust/pull/144804)
* [implement declarative `macro_rules!` derive macros](https://github.com/rust-lang/rust/pull/145208) (RFC [#3698](https://rust-lang.github.io/rfcs/3698-declarative-derive-macros.html))
* [implement `#[derive(From)]`](https://github.com/rust-lang/rust/pull/144922)
* [more `Printer` cleanups](https://github.com/rust-lang/rust/pull/144949)
* [tail call diagnostics to include lifetime info](https://github.com/rust-lang/rust/pull/145012)
 #### Library
* [add Ref/RefMut `try_map` method](https://github.com/rust-lang/rust/pull/118087)
* [add `Default` impls for `Pin`ned `Box`, `Rc`, `Arc`](https://github.com/rust-lang/rust/pull/143717)
* [add ASCII-related methods from `u8` and `MIN`/`MAX` to `core::ascii::Char`](https://github.com/rust-lang/rust/pull/143467)
* [change the desugaring of `assert!` for better error output](https://github.com/rust-lang/rust/pull/122661)
* [constify `SystemTime` methods](https://github.com/rust-lang/rust/pull/144519)
* [implement `ptr_cast_array`](https://github.com/rust-lang/rust/pull/144515)
* [migrate from `cfg_if` to `cfg_select`](https://github.com/rust-lang/rust/pull/145489)
* [stabilize `as_array_of_cells`](https://github.com/rust-lang/rust/pull/144054)
* [stabilize `const_exposed_provenance` feature](https://github.com/rust-lang/rust/pull/145462)
* [stabilize `core::iter::chain`](https://github.com/rust-lang/rust/pull/144963)
* [stabilize `ip_from`](https://github.com/rust-lang/rust/pull/141744)
* [stabilize `path_file_prefix` feature](https://github.com/rust-lang/rust/pull/144870)
* [stabilize `sse4a` and `tbm` target features](https://github.com/rust-lang/rust/pull/144542)
* [thread: return error if setting thread stack size fails](https://github.com/rust-lang/rust/pull/144210)
* [windows: replace `GetThreadId`+`GetCurrentThread` with `GetCurrentThreadId`](https://github.com/rust-lang/rust/pull/145412)
 #### Cargo
* [unstable: Added `-Zbuild-dir-new-layout` unstable feature](https://github.com/rust-lang/cargo/pull/15848)
* [unstable: add -Zbuild-analysis unstable feature](https://github.com/rust-lang/cargo/pull/15845)
* [package: Always reuse the workspace's target-dir](https://github.com/rust-lang/cargo/pull/15783)
* [add initial integration for `--json=timings` behind `-Zsection-timings`](https://github.com/rust-lang/cargo/pull/15780)
* [fix error while running the cargo clippy --all-targets -- -D warning](https://github.com/rust-lang/cargo/pull/15843)
* [implement `host`-target substitution](https://github.com/rust-lang/cargo/pull/15838)
* [more helpful error for invalid `cargo-features = []`](https://github.com/rust-lang/cargo/pull/15781)
* [stabilize `build.build-dir`](https://github.com/rust-lang/cargo/pull/15833)
 #### Rustdoc
* [search: search backend with partitioned suffix tree](https://github.com/rust-lang/rust/pull/144476)
* [allow multiple references to a single footnote](https://github.com/rust-lang/rust/pull/140434)
* [correct negative-to-implicit discriminant display](https://github.com/rust-lang/rust/pull/145216)
 #### Clippy
* [`similar_names` stop linting for 3-char names](https://github.com/rust-lang/rust-clippy/pull/15100)
* [`unnecessary_operation`: add space between stmts in suggestion](https://github.com/rust-lang/rust-clippy/pull/15432)
* [`{borrow,ptr}_as_ptr`: don't lint inside proc-macros](https://github.com/rust-lang/rust-clippy/pull/15473)
* [adjust `declare_interior_mutable_const` lint's category](https://github.com/rust-lang/rust-clippy/pull/15454)
* [do not suggest to use implicit `DerefMut` on `ManuallyDrop` reached through unions](https://github.com/rust-lang/rust-clippy/pull/14387)
* [fix `match_ref_pats` false positive on match scrutinee of never type](https://github.com/rust-lang/rust-clippy/pull/15474)
* [fix `unnecessary_semicolon`: don't lint on stmts with attrs](https://github.com/rust-lang/rust-clippy/pull/15466)
 #### Rust-Analyzer
* [hint at unterminated strings in unknown prefix errors](https://github.com/rust-lang/rust-analyzer/pull/20425)
* [fix "Implement default members" to resolve IdentPat](https://github.com/rust-lang/rust-analyzer/pull/20432)
* [add if..else completions in LetStmt and ArgList](https://github.com/rust-lang/rust-analyzer/pull/20390)
* [fix indent for `convert_match_to_let_else`](https://github.com/rust-lang/rust-analyzer/pull/20455)
* [make lang items query properly filter out overwritten/excluded sysroots](https://github.com/rust-lang/rust-analyzer/pull/20475)
* [only import the item in "Unqualify method call" if needed](https://github.com/rust-lang/rust-analyzer/pull/20442)
* [support guards in `replace_match_with_if_let`](https://github.com/rust-lang/rust-analyzer/pull/20456)
* [track diagnostic generations per package](https://github.com/rust-lang/rust-analyzer/pull/20459)
* [next-solver fun time](https://github.com/rust-lang/rust-analyzer/pull/20446)
* [switch from Chalk to the next trait solver](https://github.com/rust-lang/rust-analyzer/pull/20329)
* [use a more specific error message when talking about the server logs](https://github.com/rust-lang/rust-analyzer/pull/20467)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

<!-- Approved RFCs go here, use this format: * [Topic](URL) -->
<!-- or if none were approved this week, use: * *No RFCs were approved this week.* -->
<!-- * []() -->

<!--
### [Approved Major Change Proposals (MCP)](https://forge.rust-lang.org/compiler/mcp.html)
<!~~ MCPs occur infrequently, so this section is commented out by default. ~~>
<!~~ MCPs which have been approved or rejected this week go here, use this format: * [major change accepted|rejected] [Topic](URL) ~~>
-->

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
<!-- RFCs which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No RFCs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

#### Tracking Issues & PRs
<!-- Tracking Issues which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: -->
<!-- * *No Tracking Issues or PRs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
<!-- New or updated RFCs go here, use this format: * [new|updated] [Topic](URL) -->
<!-- or if there are no new or updated RFCs this week, use: -->
<!-- * *No New or Updated RFCs were created this week.* -->
<!-- * [new|updated] []() -->

## Upcoming Events

Rusty Events between 2025-08-20 - 2025-09-17 🦀

### Virtual
* 2025-08-14 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**August, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/307698880)
* 2025-08-14 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820307)
* 2025-08-17 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002458)
* 2025-08-18 | Virtual (Kenya) | [RustaceansKenya](https://lu.ma/RustaceansKenya)
    * [**Rust Embedded Series: 02: Reading Datasheets**](https://lu.ma/6vvg0s9y)
* 2025-08-19 | Virtual (Santa Clara, CA, US) | [UCSC Extension Community](https://www.meetup.com/ucsc-extension-community/events/)
    * [**Programming with Rust**](https://www.meetup.com/ucsc-extension-community/events/310108013)
* 2025-08-19 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/306757756)
* 2025-08-20 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731032)
* 2025-08-21 | Hybrid (Mexico City, MX) | [Rust MX](https://www.meetup.com/rust-mx)
    * [**Polars para análisis y manipulación de datos**](https://www.meetup.com/rust-mx/events/310408223/)
* 2025-08-21 | Virtual (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina)
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
* 2025-08-31 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002471)
* 2025-09-02 | Hybrid (Seattle, WA, US) | [RustConf](https://rustconf.com/)
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

### Asia
* 2025-08-20 | Seoul, KR | [Seoul Rust](https://www.meetup.com/rust-seoul-meetup)
    * [**Seoul Rust Meetup**](https://www.meetup.com/rust-seoul-meetup/events/310347685)
* 2025-08-23 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**August 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/august-2025-rustacean-meetup/)

### Europe
* 2025-08-13 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/310014719)
* 2025-08-13 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944036)
* 2025-08-16 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Embedded - Workshop #4 @letsboot**](https://www.meetup.com/rust-basel/events/309894848)
* 2025-08-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night - Robot Edition**](https://www.meetup.com/rust-aarhus/events/310039453)
* 2025-08-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592249)
* 2025-08-21 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310062129)
* 2025-08-28 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester August Code Night**](https://www.meetup.com/rust-manchester/events/307919168)
* 2025-08-30 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #16**](https://www.meetup.com/stockholm-rust/events/310322522)
* 2025-09-03 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**From bugs to parallelism to future-proofing: What makes Rust different**](https://www.meetup.com/rust-rhein-main/events/310322369)
* 2025-09-10 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944038)

### North America
* 2025-08-14 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**August, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/307698880)
* 2025-08-14 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Programming a Fighting Robot in Rust with Rex Magana**](https://www.meetup.com/utah-rust/events/310053631)
* 2025-08-14 | Portland, OR, US | [PDXRust](https://www.meetup.com/pdxrust)
    * [**Cross-Magic: personal projects, Rust games, and utilizing AI**](https://www.meetup.com/pdxrust/events/310364279)
* 2025-08-18 | Denver, CO, US | [FOSS Rust Colorado](https://mobilizon.us/@foss_rust_colorado/events)
    * [**FOSS Rust Hack Night**](https://mobilizon.us/events/9092695a-89f0-40fa-b3d0-50072827b0ec)
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
* 2025-09-02 - 2025-09-05 | Hybrid (Seattle, WA, US) | [RustConf](https://rustconf.com/)
    * [**RustConf 2025**](https://rustconf.com/)
* 2025-09-03 | Phoenix, AZ, US | [Desert Rust](https://www.meetup.com/desert-rustaceans)
    * [**RustConf 2025 Watch Party (Day 1)**](https://www.meetup.com/desert-rustaceans/events/310345446)
* 2025-09-04 | Phoenix, AZ, US | [Desert Rust](https://www.meetup.com/desert-rustaceans)
    * [**RustConf 2025 Watch Party (Day 2)**](https://www.meetup.com/desert-rustaceans/events/310345459)
* 2025-09-04 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**emulation of retro systems (NES, Gameboy) in Rust**](https://www.meetup.com/stl-rust/events/310116988)
* 2025-09-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Beacon Hill Rust Lunch, Sep 6**](https://www.meetup.com/bostonrust/events/310106310)

### Oceania
* 2025-08-26 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**August Meetup**](https://www.meetup.com/rust-canberra/events/308746519)
* 2025-08-27 - 2025-08-30 | Wellington, NZ | [Rust Forge](https://rustforgeconf.com/)
    * [**Rust Forge**](https://rustforgeconf.com/)

### South America
* 2025-08-21 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Agosto de Protocol Buffers!**](https://www.meetup.com/rust-argentina/events/310019573)

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

> It's amazing how far const eval has come in #Rust. It wasn't too long ago that even a simple if/else wasn't permitted. Now we're not that far off from having const trait impls and const closures, which will make damn near everything const capable.

– [Jacob Pratt on Mastodon](https://hachyderm.io/@jhpratt@mastodon.social/115052212557381430)

llogiq has looked at all zero suggestions and came up empty, so he just chose this quote instead.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
