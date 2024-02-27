Title: This Week in Rust 536
Number: 536
Date: 2024-02-28
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

This week's crate is [web-audio-api-rs](https://github.com/orottier/web-audio-api-rs), a Rust implementation of the Web Audio API for use *outside* the browser.

Thanks to [Otto Rottier](https://users.rust-lang.org/t/crate-of-the-week/2704/1292) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
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

430 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-02-20..2024-02-27

* [Avoid non-local definitions in functions](https://github.com/rust-lang/rust/pull/120393) (RFC [#3373](https://rust-lang.github.io/rfcs/3373-avoid-nonlocal-definitions-in-fns.html))
* [wasm: store rlib metadata in wasm object files](https://github.com/rust-lang/rust/pull/120588)
* [account for RPITIT in E0310 explicit lifetime constraint suggestion](https://github.com/rust-lang/rust/pull/121435)
* [actually use the right closure kind when checking async Fn goals](https://github.com/rust-lang/rust/pull/121617)
* [add `#[rustc_no_mir_inline]` for standard library UB checks](https://github.com/rust-lang/rust/pull/121114)
* [allow for a missing `adt_def` in `NamePrivacyVisitor`](https://github.com/rust-lang/rust/pull/121482)
* [avoid generalization inside of aliases](https://github.com/rust-lang/rust/pull/119106)
* [by changing some attributes to `only_local,` reducing encoding attributes in the crate metadate](https://github.com/rust-lang/rust/pull/121493)
* [do not const prop unions](https://github.com/rust-lang/rust/pull/121628)
* [don't ICE on anonymous `struct` in `enum` variant](https://github.com/rust-lang/rust/pull/121470)
* [expand weak alias types before collecting constrained/referenced late bound regions + refactorings](https://github.com/rust-lang/rust/pull/121344)
* [fix panic when compiling `Rocket`](https://github.com/rust-lang/rust/pull/121427)
* [make intrinsic fallback bodies cross-crate inlineable](https://github.com/rust-lang/rust/pull/121309)
* [make it possible for outside crates to inspect a `mir::ConstValue` with the interpreter](https://github.com/rust-lang/rust/pull/121396)
* [make non-PartialEq-typed consts as patterns a hard error](https://github.com/rust-lang/rust/pull/120805)
* [mark `min_exhaustive_patterns` as complete](https://github.com/rust-lang/rust/pull/120742)
* [match lowering: Introduce a `TestCase enum` to replace most matching on `PatKind`](https://github.com/rust-lang/rust/pull/121393)
* [match lowering: eagerly simplify match pairs](https://github.com/rust-lang/rust/pull/120904)
* [match lowering: simplify empty candidate selection](https://github.com/rust-lang/rust/pull/121172)
* [match lowering: test one or pattern at a time](https://github.com/rust-lang/rust/pull/121175)
* [Fix liveness analysis in the presence of never patterns](https://github.com/rust-lang/rust/pull/121391)
* [no need to `validate_alias_bound_self_from_param_env` in `assemble_alias_bound_candidates`](https://github.com/rust-lang/rust/pull/120598)
* [prevent cycle in implied predicates computation](https://github.com/rust-lang/rust/pull/121409)
* [promotion: don't promote `int::MIN / -1`](https://github.com/rust-lang/rust/pull/121515)
* [properly emit `expected ;` on `#[attr] expr`](https://github.com/rust-lang/rust/pull/121651)
* [provide suggestions through `rustc_confusables` annotations](https://github.com/rust-lang/rust/pull/120730)
* [refactor trait implementations in `core::convert::num`](https://github.com/rust-lang/rust/pull/121277)
* [split Diagnostics for Uncommon Codepoints: Add Individual Identifier Types](https://github.com/rust-lang/rust/pull/120840)
* [support async trait bounds in macros](https://github.com/rust-lang/rust/pull/121044)
* [unify dylib loading between proc macros and codegen backends](https://github.com/rust-lang/rust/pull/121392)
* [when encountering `<&T as Clone>::clone(x)` because `T: Clone`, suggest `#[derive(Clone)]`](https://github.com/rust-lang/rust/pull/121471)
* [miri: /miri many-seeds: support `MIRI_SEED_END` to control the end of the seed range](https://github.com/rust-lang/miri/pull/3328)
* [miri: add "cargo miri clean" command](https://github.com/rust-lang/miri/pull/3312)
* [miri: windows miri-script execution ergonomics](https://github.com/rust-lang/miri/pull/3316)
* [use `br` instead of a conditional when switching on a constant boolean](https://github.com/rust-lang/rust/pull/120650)
* [stabilize `cfg_target_abi`](https://github.com/rust-lang/rust/pull/119590)
* [improve UEFI stdio](https://github.com/rust-lang/rust/pull/117174)
* [windows: use ProcessPrng for random keys](https://github.com/rust-lang/rust/pull/121337)
* [require `simd_insert, simd_extract` indices to be constants](https://github.com/rust-lang/rust/pull/121225)
* [make `Barrier::new()` const](https://github.com/rust-lang/rust/pull/119536)
* [implement `MappedMutexGuard`, `MappedRwLockReadGuard`, and `MappedRwLockWriteGuard`](https://github.com/rust-lang/rust/pull/117107)
* [add "algebraic" fast-math intrinsics, based on fast-math ops that cannot return poison](https://github.com/rust-lang/rust/pull/120718)
* [remove useless `'static` bounds on `Box` allocator](https://github.com/rust-lang/rust/pull/118634)
* [mpsc: fix race between block initialization and receiver disconnection](https://github.com/rust-lang/rust/pull/121646)
* [futures: add `'static` bound to `waker_ref`](https://github.com/rust-lang/futures-rs/pull/2830)
* [cargo add: Improve error when adding registry packages while vendored](https://github.com/rust-lang/cargo/pull/13281)
* [cargo: Control clap colors through config](https://github.com/rust-lang/cargo/pull/13463)
* [cargo: Respect `CARGO_TERM_COLOR` in '--list' and '-Zhelp'](https://github.com/rust-lang/cargo/pull/13479)
* [cargo: error messages when collecting workspace members now mention the workspace root location](https://github.com/rust-lang/cargo/pull/13480)
* [cargo: support `target.<triple>.rustdocflags` officially](https://github.com/rust-lang/cargo/pull/13197)
* [rustdoc: include crate name in links for local primitives](https://github.com/rust-lang/rust/pull/121490)
* [clippy: `box_default`: preserve required path segments](https://github.com/rust-lang/rust-clippy/pull/12355)
* [clippy: `read_line_without_trim`: detect string literal comparison and `.ends_with()` calls](https://github.com/rust-lang/rust-clippy/pull/11136)
* [clippy: add `unnecessary_clippy_cfg` lint](https://github.com/rust-lang/rust-clippy/pull/12303)
* [clippy: add new `multiple_bound_locations` lint](https://github.com/rust-lang/rust-clippy/pull/12259)
* [clippy: add new `unnecessary_get_then_check` lint](https://github.com/rust-lang/rust-clippy/pull/12339)
* [clippy: allow `unused_imports,` and `unused_import_braces` on `use`](https://github.com/rust-lang/rust-clippy/pull/12333)
* [clippy: don't lint `infinite_loop` in external or proc macros](https://github.com/rust-lang/rust-clippy/pull/12317)
* [clippy: make `redundant_guards` take constness into account](https://github.com/rust-lang/rust-clippy/pull/12336)
* [clippy: `unused_unit`: be careful with expressions with attributes](https://github.com/rust-lang/rust-clippy/pull/12322)
* [clippy: new lint: `empty docs`](https://github.com/rust-lang/rust-clippy/pull/12342)
* [clippy: extend `unnecessary_to_owned` to handle `Borrow` trait in map types](https://github.com/rust-lang/rust-clippy/pull/12324)
* [clippy: fix sign-handling bugs and false negatives in `cast_sign_loss`](https://github.com/rust-lang/rust-clippy/pull/12126)
* [clippy: fix suggestion error in `useless_vec`](https://github.com/rust-lang/rust-clippy/pull/12116)
* [clippy: fix `no_effect_underscore_binding` firing on ignored parameters of async fns](https://github.com/rust-lang/rust-clippy/pull/12323)
* [clippy: look for `implied_bounds_in_impls` in more positions](https://github.com/rust-lang/rust-clippy/pull/12308)
* [clippy: take lifetime extension into account in `ref_as_ptr`](https://github.com/rust-lang/rust-clippy/pull/12260)
* [rust-analyzer: add assist for filling fields by replacing ellipsis in record syntax](https://github.com/rust-lang/rust-analyzer/pull/16651)
* [rust-analyzer: add short flag -V for consistency with other rust tooling](https://github.com/rust-lang/rust-analyzer/pull/16654)
* [rust-analyzer: add "make tuple" tactic to term search](https://github.com/rust-lang/rust-analyzer/pull/16687)
* [rust-analyzer: `replace_filter_map_next_with_find_map` shouldn't work for dyn trait](https://github.com/rust-lang/rust-analyzer/pull/16647)
* [rust-analyzer: don't panic on synthetic syntax in inference diagnostics](https://github.com/rust-lang/rust-analyzer/pull/16684)
* [rust-analyzer: fix completions panicking with certain macro setups](https://github.com/rust-lang/rust-analyzer/pull/16691)
* [rust-analyzer: fix deadlock in `recreate_crate_graph <-> file_line_index`](https://github.com/rust-lang/rust-analyzer/pull/16645)
* [rust-analyzer: fix modules in blocks not resolving in ide layer](https://github.com/rust-lang/rust-analyzer/pull/16679)
* [rust-analyzer: fix proc-macro server not accounting for string delimiters correctly](https://github.com/rust-lang/rust-analyzer/pull/16637)
* [rust-analyzer: fix recompiles due to `RUSTC_BOOTSTRAP`](https://github.com/rust-lang/rust-analyzer/pull/16621)
* [rust-analyzer: panic when inlining callsites inside macros' parameters](https://github.com/rust-lang/rust-analyzer/pull/16678)
* [rust-analyzer: merge `BorrowKind::Unique` into `BorrowKind::Mut`](https://github.com/rust-lang/rust-analyzer/pull/16669)
* [rust-analyzer: speed up Method Completions By Taking Advantage of Orphan Rules](https://github.com/rust-lang/rust-analyzer/pull/16555)

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

## Upcoming Events

Rusty Events between 2024-02-28 - 2024-03-27 🦀

### Virtual

* 2024-02-21 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 2 - Types**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/298991687/)
* 2024-02-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763497/)
* 2024-02-22 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298251150/)
* 2024-02-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/299068302/)
* 2024-02-29 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298457901/) | [**Mirror: Berline.rs page**](https://berline.rs/2024/02/29/rust-hack-and-learn.html)
* 2024-02-29 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Surfing the Rusty Wireless Waves with the ESP32-C3 Board**](https://www.meetup.com/charlottesville-rust-meetup/events/298372724/)
* 2024-03-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047891/)
* 2024-03-07 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368787/)
* 2024-03-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341582/)
* 2024-03-12 | Hybrid (Virtual + In-person) Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-03-14 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Web Frontend Co-Learning (online)**](https://www.meetup.com/opentechschool-berlin/events/298406445/)
* 2024-03-21 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/298631832/)
* 2024-03-26 | Virtual + In Person (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**13th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/299223178/) - [Stream](https://www.youtube.com/@bcnrust)

### Europe

* 2024-02-21 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #8**](https://www.meetup.com/fr-FR/rust-lyon/events/298775631/)
* 2024-02-22 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Partisia**](https://www.meetup.com/rust-aarhus/events/298689622/)
* 2024-02-29 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Season start 2024**](https://www.meetup.com/rust-berlin/events/299190389/)
* 2024-03-12 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-03-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/299028814/)
* 2024-03-20 | Girona, ES | [Rust Girona](https://www.meetup.com/rust-girona/)
    * [**Introduction to programming Microcontrollers with Rust**](https://www.meetup.com/rust-girona/events/299172343/)
* 2024-03-26 | Barcelona, ES + Virtual | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**13th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/299223178/)
* 2024-03-26, 2024-03-28 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation 2024**](https://www.rustnationuk.com/)

### North America

* 2024-02-21 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Evening Boston Rust Meetup at Microsoft, February 21**](https://www.meetup.com/bostonrust/events/299054786/)
* 2024-02-22 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299043763/)
* 2024-02-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/297380841/)
* 2024-03-07 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299043793/)

### Oceania

* 2024-02-27 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**February Meetup**](https://www.meetup.com/rust-canberra/events/297650401/)
* 2024-02-27 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**🦀 spire ⚡ & Quick**](https://www.meetup.com/rust-sydney/events/298892952/)
* 2024-02-29 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**February Meetup**](https://www.meetup.com/rust-brisbane/events/299304438/)
* 2024-03-05 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Introduction to Embedded Rust + The State of Rust UI**](https://www.meetup.com/rust-akl/events/299158887/)

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

> That would take 18 million terabytes of RAM. You don't have that much memory.

– [Alice Ryhl answering "What is MAX array size" on rust-users](https://users.rust-lang.org/t/what-is-max-array-size/107058/4)

Thanks to [Zeroexcuses](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1536) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
