Title: This Week in Rust 572
Number: 572
Date: 2024-11-06
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
* [October project goals update](https://blog.rust-lang.org/2024/10/31/project-goals-oct-update.html)
* [Next Steps on the Rust Trademark Policy](https://blog.rust-lang.org/2024/11/06/trademark-update.html)
* [This Development-cycle in Cargo: 1.83](https://blog.rust-lang.org/inside-rust/2024/10/31/this-development-cycle-in-cargo-1.83.html)
* [Re-organising the compiler team and recognising our team members](https://blog.rust-lang.org/inside-rust/2024/11/01/compiler-team-reorg.html)
* [This Month in Our Test Infra: October 2024](https://blog.rust-lang.org/inside-rust/2024/11/04/test-infra-oct-2024-2.html)
* [Call for proposals: Rust 2025h1 project goals](https://blog.rust-lang.org/inside-rust/2024/11/04/project-goals-2025h1-call-for-proposals.html)


### Foundation
* [Q3 2024 Recap from Rebecca Rumbul](https://foundation.rust-lang.org/news/q3-2024-recap-from-rebecca-rumbul/)
* [Rust Foundation Member Announcement: CodeDay, OpenSource Science(OS-Sci), & PROMOTIC](https://foundation.rust-lang.org/news/rust-foundation-member-announcement-codeday-opensource-science-os-sci-promotic/)

### Newsletters
* [The Embedded Rustacean Issue #31](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-31)

### Project/Tooling Updates
* [Announcing Intentrace, an alternative strace for everyone](https://github.com/sectordistrict/intentrace)
* [Ractor Quickstart](https://slawlor.github.io/ractor/quickstart/)
* [Announcing Sycamore v0.9.0](https://sycamore.dev/post/announcing-v0-9-0)
* [CXX-Qt 0.7 Release](https://www.kdab.com/cxx-qt-0-7/)
* [An 'Educational' Platformer for Kids to Learn Math and Reading—and Bevy for the Devs](https://github.com/uggla/rock_run)
* [ZH][EN] [Select HTML Components in Declarative Rust](https://ideas.reify.ing/en/blog/select-html-declarative-rust/)

### Observations/Thoughts
* [Safety in an unsafe world](https://lwn.net/SubscriberLink/995814/17e451bcb3015920/)
* [MinPin: yet another pin proposal](https://smallcultfollowing.com/babysteps/blog/2024/11/05/minpin/)
* [Reached the recursion limit... at build time?](https://blog.veeso.dev/blog/en/reached-the-recursion-limit-at-build-time/)
* [Building Trustworthy Software: The Power of Testing in Rust](https://tylerjw.dev/posts/20241106-building-trustworthy-software/)
* [Async Rust is not safe with io_uring](https://tonbo.io/blog/async-rust-is-not-safe-with-io-uring)
* [Macros, Safety, and SOA](https://tim-harding.github.io/blog/soa-rs/)
* [how big is your future?](https://hegdenu.net/posts/how-big-is-your-future/)
* [A comparison of Rust’s borrow checker to the one in C#](https://em-tg.github.io/csborrow/)
* [Streaming Audio APIs in Rust pt. 3: Audio Decoding](https://xd009642.github.io/2024/11/04/streaming-audio-APIs-audio-decoding.html)
* [audio] [InfinyOn with Deb Roy Chowdhury](https://corrode.dev/podcast/s03e02-infinyon/)

### Rust Walkthroughs
* [Difference Between iter() and into_iter() in Rust](https://crustc.com/difference-iter-and-into_iter/)
* [Rust's Sneaky Deadlock With `if let` Blocks](https://brooksblog.bearblog.dev/rusts-sneaky-deadlock-with-if-let-blocks/)
* [Why I love Rust for tokenising and parsing](https://xnacly.me/posts/2024/rust-pldev/)
* ["German string" optimizations in Spellbook](https://the-mikedavis.github.io/posts/german-string-optimizations-in-spellbook/)
* [Rust's Most Subtle Syntax](https://zkrising.com/writing/rusts-most-subtle-syntax/)
* [Parsing arguments in Rust with no dependencies](https://www.ntietz.com/blog/parsing-arguments-rust-no-deps/)
* [Simple way to make i18n support in Rust with with examples and tests](https://www.onlycoiners.com/user/onlycoiners/blog/simple-way-to-make-i18n-support-in-rust-with-with-examples-a)
* [How to shallow clone a Cow](https://blog.getreu.net/20241005-how_to_shallow_clone_a_cow-blog/)
* [Beginner Rust ESP32 development - Snake](https://jamesmcm.github.io/blog/beginner-rust-esp32-lcdsnake/)
* [video] [Rust Collections & Iterators Demystified 🪄](https://www.youtube.com/watch?v=oiWATcjyUEI)

### Research
* [Charon: An Analysis Framework for Rust](https://arxiv.org/abs/2410.18042)
* [Crux, a Precise Verifier for Rust and Other Languages](https://arxiv.org/abs/2410.18280)

### Miscellaneous
* [Feds: Critical Software Must Drop C/C++ by 2026 or Face Risk](https://thenewstack.io/feds-critical-software-must-drop-c-c-by-2026-or-face-risk/)
* [audio] [Let's talk about Rust with John Arundel](https://gopodcast.dev/episodes/046-lets-talk-about-rust-with-john-arundel)
* [audio] [Exploring Rust for Embedded Systems with Philip Markgraf](https://agileembeddedpodcast.com/episodes/philip-markgraf-on-rust)

## Crate of the Week

This week's crate is [wtransport](https://crates.io/crates/wtransport), an implementation of the WebTransport specification, a successor to WebSockets with many additional features.

Thanks to [Josh Triplett](https://users.rust-lang.org/t/crate-of-the-week/2704/1369) for the suggestion!

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

473 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-10-29..2024-11-05

* [account for late-bound depth when capturing all opaque lifetimes](https://github.com/rust-lang/rust/pull/132466)
* [add `--print host-tuple` to print host target tuple](https://github.com/rust-lang/rust/pull/125579)
* [add `f16` and `f128` to `invalid_nan_comparison`](https://github.com/rust-lang/rust/pull/132439)
* [add `lp64e` RISC-V ABI](https://github.com/rust-lang/rust/pull/132354)
* [also treat `impl` definition parent as transparent regarding modules](https://github.com/rust-lang/rust/pull/132453)
* [cleanup attributes around unchecked shifts and unchecked negation in const](https://github.com/rust-lang/rust/pull/132445)
* [cleanup op lookup in HIR typeck](https://github.com/rust-lang/rust/pull/132274)
* [collect item bounds for RPITITs from trait where clauses just like associated types](https://github.com/rust-lang/rust/pull/132194)
* [do not enforce `~const` constness effects in typeck if `rustc_do_not_const_check`](https://github.com/rust-lang/rust/pull/132366)
* [don't lint `irrefutable_let_patterns` on leading patterns if `else if` let-chains](https://github.com/rust-lang/rust/pull/129394)
* [double-check conditional constness in MIR](https://github.com/rust-lang/rust/pull/132276)
* [ensure that resume arg outlives region bound for coroutines](https://github.com/rust-lang/rust/pull/132151)
* [find the generic container rather than simply looking up for the assoc with const arg](https://github.com/rust-lang/rust/pull/132559)
* [fix compiler panic with a large number of threads](https://github.com/rust-lang/rust/pull/132355)
* [fix suggestion for diagnostic error E0027](https://github.com/rust-lang/rust/pull/132025)
* [fix validation when lowering `?` trait bounds](https://github.com/rust-lang/rust/pull/132209)
* [implement suggestion for never type fallback lints](https://github.com/rust-lang/rust/pull/132383)
* [improve `missing_abi` lint](https://github.com/rust-lang/rust/pull/132357)
* [improve duplicate derive Copy/Clone diagnostics](https://github.com/rust-lang/rust/pull/131153)
* [llvm: match new LLVM 128-bit integer alignment on sparc](https://github.com/rust-lang/rust/pull/132422)
* [make codegen help output more consistent](https://github.com/rust-lang/rust/pull/132522)
* [make sure `type_param_predicates` resolves correctly for RPITIT](https://github.com/rust-lang/rust/pull/132373)
* [pass `RUSTC_HOST_FLAGS` at once without the for loop](https://github.com/rust-lang/rust/pull/132365)
* [port most of `--print=target-cpus` to Rust](https://github.com/rust-lang/rust/pull/132514)
* [register `~const` preds for `Deref` adjustments in HIR typeck](https://github.com/rust-lang/rust/pull/132275)
* [reject generic self types](https://github.com/rust-lang/rust/pull/130098)
* [remap impl-trait lifetimes on HIR instead of AST lowering](https://github.com/rust-lang/rust/pull/129383)
* [remove `""` case from RISC-V `llvm_abiname` match statement](https://github.com/rust-lang/rust/pull/132421)
* [remove `do_not_const_check` from `Iterator` methods](https://github.com/rust-lang/rust/pull/132368)
* [remove region from adjustments](https://github.com/rust-lang/rust/pull/132301)
* [remove support for `-Zprofile` (gcov-style coverage instrumentation)](https://github.com/rust-lang/rust/pull/131829)
* [replace manual time convertions with std ones, comptime time format parsing](https://github.com/rust-lang/rust/pull/132521)
* [suggest creating unary tuples when types don't match a trait](https://github.com/rust-lang/rust/pull/132583)
* [support `clobber_abi` and vector registers (clobber-only) in PowerPC inline assembly](https://github.com/rust-lang/rust/pull/131341)
* [try to point out when edition 2024 lifetime capture rules cause borrowck issues](https://github.com/rust-lang/rust/pull/131186)
* [typingMode: merge intercrate, reveal, and `defining_opaque_types`](https://github.com/rust-lang/rust/pull/131856)
* [miri: change `futex_wait` errno from Scalar to IoError](https://github.com/rust-lang/miri/pull/4000)
* [stabilize `const_arguments_as_str`](https://github.com/rust-lang/rust/pull/132511)
* [stabilize `if_let_rescope`](https://github.com/rust-lang/rust/pull/131984)
* [mark `str::is_char_boundary` and `str::split_at*` unstably `const`](https://github.com/rust-lang/rust/pull/131520)
* [remove const-support for `align_offset` and `is_aligned`](https://github.com/rust-lang/rust/pull/132423)
* [unstably add `ptr::byte_sub_ptr`](https://github.com/rust-lang/rust/pull/132459)
* [implement `From<&mut {slice}>` for `Box/Rc/Arc<{slice}>`](https://github.com/rust-lang/rust/pull/129329)
* [rc/Arc: don't leak the allocation if drop panics](https://github.com/rust-lang/rust/pull/132231)
* [add LowerExp and UpperExp implementations to NonZero](https://github.com/rust-lang/rust/pull/131377)
* [use Hacker's Delight impl in `i64::midpoint` instead of wide `i128` impl](https://github.com/rust-lang/rust/pull/132238)
* [xous: sync: remove `rustc_const_stable` attribute on Condvar and Mutex `new()`](https://github.com/rust-lang/rust/pull/132321)
* [add `const_panic` macro to make it easier to fall back to non-formatting panic in const](https://github.com/rust-lang/rust/pull/132542)
* [cargo: downgrade version-exists error to warning on dry-run](https://github.com/rust-lang/cargo/pull/14742)
* [cargo: add more metadata to `rustc_fingerprint`](https://github.com/rust-lang/cargo/pull/14761)
* [cargo: add transactional semantics to `rustfix`](https://github.com/rust-lang/cargo/pull/14747)
* [cargo: add unstable `-Zroot-dir` flag to configure the path from which rustc should be invoked](https://github.com/rust-lang/cargo/pull/14752)
* [cargo: allow build scripts to report error messages through `cargo::error`](https://github.com/rust-lang/cargo/pull/14743)
* [cargo: change config paths to only check `CARGO_HOME` for cargo-script](https://github.com/rust-lang/cargo/pull/14749)
* [cargo: download targeted transitive deps of with artifact deps' target platform](https://github.com/rust-lang/cargo/pull/14723)
* [cargo fix: track version in fingerprint dep-info files](https://github.com/rust-lang/cargo/pull/14751)
* [cargo: remove requirement for --target when invoking Cargo with -Zbuild-std](https://github.com/rust-lang/cargo/pull/14317)
* [rustdoc: Fix `--show-coverage` when JSON output format is used](https://github.com/rust-lang/rust/pull/132596)
* [rustdoc: Unify variant `struct` fields margins with `struct` fields](https://github.com/rust-lang/rust/pull/132258)
* [rustdoc: make doctest span tweak a 2024 edition change](https://github.com/rust-lang/rust/pull/132210)
* [rustdoc: skip stability inheritance for some item kinds](https://github.com/rust-lang/rust/pull/132481)
* [mdbook: improve theme support when JS is disabled](https://github.com/rust-lang/mdBook/pull/2454)
* [mdbook: load the sidebar toc from a shared JS file or iframe](https://github.com/rust-lang/mdBook/pull/2414)
* [clippy: `infinite_loops`: fix incorrect suggestions on async functions/closures](https://github.com/rust-lang/rust-clippy/pull/13608)
* [clippy: `needless_continue`: check labels consistency before warning](https://github.com/rust-lang/rust-clippy/pull/13648)
* [clippy: `no_mangle` attribute requires unsafe in Rust 2024](https://github.com/rust-lang/rust-clippy/pull/13631)
* [clippy: add new `trivial_map_over_range` lint](https://github.com/rust-lang/rust-clippy/pull/13034)
* [clippy: cleanup code suggestion for `into_iter_without_iter`](https://github.com/rust-lang/rust-clippy/pull/13634)
* [clippy: do not use `gen` as a variable name](https://github.com/rust-lang/rust-clippy/pull/13628)
* [clippy: don't lint unnamed consts and nested items within functions in `missing_docs_in_private_items`](https://github.com/rust-lang/rust-clippy/pull/13573)
* [clippy: extend `large_include_file` lint to also work on attributes](https://github.com/rust-lang/rust-clippy/pull/13636)
* [clippy: fix `allow_attributes` when expanded from some macros](https://github.com/rust-lang/rust-clippy/pull/13599)
* [clippy: improve display of clippy lints page when JS is disabled](https://github.com/rust-lang/rust-clippy/pull/13585)
* [clippy: new lint `map_all_any_identity`](https://github.com/rust-lang/rust-clippy/pull/13499)
* [clippy: new lint `needless_as_bytes`](https://github.com/rust-lang/rust-clippy/pull/13437)
* [clippy: new lint `source_item_ordering`](https://github.com/rust-lang/rust-clippy/pull/13376)
* [clippy: return iterator must not capture lifetimes in Rust 2024](https://github.com/rust-lang/rust-clippy/pull/13629)
* [clippy: use match ergonomics compatible with editions 2021 and 2024](https://github.com/rust-lang/rust-clippy/pull/13630)
* [rust-analyzer: allow interpreting consts and statics with interpret function command](https://github.com/rust-lang/rust-analyzer/pull/18470)
* [rust-analyzer: avoid interior mutability in `TyLoweringContext`](https://github.com/rust-lang/rust-analyzer/pull/18447)
* [rust-analyzer: do not render meta info when hovering usages](https://github.com/rust-lang/rust-analyzer/pull/18436)
* [rust-analyzer: add assist to generate a type alias for a function](https://github.com/rust-lang/rust-analyzer/pull/18385)
* [rust-analyzer: render extern blocks in `file_structure`](https://github.com/rust-lang/rust-analyzer/pull/18473)
* [rust-analyzer: show `static` values on hover](https://github.com/rust-lang/rust-analyzer/pull/18469)
* [rust-analyzer: auto-complete import for aliased function and module](https://github.com/rust-lang/rust-analyzer/pull/18382)
* [rust-analyzer: fix the server not honoring diagnostic refresh support](https://github.com/rust-lang/rust-analyzer/pull/18432)
* [rust-analyzer: only parse `safe` as contextual kw in extern blocks](https://github.com/rust-lang/rust-analyzer/pull/18446)
* [rust-analyzer: parse patterns with leading pipe properly in all places](https://github.com/rust-lang/rust-analyzer/pull/18453)
* [rust-analyzer: support new `#[rustc_intrinsic]` attribute and fallback bodies](https://github.com/rust-lang/rust-analyzer/pull/18475)

### Rust Compiler Performance Triage

A week dominated by one large improvement and one large regression where luckily the improvement had a larger impact. The regression seems to have been caused by a newly introduced lint that might have performance issues. The improvement was in building rustc with protected visibility which reduces the number of dynamic relocations needed leading to some nice performance gains. Across a large swath of the perf suit, the compiler is on average 1% faster after this week compared to last week.

Triage done by **@rylev**.
Revision range: [c8a8c820..27e38f8f](https://perf.rust-lang.org/?start=c8a8c82035439cb2404b8f24ca0bc18209d534ca&end=27e38f8fc7efc57b75e9a763d7a0ee44822cd5f7&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.8%  | [0.1%, 2.0%]    | 80    |
| Regressions ❌ <br /> (secondary)  | 1.9%  | [0.2%, 3.4%]    | 45    |
| Improvements ✅ <br /> (primary)   | -1.9% | [-31.6%, -0.1%] | 148   |
| Improvements ✅ <br /> (secondary) | -5.1% | [-27.8%, -0.1%] | 180   |
| All ❌✅ (primary)                 | -1.0% | [-31.6%, 2.0%]  | 228   |


1 Regression, 1 Improvement, 5 Mixed; 3 of them in rollups
46 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/bd155e07def612eff4cb7fec391cf60f22674673/triage/2024-11-05.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [[RFC] Default field values](https://github.com/rust-lang/rfcs/pull/3681)
* [RFC: Give users control over feature unification](https://github.com/rust-lang/rfcs/pull/3692)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [Add support for `use Trait::func`](https://github.com/rust-lang/rfcs/pull/3591)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Stabilize Arm64EC inline assembly](https://github.com/rust-lang/rust/pull/131781)
* [disposition: merge] [Stabilize s390x inline assembly](https://github.com/rust-lang/rust/pull/131258)
* [disposition: merge] [rustdoc-search: simplify rules for generics and type params](https://github.com/rust-lang/rust/pull/127589)
* [disposition: merge] [Fix ICE when passing DefId-creating args to legacy_const_generics.](https://github.com/rust-lang/rust/pull/130443)
* [disposition: merge] [Tracking Issue for `const_option_ext`](https://github.com/rust-lang/rust/issues/91930)
* [disposition: merge] [Tracking Issue for const_unicode_case_lookup](https://github.com/rust-lang/rust/issues/101400)
* [disposition: merge] [Reject raw lifetime followed by `'`, like regular lifetimes do](https://github.com/rust-lang/rust/pull/132341)
* [disposition: merge] [Enforce that raw lifetimes must be valid raw identifiers](https://github.com/rust-lang/rust/pull/132363)
* [disposition: merge] [Stabilize WebAssembly `multivalue`, `reference-types`, and `tail-call` target features](https://github.com/rust-lang/rust/pull/131080)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Proposals entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Implement The Update Framework for Project Signing](https://github.com/rust-lang/rfcs/pull/3724)
* [new] [[RFC] Static Function Argument Unpacking](https://github.com/rust-lang/rfcs/pull/3723)
* [new] [[RFC] Explicit ABI in extern](https://github.com/rust-lang/rfcs/pull/3722)
* [new] [Add homogeneous_`try_blocks` RFC](https://github.com/rust-lang/rfcs/pull/3721)

## Upcoming Events

Rusty Events between 2024-11-06 - 2024-12-04 🦀

### Virtual
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
* 2024-11-14 | Virtual and In-Person (Lehi, UT, US) | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Green Thumb: Building a Bluetooth-Enabled Plant Waterer with Rust and Microbit**](https://www.meetup.com/utah-rust/events/304206130/)
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
* 2024-11-21 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633273/)
* 2024-11-21 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Trustworthy IoT with Rust--and passwords!**](https://www.meetup.com/charlottesville-rust-meetup/events/304216847/)
* 2024-11-21 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #7**](https://www.meetup.com/bevy-game-development/events/304078762/)
* 2024-11-25 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**ONLINE Talk, sponsored by Sonalake - Bratislava Rust Meetup**](https://www.meetup.com/bratislava-rust-meetup-group/events/304373224/)
* 2024-11-26 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/fkmcntygcpbjc/)
* 2024-11-28 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898099/)
* 2024-12-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/302007374/)

### Asia
* 2024-11-28 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**RustTechX Summit 2024 BOSCH**](https://hasgeek.com/rustbangalore/rusttechx-summit-2024-bosch/)
* 2024-11-30 | Tokyo, JP | [Rust Tokyo](https://rust.tokyo/)
    * [**Rust.Tokyo 2024**](https://rust.tokyo/lineup)

### Europe
* 2024-11-06 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123398/)
* 2024-11-06 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.fr/o/paris-rustaceans-74289178383)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1037795553437)
* 2024-11-09 - 2024-11-11 | Florence, IT | [Rust Lab](https://rustlab.it/)
    * [**Rust Lab 2024: The International Conference on Rust in Florence**](https://rustlab.it/schedule)
* 2024-11-12 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/events/)
    * [**Encrypted/distributed filesystems, wasm-bindgen**](https://www.meetup.com/rust-zurich/events/304162840)
* 2024-11-13 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/303915771/)
* 2024-11-14 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @UXStream**](https://www.meetup.com/stockholm-rust/events/304124737/)
* 2024-11-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Daten sichern mit ZFS (und Rust)**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425200/)
* 2024-11-21 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (pub)**](https://www.meetup.com/rust-and-friends/events/304110922/)
* 2024-11-21 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154277/)
* 2024-11-23 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust + HTMX - Workshop #3**](https://www.meetup.com/rust-basel/events/303714372/)
* 2024-11-27 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund**](https://www.meetup.com/rust-dortmund/events/304290556)
* 2024-11-28 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at Lind Capital**](https://www.meetup.com/rust-aarhus/events/304005322/)
* 2024-11-28 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #10**](https://www.meetup.com/rust-meetup-augsburg/events/304002691/)
* 2024-11-28 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299421381/)

### North America
* 2024-11-07 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/)
    * [**Chicago Rust Meetup**](https://www.meetup.com/chicago-rust-meetup/events/304327595/)
* 2024-11-07 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal/)
    * [**November Monthly Social**](https://www.meetup.com/rust-montreal/events/304248702/)
* 2024-11-07 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Game development with Rust and the Bevy engine**](https://www.meetup.com/stl-rust/events/302371464/)
* 2024-11-12 | Ann Arbor, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcpbqb/)
* 2024-11-14 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/hackerdojo/events/304211045/)
* 2024-11-15 | Mexico City, DF, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Multi threading y Async en Rust parte 2 - Smart Pointes y Closures**](https://www.meetup.com/rust-mx/events/304150412/)
* 2024-11-15 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch, Nov 15**](https://www.meetup.com/bostonrust/events/303708398/)
* 2024-11-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638252/)
* 2024-11-23 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston Common Rust Lunch, Nov 23**](https://www.meetup.com/bostonrust/events/303708407/)
* 2024-11-25 | Ferndale, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ferndale**](https://www.meetup.com/detroitrust/events/dmgjntygcpbhc/)
* 2024-11-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcpbkc/)

### Oceania
* 2024-11-12 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/events/)
    * [**Christchurch Rust Meetup**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/304029765/)

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

> Any sufficiently complicated C project contains an adhoc, informally specified, bug ridden, slow implementation of half of cargo.

– [Folkert de Vries at RustNL 2024 (youtube recording)](https://www.youtube.com/watch?v=mvzHQdCLkOY&t=912s)

Thanks to [Collin Richards](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1629) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
