Title: This Week in Rust 520
Number: 520
Date: 2023-11-08
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

* [rust-libp2p v0.53 has been released](https://github.com/libp2p/rust-libp2p/releases/tag/libp2p-v0.53.0)
* [Zellij 0.39.0 released](https://zellij.dev/news/session-resurrection-ui-components/)

### Observations/Thoughts
* [Writing Rust Bindings for My Python App](https://dhruv-ahuja.github.io/posts/writing-rust-bindings/)

### Rust Walkthroughs

* [Using Modern Linux Sockets](https://devork.be/blog/2023/11/modern-linux-sockets/)

### Research

### Miscellaneous
* [Migrating SecureDrop’s PGP backend from GnuPG to Sequoia](https://securedrop.org/news/migrating-securedrops-pgp-backend-from-gnupg-to-sequoia/)

## Crate of the Week

This week's crate is [floem](https://github.com/lapce/floem), a native Rust UI library with fine-grained reactivity.

Despite receiving no suggestions, llogiq is reasonably pleased with his choice.

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

366 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-10-30..2023-11-06

* [`dropck_outlives` check whether generator witness `needs_drop`](https://github.com/rust-lang/rust/pull/117134)
* [account for `ref` and `mut` in the wrong place for pattern ident renaming](https://github.com/rust-lang/rust/pull/117289)
* [add a stable MIR visitor](https://github.com/rust-lang/rust/pull/117417)
* [add all RPITITs when augmenting param-env with GAT bounds in `check_type_bounds`](https://github.com/rust-lang/rust/pull/117131)
* [add diagnostic items for a few of core's builtin macros](https://github.com/rust-lang/rust/pull/117596)
* [add support for pre-unix-epoch file dates on Apple platforms](https://github.com/rust-lang/rust/pull/117451)
* [add the `Span` of the `move` keyword to the HIR](https://github.com/rust-lang/rust/pull/117585)
* [also consider TAIT to be uncomputable if the MIR body is tainted](https://github.com/rust-lang/rust/pull/117416)
* [avoid the path trimming ICE lint in error reporting](https://github.com/rust-lang/rust/pull/117373)
* [avoid unnecessary comparison in `partition_equal`](https://github.com/rust-lang/rust/pull/117179)
* [check binders with bound vars for global bounds that don't hold](https://github.com/rust-lang/rust/pull/117637)
* [consts: remove dead code around `i1` constant values](https://github.com/rust-lang/rust/pull/117554)
* [coverage: replace impossible `coverage::Error` with assertions](https://github.com/rust-lang/rust/pull/117421)
* [derive `TyEncodable`/`TyDecodable` in `rustc_type_ir`](https://github.com/rust-lang/rust/pull/117578)
* [detect misparsed binop caused by missing semi](https://github.com/rust-lang/rust/pull/117292)
* [detect object safety errors when assoc type is missing](https://github.com/rust-lang/rust/pull/116405)
* [do not ICE on constant evaluation failure in GVN](https://github.com/rust-lang/rust/pull/117438)
* [do not assert in `op_to_const`](https://github.com/rust-lang/rust/pull/117441)
* [don't check for alias bounds in liveness when aliases have escaping bound vars](https://github.com/rust-lang/rust/pull/117466)
* [don't emit delayed good-path bugs on panic](https://github.com/rust-lang/rust/pull/117397)
* [don't pass `-stdlib=libc++` when building C files on macOS](https://github.com/rust-lang/rust/pull/116017)
* [enable cross-crate-inlining when MIR inlining is enabled](https://github.com/rust-lang/rust/pull/117363)
* [enable parallel rustc front end in nightly builds](https://github.com/rust-lang/rust/pull/117435)
* [fallback for `construct_generic_bound_failure`](https://github.com/rust-lang/rust/pull/117570)
* [fix excessive initialization and reads beyond EOF in `io::copy(_, Vec<u8>)` specialization](https://github.com/rust-lang/rust/pull/117576)
* [fix incorrect trait bound restriction suggestion](https://github.com/rust-lang/rust/pull/117505)
* [fix order of implementations in the "implementations on foreign types" section](https://github.com/rust-lang/rust/pull/117521)
* [guarantee representation of None in NPO](https://github.com/rust-lang/rust/pull/115333)
* [guarantee that `char` has the same size and alignment as `u32`](https://github.com/rust-lang/rust/pull/116894)
* [hint optimizer about try-reserved capacity](https://github.com/rust-lang/rust/pull/117503)
* [inline and remove `create_session`](https://github.com/rust-lang/rust/pull/117475)
* [make sure that predicates with unmentioned bound vars are still considered global in the old solver](https://github.com/rust-lang/rust/pull/117589)
* [make the randomize feature of `rustc_abi` additive](https://github.com/rust-lang/rust/pull/117603)
* [match usize/isize exhaustively with half-open ranges](https://github.com/rust-lang/rust/pull/116692)
* [prepopulate opaque ty storage before using it](https://github.com/rust-lang/rust/pull/117439)
* [pretty print `Fn` traits in `rustc_on_unimplemented`](https://github.com/rust-lang/rust/pull/116439)
* [recover from missing param list in function definitions](https://github.com/rust-lang/rust/pull/117298)
* [refactor: move suggestion functions from demand to suggestions](https://github.com/rust-lang/rust/pull/117401)
* [remove obsolete support for linking unwinder on Android](https://github.com/rust-lang/rust/pull/117504)
* [remove support for alias `-Z symbol-mangling-version`](https://github.com/rust-lang/rust/pull/117509)
* [remove support for compiler plugins](https://github.com/rust-lang/rust/pull/116412)
* [replace switch to unreachable by assume statements](https://github.com/rust-lang/rust/pull/113970)
* [set `max_atomic_width` for riscv32*-esp-espidf to 32](https://github.com/rust-lang/rust/pull/117307)
* [turn `const_caller_location` from a query to a hook](https://github.com/rust-lang/rust/pull/117388)
* [use `FxIndexSet` in the symbol interner](https://github.com/rust-lang/rust/pull/117508)
* [use derivative for `Clone`/`PartialOrd`/`Ord`/`Hash` in `rustc_type_ir`](https://github.com/rust-lang/rust/pull/117407)
* [use global cache when computing proof trees](https://github.com/rust-lang/rust/pull/117394)
* [use the correct span when emitting the `env!` result](https://github.com/rust-lang/rust/pull/117592)
* [warn users who set `non_exhaustive_omitted_patterns` lint level on a match arm](https://github.com/rust-lang/rust/pull/117094)
* [when encountering unclosed delimiters during lexing, check for diff markers](https://github.com/rust-lang/rust/pull/116712)
* [enable src/math for all UEFI targets](https://github.com/rust-lang/compiler-builtins/pull/553)
* [intrinsics macro: fix non-weak aeabi generation](https://github.com/rust-lang/compiler-builtins/pull/552)
* [this enables math module for riscv32 targets](https://github.com/rust-lang/compiler-builtins/pull/554)
* [stabilize `const_maybe_uninit_zeroed` and `const_mem_zeroed`](https://github.com/rust-lang/rust/pull/116218)
* [stabilize `file_set_times`](https://github.com/rust-lang/rust/pull/117422)
* [fix `switch_stdout_to` on Windows7](https://github.com/rust-lang/rust/pull/117386)
* [add `track_caller` to `transmute_copy`](https://github.com/rust-lang/rust/pull/117510)
* [delegate `<Box<E> as Error>::provide` to `<E as Error>::provide`](https://github.com/rust-lang/rust/pull/117434)
* [support `enum` variants in `offset_of!`](https://github.com/rust-lang/rust/pull/114208)
* [feature gate enums in `offset_of`](https://github.com/rust-lang/rust/pull/117537)
* [override `Waker::clone_from` to avoid cloning `Waker`s unnecessarily](https://github.com/rust-lang/rust/pull/96979)
* [codegen\_gcc: fix vector compilation error](https://github.com/rust-lang/rustc_codegen_gcc/pull/368)
* [cargo: `feat(trim-paths)`: set env `CARGO_TRIM_PATHS` for build scripts](https://github.com/rust-lang/cargo/pull/12900)
* [cargo toml: Pull out the schema](https://github.com/rust-lang/cargo/pull/12911)
* [cargo: fix an unhelpful panic message](https://github.com/rust-lang/cargo/pull/12923)
* [cargo: implement `-Ztrim-paths`](https://github.com/rust-lang/cargo/pull/12625) (RFC [#3127](https://rust-lang.github.io/rfcs/3127-trim-paths.html))
* [cargo: merge `trim-paths` from different profiles](https://github.com/rust-lang/cargo/pull/12908)
* [rustdoc: accept less invalid Rust](https://github.com/rust-lang/rust/pull/117450)
* [rustfmt: fixes comma added to comment in where-clause](https://github.com/rust-lang/rustfmt/pull/5954)
* [clippy: `unused_enumerate_index`: don't ICE on empty tuples](https://github.com/rust-lang/rust-clippy/pull/11756)
* [clippy: add `unused_enumerate_index` lint](https://github.com/rust-lang/rust-clippy/pull/10404)
* [clippy: fix `dbg_macro` semi span calculation](https://github.com/rust-lang/rust-clippy/pull/11743)
* [clippy: fix `enum_variant_names` depending lint depending on order](https://github.com/rust-lang/rust-clippy/pull/11498)
* [clippy: fix `get_first` false negative for VecDeque](https://github.com/rust-lang/rust-clippy/pull/11744)
* [clippy: new lint: `unnecessary_fallible_conversions`](https://github.com/rust-lang/rust-clippy/pull/11669)
* [rust-analyzer: add `generate_mut_trait_impl` assist](https://github.com/rust-lang/rust-analyzer/pull/15832)
* [rust-analyzer: import trait with alias](https://github.com/rust-lang/rust-analyzer/pull/15788)
* [rust-analyzer: skip checking token tree count for include! macro call](https://github.com/rust-lang/rust-analyzer/pull/15819)
* [rust-analyzer: fix docs path for derive macros](https://github.com/rust-lang/rust-analyzer/pull/15834)
* [rust-analyzer: vSCode metadata. category:formatters](https://github.com/rust-lang/rust-analyzer/pull/15827)

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

Rusty Events between 2023-11-08 - 2023-12-06 🦀

### Virtual

* 2023-11-01 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**ECS with Bevy Game Engine**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583207/)
* 2023-11-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyfcpbcb)
* 2023-11-02 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296661148/)
* 2023-11-07 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679790/) | [**Mirror**](https://berline.rs/)
* 2023-11-07 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/296827010/)
* 2023-11-09 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732666/)
* 2023-11-14 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/fvdtgtyfcpbsb/)
* 2023-11-15 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Building Our Own Locks (Atomics & Locks Chapter 9)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296582223/)
* 2023-11-15 | Virtual (Richmond, VA, US) | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2023 (Nov 13-16)**](https://lpc.events/event/17/sessions/170/)
* 2023-11-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Nightly Night: impl Trait in Type Aliases**](https://www.meetup.com/vancouver-rust/events/296600976/)
* 2023-11-16 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296833657/)
* 2023-11-21 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679794/)
* 2023-11-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/296807537/)
* 2023-11-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtyfcpblc/)

### Europe

* 2023-11-01 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/events)
    * [**Web-applications with axum: Hello CRUD!**](https://www.meetup.com/rustcologne/events/296540949/)
* 2023-11-07 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake**](https://www.meetup.com/bratislava-rust-meetup-group/events/296809100/)
* 2023-11-07 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Rust Aarhus - Rust and Talk beginners edition**](https://www.meetup.com/rust-aarhus/events/296223647/)
* 2023-11-07 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #7**](https://www.meetup.com/rust-lyon/events/296853019/)
* 2023-11-09 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/)
    * [**11th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/296567395)
* 2023-11-09 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-732823744547/)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-732823744547)
* 2023-11-09 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/296083417/)
* 2023-11-21 | Augsburg, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**GPU processing in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504264/)
* 2023-11-23 | Biel/Bienne, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Talks Bern @ Biel: Embedded Edition**](https://www.meetup.com/rust-bern/events/296556498/)

### North America

* 2023-11-01 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston Common Rust Lunch**](https://www.meetup.com/bostonrust/events/296223910/)
* 2023-11-02 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/297062678/)
* 2023-11-08 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**Let's make a Discord bot!**](https://www.meetup.com/boulder-rust-meetup/events/296437292/)
* 2023-11-14 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Mixer: Share, Show, & Tell! 🦀**](https://www.meetup.com/rust-nyc/events/296895126/)
* 2023-11-14 | Seattle, WA, US | [Cap Hill Rust Coding/Hacking/Learning](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/seattle-rust-user-group/events/296540653)
* 2023-11-15 | Richmond, VA, US + Virtual | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2023 (Nov 13-16)**](https://lpc.events/event/17/sessions/170/)
* 2023-11-16 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/297062689/)
* 2023-11-16 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Python loves Rust!**](https://www.meetup.com/music-city-rust-developers/events/296916567/)
* 2023-11-16 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/295483924)
* 2023-11-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/296917625/)
* 2023-11-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcpbdc/)

### Oceania

* 2023-11-06 | Perth, WA, AU | [Rust Perth Meetup Group](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Embracing Lifetimes: A Journey to Safe and Efficient Code**](https://www.meetup.com/perth-rust-meetup-group/events/296963595)
* 2023-11-21 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/296819540/)
* 2023-11-28 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**November Meetup**](https://www.meetup.com/rust-canberra/events/296391733/)

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

> For Binder to continue to meet Android's needs, we need better ways to manage (and reduce!) complexity without increasing the risk.
>
> The biggest change is obviously the choice of programming language. We decided to use Rust because it directly addresses a number of the challenges within Binder that we have faced during the last years.

– [Alice Rhyl on the Linux Kernel Mailing List](https://lore.kernel.org/rust-for-linux/20231101-rust-binder-v1-0-08ba9197f637@google.com/)

Thanks to [Vincent de Phily](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1475) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
