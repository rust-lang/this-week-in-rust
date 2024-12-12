Title: This Week in Rust 577
Number: 577
Date: 2024-12-11
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
* [This Month in Our Test Infra: November 2024](https://blog.rust-lang.org/inside-rust/2024/12/09/test-infra-nov-2024.html)
* [December 2024 Leadership Council Update](https://blog.rust-lang.org/inside-rust/2024/12/09/leadership-council-update.html)

### Foundation

### Newsletters
* [The Embedded Rustacean Issue #34](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-34)
* [Rust Trends Issue #55](https://rust-trends.com/newsletter/your-weekly-rust-fix-templates-dependencies-and-a-big-giveaway/)

### Project/Tooling Updates
* [Dioxus 0.6](https://dioxuslabs.com/blog/release-060/)
* [Flawless Replay](https://flawless.dev/replay/)
* [Rust9x update: Rust 1.84.0-beta](https://seri.tools/blog/rust9x-1-84/)

### Observations/Thoughts
* [Rust Macros: A Cautionary Tale](https://andrzej.lichnerowicz.pl/en/blog/rust-macros-a-cautionary-tale/)
* [Running teloxide bots cheaply on Fly.io](https://blog.valyagolev.net/fly-teloxide/)
* [Speeding up Ruby by rewriting C… in Ruby](https://jpcamara.com/2024/12/01/speeding-up-ruby.html)
* [Memory-safe PNG decoders now vastly outperform C PNG libraries](https://www.reddit.com/r/rust/comments/1ha7uyi/memorysafe_png_decoders_now_vastly_outperform_c)
* [State of the Crates 2025](https://ohadravid.github.io/posts/2024-12-state-of-the-crates/)

### Rust Walkthroughs
* [Parsing MIDI messages in Rust](https://www.ntietz.com/blog/parsing-midi-rust/)
* [Drag & Drop Images into Bevy 0.15 on the web](https://rustunit.com/blog/2024/12-10-rust-web-drag-drop-image/
* [Missing iterable traits and how to introduce them effortlessly](https://orxfun.github.io/orxfun-notes/#/missing-iterable-traits-2024-12-13)

### Research

### Miscellaneous
* [video] [How to Integrate C++ and Rust](https://www.youtube.com/playlist?list=PL6CJYn40gN6jOg_cPqRfXMNriHknKy4VW)
* [video] [2024 LLVM Developers' Meeting - Rust ❤️ LLVM](https://www.youtube.com/watch?v=Kqz-umsAnk8)
* [My Rust Story](https://softwaremill.com/andre-bogus-my-rust-story/)

## Crate of the Week

This week's crate is [include-utils](https://github.com/alekseysidorov/include-utils), a more powerful replacement for the standard library's `include_str` macro.

Thanks to [Aleksey Sidorov](https://users.rust-lang.org/t/crate-of-the-week/2704/1381) for the self-suggestion!

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

462 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-12-03..2024-12-10

* [`dataflow_const_prop`: do not eval a ptr address in SwitchInt](https://github.com/rust-lang/rust/pull/134073)
* [`fn_sig_for_fn_abi` should return a `ty::FnSig`, no need for a binder](https://github.com/rust-lang/rust/pull/133874)
* [`rust_for_linux`: -Zreg-struct-return commandline flag for X86](https://github.com/rust-lang/rust/pull/130777)
* [actually walk into lifetimes and attrs in `EarlyContextAndPass`](https://github.com/rust-lang/rust/pull/133992)
* [add `allocate_bytes` and refactor `allocate_str` in InterpCx for raw byte…](https://github.com/rust-lang/rust/pull/133861)
* [add context to "const in pattern" errors](https://github.com/rust-lang/rust/pull/133233)
* [add lint against function pointer comparisons](https://github.com/rust-lang/rust/pull/118833)
* [add more info on type/trait mismatches for different crate versions](https://github.com/rust-lang/rust/pull/133767)
* [avoid `opaque type not constrained` errors in the presence of other errors](https://github.com/rust-lang/rust/pull/133850)
* [avoid fetching the anon const hir node that is already available](https://github.com/rust-lang/rust/pull/133936)
* [deeply normalize when computing implied outlives bounds](https://github.com/rust-lang/rust/pull/133517)
* [do not implement unsafe auto traits for types with unsafe fields](https://github.com/rust-lang/rust/pull/133934)
* [don't suggest restricting bound with unstable traits on stable and mention it's unstable on nightly](https://github.com/rust-lang/rust/pull/133522)
* [don't use a SyntheticProvider for literally every type](https://github.com/rust-lang/rust/pull/133134)
* [fix MutVisitor's default implementations to visit Stmt's and BinOp's spans](https://github.com/rust-lang/rust/pull/133784)
* [fix suggestion when shorthand `self` has erroneous type](https://github.com/rust-lang/rust/pull/122161)
* [gate async fn trait bound modifier on `async_trait_bounds`](https://github.com/rust-lang/rust/pull/132612)
* [handle `--json-output` properly](https://github.com/rust-lang/rust/pull/133875)
* [hide errors whose suggestions would contain error constants or types](https://github.com/rust-lang/rust/pull/133954)
* [implement checks for tail calls](https://github.com/rust-lang/rust/pull/133607)
* [improve `TagEncoding::Niche` docs, sanity check, and UB checks](https://github.com/rust-lang/rust/pull/133681)
* [include LLDB and GDB visualizers in MSVC distribution](https://github.com/rust-lang/rust/pull/133737)
* [introduce `default_field_values` feature](https://github.com/rust-lang/rust/pull/129514)
* [lint against `Symbol::intern` on a string literal](https://github.com/rust-lang/rust/pull/133545)
* [lint: change help for pointers to dyn types in FFI](https://github.com/rust-lang/rust/pull/131669)
* [make CoercePointee errors translatable](https://github.com/rust-lang/rust/pull/133774)
* [make sure to record deps from cached task in new solver on first run](https://github.com/rust-lang/rust/pull/133828)
* [move most tests for `-l` and `#[link(..)]` into `tests/ui/link-native-libs`](https://github.com/rust-lang/rust/pull/133996)
* [no need to create placeholders for GAT args in `confirm_object_candidate`](https://github.com/rust-lang/rust/pull/133872)
* [only allow `PassMode::Direct` for aggregates on wasm when using the C ABI](https://github.com/rust-lang/rust/pull/133931)
* [parse guard patterns](https://github.com/rust-lang/rust/pull/133424)
* [reduce false positives on some common cases from if-let-rescope lint](https://github.com/rust-lang/rust/pull/133753)
* [reimplement `~const` trait specialization](https://github.com/rust-lang/rust/pull/133325)
* [structurally resolve in `adjust_for_branches`](https://github.com/rust-lang/rust/pull/133559)
* [structurally resolve in `probe_adt`](https://github.com/rust-lang/rust/pull/133558)
* [unify `sysroot_target_{bin,lib}dir` handling](https://github.com/rust-lang/rust/pull/132723)
* [use correct `hir_id` for array const arg infers](https://github.com/rust-lang/rust/pull/133779)
* [miri: cleanup: avoid passing byte slice to `anonsocket_read`](https://github.com/rust-lang/miri/pull/4074)
* [miri: fix SC fence logic](https://github.com/rust-lang/miri/pull/4076)
* [miri: fix weak memory emulation to avoid generating behaviors that are forbidden under C++ 20](https://github.com/rust-lang/miri/pull/4057)
* [miri: implement `simd_relaxed_fma`](https://github.com/rust-lang/miri/pull/4071)
* [extend Miri to correctly pass mutable pointers through FFI](https://github.com/rust-lang/rust/pull/133211)
* [remove polymorphization](https://github.com/rust-lang/rust/pull/133883)
* [introduce `MixedBitSet`](https://github.com/rust-lang/rust/pull/133891)
* [stabilize `const_collections_with_hasher` and `build_hasher_default_const_new`](https://github.com/rust-lang/rust/pull/133696)
* [stabilize `const_{size,align}_of_val`](https://github.com/rust-lang/rust/pull/133762)
* [stabilize `noop_waker`](https://github.com/rust-lang/rust/pull/133089)
* [stabilize `std::io::ErrorKind::CrossesDevices`](https://github.com/rust-lang/rust/pull/130209)
* [stabilize `std::io::ErrorKind::QuotaExceeded`](https://github.com/rust-lang/rust/pull/130254)
* [add `core::arch::breakpoint` and test](https://github.com/rust-lang/rust/pull/133726)
* [implementation of `fmt::FormattingOptions`](https://github.com/rust-lang/rust/pull/118159)
* [add Extend impls for tuples of arity 1 through 12](https://github.com/rust-lang/rust/pull/132187)
* [cargo: `docs(fingerprint)`: cargo-rustc extra flags do not affect the metadata](https://github.com/rust-lang/cargo/pull/14898)
* [cargo: `feat(build-rs)`: Add the 'error' directive](https://github.com/rust-lang/cargo/pull/14910)
* [cargo: `fix(add)`: Don't select yanked versions when normalizing names](https://github.com/rust-lang/cargo/pull/14895)
* [cargo: `fix(build-rs)`: Correctly refer to the item in assert](https://github.com/rust-lang/cargo/pull/14913)
* [cargo: `fix(build-std)`: determine root crates by target spec `std:bool`](https://github.com/rust-lang/cargo/pull/14899)
* [cargo: `fix(fingerprint)`: Don't throwaway the cache on RUSTFLAGS changes](https://github.com/rust-lang/cargo/pull/14830)
* [cargo: `fix(fix)`: Migrate workspace dependencies](https://github.com/rust-lang/cargo/pull/14890)
* [cargo: `test(build-std)`: make mock-std closer to real world](https://github.com/rust-lang/cargo/pull/14896)
* [cargo: fix(build-rs)!: remove meaningless `'cargo_cfg_debug_assertions'`](https://github.com/rust-lang/cargo/pull/14901)
* [cargo: refactor: use `Path::push` to construct remap-path-prefix](https://github.com/rust-lang/cargo/pull/14908)
* [cargo: semVer: add section on RPIT capturing](https://github.com/rust-lang/cargo/pull/14849)
* [rustdoc: remove eq for `clean::Attributes`](https://github.com/rust-lang/rust/pull/133960)
* [rustdoc: rename `issue-\d+.rs` tests to have meaningful names (part 10)](https://github.com/rust-lang/rust/pull/134053)
* [rustdoc: rename `set_back_info` to `restore_module_data`](https://github.com/rust-lang/rust/pull/133764)
* [rustdoc: always display first line of impl blocks even when collapsed](https://github.com/rust-lang/rust/pull/132155)
* [improve code for FileName retrieval in rustdoc](https://github.com/rust-lang/rust/pull/133804)
* [clippy: `doc_lazy_continuation`: Correctly count indent with backslashes](https://github.com/rust-lang/rust-clippy/pull/13742)
* [clippy: extend `precedence` for bitmasking and shift](https://github.com/rust-lang/rust-clippy/pull/13743)
* [clippy: new lint for `as *const _` and `as *mut _` pointer casts](https://github.com/rust-lang/rust-clippy/pull/13251)
* [rust-analyzer: add Configurable Option to Exclude Trigger Characters for Typing Assists](https://github.com/rust-lang/rust-analyzer/pull/18522)
* [rust-analyzer: add implict unsafety inlay hints for extern blocks](https://github.com/rust-lang/rust-analyzer/pull/18610)
* [rust-analyzer: add typing handler for param list pipe](https://github.com/rust-lang/rust-analyzer/pull/18628)
* [rust-analyzer: complete derive helper attributes](https://github.com/rust-lang/rust-analyzer/pull/18604)
* [rust-analyzer: complete diagnostics in ty lowering groundwork and serve a first diagnostic 🎉](https://github.com/rust-lang/rust-analyzer/pull/18541)
* [rust-analyzer: extend reported unsafe operations](https://github.com/rust-lang/rust-analyzer/pull/18609)
* [rust-analyzer: support `AsyncFnX` traits](https://github.com/rust-lang/rust-analyzer/pull/18594)
* [rust-analyzer: fix parsing of parenthesized type args and RTN](https://github.com/rust-lang/rust-analyzer/pull/18593)
* [rust-analyzer: better parser recovery for paths](https://github.com/rust-lang/rust-analyzer/pull/18608)
* [rust-analyzer: coerce two `FnDef`s to fn pointers even if they are the same, if they are subtypes](https://github.com/rust-lang/rust-analyzer/pull/18633)
* [rust-analyzer: disable `<` typing handler again](https://github.com/rust-lang/rust-analyzer/pull/18616)
* [rust-analyzer: do not report warnings from proc macros, ever](https://github.com/rust-lang/rust-analyzer/pull/18611)
* [rust-analyzer: fix a bug when synthetic AST node were searched in the AST ID map and caused panics](https://github.com/rust-lang/rust-analyzer/pull/18555)
* [rust-analyzer: fix parser getting stuck for bad asm expressions](https://github.com/rust-lang/rust-analyzer/pull/18625)
* [rust-analyzer: fix parsing of dyn T in generic arg on 2015 edition](https://github.com/rust-lang/rust-analyzer/pull/18622)
* [rust-analyzer: fix parsing of integer/keyword name refs in various places](https://github.com/rust-lang/rust-analyzer/pull/18618)
* [rust-analyzer: fix shadowing of record `enum` variant in patterns](https://github.com/rust-lang/rust-analyzer/pull/18607)
* [rust-analyzer: fixed another bug with glob imports](https://github.com/rust-lang/rust-analyzer/pull/18605)
* [rust-analyzer: map new replacement nodes to their mutable equivalents in `SyntaxEditor`](https://github.com/rust-lang/rust-analyzer/pull/18531)
* [rust-analyzer: non-exhaustive structs may be empty](https://github.com/rust-lang/rust-analyzer/pull/18645)
* [rust-analyzer: panic when displaying generic params with defaults](https://github.com/rust-lang/rust-analyzer/pull/18619)
* [rust-analyzer: parse lifetime bounds in lifetime param into TypeBoundList](https://github.com/rust-lang/rust-analyzer/pull/18620)
* [rust-analyzer: resolve generic parameters within use captures](https://github.com/rust-lang/rust-analyzer/pull/18621)
* [rust-analyzer: temporarily disable completion resolve support for helix and neovim](https://github.com/rust-lang/rust-analyzer/pull/18630)
* [rust-analyzer: improve heuristics for on typing semicolon insertion](https://github.com/rust-lang/rust-analyzer/pull/18627)
* [rust-analyzer: make bracket typing handler work on more things](https://github.com/rust-lang/rust-analyzer/pull/18474)
* [rust-analyzer: migrate `add_turbo_fish` to `SyntaxEditor`](https://github.com/rust-lang/rust-analyzer/pull/18551)
* [rust-analyzer: migrate `introduce_named_generic` Assist to Use `SyntaxFactory`](https://github.com/rust-lang/rust-analyzer/pull/18483)
* [rust-analyzer: migrate `sort_items` Assist to Use `SyntaxFactory`](https://github.com/rust-lang/rust-analyzer/pull/18538)
* [rust-analyzer: vscode: only show status bar item in relevant files](https://github.com/rust-lang/rust-analyzer/pull/18592)

### Rust Compiler Performance Triage

A pretty quiet week, with both few PRs landed and no large changes in performance.

Triage done by **@simulacrum**.
Revision range: [490b2cc0..1b3fb316](https://perf.rust-lang.org/?start=490b2cc09860dd62a7595bb07364d71c12ce4e60&end=1b3fb316751227d30b1523ed0e3f00d83956d4d0&absolute=false&stat=instructions%3Au)

0 Regressions, 0 Improvements, 7 Mixed; 4 of them in rollups
25 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-12-09.md)

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

Rusty Events between 2024-12-11 - 2025-01-08 🦀

### Virtual
* 2024-12-05 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/05/rust-hack-and-learn.html) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633275/)
* 2024-12-05 | Virtual (Miami, FL) | [Miami Java User Group Events](https://www.meetup.com/miami-java-user-group)
    * [**Introduction to Rust for Java Developers**](https://www.meetup.com/miami-java-user-group/events/304717903/)
* 2024-12-06 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304369723/)
* 2024-12-07 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2024-12-08 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Reading JSON files in Rust - קריאת קבצי ג'ייסון בראסט**](https://www.meetup.com/rust-tlv/events/304685546/)
* 2024-12-10 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346988/)
* 2024-12-11 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**egui**](https://www.meetup.com/vancouver-rust/events/304047666/)
* 2024-12-12 | Hybrid: In-Person and Virtual (Seattle, WA, US) | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**December Meetup**](https://www.meetup.com/Seattle-Rust-Meetup/)
* 2024-12-12 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898129/)
* 2024-12-12 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820276/)
* 2024-12-13 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304730560/)
* 2024-12-17 | Virtual (San Francisco, CA, US) | [Blockchain Center SF](https://www.meetup.com/blockchaincentersf/)
    * [**Rust in Web3: Developer Series**](https://www.meetup.com/blockchaincentersf/events/kwnzntygcqbwb/)
* 2024-12-17 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Rust Source Code Reading: The thousands crate (Virtual, English)**](https://www.meetup.com/code-mavens/events/304824684/)
* 2024-12-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346972/)
* 2024-12-19 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633276/)
* 2024-12-19 | Virtual (Mexico City, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Posada 2024**](https://www.meetup.com/rust-mx/events/304639403/)
* 2024-12-20 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcqbbc/)
* 2024-12-24 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/fkmcntygcqbgc/)
* 2024-12-26 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898129/)

### Africa
* 2024-12-10 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Hello World... again**](https://www.meetup.com/johannesburg-rust-meetup/events/304649358/)

### Asia
* 2024-12-14 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**December 2024 Rustacean meetup/workshop**](https://hasgeek.com/rustbangalore/december-2024-rustacean-meetup-workshop/)

### Europe
* 2024-12-04 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Aprenem junts Rust / Learn Rust Together**](https://lu.ma/pypwr0m7)
* 2024-12-04 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust in December: Advent of Code**](https://www.meetup.com/rustcologne/events/304760521/)
* 2024-12-04 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123399/)
* 2024-12-04 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #73**](https://www.meetup.com/rust-paris/events/304685955/)
* 2024-12-05 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #6**](https://www.meetup.com/rust-gdansk/events/304773705/)
* 2024-12-05 | Zlin, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**Rust Moravia Meetup (December 2024)**](https://www.meetup.com/rust-moravia/events/304075150/)
* 2024-12-06 | Moscow, RU | [RustCon RU](https://rustcon.ru/)
    * [**RustCon Russia**](https://rustcon.ru/)
* 2024-12-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtygcqbpb/)
* 2024-12-12 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ JetBrains**](https://www.meetup.com/rust-amsterdam-group/events/304514267/)
* 2024-12-12 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2024 / 4 - Hacking Evening**](https://www.meetup.com/rust-munich/events/304827279/)
* 2024-12-12 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/events/)
    * [**Rust Vienna - December | at Sentry.io 🦀**](https://www.meetup.com/rust-vienna/events/304815850/)
* 2024-12-17 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Secret Santa in Rust: Unwrapping Property Testing**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425056/)
* 2024-12-18 | Ghent, BE | [Systems Programming Ghent](https://sysghent.be)
    * [**Launch of new community for Rust and C++ developers**](https://sysghent.be)


### North America
* 2024-12-05 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal/events/)
    * [**December Monthly Social**](https://www.meetup.com/rust-montreal/events/304778367/)
* 2024-12-05 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Rust Strings**](https://www.meetup.com/stl-rust/events/302371466/)
* 2024-12-10 | Ann Arbor, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcqbnb/)
* 2024-12-12 | Hybrid: In-Person and Virtual (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug/)
    * [**December Meetup**](https://www.meetup.com/join-srug/events/304806455/)
* 2024-12-12 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/wqkgntygcqbqb/)
* 2024-12-16 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/events/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/304530508/)
* 2024-12-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638256/)
* 2024-12-26 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/wqkgntygcqbjc/)

### Oceania
* 2024-12-04 | Sydney, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/events/)
    * [**2024 🦀 Encore ✨ Talks**](https://www.meetup.com/rust-sydney/events/304625921/)
* 2024-12-08 | Canberra, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/events/)
    * [**CRUG Xmas party**](https://www.meetup.com/rust-canberra/events/304282046/)
* 2024-12-16 | Collingwood, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/events/)
    * [**December 2024 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/304820598/)

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

> Memory-safe implementations of PNG ([png](https://crates.io/crates/png), [zune-png](https://crates.io/crates/zune-png), [wuffs](https://github.com/google/wuffs/)) now dramatically outperform memory-unsafe ones ([libpng](http://www.libpng.org/), [spng](https://libspng.org/), [stb_image](https://github.com/nothings/stb)) when decoding images.
>
> Rust [png](https://crates.io/crates/png) crate that tops our benchmark shows **1.8x** improvement over `libpng` on x86 and **1.5x** improvement on ARM.

– [Shnatsel on /r/rust](https://www.reddit.com/r/rust/comments/1ha7uyi/memorysafe_png_decoders_now_vastly_outperform_c/)

Thanks to [Anton Fetisov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1641) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
