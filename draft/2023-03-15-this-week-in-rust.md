Title: This Week in Rust 486
Number: 486
Date: 2023-03-15
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [duplicate](https://docs.rs/duplicate), a proc macro crate for easy parametric code duplication.

Thanks to [Anton Fetisov](https://users.rust-lang.org/t/crate-of-the-week/2704/1164) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ -]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

391 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-03-06..2023-03-13

* [account for binders correctly when adding default RPITIT method assumption](https://github.com/rust-lang/rust/pull/108583)
* [force parentheses around `match` expression in binary expression](https://github.com/rust-lang/rust/pull/108542)
* [allow negative numeric literals in `concat!`](https://github.com/rust-lang/rust/pull/106844)
* [add suggestion to diagnostic when user has array but trait wants slice](https://github.com/rust-lang/rust/pull/108841)
* [always resolve to universal regions if possible](https://github.com/rust-lang/rust/pull/108121)
* [check for free regions in MIR validation](https://github.com/rust-lang/rust/pull/108786)
* [check that a query has not completed and is not executing before starting it](https://github.com/rust-lang/rust/pull/108845)
* [custom MIR: Support `as` casts](https://github.com/rust-lang/rust/pull/108855)
* [directly construct Inherited in typeck](https://github.com/rust-lang/rust/pull/108950)
* [do not ICE when failing to normalize in `ConstProp`](https://github.com/rust-lang/rust/pull/108803)
* [do not ICE when interpreting a cast between non-monomorphic types](https://github.com/rust-lang/rust/pull/108790)
* [do not ICE when we have fn pointer `Fn` obligations with bound vars in the self type](https://github.com/rust-lang/rust/pull/108834)
* [do not consider `&mut *x` as mutating `x` in `CopyProp`](https://github.com/rust-lang/rust/pull/108178)
* [don't even try to combine consts with incompatible types](https://github.com/rust-lang/rust/pull/108947)
* [emit alias-eq when equating numeric var and projection](https://github.com/rust-lang/rust/pull/108828)
* [emit the `suspicious_auto_trait_impls` lint for negative impls as well](https://github.com/rust-lang/rust/pull/108807)
* [ensure value is on the on-disk cache before returning from `ensure()`](https://github.com/rust-lang/rust/pull/108820)
* [expand on the allocator comment in `rustc-main`](https://github.com/rust-lang/rust/pull/109018)
* [improve errors in case of ident with number at start](https://github.com/rust-lang/rust/pull/108854)
* [impl better help for `.poll()` not found on `impl Future`](https://github.com/rust-lang/rust/pull/108731)
* [implement better error for manual impl of `Fn*` traits](https://github.com/rust-lang/rust/pull/108930)
* [avoid unnecessary hashing](https://github.com/rust-lang/rust/pull/108794)
* [fix invalid inlining of reexport of reexport of private item](https://github.com/rust-lang/rust/pull/108870)
* [gate usages of `dyn*` and const closures in macros](https://github.com/rust-lang/rust/pull/109029)
* [greatly improve the error messages when `run-make/translation` fails](https://github.com/rust-lang/rust/pull/108774)
* [honor current target when checking conditional compilation values](https://github.com/rust-lang/rust/pull/108949)
* [implement goal caching with the new solver](https://github.com/rust-lang/rust/pull/108071)
* [make RPITITs simple cases work when using `lower_impl_trait_in_trait_to_assoc_ty`](https://github.com/rust-lang/rust/pull/108700)
* [make `unused_allocation` lint against `Box::new` too](https://github.com/rust-lang/rust/pull/104363)
* [place binder correctly for arbitrary trait bound suggestion](https://github.com/rust-lang/rust/pull/108294)
* [relax ordering rules for `asm!` operands](https://github.com/rust-lang/rust/pull/105798)
* [strengthen state tracking in const-prop](https://github.com/rust-lang/rust/pull/108872)
* [tweak illegal `Copy` impl message](https://github.com/rust-lang/rust/pull/108884)
* [unconstrained terms should account for infer vars being equated](https://github.com/rust-lang/rust/pull/108879)
* [fix projection substitution order considering GATs](https://github.com/rust-lang/chalk/pull/790)
* [remove `box_syntax`](https://github.com/rust-lang/rust/pull/108471)
* [stabilize `nonzero_min_max`](https://github.com/rust-lang/rust/pull/106633)
* [stabilize `path_as_mut_os_str`](https://github.com/rust-lang/rust/pull/105962)
* [stabilize movbe target feature](https://github.com/rust-lang/rust/pull/107711)
* [add `round_ties_even` to `f32` and `f64`](https://github.com/rust-lang/rust/pull/95317)
* [make `ptr::from_ref` and `ptr::from_mut` const](https://github.com/rust-lang/rust/pull/108956)
* [move `Option::as_slice` to an always-sound implementation](https://github.com/rust-lang/rust/pull/108623)
* [prevent overflow through `Arc::downgrade`](https://github.com/rust-lang/rust/pull/108708)
* [introduce `Rc::into_inner`, as a parallel to `Arc::into_inner`](https://github.com/rust-lang/rust/pull/109026)
* [futures: Add `AbortRegistration::handle`](https://github.com/rust-lang/futures-rs/pull/2712)
* [futures: Make BiLock strict-provenance compatible](https://github.com/rust-lang/futures-rs/pull/2716)
* [futures: `TryFlattenUnordered`: propagate base stream error](https://github.com/rust-lang/futures-rs/pull/2607)
* [cargo: Adding display of which target failed to compile](https://github.com/rust-lang/cargo/pull/11636)
* [cargo: `cargo install --git` multiple packages with binaries found hint](https://github.com/rust-lang/cargo/pull/11835)
* [rustdoc: don't crash on `crate` references in blocks](https://github.com/rust-lang/rust/pull/108988)
* [rustdoc: don't hide anonymous reexport](https://github.com/rust-lang/rust/pull/108936)
* [rustdoc: include link on all.html location header](https://github.com/rust-lang/rust/pull/108686)
* [rustdoc: reduce allocs in `FnDecl::inner_full_print`](https://github.com/rust-lang/rust/pull/109011)
* [rustdoc: sort deprecated items lower in search](https://github.com/rust-lang/rust/pull/107629)
* [rustdoc: use restricted Damerau-Levenshtein distance for search](https://github.com/rust-lang/rust/pull/109009)
* [clippy: add `collection_is_never_read`](https://github.com/rust-lang/rust-clippy/pull/10415)
* [clippy: add `let_with_type_underscore` lint](https://github.com/rust-lang/rust-clippy/pull/10467)
* [clippy: add `missing_assert_message` lint](https://github.com/rust-lang/rust-clippy/pull/10362)
* [clippy: add new `redundant_async_block` lint](https://github.com/rust-lang/rust-clippy/pull/10448)
* [clippy: add the `popular-crates` binary](https://github.com/rust-lang/rust-clippy/pull/10466)
* [clippy: don't lint `manual_clamp` in const contexts](https://github.com/rust-lang/rust-clippy/pull/10479)
* [clippy: fix semicolon insertion in `match_single_binding`](https://github.com/rust-lang/rust-clippy/pull/10470)
* [clippy: improve diagnostic of `no_mangle_with_rust_abi`](https://github.com/rust-lang/rust-clippy/pull/10420)
* [clippy: include async functions in the `len_without_is_empty`](https://github.com/rust-lang/rust-clippy/pull/10359)
* [clippy: `arithmetic_side_effects` fix false positive on shifts](https://github.com/rust-lang/rust-clippy/pull/10309)
* [rust-analyzer: add core lib to `proc_macro` dependencies](https://github.com/rust-lang/rust-analyzer/pull/14297)
* [rust-analyzer: fix stack overflow when derefrencing `&!`](https://github.com/rust-lang/rust-analyzer/pull/14316)
* [rust-analyzer: allow adding extra `cargo` args when running build scripts](https://github.com/rust-lang/rust-analyzer/pull/14328)
* [rust-analyzer: fix multiple definition binding in match to let-else](https://github.com/rust-lang/rust-analyzer/pull/14291)
* [rust-analyzer: don't trigger unresolved method/field diagnostics on types containing errors](https://github.com/rust-lang/rust-analyzer/pull/14271)
* [rust-analyzer: evaluate consts in `path_to_const`](https://github.com/rust-lang/rust-analyzer/pull/14285)
* [rust-analyzer: fix block defmap not looking into tail expressions for macro calls](https://github.com/rust-lang/rust-analyzer/pull/14286)
* [rust-analyzer: fix search not searching bodies of attributed items](https://github.com/rust-lang/rust-analyzer/pull/14299)
* [rust-analyzer: highlight unresolved derives as being unresolved](https://github.com/rust-lang/rust-analyzer/pull/14284)
* [rust-analyzer: load proc-macros for `rustc_private` crates](https://github.com/rust-lang/rust-analyzer/pull/14282)
* [rust-analyzer: watch both stdout and stderr in flycheck](https://github.com/rust-lang/rust-analyzer/pull/14300)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

<!-- Approved RFCs go here, use this format: * [Topic](URL) -->
<!-- or if none were approved this week, use: * *No RFCs were approved this week.* -->
<!-- * []() -->

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

Rusty Events between 2023-03-15 - 2023-04-12 🦀

### Virtual

* 2023-03-08 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsryfcfblb/)
* 2023-03-09 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsyfcfbmb/)
* 2023-03-11 | Virtual | [Rust GameDev](https://discord.gg/yNtPTb2)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2)
* 2023-03-14 | Virtual (Italy) | [Hinto](https://www.eventbrite.it/o/hinto-28025248045)
    * [**Webinar online | Introduzione a Rust**](https://www.eventbrite.com/e/biglietti-webinar-online-introduzione-a-rust-558594419947?aff=ebdssbonlinesearch&keep_tld=1)
* 2023-03-14 | Virtual (Berlin, DE) | [Berlin.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2023/03/14/rust-hack-and-learn.html)
* 2023-03-14 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/291809763/)
* 2023-03-14 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Crack code interview problems in Rust: S2 Ep1**](https://www.meetup.com/microsoft-reactor-redmond/events/291676352/)
* 2023-03-14 | Virtual (Saarbrücken, DE) | [Rust-Saar](https://www.meetup.com/rust-saar/)
    * [**Meetup: 27u16**](https://www.meetup.com/rust-saar/events/292076386/)
* 2023-03-15 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Injecting Rust Hooks into a 1999 game binary (unsafe)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/291354288/)
* 2023-03-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/wqchctyfcfbtb/)
* 2023-03-16 | Virtual (Raleigh, NC, US) | [Triangle BitDevs](https://www.meetup.com/triangle-bitdevs/)
    * [**Rust for Bitcoiners**](https://www.meetup.com/triangle-bitdevs/events/292032273/)
* 2023-03-16 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Introduction to WebAssembly (WASM) with Rust and WASMEdge**](https://www.meetup.com/microsoft-reactor-redmond/events/291681809/)
* 2023-03-16 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/291847774/)
* 2023-03-21 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Crack code interview problems in Rust: S2 Ep2**](https://www.meetup.com/microsoft-reactor-redmond/events/291676961/)
* 2023-03-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Rust+Tell Lightning Talks**](https://www.meetup.com/rustdc/events/vdhxgsyfcfbcc/)
* 2023-03-22 | Virtual (Richmond, VA, US) | [Rustaceans RVA](https://www.meetup.com/rustaceans-rva/)
    * [**Rustaceans RVA - March Meetup**](https://www.meetup.com/rustaceans-rva/events/291963911/)
* 2023-03-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfcfblc/)
* 2023-03-28 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Crack code interview problems in Rust: S2 Ep3**](https://www.meetup.com/microsoft-reactor-redmond/events/291677113/)
* 2023-03-29 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Writing your own rust 'book' with mdBook**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/291892487/)
* 2023-04-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfcgbgb/)
* 2023-04-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/291967741/)
* 2023-04-05 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcgbhb/)


### Europe
 
* 2023-03-09 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**Rust Meetup #7**](https://www.meetup.com/rust-basel/events/291228934/)
* 2023-03-09 | Delft, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Regular track: Embedded Rust**](https://www.meetup.com/rust-nederland/events/291401965/)
    * [**Student track: Embedded Rust**](https://www.meetup.com/rust-nederland/events/291401778/)
* 2023-03-09 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #2**](https://www.meetup.com/fr-FR/rust-lyon/events/291727241/)
* 2023-03-15 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Walk around Embedded World Exhibition**](https://www.meetup.com/rust-noris/events/291623203/)
* 2023-03-15 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/)
    * [**Rust graphics with nannou**](https://www.meetup.com/rust-trondheim/events/292085409/) 
* 2023-03-16 | Paris, FR | [OCaml Users in Paris - OUPS](https://www.meetup.com/ocaml-paris/events/)
    * [**OUPS Mars 2023: Creusot a prophetic verifier for Rust -- Xavier Denis**](https://www.meetup.com/ocaml-paris/events/291637370/)
* 2023-03-17 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/zmppzsyfcfbwb/)
* 2023-03-28 | Zurich, CH | [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/events/291449557/)
    * [**High performance concurrent data structures in Rust - March Meetup**](https://www.meetup.com/de-DE/rust-zurich/events/291449557/)
* 2023-03-29 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #57**](https://www.meetup.com/rust-paris/events/291963747/)

### North America

* 2023-03-09 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Trails, Triumphs, & Travails of Yet-Another-Database-Crate with PJ and Food!**](https://www.meetup.com/utah-rust/events/rrwbctyfcfbmb/)

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

> The Rust compiler is a thousand unit tests that you don't have to write

– [Someone, likely Ian Purton on the Cloak blog](https://cloak.software/blog/i-built-startup-in-rust/)

Thanks to [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1380) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
