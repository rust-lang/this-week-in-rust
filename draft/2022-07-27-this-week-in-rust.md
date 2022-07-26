Title: This Week in Rust 453
Number: 453
Date: 2022-07-27
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

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [cargo-semver-checks](https://crates.io/crates/cargo-semver-checks), a CI-friendly tool to ckeck your library's API.

Thanks to [Predrag Gruevski](https://users.rust-lang.org/t/crate-of-the-week/2704/1087) and [Matthias Beyer](https://users.rust-lang.org/t/crate-of-the-week/2704/1088) for the (self and other) nominations.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

397 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-07-18..2022-07-25

* [rustc\_expand: switch `FxHashMap` to `FxIndexMap` where iteration is used](https://github.com/rust-lang/rust/pull/99320)
* [proc\_macro/bridge: stop using a remote object handle for proc\_macro `Ident` and `Literal`](https://github.com/rust-lang/rust/pull/98189)
* [allow to disable thinLTO buffer to support lto-embed-bitcode lld feature](https://github.com/rust-lang/rust/pull/98162)
* [allow `trait A: ~const B`](https://github.com/rust-lang/rust/pull/93429)
* [do not allow typeck children items to constrain outer RPITs](https://github.com/rust-lang/rust/pull/99345)
* [do not resolve associated const when there is no provided value](https://github.com/rust-lang/rust/pull/99449)
* [don't suggest unnameable generic arguments](https://github.com/rust-lang/rust/pull/99580)
* [improve suggestions for `NonZeroT` <- `T` coercion error](https://github.com/rust-lang/rust/pull/99438)
* [improve suggestions for returning binding](https://github.com/rust-lang/rust/pull/99539)
* [add E0790 as more specific variant of E0283](https://github.com/rust-lang/rust/pull/98028)
* [miri: add a scheme for always using the default toolchain, running clippy and fmt before running any other command](https://github.com/rust-lang/miri/pull/2381)
* [miri: add a flag to print a diagnostic when an outdated value is returned from an atomic load](https://github.com/rust-lang/miri/pull/2424)
* [support vec zero-alloc optimization for tuples and byte arrays](https://github.com/rust-lang/rust/pull/97581)
* [add `PhantomData` marker for dropck to `BTreeMap`](https://github.com/rust-lang/rust/pull/99413)
* [add `\[f32\]::sort_floats` and `\[f64\]::sort_floats`](https://github.com/rust-lang/rust/pull/93397)
* [protect `std::io::Take::limit` from overflow in `read`](https://github.com/rust-lang/rust/pull/95040)
* [fix `Skip::next` for non-fused inner iterators](https://github.com/rust-lang/rust/pull/99434)
* [stabilize `core::task::ready!`](https://github.com/rust-lang/rust/pull/99419)
* [add `IntoFuture::into_future` desugaring](https://github.com/rust-lang/reference/pull/1233)
* [futures: inline `WakerRef` functions](https://github.com/rust-lang/futures-rs/pull/2626)
* [cargo-miri: support nextest](https://github.com/rust-lang/miri/pull/2398)
* [cargo-miri: reorder `--target` to after the user-defined commands](https://github.com/rust-lang/miri/pull/2402)
* [cargo: stabilize workspace Inheritance](https://github.com/rust-lang/cargo/pull/10859)
* [docs.rs: add canonical URL for rustdoc pages](https://github.com/rust-lang/docs.rs/pull/1773)
* [clippy: add `arithmetic` lint](https://github.com/rust-lang/rust-clippy/pull/9130)
* [clippy: add `assertions_on_result_states` lint](https://github.com/rust-lang/rust-clippy/pull/9225)
* [clippy: check `assign_op_pattern` for conflicting borrows](https://github.com/rust-lang/rust-clippy/pull/9214)
* [clippy: check for `todo!` on every expression in `SpanlessEq`](https://github.com/rust-lang/rust-clippy/pull/9207)
* [clippy: check macro statements in `non_copy_const`](https://github.com/rust-lang/rust-clippy/pull/9246)
* [clippy: don't lint `std_instead_of_core` on `std::env`](https://github.com/rust-lang/rust-clippy/pull/9243)
* [clippy: fix ICE in `miri_to_const`](https://github.com/rust-lang/rust-clippy/pull/9241)
* [clippy: fix ICE in `question_mark`](https://github.com/rust-lang/rust-clippy/pull/9244)
* [clippy: fix `useless_format` spans for `format!("{foo}")`](https://github.com/rust-lang/rust-clippy/pull/9237)
* [clippy: fix suggestion causing error for `needless_borrow` function in field](https://github.com/rust-lang/rust-clippy/pull/9210)
* [rust-analyzer: find original as node before compute ref match](https://github.com/rust-lang/rust-analyzer/pull/12800)
* [rust-analyzer: fix missing fields check on destructuring assignment](https://github.com/rust-lang/rust-analyzer/pull/12863)
* [rust-analyzer: fix: autocomplete for struct fields includes receiver](https://github.com/rust-lang/rust-analyzer/pull/12861)
* [rust-analyzer: fix: don't add braces to 'if' completion in match guard position](https://github.com/rust-lang/rust-analyzer/pull/12851)
* [rust-analyzer: fix error tooltip message for VSCode status bar item](https://github.com/rust-lang/rust-analyzer/pull/12850)
* [rust-analyzer: fix restart server duplicating language clients](https://github.com/rust-lang/rust-analyzer/pull/12847)
* [rust-analyzer: fix: work around Code bug with empty diagnostics](https://github.com/rust-lang/rust-analyzer/pull/12809)
* [rust-analyzer: don't replace default members' body](https://github.com/rust-lang/rust-analyzer/pull/12832)
* [rustc-perf: enable measuring stable compiler builds](https://github.com/rust-lang/rustc-perf/pull/1341)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-07-27 - 2022-08-24 🦀

### Virtual

* 2022-07-20 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydckbrb/)
* 2022-07-20 | Cardiff, UK | [Rust and C++ Cardiff ](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust Book Discussion - Building a Multithreaded Web Server**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/287121101/)
* 2022-07-21 | Buenos Aires, AR | [Rust Argentina](https://www.meetup.com/es-ES/rust-argentina/)
    * [**WASM beyond the browser with Rust**](https://salesforce.zoom.us/webinar/register/WN_gaI8WBDSQFGnMgaqYAxPHA)
* 2022-07-21 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydckbcc/)
* 2022-07-26 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/Dallas-Rust/events/jqxqwrydckbjc/)
* 2022-07-27 | Cardiff, UK | [Rust and C++ Cardiff ](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Using Rust to read the Little Schemer**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/287121637/)
* 2022-07-28 | Linz, AT | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Rust Meetup Linz - 24th Edition**](https://www.meetup.com/Rust-Linz/events/287238072/)
* 2022-07-29 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/Minneapolis-Rust-Meetup/)
    * [**Beginner Rust Open "Office Hours"**](https://www.meetup.com/Minneapolis-Rust-Meetup/events/286993342/)
* 2022-07-31 | Seattle, WA, US | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Intro to Monads for Rustaceans**](https://www.meetup.com/Seattle-Rust-Meetup/events/286692243/)
* 2022-08-02 | Berlin, DE | [Berline.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2022/08/02/rust-hack-and-learn.html)
* 2022-08-02 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydclbdb/)
* 2022-08-03 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydclbfb/)
* 2022-08-03 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydclbfb/)
* 2022-08-05 | Portland, OR, US | [RustConf](https://rustconf.com/)
    * [**RustConf 2022**](https://rustconf.com/)
* 2022-08-10 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydclbnb/)
* 2022-08-10 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydclbnb/)
* 2022-08-11 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydclbpb/)
* 2022-08-13 | Virtual | [Rust Gamedev](https://gamedev.rs/)
    * [**Rust Gamedev Monthly Meetup**](https://www.google.com/url?q=https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2Eop9Blil9YUWeTq472NIi)
* 2022-08-16 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydclbvb/)
* 2022-08-17 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydclbwb/)
* 2022-08-18 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Hierarchical Task Network compiler written in Rust**](https://www.meetup.com/charlottesville-rust-meetup/events/287203159/)

### Europe

* 2022-07-20 | Warsaw, PL | [Rust Warsaw](https://www.meetup.com/rust-warsaw/)
    * [**Rust Warsaw #5**](https://www.meetup.com/rust-warsaw/events/287093615/)
* 2022-07-21 | Oslo, NO | [Rust Oslo](https://www.meetup.com/Rust-Oslo/)
    * [**Rust & Relax @ Kulturhuset**](https://www.meetup.com/Rust-Oslo/events/287184674/)
* 2022-07-21 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #27**](https://www.meetup.com/rust-wroclaw/events/287023750/)
* 2022-07-27 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Developers Amsterdam Group**](https://www.meetup.com/rust-amsterdam-group/events/287019877/)
* 2022-07-28 | Copenhagen, DK | [Copenhagen Rust Group](https://cph.rs/)
    * [**Hack Night #27**](https://www.meetup.com/copenhagen-rust-meetup-group/events/287057762/)
* 2022-08-02 | London, UK | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**Rust London Code Dojo: Rust with Front-End Web Assembly**](https://www.meetup.com/rust-london-user-group/events/287271789/)

### North America

* 2022-07-26 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**First Official Meetup - July 2022**](https://www.meetup.com/atx-rustaceans/events/285878081/)
* 2022-07-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/287009519/)
* 2022-08-05 | Portland, OR, US | [RustConf](https://rustconf.com/)
    * [**RustConf 2022**](https://rustconf.com/)
* 2022-08-10 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydclbnb/)
* 2022-08-11 | Columbus, IL | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydclbpb/)
* 2022-08-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydclbvb/)

### Oceania

* 2022-07-28 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**July Meetup**](https://www.meetup.com/rust-brisbane/events/286889804/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org


<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> # JuSt Be CaReFuL
>
> If there’s one lesson from decades of software engineering, it is the failure of “just be careful” as a strategy. C/C++ programmers still experience memory corruption constantly, no matter how careful they are. Java programmers still frequently see NullPointerExceptions, no matter how careful they are. And so on. One of the reasons that Rust is so successful is that it adds automated checks to prevent many common mistakes.

– [Robert Grosse on their blog](https://blog.polybdenum.com/2022/07/24/fixing-the-next-thousand-deadlocks-why-buffered-streams-are-broken-and-how-to-make-them-safer.html)

Thanks to [robin](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1274) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
