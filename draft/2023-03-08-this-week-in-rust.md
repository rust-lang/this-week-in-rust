Title: This Week in Rust 485
Number: 485
Date: 2023-03-08
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
* [Fornjot (code-first CAD in Rust) - Weekly Release - Progressed Extremely Well](https://www.fornjot.app/blog/weekly-release/2023-w10/)
* [Introducing runst: Handle desktop notifications neatly on Linux!](https://blog.orhun.dev/introducing-runst)

### Observations/Thoughts
* [Re-exporting an enum with a type alias is breaking, but not major](https://predr.ag/blog/re-exporting-enum-with-type-alias-breaking-change-not-major/)
* [Professional Rustacean, 3 months in](https://briankung.dev/2023/02/17/professional-rustacean-3-months-in/)
* [Rust coding style](https://tzemanovic.gitlab.io/posts/rust-coding-style/)

### Rust Walkthroughs
* [Build a Ray Tracer, pt. 2 - Enter The Matrix](https://www.superperfundo.dev/articles/ray-tracer-part2)
* [Creating and publishing a Python package written in Rust](https://antoniosbarotsis.github.io/posts/python_package_written_in_rust/)
* [video] [Matching Braces With a Stack, Beginner Tutorial](https://www.youtube.com/watch?v=i_ghB5AusDs)

* [Embedded Rust on ESP32C3 Board, a Hands-on Quickstart Guide](https://gitlab.com/cyril.marpaud/rust_esp_quickstart/)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [man-in-the-middle-proxy](https://github.com/emanuele-em/man-in-the-middle-proxy), a - surprise! - man in the middle proxy.

Thanks to [Emanuele Em](https://users.rust-lang.org/t/crate-of-the-week/2704/1163) for the self-suggestion!

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

376 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-02-27..2023-03-06

* [apply BOLT optimizations without rebuilding LLVM](https://github.com/rust-lang/rust/pull/107723)
* [add support for QNX Neutrino to standard library](https://github.com/rust-lang/rust/pull/106673)
* [recover from for-else and while-else](https://github.com/rust-lang/rust/pull/108427)
* [allow checking whether a type allows being uninitialized](https://github.com/rust-lang/rust/pull/108669)
* [allow setting hashmap toml values in `./configure`](https://github.com/rust-lang/rust/pull/108644)
* [point error span at Some constructor argument when trait resolution fails](https://github.com/rust-lang/rust/pull/108557)
* [deny capturing late-bound non-lifetime param in anon const](https://github.com/rust-lang/rust/pull/108553)
* [descriptive error when users try to combine RPITIT/AFIT with specialization](https://github.com/rust-lang/rust/pull/108551)
* [add warning on pre- and postfix decrement](https://github.com/rust-lang/rust/pull/108496)
* [exit when there are unmatched delims to avoid noisy diagnostics](https://github.com/rust-lang/rust/pull/108297)
* [erase **all** regions when probing for associated types on ambiguity in astconv](https://github.com/rust-lang/rust/pull/108575)
* [erase regions even when failing to normalize type in MIR opts](https://github.com/rust-lang/rust/pull/108787)
* [don't ICE when encountering bound var in builtin copy/clone bounds](https://github.com/rust-lang/rust/pull/108744)
* [fix ICE: check if snippet is `)`](https://github.com/rust-lang/rust/pull/108298)
* [fix another ICE in `point_at_expr_source_of_inferred_type`](https://github.com/rust-lang/rust/pull/108667)
* [feed queries on impl side for RPITITs when using `lower_impl_trait_in_trait_to_assoc_ty`](https://github.com/rust-lang/rust/pull/108672)
* [make `associated_item_def_ids` for traits use an unstable option to also return associated types for RPITITs](https://github.com/rust-lang/rust/pull/108568)
* [new solver: implement canonicalization and region constraints](https://github.com/rust-lang/rust/pull/107981)
* [don't project to RPITIT that has no default value](https://github.com/rust-lang/rust/pull/108746)
* [desugaring of drop and replace at MIR build](https://github.com/rust-lang/rust/pull/107844)
* [don't call `temporary_scope` twice](https://github.com/rust-lang/rust/pull/108692)
* [stabilize `cmpxchg16b_target_feature`](https://github.com/rust-lang/rust/pull/106774)
* [add `Atomic*::from_ptr`](https://github.com/rust-lang/rust/pull/108540)
* [add `Option::as_`(`mut_`)`slice`](https://github.com/rust-lang/rust/pull/105871)
* [fix `VecDeque::append` capacity overflow for ZSTs](https://github.com/rust-lang/rust/pull/108462)
* [use `partial_cmp` to implement tuple `lt`/`le`/`ge`/`gt`](https://github.com/rust-lang/rust/pull/108157)
* [add vectored positioned I/O on Unix](https://github.com/rust-lang/rust/pull/89518)
* [cargo: breaking endless loop on cyclic features in added dependency in cargo-add](https://github.com/rust-lang/cargo/pull/11805)
* [cargo: fix `CARGO_CFG_` vars for configs defined both with and without value](https://github.com/rust-lang/cargo/pull/11790)
* [cargo: fix help string for "--charset" option of "cargo tree"](https://github.com/rust-lang/cargo/pull/11785)
* [cargo: fix(toml): provide a way to show unused manifest keys for dependencies](https://github.com/rust-lang/cargo/pull/11630)
* [cargo: gitoxide integration: fetch](https://github.com/rust-lang/cargo/pull/11448)
* [cargo: improve error for missing crate in --offline mode for sparse index](https://github.com/rust-lang/cargo/pull/11783)
* [cargo: make `sparse` the default protocol for crates.io](https://github.com/rust-lang/cargo/pull/11791)
* [rustdoc-json: switch from HashMap to FxHashMap to fix non-determinism](https://github.com/rust-lang/rust/pull/108626)
* [rustdoc: function signature search with traits in `where` clause](https://github.com/rust-lang/rust/pull/108723)
* [rustdoc: reduce allocations when generating tooltips](https://github.com/rust-lang/rust/pull/108098)
* [rustdoc: search by macro when query ends with `!`](https://github.com/rust-lang/rust/pull/108143)
* [rustdoc: show that repeated expression arrays can be made with constant values](https://github.com/rust-lang/rust/pull/108531)
* [clippy: downgrade `let_underscore_untyped` to restriction](https://github.com/rust-lang/rust-clippy/pull/10442)
* [clippy: fix false positive for `let_unit_value` when `await` used](https://github.com/rust-lang/rust-clippy/pull/10439)
* [clippy: fix ICE in `multiple_unsafe_ops_per_block`](https://github.com/rust-lang/rust-clippy/pull/10405)
* [clippy: fix `array-size-threshold` config deserialization error](https://github.com/rust-lang/rust-clippy/pull/10423)
* [clippy: fix various ICEs](https://github.com/rust-lang/rust-clippy/pull/10403)
* [clippy: `missing_docs_in_private_items` should cover only private items](https://github.com/rust-lang/rust-clippy/pull/10324)
* [rust-analyzer: allow `generate_function` to generate in different local crate](https://github.com/rust-lang/rust-analyzer/pull/14238)
* [rust-analyzer: diagnose unresolved field, method call and call expression](https://github.com/rust-lang/rust-analyzer/pull/14243)
* [rust-analyzer: diagnose value breaks in incorrect breakables](https://github.com/rust-lang/rust-analyzer/pull/14240)
* [rust-analyzer: make `replace_or_with_or_else` assists more generally applicable](https://github.com/rust-lang/rust-analyzer/pull/14266)
* [rust-analyzer: adjust `replace_match_with_if_let` applicability range](https://github.com/rust-lang/rust-analyzer/pull/14260)
* [rust-analyzer: don't drop rustc crates in the rustc workspace](https://github.com/rust-lang/rust-analyzer/pull/14234)
* [rust-analyzer: fix associated item visibility in block-local impls](https://github.com/rust-lang/rust-analyzer/pull/14176)
* [rust-analyzer: load the sysroot in all CLI commands](https://github.com/rust-lang/rust-analyzer/pull/14239)
* [rust-analyzer: run doctests for structs with lifetime parameters from IDE](https://github.com/rust-lang/rust-analyzer/pull/14185)
* [rust-analyzer: generate correct completion edits for missing macro arguments](https://github.com/rust-lang/rust-analyzer/pull/14247)
* [rust-analyzer: implement pattern mismatch diagnostics (but keep them disabled)](https://github.com/rust-lang/rust-analyzer/pull/14222)
* [rust-analyzer: support removing nested `dbg!()`s in `remove_dbg`](https://github.com/rust-lang/rust-analyzer/pull/14225)

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

Rusty Events between 2023-03-08 - 2023-04-05 🦀

### Virtual

* 2023-03-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - Michael Baykov on Category Theory & Argument Parsing**](https://www.meetup.com/indyrs/events/qwtdjsyfcfbcb/)
* 2023-03-02 | Virtual (Raleigh, NC, US) | [Triangle BitDevs](https://www.meetup.com/triangle-bitdevs/)
    * [**Rust for Bitcoiners**](https://www.meetup.com/triangle-bitdevs/events/291710295/)
* 2023-03-02 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 30th Edition**](https://www.meetup.com/rust-linz/events/291483339/)
* 2023-03-07 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfcfbkb/)
* 2023-03-07 | Virtual (Santa Clara, CA, US) | [UCSC Extension Community](https://www.meetup.com/ucsc-extension-community/)
    * [**Programming with Rust**](https://www.meetup.com/ucsc-extension-community/events/290906954/)
* 2023-03-08 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsryfcfblb/)
* 2023-03-11 | Virtual | [Rust GameDev](https://discord.gg/yNtPTb2)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2)
* 2023-03-14 | Virtual (Berlin, DE) | [Berlin.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2023/03/14/rust-hack-and-learn.html)
* 2023-03-14 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/291809763/)
* 2023-03-14 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Crack code interview problems in Rust: S2 Ep1**](https://www.meetup.com/microsoft-reactor-redmond/events/291676352/)
* 2023-03-15 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Injecting Rust Hooks into a 1999 game binary (unsafe)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/291354288/)
* 2023-03-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/wqchctyfcfbtb/)
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
* 2023-03-28 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Crack code interview problems in Rust: S2 Ep3**](https://www.meetup.com/microsoft-reactor-redmond/events/291677113/)
* 2023-03-29 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Writing your own rust 'book' with mdBook**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/291892487/)

### Asia

* 2023-03-04 | Kyoto, JP | [Kansai Rust](https://www.meetup.com/kansai-rust/)
    * [**Fn vs FnMut vs FnOnce**](https://www.meetup.com/kansai-rust/events/291614614/)

### Europe

* 2023-03-01 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/events)
    * [**Rust traits for Fn and profit**](https://www.meetup.com/rustcologne/events/291774935/)
* 2023-03-02 | Barcelona, ES | [BcnRust](https://bcnrust.github.io/)
    * [**9th BcnRust Meetup: Full Stack**](https://www.meetup.com/es-ES/bcnrust/events/291754590/)
* 2023-03-02 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #32**](https://www.meetup.com/rust-wroclaw/events/291776357/)
* 2023-03-07 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake**](https://www.meetup.com/bratislava-rust-meetup-group/events/291657555/)   
* 2023-03-09 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**Rust Meetup #7**](https://www.meetup.com/rust-basel/events/291228934/)
* 2023-03-09 | Delft, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Regular track: Embedded Rust**](https://www.meetup.com/rust-nederland/events/291401965/)
    * [**Student track: Embedded Rust**](https://www.meetup.com/rust-nederland/events/291401778/)
* 2023-03-09 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #2**](https://www.meetup.com/fr-FR/rust-lyon/events/291727241/)
* 2023-03-15 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Walk around Embedded World Exhibition**](https://www.meetup.com/rust-noris/events/291623203/)
* 2023-03-17 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/zmppzsyfcfbwb/)
* 2023-03-28 | Zurich, CH | [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/events/291449557/)
    * [**High performance concurrent data structures in Rust - March Meetup**](https://www.meetup.com/de-DE/rust-zurich/events/291449557/)

### North America

* 2023-03-01 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/291619816/)
* 2023-03-09 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Trails, Triumphs, & Travails of Yet-Another-Database-Crate with PJ and Food!**](https://www.meetup.com/utah-rust/events/rrwbctyfcfbmb/)

### Oceania

* 2023-03-01 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**🦀 Lightning Talks - We are back!**](https://www.meetup.com/rust-sydney/events/291265163/)

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

> (…) as much as i dislike the [cargo-geiger](https://crates.io/crates/cargo-geiger) concept, the name … kind of works
>
> `unsafe` is a lot like uranium. it’s just one more metal ore you can process, refine, and machine. it doesn’t combust in atmosphere, it doesn’t corrode or make weird acids. unless you go out of your way to make it dangerous you don’t even have to worry about critical masses. you can work with it pretty normally most of the time
>
> but if you don’t know exactly what it is, what it does, and how to work with it, it will cause mysterious illnesses that only crop up long after you’ve stopped touching it

– [Alexander Payne on /r/rust](https://www.reddit.com/r/rust/comments/11eyu50/comment/jahdf3b/)

Thanks to [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1377) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
