Title: This Week in Rust 529
Number: 529
Date: 2024-01-10
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
* [This Development-cycle in Cargo: 1.76](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76.html)

### Foundation
* [Rust Foundation New Member Announcement: xFusion, Lynx, & SpruceID](https://foundation.rust-lang.org/news/rust-foundation-new-member-announcement-xfusion-lynx-spruceid/)

### Newsletters
* [This Month in Rust OSDev: December 2023](https://rust-osdev.com/this-month/2023-12/)

### Project/Tooling Updates
* [Maestro - Introduction](https://blog.lenot.re/a/introduction)
* [Polars](https://pola.rs/posts/polars_in_aggregrate-0.20/)
* [rust-analyzer changelog #215](https://rust-analyzer.github.io/thisweek/2024/01/08/changelog-215.html)
* [argmin 0.9.0 - a Rust crate for numerical optimization](https://argmin-rs.org/blog/version-v0-9-0/)
* [Continuous benchmarking for rustls](https://ochagavia.nl/blog/continuous-benchmarking-for-rustls/)
* [embedded-hal v1.0 now released!](https://blog.rust-embedded.org/embedded-hal-v1/)

### Observations/Thoughts
* [Arrays: index out of bounds error? Not always!](https://www.greyblake.com/blog/index-out-of-bounds-not-always-a-rust-surprise/)
* [What I'd like to see for Async Rust in 2024](https://smallcultfollowing.com/babysteps/blog/2024/01/03/async-rust-2024/)
* [Securing the Web: Rustls on track to outperform OpenSSL](https://www.memorysafety.org/blog/rustls-performance/)
* [audio] [Rust Audio Programming with Ian Hobson](https://thewolfsound.com/talk016/)
* [audio] [Polars with Ritchie Vink](https://rustacean-station.org/episode/ritchie-vink/)

* [Inception style builds with private GitHub dependencies](https://heikoseeberger.de/2024-01-06-inception-style-build/)

### Rust Walkthroughs
* [Getting Started with Tracing in Rust](https://www.shuttle.rs/blog/2024/01/09/getting-started-tracing-rust)
* [video] [you need to build a RUST desktop app!!](https://www.youtube.com/watch?v=7aFgeUG9TK4)
* [Doing First Grade Math in Rust's Type System](https://fprasx.github.io/articles/type-system-arithmetic/)
* [Let's make an information display in rust](https://blog.stillinbeta.com/2024-01-01-overengineered-household-display.html)

### Research
* [Why stdout is faster than stderr?](https://blog.orhun.dev/stdout-vs-stderr/)

### Miscellaneous
* [3 ways to handle number overflow or underflow in Rust](https://rust.code-maven.com/how-to-handle-overflow)
* [Rocket - multi-counter using cookies](https://rust.code-maven.com/rocket-multi-counter-using-cookies)
* [unwrap, one way to handle errors in Rust](https://rust.code-maven.com/unwrap)
* [video] [Top 10 Games from Bevy Jam 4](https://www.youtube.com/watch?v=FVhOkpIytJc)
* [video] [you need to build a RUST desktop app!!](https://m.youtube.com/watch?v=7aFgeUG9TK4)

## Crate of the Week

This week's crate is [named-sem](https://crates.io/crates/named-sem), a wrapper library for named semaphores on Linux & Windows.

Thanks to [EvianZhang](https://users.rust-lang.org/t/crate-of-the-week/2704/1277) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->
* [Hyperswitch - [FEATURE]: Make cache configuration configurable at runtime](https://github.com/juspay/hyperswitch/issues/3276)
* [Hyperswitch - [FEATURE]: Implement Code cov for local system using makefile](https://github.com/juspay/hyperswitch/issues/1622)
* [Hyperswitch - [FEATURE]: Setup code coverage for local tests & CI](https://github.com/juspay/hyperswitch/issues/1587)
* [Hyperswitch - [FEATURE]: Add domain type for client secret](https://github.com/juspay/hyperswitch/issues/1357)
* [Hyperswitch - [FEATURE]: Have get_required_value to use ValidationError in OptionExt](https://github.com/juspay/hyperswitch/issues/860)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](link to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website either through a PR to TWiR or on the [Rust-lang forums].[link TBD]

## Updates from the Rust Project

446 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-01-02..2024-01-09

* [promote `riscv32{im|imafc}` targets to tier 2](https://github.com/rust-lang/rust/pull/118704)
* [add `riscv32imafc-esp-espidf` tier 3 target for the ESP32-P4](https://github.com/rust-lang/rust/pull/119738)
* [support `reg_addr` register class in s390x inline assembly](https://github.com/rust-lang/rust/pull/119431)
* [add -Zuse-sync-unwind](https://github.com/rust-lang/rust/pull/117744)
* [`macro_rules`: Add an expansion-local cache to span marker](https://github.com/rust-lang/rust/pull/119693)
* [`macro_rules`: Less hacky heuristic for using `tt` metavariable spans](https://github.com/rust-lang/rust/pull/119204)
* [`rustc_mir_transform`: Enforce `rustc::potential_query_instability` lint](https://github.com/rust-lang/rust/pull/119252)
* [`rustc_mir_transform`: Make DestinationPropagation stable for queries](https://github.com/rust-lang/rust/pull/119591)
* [`rustc_span`: More consistent span combination operations](https://github.com/rust-lang/rust/pull/119624)
* [`rustc_span`: Optimize syntax context comparisons](https://github.com/rust-lang/rust/pull/119531)
* [allow coverage tests to ignore test modes, and to enable color in coverage reports](https://github.com/rust-lang/rust/pull/119034)
* [avoid specialization in the metadata serialization code](https://github.com/rust-lang/rust/pull/119478)
* [check yield terminator's resume type in borrowck](https://github.com/rust-lang/rust/pull/119563)
* [coverage: `llvm-cov` expects column numbers to be bytes, not code points](https://github.com/rust-lang/rust/pull/119033)
* [coverage: anonymize line numbers in branch views](https://github.com/rust-lang/rust/pull/119681)
* [coverage: avoid a query stability hazard in `function_coverage_map`](https://github.com/rust-lang/rust/pull/119514)
* [coverage: hoist some complex code out of the main span refinement loop](https://github.com/rust-lang/rust/pull/119208)
* [deny defaults for higher-ranked generic parameters](https://github.com/rust-lang/rust/pull/119494)
* [don't synthesize host effect args inside trait object types](https://github.com/rust-lang/rust/pull/119540)
* [don't synthesize host effect params for trait associated functions marked const](https://github.com/rust-lang/rust/pull/119505)
* [enable address sanitizer for MSVC targets using INFERASANLIBS linker flag](https://github.com/rust-lang/rust/pull/118521)
* [exhaustiveness: statically enforce revealing of opaques](https://github.com/rust-lang/rust/pull/119329)
* [fix scoping for let chains in match guards](https://github.com/rust-lang/rust/pull/119554)
* [handle ForeignItem as TAIT scope](https://github.com/rust-lang/rust/pull/119420)
* [hide foreign `#[doc(hidden)]` paths in import suggestions](https://github.com/rust-lang/rust/pull/119151)
* [impl trait diagnostic tweaks](https://github.com/rust-lang/rust/pull/119703)
* [imply outlives-bounds on lazy type aliases](https://github.com/rust-lang/rust/pull/119350)
* [improved support of `collapse_debuginfo` attribute for macros](https://github.com/rust-lang/rust/pull/118903)
* [inline a few utility functions around MIR](https://github.com/rust-lang/rust/pull/119459)
* [llvm: allow `noundef` in codegen tests](https://github.com/rust-lang/rust/pull/119523)
* [make `derive(Trait)` suggestion more accurate](https://github.com/rust-lang/rust/pull/119362)
* [make `named_asm_labels` lint not trigger on unicode and trigger on format args](https://github.com/rust-lang/rust/pull/119195)
* [make inductive cycles in coherence ambiguous always](https://github.com/rust-lang/rust/pull/118649)
* [mark myself as back from leave](https://github.com/rust-lang/rust/pull/119512)
* [migrate memory overlap check from validator to lint](https://github.com/rust-lang/rust/pull/119577)
* [populate `yield` and `resume` types in MIR body while body is being initialized](https://github.com/rust-lang/rust/pull/119666)
* [pretty-print always-const trait predicates correctly](https://github.com/rust-lang/rust/pull/119476)
* [query `panic!()` to useful diagnostic](https://github.com/rust-lang/rust/pull/119086)
* [recover parentheses in range patterns](https://github.com/rust-lang/rust/pull/119397)
* [reland optimized-compiler-builtins config](https://github.com/rust-lang/rust/pull/119556)
* [reorder `check_item_type` diagnostics so they occur next to the corresponding `check_well_formed` diagnostics](https://github.com/rust-lang/rust/pull/117213)
* [replace a number of FxHashMaps/Sets with stable-iteration-order alternatives](https://github.com/rust-lang/rust/pull/119192)
* [separate immediate and in-memory ScalarPair representation](https://github.com/rust-lang/rust/pull/118991)
* [set the `in-rust-tree` feature for all rust-analyzer{-proc-macro-srv} steps](https://github.com/rust-lang/rust/pull/118861)
* [skip threading over no-op SetDiscriminant](https://github.com/rust-lang/rust/pull/119675)
* [stabilize THIR unsafeck](https://github.com/rust-lang/rust/pull/117673)
* [stop feed vis when cant access for trait item](https://github.com/rust-lang/rust/pull/119553)
* [support `~const` in associated functions in trait impls](https://github.com/rust-lang/rust/pull/119705)
* [suppress change-tracker warnings in CI containers](https://github.com/rust-lang/rust/pull/119298)
* [switch from using `//~ERROR` annotations with `--error-format` to `error-pattern`](https://github.com/rust-lang/rust/pull/119184)
* [temporarily disable M1 runners on GitHub Actions](https://github.com/rust-lang/rust/pull/119546)
* [tweak suggestions for bare trait used as a type](https://github.com/rust-lang/rust/pull/119148)
* [use `resolutions(()).effective_visiblities` to avoid cycle errors in `report_object_error`](https://github.com/rust-lang/rust/pull/119506)
* [custom mir: make it clear what the return block is](https://github.com/rust-lang/rust/pull/119325)
* [miri: implement the rounding intrinsics using apfloat rounding](https://github.com/rust-lang/miri/pull/3256)
* [miri: use jemalloc as global allocator](https://github.com/rust-lang/miri/pull/3259)
* [miri: only use jemalloc on Linux and macOS](https://github.com/rust-lang/miri/pull/3261)
* [strip lld-wrapper binaries](https://github.com/rust-lang/rust/pull/119661)
* [two small bitset optimisations](https://github.com/rust-lang/rust/pull/119499)
* [codegen-cranelift: restructure x86 signed pack instructions](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1443)
* [make `intrinsics::assume` const stable](https://github.com/rust-lang/rust/pull/119583)
* [rewrite `Iterator::position` default impl](https://github.com/rust-lang/rust/pull/119599)
* [make `offset_of` field parsing use metavariable which handles any spacing](https://github.com/rust-lang/rust/pull/119532)
* [mark `vec::IntoIter` pointers as `!nonnull`](https://github.com/rust-lang/rust/pull/114205)
* [cargo fix: Call rustc fewer times](https://github.com/rust-lang/cargo/pull/13243)
* [cargo fix: set `OUT_DIR` for all units with build scripts](https://github.com/rust-lang/cargo/pull/13204)
* [cargo cli: add colors to `-Zhelp` console output](https://github.com/rust-lang/cargo/pull/13269)
* [cargo embedded: Add multiple experimental manifest syntaxes](https://github.com/rust-lang/cargo/pull/13241)
* [cargo embedded: Add prefix-char frontmatter syntax support](https://github.com/rust-lang/cargo/pull/13247)
* [cargo manifest: Provide unused key warnings for lints table](https://github.com/rust-lang/cargo/pull/13262)
* [cargo: deprecate rustc plugin support](https://github.com/rust-lang/cargo/pull/13248)
* [cargo: test: support publish package with a `public` field](https://github.com/rust-lang/cargo/pull/13245)
* [rustdoc: Fix invalid handling for static method calls in jump to definition feature](https://github.com/rust-lang/rust/pull/119586)
* [rustdoc ui: adjust tooltip z-index to be above sidebar](https://github.com/rust-lang/rust/pull/119477)
* [rustdoc-search: fix inaccurate type descriptions](https://github.com/rust-lang/rust/pull/119457)
* [rustdoc-search: tighter encoding for f index](https://github.com/rust-lang/rust/pull/119468)
* [rustdoc: search for tuples and unit by type with `()`](https://github.com/rust-lang/rust/pull/118194)
* [rustfmt: fix `enum` variant doc comments wrapped before `comment_width`](https://github.com/rust-lang/rustfmt/pull/6000)
* [clippy: add `.as_ref()` to suggestion to remove `.to_string()`](https://github.com/rust-lang/rust-clippy/pull/12091)
* [clippy: extend `map_clone` lint to also work on non-explicit closures](https://github.com/rust-lang/rust-clippy/pull/12104)
* [clippy: extend `unconditional_recursion` lint to check for `Default` trait implementation](https://github.com/rust-lang/rust-clippy/pull/12090)
* [clippy: do not suggest `[T; n]` instead of `vec![T; n]` if `T` is not `Copy`](https://github.com/rust-lang/rust-clippy/pull/11972)
* [clippy: do not suggest `bool::then()` and `bool::then_some` in `const` contexts](https://github.com/rust-lang/rust-clippy/pull/12108)
* [clippy: don't change eagerness for `struct` literal syntax with significant drop](https://github.com/rust-lang/rust-clippy/pull/12097)
* [clippy: don't emit `struct_field_names` lint if all fields are booleans and don't start with the type's name](https://github.com/rust-lang/rust-clippy/pull/12099)
* [clippy: don't lint `let_unit_value` when `()` is explicit](https://github.com/rust-lang/rust-clippy/pull/10844)
* [clippy: don't look for safety comments in doc tests](https://github.com/rust-lang/rust-clippy/pull/12066)
* [clippy: fix false positive `unconditional_recursion`](https://github.com/rust-lang/rust-clippy/pull/12062)
* [clippy: don't escape `"` in `'"'`](https://github.com/rust-lang/rust-clippy/pull/12030)
* [clippy: fix ICE in `iter_filter_is_some`/`iter_filter_is_ok`](https://github.com/rust-lang/rust-clippy/pull/12080)
* [clippy: allow 3-digit-grouped binary in `non_octal_unix_permissions`](https://github.com/rust-lang/rust-clippy/pull/12049)
* [clippy: fix: metadata-collector lists wrong affected lints](https://github.com/rust-lang/rust-clippy/pull/12088)
* [clippy: `identity_op`: correctly suggest a deference for coerced references](https://github.com/rust-lang/rust-clippy/pull/12056)
* [clippy: handle "calls" inside the closure as well in `map_clone` lint](https://github.com/rust-lang/rust-clippy/pull/12109)
* [clippy: improve `cast_sign_loss` to skip warning on always positive expressions](https://github.com/rust-lang/rust-clippy/pull/11883)
* [clippy: lint nested binary operations and handle field projections in `eager_transmute`](https://github.com/rust-lang/rust-clippy/pull/12031)
* [clippy: new lint: `option_as_ref_cloned`](https://github.com/rust-lang/rust-clippy/pull/12051)
* [rust-analyzer: completion: make the expected type a tad smarter with `Fn`s](https://github.com/rust-lang/rust-analyzer/pull/16136)
* [rust-analyzer: builtin derives are hygienic](https://github.com/rust-lang/rust-analyzer/pull/16308)
* [rust-analyzer: don't trim trailing whitespace from doc comments](https://github.com/rust-lang/rust-analyzer/pull/16081)
* [rust-analyzer: IDE features for primitive tuple fields](https://github.com/rust-lang/rust-analyzer/pull/16279)
* [rust-analyzer: add assoc func quickfix for `unresolved_method` diagnostic](https://github.com/rust-lang/rust-analyzer/pull/16100)
* [rust-analyzer: add inlay hint for exclusive ranges](https://github.com/rust-lang/rust-analyzer/pull/16298)
* [rust-analyzer: add proc-macro rebuild on save option](https://github.com/rust-lang/rust-analyzer/pull/16011)
* [rust-analyzer: add quickfix for `redundant_assoc_item` diagnostic](https://github.com/rust-lang/rust-analyzer/pull/16223)
* [rust-analyzer: add unresolved associated item diagnostic](https://github.com/rust-lang/rust-analyzer/pull/16222)
* [rust-analyzer: resolve inherent and implemented associated items in docs](https://github.com/rust-lang/rust-analyzer/pull/15933)
* [rust-analyzer: `extract_struct_from_enum_variant` assist should resolve Self generic arg](https://github.com/rust-lang/rust-analyzer/pull/16199)
* [rust-analyzer: assists panic when trying to edit usage inside macro](https://github.com/rust-lang/rust-analyzer/pull/15810)
* [rust-analyzer: correct references from `rust-analyzer.cargo.check` to `rust-analyzer.check`](https://github.com/rust-lang/rust-analyzer/pull/16062)
* [rust-analyzer: fix focus range being discarded in attributes/derives when upmapping](https://github.com/rust-lang/rust-analyzer/pull/16234)
* [rust-analyzer: fix panic on unaligned packed attribute](https://github.com/rust-lang/rust-analyzer/pull/16285)
* [rust-analyzer: fix type inference with `IndexMut` returning references](https://github.com/rust-lang/rust-analyzer/pull/16085)
* [rust-analyzer: give a userful error when rustc cannot be found in explicit sysroot](https://github.com/rust-lang/rust-analyzer/pull/16241)
* [rust-analyzer: make callable fields not complete in method access no parens case](https://github.com/rust-lang/rust-analyzer/pull/16049)
* [rust-analyzer: make functions in impl have a container name](https://github.com/rust-lang/rust-analyzer/pull/16139)
* [rust-analyzer: no code action `'introduce_named_generic'` for impl inside types](https://github.com/rust-lang/rust-analyzer/pull/16067)
* [rust-analyzer: notify user that linkedProjects is set when failing to discover projects](https://github.com/rust-lang/rust-analyzer/pull/16153)
* [rust-analyzer: pick up new names when the name conflicts in `'introduce_named_generic'`](https://github.com/rust-lang/rust-analyzer/pull/16068)
* [rust-analyzer: remove completion limit for trait importing method completions](https://github.com/rust-lang/rust-analyzer/pull/16268)
* [rust-analyzer: rewrite `code_action generate_delegate_trait`](https://github.com/rust-lang/rust-analyzer/pull/16112)
* [rust-analyzer: self type replacement in inline-function](https://github.com/rust-lang/rust-analyzer/pull/16114)

### Rust Compiler Performance Triage

Not a particularly notable week. Large swings aren't spurious but also are
driven by changes in high-level behavior (diagnostics going from zero to one
emission primarily), which causes a lot more work to happen. This isn't really
representative of the underlying rustc performance changing though.

Triage done by **@simulacrum**.
Revision range: [67b6975..76101ee](https://perf.rust-lang.org/?start=67b6975051b83ef2bd28f06e8467470d570aceb3&end=76101eecbe9aa80753664bbe637ad06d1925f315&absolute=false&stat=instructions%3Au)

4 Regressions, 4 Improvements, 6 Mixed; 1 of them in rollups
33 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-01-08.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: constants in patterns](https://github.com/rust-lang/rfcs/pull/3535)
* [disposition: merge] [Add RFC combining Infra and Release teams](https://github.com/rust-lang/rfcs/pull/3533)
* [disposition: merge] [RFC: Precise Pre-release `cargo update`](https://github.com/rust-lang/rfcs/pull/3493)
* [disposition: postpone] [[Draft] RFC: Patch dependencies using unidiff patchfiles](https://github.com/rust-lang/rfcs/pull/3177)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [error on incorrect implied bounds in wfcheck except for Bevy dependents](https://github.com/rust-lang/rust/pull/118553)
* [disposition: merge] [Stabilize `slice_first_last_chunk`](https://github.com/rust-lang/rust/pull/117561)
* [disposition: merge] [Warn on references casting to bigger memory layout](https://github.com/rust-lang/rust/pull/118983)
* [disposition: merge] [const-eval interning: get rid of type-driven traversal](https://github.com/rust-lang/rust/pull/119044)
* [disposition: merge] [Tracking Issue for `round_ties_even`](https://github.com/rust-lang/rust/issues/96710)
* [disposition: merge] [Stabilize single-field offset_of](https://github.com/rust-lang/rust/pull/118799)
* [disposition: merge] [revert stabilization of const_intrinsic_copy](https://github.com/rust-lang/rust/pull/117905)
* [disposition: merge] [[rustdoc] Allows links in headings](https://github.com/rust-lang/rust/pull/117662)
* [disposition: merge] [Use version-sorting for all sorting](https://github.com/rust-lang/rust/pull/115046)
* [disposition: merge] [Deny braced macro invocations in let-else](https://github.com/rust-lang/rust/pull/119062)

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Allow symbol re-export in cdylib crate from linked staticlib](https://github.com/rust-lang/rfcs/pull/3556)
* [RFC: cargo-sbom](https://github.com/rust-lang/rfcs/pull/3553)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2024-01-10 - 2024-02-07 🦀

### Virtual

* 2024-01-11 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/297687491/)
* 2024-01-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/295679708/)
* 2024-01-11 | Virtual (San Diego, CA, US) | [San Diego Rust](https://www.meetup.com/san-diego-rust/)
    * [**San Diego Rust January 2024 Tele-Meetup**](https://www.meetup.com/san-diego-rust/events/298441403/)
* 2024-01-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/297128172/)
* 2024-01-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763502/)
* 2024-01-21 | Virtual | [Rust Maven](https://meet-os.com/group/1)
    * [**Web development with Rocket - In English**](https://meet-os.com/event/1)
* 2024-01-23 | Virtual (Halifax, NS, CA) | [Rust Halifax](https://www.meetup.com/rust-tell-halifax/)
    * [**Rust&Tell - Halifax**](https://www.meetup.com/rust-tell-halifax/events/298011202/)
* 2024-01-24 | Virtual (Berlin, DE) | [WeAreDevelopers Community](https://www.meetup.com/wearedevelopers-community/)
    * [**WeAreDevelopers LIVE - Rust Day**](https://www.meetup.com/wearedevelopers-community/events/297065638/)
* 2024-01-25 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298058222/)
* 2024-01-25 | Virtual (Mexico City, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Iniciando 2024 con Rust**](https://www.meetup.com/rust-mx/events/298439198/)
* 2024-01-28 | Virtual (Wrocław, PL) | [Stacja IT Wrocław](https://www.meetup.com/stacja-it-wroclaw/)
    * [**Wprowadzenie do języka Rust**](https://www.meetup.com/stacja-it-wroclaw/events/297899705/)
* 2024-01-30 | Virtual | [Bevy Game Development](https://www.meetup.com/bevy-game-development/)
    * [**Bevy Meetup #1**](https://www.meetup.com/bevy-game-development/events/298399958/)
* 2024-01-30 | Virtual (Buffalo, NY, US) | [Buffalo Rust User Group](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/297965826/)
* 2024-01-30 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtygccbnc/)
* 2024-02-01 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/)
    * [**Rust Hack n Learn**](https://meet.jit.si/RustHackAndLearnBerlin)
* 2024-02-03 | Virtual + In-person (Brussels, BE) | [FOSDEM 2024](https://fosdem.org/2024/)
    * [**FOSDEM Conference: Rust devroom - talks**](https://fosdem.org/2024/schedule/track/rust/)
* 2024-02-03 | Virtual (Kampala, UG) | [Rust Circle](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdsoporgprofile)
* 2024-02-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftygcdbkb/)
* 2024-02-08 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298251149/)

### Europe

* 2024-01-10 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/)
    * [**Game development in Rust**](https://www.meetup.com/rustcologne/events/298303772/)
* 2024-01-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/296020357/)
* 2024-01-11 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Meetup #36**](https://www.meetup.com/rust-wroclaw/events/298029291/)
* 2024-01-13 | Tampere, FI | [Finland Rust-lang Group](https://www.meetup.com/finland-rust-meetup/)
    * [**January Meetup**](https://www.meetup.com/finland-rust-meetup/events/297811750/)
* 2024-01-16 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Async in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/297376712/)
* 2024-01-17 | Girona, ES | [Rust Girona](https://www.meetup.com/rust-girona/)
    * [**Que és Rust i els seus beneficis / What's Rust and its advantages**](https://www.meetup.com/rust-girona/events/294080437/)
* 2024-01-17 | Praha / Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Reloaded 2024**](https://www.meetup.com/rust-prague/events/298005196/) 
* 2024-01-17 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**How to Community - January Meetup**](https://www.meetup.com/rust-zurich/events/298066842/)
* 2024-01-23 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack and Learn**](https://www.meetup.com/rust-aarhus/events/297463730/)
* 2024-01-23 | Paris, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [**Paris Rust Meetup #64**](https://mobilizon.fr/events/0fce31cd-3578-43f2-abf4-ffecd8d16da2)
* 2024-02-01 | Barcelona, ES | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**12th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/297439924/)
* 2024-02-03 | Brussels, BE | [FOSDEM '24](https://fosdem.org/2024/)
    * [**FOSDEM '24 Conference: Rust devroom - talks**](https://fosdem.org/2024/schedule/track/rust/) | [**Rust Aarhus FOSDEM Meetup**](https://www.meetup.com/rust-aarhus/events/295946777/)

### North America

* 2024-01-11 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Arena Allocation: Another approach to managing lifetimes w/Taylor Allred**](https://www.meetup.com/utah-rust/events/298448713/)
* 2024-01-14 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Alewife Rust Lunch**](https://www.meetup.com/bostonrust/events/297634920/)
* 2024-01-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/297452643/)
* 2024-01-17 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Happy Hour**](https://www.meetup.com/deep-dish-rust/events/298003233/)
* 2024-01-18 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/298304117/)
* 2024-01-22 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**North End Rust Lunch**](https://www.meetup.com/bostonrust/events/297634962/)
* 2024-01-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygccbgc/)
* 2024-01-27-28 | Calgary, AB, CA | [Rust Calgary](https://www.eventbrite.ca/o/rust-calgary-63449860593)
    * [**Harnessing Rust for Real-World Problems hackathon: Day 1**](https://www.eventbrite.ca/e/harnessing-rust-for-real-world-problems-hackathon-day-1-tickets-794992302377?aff=ebdsoporgprofile)
    * [**Harnessing Rust for Real-World Problems hackathon: Day 2**](https://www.eventbrite.ca/e/harnessing-rust-for-real-world-problems-hackathon-day-2-tickets-794994147897?aff=ebdsoporgprofile)  
* 2024-01-30 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Harvard Square Rust Lunch**](https://www.meetup.com/bostonrust/events/297634994/)
* 2024-02-07 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, Feb 7**](https://www.meetup.com/bostonrust/events/297635028/)

### Oceania

* 2024-01-16 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/298382221/)
* 2024-02-06 | Perth, WA, AU | [Perth Rust Meetup Group](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Rust Feb 2024 Meetup**](https://www.meetup.com/perth-rust-meetup-group/events/297330668/)

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

> * Modular
> * Very high quality construction compared to its competitors
> * If you leave it lying around forget about it, stepping into a project is painful?

– [Leonardo Giovanni Scur on mastodon](https://floss.social/@kroltan@functional.cafe/111687927473117112) explaining how [bevy](https://bevyengine.org) is like Lego™

Thanks to [Jan Riemer](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1506) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
