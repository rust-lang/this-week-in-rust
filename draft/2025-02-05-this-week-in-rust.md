Title: This Week in Rust 585
Number: 585
Date: 2025-02-05
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://x.com/ThisWeekInRust) on X (formerly Twitter) or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

Want TWIR in your inbox? [Subscribe here](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

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

This week's crate is [ratzilla](https://github.com/orhun/ratzilla), a library for building terminal-themed web applications with Rust and WebAssembly.

Thanks to [Orhun Parmaksız](https://users.rust-lang.org/t/crate-of-the-week/2704/1397) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Calls for Testing go here, use this format:
    * [<RFC Topic>](<RFC URL>)
        * [Tracking Issue](<Tracking Issue URL>)
        * [Testing steps](<Testing Steps URL>)
-->
<!-- or if there are no new or updated RFCs this week, use: -->
<!-- * *No New or Updated RFCs were created this week.* -->
<!-- Remember to remove the `call-for-testing` label from the RFC so that the maintainer can signal for testers again, if desired. -->

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

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

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

425 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-01-28..2025-02-04

* [`#[optimize(none)]` implies `#[inline(never)]`](https://github.com/rust-lang/rust/pull/136358)
* [-Znext-solver: "normalize" signature before checking it mentions self in `deduce_closure_signature`](https://github.com/rust-lang/rust/pull/135892)
* [`rustc_allowed_through_unstable_modules`: require deprecation message](https://github.com/rust-lang/rust/pull/136434)
* [`rustc_hir_analysis` cleanups](https://github.com/rust-lang/rust/pull/136281)
* [ABI-required target features: warn when they are missing in base CPU](https://github.com/rust-lang/rust/pull/136147)
* [add `AsyncFn*` to `core` prelude](https://github.com/rust-lang/rust/pull/135852)
* [add `kl` and `widekl` target features, and the feature gate](https://github.com/rust-lang/rust/pull/134814)
* [add constraint graph to polonius MIR dump](https://github.com/rust-lang/rust/pull/136278)
* [add link attribute for Enzyme's LLVMRust FFI](https://github.com/rust-lang/rust/pull/136374)
* [add mermaid graphs of NLL regions and SCCs to polonius MIR dump](https://github.com/rust-lang/rust/pull/136104)
* [add missing allocator safety in alloc crate](https://github.com/rust-lang/rust/pull/135805)
* [allow transmuting generic pattern types to and from their base](https://github.com/rust-lang/rust/pull/136179)
* [cast global variables to default address space](https://github.com/rust-lang/rust/pull/135026)
* [deduplicate operand creation between scalars, non-scalars and string patterns](https://github.com/rust-lang/rust/pull/136121)
* [delay a bug when indexing unsized slices](https://github.com/rust-lang/rust/pull/136325)
* [diagnostics: fix borrowck suggestions for if/while let conditionals](https://github.com/rust-lang/rust/pull/136402)
* [disable `overflow_delimited_expr` in edition 2024](https://github.com/rust-lang/rust/pull/136312)
* [do not consider child bound assumptions for rigid alias](https://github.com/rust-lang/rust/pull/135902)
* [enable `unreachable_pub` lint in `alloc`](https://github.com/rust-lang/rust/pull/135367)
* [explain why we retroactively change a static initializer to have a different type](https://github.com/rust-lang/rust/pull/136426)
* [explicitly choose x86 softfloat/hardfloat ABI](https://github.com/rust-lang/rust/pull/136146)
* [filter out RPITITs when suggesting unconstrained assoc type on too many generics](https://github.com/rust-lang/rust/pull/136313)
* [fix autodiff compile time regression](https://github.com/rust-lang/rust/pull/136413)
* [fix broken release notes id](https://github.com/rust-lang/rust/pull/136266)
* [fix deduplication mismatches in vtables leading to upcasting unsoundness](https://github.com/rust-lang/rust/pull/135318)
* [ignore NLL boring locals in polonius diagnostics](https://github.com/rust-lang/rust/pull/136299)
* [implement MIR const trait stability checks](https://github.com/rust-lang/rust/pull/136055)
* [implement MIR lowering for unsafe binders](https://github.com/rust-lang/rust/pull/130514)
* [implement `int_from_ascii`](https://github.com/rust-lang/rust/pull/134824)
* [insert null checks for pointer dereferences when debug assertions are enabled](https://github.com/rust-lang/rust/pull/134424)
* [interpret: `is_alloc_live`: check global allocs last](https://github.com/rust-lang/rust/pull/136166)
* [introduce a wrapper for "typed valtrees" and properly check the type before extracting the value](https://github.com/rust-lang/rust/pull/136180)
* [lower index bounds checking to `PtrMetadata`, this time with the right fake borrow semantics 😸](https://github.com/rust-lang/rust/pull/135748)
* [make comma separated lists of anything easier to make for errors](https://github.com/rust-lang/rust/pull/136368)
* [make crate AST mutation accessible for driver callback](https://github.com/rust-lang/rust/pull/136214)
* [manually walk into WF obligations in `BestObligation` proof tree visitor](https://github.com/rust-lang/rust/pull/135900)
* [merge `PatKind::Path` into `PatKind::Expr`](https://github.com/rust-lang/rust/pull/134248)
* [miri: improve error when `offset_from` preconditions are violated](https://github.com/rust-lang/rust/pull/136438)
* [miri: make float min/max non-deterministic](https://github.com/rust-lang/rust/pull/136348)
* [miri: optimize zeroed alloc](https://github.com/rust-lang/rust/pull/136035)
* [notes on types/traits used for in-memory query caching](https://github.com/rust-lang/rust/pull/136484)
* [omit argument names from function pointers that do not have argument names](https://github.com/rust-lang/rust/pull/136411)
* [omit unused args warnings for intrinsics without body](https://github.com/rust-lang/rust/pull/135840)
* [overhaul `rustc_middle::util`](https://github.com/rust-lang/rust/pull/136336)
* [pass spans to `perform_locally_in_new_solver`](https://github.com/rust-lang/rust/pull/136066)
* [properly check that array length is valid type during built-in unsizing in index](https://github.com/rust-lang/rust/pull/136205)
* [reject unsound toggling of Arm atomics-32 target feature](https://github.com/rust-lang/rust/pull/136170)
* [shorten error message for callable with wrong return type](https://github.com/rust-lang/rust/pull/136414)
* [suggest considering casting fn item as fn pointer in more cases](https://github.com/rust-lang/rust/pull/133382)
* [support `clobber_abi` in BPF inline assembly](https://github.com/rust-lang/rust/pull/136194)
* [target modifiers (special marked options) are recorded in metainfo](https://github.com/rust-lang/rust/pull/133138)
* [target option to require explicit cpu](https://github.com/rust-lang/rust/pull/135030)
* [test validity of pattern types](https://github.com/rust-lang/rust/pull/136145)
* [use proper type when applying deref adjustment in const](https://github.com/rust-lang/rust/pull/136314)
* [use the type-level constant value `ty::Value` where needed](https://github.com/rust-lang/rust/pull/136430)
* [when encountering unexpected closure return type, point at return type/expression](https://github.com/rust-lang/rust/pull/132156)
* [`miri_get_backtrace`: stop supporting the v0 protocol](https://github.com/rust-lang/miri/pull/4172)
* [miri: added a helper to dedup target OS checks](https://github.com/rust-lang/miri/pull/4160)
* [miri: check fixed args number for variadic function](https://github.com/rust-lang/miri/pull/4122)
* [miri: files: make read/write take callback to store result](https://github.com/rust-lang/miri/pull/4174)
* [miri: increase thread limit for many-seeds mode](https://github.com/rust-lang/miri/pull/4168)
* [miri: set `st_fstype` of stat on Solaris and Illumos OSes](https://github.com/rust-lang/miri/pull/4159)
* [miri: shim Apple's futex primitives](https://github.com/rust-lang/miri/pull/4142)
* [miri: use `deref_pointer_as` instead of `deref_pointer`](https://github.com/rust-lang/miri/pull/4140)
* [stabilize `const_black_box`](https://github.com/rust-lang/rust/pull/135414)
* [stabilize `once_wait`](https://github.com/rust-lang/rust/pull/136360)
* [optimize `slice::ptr_rotate` for small rotates](https://github.com/rust-lang/rust/pull/135847)
* [implement all mix/max functions in a (hopefully) more optimization amendable way](https://github.com/rust-lang/rust/pull/136307)
* [fix off-by-one error causing `slice::sort` to abort the program](https://github.com/rust-lang/rust/pull/136163)
* [don't build out of line atomics support code for uefi](https://github.com/rust-lang/compiler-builtins/pull/752)
* [uefi: implement path](https://github.com/rust-lang/rust/pull/135475)
* [cargo: conditionally mark the `test` cfg as a well known cfg](https://github.com/rust-lang/cargo/pull/15007)
* [cargo: don't suggest `cargo login` when using incompatible credental providers](https://github.com/rust-lang/cargo/pull/15124)
* [apply LTO config to rustdoc](https://github.com/rust-lang/rust/pull/135832)
* [rustdoc: add `--output-format=doctest` command-line flag](https://github.com/rust-lang/rust/pull/134531)
* [rustdoc: add sans-serif font setting](https://github.com/rust-lang/rust/pull/133636)
* [rustdoc: always use a channel when linking to doc.rust-lang.org](https://github.com/rust-lang/rust/pull/134807)
* [bindgen: `process_comment`: Use last defined callback](https://github.com/rust-lang/rust-bindgen/pull/3103)
* [bindgen: use `link_name` for dynamic library loading](https://github.com/rust-lang/rust-bindgen/pull/3101)
* [clippy: `needless_option_take`: add autofix](https://github.com/rust-lang/rust-clippy/pull/14042)
* [clippy: add `manual_slice_fill` lint](https://github.com/rust-lang/rust-clippy/pull/14082)
* [clippy: autofix for `cmp_null`](https://github.com/rust-lang/rust-clippy/pull/14122)
* [clippy: autofix for `redundant_else` lint](https://github.com/rust-lang/rust-clippy/pull/13936)
* [clippy: do not remove semicolon if it changes the block type](https://github.com/rust-lang/rust-clippy/pull/14103)
* [clippy: new lint for `and_then` when returning Option or Result](https://github.com/rust-lang/rust-clippy/pull/14051)
* [clippy: fix escaping problem in `write_literal` and `print_literal` lint suggestion](https://github.com/rust-lang/rust-clippy/pull/13990)
* [clippy: include generic arguments when suggesting a closure η-reduction](https://github.com/rust-lang/rust-clippy/pull/14105)
* [clippy: move `format_push_string` and `format_collect` to pedantic](https://github.com/rust-lang/rust-clippy/pull/13894)
* [clippy: new `manual_option_as_slice` lint](https://github.com/rust-lang/rust-clippy/pull/13901)
* [clippy: new lint: `precedence_bits`, with recent additions to precedence](https://github.com/rust-lang/rust-clippy/pull/14115)
* [rust-analyzer: disable `Receiver` based autoderef temporarily](https://github.com/rust-lang/rust-analyzer/pull/19061)
* [rust-analyzer: ensure `completion_item_hash` serializes items uniquely](https://github.com/rust-lang/rust-analyzer/pull/19072)
* [rust-analyzer: fix scip indexing of module names](https://github.com/rust-lang/rust-analyzer/pull/19062)
* [rust-analyzer: fix some mir eval/lowerings](https://github.com/rust-lang/rust-analyzer/pull/19086)
* [rust-analyzer: properly handle CRLF line endings in the syntax tree view](https://github.com/rust-lang/rust-analyzer/pull/19056)
* [rust-analyzer: try to infer array type from slice pattern](https://github.com/rust-lang/rust-analyzer/pull/19066)
* [rust-analyzer: remove mutable syntax tree shenanigans from adjustment hints](https://github.com/rust-lang/rust-analyzer/pull/19070)
* [rust-analyzer: show status bar in output](https://github.com/rust-lang/rust-analyzer/pull/19057)

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

#### Tracking Issues & PRs
<!-- Tracking Issues which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: -->
<!-- * *No Tracking Issues or PRs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
<!-- New or updated RFCs go here, use this format: * [new|updated] [Topic](URL) -->
<!-- or if there are no new or updated RFCs this week, use: -->
<!-- * *No New or Updated RFCs were created this week.* -->
<!-- * [new|updated] []() -->

## Upcoming Events

Rusty Events between 2025-02-05 - 2025-03-05 🦀

### Virtual
* 2025-01-30 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/299468340)
* 2025-01-30 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Quantum Computers Can’t Rust-Proof This!**](https://www.meetup.com/charlottesville-rust-meetup/events/305391474)
* 2025-01-30 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Are We Embedded Yet? - Implementing tiny HTTP server on a microcontroller**](https://www.meetup.com/code-mavens/events/305382647)
* 2025-01-31 | Virtual (Delhi, IN) | [Hackathon Raptors Association](https://www.meetup.com/hackathon-raptors-association/)
    * [**Blazingly Fast Rust Hackathon**](https://www.meetup.com/hackathon-raptors-association/events/305435372/)
* 2025-01-31 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/305560416/)
* 2025-02-01 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2025-02-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304216)
* 2025-02-04 | Virtual (London, GB) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: Rust Nation UK Talks**](https://www.meetup.com/women-in-rust/events/305647334)
* 2025-02-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031658)
* 2025-02-06 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820280/)
* 2025-02-07 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntyhcdbkb/)
* 2025-02-11 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Meet Elusion: New DataFrame Library powered by Rust 🦀 with Borivoj Grujicic**](https://www.meetup.com/code-mavens/events/305513416)
* 2025-02-13 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/299468342)
* 2025-02-14 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/305815307)
* 2025-02-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Pointer Provenance**](https://www.meetup.com/vancouver-rust/events/304051783)
* 2025-02-20 | Hybrid (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**February, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/join-srug/events/305658424)
* 2025-02-21 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntyhcdbcc)
* 2025-02-25 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/305361428)
* 2025-02-25 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: The complicated world of Strings in Rust**](https://www.meetup.com/women-in-rust/events/305716182)
* 2025-02-25 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful—Everett Pompeii presents Bencher 🐰**](https://www.meetup.com/rustdc/events/305170682)

### Asia
* 2025-02-24 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**In person Rust February 2025 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/305570131)

### Europe
* 2025-01-30 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #11: Hypermedia-driven development in Rust**](https://rust-augsburg.github.io/meetup/Meetup_11.html)
* 2025-01-30 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299421383)
* 2025-01-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust meetup #54 sponsored by Google**](https://www.meetup.com/copenhagen-rust-community/events/305453880)
* 2025-02-01 | Brussels, BE | [FOSDEM 2025](https://fosdem.org/2025/)
    * [**FOSDEM Rust Devroom**](https://fosdem.org/2025/schedule/track/rust/)
* 2025-02-01 | Helsinki, FI | [Finland Rust-lang Group](https://www.meetup.com/finland-rust-meetup/events/)
    * [**February Meetup**](https://www.meetup.com/finland-rust-meetup/events/305666104)
* 2025-02-01 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Technikmuseum Sinsheim**](https://www.meetup.com/rust-noris/events/305361544)
* 2025-02-05 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123401)
* 2025-02-06 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #7**](https://www.meetup.com/rust-gdansk/events/305742562)
* 2025-02-12 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305045444)
* 2025-02-14 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/305791536)
* 2025-02-18 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Introduction to Context-Generic Programming in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/303729528)
* 2025-02-19 - 2025-02-20 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation UK 2025**](https://www.rustnationuk.com/)
* 2025-02-20 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #1 @Puzzle ITC**](https://www.meetup.com/rust-bern/events/305597994)
* 2025-02-21 | London, UK | [Rust Global: London 2025](https://rustfoundation.org/event/rust-global-london-2025/)
    * [**Rust Global: London 2025**](https://rustfoundation.org/event/rust-global-london-2025/)
* 2025-02-22 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #9**](https://www.meetup.com/stockholm-rust/events/305848723)

### North America
* 2025-01-31 | Detroit, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/events/)
    * [**Rust Community Meetup - Ann Arbor**](https://www.meetup.com/detroitrust/events/305847640)
* 2025-02-03 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Central Cambridge Rust Lunch, Feb 3**](https://www.meetup.com/bostonrust/events/305804837)
* 2025-02-06 | Mountain view, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/305517476)
* 2025-02-06 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Async, the Future of Futures**](https://www.meetup.com/stl-rust/events/304959018)
* 2025-02-11 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/events/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/305720765)
* 2025-02-14 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Downtown Rust Lunch, Feb 14**](https://www.meetup.com/bostonrust/events/305804954)
* 2025-02-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638261)
* 2025-02-20 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust Game Development Series 2: Basics of Game Development**](https://www.meetup.com/music-city-rust-developers/events/304333047)
* 2025-02-20 | Redmond, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**February, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/join-srug/events/305658424)
* 2025-02-21 | México City, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Rust y ciencia de datos**](https://www.meetup.com/rust-mx/events/305793010)
* 2025-02-22 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Somerville Union Square Rust Lunch, Feb 22**](https://www.meetup.com/bostonrust/events/305805059)
* 2025-02-26 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcdbjc)

### Oceania
* 2025-02-04 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/events/)
    * [**Rust AKL: How We Learn Rust**](https://www.meetup.com/rust-akl/events/305583693)
* 2025-02-04 | Sydney, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/events/)
    * [**2025 🦀 Kickstart ✨ Talks**](https://www.meetup.com/rust-sydney/events/305816610)

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

> If your rust code compiles and you don't use "unsafe", that is a pretty good certification.

– [Richard Gould about Rust certifications on rust-users](https://users.rust-lang.org/t/recognized-rust-certification/124906/11)

Thanks to [ZiCog](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1657) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
