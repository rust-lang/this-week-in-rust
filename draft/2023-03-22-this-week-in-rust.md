Title: This Week in Rust 487
Number: 487
Date: 2023-03-22
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

This week's crate is [Speedy2D](https://github.com/QuantumBadger/Speedy2D), a crate offering cross-platform Hardware-accelerated drawing of shapes, images, and text, with an easy to use API.

Thanks to [Aleksey Kladov](https://users.rust-lang.org/t/crate-of-the-week/2704/1169) for the suggestion!

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

321 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-03-13..2023-03-20

* [`inherit_overflow`: adapt pattern to also work with v0 mangling](https://github.com/rust-lang/rust/pull/109181)
* [`read_buf_exact`: on error, all read bytes are appended to the buffer](https://github.com/rust-lang/rust/pull/109022)
* [add `enable-warnings` flag for llvm, and disable it by default](https://github.com/rust-lang/rust/pull/108991)
* [add `useless_anonymous_reexport` lint](https://github.com/rust-lang/rust/pull/109003)
* [add note for mismatched types because of circular dependencies](https://github.com/rust-lang/rust/pull/105793)
* [do not ICE for unexpected lifetime with ConstGeneric rib](https://github.com/rust-lang/rust/pull/109222)
* [don't ICE for late-bound consts across `AnonConstBoundary`](https://github.com/rust-lang/rust/pull/109105)
* [don't suggest similar method when unstable](https://github.com/rust-lang/rust/pull/109212)
* [fix ICE in `custom-test-frameworks` feature](https://github.com/rust-lang/rust/pull/109301)
* [fix ClashingExternDeclarations lint ICE](https://github.com/rust-lang/rust/pull/109370)
* [emit diagnostic when calling methods on the unit type in method chains](https://github.com/rust-lang/rust/pull/109116)
* [ensure `ptr::read` gets all the same LLVM `load` metadata that dereferencing does](https://github.com/rust-lang/rust/pull/109035)
* [erase escaping late-bound regions when probing for ambiguous associated types](https://github.com/rust-lang/rust/pull/109102)
* [error-msg: expand suggestion for `unused_def` lint](https://github.com/rust-lang/rust/pull/109158)
* [error-msg: impl better suggestion for `E0532`](https://github.com/rust-lang/rust/pull/108971)
* [fall back to old metadata computation when type references errors](https://github.com/rust-lang/rust/pull/109101)
* [fast path for `process_obligations`](https://github.com/rust-lang/rust/pull/108815)
* [fix `generics_of` for impl's RPITIT (Return Position Impl Trait In Trait) synthesized associated type](https://github.com/rust-lang/rust/pull/109277)
* [fix generics mismatch errors for RPITITs on -Zlower-impl-trait-in-trait-to-assoc-ty](https://github.com/rust-lang/rust/pull/109238)
* [install projection from RPITIT to default trait method opaque correctly](https://github.com/rust-lang/rust/pull/109198)
* [make fns from other crates with RPITIT work for `-Zlower-impl-trait-in-trait-to-assoc-ty`](https://github.com/rust-lang/rust/pull/108923)
* [fix object safety checks for new RPITITs](https://github.com/rust-lang/rust/pull/108909)
* [fix linker detection for clang with prefix](https://github.com/rust-lang/rust/pull/109156)
* [flatten/inline `format_args!()` and (string and int) literal arguments into `format_args!()`](https://github.com/rust-lang/rust/pull/106824)
* [implement `FixedSizeEncoding` for `UnusedGenericParams`](https://github.com/rust-lang/rust/pull/109324)
* [implement checked `Shl`/`Shr` at MIR building](https://github.com/rust-lang/rust/pull/108282)
* [only expect a GAT const param for `type_of` of GAT const arg](https://github.com/rust-lang/rust/pull/109364)
* [pass the right HIR back from `get_fn_decl`](https://github.com/rust-lang/rust/pull/109248)
* [remove `identity_future` indirection](https://github.com/rust-lang/rust/pull/104833)
* [remove box expressions from HIR](https://github.com/rust-lang/rust/pull/108958)
* [replace ZST operands and debuginfo by constants](https://github.com/rust-lang/rust/pull/107270)
* [simplify proc macro signature validity check](https://github.com/rust-lang/rust/pull/109136)
* [some cleanups in our normalization logic](https://github.com/rust-lang/rust/pull/109171)
* [suggest surrounding the macro with `{}` to interpret as a statement](https://github.com/rust-lang/rust/pull/109251)
* [use `unused_generic_params` from crate metadata](https://github.com/rust-lang/rust/pull/109109)
* [miri: move reject with isolation logic in fcntl](https://github.com/rust-lang/miri/pull/2787)
* [miri: tree borrows](https://github.com/rust-lang/miri/pull/2785)
* [properly allow macro expanded `format_args` invocations to use captures](https://github.com/rust-lang/rust/pull/106505)
* [optimize dep node backtrace and ignore fatal errors](https://github.com/rust-lang/rust/pull/108524)
* [fallback to lstat when stat fails on Windows](https://github.com/rust-lang/rust/pull/109235)
* [stabilise `unix_socket_abstract`](https://github.com/rust-lang/rust/pull/109288)
* [stabilize `atomic_as_ptr`](https://github.com/rust-lang/rust/pull/108419)
* [use index based drop loop for slices and arrays](https://github.com/rust-lang/rust/pull/109085)
* [allow using `Range` as an `Iterator` in const contexts](https://github.com/rust-lang/rust/pull/104100)
* [cargo: accurately show status when downgrading dependencies](https://github.com/rust-lang/cargo/pull/11839)
* [cargo: add `--ignore-rust-version` flag to cargo install](https://github.com/rust-lang/cargo/pull/11859)
* [cargo: add more information to wait-for-publish](https://github.com/rust-lang/cargo/pull/11713)
* [cargo: align semantics of generated vcs ignore files](https://github.com/rust-lang/cargo/pull/11855)
* [cargo: handle case mismatches when looking up env vars in the Config snapshot](https://github.com/rust-lang/cargo/pull/11824)
* [rustdoc: correctly merge import's and its target's docs in one more case](https://github.com/rust-lang/rust/pull/109266)
* [rustdoc: docFS: replace rayon with threadpool and enable it for all targets](https://github.com/rust-lang/rust/pull/109139)
* [rustdoc: implement bag semantics for function parameter search](https://github.com/rust-lang/rust/pull/109331)
* [clippy: add `allow_attribute` lint](https://github.com/rust-lang/rust-clippy/pull/10481)
* [clippy: new lint to detect `&std::path::MAIN_SEPARATOR.to_string()`](https://github.com/rust-lang/rust-clippy/pull/10483)
* [clippy: enhance `ifs_same_cond` to warn same immutable method calls as well](https://github.com/rust-lang/rust-clippy/pull/10350)
* [clippy: fix `almost_swapped` false positive (`let mut a = b; a = a`)](https://github.com/rust-lang/rust-clippy/pull/10499)
* [clippy: fix `almost_swapped`: Ignore external macros](https://github.com/rust-lang/rust-clippy/pull/10502)
* [clippy: issue function modifiers in the right order in `manual_async_fn` lint](https://github.com/rust-lang/rust-clippy/pull/10456)
* [rust-analyzer: add an autofix for inserting an unsafe block to missing unsafe diagnostic](https://github.com/rust-lang/rust-analyzer/pull/14281)
* [rust-analyzer: prioritize missing variants in match pattern completions](https://github.com/rust-lang/rust-analyzer/pull/13789)
* [rust-analyzer: allow the status bar item to be clicked again](https://github.com/rust-lang/rust-analyzer/pull/14337)
* [rust-analyzer: fix reference completions being emitted in places other than argument lists](https://github.com/rust-lang/rust-analyzer/pull/14355)
* [rust-analyzer: fix rustc proc-macro handling being broken on the rustc workspace itself](https://github.com/rust-lang/rust-analyzer/pull/14348)
* [rust-analyzer: fix visibility resolution not respecting parent blocks](https://github.com/rust-lang/rust-analyzer/pull/14349)
* [rust-analyzer: only skip adjustment hints for block, if and match expressions for reborrows](https://github.com/rust-lang/rust-analyzer/pull/14338)
* [rust-analyzer: lint incoherent inherent impls](https://github.com/rust-lang/rust-analyzer/pull/13994)

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

Rusty Events between 2023-03-22 - 2023-04-19 🦀

### Virtual

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
    * [**Mid-month Rustful—Using Category Theory to Parse Command-line Options**](https://www.meetup.com/rustdc/events/vdhxgsyfcfbcc/)
* 2023-03-22 | Virtual (Richmond, VA, US) | [Rustaceans RVA](https://www.meetup.com/rustaceans-rva/)
    * [**Rustaceans RVA - March Meetup**](https://www.meetup.com/rustaceans-rva/events/291963911/)
* 2023-03-27 | Virtual | [Rust Formal Methods Interest Group](https://www.eventbrite.com/cc/rfmig-87969)
    * [**Flux: Ergonomic Verification of Rust Programs with Liquid Types**](https://www.eventbrite.com/e/flux-ergonomic-verification-of-rust-programs-with-liquid-types-tickets-577742873487?aff=ebdssbonlinesearch)
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
* 2023-04-11 | Virtual | [Rust Live](https://www.eventbrite.com/cc/rust-live-1876809)
    * [**Rust Live: Asynchronous Rust**](https://www.eventbrite.com/e/rust-live-asynchronous-rust-tickets-575865518267?aff=ebdssbonlinesearch&keep_tld=1)

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
* 2023-04-04 | Berlin, DE | [Berline.rs](https://berline.rs)
    * [**Rust and Tell - Goodbye👋 Edition**](https://berline.rs/2023/04/04/rust-and-tell-goodbye-edition.html)

### North America

* 2023-03-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfcfbcc/)

### Asia
* 2023-04-08 | Kyoto, JP | [Kansai Rust](https://www.meetup.com/kansai-rust/)
    * [**Demystifying Closures**](https://www.meetup.com/kansai-rust/events/292202435/) 

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

> The generated program is a random sequence of bytes that just happens to take the shape of a seemingly working program by accident. Such is the joy of code that causes UB. You cannot deduce anything from what happens when you execute a program with UB, since that act is by itself meaningless. You need to establish that your program has no UB before making any inference based on what you see the program do after it came out of the compiler.

– [Ralf Jung on github](https://github.com/rust-lang/miri/issues/2807#issuecomment-1462385947)

Thanks to [bugaevc](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1381) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
