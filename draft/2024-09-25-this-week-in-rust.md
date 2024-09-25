Title: This Week in Rust 566
Number: 566
Date: 2024-09-25
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
* [Perpetual 0.4.7, Quantile regression support and improved multi-threading](https://github.com/perpetual-ml/perpetual)
* [git-cliff 2.6.0 is released! (a highly customizable changelog generator)](https://git-cliff.org/blog/2.6.0)

* [Fjall 2.0, LSM-based safe Rust key-value storage engine](https://fjall-rs.github.io/post/announcing-fjall-2/)

### Observations/Thoughts

### Rust Walkthroughs

* [video] [Build with Naz : Rust lifetimes](https://www.youtube.com/watch?v=eIJxAEcle7E)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [perpetual](https://github.com/perpetual-ml/perpetual), a self-generalizing gradient boosting implementation.

Thanks to [Mutlu Simsek](https://users.rust-lang.org/t/crate-of-the-week/2704/1348) for the self-suggestion!

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

400 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-09-17..2024-09-24

* [add arm64e-apple-tvos target](https://github.com/rust-lang/rust/pull/130614)
* [parser: recover from `::`: to `:`:](https://github.com/rust-lang/rust/pull/130673)
* [`read_volatile __rust_no_alloc_shim_is_unstable` in `alloc_zeroed`](https://github.com/rust-lang/rust/pull/130497)
* [add `unqualified_local_imports` lint](https://github.com/rust-lang/rust/pull/125645)
* [add recursion limit to FFI safety lint](https://github.com/rust-lang/rust/pull/130598)
* [apply `EarlyOtherwiseBranch` to scalar value](https://github.com/rust-lang/rust/pull/129047)
* [assert that `explicit_super_predicates_of` and `explicit_item_super_predicates` truly only contains bounds for the type itself](https://github.com/rust-lang/rust/pull/130666)
* [bail if there are too many non-region infer vars in the query response](https://github.com/rust-lang/rust/pull/130617)
* [call `module_name_to_str` instead of just unwrapping](https://github.com/rust-lang/rust/pull/130680)
* [change `download-ci-llvm` default from `if-unchanged` to `true`](https://github.com/rust-lang/rust/pull/130529)
* [check params for unsafety in THIR](https://github.com/rust-lang/rust/pull/130531)
* [compiler: accept "improper" ctypes in extern "rust-cold" fn](https://github.com/rust-lang/rust/pull/130667)
* [compiler: factor out `OVERFLOWING_LITERALS` impl](https://github.com/rust-lang/rust/pull/130646)
* [correct outdated object size limit](https://github.com/rust-lang/rust/pull/127546)
* [disallow hidden references to mutable static](https://github.com/rust-lang/rust/pull/124895)
* [do not ICE with incorrect empty suggestion](https://github.com/rust-lang/rust/pull/127988)
* [do not expect infer/bound/placeholder/error in v0 symbol mangling](https://github.com/rust-lang/rust/pull/130485)
* [don't ICE in `opaque_hidden_inferred_bound` lint for RPITIT in trait with no default method body](https://github.com/rust-lang/rust/pull/130440)
* [don't call `extern_crate` when local crate name is the same as a dependency and we have a trait error](https://github.com/rust-lang/rust/pull/130275)
* [don't call `ty::Const::normalize` in error reporting](https://github.com/rust-lang/rust/pull/130712)
* [encode `coroutine_by_move_body_def_id` in crate metadata](https://github.com/rust-lang/rust/pull/130201)
* [ensure that `keyword_ident` lint doesn't trigger on `'r#kw` lifetime](https://github.com/rust-lang/rust/pull/130489)
* [fix `break_last_token`](https://github.com/rust-lang/rust/pull/130551)
* [fix anon const def-creation when macros are involved take 2](https://github.com/rust-lang/rust/pull/130337)
* [fix circular `fn_sig` queries to correct number of args for methods](https://github.com/rust-lang/rust/pull/130496)
* [fix fluent diagnostics](https://github.com/rust-lang/rust/pull/129477)
* [further improve diagnostics for expressions in pattern position](https://github.com/rust-lang/rust/pull/123877)
* [gate `repr(Rust)` correctly on non-ADT items](https://github.com/rust-lang/rust/pull/129422)
* [get rid of niche selection's dependence on fields's order](https://github.com/rust-lang/rust/pull/130508)
* [handle unsized consts with type `str` in v0 symbol mangling](https://github.com/rust-lang/rust/pull/130344)
* [implement Return Type Notation (RTN)'s path form in where clauses](https://github.com/rust-lang/rust/pull/129629)
* [improve handling of raw-idents in check-cfg](https://github.com/rust-lang/rust/pull/130507)
* [normalize consts in writeback when GCE is enabled](https://github.com/rust-lang/rust/pull/130645)
* [relate receiver invariantly in method probe for `Mode::Path`](https://github.com/rust-lang/rust/pull/129073)
* [remove semi-nondeterminism of `DefPathHash` ordering from inliner](https://github.com/rust-lang/rust/pull/130455)
* [replace calls to `ty::Const::{try_}eval` in mir build/pattern analysis](https://github.com/rust-lang/rust/pull/130715)
* [rework `non_local_definitions` lint to only use a syntactic heuristic](https://github.com/rust-lang/rust/pull/127117)
* [support 128-bit atomics on s390x](https://github.com/rust-lang/rust/pull/130558)
* [take more advantage of the `isize::MAX` limit in `Layout`](https://github.com/rust-lang/rust/pull/129845)
* [use `Vec` in `rustc_interface::Config::locale_resources`](https://github.com/rust-lang/rust/pull/129988)
* [win: open dir for sync access in `remove_dir_all`](https://github.com/rust-lang/rust/pull/129934)
* [miri: automatically add/remove labels when github review (requests) are used](https://github.com/rust-lang/miri/pull/3893)
* [miri: refactor fd read/write](https://github.com/rust-lang/miri/pull/3852)
* [add `extern "C-cmse-nonsecure-entry" fn`](https://github.com/rust-lang/rust/pull/127766)
* [don't alloca for unused locals](https://github.com/rust-lang/rust/pull/129283)
* [perf: skip normalizing param env if it is already normalized](https://github.com/rust-lang/rust/pull/130561)
* [begin experimental support for pin reborrowing](https://github.com/rust-lang/rust/pull/130526)
* [`RepeatN`: use MaybeUninit](https://github.com/rust-lang/rust/pull/130145)
* [add `Thread::{into_raw, from_raw}`](https://github.com/rust-lang/rust/pull/97524)
* [add `Vec::as_non_null`](https://github.com/rust-lang/rust/pull/130624)
* [add `new_cyclic_in` for Rc and Arc](https://github.com/rust-lang/rust/pull/129674)
* [add `str.as_str()` for easy Deref to string slices](https://github.com/rust-lang/rust/pull/129550)
* [avoid re-validating UTF-8 in `FromUtf8Error::into_utf8_lossy`](https://github.com/rust-lang/rust/pull/130408)
* [delay uncapping the `max_read_size` in `File::read_to_end`](https://github.com/rust-lang/rust/pull/130670)
* [add `Lazy{Cell,Lock}::get[_mut]` and `force_mut`](https://github.com/rust-lang/rust/pull/130476)
* [make unstable `Result::flatten` a const fn](https://github.com/rust-lang/rust/pull/130692)
* [mark `char::make_ascii_uppercase` and `char::make_ascii_lowercase` as const](https://github.com/rust-lang/rust/pull/130697)
* [mark `u8::make_ascii_uppercase` and `u8::make_ascii_lowercase` as const](https://github.com/rust-lang/rust/pull/130713)
* [pass `fmt::Arguments` by reference to `PanicInfo` and `PanicMessage`](https://github.com/rust-lang/rust/pull/129491)
* [stabilize const `MaybeUninit::as_mut_ptr`](https://github.com/rust-lang/rust/pull/130542)
* [remove uneeded `PartialOrd` bound in `cmp::Ord::clamp`](https://github.com/rust-lang/rust/pull/130481)
* [std: implement the `random` feature (alternative version)](https://github.com/rust-lang/rust/pull/129201)
* [support `char::encode_utf16` in const scenarios](https://github.com/rust-lang/rust/pull/130659)
* [support `char::encode_utf8` in const scenarios](https://github.com/rust-lang/rust/pull/130511)
* [futures: fix issues with `AsyncBufRead::read_line` and `AsyncBufReadExt::lines`](https://github.com/rust-lang/futures-rs/pull/2884)
* [hashbrown: implement Clone and Debug for HashTable's Iter `struct`](https://github.com/rust-lang/hashbrown/pull/541)
* [hashbrown: implement `Debug`, `FusedIterator` and `Iterator::fold` for all `HashTable` iterators](https://github.com/rust-lang/hashbrown/pull/561)
* [hashbrown: re-introduce a way to get the allocation size of a table](https://github.com/rust-lang/hashbrown/pull/553)
* [cargo: complete: Upgrade `clap_complete`](https://github.com/rust-lang/cargo/pull/14573)
* [cargo: complete: Harden `--target` completions](https://github.com/rust-lang/cargo/pull/14564)
* [cargo: resolve: Don't list transitive, incompatible dependencies as available](https://github.com/rust-lang/cargo/pull/14568)
* [cargo: resolve: Improve multi-MSRV workspaces](https://github.com/rust-lang/cargo/pull/14569)
* [cargo: add a `--dry-run` flag to the `install` command](https://github.com/rust-lang/cargo/pull/14280)
* [cargo: add custom completer for `cargo build --example=<TAB>`](https://github.com/rust-lang/cargo/pull/14531)
* [cargo: add custom completer for `cargo help <TAB>`](https://github.com/rust-lang/cargo/pull/14557)
* [cargo: add custom completer for completing benchmark names](https://github.com/rust-lang/cargo/pull/14532)
* [cargo: add custom completer for completing target triple](https://github.com/rust-lang/cargo/pull/14535)
* [cargo: add custom completer for completing test names](https://github.com/rust-lang/cargo/pull/14548)
* [cargo: suggest `cargo info` command in the `cargo search` result](https://github.com/rust-lang/cargo/pull/14537)
* [rustdoc: use the correct span for doctests](https://github.com/rust-lang/rust/pull/130582)
* [clippy: ignore `missing_panics_doc` in const context](https://github.com/rust-lang/rust-clippy/pull/13382)
* [clippy: fix `if_then_some_else_none` sugg missing closure intro](https://github.com/rust-lang/rust-clippy/pull/13409)
* [clippy: generate versions HTML directly](https://github.com/rust-lang/rust-clippy/pull/13414) (nice poem, @xFredNet)
* [clippy: initial impl of `unnecessary_first_then_check`](https://github.com/rust-lang/rust-clippy/pull/13421)
* [clippy: lint comparison to empty slice using `PartialEq` methods](https://github.com/rust-lang/rust-clippy/pull/13432)
* [clippy: unused trait imports (formerly anonymous trait import)](https://github.com/rust-lang/rust-clippy/pull/13322)
* [clippy: use contiguous spans for `empty_line_after_*` suggestion](https://github.com/rust-lang/rust-clippy/pull/13439)
* [rust-analyzer: don't lint names of `#[no_mangle]` extern fns](https://github.com/rust-lang/rust-analyzer/pull/18136)
* [rust-analyzer: add diagnostics for `unsafe_op_in_unsafe_fn`](https://github.com/rust-lang/rust-analyzer/pull/18135)
* [rust-analyzer: implement `expr_2021`](https://github.com/rust-lang/rust-analyzer/pull/18137)
* [rust-analyzer: support the `${concat(...)}` metavariable expression](https://github.com/rust-lang/rust-analyzer/pull/18151)
* [rust-analyzer: always cache macro expansions' root node in Semantics](https://github.com/rust-lang/rust-analyzer/pull/18117)
* [rust-analyzer: don't complete `;` when in closure return expression](https://github.com/rust-lang/rust-analyzer/pull/18132)
* [rust-analyzer: extend `type_variable_table` when modifying index is larger than the table size](https://github.com/rust-lang/rust-analyzer/pull/18139)
* [rust-analyzer: get rid of `$crate` in expansions shown to the user](https://github.com/rust-lang/rust-analyzer/pull/18131)
* [rust-analyzer: handle errors and lints from external macros](https://github.com/rust-lang/rust-analyzer/pull/18128)
* [rust-analyzer: handle lint attributes that are under `#[cfg_attr]`](https://github.com/rust-lang/rust-analyzer/pull/18108)
* [rust-analyzer: remove check that text of `parse_expr_from_str()` matches the produced parsed tree](https://github.com/rust-lang/rust-analyzer/pull/18146)
* [rust-analyzer: support expect in attribute completion and hover](https://github.com/rust-lang/rust-analyzer/pull/18172)
* [rust-analyzer: when checking for forbidden expr kind matches, account for rawness](https://github.com/rust-lang/rust-analyzer/pull/18153)

### Rust Compiler Performance Triage

Not much happened this week. Most changes of note were readily
justified as removing sources of unpredictable/inconsistent behavior
from code-generation.

Triage done by **@pnkfelix**.
Revision range: [170d6cb8..749f80ab](https://perf.rust-lang.org/?start=170d6cb845c8c3f0dcec5cdd4210df9ecf990244&end=749f80ab051aa0b3724b464130440b0e70a975ac&absolute=false&stat=instructions%3Au)

1 Regression, 0 Improvements, 4 Mixed; 1 of them in rollups
28 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/4c714caeb2bb6907dd527fd56da5cd6d79b30818/triage/2024-09-23.md)

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

Rusty Events between 2024-09-25 - 2024-10-23 🦀

### Virtual
* 2024-09-18 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Cells**](https://www.meetup.com/vancouver-rust/events/298631736/)
* 2024-09-18 - 2024-09-20 | Hybrid - Virtual and In-Person (Vienna, AT) | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2024**](https://lpc.events/event/18/sessions/186/)
* 2024-09-19 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897973/)
* 2024-09-19 | Virtual and In-Person (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**September Meetup**](https://www.meetup.com/join-srug/events/303067835/)
* 2024-09-24 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/301585670/)
* 2024-09-25 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Reflections on RustConf 2024**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/303444953/)
* 2024-09-26 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633269/)
* 2024-09-26 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Rusty secure communication on embedded devices**](https://www.meetup.com/charlottesville-rust-meetup/events/303159380/)
* 2024-10-02 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 8 - Asynchronous Programming**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/301314544/)
* 2024-10-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031649/)
* 2024-10-02 | Virtual (Vancouver, BC, CA) | [Vancouver Postgres](https://www.meetup.com/vancouver-postgres/)
    * [**Leveraging a PL/RUST extension to protect sensitive data in PostgreSQL**](https://www.meetup.com/vancouver-postgres/events/302160672/)
* 2024-10-03 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897992/)
* 2024-10-08 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346983/)
* 2024-10-10 | Virtual (Barcelona, ES) | [BcnRust](https://bcnrust.github.io) + [Codurance](https://www.codurance.com/)
    * [**15th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/303443195/)
* 2024-10-10 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633270/)
* 2024-10-10 - 2024-10-11 | Virtual and In-Person (Vienna, AT) | [Euro Rust](eurorust)
    * [**Euro Rust 2024**](https://eurorust.eu/)
* 2024-10-16 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631737/)
    
### Africa
* 2024-10-05 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2024-10-09 | Subang Jaya / Kuala Lumpur, Selangor, MY | [Rust Malaysia](https://rust-malaysia.github.io/meetup/)
    * [**Rust Malaysia Meetup - Traits and How to Read Trait (October 2024)**](https://docs.google.com/forms/d/e/1FAIpQLScNS5IWmnzTTJAOw-RIxdj4_BWbxB5NVmAVO30XHr_viMbLqQ/viewform)

### Asia
* 2024-09-21 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**September 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/september-2024-rustacean-meetup/)
* 2024-09-24 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup/)
    * [**Building Tangible Things With Rust**](https://www.meetup.com/tokyo-rust-meetup/events/303402114/)

### Europe
* 2024-09-18 | Moravia, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/)
    * [**Rust Moravia Meetup (September 2024)**](https://www.meetup.com/rust-moravia/events/301360936)
* 2024-09-18 | Vienna, AT + Virtual | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2024 (Sep 18-20)**](https://lpc.events/event/18/sessions/186/)
* 2024-09-20 | Hamburg, DE | [Code.Talks](https://codetalks.com/)
    * [**Code.Talks Conference 2024: "Journey to Fullstack Mobile Game Development in Rust" (Stephan Dilly)**](https://codetalks.com/program?talkId=2290)
* 2024-09-21 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Ferris' Fika Forum #5**](https://www.meetup.com/Stockholm-Rust/events/303210419)
* 2024-09-23 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake #6**](https://www.meetup.com/bratislava-rust-meetup-group/events/302916594/)
* 2024-09-24 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust Meetup #70**](https://www.meetup.com/rust-paris/events/303212378/)
* 2024-09-24 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust)
    * [**Rust meetup #70**](https://www.meetup.com/Stockholm-Rust/events/303210419)
* 2024-09-24 | Zagreb, HR | [impl Zagreb for Rust](https://www.meetup.com/zagreb-rust-meetup/)
    * [**Rust Drinkup 2024/09**](https://www.meetup.com/zagreb-rust-meetup/events/303484490/)
* 2024-09-26 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at Talk Night at Alexandra Instituttet**](https://www.meetup.com/rust-aarhus/events/301522991/)
* 2024-09-26 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Rust Meetup #9: From loops to folds**](https://www.meetup.com/rust-meetup-augsburg/events/302437593)
* 2024-09-26 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299421380/)
* 2024-09-26 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Prague (September 2024)**](https://www.meetup.com/rust-prague/events/303346494/)
* 2024-09-27 | Mannheim, DE | [Hackerstolz e.V.](https://www.hackerstolz.de/en/)
    * [**Hackerstolz Stammtisch Rhein-Neckar**](https://www.hackerstolz.de/en/)
* 2024-10-02 | Oxford, UK | [Oxfrod Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Rust for Rustaceans Book Club: Chapter 11: Foreign Function Interfaces**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/303445033/)
* 2024-10-02 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @Funnel**](https://www.meetup.com/Stockholm-Rust/events/303213095)
* 2022-10-03 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820273/)
* 2024-10-03 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154268/)
* 2024-10-09 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtygcnbmb/)
* 2024-10-10 - 2024-10-11 | Virtual and In-Person (Vienna, AT) | [Euro Rust](eurorust)
    * [**Euro Rust 2024**](https://eurorust.eu/)
* 2024-10-15 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425056/)

### North America
* 2024-09-18 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Cells**](https://www.meetup.com/vancouver-rust/events/298631736/)
* 2024-09-19 | Virtual and In-Person (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**September Meetup**](https://www.meetup.com/join-srug/events/303067835/)
* 2024-09-21 | Longview, TX, US | [Longview Code and Coffee](https://www.meetup.com/longview-code-and-coffee/)
    * [**Longview Code and Coffee**](https://www.meetup.com/longview-code-and-coffee/events/301976355/)
* 2024-09-24 | Detroit, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ferndale**](https://www.meetup.com/detroitrust/events/zfcbntygcmbgc/)
* 2024-09-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/302274449/)
* 2024-09-26 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Develpers : Community Presentations**](https://www.meetup.com/music-city-rust-developers/events/301420118/)
* 2024-09-27 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Beacon Hill Rust Lunch, Sep 27**](https://www.meetup.com/bostonrust/events/302498761/)
* 2024-10-03 | Boston, MA, US | [SquiggleConf](https://2024.squiggleconf.com/)
    * [**SquiggleConf 2024: "Oxc: Pluggable Next-Gen Tooling At Rust Speed", Don Isaac**](https://2024.squiggleconf.com/schedule)
* 2024-10-03 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Iterators in Rust**](https://www.meetup.com/stl-rust/events/302371456/)
* 2024-10-04 | Mexico City, DF, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Multi threading y Async en Rust pt1. Prerequisitos**](https://www.meetup.com/rust-mx/events/303480458/)
* 2024-10-08 | Detroit, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcnblb/)
* 2024-10-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346970/)
* 2024-10-16 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631737/)

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

> New users feel like iteration times are so slow and it takes forever to get going with Rust. But if there's a library available, I feel like I'm roughly as productive with Rust as I am with Ruby, if not more, when I think about the whole amount of work I'm doing. I haven't really figured out how to talk about that without sounding purely like a zealot, but yeah, I feel like Rust is actually very, very productive, even though many people don't see it that way initially.

– [Steve Klabnik at Oxidize Conference](https://youtu.be/q8qn0dyT3xc?t=2784)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1611) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
