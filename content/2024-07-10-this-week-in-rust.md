Title: This Week in Rust 555
Number: 555
Date: 2024-07-10
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://x.com/ThisWeekInRust) on X(formerly Twitter) or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

### Newsletters
* [Bevy 0.14's Release, Cosmic Text, and water reflections](https://thisweekinbevy.com/issue/2024-07-08-bevy-014s-release-cosmic-text-and-water-reflections)

### Project/Tooling Updates
* [Maelstrom v0.10.0: A Hermetic, Fast, Clustered Test Runner: Added Support for Python Tests, Other Container Image Repositores, and more](https://github.com/maelstrom-software/maelstrom/releases/tag/v0.10.0)
* [Rust for filesystems](https://lwn.net/Articles/978738/)
* [Bevy 0.14](https://bevyengine.org/news/bevy-0-14/)
* [Introducing Avian Physics 0.1](https://joonaa.dev/blog/06/avian-0-1)
* [iroh 0.20.0](https://iroh.computer/blog/iroh-0-20-more-ways-to-connect)
* [Release Nutype 0.4.3](https://github.com/greyblake/nutype/releases/tag/v0.4.3)
* [Rerun 0.17 - better blueprints with defaults and overrides for any data](https://rerun.io/blog/blueprint-overrides)

### Observations/Thoughts
* [Rustic Witcher: Reimagining data anonymization](https://engineering.theblueground.com/rustic-witcher-reimagining-data-anonymization/)
* [Memory Safety in C++ vs Rust vs Zig](https://medium.com/@shyamsundarb/memory-safety-in-c-vs-rust-vs-zig-f78fa903f41e)
* [Using unsafe in our Rust interpreters: easy, debatably ethical performance](https://octavelarose.github.io/2024/07/08/unsafeing.html)
* [How to configure CPU cores to be used in a Tokio application with core_affinity](https://blog.veeso.dev/blog/en/how-to-configure-cpu-cores-to-be-used-on-a-tokio-with-core--affinity/)
* [Further simplifying self-referential types for Rust](https://blog.yoshuawuyts.com/self-referential-types-2/)
* [Network Manager and Rust's zbus](https://rbs.io/2024/07/network-manager-and-rusts-zbus/)
* [video] [PyO3: From Python to Rust and Back Again](https://www.youtube.com/watch?v=UmL_CA-v3O8)
* [Properly Testing Concurrent Data Structures](https://matklad.github.io/2024/07/05/properly-testing-concurrent-data-structures.html)
* [A Unified Typesystem](https://gist.github.com/Aras14HD/f96cebf827975ba51852b1b981c389ab)

### Rust Walkthroughs
* [Writing Production Rust Macros with `macro_rules!`](https://www.howtocodeit.com/articles/writing-production-rust-macros-with-macro-rules)
* [Mix in Rust](https://tweedegolf.nl/en/blog/123/mix-in-rust)
* [Demystifying Rust's HTTP ecosystem: Here is how the different crates fit together](https://kerkour.com/rust-http-ecosystem-2024)
* [Build with Naz : Rust async in practice tokio::select! & cancellation safety](https://developerlife.com/2024/07/10/rust-async-cancellation-safety-tokio/)

### Miscellaneous
* [June 2024 Rust Jobs Report](https://filtra.io/rust-jun-24)
* [video] [Jan Hohenheim discusses Rust, Rust GameDev Newsletter & The Bevy Engine](https://www.youtube.com/watch?v=pLcIY0TlOLk)

## Crate of the Week

This week's crate is [derive_deftly](https://docs.rs/derive-deftly/latest/derive_deftly/), a proc macro to create derive macros from declarative code.

Thanks to [duelafn](https://users.rust-lang.org/t/crate-of-the-week/2704/1319) for the suggestion!

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
## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

*No Calls for participation were submitted this week.*

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

*No Calls for papers or presentations were submitted this week.*

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

469 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-07-02..2024-07-09

* [wasm64 build with target-feature=+simd128,+atomics](https://github.com/rust-lang/rust/pull/126792)
* [`macro_metavar_expr_concat` Add support for literals](https://github.com/rust-lang/rust/pull/126841)
* [`out_of_scope_macro_calls`: Detect calls inside attributes more precisely](https://github.com/rust-lang/rust/pull/126987)
* [actually report normalization-based type errors correctly for alias-relate obligations in new solver](https://github.com/rust-lang/rust/pull/126403)
* [add `as_lang_item` to `LanguageItems`, new trait solver](https://github.com/rust-lang/rust/pull/127145)
* [change `asm-comments` to `verbose-asm`, always emit user comments](https://github.com/rust-lang/rust/pull/126803)
* [change return-type-notation to use `(..)`](https://github.com/rust-lang/rust/pull/127092)
* [check alias args for WF even if they have escaping bound vars](https://github.com/rust-lang/rust/pull/123737)
* [coverage: extract hole spans from HIR instead of MIR](https://github.com/rust-lang/rust/pull/127199)
* [coverage: rename `mir::coverage::BranchInfo` to `CoverageInfoHi`](https://github.com/rust-lang/rust/pull/127352)
* [disable dead variant removal for `#[repr(C)]` enums](https://github.com/rust-lang/rust/pull/123043)
* [emit a wrap expr `span_bug` only if context is not tainted](https://github.com/rust-lang/rust/pull/127409)
* [fix `FnMut::call_mut`/`Fn::call` shim for async closures that capture references](https://github.com/rust-lang/rust/pull/127136)
* [fix import suggestion error when path segment failed not from starting](https://github.com/rust-lang/rust/pull/127203)
* [fix incorrect suggestion for extra argument with a type error](https://github.com/rust-lang/rust/pull/127253)
* [fix intrinsic const parameter counting with `effects`](https://github.com/rust-lang/rust/pull/127452)
* [improve dead code analysis](https://github.com/rust-lang/rust/pull/127107)
* [improve well known value check-cfg diagnostic for the standard library](https://github.com/rust-lang/rust/pull/127221)
* [infer async closure signature from (old-style) two-part `Fn` + `Future` bounds](https://github.com/rust-lang/rust/pull/127482)
* [linker: link dylib crates by path](https://github.com/rust-lang/rust/pull/126094)
* [make `FloatTy` checks exhaustive in pretty print](https://github.com/rust-lang/rust/pull/127224)
* [make `NEVER_TYPE_FALLBACK_FLOWING_INTO_UNSAFE` a deny-by-default lint in edition 2024](https://github.com/rust-lang/rust/pull/126881)
* [make `can_eq` process obligations (almost) everywhere](https://github.com/rust-lang/rust/pull/127172)
* [make `push_outlives_components` into a `TypeVisitor`](https://github.com/rust-lang/rust/pull/127438)
* [make casts of pointers to trait objects stricter](https://github.com/rust-lang/rust/pull/120248)
* [match ergonomics 2024: Implement TC's match ergonomics proposal](https://github.com/rust-lang/rust/pull/127008)
* [match ergonomics 2024: align with RFC again](https://github.com/rust-lang/rust/pull/127369)
* [parenthesize break values containing leading label](https://github.com/rust-lang/rust/pull/126883)
* [re-implement a type-size based limit](https://github.com/rust-lang/rust/pull/125507)
* [remove a use of `StructuredDiag`, which is incompatible with automatic error tainting and error translations](https://github.com/rust-lang/rust/pull/127319)
* [remove global error count checks from typeck](https://github.com/rust-lang/rust/pull/127202)
* [rewrite handling of universe-leaking placeholder regions into outlives constraints](https://github.com/rust-lang/rust/pull/123720)
* [rustdoc-json: better representation of lifetime bounds in where clauses](https://github.com/rust-lang/rust/pull/127289)
* [rustdoc-search: stop constructing pointless arrays in decode](https://github.com/rust-lang/rust/pull/127379)
* [show fnsig's unit output explicitly when there is output diff in diagnostics](https://github.com/rust-lang/rust/pull/127417)
* [stop using specialization in `rustc_index` and `rustc_borrowck`](https://github.com/rust-lang/rust/pull/127170)
* [tweak `-1 as usize` suggestion](https://github.com/rust-lang/rust/pull/127349)
* [use `ControlFlow` results for visitors that are only looking for a single value](https://github.com/rust-lang/rust/pull/127366)
* [use `IndexVec` for coroutine local mapping](https://github.com/rust-lang/rust/pull/127293)
* [use field ident spans directly instead of the full field span in diagnostics on local fields](https://github.com/rust-lang/rust/pull/127431)
* [use the aligned size for alloca at args/ret when the pass mode is cast](https://github.com/rust-lang/rust/pull/127168)
* [tweak some structured suggestions to be more verbose and accurate](https://github.com/rust-lang/rust/pull/127301)
* [use verbose style for argument removal suggestion](https://github.com/rust-lang/rust/pull/127383)
* [use verbose suggestion for `ptr::null_mut()`](https://github.com/rust-lang/rust/pull/127391)
* [use verbose suggestion for changing arg type](https://github.com/rust-lang/rust/pull/127392)
* [verify that allocations output by GVN are sufficiently aligned](https://github.com/rust-lang/rust/pull/127399)
* [support tail calls in mir via `TerminatorKind::TailCall`](https://github.com/rust-lang/rust/pull/113128)
* [use the native unwind function in miri where possible](https://github.com/rust-lang/rust/pull/127214)
* [miri function identity hack: account for possible inlining](https://github.com/rust-lang/rust/pull/123781)
* [miri: TB: refine protector end semantics](https://github.com/rust-lang/miri/pull/3732)
* [miri: add syscall `dup()` for unix target](https://github.com/rust-lang/miri/pull/3707)
* [miri: implement `libc::sched_setaffinity` on linux](https://github.com/rust-lang/miri/pull/3698)
* [miri: implement the `_mm256_zeroupper` and `_mm256_zeroall` intrinsics](https://github.com/rust-lang/miri/pull/3726)
* [miri: stacked Borrows: fix PartialEq for Stack](https://github.com/rust-lang/miri/pull/3738)
* [miri: use the `symbol_name` query instead of trying to infer from the `link_name` attribute](https://github.com/rust-lang/miri/pull/3724)
* [cache `hir_owner_nodes` in ParentHirIterator](https://github.com/rust-lang/rust/pull/127421)
* [cache type sizes in type-size limit visitor](https://github.com/rust-lang/rust/pull/127288)
* [make jump threading state sparse](https://github.com/rust-lang/rust/pull/127036)
* [stabilize `atomic_bool_fetch_not`](https://github.com/rust-lang/rust/pull/127204)
* [stabilize `hint::assert_unchecked`](https://github.com/rust-lang/rust/pull/123588)
* [add `new_range_api` for](https://github.com/rust-lang/rust/pull/125751) RFC [#3550](https://rust-lang.github.io/rfcs/3550-new-range.html)
* [impl `PathBuf::add_extension` and `Path::with_added_extension`](https://github.com/rust-lang/rust/pull/123600)
* [improve `std::Path's` Hash quality by avoiding prefix collisions](https://github.com/rust-lang/rust/pull/127297)
* [specialize `TrustedLen` for `Iterator::unzip()`](https://github.com/rust-lang/rust/pull/123253)
* [linkedList's Cursor: method to get a ref to the cursor's list](https://github.com/rust-lang/rust/pull/127189)
* [mark format! with `must_use` hint](https://github.com/rust-lang/rust/pull/127355)
* [optimize SipHash by reordering compress instructions](https://github.com/rust-lang/rust/pull/127226)
* [cargo: add rustdocflags to Unit's Debug impl](https://github.com/rust-lang/cargo/pull/14201)
* [cargo: allow enabling `config-include` feature in config](https://github.com/rust-lang/cargo/pull/14196)
* [cargo: dont make new constant `InternedString` in hot path](https://github.com/rust-lang/cargo/pull/14211)
* [cargo: fix `compatible_with_older_cargo` test](https://github.com/rust-lang/cargo/pull/14212)
* [cargo: fix: improve message for inactive weak optional feature with edition2024 through unused dep collection](https://github.com/rust-lang/cargo/pull/14026)
* [cargo: pass rustflags to artifacts built with implicit targets when using target-applies-to-host](https://github.com/rust-lang/cargo/pull/13900)
* [bindgen: fix generated constants: `f64::INFINITY` & `f64::NEG_INFINITY`](https://github.com/rust-lang/rust-bindgen/pull/2854)
* [jsondocck: add `$FILE` built-in variable](https://github.com/rust-lang/rust/pull/127309)
* [jsondocck: use correct index for error message](https://github.com/rust-lang/rust/pull/127287)
* [clippy: `almost_complete_range`: Delay suggestion creation](https://github.com/rust-lang/rust-clippy/pull/13042)
* [clippy: `doc_markdown`: detect escaped ```  ``` when checking unmatched](https://github.com/rust-lang/rust-clippy/pull/13010)
* [clippy: `missing_const_for_fn`: fix FP when arg ty is impl trait alias ty](https://github.com/rust-lang/rust-clippy/pull/13045)
* [clippy: `missing_const_for_fn`: fix suggestions for fn with abi that requires `const_extern_fn` feature](https://github.com/rust-lang/rust-clippy/pull/13037)
* [clippy: `needless_return`: Support `#[expect]` on the return statement](https://github.com/rust-lang/rust-clippy/pull/13027)
* [clippy: `significant_drop_in_scrutinee`: Trigger lint also for scrutinees in `while let` and `if let`](https://github.com/rust-lang/rust-clippy/pull/12870)
* [clippy: `unnecessary_to_owned`: catch `to_owned` on byte slice to create temporary `&str`](https://github.com/rust-lang/rust-clippy/pull/11656)
* [clippy: add new lint `hashset_insert_after_contains`](https://github.com/rust-lang/rust-clippy/pull/12873)
* [clippy: add new lint for byte char slices](https://github.com/rust-lang/rust-clippy/pull/10155)
* [clippy: add `cfg_not_test` lint](https://github.com/rust-lang/rust-clippy/pull/11293)
* [clippy: fix false positive with `into_iter_without_iter`](https://github.com/rust-lang/rust-clippy/pull/13030)
* [clippy: fix some false-positive cases of `explicit_auto_deref`](https://github.com/rust-lang/rust-clippy/pull/12976)
* [clippy: honor `avoid-breaking-exported-api` in `needless_pass_by_ref_mut`](https://github.com/rust-lang/rust-clippy/pull/11647)
* [clippy: only check for `automatically_derived` on impl blocks](https://github.com/rust-lang/rust-clippy/pull/13055)
* [clippy: refactor `disallowed_methods` and narrow span](https://github.com/rust-lang/rust-clippy/pull/13048)
* [clippy: rework `init_numbered_fields`](https://github.com/rust-lang/rust-clippy/pull/13068)
* [rust-analyzer: add `--keep-going` to the check command](https://github.com/rust-lang/rust-analyzer/pull/17561)
* [rust-analyzer: add an option to use `"::"` for the external crate prefix](https://github.com/rust-lang/rust-analyzer/pull/17523)
* [rust-analyzer: also mark `InferenceResult::has_errors` flag when there are error types](https://github.com/rust-lang/rust-analyzer/pull/17551)
* [rust-analyzer: disallow nested impl traits](https://github.com/rust-lang/rust-analyzer/pull/17541)
* [rust-analyzer: don't emit semantic diagnostics in files with a lot of syntax errors](https://github.com/rust-lang/rust-analyzer/pull/17536)
* [rust-analyzer: fix callHierarchy LSP violation](https://github.com/rust-lang/rust-analyzer/pull/17554)
* [rust-analyzer: fix double rounding of `f32` literals](https://github.com/rust-lang/rust-analyzer/pull/17558)
* [rust-analyzer: fix lifetime parameters moving parameter defaults](https://github.com/rust-lang/rust-analyzer/pull/17529)
* [rust-analyzer: fix parameter completions using macro expanded source ranges](https://github.com/rust-lang/rust-analyzer/pull/17552)
* [rust-analyzer: fix passing `message-format` after -- in debugging](https://github.com/rust-lang/rust-analyzer/pull/17548)
* [rust-analyzer: fix runnables being incorrectly constructed](https://github.com/rust-lang/rust-analyzer/pull/17549)
* [rust-analyzer: fix up the syntax tree for macro 2.0](https://github.com/rust-lang/rust-analyzer/pull/17535)
* [rust-analyzer: skip match exhaustiveness checking if pattern type contains errors](https://github.com/rust-lang/rust-analyzer/pull/17534)
* [rust-analyzer: move lifetimes in front of type and const params but after self](https://github.com/rust-lang/rust-analyzer/pull/17530)
* [rust-analyzer: remove version check before using `--keep-going`](https://github.com/rust-lang/rust-analyzer/pull/17565)

### Rust Compiler Performance Triage

More regressions than improvements this week, caused by a combination of fixes,
refactorings, third-party dependency updates and in general the compiler doing
slightly more work.

Triage done by **@kobzol**.
Revision
range: [cf2df68d..a2d58197](https://perf.rust-lang.org/?start=cf2df68d1f5e56803c97d91e2b1a9f1c9923c533&end=a2d58197a766085856504328948c89a33a6a36e8&absolute=false&stat=instructions%3Au)

**Summary**:

|         (instructions:u)          | mean  |     range      | count |
|:---------------------------------:|:-----:|:--------------:|:-----:|
|  Regressions ❌ <br /> (primary)   | 0.7%  |  [0.2%, 2.5%]  |  53   |
| Regressions ❌ <br /> (secondary)  | 1.0%  |  [0.4%, 1.5%]  |  31   |
|  Improvements ✅ <br /> (primary)  | -0.6% | [-1.2%, -0.2%] |  10   |
| Improvements ✅ <br /> (secondary) | -1.7% | [-2.4%, -1.4%] |   4   |
|         All ❌✅ (primary)          | 0.5%  | [-1.2%, 2.5%]  |  63   |

1 Regression, 2 Improvements, 7 Mixed; 3 of them in rollups
62 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/b58ff6d177b00e21d8ac6e08b8d621632adb14e4/triage/2024-07-09.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:
* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [`repr(discriminant = ...)` for type aliases](https://github.com/rust-lang/rfcs/pull/3659)
* [disposition: merge] [Match ergonomics 2024](https://github.com/rust-lang/rfcs/pull/3627)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Tracking Issue for Missing BMI1, AVX2, SSE2, SSE4.1, SSE4a and TBM intrinsics](https://github.com/rust-lang/rust/issues/126936)
* [disposition: merge] [offset_from: always allow pointers to point to the same address](https://github.com/rust-lang/rust/pull/124921)
* [disposition: merge] [Fix ambiguous cases of multiple & in elided self lifetimes](https://github.com/rust-lang/rust/pull/117967)
* [disposition: merge] [Tracking issue for RFC 2351, "Add is_sorted to the standard library"](https://github.com/rust-lang/rust/issues/53485)
* [disposition: merge] [Tracking issue for io_slice_advance](https://github.com/rust-lang/rust/issues/62726)
* [disposition: merge] [Tracking Issue for `const_waker`](https://github.com/rust-lang/rust/issues/102012)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: (unspecified)] [elaborate on slice wide pointer metadata](https://github.com/rust-lang/reference/pull/1499)

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Promote aarch64-apple-darwin to Tier 1](https://github.com/rust-lang/rfcs/pull/3671)

## Upcoming Events

Rusty Events between 2024-07-10 - 2024-08-07 🦀

### Virtual
* 2024-07-10 | Virtual | [Centre for eResearch](https://www.eventbrite.co.nz/o/centre-for-eresearch-75893560993)
    * [**Research Computing With The Rust Programming Language**](https://www.eventbrite.com/e/research-computing-with-the-rust-programming-language-tickets-908002037537?aff=ebdssbdestsearch&keep_tld=1)
* 2024-07-11 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897842/)
* 2024-07-11 | Hybrid - Virtual and In-person (Mexico City, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Programación de sistemas con Rust**](https://www.meetup.com/rust-mx/events/301740677/)
* 2024-07-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/298076822/)
* 2024-07-11 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Reading JSON files in Rust (English)**](https://www.meetup.com/code-mavens/events/301636580/)
* 2024-07-11 | Virtual (IL) | [Rust in Israel](https://www.meetup.com/rust-in-israel/)
    * [**Getting started with Rust (Virtual) - מבוא לתכנות ראסט - בזום**](https://www.meetup.com/rust-in-israel/events/301872689/)
* 2024-07-17 | Hybrid - Virtual and In-person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631734/)
* 2024-07-18 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298488824/)
* 2024-07-23 | Hybrid - Virtual and In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/301062840/)
* 2024-07-24 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Lunch & Learn: Exploring Rust API Use Cases**](https://www.meetup.com/women-in-rust/events/301730780/)
* 2024-07-25 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897865/)
* 2024-07-27 | Hybrid - Virtual and In-Person (Kyiv, UA) | [UA Rust](https://uarust.com/)
    * [**UARust Conference 2024**](https://uarust.com/)
* 2024-07-27 | Virtual | [Leptos Monthly Meetup](https://lu.ma/user/leptos)
    * [**Leptos Monthly Meetup: Pavex with Luca Palmieri**](https://lu.ma/3ouqapsr)
* 2024-07-30 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/301585665/)
* 2024-07-31 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Command Line Tools: Implementing wc in Rust (English)**](https://www.meetup.com/code-mavens/events/302151487/)
* 2024-08-01 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633265/)
* 2024-08-06 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Lunch & Learn! (Virtual)**](https://www.meetup.com/women-in-rust/events/300994574/)
* 2024-08-06 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300191718/)
* 2024-08-06 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Web development in Rust using Rocket - part 2 (English)**](https://www.meetup.com/code-mavens/events/301736709/)
* 2024-08-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/300328027/)

### Africa
* 2024-08-02 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)

### Asia
* 2024-07-20 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**July 2024 Rustacean meetup 🤝 C4GT**](https://hasgeek.com/rustbangalore/july-2024-rustacean-meetup-c4gt/)

### Europe
* 2024-07-10 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup - July**](https://www.meetup.com/reading-rust-workshop/events/301359031/)
* 2024-07-11 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Prague (July 2024)**](https://www.meetup.com/rust-prague/events/301227195)
* 2024-07-16 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Building a REST API in Rust using Axum, SQLx and SQLite**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/301716171/)
* 2024-07-16 | Mannheim, DE | [Hackschool - Rhein-Neckar](https://www.meetup.com/hackschool-rhein-neckar)
    * [**Nix Your Bugs & Rust Your Engines #4**](https://www.meetup.com/hackschool-rhein-neckar/events/301504325/)
* 2024-07-18 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**Rust Bern Meetup #3 2024**](https://www.meetup.com/rust-bern/events/301952761/)
* 2024-07-23 | Hybrid - Virtual and In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-07-25 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/de-DE/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #8**](https://www.meetup.com/rust-meetup-augsburg/events/301642385/)
* 2024-07-25 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299288967/)
* 2024-07-27 | Hybrid - Virtual and In-Person (Kyiv, UA) | [UA Rust](https://uarust.com/)
    * [**UARust Conference 2024**](https://uarust.com/)
* 2024-07-30 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #9**](https://www.meetup.com/rust-basel/events/301459503/)

### North America
* 2024-07-11 | Hybrid - Mexico City, DF, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Programación de sistemas con Rust**](https://www.meetup.com/rust-mx/events/301740677/)
* 2024-07-11 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/301613495/)
* 2024-07-13 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Lechmere Rust Lunch, July 13**](https://www.meetup.com/bostonrust/events/301549799/)
* 2024-07-17 | Hybrid - Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631734/)
* 2024-07-18 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers : holding pattern**](https://www.meetup.com/music-city-rust-developers/events/301411794/)
* 2024-07-18 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/301883176/)
* 2024-07-21 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Back Bay Rust Lunch, July 21**](https://www.meetup.com/bostonrust/events/301550076/)
* 2024-07-24 | Austin, TX, US | [Rust ATC](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygckbgc/)
* 2024-07-25 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/302066981/)
* 2024-07-29 | Cambridge, MA, US| [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Alewife Rust Lunch, July 29**](https://www.meetup.com/bostonrust/events/301550289/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1dvlhl6/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> In Rust, the preferred solution is to avoid the need for such document to exist.

– [Kornel on rust-users replying to a question about Rust code guidelines](https://users.rust-lang.org/t/is-there-something-like-rust-core-guidelines-like-c-core-guidelines/113850/3)

Thanks to [Chayim Refael Friedman](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1590?u=llogiq) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
