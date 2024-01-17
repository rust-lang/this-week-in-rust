Title: This Week in Rust 530
Number: 530
Date: 2024-01-17
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
* [esp-rs quarterly planning: Q1 2024](https://beta7.io/posts/esp-rs-quarterly-planning-q1-2024/)
* [Meilisearch 1.6 - AI search & 2x faster indexing](https://blog.meilisearch.com/meilisearch-1-6/)

### Observations/Thoughts

### Rust Walkthroughs
* [Adventures in Binary Serialisation](https://blog.maguire.tech/posts/explorations/binary-serialisation/)
* [Creating 2 'Simple' Allocators](https://blog.maguire.tech/posts/explorations/allocators/)
* [Rust macros and Lambda boilerplate](https://medium.com/@sam.van.overmeire/rust-macros-taking-care-of-some-lambda-boilerplate-96244d9e1924)

### Research

### Miscellaneous
* [Clap - subcommands for command line applications in Rust](https://rust.code-maven.com/clap-subcommand)
* [SurrealDB in-memory with SQL demo in Rust](https://rust.code-maven.com/surrealdb-in-memory-with-sql-demo)
* [Multi-counter with embedded SurrealDB database](https://rust.code-maven.com/surrealdb-cli-multi-counter)

## Crate of the Week

This week's crate is [fish](https://github.com/fish-shell/fish-shell), a *f*riendly *i*nteractive *sh*ell that used to be written in C++, but was recently rewritten in Rust (though admittedly they'll have to do some work until it hits your distro's repos).

Despite a lamentable lack of suggestions, llogiq is reasonably satisfied with his choice.

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

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website either through a PR to TWiR or on the [Rust-lang forums].[link TBD]

## Updates from the Rust Project

418 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-01-09..2024-01-16

* [GNU/Hurd: unconditionally use inline stack probes](https://github.com/rust-lang/rust/pull/119758)
* [`rustc_codegen_ssa`: Enforce `rustc::potential_query_instability` lint](https://github.com/rust-lang/rust/pull/119409)
* [`~const` trait and projection bounds do not imply their non-const counterparts](https://github.com/rust-lang/rust/pull/119721)
* [add assume into `NonZeroIntX::get`](https://github.com/rust-lang/rust/pull/119452)
* [add explicit `none()` value variant in check-cfg](https://github.com/rust-lang/rust/pull/119473)
* [add more information to `visit_projection_elem`](https://github.com/rust-lang/rust/pull/119877)
* [allow `~const` on associated type bounds again](https://github.com/rust-lang/rust/pull/119894)
* [avoid silencing relevant follow-up errors](https://github.com/rust-lang/rust/pull/117449)
* [check rust lints when an unknown lint is detected](https://github.com/rust-lang/rust/pull/119819)
* [coverage: add enums to accommodate other kinds of coverage mappings](https://github.com/rust-lang/rust/pull/119842)
* [coverage: simplify building the coverage graph with `CoverageSuccessors`](https://github.com/rust-lang/rust/pull/119508)
* [delegation implementation: step 1](https://github.com/rust-lang/rust/pull/118947)
* [disallow reference to `static mut` and adding `static_mut_ref` lint](https://github.com/rust-lang/rust/pull/117556)
* [don't ICE when noting GAT bounds in `report_no_match_method_error`](https://github.com/rust-lang/rust/pull/119944)
* [don't reexport `atomic::ordering` via `rustc_data_structures,` use std import](https://github.com/rust-lang/rust/pull/119527)
* [exclude well known names from showing a suggestion in check-cfg](https://github.com/rust-lang/rust/pull/118924)
* [exhaustiveness: abort on type error](https://github.com/rust-lang/rust/pull/119715)
* [exhaustiveness: track overlapping ranges precisely](https://github.com/rust-lang/rust/pull/119396)
* [exhaustiveness: use an `Option` instead of allocating fictitious patterns](https://github.com/rust-lang/rust/pull/119688)
* [fix ICE when suggesting dereferencing binop operands](https://github.com/rust-lang/rust/pull/119361)
* [fix `all_trait*` methods to return all traits available in StableMIR](https://github.com/rust-lang/rust/pull/119790)
* [fix `allow_internal_unstable` for `(min_)specialization`](https://github.com/rust-lang/rust/pull/119963)
* [fix `is_global` special address handling](https://github.com/rust-lang/rust/pull/119006)
* [fix `unused_parens` issue when cast is followed LT](https://github.com/rust-lang/rust/pull/117321)
* [fix an ICE that occurs after an error has already been reported](https://github.com/rust-lang/rust/pull/119772)
* [new flag to emit all the delayed bugs as errors (add `-Zeagerly-emit-delayed-bugs`)](https://github.com/rust-lang/rust/pull/119872)
* [making `User<T>` and `User<[T]> Send`](https://github.com/rust-lang/rust/pull/118241)
* [merge dead bb pruning and unreachable bb deduplication](https://github.com/rust-lang/rust/pull/119699)
* [never patterns: Check bindings wrt never patterns](https://github.com/rust-lang/rust/pull/119610)
* [pass LLVM error message back to pass wrapper](https://github.com/rust-lang/rust/pull/119637)
* [register even erroneous impls](https://github.com/rust-lang/rust/pull/119868)
* [remove `-Zdont-buffer-diagnostics`](https://github.com/rust-lang/rust/pull/119723)
* [stop mentioning internal lang items in `no_std` binary errors](https://github.com/rust-lang/rust/pull/116343)
* [store the segment name when resolution fails](https://github.com/rust-lang/rust/pull/119925)
* [suggest upgrading compiler for gated features](https://github.com/rust-lang/rust/pull/119088)
* [suggest quoting unquoted idents in attrs](https://github.com/rust-lang/rust/pull/119341)
* [support async recursive calls (as long as they have indirection)](https://github.com/rust-lang/rust/pull/117703)
* [taint `_` placeholder types in trait impl method signatures](https://github.com/rust-lang/rust/pull/119896)
* [unify query canonicalization mode](https://github.com/rust-lang/rust/pull/118968)
* [remove a large amount of leb128-coded integers](https://github.com/rust-lang/rust/pull/119791)
* [varargs support for system ABI](https://github.com/rust-lang/rust/pull/119587)
* [stabilize `mutex_unpoison` feature](https://github.com/rust-lang/rust/pull/119804)
* [A more efficient slice comparison implementation for T: !BytewiseEq](https://github.com/rust-lang/rust/pull/116846)
* [tune the inlinability of `unwrap`](https://github.com/rust-lang/rust/pull/119878)
* [cargo metadata: Stabilize id format as PackageIDSpec](https://github.com/rust-lang/cargo/pull/12914)
* [cargo resolver: do not panic when sorting empty summaries](https://github.com/rust-lang/cargo/pull/13287)
* [cargo: add guidance on setting homepage in manifest](https://github.com/rust-lang/cargo/pull/13293)
* [cargo: add unstable `--output-format` option to `cargo rustdoc`](https://github.com/rust-lang/cargo/pull/12252)
* [cargo: crates-io: set `Content-Type: application/json` only for requests with a body payload](https://github.com/rust-lang/cargo/pull/13264)
* [cargo: add `rustc` style errors for manifest parsing](https://github.com/rust-lang/cargo/pull/13172)
* [cargo: only inherit workspace package table if the new package is a member](https://github.com/rust-lang/cargo/pull/13261)
* [cargo: implementation of shallow libgit2 fetches behind an unstable flag](https://github.com/rust-lang/cargo/pull/13252)
* [cargo: introduce `-Zprecise-pre-release` unstable flag](https://github.com/rust-lang/cargo/pull/13296)
* [cargo: strip debuginfo when debuginfo is not requested](https://github.com/rust-lang/cargo/pull/13257)
* [rustdoc-search: reuse individual types in function signatures](https://github.com/rust-lang/rust/pull/119756)
* [clippy: `from_over_into`: suggest a correct conversion to ()](https://github.com/rust-lang/rust-clippy/pull/12141)
* [clippy: `useless_asref`: check that the clone receiver is the parameter](https://github.com/rust-lang/rust-clippy/pull/12136)
* [clippy: correctly suggest `std` or `core` path depending if this is a `no_std` crate](https://github.com/rust-lang/rust-clippy/pull/12149)
* [clippy: extend `useless_asref` lint on `map(clone)`](https://github.com/rust-lang/rust-clippy/pull/12105)
* [clippy: fix false positive in `PartialEq` check in `unconditional_recursion` lint](https://github.com/rust-lang/rust-clippy/pull/12137)
* [clippy: fix suggestion for `map_clone` lint on types implementing `Copy`](https://github.com/rust-lang/rust-clippy/pull/12129)
* [clippy: make `HirEqInterExpr::eq_block` take comments into account while checking if two blocks are equal](https://github.com/rust-lang/rust-clippy/pull/12074)
* [rust-analyzer: add `notable_trait` predicate to `CompletionRelevance`](https://github.com/rust-lang/rust-analyzer/pull/16274)
* [rust-analyzer: assist to merge nested if](https://github.com/rust-lang/rust-analyzer/pull/16209)
* [rust-analyzer: acknowledge `pub(crate)` imports in import suggestions](https://github.com/rust-lang/rust-analyzer/pull/16265)
* [rust-analyzer: differentiate between vfs config load and file changed events](https://github.com/rust-lang/rust-analyzer/pull/16319)
* [rust-analyzer: fix `ast::Path::segments` implementation](https://github.com/rust-lang/rust-analyzer/pull/16275)
* [rust-analyzer: fix incorrect parsing error on method call on range](https://github.com/rust-lang/rust-analyzer/pull/16310)
* [rust-analyzer: fix nested includes resolving from the wrong base file](https://github.com/rust-lang/rust-analyzer/pull/16348)
* [rust-analyzer: fix rust-analyzer-proc-macro-srv failing to spawn on windows](https://github.com/rust-lang/rust-analyzer/pull/16312)
* [rust-analyzer: preserve comments for extracted block expression in `'extract_function'`](https://github.com/rust-lang/rust-analyzer/pull/16333)
* [rust-analyzer: remove sysroot-abi feature flag from proc-macro-test](https://github.com/rust-lang/rust-analyzer/pull/16271)
* [rust-analyzer: replace SourceRootCrates hashset output with slice for deterministic order](https://github.com/rust-lang/rust-analyzer/pull/16339)
* [rust-analyzer: resolve panic in `generate_delegate_methods`](https://github.com/rust-lang/rust-analyzer/pull/16277)

### Rust Compiler Performance Triage

This week had some small regressions that did not warrant further investigation,
several of which were dismissed as being noise/blips in the data. There were
also a number of gains. (Don't get exicited about that 20.6% improvement, its an
measurement artifact from a temporary blip in the PR that immediately preceded
this week's triage.)

Triage done by **@pnkfelix**.
Revision range: [76101eec..f9c2421a](https://perf.rust-lang.org/?start=76101eecbe9aa80753664bbe637ad06d1925f315&end=f9c2421a2a6e34f3756900dd7d600704c08bfccb&absolute=false&stat=instructions%3Au)

3 Regressions, 5 Improvements, 5 Mixed; 3 of them in rollups
55 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/703f3ddf6f2777a4ab91e2a6f4e369b8f690cdfc/triage/2024-01-16.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Precise Pre-release cargo update](https://github.com/rust-lang/rfcs/pull/3493)
* [Add RFC combining Infra and Release teams](https://github.com/rust-lang/rfcs/pull/3533)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Avoid non-local definitions in functions](https://github.com/rust-lang/rfcs/pull/3373)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Stabilize single-field offset_of](https://github.com/rust-lang/rust/pull/118799)
* [disposition: merge] [Undeprecate lint `unstable_features` and make use of it in the compiler](https://github.com/rust-lang/rust/pull/118639)
* [disposition: close] [Fix `non_camel_case_types` for screaming single-words](https://github.com/rust-lang/rust/pull/116389)

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Out-of-tree test suite](https://github.com/rust-lang/rfcs/pull/3557)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2024-01-17 - 2024-02-14 🦀

### Virtual

* 2024-01-11 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/297687491/)
* 2024-01-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/295679708/)
* 2024-01-11 | Virtual (San Diego, CA, US) | [San Diego Rust](https://www.meetup.com/san-diego-rust/)
    * [**San Diego Rust January 2024 Tele-Meetup**](https://www.meetup.com/san-diego-rust/events/298441403/)
* 2024-01-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/297128172/)
* 2024-01-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763502/)
* 2024-01-23 | Virtual (Halifax, NS, CA) | [Rust Halifax](https://www.meetup.com/rust-tell-halifax/)
    * [**Rust&Tell - Halifax**](https://www.meetup.com/rust-tell-halifax/events/298011202/)
* 2024-01-24 | Virtual (Berlin, DE) | [WeAreDevelopers Community](https://www.meetup.com/wearedevelopers-community/)
    * [**WeAreDevelopers LIVE - Rust Day**](https://www.meetup.com/wearedevelopers-community/events/297065638/)
* 2024-01-25 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298058222/)
* 2024-01-25 | Virtual (Mexico City, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Iniciando 2024 con Rust**](https://www.meetup.com/rust-mx/events/298439198/)
* 2024-01-28 | Virtual (Wrocław, PL) | [Stacja IT Wrocław](https://www.meetup.com/stacja-it-wroclaw/)
    * [**Wprowadzenie do języka Rust**](https://www.meetup.com/stacja-it-wroclaw/events/297899705/)
* 2024-01-30 | Virtual | [Bevy Game Development](https://www.meetup.com/bevy-game-development/)
    * [**Bevy Meetup #1**](https://www.meetup.com/bevy-game-development/events/298399958/)
* 2024-01-30 | Virtual (Buffalo, NY, US) | [Buffalo Rust User Group](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/297965826/)
* 2024-01-30 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtygccbnc/)
* 2024-02-01 | Barcelona, ES | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**12th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/297439924/) - [Stream](https://www.youtube.com/@bcnrust)
* 2024-02-01 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/)
    * [**Rust Hack n Learn**](https://meet.jit.si/RustHackAndLearnBerlin)
* 2024-02-03 | Virtual + In-person (Brussels, BE) | [FOSDEM 2024](https://fosdem.org/2024/)
    * [**FOSDEM Conference: Rust devroom - talks**](https://fosdem.org/2024/schedule/track/rust/)
* 2024-02-03 | Virtual (Kampala, UG) | [Rust Circle](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdsoporgprofile)
* 2024-02-04 | Virtual | [Rust Maven](https://meet-os.com/group/1)
    * [**Web development with Rocket - In English**](https://meet-os.com/event/1)
* 2024-02-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftygcdbkb/)
* 2024-02-08 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298251149/)

### Europe

* 2024-01-10 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/)
    * [**Game development in Rust**](https://www.meetup.com/rustcologne/events/298303772/)
* 2024-01-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/296020357/)
* 2024-01-11 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Meetup #36**](https://www.meetup.com/rust-wroclaw/events/298029291/)
* 2024-01-13 | Tampere, FI | [Finland Rust-lang Group](https://www.meetup.com/finland-rust-meetup/)
    * [**January Meetup**](https://www.meetup.com/finland-rust-meetup/events/297811750/)
* 2024-01-16 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Async in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/297376712/)
* 2024-01-17 | Girona, ES | [Rust Girona](https://www.meetup.com/rust-girona/)
    * [**Que és Rust i els seus beneficis / What's Rust and its advantages**](https://www.meetup.com/rust-girona/events/294080437/)
* 2024-01-17 | Praha / Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Reloaded 2024**](https://www.meetup.com/rust-prague/events/298005196/) 
* 2024-01-17 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**How to Community - January Meetup**](https://www.meetup.com/rust-zurich/events/298066842/)
* 2024-01-23 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack and Learn**](https://www.meetup.com/rust-aarhus/events/297463730/)
* 2024-01-23 | Paris, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [**Paris Rust Meetup #64**](https://mobilizon.fr/events/0fce31cd-3578-43f2-abf4-ffecd8d16da2)
* 2024-01-24 | Zagreb, HR | [impl Zagreb for Rust](https://www.meetup.com/Zagreb-Rust-Meetup/)
    * [**Rust Meetup 2024/01: WebGPU intro using Rust**](https://www.meetup.com/zagreb-rust-meetup/events/298540606/)
* 2024-02-01 | Barcelona, ES | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**12th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/297439924/)
* 2024-02-03 | Brussels, BE | [FOSDEM '24](https://fosdem.org/2024/)
    * [**FOSDEM '24 Conference: Rust devroom - talks**](https://fosdem.org/2024/schedule/track/rust/) | [**Rust Aarhus FOSDEM Meetup**](https://www.meetup.com/rust-aarhus/events/295946777/)

### North America

* 2024-01-11 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Arena Allocation: Another approach to managing lifetimes w/Taylor Allred**](https://www.meetup.com/utah-rust/events/298448713/)
* 2024-01-14 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Alewife Rust Lunch**](https://www.meetup.com/bostonrust/events/297634920/)
* 2024-01-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/297452643/)
* 2024-01-17 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Happy Hour**](https://www.meetup.com/deep-dish-rust/events/298003233/)
* 2024-01-18 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/298304117/)
* 2024-01-22 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**North End Rust Lunch**](https://www.meetup.com/bostonrust/events/297634962/)
* 2024-01-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygccbgc/)
* 2024-01-27-28 | Calgary, AB, CA | [Rust Calgary](https://www.eventbrite.ca/o/rust-calgary-63449860593)
    * [**Harnessing Rust for Real-World Problems hackathon: Day 1**](https://www.eventbrite.ca/e/harnessing-rust-for-real-world-problems-hackathon-day-1-tickets-794992302377?aff=ebdsoporgprofile)
    * [**Harnessing Rust for Real-World Problems hackathon: Day 2**](https://www.eventbrite.ca/e/harnessing-rust-for-real-world-problems-hackathon-day-2-tickets-794994147897?aff=ebdsoporgprofile)  
* 2024-01-30 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Harvard Square Rust Lunch**](https://www.meetup.com/bostonrust/events/297634994/)
* 2024-02-07 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, Feb 7**](https://www.meetup.com/bostonrust/events/297635028/)

### Oceania

* 2024-01-16 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/298382221/)
* 2024-02-06 | Perth, WA, AU | [Perth Rust Meetup Group](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Rust Feb 2024 Meetup**](https://www.meetup.com/perth-rust-meetup-group/events/297330668/)

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

> Congrats to the #Rustlang and #Rust-for-#Linux community: the #LinuxKernel now contains the first useful thing built using Rust!

– [Thorsten Leemhuis on FOSStodon](https://fosstodon.org/@kernellogger/111741507899977461)

As with the crate of the week, this week saw a total lack of suggestions, so llogiq would like to offer you this piece of good news from the Linux side of things.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
