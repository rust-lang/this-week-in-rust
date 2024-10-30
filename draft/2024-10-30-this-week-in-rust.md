Title: This Week in Rust 571
Number: 571
Date: 2024-10-30
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

This week's crate is [tower-http-client](https://github.com/alekseysidorov/tower-http-client), a library of middlewares and various utilities for HTTP-clients.

Thanks to [Aleksey Sidorov](https://users.rust-lang.org/t/crate-of-the-week/2704/1366) for the self-suggestion!

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

447 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-10-22..2024-10-29

* [add wasm32v1-none target](https://github.com/rust-lang/rust/pull/131487)
* [AIX: use /dev/urandom for random implementation](https://github.com/rust-lang/rust/pull/132048)
* [`rustc_target`: Add pauth-lr aarch64 target feature](https://github.com/rust-lang/rust/pull/131900)
* [add a note for `?` on a `impl Future<Output = Result<..>>` in sync function](https://github.com/rust-lang/rust/pull/131549)
* [add support for `~const` item bounds](https://github.com/rust-lang/rust/pull/132118)
* [consider param-env candidates even if they have errors](https://github.com/rust-lang/rust/pull/132084)
* [const stability checks v2](https://github.com/rust-lang/rust/pull/131349)
* [coverage: consolidate creation of covmap/covfun records](https://github.com/rust-lang/rust/pull/132124)
* [coverage: don't rely on the custom traversal to find enclosing loops](https://github.com/rust-lang/rust/pull/132091)
* [coverage: emit LLVM intrinsics using the normal helper method](https://github.com/rust-lang/rust/pull/132125)
* [coverage: pass coverage mappings to LLVM as separate structs](https://github.com/rust-lang/rust/pull/131956)
* [deeply normalize `TypeTrace` when reporting type error in new solver](https://github.com/rust-lang/rust/pull/131756)
* [deny calls to non-`#[const_trait]` methods in MIR constck](https://github.com/rust-lang/rust/pull/132169)
* [do not remove `.cargo` directory](https://github.com/rust-lang/rust/pull/132054)
* [don't stage-off to previous compiler when CI rustc is available](https://github.com/rust-lang/rust/pull/132006)
* [emit future-incompatibility lint when calling/declaring functions with vectors that require missing target feature](https://github.com/rust-lang/rust/pull/127731)
* [enable LSX feature for LoongArch Linux targets](https://github.com/rust-lang/rust/pull/132140)
* [error on alignments greater than `isize::MAX`](https://github.com/rust-lang/rust/pull/131633)
* [expand: stop using artificial `ast::Item` for macros loaded from metadata](https://github.com/rust-lang/rust/pull/132192)
* [fixup Windows verbatim paths when used with the `include!` macro](https://github.com/rust-lang/rust/pull/125205)
* [hashStable for `rustc_feature::Features`: stop hashing compile-time constant](https://github.com/rust-lang/rust/pull/132076)
* [lint against getting pointers from immediately dropped temporaries](https://github.com/rust-lang/rust/pull/128985)
* [move `cmp_in_dominator_order` out of graph dominator computation](https://github.com/rust-lang/rust/pull/132022)
* [pass constness with span into `lower_poly_trait_ref`](https://github.com/rust-lang/rust/pull/132227)
* [prevent overflowing `enum` cast from ICEing](https://github.com/rust-lang/rust/pull/131909)
* [refactor change detection for rustdoc and download-rustc](https://github.com/rust-lang/rust/pull/131043)
* [replace an FTP link in comments with an equivalent HTTPS link](https://github.com/rust-lang/rust/pull/132096)
* [replace some LLVMRust wrappers with calls to the LLVM C API](https://github.com/rust-lang/rust/pull/132167)
* [represent `hir::TraitBoundModifiers` as distinct parts in HIR](https://github.com/rust-lang/rust/pull/131982)
* [represent trait constness as a distinct predicate](https://github.com/rust-lang/rust/pull/131985)
* [round negative signed integer towards zero in `iN::midpoint`](https://github.com/rust-lang/rust/pull/132191)
* [simplify force-recompile logic for "library"](https://github.com/rust-lang/rust/pull/132215)
* [simplify param handling in `resolve_bound_vars`](https://github.com/rust-lang/rust/pull/132043)
* [taking a raw ref (`&raw (const|mut)`) of a deref of pointer (`*ptr`) is always safe](https://github.com/rust-lang/rust/pull/129248)
* [use `Enabled{Lang,Lib}Feature` instead of n-tuples](https://github.com/rust-lang/rust/pull/132114)
* [validate args are correct for `UnevaluatedConst`, `ExistentialTraitRef`/`ExistentialProjection`](https://github.com/rust-lang/rust/pull/131049)
* [x86 target features: make pclmulqdq imply sse2](https://github.com/rust-lang/rust/pull/132174)
* [x86-32 float return for 'Rust' ABI: treat all float types consistently](https://github.com/rust-lang/rust/pull/131871)
* [miri: add option for generating coverage reports](https://github.com/rust-lang/miri/pull/3954)
* [miri: android: added syscall support](https://github.com/rust-lang/miri/pull/3992)
* [miri: clear `eval_libc` errors from unix shims](https://github.com/rust-lang/miri/pull/3984)
* [miri: consistently use io error handlers](https://github.com/rust-lang/miri/pull/3990)
* [miri: fix error returned from `readdir_r` when isolation is enabled, and uses of `raw_os_error`](https://github.com/rust-lang/miri/pull/3995)
* [miri: implement LLVM x86 vpclmulqdq intrinsics](https://github.com/rust-lang/miri/pull/3987)
* [miri: indicate more explicitly where we close host file/dir handles](https://github.com/rust-lang/miri/pull/3993)
* [(Big performance change) Do not run lints that cannot emit](https://github.com/rust-lang/rust/pull/125116)
* [optimize `Rc<T>::default`](https://github.com/rust-lang/rust/pull/132031)
* [specialize `read_exact` and `read_buf_exact` for `VecDeque`](https://github.com/rust-lang/rust/pull/132039)
* [stabilize `isqrt` feature](https://github.com/rust-lang/rust/pull/131391)
* [stabilize shorter-tail-lifetimes](https://github.com/rust-lang/rust/pull/131983)
* [support `char::is_digit` in const contexts](https://github.com/rust-lang/rust/pull/132242)
* [remove the `Arc rt::init` allocation for thread info](https://github.com/rust-lang/rust/pull/123550)
* [provide a default impl for `Pattern::as_utf8_pattern`](https://github.com/rust-lang/rust/pull/132113)
* [vectorized `SliceContains`](https://github.com/rust-lang/rust/pull/130991)
* [avoid using imports in `thread_local_inner!` in static](https://github.com/rust-lang/rust/pull/132101)
* [better default capacity for `str::replace`](https://github.com/rust-lang/rust/pull/131929)
* [musl: use `posix_spawn` if a directory change was requested](https://github.com/rust-lang/rust/pull/131851)
* [cargo resolver: Make room for v3 resolver](https://github.com/rust-lang/cargo/pull/14725)
* [cargo complete: Include descriptions in zsh](https://github.com/rust-lang/cargo/pull/14726)
* [cargo env: remove unnecessary clones](https://github.com/rust-lang/cargo/pull/14730)
* [cargo: fingerprint: avoid unnecessary fopen calls](https://github.com/rust-lang/cargo/pull/14728)
* [cargo: added unstable-schema generation for Cargo.toml](https://github.com/rust-lang/cargo/pull/14683)
* [cargo: deprecate `cargo verify-project`](https://github.com/rust-lang/cargo/pull/14736)
* [cargo fix: add source replacement info when no matching package found](https://github.com/rust-lang/cargo/pull/14715)
* [cargo fix: trace `config [env]` table in dep-info](https://github.com/rust-lang/cargo/pull/14701)
* [cargo test: add fixes in the sat resolver](https://github.com/rust-lang/cargo/pull/14707)
* [rustdoc: Do not consider nested functions as main function even if named `main` in doctests](https://github.com/rust-lang/rust/pull/132105)
* [rustdoc: extend `fake_variadic` to "wrapped" tuples](https://github.com/rust-lang/rust/pull/132115)
* [rustdoc: hash assets at rustdoc build time](https://github.com/rust-lang/rust/pull/131951)
* [allow type-based search on foreign functions](https://github.com/rust-lang/rust/pull/132123)
* [clippy: `borrow_deref_ref`: do not trigger on `&raw` references](https://github.com/rust-lang/rust-clippy/pull/13600)
* [clippy: don't trigger `const_is_empty` for inline const assertions](https://github.com/rust-lang/rust-clippy/pull/13558)
* [clippy: fire `large_const_arrays` for computed array lengths](https://github.com/rust-lang/rust-clippy/pull/13620)
* [clippy: fix incorrect suggestion for `!(a >= b) as i32 == c`](https://github.com/rust-lang/rust-clippy/pull/13338)
* [clippy: fix not working lint anchor (generation and filtering)](https://github.com/rust-lang/rust-clippy/pull/13588)
* [clippy: remove unnecessary `filter_map` usages](https://github.com/rust-lang/rust-clippy/pull/13548)
* [clippy: stop linting `unused_io_amount` in io traits](https://github.com/rust-lang/rust-clippy/pull/13605)
* [rust-analyzer: add text edits to more inlay hints](https://github.com/rust-lang/rust-analyzer/pull/18376)
* [rust-analyzer: implement diagnostics pull model](https://github.com/rust-lang/rust-analyzer/pull/18404)
* [rust-analyzer: render docs from aliased type when type has no docs](https://github.com/rust-lang/rust-analyzer/pull/18349)
* [rust-analyzer: resolve range patterns to their structs](https://github.com/rust-lang/rust-analyzer/pull/18370)
* [rust-analyzer: split `macro-error` diagnostic so users can ignore only parts of it](https://github.com/rust-lang/rust-analyzer/pull/18418)
* [rust-analyzer: support `cfg(true)` and `cfg(false)`](https://github.com/rust-lang/rust-analyzer/pull/18420)
* [rust-analyzer: fix diagnostic enable config being ignored](https://github.com/rust-lang/rust-analyzer/pull/18399)
* [rust-analyzer: fix dyn incompatible hint message](https://github.com/rust-lang/rust-analyzer/pull/18379)
* [rust-analyzer: fix formatting on welcome page, read only paths setting example](https://github.com/rust-lang/rust-analyzer/pull/18407)
* [rust-analyzer: add missing cfg flags for `core` crate](https://github.com/rust-lang/rust-analyzer/pull/18395)
* [rust-analyzer: allow public re-export of `extern crate` import](https://github.com/rust-lang/rust-analyzer/pull/18413)
* [rust-analyzer: correctly handle `#""` in edition `<2024`](https://github.com/rust-lang/rust-analyzer/pull/18417)
* [rust-analyzer: don't compute diagnostics for non local files](https://github.com/rust-lang/rust-analyzer/pull/18408)
* [rust-analyzer: fix checking for `false labelDetailsSupport` value](https://github.com/rust-lang/rust-analyzer/pull/18388)
* [rust-analyzer: fix incorrect parsing of use bounds](https://github.com/rust-lang/rust-analyzer/pull/18371)
* [rust-analyzer: handle missing time offsets gracefully](https://github.com/rust-lang/rust-analyzer/pull/18386)
* [rust-analyzer: implement mixed site hygiene](https://github.com/rust-lang/rust-analyzer/pull/18264)
* [rust-analyzer: nail destructuring assignment once and for all](https://github.com/rust-lang/rust-analyzer/pull/18254)
* [rust-analyzer: prevent public re-export of private item](https://github.com/rust-lang/rust-analyzer/pull/18390)
* [rust-analyzer: properly resolve prelude paths inside modules inside blocks](https://github.com/rust-lang/rust-analyzer/pull/18422)
* [rust-analyzer: put leading `|` in patterns under `OrPat`](https://github.com/rust-lang/rust-analyzer/pull/18419)
* [rust-analyzer: turn "Remove `dbg!`" into a quick fix for better prioritization](https://github.com/rust-lang/rust-analyzer/pull/18415)
* [rust-analyzer: move text-edit into ide-db](https://github.com/rust-lang/rust-analyzer/pull/18421)
* [rust-analyzer: only construct a resolver in macro descension when needed](https://github.com/rust-lang/rust-analyzer/pull/18409)
* [rust-analyzer: swap query call order in `file_item_tree_query`](https://github.com/rust-lang/rust-analyzer/pull/18392)

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

Rusty Events between 2024-10-30 - 2024-11-27 🦀

### Virtual
* 2024-10-24 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Part 4 of 4 - Hackathon Showcase: Final Projects and Presentations**](https://www.meetup.com/women-in-rust/events/303213835/)
* 2024-10-24 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633271/)
* 2024-10-25 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304100127/)
* 2024-10-26 | Virtual (Gdansk, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-trojmiasto/events/303550643/)
* 2024-10-29 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/301585671/)
* 2024-10-31 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898048/)
* 2024-10-31 | Virtual (Nürnberg, DE) | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820274/)
* 2024-11-01 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcpbcb/)
* 2024-11-02 | Virtual( Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2024-11-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031651/)
* 2024-11-07 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633272/)
* 2024-11-08 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304099245/)
* 2024-11-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346985/)
* 2024-11-14 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898070/)
* 2024-11-14 | Virtual and In-Person (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**November Meetup**](https://www.meetup.com/join-srug/events/304166747/)
* 2024-11-15 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcpbtb/)
* 2024-11-19 | Virtual (Los Angeles, CA, US) | [DevTalk LA](https://www.meetup.com/lajugstudygroup/)
    * [**Discussion - Topic: Rust for UI**](https://www.meetup.com/lajugstudygroup/events/302952703/)
* 2024-11-19 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346971/)
* 2024-11-20 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Embedded Rust Workshop**](https://www.meetup.com/vancouver-rust/events/304047664/)

### Asia
* 2024-10-29 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup/)
    * [**Bug-Free Concurrency in Rust**](https://www.meetup.com/tokyo-rust-meetup/events/304107583/)

### Europe
* 2024-10-26 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Ferris' Fika Forum #6**](https://www.meetup.com/stockholm-rust/events/303918943/)
* 2024-10-28 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust Meetup #71**](https://www.meetup.com/rust-paris/events/303663366/)
* 2024-10-29 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/303479865)
* 2024-10-30 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn October 2024**](https://www.meetup.com/rust-meetup-hamburg/events/303373054/)
* 2024-10-31 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/300820289/)
* 2024-11-06 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123398/)
* 2024-11-06 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.fr/o/paris-rustaceans-74289178383)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1037795553437)
* 2024-11-14 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @UXStream**](https://www.meetup.com/stockholm-rust/events/304124737/)
* 2024-11-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Daten sichern mit ZFS (und Rust)**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425200/)

### North America
* 2024-10-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcnbfc/)
* 2024-10-26 | Newark, NJ, US | [NJ Code & Coffee](https://www.meetup.com/nj-code-coffee/)
    * [**Intro to Rust: Build a Text Adventure Game x NJ Code & Coffee**](https://www.meetup.com/nj-code-coffee/events/304149949/)
* 2024-10-27 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Kendall Rust Lunch, Oct 27**](https://www.meetup.com/bostonrust/events/303708359/)
* 2024-10-28 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**Generics from the Ground Up**](https://www.meetup.com/boulder-rust-meetup/events/303766925/)
* 2024-10-28 | Ferndale, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ferndale**](https://www.meetup.com/detroitrust/events/303909299/)
* 2024-10-28 | Minneapolis, MN US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/303884468/)
* 2024-10-29 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers : State of the Group and Expectations for 2025**](https://www.meetup.com/music-city-rust-developers/events/301425524/)
* 2024-10-30 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Workshop: deploying your code**](https://www.meetup.com/deep-dish-rust/events/304071348/)
* 2024-11-04 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, Nov 4**](https://www.meetup.com/bostonrust/events/303708387/)
* 2024-11-07 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Game development with Rust and the Bevy engine**](https://www.meetup.com/stl-rust/events/302371464/)
* 2024-11-12 | Ann Arbor, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcpbqb/)
* 2024-11-15 | Mexico City, DF, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Multi threading y Async en Rust parte 2 - Smart Pointes y Closures**](https://www.meetup.com/rust-mx/events/304150412/)
* 2024-11-15 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch, Nov 15**](https://www.meetup.com/bostonrust/events/303708398/)

### Oceania
* 2024-10-28 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**October 2024 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/304034898/)
* 2024-10-29 | Canberra, ACT, AU | [Canberra Rust User Group (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/303128131/)
* 2024-10-31 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Rust on AWS: Sustainability + Peace: Zero Stress Automation**](https://www.meetup.com/rust-akl/events/303824919/)

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

> An earnest effort to pursue \[[P1179R1](https://wg21.link/p1179r1)\] as a Lifetime TS\[[P3465R0](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2024/p3465r0.pdf)\] will compromise on C++’s outdated and unworkable core principles and adopt mechanisms more like Rust’s. In the compiler business this is called *carcinization*: a tendency of non-crab organisms to evolve crab-like features.

– [Sean Baxter on circle-lang.org](https://www.circle-lang.org/draft-profiles.html#carcinization)

Thanks to [Collin Richards](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1627) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
