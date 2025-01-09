Title: This Week in Rust 581
Number: 581
Date: 2025-01-08
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
* [The Embedded Rustacean Issue #36](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-36)

### Project/Tooling Updates
* [rust-gcc December 2024 Monthly report](https://rust-gcc.github.io/2025/01/07/2024-12-monthly-report.html)
* [Helix Release 25.01 Highlights](https://helix-editor.com/news/release-25-01-highlights/)

### Observations/Thoughts
* [Great things about Rust that aren't just performance](https://ntietz.com/blog/great-things-about-rust-beyond-perf/)
* [Rust In Space! How Rust is Powering Next-Generation Space Mission Simulators](https://www.howtocodeit.com/articles/rust-in-space)
* [Bevy Efficiency on Mobile](https://rustunit.com/blog/2025/01-02-bevy-mobile-framerate/)
* [Why nextest is process-per-test](https://sunshowers.io/posts/nextest-process-per-test/)
* [From Go to Rust 1: async Dispatch](https://medium.com/rustaceans/from-go-to-rust-1-async-dispatch-866e042cd98a)
* [This Month in @compiler-errors (rustc contributions)](https://hackmd.io/@compiler-errors/errs-december-2024)
* [Building functional safety at speed with Rust](https://www.sonair.com/journal/building-for-safety-with-rust)
* [You do not need multithreading to do more than one thing at a time](https://sander.saares.eu/2024/12/31/you-do-not-need-multithreading-to-do-more-than-one-thing-at-a-time/)
* [A journey into File Transfer Protocols in Rust](https://blog.veeso.dev/blog/en/a-journey-into-file-transfer-protocols-in-rust/)

### Rust Walkthroughs
* [Master Hexagonal Architecture in Rust Part IV: Trade-Offs of Hexagonal Architecture](https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust#trade-offs-of-hexagonal-architecture-in-rust)
* [The Definitive Guide to Error Handling in Rust Part III: Structured Error Handling](https://www.howtocodeit.com/articles/the-definitive-guide-to-rust-error-handling#structured-error-handling-in-rust)

### Research

### Miscellaneous
* [video] [Tue Henriksen on Rust in Embedded Systems, Misconceptions, & Building Community](https://www.youtube.com/watch?v=qt7ZLYnlBzk)
* [Bevy at RustWeek 2025: come hack with us!](https://bevyengine.org/news/bevy-at-rust-week/)
* [My failed attempt at AGI on the Tokio Runtime](https://www.christo.sh/building-agi-on-the-tokio-runtime/)
* [The JIT calculator challenge](https://ochagavia.nl/blog/the-jit-calculator-challenge/)
* [Gameboy on your terminal written in Rust](https://github.com/raphamorim/gameboy)
* [video] [BlockMesh Network - a full-stack open source project in Rust with Ohad Dahan](https://www.youtube.com/watch?v=4J8jxLnWmGs)

## Crate of the Week

This week's crate is [terminal-colorsaurus](https://crates.io/crates/terminal-colorsaurus), a small library to detect whether the terminal is in light or dark mode.

Thanks to [Tau](https://users.rust-lang.org/t/crate-of-the-week/2704/1386) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No calls for testing were issued this week.*

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* [Tracking issue for RFC 3695: Allow boolean literals as cfg predicates](https://github.com/rust-lang/rust/issues/131204)
  - [Testing steps](https://github.com/rust-lang/rust/issues/131204#issuecomment-2569314526)

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

* [**Rust Week (Rust NL)**](https://www.papercall.io/rust-week) | Closes on 2024-01-12 | Utrecht, NL | Event on 2025-05-13

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

375 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-12-31..2025-01-07

* [add m68k-unknown-none-elf target](https://github.com/rust-lang/rust/pull/135085)
* [target: add mips mti baremetal support](https://github.com/rust-lang/rust/pull/135074)
* [A few borrowck tweaks to improve 2024 edition migration lints](https://github.com/rust-lang/rust/pull/135147)
* [E0277: suggest dereferencing function arguments in more cases](https://github.com/rust-lang/rust/pull/133292)
* [Debuginfo: Force `enum DISCR_*` to `static const u64` to allow for inspection via LLDB](https://github.com/rust-lang/rust/pull/133990)
* [`ObligationCause` construction tweaks in typeck](https://github.com/rust-lang/rust/pull/134984)
* [`generic_assert` Constify methods used by the formatting system](https://github.com/rust-lang/rust/pull/135139)
* [`cg_llvm`: Use constants for DWARF opcodes, instead of FFI calls](https://github.com/rust-lang/rust/pull/135115)
* [`rustc_intrinsic`: support functions without body](https://github.com/rust-lang/rust/pull/135031)
* [add a notion of "some ABIs require certain target features"](https://github.com/rust-lang/rust/pull/134794)
* [add suggestion for wrongly ordered format parameters](https://github.com/rust-lang/rust/pull/134877)
* [add support for wasm exception handling to Emscripten target](https://github.com/rust-lang/rust/pull/131830)
* [avoid use of LFS64 symbols on Emscripten](https://github.com/rust-lang/rust/pull/134080)
* [borrowck diagnostics: make `add_move_error_suggestions` use the HIR rather than `SourceMap`](https://github.com/rust-lang/rust/pull/133486)
* [const-in-pattern: test that the PartialEq impl does not need to be const](https://github.com/rust-lang/rust/pull/135064)
* [deny usage of special FileCheck prefixes as revision names](https://github.com/rust-lang/rust/pull/134925)
* [don't enable anyhow's `backtrace` feature in opt-dist](https://github.com/rust-lang/rust/pull/135146)
* [don't ice on bad transmute in typeck in new solver](https://github.com/rust-lang/rust/pull/134744)
* [fix ICE when opaque captures a duplicated/invalid lifetime](https://github.com/rust-lang/rust/pull/135000)
* [force code generation in assembly generation smoke-tests](https://github.com/rust-lang/rust/pull/135088)
* [improve diagnostics for `HostEffectPredicate` in the new solver](https://github.com/rust-lang/rust/pull/132345)
* [improve infer (`_`) suggestions in `const`s and static`s`](https://github.com/rust-lang/rust/pull/135044)
* [pass objcopy args for stripping on OSX](https://github.com/rust-lang/rust/pull/135034)
* [pass the arch rather than full target name to `windows_registry::find_tool`](https://github.com/rust-lang/rust/pull/133955)
* [project to `TyKind::Error` when there are unconstrained non-lifetime (ty/const) impl params](https://github.com/rust-lang/rust/pull/135057)
* [provide structured suggestion for `impl Default` of type where all fields have defaults](https://github.com/rust-lang/rust/pull/134979)
* [remove allowing `static_mut_refs` lint](https://github.com/rust-lang/rust/pull/131439)
* [remove range-metadata amdgpu workaround](https://github.com/rust-lang/rust/pull/135027)
* [report correct `SelectionError` for `ConstArgHasType` in new solver fulfill](https://github.com/rust-lang/rust/pull/134771)
* [report impl method has stricter requirements even when RPITIT inference gets in the way](https://github.com/rust-lang/rust/pull/135055)
* [some type-outlives computation tweaks](https://github.com/rust-lang/rust/pull/135007)
* [suggest to replace tuple constructor through projection](https://github.com/rust-lang/rust/pull/135090)
* [suppress host effect predicates if underlying trait doesn't hold](https://github.com/rust-lang/rust/pull/134951)
* [switch rtems target to panic unwind](https://github.com/rust-lang/rust/pull/133420)
* [taint fcx on selection errors during unsizing](https://github.com/rust-lang/rust/pull/135042)
* [turn `rustc_box` into an intrinsic](https://github.com/rust-lang/rust/pull/135046)
* [use `PostBorrowckAnalysis` in `check_coroutine_obligations`](https://github.com/rust-lang/rust/pull/134742)
* [miri: concurrency: generalize UnblockCallback to MachineCallback](https://github.com/rust-lang/miri/pull/4106)
* [library: fix adler{ → 2}.debug](https://github.com/rust-lang/rust/pull/135110)
* [mark `slice::reverse` unstably const](https://github.com/rust-lang/rust/pull/135121)
* [std: sync to dep versions of backtrace](https://github.com/rust-lang/rust/pull/135070)
* [try to write the panic message with a single `write_all` call](https://github.com/rust-lang/rust/pull/122565)
* [char `to_digit`: avoid unnecessary casts to u64](https://github.com/rust-lang/rust/pull/134969)
* [core: implement `bool::select_unpredictable`](https://github.com/rust-lang/rust/pull/133964)
* [do not in-place-iterate over flatmap/flatten](https://github.com/rust-lang/rust/pull/135104)
* [cargo: fix `https::self_signed_should_fail` for macos](https://github.com/rust-lang/cargo/pull/15016)
* [cargo: fix: env table config can't trigger rebuild with `rerun-if-env-changed`](https://github.com/rust-lang/cargo/pull/14756)
* [rustdoc: fix mismatched capitalization in sidebar](https://github.com/rust-lang/rust/pull/135116)
* [rustdoc: treat `allowed_through_unstable_modules` as deprecation](https://github.com/rust-lang/rust/pull/135043)
* [clippy: `clippy::redundant_locals` is not a correctness lint](https://github.com/rust-lang/rust-clippy/pull/13747)
* [clippy: `needless_continue`: lint if the last stmt in loop is `continue` recurisvely](https://github.com/rust-lang/rust-clippy/pull/13891)
* [clippy: add lint for calling `Iterator::last()` on `DoubleEndedIterator`](https://github.com/rust-lang/rust-clippy/pull/13922)
* [clippy: check if deref target implements `is_empty` for `len_zero` lint](https://github.com/rust-lang/rust-clippy/pull/13871)
* [clippy: do not trigger `missing_const_for_fn` for tests](https://github.com/rust-lang/rust-clippy/pull/13945)
* [clippy: improve `slow_vector_initialization` suggestion](https://github.com/rust-lang/rust-clippy/pull/13912)
* [clippy: only emit `useless_vec` suggestion if the macro does not contain code comments](https://github.com/rust-lang/rust-clippy/pull/13911)
* [rust-analyzer: allow targetDir to be an absolute path](https://github.com/rust-lang/rust-analyzer/pull/18822)
* [rust-analyzer: disable `rustc_test` metrics again](https://github.com/rust-lang/rust-analyzer/pull/18829)
* [rust-analyzer: allow excluding specific traits from completion](https://github.com/rust-lang/rust-analyzer/pull/18179)
* [rust-analyzer: support the new `CoercePointee` derive](https://github.com/rust-lang/rust-analyzer/pull/18821)
* [rust-analyzer: support updating snapshot tests with codelens/hovering/runnables](https://github.com/rust-lang/rust-analyzer/pull/18757)
* [rust-analyzer: fix case where completion inside macro that expands to `#[test]` was unavailable](https://github.com/rust-lang/rust-analyzer/pull/18853)
* [rust-analyzer: fix metrics workflow](https://github.com/rust-lang/rust-analyzer/pull/18831)
* [rust-analyzer: fix no space insert before and after if value is only spaces](https://github.com/rust-lang/rust-analyzer/pull/18820)
* [rust-analyzer: be more permissive with completion resolve data](https://github.com/rust-lang/rust-analyzer/pull/18836)
* [rust-analyzer: clear diagnostics on cancel unconditionally](https://github.com/rust-lang/rust-analyzer/pull/18858)
* [rust-analyzer: clear flycheck diagnostics per package properly](https://github.com/rust-lang/rust-analyzer/pull/18826)
* [rust-analyzer: deduplicate crate graph](https://github.com/rust-lang/rust-analyzer/pull/18806)
* [rust-analyzer: fix a bug that was caused by fixup reversing](https://github.com/rust-lang/rust-analyzer/pull/18852)
* [rust-analyzer: fix flycheck cancellations leaving stale errors](https://github.com/rust-lang/rust-analyzer/pull/18817)
* [rust-analyzer: fix flycheck getting confused which package to check](https://github.com/rust-lang/rust-analyzer/pull/18845)
* [rust-analyzer: fix non-cargo flychecks immediately clearing received diagnostics](https://github.com/rust-lang/rust-analyzer/pull/18848)
* [rust-analyzer: fix overflow detection in MIR evaluation](https://github.com/rust-lang/rust-analyzer/pull/18819)
* [rust-analyzer: fix relative .cargo env vars not working](https://github.com/rust-lang/rust-analyzer/pull/18841)
* [rust-analyzer: handle newstyle `rustc_intrinsic` safety correctly](https://github.com/rust-lang/rust-analyzer/pull/18843)
* [rust-analyzer: hide synthetic locals from completions](https://github.com/rust-lang/rust-analyzer/pull/18835)
* [rust-analyzer: store token trees in contiguous `Vec` instead of as a tree](https://github.com/rust-lang/rust-analyzer/pull/18327)

### Rust Compiler Performance Triage

A quiet week with not much going on. A small regression was caused by a bugfix related to traits, but
it was somewhat offset by a cargo update that brought a small perf. win.

Triage done by **@kobzol**.
Revision range: [93722f7e..0f1e965f](https://perf.rust-lang.org/?start=93722f7ed56bcf27839a6355074095c4320b7d37&end=0f1e965fec3bc2f97b932e9dd8e85fca6d7faadc&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.4%  | [0.1%, 1.1%]   | 20    |
| Regressions ❌ <br /> (secondary)  | 0.4%  | [0.1%, 2.5%]   | 19    |
| Improvements ✅ <br /> (primary)   | -0.4% | [-1.6%, -0.2%] | 8     |
| Improvements ✅ <br /> (secondary) | -1.3% | [-1.7%, -0.2%] | 13    |
| All ❌✅ (primary)                 | 0.1%  | [-1.6%, 1.1%]  | 28    |

0 Regressions, 2 Improvements, 4 Mixed; 4 of them in rollups
51 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/e5ffd3c575a14d4a84f0e797c5006948424a2192/triage/2025-01-07.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [Explicit ABIs in extern](https://github.com/rust-lang/rfcs/pull/3722)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Tracking Issue for `float_next_up_down`](https://github.com/rust-lang/rust/issues/91399)
* [`Impl TryFrom<Vec<u8>> for String`](https://github.com/rust-lang/rust/pull/132268)
* [Tracking Issue for anonymous pipe API](https://github.com/rust-lang/rust/issues/127154)
* [Convert `struct FromBytesWithNulError` into enum](https://github.com/rust-lang/rust/pull/134143)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [simplify SourceID Ord/Eq](https://github.com/rust-lang/cargo/pull/14980)

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Proposals entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2025-01-08 - 2025-02-05 🦀

### Virtual
* 2025-01-08 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**BlockMesh Network implemented in Rust with Ohad Dahan (Virtual, English)**](https://www.meetup.com/code-mavens/events/304951805)
* 2025-01-09 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898167)
* 2025-01-09 | Virtual (Miami, FL, US) | [Rust Miami](https://www.meetup.com/rust-miami/)
    * [**Rust / Wasm on Serverless and Frontend**](https://www.meetup.com/rust-miami/events/305122950)
* 2025-01-09 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820279/)
* 2025-01-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/302815031)
* 2025-01-15 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**An introduction to WASM in Rust with Márk Tolmács (Virtual, English)**](https://www.meetup.com/code-mavens/events/305064546)
* 2025-01-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Leptos**](https://www.meetup.com/vancouver-rust/events/304051782)
* 2025-01-16 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633278/)
* 2025-01-16 | Virtual and In-Person (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug/events)
    * [**January Meetup**](https://www.meetup.com/join-srug/events/305505409/)
* 2025-01-21 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Exploring Rust Enums with Yoni Peleg (Virtual, Hebrew)**](https://www.meetup.com/rust-tlv/events/305110744)
* 2025-01-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/rdhhptyhccbcc)
* 2025-01-22 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #8**](https://www.meetup.com/bevy-game-development/events/305111151)
* 2025-01-23 & 2025-01-24 | Virtual | [Mainmatter Rust Workshop](https://ti.to/mainmatter/)
    * [**Remote Workshop: Testing for Rust projects – going beyond the basics**](https://ti.to/mainmatter/rust-testing-jan-2025)
* 2025-01-26 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Rust and embedded programming with Leon Vak (online in Hebrew)**](https://www.meetup.com/rust-tlv/events/304971264)
* 2025-01-28 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/305361243)
* 2025-01-30 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/299468340)
* 2025-01-30 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Quantum Computers Can’t Rust-Proof This!**](https://www.meetup.com/charlottesville-rust-meetup/events/305391474)
* 2025-01-30 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Are We Embedded Yet? - Implementing tiny HTTP server on a microcontroller**](https://www.meetup.com/code-mavens/events/305382647)
* 2025-02-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304216)

### Asia
* 2025-01-12 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**In person Rust January 2025 at Abra in Raanana**](https://www.meetup.com/rust-tlv/events/304898730/)

### Europe
* 2025-01-08 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona - 2025 01**](https://lu.ma/ckf2s00f)
* 2025-01-08 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust in January: How to (not) structure your Rust projects**](https://www.meetup.com/rustcologne/events/305388321)
* 2025-01-08 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305038426)
* 2025-01-09 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154281)
* 2025-01-14 | Mannheim, DE | [Hackschool - Rhein-Neckar](https://www.meetup.com/hackschool-rhein-neckar/events/)
    * [**Rust Your Engines #5**](https://www.meetup.com/hackschool-rhein-neckar/events/305230542)
* 2025-01-16 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Meetup @ Avalor AI**](https://www.meetup.com/rust-amsterdam-group/events/305339712)
* 2025-01-16 | Karlsruhe, DE | [Rust Hack & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe/events/)
    * [**Karlsruhe Rust Hack and Learn Meetup bei BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/305144321)
* 2025-01-18 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #8**](https://www.meetup.com/stockholm-rust/events/305475761)
* 2025-01-21 | Ghent, BE | [Systems Programming Ghent](https://sysghent.be)
    * [**Tech Talks & Dinner: Insights on Systems Programming Side Projects (in Rust) - Leptos (full-stack Rust with webassembly), Karyon (distributed p2p software in Rust), FunDSP (audio synthesis in Rust)**](https://www.meetup.com/systems-programming-ghent/events/305201540/?slug=systems-programming-ghent&eventId=305201540)
* 2025-01-21 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Self-Organized Peer-to-Peer Networks using Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/303604074)
* 2025-01-22 | Oberursel, DE | [Rust Rhein Main](https://www.meetup.com/rust-rhein-main)
    * [**Rust 2024 Edition and Beyond**](https://www.meetup.com/rust-rhein-main/events/305330873)
* 2025-01-23 | Barcelona, ES | [Barcelona Free Software](https://www.meetup.com/barcelona-free-software/events/)
    * [**Why Build a New Browser Engine in Rust?**](https://www.meetup.com/barcelona-free-software/events/305179554)
* 2025-01-23 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #74**](https://www.meetup.com/rust-paris/events/305455221)
* 2025-01-27 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/events/)
    * [**Rust Meetup Prague (January 2025)**](https://www.meetup.com/rust-prague/events/305455153)
* 2025-01-28 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Hack Night - Advent of Code**](https://www.meetup.com/rust-aarhus/events/304487851)
* 2025-01-30 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #11: Hypermedia-driven development in Rust**](https://rust-augsburg.github.io/meetup/Meetup_11.html)
* 2025-01-30 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299421383)
* 2025-02-01 | Brussels, BE | [FOSDEM 2025](https://fosdem.org/2025/)
    * [**FOSDEM Rust Devroom**](https://fosdem.org/2025/schedule/track/rust/)
* 2025-02-01 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Technikmuseum Sinsheim**](https://www.meetup.com/rust-noris/events/305361544)
* 2025-02-05 | Oxford, GB | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123401)

### North America
* 2025-01-08 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Dinner - Pinthouse Pizza South Lamar**](https://www.meetup.com/rust-atx/events/305125929)
* 2025-01-09 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/305044124)
* 2025-01-10 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/boston-rust-meetup-25317522anphwzdw/events/)
    * [**Lechmere Rust Lunch, Jan 10**](https://www.meetup.com/bostonrust/events/304951467)
* 2025-01-14 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/305460360)
* 2025-01-16 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust Game Development Series 1: Community Introductions**](https://www.meetup.com/music-city-rust-developers/events/304333017)
* 2025-01-16 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events)
    * [**January Meetup**](https://www.meetup.com/join-srug/events/305505409/)
* 2025-01-17 | México City, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Multithreading y Async en Rust 101 - HolaMundo - Parte 3**](https://www.meetup.com/rust-mx/events/305464827)
* 2025-01-18 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/boston-rust-meetup-25317522anphwzdw/events/)
    * [**Back Bay Rust Lunch, Jan 18**](https://www.meetup.com/bostonrust/events/304951470)
* 2025-01-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638258)
* 2025-01-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/305325657)
* 2025-01-23 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/305414182)


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

> Also, there is often a trade-off between accuracy and education. For example, when I correct my toddler that the Sun is actually not moving, but we are rotating. That's wrong, the Sun *is* moving, but arguably less wrong than his impression. (I once tried to give him the full explanation, but halfway through he ran away to play with his trains.)
>
> Not that readers of the Rust book are toddlers, but the principle generalizes in my experience.

– [Andrew Gallant a.k.a. @BurntSushi on rust-users](https://users.rust-lang.org/t/why-do-some-people-confound-t-with-stack-memory/123336/8)

Thanks to [Aleksander Krauze](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1648) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
