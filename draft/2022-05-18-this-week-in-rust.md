Title: This Week in Rust 443
Number: 443
Date: 2022-05-18
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
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

* [Asteracea (platform-agnostic mixed-style frontend components)](https://blog.schichler.dev/posts/Asteracea/)
* [video] [Building a Postgres storage system in Rust](https://www.youtube.com/watch?v=kAQeout-mh8)

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [cargo-supply-chain](https://crates.io/crates/cargo-supply-chain), a cargo subcommand to gather author, contributor and publisher data on crates in your dependency graph.

Despite a lack of nominations, llogiq is pleased with his choice.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

341 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-05-09..2022-05-16

* [Add `EarlyBinder`](https://github.com/rust-lang/rust/pull/96883)
* [rustc: Stricter checking for `#[link]` attributes](https://github.com/rust-lang/rust/pull/96885)
* [Detect trait fulfillment in `subst_and_check_impossible_predicates`](https://github.com/rust-lang/rust/pull/96808)
* [Gracefully fail to resolve associated items instead of `delay_span_bug`](https://github.com/rust-lang/rust/pull/96806)
* [Stop suggesting non-existing fully qualified paths](https://github.com/rust-lang/rust/pull/96940)
* [Implement a lint to warn about unused macro rules](https://github.com/rust-lang/rust/pull/96150)
* [Check hidden types for well formedness at the definition site instead of only at the opaque type itself](https://github.com/rust-lang/rust/pull/96736)
* [Drop tracking: handle invalid assignments better](https://github.com/rust-lang/rust/pull/97031)
* [Forbid nested opaque types to reference HRTB from opaque types](https://github.com/rust-lang/rust/pull/97039)
* [Don't subst an `AdtDef` with its own substs](https://github.com/rust-lang/rust/pull/96882)
* [miri: Add a command line flag to avoid printing to stdout and stderr](https://github.com/rust-lang/miri/pull/2084)
* [miri: Print spans where tags are created and invalidated](https://github.com/rust-lang/miri/pull/2030)
* [miri: Use atomic RMW for `{mutex, rwlock, cond, srwlock}_get_or_create_id` functions](https://github.com/rust-lang/miri/pull/2114)
* [Initial work on Miri permissive-exposed-provenance](https://github.com/rust-lang/rust/pull/95826)
* [Use `FxIndexSet` to avoid sorting fake borrows](https://github.com/rust-lang/rust/pull/96888)
* [make sure `ScalarPair` enums have `ScalarPair` variants; add some layout sanity checks](https://github.com/rust-lang/rust/pull/96872)
* [optimize `insert_range` method of `IntervalSet`](https://github.com/rust-lang/rust/pull/96895)
* [Make `BorrowedFd::borrow_raw` a const fn](https://github.com/rust-lang/rust/pull/96232)
* [`ExitCode::exit_process()` method](https://github.com/rust-lang/rust/pull/95356)
* [Fix `array::IntoIter::fold` to use the optimized `Range::fold`](https://github.com/rust-lang/rust/pull/95602)
* [futures: Make `run_until_stalled` handle self-waking futures](https://github.com/rust-lang/futures-rs/pull/2593)
* [futures: Remove TryStreamExt::into_async_read Unpin bound](https://github.com/rust-lang/futures-rs/pull/2599)
* [cargo: Pass `--target` to `rustdoc` for `cargo test` if specified with host target](https://github.com/rust-lang/cargo/pull/10594)
* [cargo install: Support `foo@version` like cargo-add](https://github.com/rust-lang/cargo/pull/10650)
* [cargo yank: Support foo@version like cargo-add](https://github.com/rust-lang/cargo/pull/10597)
* [rustdoc: correct path to type alias methods](https://github.com/rust-lang/rust/pull/96887)
* [rustdoc: search result ranking fix](https://github.com/rust-lang/rust/pull/96879)
* [clippy: Add `duplicate_mod` lint](https://github.com/rust-lang/rust-clippy/pull/8832)
* [clippy: Add version filtering option to the lint list](https://github.com/rust-lang/rust-clippy/pull/8752)
* [clippy: Don't lint `vec_init_then_push` when further extended](https://github.com/rust-lang/rust-clippy/pull/8699)
* [clippy: Fix ICE in `let_unit_value` when calling a static or const callable type](https://github.com/rust-lang/rust-clippy/pull/8835)
* [clippy: Fix `match_single_binding` suggestion for assign expressions](https://github.com/rust-lang/rust-clippy/pull/8726)
* [clippy: Fix `redundant_allocation` warning for `Rc<Box<str>>`](https://github.com/rust-lang/rust-clippy/pull/8813)
* [clippy: New lint: `derive_partial_eq_without_eq`](https://github.com/rust-lang/rust-clippy/pull/8796)
* [clippy: Rename `eval_order_dependence` to `mixed_read_write_expression`, move to nursery](https://github.com/rust-lang/rust-clippy/pull/8621)
* [clippy: `undocumented_unsafe_blocks` does not trigger on unsafe trait impls](https://github.com/rust-lang/rust-clippy/pull/8761)
* [clippy: introduce `rc_clone_in_vec_init` lint](https://github.com/rust-lang/rust-clippy/pull/8769)
* [rust-analyzer: Config revamp](https://github.com/rust-lang/rust-analyzer/pull/12010)
* [rust-analyzer: Add binding mode inlay hints](https://github.com/rust-lang/rust-analyzer/pull/12253)
* [rust-analyzer: Allow reborrow inlay hints to be restricted to mutable reborrows only](https://github.com/rust-lang/rust-analyzer/pull/12226)
* [rust-analyzer: Change VSCode extension publisher to `rust-lang`](https://github.com/rust-lang/rust-analyzer/pull/12238)
* [rust-analyzer: Handle getters and setters in documentation template assist](https://github.com/rust-lang/rust-analyzer/pull/12274)
* [rust-analyzer: Improve "Generate `Deref` impl" assist](https://github.com/rust-lang/rust-analyzer/pull/12276)
* [rust-analyzer: Show inlay hints after a `}` to indicate the closed item](https://github.com/rust-lang/rust-analyzer/pull/12244)
* [rust-analyzer: Support variable substitution in VSCode settings](https://github.com/rust-lang/rust-analyzer/pull/12215)
* [rust-analyzer: include associated types in trait signature help](https://github.com/rust-lang/rust-analyzer/pull/12208)
* [rust-analyzer: Add cast expressions to param name inlay hint heuristics](https://github.com/rust-lang/rust-analyzer/pull/12201)
* [rust-analyzer: Fix config patching failing when appending suffixes](https://github.com/rust-lang/rust-analyzer/pull/12209)
* [rust-analyzer: Fix fill-arguments completions not working](https://github.com/rust-lang/rust-analyzer/pull/12245)
* [rust-analyzer: Fix incorrect hover actions config keys](https://github.com/rust-lang/rust-analyzer/pull/12246)
* [rust-analyzer: Fix old config patching overwriting callable snippet config unconditionally](https://github.com/rust-lang/rust-analyzer/pull/12228)
* [rust-analyzer: don't panic at fully qualified call syntax in signature help](https://github.com/rust-lang/rust-analyzer/pull/12202)
* [rust-analyzer: insert whitespace between 'mut' and 'self' in macro expansion](https://github.com/rust-lang/rust-analyzer/pull/12262)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs are at point where user testing is needed before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-05-18 - 2022-06-15 🦀

### Virtual

* 2022-05-11 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydchbpb/)
* 2022-05-11 | Malaysia, MY | [Rust Malaysia Meetup](https://rust-malaysia.github.io/meetup/)
    * [**Rust Malaysia Meetup**](https://forms.gle/Xe61Zebj6tY53HR7A)
* 2022-05-12 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/tzjtssydchbqb/)
* 2022-05-12 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/San-Diego-Rust/)
    * [**San Diego Rust May 2022 Tele-Meetup**](https://www.meetup.com/San-Diego-Rust/events/285767149/)
* 2022-05-12 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/swgrssydchbqb/)
* 2022-05-17 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydchbwb/)
* 2022-05-18 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydchbxb/)
* 2022-05-18 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out night**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydchbxb/)
* 2022-05-24 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/284399980/)
* 2022-05-24 | San Francisco, CA, US | [Rust Bay Area](https://www.meetup.com/Rust-Bay-Area/)
    * [**(@ Google) What is soundness anyways?**](https://www.meetup.com/Rust-Bay-Area/events/285563981/)
* 2022-05-25 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/qwgrssydchbhc/)
* 2022-06-01 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcjbcb/)
* 2022-06-01 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydcjbcb/)
* 2022-06-07 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/Buffalo-Rust-Meetup/events/xgmfssydcjbkb/)
* 2022-06-08 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcjblb/)

### North America

* 2022-05-11 | Atlanta, GA, US | [Rust ATL](https://www.meetup.com/Rust-ATL/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/Rust-ATL/events/pczdssydchbpb/)
* 2022-05-12 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydchbqb/)
* 2022-05-17 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/285680468/)
* 2022-05-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydchbwb/)
* 2022-05-24 | San Francisco, CA, US | [Rust Bay Area](https://www.meetup.com/Rust-Bay-Area/)
    * [**(@ Google) What is soundness anyways?**](https://www.meetup.com/Rust-Bay-Area/events/285563981/)
* 2022-06-08 | Atlanta, GA, US | [Rust ATL](https://www.meetup.com/Rust-ATL/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/Rust-ATL/events/pczdssydcjblb/)

### Europe

* 2022-05-17 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**On Site Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/285820373/)
* 2022-05-18 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @AWS**](https://www.meetup.com/Stockholm-Rust/events/285701456/)
* 2022-05-19 & 05-20 | Berlin, DE | [Entwickler.de](https://entwickler.de/)
    * [**Rust Summit (paid)**](https://entwickler.de/rust-summit)
* 2022-05-24 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Developer Meetup: Lightning Talks @ Fiberplane**](https://www.meetup.com/rust-amsterdam-group/events/285291653/)

### Oceania

* 2022-05-26 | Brisbane City, QL | [Rust Brisbane](https://www.meetup.com/Rust-Brisbane/)
    * [**May Meetup**](https://www.meetup.com/Rust-Brisbane/events/285665676/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

<!--

New jobs can be posted here.

They should be of the form:

**Company Name**

* [Job Title (Location)](https://example.com/my-job-link)

-->

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> It is worth remembering that there is an infinitely large set of programs but a very small set of useful programs. A programming language is a form of ad-hoc compression algorithm, it is intended to sort the set of useful programs to have shorter encodings than the undesirable ones. Programs with certain categories of error or security vulnerability should be harder or impossible to express.

– [david_chisnall on lobste.rs](https://lobste.rs/s/vtcocq/c_is_truly_all_we_need_everything_else_is#c_yrcjm1)

Thanks to [Anton Fetisov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1241) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
