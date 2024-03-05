Title: This Week in Rust 537
Number: 537
Date: 2024-03-06
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

This week's crate is [srgn](https://github.com/alexpovel/srgn), a mix of tr, sed, rip-grep and tree-sitter.

Thanks to [Alex Povel](https://users.rust-lang.org/t/crate-of-the-week/2704/1294) for the self-suggestion!

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

488 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-02-27..2024-03-05

* [`ffi_unwind_calls`: treat RustIntrinsic like regular Rust calls](https://github.com/rust-lang/rust/pull/121548)
* [`pattern_analysis`: rework how we hide empty private fields](https://github.com/rust-lang/rust/pull/121000)
* [rustc: fix wasm64 metadata object files](https://github.com/rust-lang/rust/pull/121464)
* [add a proper `with_no_queries` to printing](https://github.com/rust-lang/rust/pull/121927)
* [add a way to add constructors for `rustc_type_ir` types](https://github.com/rust-lang/rust/pull/121703)
* [add initial support for DataFlowSanitizer](https://github.com/rust-lang/rust/pull/120761)
* [add new `pattern_complexity` attribute to add possibility to limit and check recursion in pattern matching](https://github.com/rust-lang/rust/pull/121917)
* [add platform-specific function to get the error number for HermitOS](https://github.com/rust-lang/rust/pull/121765)
* [add profiling support to AIX](https://github.com/rust-lang/rust/pull/121730)
* [add proper cfg to keep only one AlignmentEnum definition for different `target_pointer_widths`](https://github.com/rust-lang/rust/pull/121753)
* [allow statics pointing to mutable statics](https://github.com/rust-lang/rust/pull/121782)
* [always generate GEP i8 / ptradd for `struct` offsets](https://github.com/rust-lang/rust/pull/121665)
* [avoid collecting into vecs in some places](https://github.com/rust-lang/rust/pull/121895)
* [avoid using unnecessary queries when printing the query stack in panics](https://github.com/rust-lang/rust/pull/121993)
* [consider middle segments of paths in `unused_qualifications`](https://github.com/rust-lang/rust/pull/121528)
* [correctly generate item info of trait items](https://github.com/rust-lang/rust/pull/121855)
* [delete line if suggestion would replace it with an empty line](https://github.com/rust-lang/rust/pull/120305)
* [display short types for unimplemented trait](https://github.com/rust-lang/rust/pull/121739)
* [don't grab variances in `TypeRelating` relation if we're invariant](https://github.com/rust-lang/rust/pull/121864)
* [don't panic when waiting on poisoned queries](https://github.com/rust-lang/rust/pull/121913)
* [fix incorrect suggestion for uninitialized binding in pattern](https://github.com/rust-lang/rust/pull/120646)
* [fix issues in suggesting importing extern crate paths](https://github.com/rust-lang/rust/pull/121226)
* [fix link generation for foreign macro in jump to definition feature](https://github.com/rust-lang/rust/pull/121680)
* [implement missing ABI structures in StableMIR](https://github.com/rust-lang/rust/pull/121824)
* [improve renaming suggestion when item starts with underscore](https://github.com/rust-lang/rust/pull/121792)
* [made `INVALID_DOC_ATTRIBUTES` lint deny by default](https://github.com/rust-lang/rust/pull/111505)
* [make `ZeroablePrimitive` trait unsafe](https://github.com/rust-lang/rust/pull/121850)
* [make the success arms of `if lhs || rhs` meet up in a separate block](https://github.com/rust-lang/rust/pull/121784)
* [match lowering: Lower bindings in a predictable order](https://github.com/rust-lang/rust/pull/121716)
* [match lowering: Separate the `bool` case from other integers in `TestKind`](https://github.com/rust-lang/rust/pull/121750)
* [match lowering: pre-simplify or-patterns too](https://github.com/rust-lang/rust/pull/121715)
* [never say "`Trait is implemented for `{type error}`"`](https://github.com/rust-lang/rust/pull/121803)
* [normalizes-to: handle negative impls](https://github.com/rust-lang/rust/pull/121853)
* [opportunistically resolve regions when processing region outlives obligations](https://github.com/rust-lang/rust/pull/121743)
* [pattern analysis: Don't panic when encountering unexpected constructor](https://github.com/rust-lang/rust/pull/121735)
* [pattern analysis: abort on arity mismatch](https://github.com/rust-lang/rust/pull/121987)
* [preserve same vtable pointer when cloning raw waker, to fix `Waker::will_wake`](https://github.com/rust-lang/rust/pull/121622)
* [process alias-relate obligations in CoerceUnsized loop](https://github.com/rust-lang/rust/pull/121702)
* [properly deal with GATs when looking for method chains to point at](https://github.com/rust-lang/rust/pull/121912)
* [safe Transmute: Revise safety analysis](https://github.com/rust-lang/rust/pull/121681)
* [skip unnecessary comparison with half-open range patterns](https://github.com/rust-lang/rust/pull/121376)
* [split `rustc_type_ir` to avoid `rustc_ast` from depending on it](https://github.com/rust-lang/rust/pull/121695)
* [style library/core/src/error.rs](https://github.com/rust-lang/rust/pull/121888)
* [suggest moving definition if non-found `macro_rules!` is defined later](https://github.com/rust-lang/rust/pull/121130)
* [suggest removing superfluous semicolon when statements used as expression](https://github.com/rust-lang/rust/pull/121153)
* [the ordinary lowering of `thir::ExprKind::Let` is unreachable](https://github.com/rust-lang/rust/pull/121892)
* [use volatile access instead of `#[used]` for `on_tls_callback`](https://github.com/rust-lang/rust/pull/121596)
* [miri: add -Zmiri-track-alloc-accesses to readme and fix its wording](https://github.com/rust-lang/miri/pull/3346)
* [miri: log when we change the active thread, and fix logging for concurrency](https://github.com/rust-lang/miri/pull/3348)
* [miri: print thread name in miri error backtraces; add option to track read/write accesses](https://github.com/rust-lang/miri/pull/3338)
* [miri: tree Borrows diagnostic improvements](https://github.com/rust-lang/miri/pull/3343)
* [miri: windows: support getting the thread name](https://github.com/rust-lang/miri/pull/3345)
* [add ASCII fast-path for `char::is_grapheme_extended`](https://github.com/rust-lang/rust/pull/121138)
* [perf: improve `write_fmt` to handle simple strings](https://github.com/rust-lang/rust/pull/121001)
* [add `display` method to `OsStr`](https://github.com/rust-lang/rust/pull/120051)
* [have `String` use `SliceIndex` impls from `str`](https://github.com/rust-lang/rust/pull/120291)
* [use the OS thread name by default if `THREAD_INFO` has not been initialized](https://github.com/rust-lang/rust/pull/121666)
* [add missing `get_name` for `wasm::thread`](https://github.com/rust-lang/rust/pull/121933)
* [remove `Mutex::unlock` Function](https://github.com/rust-lang/rust/pull/121736)
* [implement unwind safety for Condvar on all platforms](https://github.com/rust-lang/rust/pull/121768)
* [make `ReentrantLock` public](https://github.com/rust-lang/rust/pull/110543)
* [codegen\_gcc: debuginfo: Add support for debuginfo, without scope support](https://github.com/rust-lang/rustc_codegen_gcc/pull/455)
* [codegen\_gcc: switch to the new `set_special_chars_allowed_in_func_names` API](https://github.com/rust-lang/rustc_codegen_gcc/pull/462)
* [cargo add: Fallback to `rustc -v` when no MSRV is set](https://github.com/rust-lang/cargo/pull/13516)
* [cargo toml: Warn on unset Edition](https://github.com/rust-lang/cargo/pull/13505)
* [cargo msrv: Report all incompatible packages, not just a random one](https://github.com/rust-lang/cargo/pull/13514)
* [cargo rustc: Always pass --edition to rustc](https://github.com/rust-lang/cargo/pull/13499)
* [cargo toml: Don't warn on unset Edition if only 2015 is compatible](https://github.com/rust-lang/cargo/pull/13533)
* [cargo: add all unit's children recursively for `doc.extern-map` option](https://github.com/rust-lang/cargo/pull/13481)
* [cargo: add "-Zpublic-dependency" for public-dependency feature](https://github.com/rust-lang/cargo/pull/13340)
* [cargo: silently ignore `cargo::rustc-check-cfg` to avoid MSRV annoyance when stabilizing `-Zcheck-cfg`](https://github.com/rust-lang/cargo/pull/13438)
* [cargo: stabilize global cache data tracking](https://github.com/rust-lang/cargo/pull/13492)
* [rustdoc: Prevent inclusion of whitespace character after `macro_rules` ident](https://github.com/rust-lang/rust/pull/121689)
* [rustfmt: ensure space around binary exprs](https://github.com/rust-lang/rustfmt/pull/6085)
* [clippy: `identity_op`: Fix duplicate diagnostics](https://github.com/rust-lang/rust-clippy/pull/12409)
* [clippy: `let_underscore_untyped`: fix false positive on async function](https://github.com/rust-lang/rust-clippy/pull/12400)
* [clippy: `map_entry`: Check insert expression for map use](https://github.com/rust-lang/rust-clippy/pull/12362)
* [clippy: `misrefactored_assign_op`: fix duplicate diagnostics](https://github.com/rust-lang/rust-clippy/pull/12413)
* [clippy: `redundant_closure_call`: don't lint closure originating from a macro](https://github.com/rust-lang/rust-clippy/pull/12380)
* [clippy: `unnecessary_cast`: avoid breaking precedence](https://github.com/rust-lang/rust-clippy/pull/12365)
* [clippy: add `assigning_clones` lint](https://github.com/rust-lang/rust-clippy/pull/12077)
* [clippy: add `mixed_attributes_style` lint](https://github.com/rust-lang/rust-clippy/pull/12354)
* [clippy: added msrv to threadlocal initializer check](https://github.com/rust-lang/rust-clippy/pull/12405)
* [clippy: check for try blocks in `question_mark` more consistently](https://github.com/rust-lang/rust-clippy/pull/12341)
* [clippy: dedup `std_instead_of_core` by using first segment span for uniqueness](https://github.com/rust-lang/rust-clippy/pull/12406)
* [clippy: don't emit "missing backticks" lint if the element is wrapped in `<code>` HTML tags](https://github.com/rust-lang/rust-clippy/pull/12423)
* [clippy: fix false positive in `threadlocal!` when falling back to `os_local`](https://github.com/rust-lang/rust-clippy/pull/12276)
* [clippy: fix `derive_partial_eq_without_eq` false positive on trait projection](https://github.com/rust-lang/rust-clippy/pull/12393)
* [clippy: fix `nonminimal_bool` lint regression](https://github.com/rust-lang/rust-clippy/pull/12372)
* [clippy: fix `manual_memcpy` wrong indexing for multi dimensional arrays](https://github.com/rust-lang/rust-clippy/pull/12010)
* [clippy: handle plural acronyms in `doc_markdown`](https://github.com/rust-lang/rust-clippy/pull/12419)
* [clippy: improve `is_lint_level` code](https://github.com/rust-lang/rust-clippy/pull/12375)
* [clippy: lower `bstr` version requirement to `1.6.0`](https://github.com/rust-lang/rust-clippy/pull/12363)
* [clippy: pointers cannot be converted to integers at compile time](https://github.com/rust-lang/rust-clippy/pull/12403)
* [rust-analyzer: add hover display for trait assoc items](https://github.com/rust-lang/rust-analyzer/pull/15938)
* [rust-analyzer: add basic support for native debug](https://github.com/rust-lang/rust-analyzer/pull/16719)
* [rust-analyzer: autocomplete constants inside format strings](https://github.com/rust-lang/rust-analyzer/pull/16723)
* [rust-analyzer: don't destructure `struct` with no public fields](https://github.com/rust-lang/rust-analyzer/pull/16752)
* [rust-analyzer: don't highlight related assoc items of super traits](https://github.com/rust-lang/rust-analyzer/pull/16727)
* [rust-analyzer: goto definition for `deref_mut`](https://github.com/rust-lang/rust-analyzer/pull/16696)
* [rust-analyzer: goto definition for `index_mut`](https://github.com/rust-lang/rust-analyzer/pull/16709)
* [rust-analyzer: goto-definition for constants inside range pattern](https://github.com/rust-lang/rust-analyzer/pull/16759)
* [rust-analyzer: ignore generic arguments in intra doc link path resolution](https://github.com/rust-lang/rust-analyzer/pull/16702)
* [rust-analyzer: put style lints behind disabled-by-default config](https://github.com/rust-lang/rust-analyzer/pull/16757)
* [rust-analyzer: fix rust-project.json projects not preferring sysroot rustc](https://github.com/rust-lang/rust-analyzer/pull/16693)
* [rust-analyzer: fix wrong closure kind deduction for closures with predicates](https://github.com/rust-lang/rust-analyzer/pull/16630)
* [futures: parse rhs of `select!` arms using match-arm rules](https://github.com/rust-lang/futures-rs/pull/2832)

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

Rusty Events between 2024-03-06 - 2024-04-03 🦀

### Virtual

* 2024-02-29 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298457901/) | [**Mirror: Berline.rs page**](https://berline.rs/2024/02/29/rust-hack-and-learn.html)
* 2024-02-29 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Surfing the Rusty Wireless Waves with the ESP32-C3 Board**](https://www.meetup.com/charlottesville-rust-meetup/events/298372724/)
* 2024-03-06 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/rust-dublin/)
    * [**An intro to `nom`, parsing made easy for Rustaceans**](https://www.meetup.com/rust-dublin/events/299358988/)
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
* 2024-03-14 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298457903/) | [**Mirror: Berline.rs page**](https://berline.rs/2024/03/14/rust-hack-and-learn.html)
* 2024-03-14 | Virtual (Nürnberg, DE) | [Rust Nüremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945252/)
* 2024-03-21 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/298631832/)
* 2024-03-26 | Virtual + In Person (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**13th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/299223178/) - [Stream](https://www.youtube.com/@bcnrust)

### Europe

* 2024-02-29 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Season start 2024**](https://www.meetup.com/rust-berlin/events/299190389/)
* 2024-02-29 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust Meetup #44 sponsored by Bang & Olufsen**](https://www.meetup.com/copenhagen-rust-community/events/299353844/)
* 2024-03-06 | Zürich, CH | [Rust Zürisee](https://www.meetup.com/rust-zurich/)
    * [**How to (partial) Migration - March Meetup**](https://www.meetup.com/rust-zurich/events/299380190/)
* 2024-03-12 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-03-13 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.com/o/paris-rustaceans-74289178383)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-830340830777)
* 2024-03-14 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/298533419/)
* 2024-03-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/299028814/)
* 2024-03-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Rust Interactive Session**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/299309224/)
* 2024-03-20 | Girona, ES | [Rust Girona](https://www.meetup.com/rust-girona/)
    * [**Introduction to programming Microcontrollers with Rust**](https://www.meetup.com/rust-girona/events/299172343/)
* 2024-03-21 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/de-DE/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #6**](https://www.meetup.com/de-DE/rust-meetup-augsburg/events/299354449/)
* 2024-03-21 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #6: Du RSS et de L'ECS !**](https://www.meetup.com/meetup-group-zgphbyet/events/299295547/)
* 2024-03-26 | Barcelona, ES + Virtual | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**13th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/299223178/)
* 2024-03-26, 2024-03-28 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation 2024**](https://www.rustnationuk.com/)

### North America

* 2024-02-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/297380841/)
* 2024-02-28 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Happy Hour**](https://www.meetup.com/deep-dish-rust/events/299284926/)
* 2024-03-04 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Central Cambridge Rust Lunch**](https://www.meetup.com/bostonrust/events/299261953/)
* 2024-03-07 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299043793/)
* 2024-03-13 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Northeastern Rust Lunch**](https://www.meetup.com/bostonrust/events/299262009/)
* 2024-03-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186823/)
* 2024-03-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/299220136/)
* 2024-03-27 | Hawthorne, CA, US | [Freeform](https://freeform.co/)
    * [**Rust in the Physical World 🦀 Tech Talk Event at Freeform**](https://freeformxrust.rsvpify.com/)

### Oceania

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

> My experience with C++ is that, as I’ve become more of an expert in the language, I’ve become more disillusioned with it. It’s incredibly hard to do things that you should be able to do in software. And, it’s a huge problem for me to constantly be helping other engineers debug the same bugs over and over. It’s always another use after free. I’ve probably debugged 300 of those. \[...\]
>
> In our experience using the Rust ecosystem for almost three years now, I don't think we found a bug in a single Rust crate that we've pulled off the shelf. We found a bug in one of them and that was a Rust crate wrapping a C library and the bug was in the C library. The software quality that you kind of get for free is amazing.

– [Carter Schultz interviewed on the filtra blog](https://filtra.io/rust-amp-feb-24)

Thanks to [George Barwood](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1543) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
