Title: This Week in Rust 521
Number: 521
Date: 2023-11-15
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

This week's crate is [cargo-msrv](https://github.com/foresterre/cargo-msrv), a cargo subcommand to find out the Minimum Supported Rust Version (MSRV) of your crate.

llogiq is a bit worried about not having received suggestions for two weeks in a row, but still offers you his choice.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

364 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-11-06..2023-11-13

* [speed up `x clean`](https://github.com/rust-lang/rust/pull/117723)
* [llvm-wrapper: remove include of non-existant Vectorize.h](https://github.com/rust-lang/rust/pull/117675)
* [`rustc_llvm`: Link to libkstat on Solaris/SPARC](https://github.com/rust-lang/rust/pull/114224)
* [add -Zcross-crate-inline-threshold=yes](https://github.com/rust-lang/rust/pull/117650)
* [add `std::hash::{DefaultHasher, RandomState}` exports](https://github.com/rust-lang/rust/pull/115694)
* [add a new `download-ci-llvm = if-unchanged` option and enable it by default for `profile = codegen`](https://github.com/rust-lang/rust/pull/116881)
* [allow configuring the parent GitHub repository](https://github.com/rust-lang/rust/pull/117122)
* [build a better MIR body when errors are encountered](https://github.com/rust-lang/rust/pull/117418)
* [compute layout with spans for better cycle errors in coroutines](https://github.com/rust-lang/rust/pull/117858)
* [compute polonius loan scopes over the region graph](https://github.com/rust-lang/rust/pull/117560)
* [coverage: avoid creating malformed macro name spans](https://github.com/rust-lang/rust/pull/117827)
* [coverage: rename the `run-coverage` test mode to `coverage-run`](https://github.com/rust-lang/rust/pull/117700)
* [deny more `~const` trait bounds](https://github.com/rust-lang/rust/pull/117817)
* [extend builtin/auto trait args with error when they have \>1 argument](https://github.com/rust-lang/rust/pull/117645)
* [format macro const literals with pretty printer](https://github.com/rust-lang/rust/pull/115485)
* [generator layout: ignore fake borrows](https://github.com/rust-lang/rust/pull/117712)
* [give a better diagnostic for missing parens in Fn* bounds](https://github.com/rust-lang/rust/pull/117297)
* [handle the case when the change-id isn't found](https://github.com/rust-lang/rust/pull/117263)
* [improve diagnostic for const ctors in array repeat expressions](https://github.com/rust-lang/rust/pull/113925)
* [make `FatalErrorMarker` lower priority than other panics](https://github.com/rust-lang/rust/pull/117557)
* [on method chain expression failure, look for missing method in earlier segments of the chain](https://github.com/rust-lang/rust/pull/115229)
* [only instantiate binder during dyn's built-in trait candidate probe once](https://github.com/rust-lang/rust/pull/117610)
* [only use `normalize_param_env` when normalizing predicate in `check_item_bounds`](https://github.com/rust-lang/rust/pull/117542)
* [patterns: reject raw pointers that are not just integers](https://github.com/rust-lang/rust/pull/116930)
* [recover from incorrectly ordered/duplicated function keywords](https://github.com/rust-lang/rust/pull/117282)
* [reorder checks to make sure potential missing expect on Option/Result…](https://github.com/rust-lang/rust/pull/117695)
* [restore rustc shim error message](https://github.com/rust-lang/rust/pull/117724)
* [catch stray `{` in let-chains](https://github.com/rust-lang/rust/pull/117770)
* [suggest removing `;` for `;` within let-chains](https://github.com/rust-lang/rust/pull/117743)
* [thir unsafeck fixes](https://github.com/rust-lang/rust/pull/117229)
* [warn when using an unstable feature with -Ctarget-feature](https://github.com/rust-lang/rust/pull/117616)
* [when not finding assoc fn on type, look for builder fn](https://github.com/rust-lang/rust/pull/117006)
* [miri: `data_race`: link to docs for 'unusual' race conditions](https://github.com/rust-lang/miri/pull/3155)
* [miri: freebsd adding getentropy interception support](https://github.com/rust-lang/miri/pull/3161)
* [miri: implement round.ps and round.pd SSE4.1 intrinsics](https://github.com/rust-lang/miri/pull/3159)
* [miri: share getentropy shim across various unixes](https://github.com/rust-lang/miri/pull/3162)
* [miri: treat thread-local statics on main thread as static roots for leakage analysis](https://github.com/rust-lang/miri/pull/2931)
* [emit `#[inline]` on `derive(Debug)`](https://github.com/rust-lang/rust/pull/117727)
* [stabilize `result_option_inspect`](https://github.com/rust-lang/rust/pull/116866)
* [move `BorrowedBuf` and `BorrowedCursor` from `std:io` to `core::io`](https://github.com/rust-lang/rust/pull/117694)
* [closure-consuming helper functions for `fmt::Debug` helpers](https://github.com/rust-lang/rust/pull/117730)
* [don't panic in `<BorrowedCursor as io::Write>::write`](https://github.com/rust-lang/rust/pull/115460)
* [futures: provide a non-destructive mechanism to determine if a sink/stream are paired](https://github.com/rust-lang/futures-rs/pull/2797)
* [codegen-cranelift: implement AArch64 intrinsics necessary for simd-json](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1416)
* [codegen-cranelift: implement AES-NI and SHA256 crypto intrinsics using inline asm](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1425)
* [codegen-cranelift: implement a lot of SIMD intrinsics](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1417)
* [codegen\_gcc: do not emit `.eh_frame` section if using -Cpanic=abort](https://github.com/rust-lang/rustc_codegen_gcc/pull/374)
* [cargo: `query`{`_vec`} use `IndexSummary`](https://github.com/rust-lang/cargo/pull/12970)
* [cargo: add better error message when it can not find the search section](https://github.com/rust-lang/cargo/pull/12865)
* [cargo: add cache garbage collection](https://github.com/rust-lang/cargo/pull/12634)
* [cargo credential: include license files in all published crates](https://github.com/rust-lang/cargo/pull/12953)
* [cargo: do not allow empty feature name](https://github.com/rust-lang/cargo/pull/12928)
* [cargo: make browser links out of HTML file paths](https://github.com/rust-lang/cargo/pull/12889)
* [cargo: filter `cargo-credential-*` dependencies by OS](https://github.com/rust-lang/cargo/pull/12949)
* [cargo: fix `--quiet` being used with nested subcommands](https://github.com/rust-lang/cargo/pull/12959)
* [cargo: fix non-deterministic behavior in last-use repopulation](https://github.com/rust-lang/cargo/pull/12958)
* [cargo: do not panic when failed to parse rustc commit-hash](https://github.com/rust-lang/cargo/pull/12965)
* [cargo: preserve jobserver file descriptors on rustc invocation in `fix_exec_rustc`](https://github.com/rust-lang/cargo/pull/12951)
* [cargo: report more detailed semver errors](https://github.com/rust-lang/cargo/pull/12924)
* [rustdoc: properly elide cross-crate host effect args](https://github.com/rust-lang/rust/pull/117531)
* [clippy: `arc_with_non_send_sync` Improve suggested resolution](https://github.com/rust-lang/rust-clippy/pull/11772)
* [clippy: `map_identity`: respect match ergonomics](https://github.com/rust-lang/rust-clippy/pull/11792)
* [clippy: `mod_module_files` Don't emit lint for mod.rs in tests](https://github.com/rust-lang/rust-clippy/pull/11779)
* [clippy: add type details to `unnecessary_fallible_conversions` note](https://github.com/rust-lang/rust-clippy/pull/11767)
* [clippy: destructure `Conf` in `register_lints`](https://github.com/rust-lang/rust-clippy/pull/11790)
* [clippy: disable `vec_box` when using different allocators](https://github.com/rust-lang/rust-clippy/pull/11780)
* [clippy: don't check for late-bound vars, check for escaping bound vars](https://github.com/rust-lang/rust-clippy/pull/11760)
* [clippy: fixes to `manual_let_else`'s divergence check](https://github.com/rust-lang/rust-clippy/pull/11787)
* [clippy: lint `needless_borrow` and `explicit_auto_deref` on most union field accesses](https://github.com/rust-lang/rust-clippy/pull/11508)
* [clippy: move `suspicious_doc_comments` to doc pass](https://github.com/rust-lang/rust-clippy/pull/11798)
* [clippy: replace `if_chain` with let chains](https://github.com/rust-lang/rust-clippy/pull/11750)
* [rust-analyzer: add config for preferring / ignoring prelude modules when inserting imports](https://github.com/rust-lang/rust-analyzer/pull/15871)
* [rust-analyzer: preview adt field when hover](https://github.com/rust-lang/rust-analyzer/pull/15847)
* [rust-analyzer: find `Self` reference](https://github.com/rust-lang/rust-analyzer/pull/15864)
* [rust-analyzer: ignore `doc(hidden)` attr if no body is present](https://github.com/rust-lang/rust-analyzer/pull/15854)
* [rust-analyzer: truncate closure capture place for raw pointer](https://github.com/rust-lang/rust-analyzer/pull/15860)
* [rust-analyzer: improve check for include macro](https://github.com/rust-lang/rust-analyzer/pull/15866)

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

Rusty Events between 2023-11-15 - 2023-12-13 🦀

### Virtual

* 2023-11-08 | Virtual(Boulder, CO, US) | [Solid State Depot - The Boulder Makerspace](https://www.meetup.com/solidstatedepot/)
    * [**Placeholder: Boulder Rust Meetup**](https://www.meetup.com/solidstatedepot/events/296661062/)
* 2023-11-09 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 34rd Edition**](https://www.meetup.com/rust-linz/events/297133538/)
* 2023-11-09 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732666/)
* 2023-11-10 | Virtual (Bangalore, IN) | [Learn Everything About Programming](https://www.meetup.com/just-code/)
    * [**Getting started with rust-lang**](https://www.meetup.com/just-code/events/297172855/)
* 2023-11-12 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Rust in Israel - Rust Digger**](https://www.meetup.com/code-mavens/events/297064458/)
* 2023-11-14 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/fvdtgtyfcpbsb/)
* 2023-11-14 | Virtual (Kyiv, UA) | [Yalantis Education](https://www.meetup.com/yeducation/)
    * [**Довгий шлях до першого комерційного досвіду або до чого тут Rust?**](https://www.meetup.com/yeducation/events/297219539/)
* 2023-11-15 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Building Our Own Locks (Atomics & Locks Chapter 9)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296582223/)
* 2023-11-15 | Virtual (Richmond, VA, US) | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2023 (Nov 13-16)**](https://lpc.events/event/17/sessions/170/)
* 2023-11-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Nightly Night: impl Trait in Type Aliases**](https://www.meetup.com/vancouver-rust/events/296600976/)
* 2023-11-16 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296833657/)
* 2023-11-16 | Virtual (Vilnius, LT) | [Vilnius Rust and Go Meetup Group](https://www.meetup.com/vilnius-rust-go-meetup-group/)
    * [**Enjoy our first Rust event**](https://www.meetup.com/vilnius-rust-go-meetup-group/events/297133832/)
* 2023-11-21 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679794/)
* 2023-11-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/296807537/)
* 2023-11-23 | Virtual (Edmonton, AB, CA) | [Edmonton R User Group - Yegrug](https://www.meetup.com/edmonton-r-user-group-yegrug/)
    * [**Edmonton R User Group Meetup: R and Rust, like a match made in heaven**](https://www.meetup.com/edmonton-r-user-group-yegrug/events/296605221/)
* 2023-11-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtyfcpblc/)
* 2023-11-29 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Atomics & Locks Book Club Final Chapter! (Chapter 10)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583091/)
* 2023-11-30 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296833665/)
* 2023-11-30 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/rust-dublin/)
    * [**Automating expertise with cargo-semver-checks**](https://www.meetup.com/rust-dublin/events/296346693/)
* 2023-12-01 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Kick-Off!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583626/)
* 2023-12-02 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdssbdestsearch)
* 2023-12-05 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679797/) | [**Mirror**](https://berline.rs/)
* 2023-12-05 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/297021574/)


### Europe

* 2023-11-09 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/)
    * [**11th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/296567395)
* 2023-11-09 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-732823744547/)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-732823744547)
* 2023-11-09 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/296083417/)
* 2023-11-21 | Augsburg, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**GPU processing in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504264/)
* 2023-11-23 | Biel/Bienne, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Talks Bern @ Biel: Embedded Edition**](https://www.meetup.com/rust-bern/events/296556498/)

### North America

* 2023-11-08 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**Let's make a Discord bot!**](https://www.meetup.com/boulder-rust-meetup/events/296437292/)
* 2023-11-14 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Mixer: Share, Show, & Tell! 🦀**](https://www.meetup.com/rust-nyc/events/296895126/)
* 2023-11-14 | Seattle, WA, US | [Cap Hill Rust Coding/Hacking/Learning](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/seattle-rust-user-group/events/296540653)
* 2023-11-15 | Richmond, VA, US + Virtual | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2023 (Nov 13-16)**](https://lpc.events/event/17/sessions/170/)
* 2023-11-16 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/297062689/)
* 2023-11-16 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Python loves Rust!**](https://www.meetup.com/music-city-rust-developers/events/296916567/)
* 2023-11-16 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/295483924)
* 2023-11-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/296917625/)
* 2023-11-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcpbdc/)
* 2023-11-28 | Pasadena, CA, US | [Pasadena Thursday Go / Rust](https://www.meetup.com/thursday-go/)
    * [**Monthly Rust group**](https://www.meetup.com/thursday-go/events/297062186/)

### Oceania

* 2023-11-21 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/296819540/)
* 2023-11-28 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**November Meetup**](https://www.meetup.com/rust-canberra/events/296391733/)

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

> I decided to keep learning Rust because I liked the syntax. I liked the speed. I liked the community. I liked it all. It felt like a breath of fresh air: a syntax more intuitive than Python, JavaScript, or C, yet still faster.

– [Goren Barak on their blog](https://digital-goobers.vercel.app/posts/learning-rust)

Thanks to [Goren Barak](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1488) for the self-suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
