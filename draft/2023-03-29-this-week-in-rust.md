Title: This Week in Rust 488
Number: 488
Date: 2023-03-29
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

This week's crate is [typst](https://github.com/typst/typst), a modern LaTeX replacement.

Thanks to [H2CO3](https://users.rust-lang.org/t/crate-of-the-week/2704/1174) for the suggestion!

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

398 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-03-20..2023-03-27

* [`panic_immediate_abort` requires abort as a panic strategy](https://github.com/rust-lang/rust/pull/108924)
* [`rustc_interface`: Add a new query `pre_configure`](https://github.com/rust-lang/rust/pull/108221)
* [a general type system cleanup](https://github.com/rust-lang/rust/pull/109119)
* [add `CastKind::Transmute` to MIR](https://github.com/rust-lang/rust/pull/108442)
* [add `dist.compression-profile` option to control compression speed](https://github.com/rust-lang/rust/pull/109124)
* [add inlining annotations in `dec2flt`](https://github.com/rust-lang/rust/pull/108717)
* [add parentheses properly for method calls](https://github.com/rust-lang/rust/pull/109472)
* [avoid ICE of attempt to add with overflow in emitter](https://github.com/rust-lang/rust/pull/109403)
* [cleanup `codegen_fn_attrs`](https://github.com/rust-lang/rust/pull/109091)
* [constrain const vars to error if const types are mismatched](https://github.com/rust-lang/rust/pull/109336)
* [custom MIR: Allow optional RET type annotation](https://github.com/rust-lang/rust/pull/109392)
* [custom MIR: Support aggregate expressions](https://github.com/rust-lang/rust/pull/109390)
* [deeply check well-formedness of return-position `impl Trait` in trait](https://github.com/rust-lang/rust/pull/109545)
* [detect uninhabited types early in const eval](https://github.com/rust-lang/rust/pull/109435)
* [do not consider synthesized RPITITs on missing items checks](https://github.com/rust-lang/rust/pull/109414)
* [do not suggest bounds restrictions for synthesized RPITITs](https://github.com/rust-lang/rust/pull/109446)
* [don't elaborate non-obligations into obligations](https://github.com/rust-lang/rust/pull/109641)
* [don't pass `TreatProjections` separately to `fast_reject`](https://github.com/rust-lang/rust/pull/109202)
* [eagerly intern and check CrateNum/StableCrateId collisions](https://github.com/rust-lang/rust/pull/109213)
* [enforce non-lifetime-binders in supertrait preds are not object safe](https://github.com/rust-lang/rust/pull/108842)
* [fix bad suggestion for `clone/is_some` in field init shorthand](https://github.com/rust-lang/rust/pull/109355)
* [fix handling of trailing bare CR in `str::lines`](https://github.com/rust-lang/rust/pull/100311)
* [fix type suggestions in match arms](https://github.com/rust-lang/rust/pull/109613)
* [make alias-eq have a relation direction (and rename it to alias-relate)](https://github.com/rust-lang/rust/pull/109462)
* [make link clickable](https://github.com/rust-lang/rust/pull/109501)
* [make local query providers receive local keys](https://github.com/rust-lang/rust/pull/109092)
* [make param bound vars visibly bound vars with -Zverbose](https://github.com/rust-lang/rust/pull/109506)
* [mv tests/codegen/issue-* tests/codegen/issues/](https://github.com/rust-lang/rust/pull/109172)
* [new solver cleanup + implement coherence](https://github.com/rust-lang/rust/pull/109447)
* [new solver: make all goal evaluation able to be automatically rerun](https://github.com/rust-lang/rust/pull/108896)
* [not *all* retags might be explicit in Runtime MIR](https://github.com/rust-lang/rust/pull/109408)
* [only clear written-to locals in ConstProp](https://github.com/rust-lang/rust/pull/109087)
* [only implement `Fn*` traits for extern "Rust" safe function pointers and items](https://github.com/rust-lang/rust/pull/109441)
* [optimize `incremental_verify_ich`](https://github.com/rust-lang/rust/pull/109371)
* [permit the MIR inliner to inline diverging functions](https://github.com/rust-lang/rust/pull/106428)
* [rPITITs are `DefKind::Opaque` with new lowering strategy](https://github.com/rust-lang/rust/pull/109405)
* [simpler checked shifts in MIR building](https://github.com/rust-lang/rust/pull/109475)
* [split `execute_job` into `execute_job_incr` and `execute_job_non_incr`](https://github.com/rust-lang/rust/pull/109046)
* [upgrade to LLVM 16, again](https://github.com/rust-lang/rust/pull/109474)
* [use `SmallVec` in bitsets](https://github.com/rust-lang/rust/pull/109458)
* [use an `IndexVec` to debug fingerprints](https://github.com/rust-lang/rust/pull/109587)
* [use poison instead of undef](https://github.com/rust-lang/rust/pull/109220)
* [use region-erased self type during IAT selection](https://github.com/rust-lang/rust/pull/109423)
* [walk un-shifted nested `impl Trait` in trait when setting up default trait method assumptions](https://github.com/rust-lang/rust/pull/109240)
* [miri: fix raw pointer dyn receivers](https://github.com/rust-lang/rust/pull/109568)
* [miri: correctly quote env vars in single quoted string in bash](https://github.com/rust-lang/miri/pull/2822)
* [miri: have the miri cronjob link to the failed run](https://github.com/rust-lang/miri/pull/2820)
* [stabilize `arc_into_inner` and `rc_into_inner`](https://github.com/rust-lang/rust/pull/109504)
* [stabilize `nonnull_slice_from_raw_parts`](https://github.com/rust-lang/rust/pull/97506)
* [drop all messages in bounded channel when destroying the last receiver](https://github.com/rust-lang/rust/pull/108164)
* [implement Default for some alloc/core iterators](https://github.com/rust-lang/rust/pull/99929)
* [implement `read_buf` for a few more types](https://github.com/rust-lang/rust/pull/108326)
* [add `#[inline]` to `as_deref`](https://github.com/rust-lang/rust/pull/109357)
* [add `#[inline]` to the Into for From impl](https://github.com/rust-lang/rust/pull/109546)
* [shrink unicode case-mapping LUTs by 24k](https://github.com/rust-lang/rust/pull/109216)
* [windows: make `Command` prefer non-verbatim paths](https://github.com/rust-lang/rust/pull/96391)
* [hashbrown: use strict provenance APIs on nightly](https://github.com/rust-lang/hashbrown/pull/390)
* [stdarch: mark more `arm_shared` intrinsics and types as stable in docs](https://github.com/rust-lang/stdarch/pull/1398)
* [futures: don't ignore empty state polling](https://github.com/rust-lang/futures-rs/pull/2728)
* [futures: fu: always replace inner wakers](https://github.com/rust-lang/futures-rs/pull/2726)
* [futures: selectAll doesn't need pin-project](https://github.com/rust-lang/futures-rs/pull/2729)
* [futures: use `Waker::will_wake()` to avoid a cloning op](https://github.com/rust-lang/futures-rs/pull/2723)
* [cargo: add the old github keys as revoked](https://github.com/rust-lang/cargo/pull/11889)
* [cargo: added new GitHub RSA Host Key](https://github.com/rust-lang/cargo/pull/11883)
* [rustdoc: add support for type filters in arguments and generics](https://github.com/rust-lang/rust/pull/108629)
* [rustdoc: handle generics better when matching notable traits](https://github.com/rust-lang/rust/pull/108954)
* [rustdoc: optimize impl sorting during rendering](https://github.com/rust-lang/rust/pull/109399)
* [rustdoc: remove redundant `.content` prefix from span/a colors](https://github.com/rust-lang/rust/pull/109461)
* [clippy: lint clear with drain](https://github.com/rust-lang/rust-clippy/pull/10528)
* [clippy: do not propose to remove `async move` if variables are captured by ref](https://github.com/rust-lang/rust-clippy/pull/10490)
* [clippy: do not propose to simplify a not expression coming from a macro](https://github.com/rust-lang/rust-clippy/pull/10527)
* [clippy: fix `cast_possible_truncation` offering wrong suggestion for casting float to integer](https://github.com/rust-lang/rust-clippy/pull/10496)
* [clippy: move `unnecessary_struct_initialization` to nursery](https://github.com/rust-lang/rust-clippy/pull/10552)
* [clippy: new lint: detect unnecessary `struct` building](https://github.com/rust-lang/rust-clippy/pull/10489)
* [clippy: significantly optimize `significant_drop_tightening`](https://github.com/rust-lang/rust-clippy/pull/10533)
* [clippy: wrap `transmutes_expressible_as_ptr_casts` suggestions in parentheses](https://github.com/rust-lang/rust-clippy/pull/10454)
* [rust-analyzer: don't escape double hashes outside of Rust code blocks](https://github.com/rust-lang/rust-analyzer/pull/14422)
* [rust-analyzer: load proc-macros asynchronously](https://github.com/rust-lang/rust-analyzer/pull/14405)
* [rust-analyzer: fix VS Code status message formatting error](https://github.com/rust-lang/rust-analyzer/pull/14385)
* [rust-analyzer: fix proc-macro paths using incorrect CrateId's for `rust-project.json` workspaces](https://github.com/rust-lang/rust-analyzer/pull/14419)
* [rust-analyzer: fix renames of locals being broken in macro calls](https://github.com/rust-lang/rust-analyzer/pull/14407)
* [rust-analyzer: remove client side proc-macro version check](https://github.com/rust-lang/rust-analyzer/pull/14404)

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

Rusty Events between 2023-03-29 - 2023-04-26 🦀

### Virtual

* 2023-03-22 | Virtual (Richmond, VA, US) | [Rustaceans RVA](https://www.meetup.com/rustaceans-rva/)
    * [**Rustaceans RVA - March Meetup**](https://www.meetup.com/rustaceans-rva/events/291963911/)
* 2023-03-27 | Virtual | [Rust Formal Methods Interest Group](https://www.eventbrite.com/cc/rfmig-87969)
    * [**Flux: Ergonomic Verification of Rust Programs with Liquid Types**](https://www.eventbrite.com/e/flux-ergonomic-verification-of-rust-programs-with-liquid-types-tickets-577742873487?aff=ebdssbonlinesearch)
* 2023-03-28 | Virtual (Berlin, DE) | [Berline.rs - OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/292113239/)
* 2023-03-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfcfblc/)
* 2023-03-28 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Crack code interview problems in Rust: S2 Ep3**](https://www.meetup.com/microsoft-reactor-redmond/events/291677113/)
* 2023-03-29 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Writing your own rust 'book' with mdBook**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/291892487/)
* 2023-03-31 | Virtual (Tunis, TN) | [Rust Tunisia](https://www.meetup.com/rust-tunisia/)
    * [**Rust Meetup Tunisia - Volume I, Number III**](https://www.meetup.com/rust-tunisia/events/292402446/)
* 2023-04-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfcgbgb/)
* 2023-04-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/291967741/)
* 2023-04-05 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcgbhb/)
* 2023-04-11 | Virtual (Berlin, DE) | [Berline.rs - OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/292236794/)
* 2023-04-11 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsyfcgbpb/)
* 2023-04-11 | Virtual | [Rust Live](https://www.eventbrite.com/cc/rust-live-1876809)
    * [**Rust Live: Asynchronous Rust**](https://www.eventbrite.com/e/rust-live-asynchronous-rust-tickets-575865518267?aff=ebdssbonlinesearch&keep_tld=1)
* 2023-04-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—Introducing duplicate! and the peculiarities of proc macros**](https://www.meetup.com/rustdc/events/291830834/)
* 2023-04-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/-0)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/lqkkctyfcgbzb/)

### Europe

* 2023-03-28 | Zurich, CH | [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**High performance concurrent data structures in Rust - March Meetup**](https://www.meetup.com/de-DE/rust-zurich/events/291449557/)
* 2023-03-29 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #57**](https://www.meetup.com/rust-paris/events/291963747/)
* 2023-04-04 | Berlin, DE | [Berline.rs](https://berline.rs)
    * [**Rust and Tell - Goodbye👋 Edition**](https://berline.rs/2023/04/04/rust-and-tell-goodbye-edition.html)
* 2023-04-06 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #3**](https://www.meetup.com/fr-FR/rust-lyon/events/292283973/)
* 2023-04-19 | Zurich, CH | [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**sett: data encryption and transfer made easy(ier)**](https://www.meetup.com/de-DE/rust-zurich/events/292151879/)

### Asia
* 2023-04-08 | Kyoto, JP | [Kansai Rust](https://www.meetup.com/kansai-rust/)
    * [**Demystifying Closures**](https://www.meetup.com/kansai-rust/events/292202435/) 
* 2023-04-12 | Kuala Lumpur, MY | [Rust Malaysia](https://rust-malaysia.github.io/meetup/); [Telegram](https://t.me/golangmalaysia)
    * [**Rust Meetup Malaysia April 2023: How far is Dioxus from React? by Ivan Tham**](https://www.google.com/calendar/event?eid=MWI0bWJzY21qZTI2NWsyZDgzOG0xb2JkdTkgYXBkOXZtYmMyMmVnZW5tdHU1bDZjNWpiZmNAZw&ctz=America/Los_Angeles) | [Map](https://goo.gl/maps/w2ogftac6mqpBbvt5)

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

> As part of this work, I even found two memory safety bugs in the DRM scheduler component that were causing kernel oopses for Alyssa and other developers, so the Rust driver work also benefits other kernel drivers that use this shared code! Meanwhile, I still haven't gotten any reports of kernel oopses due to bugs in the Rust code at all.

– [Asahi Lina on the Asahi Linux blog](https://asahilinux.org/2023/03/road-to-vulkan/)

llogiq is patting himself on the back for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
