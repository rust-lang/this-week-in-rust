Title: This Week in Rust 605
Number: 605
Date: 2025-06-25
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

### Foundation

### Newsletters
* [Rust Trends Issue #67](https://rust-trends.com/newsletter/untangling-rust-errors-the-bzip2-rewrite/)

### Project/Tooling Updates
* [GlueSQL v0.17.0 - Added redb storage support](https://github.com/gluesql/gluesql/releases/tag/v0.17.0)

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [primitive\_fixed\_point\_decimal](https://docs.rs/primitive_fixed_point_decimal), a crate of *real* fixed-point decimal types.

Thanks to [Wu Bingzheng](https://users.rust-lang.org/t/crate-of-the-week/2704/1445) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Calls for Testing go here, use this format:
    * [<RFC Topic>](<RFC URL>)
        * [Tracking Issue](<Tracking Issue URL>)
        * [Testing steps](<Testing Steps URL>)
-->
<!-- or if there are no new or updated RFCs this week, use: -->
<!-- * *No New or Updated RFCs were created this week.* -->
<!-- Remember to remove the `call-for-testing` label from the RFC so that the maintainer can signal for testers again, if desired. -->

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
* [Continuwuity - Default room ACLs](https://forgejo.ellis.link/continuwuation/continuwuity/issues/775)
* [Continuwuity - Ability to entirely disable typing and read receipts](https://forgejo.ellis.link/continuwuation/continuwuity/issues/821)
* [Continuwuity - bug: appservice users are not created on registration](https://forgejo.ellis.link/continuwuation/continuwuity/issues/813)
* [Continuwuity - Invite filtering / disable invites per account](https://forgejo.ellis.link/continuwuation/continuwuity/issues/836)
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

448 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-06-17..2025-06-24

#### Compiler
* [perf: Cache the canonical *instantiation* of param-envs](https://github.com/rust-lang/rust/pull/142316)
* [asyncDrop trait without sync Drop generates an error](https://github.com/rust-lang/rust/pull/142606)
* [stabilize `generic_arg_infer`](https://github.com/rust-lang/rust/pull/141610)
* [skip no-op drop glue](https://github.com/rust-lang/rust/pull/142508)

#### Library
* [add `trim_prefix` and `trim_suffix` methods for both `slice` and `str` types](https://github.com/rust-lang/rust/pull/142331)
* [allow comparisons between `CStr`, CString`, and Cow<CStr>`](https://github.com/rust-lang/rust/pull/137268)
* [allow storing `format_args!()` in variable](https://github.com/rust-lang/rust/pull/140748)
* [impl `Default` for `array::IntoIter`](https://github.com/rust-lang/rust/pull/141574)
* [change `core::iter::Fuse`'s `Default` impl to do what its docs say it does](https://github.com/rust-lang/rust/pull/140985)
* [let String pass `#[track_caller]` to its Vec calls](https://github.com/rust-lang/rust/pull/142728)
* [safer implementation of RepeatN](https://github.com/rust-lang/rust/pull/130887)
* [use a distinct `ToString` implementation for `u128` and `i128`](https://github.com/rust-lang/rust/pull/142294)

#### Cargo
* [cargo: `feat(toml)`: Parse support for multiple build scripts](https://github.com/rust-lang/cargo/pull/15630)
* [cargo: feat: introduce perma unstable `--compile-time-deps` option for `cargo build`](https://github.com/rust-lang/cargo/pull/15674)
* [cargo: fix potential deadlock in `CacheState::lock`](https://github.com/rust-lang/cargo/pull/15698)

#### Rustdoc
* [avoid a few more allocations in `write_shared.rs`](https://github.com/rust-lang/rust/pull/142667)
* [rustdoc-json: keep empty generic args if parenthesized](https://github.com/rust-lang/rust/pull/142932)
* [rustdoc: make srcIndex no longer a global variable](https://github.com/rust-lang/rust/pull/142100)

#### Clippy
* [use jemalloc for Clippy](https://github.com/rust-lang/rust/pull/142286)
* [perf: Don't spawn so many compilers (3/2) (19m → 250k)](https://github.com/rust-lang/rust-clippy/pull/15030)
* [`Sugg`: do not parenthesize a double unary operator](https://github.com/rust-lang/rust-clippy/pull/14983)
* [`or_fun_call`: lint more methods](https://github.com/rust-lang/rust-clippy/pull/15071)
* [add missing space when expanding a struct-like variant](https://github.com/rust-lang/rust-clippy/pull/15096)
* [check MSRV before suggesting applying `const` to a function](https://github.com/rust-lang/rust-clippy/pull/15080)
* [emit lint about redundant closure on the closure node itself](https://github.com/rust-lang/rust-clippy/pull/14791)
* [fix `branches_sharing_code` suggests misleadingly when in assignment](https://github.com/rust-lang/rust-clippy/pull/15076)
* [fix `clippy::question_mark` on let-else with cfg](https://github.com/rust-lang/rust-clippy/pull/15082)
* [fix `exhaustive_structs` false positive on structs with default valued field](https://github.com/rust-lang/rust-clippy/pull/15022)
* [fix `manual_ok_err` suggests wrongly with references](https://github.com/rust-lang/rust-clippy/pull/15053)
* [fix `non_copy_const` ICE](https://github.com/rust-lang/rust-clippy/pull/15083)
* [fix `wildcard_enum_match_arm` suggests wrongly with raw identifiers](https://github.com/rust-lang/rust-clippy/pull/15093)
* [fix false positive of `borrow_deref_ref`](https://github.com/rust-lang/rust-clippy/pull/14967)
* [fix suggestion-causes-error of `empty_line_after_outer_attr`](https://github.com/rust-lang/rust-clippy/pull/15078)
* [new lint: `manual_is_multiple_of`](https://github.com/rust-lang/rust-clippy/pull/14292)

#### Rust-Analyzer
* [rust-analyzer: add `fn parent(self, db) → GenericDef` to `hir::TypeParam`](https://github.com/rust-lang/rust-analyzer/pull/20046)
* [rust-analyzer: cleanup `folding_ranges` and support more things](https://github.com/rust-lang/rust-analyzer/pull/20080)
* [rust-analyzer: do not default to 'static for trait object lifetimes](https://github.com/rust-lang/rust-analyzer/pull/20036)
* [rust-analyzer: closure capturing for let exprs](https://github.com/rust-lang/rust-analyzer/pull/20039)
* [rust-analyzer: fix cargo project manifest not pointing to the workspace root](https://github.com/rust-lang/rust-analyzer/pull/20069)
* [rust-analyzer: in "Wrap return type" assist, don't wrap exit points if they already have the right type](https://github.com/rust-lang/rust-analyzer/pull/20061)
* [rust-analyzer: respect `.cargo/config.toml build.target-dir`](https://github.com/rust-lang/rust-analyzer/pull/20072)
* [rust-analyzer: temporarily disable `+` typing handler as it moves the cursor position](https://github.com/rust-lang/rust-analyzer/pull/20042)
* [rust-analyzer: use `ROOT` hygiene for `args` inside new `format_args!` expansion](https://github.com/rust-lang/rust-analyzer/pull/20073)
* [rust-analyzer: hide imported privates if private editable is disabled](https://github.com/rust-lang/rust-analyzer/pull/20025)
* [rust-analyzer: mimic rustc's new `format_args!` expansion](https://github.com/rust-lang/rust-analyzer/pull/20056)

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

Rusty Events between 2025-06-25 - 2025-07-23 🦀

### Virtual
* 2025-06-18 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Panics and FFI Boundaries**](https://www.meetup.com/vancouver-rust/events/307730493)
* 2025-06-19 | Hybrid (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**June, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 2025-06-19 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820303)
* 2025-06-19 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/vna66he1)
* 2025-06-22 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/308246353)
* 2025-06-24 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361436)
* 2025-06-24 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Building Efficient Web Scrapers: Rust vs. Python for Data Ingestion**](https://www.meetup.com/women-in-rust/events/306683025)
* 2025-06-25 | Virtual (Lima, PE)| [Perú Rust User Group](https://www.meetup.com/peru-rust-user-group/)
    * [**Interfaces y Costos en la nube con Rust**](https://www.meetup.com/peru-rust-user-group/events/308543965/)
* 2025-06-26 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/cgamfls6)
* 2025-06-26 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567869)
* 2025-06-29 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/kvqfrtyhcjbmc)
* 2025-07-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031667)
* 2025-07-03 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820304)
* 2025-07-03 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #11**](https://www.meetup.com/bevy-game-development/events/308463394)
* 2025-07-05 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2025-07-06 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/308298511)
* 2025-07-08 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361452)
* 2025-07-13 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/308298512)
* 2025-07-15 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/307560349)
* 2025-07-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/306757755)
* 2025-07-16 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731031)


### Asia
* 2025-06-28 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**June 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/june-2025-rustacean-meetup/)
* 2025-07-02 | Seoul, KR | [Seoul Rust (Programming Language) Meetup](https://www.meetup.com/rust-seoul-meetup/events/)
    * [**Seoul Rust Meetup**](https://www.meetup.com/rust-seoul-meetup/events/308408246)

### Europe
* 2025-06-18 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Rust Meetup @Magello**](https://www.meetup.com/stockholm-rust/events/308129156)
* 2025-06-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Rust Aarhus meetup at Trifork**](https://www.meetup.com/rust-aarhus/events/308060489)
* 2025-06-19 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (evening pub)**](https://www.meetup.com/rust-and-friends/events/308023524)
* 2025-06-20 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/308023512)
* 2025-06-23 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/events/)
    * [**Rust London: Rust Hack & Learn June 2025**](https://www.meetup.com/rust-london-user-group/events/308529202)
* 2025-06-24 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester June Code Night**](https://www.meetup.com/rust-manchester/events/307919158)
* 2025-06-25 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group)
    * [**Lessons learnt from making a tiny game in nostd Rust**](https://www.meetup.com/london-rust-project-group/events/306809962)
* 2025-06-25 | Paris, FR | [Systematic Paris Region](https://systematic-paris-region.org/)   
    * [**Rust Paris Conference 2025**](https://my.weezevent.com/rust-paris-2025)
* 2025-06-26 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**18th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/308399403)
* 2025-06-26 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #58**](https://www.meetup.com/copenhagen-rust-community/events/308161212)
* 2025-06-26 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #77**](https://www.meetup.com/rust-paris/events/308416060)
* 2025-06-30 | Zagreb, HR | [impl Zagreb for Rust](https://www.meetup.com/zagreb-rust-meetup/events/)
    * [**Meetup 2025/06: Drink-up zatvaranje sezone**](https://www.meetup.com/zagreb-rust-meetup/events/308477879)
* 2025-07-01 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #9**](https://www.meetup.com/rust-gdansk/events/308349712)
* 2025-07-02 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #12 @ kHaus**](https://www.meetup.com/rust-basel/events/307567391)
* 2025-07-02 | London, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust and ACCU special - Vibe coding workshop**](https://www.meetup.com/oxford-rust-meetup-group/events/308435063/)
* 2025-07-02 | Posnan, PL | [Rust Poland](https://www.meetup.com/rust-poland-meetup/)
    * [**Rust Poland Meetup x Poznan**](https://www.meetup.com/rust-poland-meetup/events/308480357)
* 2025-07-05 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #13**](https://www.meetup.com/stockholm-rust/events/308530949)
* 2025-07-08 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group/events/)
    * [**Garbage Collection for Rust: the Finalizer Frontier**](https://www.meetup.com/london-rust-project-group/events/308443710)
* 2025-07-09 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 07 2025**](https://lu.ma/hismn492)
* 2025-07-09 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtyhckbmb)
* 2025-07-15 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group/events/)
    * [**TUI Power: Simulating & Visualising Sensor Data with Rust**](https://www.meetup.com/london-rust-project-group/events/308434768)


### North America
* 2025-06-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307730493)
* 2025-06-19 | Hybrid (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**June, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 2025-06-19 | México City, MX | [Rust MX](https://www.meetup.com/rust-mx)
    * [**programación asíncrona en Rust usando Tokio**](https://www.meetup.com/rust-mx/events/308248260)
* 2025-06-19 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Using Rust For Web Series 3 : Final Presentations and Community Social**](https://www.meetup.com/music-city-rust-developers/events/304333108)
* 2025-06-19 | Redmond, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**June, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 2025-06-20 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Lechmere Rust Lunch, June 20**](https://www.meetup.com/bostonrust/events/307936242)
* 2025-06-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcjbhc)
* 2025-06-26 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles/events/)
    * [**Rust in Web3 Developer Group**](https://www.meetup.com/rust-los-angeles/events/308401269)
* 2025-06-26 | Los Angeles (Chino Hills), CA, US | [Vara Network](https://lu.ma/events-by-vara-gear)
    * [**Rust in Web3**](https://lu.ma/ek8jx2r3)
* 2025-06-26 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust)
    * [**Monthly Meetup: Making a CRUD API with Rust!**](https://www.meetup.com/spokane-rust/events/307969600)
* 2025-06-28 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Back Bay Rust Lunch, June 28**](https://www.meetup.com/bostonrust/events/307936269)
* 2025-07-03 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal/events/)
    * [**July Monthly Social**](https://www.meetup.com/rust-montreal/events/308532058)
* 2025-07-03 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Building Resilient and Observable Rust Services with steady_state**](https://www.meetup.com/stl-rust/events/306345853)
* 2025-07-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Alewife Rust Lunch, July 6**](https://www.meetup.com/bostonrust/events/307936287)
* 2025-07-09 | Phoenix, AZ, US | [Desert Rust](https://www.meetup.com/desert-rustaceans/events/)
    * [**Rust <> AI**](https://www.meetup.com/desert-rustaceans/events/308507249/)
* 2025-07-15 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/307931266)

### Oceania
* 2025-06-24 | Barton, AC, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/events/)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/307520854)
* 2025-06-30 | Collingwood, VI, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/events/)
    * [**June 2025 Mini Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/308546374)

### South America
* 2025-07-12 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup/events/)
    * [**Encontro do Rust-SP na WillBank**](https://www.meetup.com/rust-sao-paulo-meetup/events/307308851)

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

> Our experience is that no matter how many safeguards you put on code, there’s no cure-all that prevents bad programming. Of course, to take the contrary argument, seat belts don’t stop all traffic fatalities, but you could just choose not to have accidents. So we do have seat belts. If Rust can prevent some mistakes or malicious intent, maybe it’s worth it even if it isn’t perfect.

– [Al Williams on hackaday](https://hackaday.com/2025/06/21/if-your-kernel-development-is-a-little-rusty/)

Thanks to [Kill The Mule](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1700) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
