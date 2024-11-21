Title: This Week in Rust 574
Number: 574
Date: 2024-11-20
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
* [ Streaming data analytics, Fluvio 0.13.0 release](https://www.fluvio.io/news/this-week-in-fluvio-0066)

* [Zed Decoded: Rope Optimizations](https://zed.dev/blog/zed-decoded-rope-optimizations-part-1)
* [Rerun 0.20 - Geospatial data and full H.264 support](https://rerun.io/blog/maps)

### Observations/Thoughts
* [The fastest WASM zlib](https://trifectatech.org/blog/fastest-wasm-zlib/)

* [You don't (always) need async](https://blog.veeso.dev/blog/en/you-dont-always-need-async/)

### Rust Walkthroughs

* [Traits to Unify all Vectors](https://orxfun.github.io/orxfun-notes/#/v-for-vectors-2024-11-18)
* [Basics of Pinning in Rust](https://garden.christophertee.dev/tech/rust/Pinning)
* [Building a Wifi-controlled car with Rust and ESP32](https://jamesmcm.github.io/blog/esp32-wifi-tank/)
* [video] [Build with Naz : Diesel ORM, SQLite and Rust](https://www.youtube.com/watch?v=d9x_5X9R5LI)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [fixed-slice-vec](https://crates.io/crates/fixed-slice-vec), a no-std dynamic length Vec with runtime-determined maximum capacity backed by a slice.

Thanks to [Jay Oster](https://users.rust-lang.org/t/crate-of-the-week/2704/1376) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No calls for testing were issued this week.*
  - [Testing Steps](https://github.com/rust-lang/cargo/issues/13873)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No calls for testing were issued this week.*
  - [Testing steps]()

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No calls for testing were issued this week.*
  - [Testing steps]()

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

480 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-11-12..2024-11-19

* [ABI checks: add support for some tier3 arches, warn on others](https://github.com/rust-lang/rust/pull/133029)
* [ABI checks: add support for tier2 arches](https://github.com/rust-lang/rust/pull/132842)
* [CFI: append debug location to CFI blocks](https://github.com/rust-lang/rust/pull/132702)
* [AIX: Add crate "unwind" to link with libunwind](https://github.com/rust-lang/rust/pull/132905)
* [illumos: use pipe2 to create anonymous pipes](https://github.com/rust-lang/rust/pull/132984)
* [`check_consts`: fix error requesting feature gate when that gate is not actually needed](https://github.com/rust-lang/rust/pull/132992)
* [`const_panic`: inline in bootstrap builds to avoid f16/f128 crashes](https://github.com/rust-lang/rust/pull/133182)
* [`rustc_metadata`: Preprocess search paths for better performance](https://github.com/rust-lang/rust/pull/132910)
* [`suggest_borrow_generic_arg`: instantiate clauses properly](https://github.com/rust-lang/rust/pull/133130)
* [add `visit_coroutine_kind` to `ast::Visitor`](https://github.com/rust-lang/rust/pull/132956)
* [add parentheses when unboxing suggestion needed](https://github.com/rust-lang/rust/pull/132944)
* [add reference annotations for diagnostic attributes](https://github.com/rust-lang/rust/pull/133187)
* [allow CFGuard on windows-gnullvm](https://github.com/rust-lang/rust/pull/132965)
* [always inline functions signatures containing `f16` or `f128`](https://github.com/rust-lang/rust/pull/133050)
* [borrowck diagnostics: suggest borrowing function inputs in generic positions](https://github.com/rust-lang/rust/pull/132172)
* [change `Visitor::visit_precise_capturing_arg` so it returns a `Visitor::Result`](https://github.com/rust-lang/rust/pull/133049)
* [change intrinsic declarations to new style](https://github.com/rust-lang/rust/pull/132907)
* [check `use<..>` in RPITIT for refinement](https://github.com/rust-lang/rust/pull/132795)
* [consolidate type system const evaluation under `traits::evaluate_const`](https://github.com/rust-lang/rust/pull/132927)
* [delete the `cfg(not(parallel))` serial compiler](https://github.com/rust-lang/rust/pull/132282)
* [deny capturing late-bound ty/const params in nested opaques](https://github.com/rust-lang/rust/pull/132832)
* [diagnostics for let mut in item context](https://github.com/rust-lang/rust/pull/133143)
* [extend the "if-unchanged" logic for compiler builds](https://github.com/rust-lang/rust/pull/131831)
* [feature gate yield expressions not in 2024](https://github.com/rust-lang/rust/pull/132668)
* [fix ICE when passing DefId-creating args to `legacy_const_generics`](https://github.com/rust-lang/rust/pull/130443)
* [fix `REGISTRY_USERNAME` to reuse cache between auto and pr jobs](https://github.com/rust-lang/rust/pull/132967)
* [fix a copy-paste issue in the NuttX raw type definition](https://github.com/rust-lang/rust/pull/133027)
* [fix compilation error on Solaris due to flock usage](https://github.com/rust-lang/rust/pull/132977)
* [fix span edition for 2024 RPIT coming from an external macro](https://github.com/rust-lang/rust/pull/133080)
* [for expr `return (_ = 42); unused_paren` lint should not be triggered](https://github.com/rust-lang/rust/pull/132936)
* [handle infer vars in anon consts on stable](https://github.com/rust-lang/rust/pull/132971)
* [improve VecCache under parallel frontend](https://github.com/rust-lang/rust/pull/124780)
* [increase accuracy of `if` condition misparse suggestion](https://github.com/rust-lang/rust/pull/133051)
* [liberate `aarch64-gnu-debug` from the shackles of `--test-args=clang`](https://github.com/rust-lang/rust/pull/132646)
* [likely unlikely fix](https://github.com/rust-lang/rust/pull/120370)
* [make precise capturing suggestion machine-applicable only if it has no APITs](https://github.com/rust-lang/rust/pull/132938)
* [make sure to ignore elided lifetimes when pointing at args for fulfillment errors](https://github.com/rust-lang/rust/pull/132935)
* [mention both release *and* edition breakage for never type lints](https://github.com/rust-lang/rust/pull/132978)
* [move all mono-time checks into their own folder, and their own query](https://github.com/rust-lang/rust/pull/132843)
* [proper support for cross-crate recursive const stability checks](https://github.com/rust-lang/rust/pull/132541)
* [querify MonoItem collection](https://github.com/rust-lang/rust/pull/132566)
* [recurse into APITs in `impl_trait_overcaptures`](https://github.com/rust-lang/rust/pull/132817)
* [refactor `configure_annotatable`](https://github.com/rust-lang/rust/pull/133021)
* [remove attributes from generics in built-in derive macros](https://github.com/rust-lang/rust/pull/132651)
* [rename `rustc_const_stable_intrinsic` → `rustc_intrinsic_const_stable_indirect`](https://github.com/rust-lang/rust/pull/133142)
* [skip locking span interner for some syntax context checks](https://github.com/rust-lang/rust/pull/128197)
* [trim extra space when suggesting removing bad `let`](https://github.com/rust-lang/rust/pull/132996)
* [trim whitespace in RemoveLet primary span](https://github.com/rust-lang/rust/pull/133060)
* [tweak attributes for const panic macro](https://github.com/rust-lang/rust/pull/132662)
* [unify FnKind between AST visitors and make WalkItemKind more straight forward](https://github.com/rust-lang/rust/pull/132787)
* [use `TypingMode` throughout the compiler instead of `ParamEnv`](https://github.com/rust-lang/rust/pull/132460)
* [warn about invalid `mir-enable-passes` pass names](https://github.com/rust-lang/rust/pull/132901)
* [miri: implement blocking eventfd](https://github.com/rust-lang/miri/pull/3939)
* [miri: refactor: refine thread variant for windows](https://github.com/rust-lang/miri/pull/4035)
* [miri: renamed `this` to `ecx` in `extern_static`](https://github.com/rust-lang/miri/pull/4030)
* [miri: use -Zroot-dir instead of --remap-path-prefix for diagnostic dir handling](https://github.com/rust-lang/miri/pull/4039)
* [stabilize `const_atomic_from_ptr`](https://github.com/rust-lang/rust/pull/131717)
* [stabilize `const_option_ext`](https://github.com/rust-lang/rust/pull/132966)
* [stabilize `const_ptr_is_null`](https://github.com/rust-lang/rust/pull/133116)
* [stabilize `const_unicode_case_lookup`](https://github.com/rust-lang/rust/pull/132948)
* [vectorize `slice::is_sorted`](https://github.com/rust-lang/rust/pull/132883)
* [`#[inline]` integer parsing functions](https://github.com/rust-lang/rust/pull/132870)
* [add `as_slice/into_slice` for IoSlice/IoSliceMut](https://github.com/rust-lang/rust/pull/132790)
* [generalize `NonNull::from_raw_parts` per ACP362](https://github.com/rust-lang/rust/pull/132895)
* [rwlock downgrade](https://github.com/rust-lang/rust/pull/128219)
* [implement `mixed_integer_ops_unsigned_sub`](https://github.com/rust-lang/rust/pull/126046)
* [improve codegen of `fmt_num` to delete unreachable panic](https://github.com/rust-lang/rust/pull/122770)
* [float types: move copysign, abs, signum to libcore](https://github.com/rust-lang/rust/pull/131304)
* [make `CloneToUninit` dyn-compatible](https://github.com/rust-lang/rust/pull/133003)
* [mark `is_val_statically_known` intrinsic as stably const-callable](https://github.com/rust-lang/rust/pull/132449)
* [optimize `char::to_digit` and assert radix is at least 2](https://github.com/rust-lang/rust/pull/132709)
* [hashbrown: further sequester `Group`/`Tag` code](https://github.com/rust-lang/hashbrown/pull/568)
* [hashbrown: mark const fn constructors as `rustc_const_stable_indirect`](https://github.com/rust-lang/hashbrown/pull/586)
* [codegen\_gcc: fix volatile loads and stores](https://github.com/rust-lang/rustc_codegen_gcc/pull/572)
* [cargo resolver: Stabilize resolver v3](https://github.com/rust-lang/cargo/pull/14754)
* [cargo rustdoc: diplay env vars in extra verbose mode](https://github.com/rust-lang/cargo/pull/14812)
* [cargo fix: error context for `git_fetch` refspec not found](https://github.com/rust-lang/cargo/pull/14806)
* [cargo: always include Cargo.lock in published crates](https://github.com/rust-lang/cargo/pull/14815)
* [cargo: migrate build-rs to the Cargo repo](https://github.com/rust-lang/cargo/pull/14786)
* [cargo: simplify English used in guide](https://github.com/rust-lang/cargo/pull/14825)
* [rustdoc search: allow queries to end in an empty path segment](https://github.com/rust-lang/rust/pull/132569)
* [rustdoc-search: case-sensitive only when capitals are used](https://github.com/rust-lang/rust/pull/133043)
* [rustdoc-search: use smart binary search in bitmaps](https://github.com/rust-lang/rust/pull/133185)
* [rustdoc: treat declarative macros more like other item kinds](https://github.com/rust-lang/rust/pull/132302)
* [rustdoc: use a trie for name-based search](https://github.com/rust-lang/rust/pull/133005)
* [rustdoc: Fix duplicated footnote IDs](https://github.com/rust-lang/rust/pull/133000)
* [rustdoc: Fix handling of footnote reference in footnote definition](https://github.com/rust-lang/rust/pull/133040)
* [rustdoc: Fix items with generics not having their jump to def link generated](https://github.com/rust-lang/rust/pull/133180)
* [rustdoc: Perform less work when cleaning `middle::ty` parenthesized generic args](https://github.com/rust-lang/rust/pull/132886)
* [clippy: `missing_safety_doc` accept uppercase "SAFETY"](https://github.com/rust-lang/rust-clippy/pull/13701)
* [clippy: allow conditional `Send` futures in `future_not_send`](https://github.com/rust-lang/rust-clippy/pull/13590)
* [clippy: do not trigger `if_let_mutex` starting from Edition 2024](https://github.com/rust-lang/rust-clippy/pull/13695)
* [clippy: don't lint CStr literals, do lint float literals in `redundant_guards`](https://github.com/rust-lang/rust-clippy/pull/13698)
* [clippy: handle `Option::map_or(true, …)` in `unnecessary_map_or` lint](https://github.com/rust-lang/rust-clippy/pull/13653)
* [clippy: new lint: `unnecessary_map_or`](https://github.com/rust-lang/rust-clippy/pull/11796)
* [clippy: support user format-like macros](https://github.com/rust-lang/rust-clippy/pull/9948)
* [rust-analyzer: migrate `reorder_fields` assist to use `SyntaxFactory`](https://github.com/rust-lang/rust-analyzer/pull/18495)

### Rust Compiler Performance Triage

We saw improvements to a large swath of benchmarks with the querification of
MonoItem collection (PR #132566). There were also some PRs where we are willing
to pay a compile-time cost for expected runtime benefit (PR #132870, PR #120370),
or pay a small cost in the single-threaded case in exchange for a big parallel
compilation win (PR #124780).

Triage done by **@pnkfelix**.
Revision range: [d4822c2d..7d40450b](https://perf.rust-lang.org/?start=d4822c2d84c242cc7403118b50c571464f38ef8f&end=7d40450b2df92bdc9dec414b30cf5f7a5979a92e&absolute=false&stat=instructions%3Au)

2 Regressions, 4 Improvements, 10 Mixed; 6 of them in rollups
47 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/d1b574c0c528c74491412625aa5bd3f27a9c2268/triage/2024-11-19.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [[RFC] Thread spawn hook (inheriting thread locals)](https://github.com/rust-lang/rfcs/pull/3642)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *No RFCs were approved this week.*

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Always display first line of impl blocks even when collapsed](https://github.com/rust-lang/rust/pull/132155)
* [disposition: merge] [Stabilize async closures (RFC 3668)](https://github.com/rust-lang/rust/pull/132706)
* [disposition: merge] [Tracking Issue for fn const BuildHasherDefault::new()](https://github.com/rust-lang/rust/issues/123197)
* [disposition: merge] [Add `AsyncFn*` to to the prelude in all editions](https://github.com/rust-lang/rust/pull/132611)
* [disposition: merge] [Tracking Issue for #![feature(const_float_methods)]](https://github.com/rust-lang/rust/issues/130843)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Add future-incompat warning against keywords in cfgs and add raw-idents](https://github.com/rust-lang/cargo/pull/14671)

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* [disposition: merge] [Consensus check: let-chains and is are not mutually exclusive](https://github.com/rust-lang/lang-team/issues/297)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Hierarchy of Sized traits](https://github.com/rust-lang/rfcs/pull/3729)

## Upcoming Events

Rusty Events between 2024-11-20 - 2024-12-18 🦀

### Virtual
* 2024-11-20 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Rust for Rustaceans Book Club: Chapter 12: Rust Without the Standard Library**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/304441931/)
* 2024-11-20 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Embedded Rust Workshop**](https://www.meetup.com/vancouver-rust/events/304047664/)
* 2024-11-21 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633273/)
* 2024-11-21 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Trustworthy IoT with Rust--and passwords!**](https://www.meetup.com/charlottesville-rust-meetup/events/304216847/)
* 2024-11-21 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #7**](https://www.meetup.com/bevy-game-development/events/304078762/)
* 2024-11-25 | Virtual (Bratislava, SK) | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**ONLINE Talk, sponsored by Sonalake - Bratislava Rust Meetup**](https://www.meetup.com/bratislava-rust-meetup-group/events/304373224/)
* 2024-11-26 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/fkmcntygcpbjc/)
* 2024-11-28 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898099/)
* 2024-11-28 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820276/)
* 2024-12-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/302007374/)
* 2024-12-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031652)
* 2024-12-05 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/05/rust-hack-and-learn.html) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633275/)
* 2024-12-07 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2024-12-10 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346988/)
* 2024-12-11 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/304047666/)
* 2024-12-12 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898129/)
* 2024-12-12 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820276/)
* 2024-12-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346972/)

### Africa
* 2024-12-10 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Hello World... again**](https://www.meetup.com/johannesburg-rust-meetup/events/304649358/)
* 2024-12-07 | Virtual( Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia
* 2024-11-21 | Seoul, KR | [Rust Programming Meetup Seoul](https://www.meetup.com/rust-seoul-meetup/events/)
    * [**Seoul Rust Meetup**](https://www.meetup.com/rust-seoul-meetup/events/304590280/)
* 2024-11-28 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**RustTechX Summit 2024 BOSCH**](https://hasgeek.com/rustbangalore/rusttechx-summit-2024-bosch/)
* 2024-11-30 | Tokyo, JP | [Rust Tokyo](https://rust.tokyo/)
    * [**Rust.Tokyo 2024**](https://rust.tokyo/lineup)

### Europe
* 2024-11-20 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #72**](https://www.meetup.com/rust-paris/events/304396616/)
* 2024-11-21 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust meetup #53 sponsored by Microsoft**](https://www.meetup.com/copenhagen-rust-community/events/304608747/)
* 2024-11-21 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (pub)**](https://www.meetup.com/rust-and-friends/events/304110922/)
* 2024-11-21 | Madrid, ES | [MadRust](https://www.meetup.com/madrust/events/)
    * [**Taller de introducción a unit testing en Rust**](https://www.meetup.com/madrust/events/304484962/)
* 2024-11-21 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154277/)
* 2024-11-23 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust + HTMX - Workshop #3**](https://www.meetup.com/rust-basel/events/303714372/)
* 2024-11-25 | Zagreb, HR | [impl Zagreb for Rust](https://www.meetup.com/zagreb-rust-meetup/events/)
    * [**Rust Meetup 2024/11: Panel diskusija - Usvajanje Rusta i iskustva iz industrije**](https://www.meetup.com/zagreb-rust-meetup/events/304576915/)
* 2024-11-26 | Warsaw, PL | [Rust Warsaw](https://www.meetup.com/rust-warsaw/events/)
    * [**New Rust Warsaw Meetup #3**](https://www.meetup.com/rust-warsaw/events/304379707/)
* 2024-11-27 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund**](https://www.meetup.com/rust-dortmund/events/304290556)
* 2024-11-28 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at Lind Capital**](https://www.meetup.com/rust-aarhus/events/304005322/)
* 2024-11-28 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #10**](https://www.meetup.com/rust-meetup-augsburg/events/304002691/)
* 2024-11-28 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299421381/)
* 2024-11-28 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #5**](https://www.meetup.com/rust-gdansk/events/304462668/)
* 2024-11-28 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/events/)
    * [**Rust Hack & Learn with Mainmatter & Otto**](https://www.meetup.com/rust-meetup-hamburg/events/303898286/)
* 2024-11-28 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester November Code Night**](https://www.meetup.com/rust-manchester/events/304556866/)
* 2024-11-28 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/events/)
    * [**Rust/C++ Meetup Prague (November 2024)**](https://www.meetup.com/rust-prague/events/304002733/)
* 2024-12-03 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust Hack Night #11: Advent of Code**](https://www.meetup.com/copenhagen-rust-community/events/304427710/)
* 2024-12-04 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123399/)
* 2024-12-05 | Olomouc, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**Rust Moravia Meetup (December 2024)**](https://www.meetup.com/rust-moravia/events/304075150/)
* 2024-12-06 | Moscow, RU | [RustCon RU](https://rustcon.ru/)
    * [**RustCon Russia**](https://rustcon.ru/)
* 2024-12-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtygcqbpb/)
* 2024-12-12 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ JetBrains**](https://www.meetup.com/rust-amsterdam-group/events/304514267/)
* 2024-12-17 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Types, Traits und Best Practices**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425056/)

### North America
* 2024-11-21 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/304568425/)
* 2024-11-23 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston Common Rust Lunch, Nov 23**](https://www.meetup.com/bostonrust/events/303708407/)
* 2024-11-25 | Ferndale, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ferndale**](https://www.meetup.com/detroitrust/events/dmgjntygcpbhc/)
* 2024-11-26 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/events/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/304530470/)
* 2024-11-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcpbkc/)
* 2024-11-28 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/304468157/)
* 2024-12-05 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Rust Strings**](https://www.meetup.com/stl-rust/events/302371466/)
* 2024-12-10 | Ann Arbor, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcqbnb/)
* 2024-12-12 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/wqkgntygcqbqb/)
* 2024-12-16 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/events/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/304530508/)
* 2024-12-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638256/)

### Oceania
* 2024-12-04 | Sydney, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/events/)
    * [**2024 🦀 Encore ✨ Talks**](https://www.meetup.com/rust-sydney/events/304625921/)
* 2024-12-08 | Canberra, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/events/)
    * [**CRUG Xmas party**](https://www.meetup.com/rust-canberra/events/304282046/)

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

> The whole point of Rust is that before there were two worlds:
>
> * Inefficient, garbage collected, reliable languages
> * Efficient, manually allocated, dangerous languages
>
> And the mark of being a good developer in the first was mitigating the inefficiency well, and for the second it was it didn't crash, corrupt memory, or be riddled with security issues. Rust makes the trade-off instead that being good means understanding how to avoid the compiler yelling at you.

– [Simon Buchan on rust-users]()

Thanks to [binarycat](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1632) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
