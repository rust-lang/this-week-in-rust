Title: This Week in Rust 533
Number: 533
Date: 2024-02-07
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

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

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [embedded-cli-rs](https://github.com/funbiscuit/embedded-cli-rs), a library that makes it easy to create CLIs on embedded devices.

Thanks to [Sviatoslav Kokurin](https://users.rust-lang.org/t/crate-of-the-week/2704/1286) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker. 

<!-- CFPs go here, use this format: * [**event name**](link to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

309 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-01-30..2024-02-06

* [add avx512fp16 to x86 target features](https://github.com/rust-lang/rust/pull/119543)
* [riscv only supports `split_debuginfo=off` for now](https://github.com/rust-lang/rust/pull/120518)
* [target: default to the medium code model on LoongArch targets](https://github.com/rust-lang/rust/pull/120661)
* [`#![feature(inline_const_pat)]` is no longer incomplete](https://github.com/rust-lang/rust/pull/120547)
* [actually abort in -Zpanic-abort-tests](https://github.com/rust-lang/rust/pull/120326)
* [add missing `potential_query_instability` for keys and values in hashmap](https://github.com/rust-lang/rust/pull/120485)
* [avoid ICE when `is_val_statically_known` is not of a supported type](https://github.com/rust-lang/rust/pull/120484)
* [be more careful about interpreting a label/lifetime as a mistyped char literal](https://github.com/rust-lang/rust/pull/120460)
* [check `RUST_BOOTSTRAP_CONFIG` in `profile_user_dist` test](https://github.com/rust-lang/rust/pull/120207)
* [correctly check `never_type` feature gating](https://github.com/rust-lang/rust/pull/120552)
* [coverage: improve handling of function/closure spans](https://github.com/rust-lang/rust/pull/120569)
* [coverage: use normal `edition`: headers in coverage tests](https://github.com/rust-lang/rust/pull/120566)
* [deduplicate more sized errors on call exprs](https://github.com/rust-lang/rust/pull/120293)
* [`pattern_analysis`: Gracefully abort on type incompatibility](https://github.com/rust-lang/rust/pull/120313)
* [`pattern_analysis`: cleanup manual impls](https://github.com/rust-lang/rust/pull/120516)
* [`pattern_analysis`: cleanup the contexts](https://github.com/rust-lang/rust/pull/120321)
* [fix BufReader unsoundness by adding a check in `default_read_buf`](https://github.com/rust-lang/rust/pull/120607)
* [fix ICE on field access on a tainted type after const-eval failure](https://github.com/rust-lang/rust/pull/120616)
* [hir: refactor getters for owner nodes](https://github.com/rust-lang/rust/pull/120346)
* [hir: remove the generic type parameter from `MaybeOwned`](https://github.com/rust-lang/rust/pull/120610)
* [improve the diagnostics for unused generic parameters](https://github.com/rust-lang/rust/pull/120556)
* [introduce support for `async` bound modifier on `Fn*` traits](https://github.com/rust-lang/rust/pull/120392)
* [make matching on NaN a hard error, and remove the rest of `illegal_floating_point_literal_pattern`](https://github.com/rust-lang/rust/pull/116284)
* [make the coroutine def id of an async closure the child of the closure def id](https://github.com/rust-lang/rust/pull/120402)
* [miscellaneous diagnostics cleanups](https://github.com/rust-lang/rust/pull/120571)
* [move UI issue tests to subdirectories](https://github.com/rust-lang/rust/pull/120439)
* [move predicate, region, and const stuff into their own modules in middle](https://github.com/rust-lang/rust/pull/120497)
* [never patterns: It is correct to lower `!` to `_`](https://github.com/rust-lang/rust/pull/120517)
* [normalize region obligation in lexical region resolution with next-gen solver](https://github.com/rust-lang/rust/pull/119101)
* [only suggest removal of `as_*` and `to_` conversion methods on E0308](https://github.com/rust-lang/rust/pull/120473)
* [provide more context on derived obligation error primary label](https://github.com/rust-lang/rust/pull/120469)
* [suggest changing type to const parameters if we encounter a type in the trait bound position](https://github.com/rust-lang/rust/pull/120570)
* [suppress unhelpful diagnostics for unresolved top level attributes](https://github.com/rust-lang/rust/pull/118533)
* [miri: normalize `struct` tail in ABI compat check](https://github.com/rust-lang/rust/pull/120587)
* [miri: moving out `sched_getaffinity` interception from linux'shim, FreeBSD su…](https://github.com/rust-lang/miri/pull/3283)
* [miri: switch over to rustc's `tracing` crate instead of using our own `log` crate](https://github.com/rust-lang/miri/pull/2956)
* [revert unsound libcore changes](https://github.com/rust-lang/rust/pull/120562)
* [fix some `Arc` allocator leaks](https://github.com/rust-lang/rust/pull/120445)
* [use `<T, U>` for array/slice equality `impl`s](https://github.com/rust-lang/rust/pull/120384)
* [improve `io::Read::read_buf_exact` error case](https://github.com/rust-lang/rust/pull/120523)
* [reject infinitely-sized reads from `io::Repeat`](https://github.com/rust-lang/rust/pull/119991)
* [`thread_local::register_dtor` fix proposal for FreeBSD](https://github.com/rust-lang/rust/pull/120430)
* [add LocalWaker and ContextBuilder types to core, and LocalWake trait to alloc](https://github.com/rust-lang/rust/pull/118960)
* [codegen\_gcc: improve iterator for files suppression](https://github.com/rust-lang/rustc_codegen_gcc/pull/416)
* [cargo: Don't panic on empty spans](https://github.com/rust-lang/cargo/pull/13375)
* [cargo: Improve map/sequence error message](https://github.com/rust-lang/cargo/pull/13376)
* [cargo: apply `-Zpanic-abort-tests` to doctests too](https://github.com/rust-lang/cargo/pull/13388)
* [cargo: don't print rustdoc command lines on failure by default](https://github.com/rust-lang/cargo/pull/13387)
* [cargo: stabilize lockfile v4](https://github.com/rust-lang/cargo/pull/12852)
* [cargo: fix markdown line break in cargo-add](https://github.com/rust-lang/cargo/pull/13400)
* [cargo: use spec id instead of name to match package](https://github.com/rust-lang/cargo/pull/13335)
* [rustdoc: fix footnote handling](https://github.com/rust-lang/rust/pull/120443)
* [rustdoc: correctly handle attribute merge if this is a glob reexport](https://github.com/rust-lang/rust/pull/120501)
* [rustdoc: prevent JS injection from localStorage](https://github.com/rust-lang/rust/pull/120250)
* [rustdoc: trait.impl, type.impl: sort impls to make it not depend on serialization order](https://github.com/rust-lang/rust/pull/120641)
* [clippy: `redundant_locals`: take by-value closure captures into account](https://github.com/rust-lang/rust-clippy/pull/12227)
* [clippy: new lint: `manual_c_str_literals`](https://github.com/rust-lang/rust-clippy/pull/11919)
* [clippy: add `lint_groups_priority` lint](https://github.com/rust-lang/rust-clippy/pull/11832)
* [clippy: add new lint: `ref_as_ptr`](https://github.com/rust-lang/rust-clippy/pull/12087)
* [clippy: add configuration for `wildcard_imports` to ignore certain imports](https://github.com/rust-lang/rust-clippy/pull/11979)
* [clippy: avoid deleting labeled blocks](https://github.com/rust-lang/rust-clippy/pull/12219)
* [clippy: fixed FP in `unused_io_amount` for Ok(lit), unrachable! and unwrap de…](https://github.com/rust-lang/rust-clippy/pull/12217)
* [rust-analyzer: "Normalize import" assist and utilities for normalizing use trees](https://github.com/rust-lang/rust-analyzer/pull/16417)
* [rust-analyzer: enable excluding refs search results in test](https://github.com/rust-lang/rust-analyzer/pull/16441)
* [rust-analyzer: support for GOTO def from *inside* files included with `include!` macro](https://github.com/rust-lang/rust-analyzer/pull/16439)
* [rust-analyzer: emit parser error for missing argument list](https://github.com/rust-lang/rust-analyzer/pull/16493)
* [rust-analyzer: swap `Subtree::token_trees` from `Vec` to boxed slice](https://github.com/rust-lang/rust-analyzer/pull/16482)

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

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

<!-- Tracking Issues which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No Tracking Issues or PRs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
<!-- Remove this section if empty>

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
<!-- Remove this section if empty>

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

<!-- New or updated RFCs go here, use this format: * [new|updated] [Topic](URL) -->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- * [new|updated] []() -->

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Calls for Testing go here, use this format:
    * [<RFC Topic>](<RFC URL>)
        * [Tracking Issue](<Tracking Issue URL>)
        * [Testing steps](<Testing Steps URL>)
-->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- Remember to remove the `call-for-testing` label from the RFC so that the maintainer can signal for testers again, if desired. -->

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2024-02-07 - 2024-03-06 🦀

### Virtual

* 2024-01-31 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club launch!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/298563633/)
* 2024-02-01 | Virtual + In Person (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**12th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/297439924/) - [Stream](https://www.youtube.com/@bcnrust)
* 2024-02-01 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack n Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298457951/)
* 2024-02-03 | Virtual + In-person (Brussels, BE) | [FOSDEM 2024](https://fosdem.org/2024/)
    * [**FOSDEM Conference: Rust devroom - talks**](https://fosdem.org/2024/schedule/track/rust/)
* 2024-02-03 | Virtual (Kampala, UG) | [Rust Circle](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdsoporgprofile)
* 2024-02-04 | Virtual | [Rust Maven](https://meet-os.com/group/1)
    * [**Web development with Rocket - In English**](https://meet-os.com/event/1)
* 2024-02-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - Ezra Singh - How Rust Saved My Eyes**](https://www.meetup.com/indyrs/events/298641965/)
* 2024-02-08 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298251149/)
* 2024-02-08 | Virtual (Nürnberg, DE) | [Rust Nüremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945246/)
* 2024-02-10 | Virtual (Krakow, PL) | [Stacja IT Kraków](https://www.meetup.com/stacja-it-krakow/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-krakow/events/298303129/)
* 2024-02-10 | Virtual (Wrocław, PL) | [Stacja IT Wrocław](https://www.meetup.com/stacja-it-wroclaw/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-wroclaw/events/298303130/)
* 2024-02-13 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341575/)
* 2024-02-15 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack n Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298457899/)
* 2024-02-15 |  Virtual + In person (Praha, CZ) | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/)
    * [**Introduction and Rust in production**](https://www.meetup.com/rust-czech-republic/events/298605120/)
* 2024-02-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763497/)
* 2024-02-22 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298251150/)

### Asia

* 2024-02-10 | Hyderabad, IN | [Rust Language Hyderabad](https://www.meetup.com/rust-hyderabad/)
    * [**Rust Language Develope BootCamp**](https://www.meetup.com/rust-hyderabad/events/298687498/)

### Europe

* 2024-02-01 | Hybrid (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**12th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/297439924/)
* 2024-02-03 | Brussels, BE | [FOSDEM '24](https://fosdem.org/2024/)
    * [**FOSDEM '24 Conference: Rust devroom - talks**](https://fosdem.org/2024/schedule/track/rust/) | [**Rust Aarhus FOSDEM Meetup**](https://www.meetup.com/rust-aarhus/events/295946777/)
* 2024-02-03 | Nürnberg, BY, DE | [Paessler Rust Camp 2024](https://www.meetup.com/paessler-rust-camp-2024/)
    * [**Paessler Rust Camp 2024**](https://www.meetup.com/paessler-rust-camp-2024/events/298603948)
* 2024-02-05 | Brussels, BE | [Belgium Rust user group](https://www.meetup.com/fr-FR/belgium-rust-user-group/)
    * [**Post-FOSDEM Rust meetup @ Vrije Universiteit Brussel**](https://www.meetup.com/fr-FR/belgium-rust-user-group/events/298754029/)
* 2024-02-06 | Bremen, DE | [Rust Meetup Bremen](https://www.linkedin.com/company/rust-meetup-bremen/)
    * [**Rust Meetup Bremen [1]**](https://www.linkedin.com/events/rustmeetupbremen-17153350929486868481/)
* 2024-02-07 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/)
    * [**Embedded Abstractions**](https://www.meetup.com/rustcologne/events/298913201/) | [**Event page**](https://rust.cologne/2024/02/07/embedded-hal.html)
* 2024-02-07 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust for the Web — Mainmatter x Shuttle Takeover**](https://www.meetup.com/rust-london-user-group/events/298413388/)
* 2024-02-08 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Bern Meetup #1 2024 🦀**](https://www.meetup.com/rust-bern/events/298488858/)
* 2024-02-15 | Praha, CZ - Virtual + In-person | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/)
    * [**Introduction and Rust in production**](https://www.meetup.com/rust-czech-republic/events/298605120/)
* 2024-02-21 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #8**](https://www.meetup.com/fr-FR/rust-lyon/events/298775631/)
* 2024-02-22 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Partisia**](https://www.meetup.com/rust-aarhus/events/298689622/)

### North America

* 2024-02-07 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, Feb 7**](https://www.meetup.com/bostonrust/events/297635028/)
* 2024-02-08 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**BEAST: Recreating a classic DOS terminal game in Rust**](https://www.meetup.com/utah-rust/events/298888955/)
* 2024-02-12 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust: Open Source Contrib Hackathon & Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760219/)
* 2024-02-13 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Mixer**](https://www.meetup.com/rust-nyc/events/298593474/)
* 2024-02-13 | Seattle, WA, US | [Cap Hill Rust Coding/Hacking/Learning](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564994/)
* 2024-02-15 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Back Bay Rust Lunch, Feb 15**](https://www.meetup.com/bostonrust/events/297635043/)
* 2024-02-15 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/298631774/)
* 2024-02-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/298603354/)
* 2024-02-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/297380841/)

### Oceania

* 2024-02-06 | Perth, WA, AU | [Perth Rust Meetup Group](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Rust Feb 2024 Meetup**](https://www.meetup.com/perth-rust-meetup-group/events/297330668/)
* 2024-02-27 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**February Meetup**](https://www.meetup.com/rust-canberra/events/297650401/)
* 2024-02-27 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**🦀 spire ⚡ & Quick**](https://www.meetup.com/rust-sydney/events/298892952/)

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

> My take on this is that you cannot use async Rust correctly and fluently without understanding Arc, Mutex, the mutability of variables/references, and how async and await syntax compiles in the end. Rust forces you to understand how and why things are the way they are. It gives you minimal abstraction to do things that could’ve been tedious to do yourself.
> 
> I got a chance to work on two projects that drastically forced me to understand how async/await works. The first one is to transform a library that is completely sync and only requires a sync trait to talk to the outside service. This all sounds fine, right? Well, this becomes a problem when we try to port it into browsers. The browser is single-threaded and cannot block the JavaScript runtime at all! It is arguably the most weird environment for Rust users. It is simply impossible to rewrite the whole library, as it has already been shipped to production on other platforms.
> 
> What we did instead was rewrite the network part using async syntax, but using our own generator. The idea is simple: the generator produces a future when called, and the produced future can be awaited. But! The produced future contains an arc pointer to the generator. That means we can feed the generator the value we are waiting for, then the caller who holds the reference to the generator can feed the result back to the function and resume it. For the browser, we use the native browser API to derive the network communications; for other platforms, we just use regular blocking network calls. The external interface remains unchanged for other platforms.
> 
> Honestly, I don’t think any other language out there could possibly do this. Maybe C or C++, but which will never have the same development speed and developer experience.
> 
> I believe people have already mentioned it, but the current asynchronous model of Rust is the most reasonable choice. It does create pain for developers, but on the other hand, there is no better asynchronous model for Embedded or WebAssembly.
                    

– [/u/Top_Outlandishness78 on /r/rust](https://reddit.com/r/rust/comments/1ai1a97/let_futures_be_futures/korxtar/)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1521) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
