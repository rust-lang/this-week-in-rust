Title: This Week in Rust 584
Number: 584
Date: 2025-01-29
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

This week's crate is [embed\_it](https://github.com/riberk/embed_it), a crate that helps you to embed assets into your binary and generates structs / trait implementations for each file or directory.

Thanks to [Riberk](https://users.rust-lang.org/t/crate-of-the-week/2704/1390) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No calls for testing were issued this week.*

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No calls for testing were issued this week.*

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No calls for testing were issued this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

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

408 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-01-21..2025-01-28

* [support QNX 7.1 with `io-sock`+libstd and QNX 8.0 (`no_std` only)](https://github.com/rust-lang/rust/pull/133631)
* [mark all NuttX targets as tier 3 target and support the standard library](https://github.com/rust-lang/rust/pull/136037)
* [add NuttX support for AArch64 and ARMv7-A targets](https://github.com/rust-lang/rust/pull/135757)
* [add `#[optimize(none)]`](https://github.com/rust-lang/rust/pull/128657)
* [account for mutable borrow in argument suggestion](https://github.com/rust-lang/rust/pull/136032)
* [add a suggestion to cast `target_feature` fn items to fn pointers](https://github.com/rust-lang/rust/pull/136064)
* [add a workaround for parallel rustc crashing when there are delayed bugs](https://github.com/rust-lang/rust/pull/135988)
* [change `collect_and_partition_mono_items` tuple return type to a `struct`](https://github.com/rust-lang/rust/pull/136118)
* [codegen: store ScalarPair via memset when one side is undef and the other side can be memset](https://github.com/rust-lang/rust/pull/135335)
* [compiler: set `target_abi = "ilp32e"` on all riscv32e targets](https://github.com/rust-lang/rust/pull/134358)
* [coverage: prepare for upcoming changes to counter creation](https://github.com/rust-lang/rust/pull/135873)
* [detect missing fields with default values and suggest `..`](https://github.com/rust-lang/rust/pull/135794)
* [do not assume const params are printed after type params](https://github.com/rust-lang/rust/pull/135749)
* [don't ICE in coerce when autoderef fails to structurally normalize non-WF type in new solver](https://github.com/rust-lang/rust/pull/134746)
* [don't drop types with no drop glue when building drops for tailcalls](https://github.com/rust-lang/rust/pull/135976)
* [don't pick `T: FnPtr` nested goals as the leaf goal in diagnostics for new solver](https://github.com/rust-lang/rust/pull/135866)
* [enable `unreachable_pub` lint in `test` and `proc_macro` crates](https://github.com/rust-lang/rust/pull/135366)
* [enable kernel sanitizers for aarch64-unknown-none-softfloat](https://github.com/rust-lang/rust/pull/135905)
* [fix GDB `OsString` provider on Windows](https://github.com/rust-lang/rust/pull/135812)
* [fix ICE: multiple never-pattern arm doesn't have `false_edge_start_block`](https://github.com/rust-lang/rust/pull/135409)
* [forbid usage of `hir Infer` const/ty variants in ambiguous contexts](https://github.com/rust-lang/rust/pull/135272)
* [handle global trait bounds defining assoc types](https://github.com/rust-lang/rust/pull/135766)
* [implement `needs-subprocess` directive, and cleanup a bunch of tests to use `needs-{subprocess,threads}`](https://github.com/rust-lang/rust/pull/135926)
* [improve check-cfg expected names diagnostic](https://github.com/rust-lang/rust/pull/136016)
* [make our `DIFlags` match `LLVMDIFlags` in the LLVM-C API](https://github.com/rust-lang/rust/pull/135156)
* [make the `wasm_c_abi` future compat warning a hard error](https://github.com/rust-lang/rust/pull/133951)
* [point at invalid utf-8 span on user's source code](https://github.com/rust-lang/rust/pull/135557)
* [properly record metavar spans for other expansions other than TT](https://github.com/rust-lang/rust/pull/134478)
* [properly report error when object type param default references self](https://github.com/rust-lang/rust/pull/135971)
* [remove support for the (unstable) `#[start]` attribute](https://github.com/rust-lang/rust/pull/134299)
* [remove usages of `QueryNormalizer` in the compiler](https://github.com/rust-lang/rust/pull/135914)
* [reword resolve errors caused by likely missing crate in dep tree](https://github.com/rust-lang/rust/pull/133154)
* [rework dyn trait lowering to stop being so intertwined with trait alias expansion](https://github.com/rust-lang/rust/pull/133830)
* [shorten linker output even more when `--verbose` is not present](https://github.com/rust-lang/rust/pull/135707)
* [show linker output even if the linker succeeds](https://github.com/rust-lang/rust/pull/119286)
* [simplify `parse_format::Parser::ws` by using `next_if`](https://github.com/rust-lang/rust/pull/135920)
* [skip `if-let-rescope` lint unless requested by migration](https://github.com/rust-lang/rust/pull/132666)
* [skip suggestions in `derive`d code](https://github.com/rust-lang/rust/pull/136027)
* [support wasm inline assembly in `naked_asm!`](https://github.com/rust-lang/rust/pull/135648)
* [tidy Python improvements](https://github.com/rust-lang/rust/pull/135950)
* [uplift `clippy::double_neg` lint as `double_negations`](https://github.com/rust-lang/rust/pull/126604)
* [use `structurally_normalize` instead of manual `normalizes-to` goals in alias relate errors](https://github.com/rust-lang/rust/pull/135816)
* [use identifiers more in diagnostics code](https://github.com/rust-lang/rust/pull/136114)
* [use short type string in E0308 secondary span label](https://github.com/rust-lang/rust/pull/135949)
* [miri: many-seeds: do not use more than 8 threads](https://github.com/rust-lang/miri/pull/4152)
* [miri: pre-intern name when searching module children](https://github.com/rust-lang/miri/pull/4153)
* [implement `ByteStr` and `ByteString` types](https://github.com/rust-lang/rust/pull/135073)
* [implement `VecDeque::pop_front_if` & `VecDeque::pop_back_if`](https://github.com/rust-lang/rust/pull/135890)
* [implement phantom variance markers](https://github.com/rust-lang/rust/pull/135807)
* [windows x86: Change i128 to return via the vector ABI](https://github.com/rust-lang/rust/pull/134290)
* [cargo: config: When merging, replace rather than combining specific configuration keys](https://github.com/rust-lang/cargo/pull/15066)
* [cargo: login: Deprecate CLI token](https://github.com/rust-lang/cargo/pull/15057)
* [cargo: fix `shared_std_dependency_rebuild` running on Windows](https://github.com/rust-lang/cargo/pull/15111)
* [cargo: fix broken links in the Cargo book](https://github.com/rust-lang/cargo/pull/15109)
* [cargo: make --allow-dirty imply --allow-staged](https://github.com/rust-lang/cargo/pull/15013)
* [cargo: print globs when workspace members can't be found](https://github.com/rust-lang/cargo/pull/15093)
* [cargo: remove unused `-C link-arg=-fuse-ld=lld`](https://github.com/rust-lang/cargo/pull/15097)
* [rustdoc: Fix indent of trait items on mobile](https://github.com/rust-lang/rust/pull/135998)
* [rustfmt: fix: `wrap_comments` creating invalid code blocks](https://github.com/rust-lang/rustfmt/pull/6417)
* [clippy: `arithmetic_side_effects`: check adjusted expression types](https://github.com/rust-lang/rust-clippy/pull/14062)
* [clippy: `match_bool`: fix suggestion if guard is present](https://github.com/rust-lang/rust-clippy/pull/14039)
* [clippy: `short_circuit_statement`: handle macros and parenthesis better](https://github.com/rust-lang/rust-clippy/pull/14047)
* [clippy: `unnecessary_semicolon`: do not lint if it may cause borrow errors](https://github.com/rust-lang/rust-clippy/pull/14049)
* [clippy: add necessary adjustments to suggestion to remove redundant `.into_iter()` calls](https://github.com/rust-lang/rust-clippy/pull/14035)
* [clippy: add new lint `doc_overindented_list_items`](https://github.com/rust-lang/rust-clippy/pull/13711)
* [clippy: add new lint `non_std_lazy_statics`](https://github.com/rust-lang/rust-clippy/pull/13770)
* [clippy: correct suggestions in `no_std`](https://github.com/rust-lang/rust-clippy/pull/13999)
* [clippy: don't trigger `needless_late_init` when the first usage is in macro](https://github.com/rust-lang/rust-clippy/pull/14053)
* [clippy: make `unnecessary_map_or` work with ref and `Deref` to `Option`/`Result`](https://github.com/rust-lang/rust-clippy/pull/14024)
* [clippy: new lint `sliced_string_as_bytes`](https://github.com/rust-lang/rust-clippy/pull/14002)
* [clippy: proper applicability for `obfuscated_if_else`](https://github.com/rust-lang/rust-clippy/pull/14061)
* [clippy: suggest using `Vec::extend()` in `same_item_push`](https://github.com/rust-lang/rust-clippy/pull/13987)
* [clippy: trigger `obfuscated_if_else` for `.then(..).unwrap_or(..)`](https://github.com/rust-lang/rust-clippy/pull/14021)
* [rust-analyzer: check cfg when collecting macro defs](https://github.com/rust-lang/rust-analyzer/pull/19014)
* [rust-analyzer: explicitly add buildfiles when constructing ProjectFolders](https://github.com/rust-lang/rust-analyzer/pull/19019)
* [rust-analyzer: implement `arbitrary-self-types`](https://github.com/rust-lang/rust-analyzer/pull/19012)
* [rust-analyzer: implement `default-field-values`](https://github.com/rust-lang/rust-analyzer/pull/19001)
* [rust-analyzer: provide a config to control auto-insertion of `await` and `iter()`](https://github.com/rust-lang/rust-analyzer/pull/18993)
* [rust-analyzer: support safe functions marked with `#[target_feature(..)]`](https://github.com/rust-lang/rust-analyzer/pull/19038)
* [rust-analyzer: don't complete `doc(hidden) enum` variants and use trees](https://github.com/rust-lang/rust-analyzer/pull/19034)
* [rust-analyzer: don't suggest `into_iter().method()` on iterators](https://github.com/rust-lang/rust-analyzer/pull/19050)
* [rust-analyzer: fix `ItemScope` not recording glob imports](https://github.com/rust-lang/rust-analyzer/pull/19016)
* [rust-analyzer: fix a missing standard token in semantic highlighting](https://github.com/rust-lang/rust-analyzer/pull/19045)
* [rust-analyzer: fix flycheck panicking with "once" invocation strategy](https://github.com/rust-lang/rust-analyzer/pull/19017)
* [rust-analyzer: fix flyimport not filtering via stability of import path](https://github.com/rust-lang/rust-analyzer/pull/19028)
* [rust-analyzer: fix syntactic highlighting for renames](https://github.com/rust-lang/rust-analyzer/pull/19047)
* [rust-analyzer: in completion's expand, consider recursion stop condition (when we're not inside a macro call anymore) *after* the recursive call instead of before it](https://github.com/rust-lang/rust-analyzer/pull/19037)
* [rust-analyzer: prevent infinite recursion of bounds formatting](https://github.com/rust-lang/rust-analyzer/pull/19020)
* [rust-analyzer: report calling unsafe fn pointer as unsafe](https://github.com/rust-lang/rust-analyzer/pull/19051)
* [rust-analyzer: sort completion items that skip `await` and `iter()` behind those that don't](https://github.com/rust-lang/rust-analyzer/pull/18988)
* [rust-analyzer: goto `Display::fmt` when invoked on `to_string`](https://github.com/rust-lang/rust-analyzer/pull/18986)
* [rust-analyzer: increase `AUTODEREF_RECURSION_LIMIT` to 20](https://github.com/rust-lang/rust-analyzer/pull/19004)
* [rust-analyzer: keep already computed inlay hint properties instead of late resolving them](https://github.com/rust-lang/rust-analyzer/pull/18991)
* [rust-analyzer: make niches into nices](https://github.com/rust-lang/rust-analyzer/pull/18973)
* [rust-analyzer: only collect implicit visibile use symbols if they have renames](https://github.com/rust-lang/rust-analyzer/pull/19026)
* [rust-analyzer: prioritize formatting thread tasks in `main_loop`](https://github.com/rust-lang/rust-analyzer/pull/19052)
* [rust-analyzer: split out `ExpressionStore` from `Body`](https://github.com/rust-lang/rust-analyzer/pull/19036)
* [rust-analyzer: use `strict_provenance`](https://github.com/rust-lang/rust-analyzer/pull/18909)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *No RFCs entered Final Comment Period this week.*

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [std: print a backtrace on stackoverflow](https://github.com/rust-lang/rust/pull/133170)
* [Stabilize const_slice_flatten](https://github.com/rust-lang/rust/pull/134995)
* [Derive `Copy` and `Hash` for `IntErrorKind`](https://github.com/rust-lang/rust/pull/131923)
* [Tracking Issue for map_many_mut](https://github.com/rust-lang/rust/issues/97601)
* [Tracking Issue for `const_vec_string_slice`](https://github.com/rust-lang/rust/issues/129041)
* [Tracking Issue for `const_mut_cursor`](https://github.com/rust-lang/rust/issues/130801)
* [Stabilize `const_is_char_boundary` and `const_str_split_at`.](https://github.com/rust-lang/rust/pull/134016)
* [Tracking Issue for `NonZero*::count_ones`](https://github.com/rust-lang/rust/issues/120287)
* [Stabilize `const_black_box`](https://github.com/rust-lang/rust/pull/135414)
* [Make cenum_impl_drop_cast a hard error](https://github.com/rust-lang/rust/pull/135964)
* [Tracking Issue for `once_wait`](https://github.com/rust-lang/rust/issues/127527)
* [[rustdoc] Add sans-serif font setting](https://github.com/rust-lang/rust/pull/133636)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Proposals entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2025-01-29 - 2025-02-26 🦀

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

> I have experience in multiple styles of MMA gained from fighting the borrow checker, if that counts.

– [Richard Neumann on rust-users](https://users.rust-lang.org/t/is-it-worth-getting-a-degree-in-rust/124678/2)

Thanks to [Jonas Fassbender](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1654) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
