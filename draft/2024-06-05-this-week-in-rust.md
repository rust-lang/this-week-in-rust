Title: This Week in Rust 550
Number: 550
Date: 2024-06-05
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://x.com/ThisWeekInRust) on X(formerly Twitter) or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

This week's crate is [layoutparser-ort](https://docs.rs/layoutparser-ort), a simplified port of LayoutParser for ML-based document layout element detection.

Despite there being no suggestions, llogiq is reasonably happy with his choice. Are you?

[No matter what your answer is, please submit your suggestions and votes for next week][submit_crate]!

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

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

308 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-05-28..2024-06-04

* [`-Znext-solver`: eagerly normalize when adding goals](https://github.com/rust-lang/rust/pull/125343)
* [`fn_arg_sanity_check`: fix panic message](https://github.com/rust-lang/rust/pull/125695)
* [add `--print=check-cfg` to get the expected configs](https://github.com/rust-lang/rust/pull/124320)
* [add `-Zfixed-x18`](https://github.com/rust-lang/rust/pull/124655)
* [also InstSimplify `&raw*`](https://github.com/rust-lang/rust/pull/125796)
* [also resolve the type of constants, even if we already turned it into an error constant](https://github.com/rust-lang/rust/pull/125807)
* [avoid unwrap diag.code directly in `note_and_explain_type_err`](https://github.com/rust-lang/rust/pull/125774)
* [check index `value <= 0xFFFF_FF00`](https://github.com/rust-lang/rust/pull/125821)
* [coverage: avoid overflow when the MC/DC condition limit is exceeded](https://github.com/rust-lang/rust/pull/125700)
* [coverage: optionally instrument the RHS of lazy logical operators](https://github.com/rust-lang/rust/pull/125756)
* [coverage: rename MC/DC `conditions_num` to `num_conditions`](https://github.com/rust-lang/rust/pull/125754)
* [create const block DefIds in typeck instead of ast lowering](https://github.com/rust-lang/rust/pull/124650)
* [do not equate `Const`'s ty in `super_combine_const`](https://github.com/rust-lang/rust/pull/125671)
* [do not suggest unresolvable builder methods](https://github.com/rust-lang/rust/pull/125397)
* [a small diagnostic improvement for `dropping_copy_types`](https://github.com/rust-lang/rust/pull/125433)
* [don't recompute `tail` in `lower_stmts`](https://github.com/rust-lang/rust/pull/125790)
* [don't suggest turning non-char-literal exprs of ty `char` into string literals](https://github.com/rust-lang/rust/pull/125640)
* [enable DestinationPropagation by default](https://github.com/rust-lang/rust/pull/115105)
* [fold item bounds before proving them in `check_type_bounds` in new solver](https://github.com/rust-lang/rust/pull/125786)
* [implement `needs_async_drop` in rustc and optimize async drop glue](https://github.com/rust-lang/rust/pull/124662)
* [improve diagnostic output of `non_local_definitions` lint](https://github.com/rust-lang/rust/pull/125089)
* [make `ProofTreeBuilder` actually generic over `Interner`](https://github.com/rust-lang/rust/pull/125598)
* [make `body_owned_by` return the `Body` instead of just the `BodyId`](https://github.com/rust-lang/rust/pull/125711)
* [make `repr(packed)` vectors work with SIMD intrinsics](https://github.com/rust-lang/rust/pull/125311)
* [make lint: `lint_dropping_references lint_forgetting_copy_types lint_forgetting_references` give suggestion if possible](https://github.com/rust-lang/rust/pull/125531)
* [omit `non-needs_drop drop_in_place` in vtables](https://github.com/rust-lang/rust/pull/122662)
* [opt-in to `FulfillmentError` generation to avoid doing extra work in the new solver](https://github.com/rust-lang/rust/pull/125864)
* [reintroduce name resolution check for trying to access locals from an inline const](https://github.com/rust-lang/rust/pull/125705)
* [reject `CVarArgs` in `parse_ty_for_where_clause`](https://github.com/rust-lang/rust/pull/125863)
* [show files produced by `--emit foo` in json artifact notifications](https://github.com/rust-lang/rust/pull/122597)
* [silence some resolve errors when there have been glob import errors](https://github.com/rust-lang/rust/pull/125381)
* [stop using `translate_args` in the new solver](https://github.com/rust-lang/rust/pull/125776)
* [support mdBook preprocessors for TRPL in rustbook](https://github.com/rust-lang/rust/pull/125408)
* [test codegen for `repr(packed,simd)` → `repr(simd)`](https://github.com/rust-lang/rust/pull/125904)
* [tweak relations to no longer rely on `TypeTrace`](https://github.com/rust-lang/rust/pull/125664)
* [unroll first iteration of `checked_ilog` loop](https://github.com/rust-lang/rust/pull/124294)
* [uplift `{Closure,Coroutine,CoroutineClosure}Args` and friends to `rustc_type_ir`](https://github.com/rust-lang/rust/pull/125775)
* [use parenthetical notation for `Fn` traits](https://github.com/rust-lang/rust/pull/125778)
* [add some more specific checks to the MIR validator](https://github.com/rust-lang/rust/pull/125851)
* [miri: avoid making a full copy of all new allocations](https://github.com/rust-lang/rust/pull/125633)
* [miri: fix "local crate" detection](https://github.com/rust-lang/miri/pull/3644)
* [don't inhibit random field reordering on `repr(packed(1))`](https://github.com/rust-lang/rust/pull/125360)
* [avoid checking the edition as much as possible](https://github.com/rust-lang/rust/pull/125828)
* [increase vtable layout size](https://github.com/rust-lang/rust/pull/123572)
* [stabilise `IpvNAddr::`{`BITS`, `to_bits`, `from_bits`} (`ip_bits`)](https://github.com/rust-lang/rust/pull/125551)
* [stabilize `custom_code_classes_in_docs` feature](https://github.com/rust-lang/rust/pull/124577)
* [stablize `const_binary_heap_constructor`](https://github.com/rust-lang/rust/pull/125211)
* [make `std::env::`{`set_var`, `remove_var`} unsafe in edition 2024](https://github.com/rust-lang/rust/pull/124636)
* [implement feature `integer_sign_cast`](https://github.com/rust-lang/rust/pull/125884)
* [NVPTX: avoid `PassMode::Direct` for args in C abi](https://github.com/rust-lang/rust/pull/117671)
* [genericize `ptr::from_raw_parts`](https://github.com/rust-lang/rust/pull/125701)
* [`std::pal::unix::thread` fetching min stack size on netbsd](https://github.com/rust-lang/rust/pull/125577)
* [add an intrinsic for `ptr::metadata`](https://github.com/rust-lang/rust/pull/124251)
* [change `f32::midpoint` to upcast to f64](https://github.com/rust-lang/rust/pull/121062)
* [rustc-hash: replace hash with faster and better finalized hash](https://github.com/rust-lang/rustc-hash/pull/37)
* [cargo test: Auto-redact elapsed time](https://github.com/rust-lang/cargo/pull/13973)
* [cargo add: Avoid escaping double-quotes by using string literals](https://github.com/rust-lang/cargo/pull/14006)
* [cargo config: Ensure `--config net.git-fetch-with-cli=true` is respected](https://github.com/rust-lang/cargo/pull/13992)
* [cargo new: Dont say were adding to a workspace when a regular package is in root](https://github.com/rust-lang/cargo/pull/13987)
* [cargo toml: Ensure targets are in a deterministic order](https://github.com/rust-lang/cargo/pull/13989)
* [cargo vendor: Ensure sort happens for vendor](https://github.com/rust-lang/cargo/pull/14004)
* [cargo: allows the default git/gitoxide configuration to be obtained from the ENV and config](https://github.com/rust-lang/cargo/pull/13687)
* [cargo: adjust custom err from cert-check due to libgit2 1.8 change](https://github.com/rust-lang/cargo/pull/13970)
* [cargo: skip deserialization of unrelated fields with overlapping name](https://github.com/rust-lang/cargo/pull/14000)
* [clippy: `many_single_char_names`: deduplicate diagnostics](https://github.com/rust-lang/rust-clippy/pull/12859)
* [clippy: add `needless_character_iteration` lint](https://github.com/rust-lang/rust-clippy/pull/12815)
* [clippy: deprecate `maybe_misused_cfg` and `mismatched_target_os`](https://github.com/rust-lang/rust-clippy/pull/12875)
* [clippy: disable `indexing_slicing` for custom `Index` impls](https://github.com/rust-lang/rust-clippy/pull/12488)
* [clippy: fix `redundant_closure` suggesting incorrect code with `F: Fn()`](https://github.com/rust-lang/rust-clippy/pull/12865)
* [clippy: let `non_canonical_impls` skip proc marco](https://github.com/rust-lang/rust-clippy/pull/12857)
* [clippy: ignore array from `deref_addrof` lint](https://github.com/rust-lang/rust-clippy/pull/12864)
* [clippy: make `str_to_string` machine-applicable](https://github.com/rust-lang/rust-clippy/pull/12871)
* [rust-analyzer: add `Function::fn_ptr_type(…)` for obtaining name-erased function type](https://github.com/rust-lang/rust-analyzer/pull/17312)
* [rust-analyzer: don't mark `#[rustc_deprecated_safe_2024]` functions as unsafe](https://github.com/rust-lang/rust-analyzer/pull/17329)
* [rust-analyzer: enable completions within derive helper attributes](https://github.com/rust-lang/rust-analyzer/pull/17328)
* [rust-analyzer: fix container search failing for tokens originating within derive attributes](https://github.com/rust-lang/rust-analyzer/pull/17326)
* [rust-analyzer: fix diagnostics clearing when flychecks run per-workspace](https://github.com/rust-lang/rust-analyzer/pull/17302)
* [rust-analyzer: only generate snippets for `extract_expressions_from_format_string` if snippets are supported](https://github.com/rust-lang/rust-analyzer/pull/17333)
* [rustfmt: collapse nested if detected by clippy](https://github.com/rust-lang/rustfmt/pull/6169)
* [rustfmt: rustfmt should not remove inner attributes from inline const blocks](https://github.com/rust-lang/rustfmt/pull/6173)
* [rustfmt: rust rewrite `check_diff` (Skeleton)](https://github.com/rust-lang/rustfmt/pull/6166)
* [rustfmt: use `with_capacity` in `rewrite_path`](https://github.com/rust-lang/rustfmt/pull/6174)

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

Rusty Events between 2024-06-05 - 2024-07-03 🦀

### Virtual

* 2024-05-29 | Virtual | [Training 4 Programmers LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Enums, Structs, and Traits - Essential Building Blocks of Rust Programming**](https://www.eventbrite.com/e/enums-structs-and-traits-essential-building-blocks-of-rust-programming-tickets-904696681127)
* 2024-05-30 | Virtual + In Person (Barcelona, ES) | [Mainmatter](https://mainmatter.com/) & [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**Rust for the web, Barcelona 2024**](https://www.meetup.com/es-ES/bcnrust/events/300765894/)
* 2024-05-30 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298542326/)
* 2024-06-04 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Lunch & Learn: A Creative Thinker's Programming Language**](https://www.meetup.com/women-in-rust/events/300918713/)
* 2024-06-04 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300191681/)
* 2024-06-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047896/)
* 2024-06-06 | Virtual | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Rust Maven Workshop: Your first contribution to an Open Source Rust project**](https://www.meetup.com/code-mavens/events/301156302/)
* 2024-06-06 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477702/)
* 2024-06-09 | Virtual | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Rust Maven Workshop: GitHub pages for Rust developers (English)**](https://www.meetup.com/code-mavens/events/301215326/)
* 2024-06-11 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341709/)
* 2024-06-13 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897800/)
* 2024-06-13 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945258/)
* 2024-06-16 | Virtual | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Workshop: Web development in Rust using Rocket (English)**](https://www.meetup.com/code-mavens/events/301294669/)
* 2024-06-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346963/)
* 2024-06-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631733/)
* 2024-06-20 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477705/)
* 2024-06-25 | Virtual (Dallas, TX, US)| [Dallas Rust User Group](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtygcjbhc/)

### Africa

* 2024-06-01 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Europe

* 2024-05-28 - 2024-05-30 | Berlin, DE | [Oxidize](https://oxidizeconf.com/)
    * [**Oxidize Conf 2024**](https://oxidizeconf.com/)
* 2024-05-30 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Developer Meetup @ Avalor AI**](https://www.meetup.com/rust-amsterdam-group/events/301065548/)
* 2024-05-30 | Barcelona, ES | [Mainmatter](https://mainmatter.com/) & [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**Rust for the web, Barcelona 2024**](https://www.meetup.com/es-ES/bcnrust/events/300765894/)
* 2024-05-30 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299288963/)
* 2024-05-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #47 sponsored by Microsoft!**](https://www.meetup.com/copenhagen-rust-community/events/300458222/)
* 2024-05-30 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/300453310/)
* 2024-05-30 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/)
    * [**Rust Vienna Meetup - May - Rust Backend 101**](https://www.meetup.com/rust-vienna/events/301162548/)
* 2024-06-05 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn June 2024**](https://www.meetup.com/rust-meetup-hamburg/events/299235215/)
* 2025-06-06 | Vilnius, LT | [Rust Vilnius](https://www.meetup.com/rust-in-vilnius/)
    * [**Enjoy our second Rust and ZIG event**](https://www.meetup.com/rust-in-vilnius/events/301012097/)
* 2024-06-19 - 2024-06-24 | Zürich, CH | [RustFest Zürich](https://rustfest.ch/)
    * [**RustFest Zürich 2024**](https://rustfest.ch/)
* 2024-06-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at Trifork**](https://www.meetup.com/rust-aarhus/events/300865116/)

### North America

* 2024-05-30 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/300775547/)
* 2024-05-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston Common Rust Lunch, May 31**](https://www.meetup.com/bostonrust/events/300116786/)
* 2024-06-08 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Porter Square Rust Lunch, Jun 8**](https://www.meetup.com/bostonrust/events/300116799/)
* 2024-06-13 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/)
    * [**Monthly Meetup: Topic TBD!**](https://www.meetup.com/spokane-rust/events/300020010/)
* 2024-06-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186953/)
* 2024-06-20 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/299509396/)

### Oceania

* 2024-06-25 | Canberra, ACt, AU | [Canberra Rust User Group (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/300749371/)

### South America

* 2024-06-06 | Buenos Aires, AR | [Rust en Español | Rust Argentina](https://www.meetup.com/rust-argentina/)
    * [**Juntada de Junio**](https://www.meetup.com/rust-argentina/events/299740249)

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

> Every PR is Special™

– [Hieyou Xu describing being on t-compiler review rotation](https://jieyouxu.github.io/blog/review-rotation/)

Sadly, there was no suggestion, so llogiq came up with something hopefully suitable.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
