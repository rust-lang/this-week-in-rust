Title: This Week in Rust 564
Number: 564
Date: 2024-09-11
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

This week's crate is [cargo-override](https://github.com/eopb/cargo-override/), a cargo plugin for quick overriding of dependencies.

Thanks to [Ajith](https://users.rust-lang.org/t/crate-of-the-week/2704/1344) for the suggestion!

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

399 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-09-03..2024-09-10

* [add support for GNU/Hurd on `x86_64`](https://github.com/rust-lang/rust/pull/128345)
* [add target support for RTEMS Arm](https://github.com/rust-lang/rust/pull/127021)
* [`impl_trait_overcaptures`: Don't worry about uncaptured contravariant lifetimes if they outlive a captured lifetime](https://github.com/rust-lang/rust/pull/129028)
* [add suggestions for misspelled keywords](https://github.com/rust-lang/rust/pull/129899)
* [add an internal lint that warns when accessing untracked data](https://github.com/rust-lang/rust/pull/128919)
* [arbitrary self types v2: pointers feature gate](https://github.com/rust-lang/rust/pull/129664)
* [autodiff Upstreaming - enzyme backend](https://github.com/rust-lang/rust/pull/129176)
* [bypass linker configuration and cross target check for specific commands](https://github.com/rust-lang/rust/pull/128871)
* [check WF of source type's signature on fn pointer cast](https://github.com/rust-lang/rust/pull/129021)
* [correctly handle stability of `#[diagnostic]` attributes](https://github.com/rust-lang/rust/pull/130036)
* [coverage: count await when the Future is immediately ready](https://github.com/rust-lang/rust/pull/130013)
* [delegation: support generics in associated delegation items](https://github.com/rust-lang/rust/pull/126161)
* [distribute `rustc_codegen_cranelift` for Windows](https://github.com/rust-lang/rust/pull/128939)
* [do not attempt to prove unknowable goals](https://github.com/rust-lang/rust/pull/129896)
* [do not call query to compute coroutine layout for synthetic body of async closure](https://github.com/rust-lang/rust/pull/129847)
* [do not request sanitizers for naked functions](https://github.com/rust-lang/rust/pull/129891)
* [do not skip linker configuration for `check` builds](https://github.com/rust-lang/rust/pull/130135)
* [don't suggest labeling `const` and `unsafe` blocks](https://github.com/rust-lang/rust/pull/128701)
* [don't build by-move body when async closure is tainted](https://github.com/rust-lang/rust/pull/129677)
* [don't emit `expect`/`assume` in opt-level=0](https://github.com/rust-lang/rust/pull/121614)
* [don't store region in `CapturedPlace`](https://github.com/rust-lang/rust/pull/129987)
* [fix ICE caused by missing span in a region error](https://github.com/rust-lang/rust/pull/130137)
* [fix ICE in CMSE type validation](https://github.com/rust-lang/rust/pull/130064)
* [fix ICE when `asm_const` and `const_refs_to_static` are combined](https://github.com/rust-lang/rust/pull/129472)
* [fix double handling in `collect_tokens`](https://github.com/rust-lang/rust/pull/129346)
* [fix enabling wasm-component-ld to match other tools](https://github.com/rust-lang/rust/pull/130034)
* [fix: get llvm type of global val](https://github.com/rust-lang/rust/pull/128820)
* [implement raw lifetimes and labels (`'r#ident`)](https://github.com/rust-lang/rust/pull/126452)
* [implement suggestions for `elided_named_lifetimes`](https://github.com/rust-lang/rust/pull/129840)
* [interpret: make typed copies lossy wrt provenance and padding](https://github.com/rust-lang/rust/pull/129778)
* [make supertrait and implied predicates queries defaulted](https://github.com/rust-lang/rust/pull/129752)
* [non-exhaustive structs may be empty](https://github.com/rust-lang/rust/pull/128934)
* [rename dump of coroutine by-move-body to be more consistent, fix ICE in `dump_mir`](https://github.com/rust-lang/rust/pull/129706)
* [s390x: fix a regression related to backchain feature](https://github.com/rust-lang/rust/pull/129940)
* [suggest `impl Trait` for References to Bare Trait in Function Header](https://github.com/rust-lang/rust/pull/127692)
* [supress niches in coroutines to avoid aliasing violations](https://github.com/rust-lang/rust/pull/129313)
* [use `DeepRejectCtxt` to quickly reject `ParamEnv` candidates](https://github.com/rust-lang/rust/pull/128776)
* [miri: a bit of refactoring in "sync"](https://github.com/rust-lang/miri/pull/3874)
* [miri: detect when `pthread_mutex_t` is moved](https://github.com/rust-lang/miri/pull/3784)
* [miri: detect when `pthread_rwlock_t` is moved](https://github.com/rust-lang/miri/pull/3871)
* [miri: enable native libraries on macOS](https://github.com/rust-lang/miri/pull/3856)
* [miri: fix comment in `mutex_id_offset`](https://github.com/rust-lang/miri/pull/3865)
* [miri: renamed variable and fixed comments referring to renamed FileDescriptor](https://github.com/rust-lang/miri/pull/3867)
* [stabilize `-Znext-solver=coherence`](https://github.com/rust-lang/rust/pull/121848)
* [stabilize `char::MIN`](https://github.com/rust-lang/rust/pull/130154)
* [stabilize `const_float_bits_conv`](https://github.com/rust-lang/rust/pull/129555)
* [stabilize `waker_getters`](https://github.com/rust-lang/rust/pull/129919)
* [Fix CVE-2024-43402](https://github.com/rust-lang/rust/pull/129962)
* [break into the debugger (if attached) on panics (Windows, Linux, macOS, FreeBSD)](https://github.com/rust-lang/rust/pull/129019)
* [const: make `ptr.is_null()` stop execution on ambiguity](https://github.com/rust-lang/rust/pull/130107)
* [make `Result::copied` unstably const](https://github.com/rust-lang/rust/pull/130090)
* [str: make `as_mut_ptr` and `as_bytes_mut` unstably const](https://github.com/rust-lang/rust/pull/130046)
* [use the trifecta div algorithm for 128-bit div on wasm](https://github.com/rust-lang/compiler-builtins/pull/685)
* [cargo: resolve: Report MSRV compatible version instead of incomptible](https://github.com/rust-lang/cargo/pull/14471)
* [cargo: new: Add to workspace relative to manifest, not current-dir](https://github.com/rust-lang/cargo/pull/14505)
* [cargo: bail before packaging on same version](https://github.com/rust-lang/cargo/pull/14448)
* [cargo: don't automatically include the current crate when packaging](https://github.com/rust-lang/cargo/pull/14488)
* [cargo: fix cargo add behaving different when translating package name](https://github.com/rust-lang/cargo/pull/13765)
* [cargo: fix parsing of comma separated values in --crate-type flag](https://github.com/rust-lang/cargo/pull/14499)
* [cargo: include public/private dependency status in `cargo metadata`](https://github.com/rust-lang/cargo/pull/14504)
* [cargo: publish workspace](https://github.com/rust-lang/cargo/pull/14433)
* [cargo: remove unnecessary symbols](https://github.com/rust-lang/cargo/pull/14519)
* [cargo: uplift windows gnullvm import libraries](https://github.com/rust-lang/cargo/pull/14451)
* [rustdoc-search: allow trailing `Foo →` arg search](https://github.com/rust-lang/rust/pull/130009)
* [rustdoc: Sort impl associated items by kinds and then by appearance](https://github.com/rust-lang/rust/pull/129471)
* [rustdoc: add header map to the table of contents](https://github.com/rust-lang/rust/pull/120736)
* [rustdoc: normalise type/field names](https://github.com/rust-lang/rust/pull/128667)
* [rustdoc: use strategic boxing to shrink `clean::Item`](https://github.com/rust-lang/rust/pull/129789)
* [rustfmt: impl `rewrite_result` for ForeignItem, TraitAliasBounds, WherePredicate](https://github.com/rust-lang/rustfmt/pull/6309)
* [rustfmt: impl `rewrite_result` for `ast::Expr`](https://github.com/rust-lang/rustfmt/pull/6311)
* [rustfmt: implement version-sort for imports in `style_edition` 2024](https://github.com/rust-lang/rustfmt/pull/6284)
* [bindgen: stabilize `--wrap-static-fns`](https://github.com/rust-lang/rust-bindgen/pull/2928)
* [clippy: `single_match`, `single_match_else`: fix suggestion when match irrefutable](https://github.com/rust-lang/rust-clippy/pull/13324)
* [clippy: `manual_div_ceil`: init](https://github.com/rust-lang/rust-clippy/pull/12987)
* [clippy: add new check for passing pointers to an `asm!` block with `nomem` option](https://github.com/rust-lang/rust-clippy/pull/13247)
* [clippy: add new lint `manual_is_power_of_two`](https://github.com/rust-lang/rust-clippy/pull/13327)
* [clippy: added new `non_zero_suggestions` lint](https://github.com/rust-lang/rust-clippy/pull/13167)
* [clippy: fix `needless_return` false negative](https://github.com/rust-lang/rust-clippy/pull/13214)
* [clippy: move `manual_c_str_literals` to complexity](https://github.com/rust-lang/rust-clippy/pull/13263)
* [clippy: only lint `manual_non_exhaustive` for exported types](https://github.com/rust-lang/rust-clippy/pull/13345)
* [clippy: visit `struct` fields recursively in uninit fallback check](https://github.com/rust-lang/rust-clippy/pull/13367)
* [rust-analyzer: IDE support for `asm!` expressions](https://github.com/rust-lang/rust-analyzer/pull/18022)
* [rust-analyzer: better name suggestions for fn](https://github.com/rust-lang/rust-analyzer/pull/18041)
* [rust-analyzer: always explicitly set `TraitRef` self types when lowering](https://github.com/rust-lang/rust-analyzer/pull/18068)
* [rust-analyzer: catch panics from diagnostics computation](https://github.com/rust-lang/rust-analyzer/pull/18065)
* [rust-analyzer: couple asm! parsing and lowering fixes](https://github.com/rust-lang/rust-analyzer/pull/18053)
* [rust-analyzer: don't panic lsp writer thread on dropped receiver](https://github.com/rust-lang/rust-analyzer/pull/18066)
* [rust-analyzer: fix lowering of for loops dropping the loop block](https://github.com/rust-lang/rust-analyzer/pull/18045)
* [rust-analyzer: properly prevent mir building with unknown types present](https://github.com/rust-lang/rust-analyzer/pull/18067)
* [rust-analyzer: updating settings should not clobber discovered projects](https://github.com/rust-lang/rust-analyzer/pull/18059)

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

Rusty Events between 2024-09-11 - 2024-10-09 🦀

### Virtual
* 2024-09-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - Typestate Pattern in Rust: With a Strict Builder Example**](https://www.meetup.com/indyrs/events/300328029/)
* 2024-09-05 | Virtual (Buenos Aires, AR) | [LambdaClass](https://lu.ma/user/usr-dkk9KnFvsvZEb7k)
    * [**Meetup Rust Septiembre [Spanish]**](https://lu.ma/uh1qpox0)
* 2024-09-05 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897957/)
* 2024-09-05 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820268/)
* 2024-09-10 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346981/)
* 2024-09-10 - 2024-09-13 | Hybrid: Virtual and In-Person (Montreal, QC, CA) | [Rust Conf](https://rustconf.com/)
    * [**Rust Conf 2024**](https://foundation.rust-lang.org/events/rustconf-2024/)
* 2024-09-12 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633268/)
* 2024-09-12 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/)
    * [**Bevy Meetup #6**](https://www.meetup.com/bevy-game-development/events/302841892/)
* 2024-09-16 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/302827971/)
* 2024-09-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346969/)
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
* 2024-09-26 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Rusty secure communication on embedded devices**](https://www.meetup.com/charlottesville-rust-meetup/events/303159380/)
* 2024-10-02 | Virtual (Vancouver, BC, CA) | [Vancouver Postgres](https://www.meetup.com/vancouver-postgres/)
    * [**Leveraging a PL/RUST extension to protect sensitive data in PostgreSQL**](https://www.meetup.com/vancouver-postgres/events/302160672/)

### Africa
* 2024-09-06 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587/)

### Asia
* 2024-09-07 - 2024-09-08 | Shanghai, CN | [Rust China](https://rustcc.cn/)
    * [**Rust China Conf: Shanghai**](https://rustcc.cn/2024conf/)
* 2024-09-09 | Ramat Gan, IL | [Coralogix](https://coralogix.com/)
    * [**Rust as Scale**](https://coralogix.com/rust-coralogix-meetup/)
* 2024-09-14 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**September 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/september-2024-rustacean-meetup/)

### Europe
* 2024-09-04 | Oxford, UK | [Oxfrod Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**More Rust - Generics, constraints, safety.**](https://www.meetup.com/oxford-rust-meetup-group/events/302848276/)
* 2024-09-11 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/302833564/)
* 2024-09-17 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425049/)
* 2024-09-17 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/)
    * [**Making AI-models perform tasks, in Rust!**](https://www.meetup.com/rust-trondheim/events/302957040/)
* 2024-09-18 | Moravia, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/)
    * [**Rust Moravia Meetup (September 2024)**](https://www.meetup.com/rust-moravia/events/301360936)
* 2024-09-18 | Vienna, AT + Virtual | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2024 (Sep 18-20)**](https://lpc.events/event/18/sessions/186/)
* 2024-09-21 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Ferris' Fika Forum #5**](https://www.meetup.com/Stockholm-Rust/events/303210419)
* 2024-09-23 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake #6**](https://www.meetup.com/bratislava-rust-meetup-group/events/302916594/)
* 2024-09-24 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust)
    * [**Rust meetup #70**](https://www.meetup.com/Stockholm-Rust/events/303210419)
* 2024-09-26 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night**](https://www.meetup.com/rust-aarhus/events/301522991/)
* 2024-09-27 | Mannheim, DE | [Hackerstolz e.V.](https://www.hackerstolz.de/en/)
    * [**Hackerstolz Stammtisch Rhein-Neckar**](https://www.hackerstolz.de/en/)
* 2024-10-02 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @Funnel**](https://www.meetup.com/Stockholm-Rust/events/303213095)

### North America
* 2024-09-05 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Shooting Stars: Create a game from scratch in 25 minutes!**](https://www.meetup.com/utah-rust/events/303204006/)
* 2024-09-05 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/302723843/)
* 2024-09-05 | Portland, OR, US | [PDX Rust](https://www.meetup.com/pdxrust/)
    * [**PDX Rust September!**](https://www.meetup.com/pdxrust/events/302588479/)
* 2024-09-05 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Lifetimes**](https://www.meetup.com/stl-rust/events/hdzdmtygcmbhb/)
* 2024-09-07 | Longview, TX, US | [Longview Code and Coffee](https://www.meetup.com/longview-code-and-coffee/)
    * [**Longview Code and Coffee**](https://www.meetup.com/longview-code-and-coffee/events/301976293/)
* 2024-09-08 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/302498734/)
    * [**Northeastern Rust Lunch, Sep 8**](https://www.meetup.com/bostonrust/events/302498706/)
* 2024-09-10 - 2024-09-13 | Hybrid: Virtual and In-Person (Montreal, QC, CA) | [Rust Conf](https://rustconf.com/)
    * [**Rust Conf 2024**](https://foundation.rust-lang.org/events/rustconf-2024/)
* 2024-09-11 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**Boulder Elixir Meetup**](hhttps://www.meetup.com/boulder-elixir/events/302991078/)
* 2024-09-16 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Somerville Union Square Rust Lunch, Sep 16**](https://www.meetup.com/bostonrust/events/302498750/)
* 2024-09-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638248/)
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

> Alas! We are once more bereft  
> of a quote to elate or explain  
> so this editor merely has left  
> the option in rhyme to complain.

– llogiq

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
