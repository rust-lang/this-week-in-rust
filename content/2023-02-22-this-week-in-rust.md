Title: This Week in Rust 483
Number: 483
Date: 2023-02-22
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
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
* [RustConf 2023: Call for Proposals](https://sessionize.com/rustconf-2023)

### Official
* [Language team advisors](https://blog.rust-lang.org/inside-rust/2023/02/14/lang-advisors.html)
* [Welcome Tyler Mandry to the Rust language team!](https://blog.rust-lang.org/inside-rust/2023/02/14/lang-team-membership-update.html)
* [Governance Reform RFC Announcement](https://blog.rust-lang.org/inside-rust/2023/02/22/governance-reform-rfc.html)

### Project/Tooling Updates
* [rust-analyzer Changelog #169](https://rust-analyzer.github.io/thisweek/2023/02/20/changelog-169.html)
* [Rust now available for Real-Time Operating System and Hypervisor PikeOS](https://www.sysgo.com/press-releases/rust-now-available-for-real-time-operating-system-and-hypervisor-pikeos)
* [Announcing Relm4 v0.5](https://relm4.org/blog/posts/announcing_relm4_v0.5)
* [Fornjot (code-first CAD in Rust) - Weekly Release - Accidental Side-Effect](https://www.fornjot.app/blog/weekly-release/2023-w08/)

### Observations/Thoughts
* [Why is building a UI in Rust so hard?](https://www.warp.dev/blog/why-is-building-a-ui-in-rust-so-hard)
* [Lightweight, Predictable Async Send Bounds](https://theincredibleholk.org/blog/2023/02/16/lightweight-predictable-async-send-bounds/)
* [Return type notation (send bounds, part 2)](https://smallcultfollowing.com/babysteps/blog/2023/02/13/return-type-notation-send-bounds-part-2/)
* [Faking Algebraic Effects and Handlers With Traits: A Rust Design Pattern](https://blog.shtsoft.eu/2022/12/22/effect-trait-dp.html)
* [winnow = toml_edit + combine + nom](https://epage.github.io/blog/2023/02/winnow-toml-edit-combine-nom/)
* [Battle Of The Backends: Rust vs. Go vs. C# vs. Kotlin - inovex GmbH](https://www.inovex.de/de/blog/rust-vs-go-vs-c-vs-kotlin/)
* [The Bull Case for Rust on the Web](https://driftingin.space/posts/bull-case-for-rust-on-web)
* [I love building a startup in Rust. I wouldn't pick it again.](https://www.propelauth.com/post/i-love-building-a-startup-in-rust-i-wouldnt-pick-it-again)
* [Rust development for the Raspberry PI on Apple Silicon](https://manuel.bernhardt.io/posts/2022-11-04-rust-development-for-the-raspberry-pi-on-apple-silicon/)

### Rust Walkthroughs
* [Learn how to build and deploy a down detector Telegram bot in Rust](https://joshmo.hashnode.dev/building-deploying-a-down-detector-telegram-bot-in-rust)
* [Compile Time Correctness: Type State](https://peace.mk/blog/compile-time-correctness-type-state/)
* [Build a casual side scroller with Rust](https://www.ardanlabs.com/blog/2023/02/build-a-casual-side-scroller-with-rust.html)
* [True Observer Pattern with Unsubscribe mechanism using Rust](https://ybnesm.github.io/blah/articles/true-observer-pattern-rust/)
* [Refactoring in Rust: Abstraction with the Newtype Pattern](https://fettblog.eu/refactoring-rust-abstraction-newtype/)
* [Rust to WebAssembly the hard way](https://surma.dev/things/rust-to-webassembly/)
* [STM32F4 Embedded Rust at the PAC: System Clock Configuration](https://apollolabsblog.hashnode.dev/stm32f4-embedded-rust-at-the-pac-system-clock-configuration)
* [Implement base64 encoding using Rust - (Part 1) Base64 for non-unicode characters](https://dev.to/quackquack/implement-base64-encoding-using-rust-part-1-base64-for-non-unicode-characters-4944)
* [Build a Apache Kafka Producer/Consumer Application in Rust](https://dev.to/schultyy/how-to-build-a-simple-kafka-producerconsumer-application-in-rust-3pl4)
* [Learning Rust by Building a To-Do App](https://dev.to/opendataanalytics/learning-rust-by-building-a-to-do-app-2okd)
* [A Nibble of Quadtrees in Rust](https://dev.to/kurt2001/a-nibble-of-quadtrees-in-rust-4o7g)
* [Embedded Rust on ESP32C3 Board, a Hands-on Quickstart Guide](https://dev.to/cyrilmarpaud/embedded-rust-on-esp32c3-board-a-hands-on-quickstart-guide-28mf)
* [How to make a Text Adventure game in Rust - X - More Attributes](https://www.riskpeep.com/2023/02/make-text-adventure-game-rust-10.html)
* [Nothing in Rust](https://geeklaunch.io/blog/nothing-in-rust/)
* [ES] [Aprendiendo Rust 🦀️ II. Programming a guessing game](https://dev.to/retronauta/libro-the-rust-programming-language-ii-programming-a-guessing-game-8ni)
* [video] [Speed up your Rust code with Rayon](https://youtu.be/YxG7PhZ3fb4)
* [video] [Making Custom Asset Types: Platformer In Bevy #4](https://youtu.be/BWhe2UyJv1E)
* [Implementing a Binary Tree in Rust](https://rusty-ferris.pages.dev/blog/binary-tree-sum-of-values/)
* [Run WebAssembly from your Rust Program](https://21-lessons.com/how-to-run-webassembly-from-your-rust-program/)

### Research
* [The Usability of Advanced Type Systems: Rust as a Case Study](https://arxiv.org/abs/2301.02308)

### Miscellaneous
* [Learn Rust With JetBrains IDEs](https://blog.jetbrains.com/rust/2023/02/21/learn-rust-with-jetbrains-ides/)
* [Rust in Rhymes II explainer](https://llogiq.github.io/2023/02/19/rhymes2.html)
* [audio] [Lodestone with Wilbur Zhang, Peter Jiang, and Kevin Huang](https://rustacean-station.org/episode/lodestone/)
* [Rust Nation UK 2023](https://www.jetstack.io/blog/rust-nation-uk-2023/)

## Crate of the Week

This week's crate is [Darkbird](https://github.com/Rustixir/darkbird), a high-concurrency real-time in-memory database.

Thanks to [DanyalMh](https://users.rust-lang.org/t/crate-of-the-week/2704/1160) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [miri - Get Miri working on ARM again](https://github.com/rust-lang/miri/issues/2791)
* [man-in-the-middle-proxy - Add Custom headers requests](https://github.com/emanuele-em/man-in-the-middle-proxy/issues/9)
* [Ockam - Create clap command to delete a TCP Outlet on a node](https://github.com/build-trust/ockam/issues/4268)
* [Ockam - Create clap command to delete a TCP Inlet on a node](https://github.com/build-trust/ockam/issues/4269)
* [Ockam - Add a Github Action to avoid conflicts in TypeTag ids](https://github.com/build-trust/ockam/issues/4108)
* [Ockam - Remove the disable/enable_check_credential arguments from ockam tcp-outlet create](https://github.com/build-trust/ockam/issues/4307)
* [Ockam - Remove the disable/enable_check_credential arguments from ockam tcp-inlet create](https://github.com/build-trust/ockam/issues/4308)
* [Ockam - Update ockam project addon configure influx-db clap command to ockam project addon configure influxdb](https://github.com/build-trust/ockam/issues/4318)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

396 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-02-13..2023-02-20

* [wasm: register the `relaxed-simd` target feature](https://github.com/rust-lang/rust/pull/108086)
* [enable `#[thread_local]` on armv6k-nintendo-3ds](https://github.com/rust-lang/rust/pull/107968)
* [add sanitizer support for modern iOS platforms](https://github.com/rust-lang/rust/pull/106476)
* [add `kernel-address` sanitizer support for freestanding targets](https://github.com/rust-lang/rust/pull/99679)
* [add an unstable `#[rustc_coinductive]` attribute](https://github.com/rust-lang/rust/pull/108033)
* [added another error to be processed in fallback](https://github.com/rust-lang/rust/pull/107985)
* [check that built-in callable types validate their output type is `Sized` (in new solver)](https://github.com/rust-lang/rust/pull/107867)
* [implement partial support for non-lifetime binders](https://github.com/rust-lang/rust/pull/107489)
* [deny non-lifetime bound vars in `for<..> ||` closure binders](https://github.com/rust-lang/rust/pull/108186)
* [don't call `with_reveal_all_normalized` in const-eval when `param_env` has inference vars in it](https://github.com/rust-lang/rust/pull/107542)
* [don't eagerly convert principal to string](https://github.com/rust-lang/rust/pull/108162)
* [don't recover lifetimes/labels containing emojis as character literals](https://github.com/rust-lang/rust/pull/108031)
* [don't suggest `#[doc(hidden)]` trait methods with matching return type](https://github.com/rust-lang/rust/pull/108049)
* [make codegen choose whether to emit overflow checks](https://github.com/rust-lang/rust/pull/107921)
* [fix RPITITs in default trait methods (by assuming projection predicates in param-env)](https://github.com/rust-lang/rust/pull/108203)
* [fix json reexports of different items with same name](https://github.com/rust-lang/rust/pull/107766)
* [improve the suggestion on future not awaited](https://github.com/rust-lang/rust/pull/107902)
* [unexpected trait bound not satisfied in HRTB and Associated Type](https://github.com/rust-lang/rust/pull/103695)
* [make `dyn*`'s value backend type a pointer](https://github.com/rust-lang/rust/pull/107772)
* [more accurate spans for arg removal suggestion](https://github.com/rust-lang/rust/pull/106347)
* [enable CopyProp](https://github.com/rust-lang/rust/pull/107449)
* [enable instcombine for mutable reborrows](https://github.com/rust-lang/rust/pull/105274)
* [factor query arena allocation out from query caches](https://github.com/rust-lang/rust/pull/107833)
* [avoid accessing HIR when it can be avoided](https://github.com/rust-lang/rust/pull/108006)
* [optimize `LazyLock` size](https://github.com/rust-lang/rust/pull/107329)
* [optimize `mk_region`](https://github.com/rust-lang/rust/pull/108020)
* [prevent some attributes from being merged with others on reexports](https://github.com/rust-lang/rust/pull/108057)
* [remove save-analysis](https://github.com/rust-lang/rust/pull/101841)
* [rework `min_choice` algorithm of member constraints](https://github.com/rust-lang/rust/pull/105300)
* [suggest fix for misplaced generic params on fn item](https://github.com/rust-lang/rust/pull/103478)
* [suggest the correct array length on mismatch](https://github.com/rust-lang/rust/pull/107173)
* [tighter spans for bad inherent `impl` self types](https://github.com/rust-lang/rust/pull/107942)
* [type-directed probing for inherent associated types](https://github.com/rust-lang/rust/pull/105961)
* [use `is_str` instead of string kind comparison](https://github.com/rust-lang/rust/pull/108007)
* [use `target` instead of `machine` for mir interpreter integer handling](https://github.com/rust-lang/rust/pull/108047)
* [use covariance on type relations of field projection types if possible](https://github.com/rust-lang/rust/pull/107969)
* [use derive attributes for uninteresting traversals](https://github.com/rust-lang/rust/pull/108040)
* [use id-based thread parking on SOLID](https://github.com/rust-lang/rust/pull/106372)
* [use restricted Damerau-Levenshtein distance for diagnostics](https://github.com/rust-lang/rust/pull/108200)
* [use semantic equality for const param type equality assertion](https://github.com/rust-lang/rust/pull/107940)
* [constify `RangeBounds`, `RangeX::contains` and `RangeX::is_empty` (where applicable)](https://github.com/rust-lang/rust/pull/108084)
* [implement more methods for `vec_deque::IntoIter`](https://github.com/rust-lang/rust/pull/106241)
* [use custom implementation of `read_buf` in Read for &'a FileDesc](https://github.com/rust-lang/rust/pull/108235)
* [futures: add `AbortHandle::is_aborted()`](https://github.com/rust-lang/futures-rs/pull/2710)
* [cargo: enhance help texts of position args](https://github.com/rust-lang/cargo/pull/11740)
* [only include stable lints in `rustdoc::all` group](https://github.com/rust-lang/rust/pull/106316)
* [rustdoc: perform name resolver cleanups](https://github.com/rust-lang/rust/pull/107765)
* [rustdoc: correctly handle links starting with whitespace](https://github.com/rust-lang/rust/pull/108129)
* [rustdoc: cleanup doc link extraction](https://github.com/rust-lang/rust/pull/108209)
* [rustfmt: use correct span for struct generics](https://github.com/rust-lang/rustfmt/pull/5692)
* [bindgen: add support for enums with the wrapped static functions feature](https://github.com/rust-lang/rust-bindgen/pull/2415)
* [clippy: add `let_underscore_untyped` lint](https://github.com/rust-lang/rust-clippy/pull/10356)
* [clippy: add `question_mark_used` lint](https://github.com/rust-lang/rust-clippy/pull/10342)
* [clippy: add the `transmute_int_to_non_zero` lint](https://github.com/rust-lang/rust-clippy/pull/10360)
* [clippy: add `significant_drop_tightening` lint](https://github.com/rust-lang/rust-clippy/pull/10163)
* [clippy: `significant_drop_tightening`: evaluate the return expression of a block](https://github.com/rust-lang/rust-clippy/pull/10368)
* [clippy: `significant_drop_tightening`: ignore inexpensive statements](https://github.com/rust-lang/rust-clippy/pull/10363)
* [clippy: fix false positives for `extra_unused_type_parameters`](https://github.com/rust-lang/rust-clippy/pull/10321)
* [clippy: stop `bytes_nth` from suggesting code that does not compile](https://github.com/rust-lang/rust-clippy/pull/10361)
* [clippy: stop `doc_markdown` requiring backticks on links to external websites](https://github.com/rust-lang/rust-clippy/pull/10357)
* [clippy: `box_default`: don't omit the type of the removed trait object](https://github.com/rust-lang/rust-clippy/pull/10382)
* [clippy: `manual_let_else`: do not suggest semantically different replacements](https://github.com/rust-lang/rust-clippy/pull/10336)
* [clippy: `manual_let_else`: let/else is not divergent by default](https://github.com/rust-lang/rust-clippy/pull/10332)
* [clippy: `never_loop` Fix false positive with labeled blocks](https://github.com/rust-lang/rust-clippy/pull/10311)
* [clippy: `uninlined_format_args`: do not inline argument with generic parameters](https://github.com/rust-lang/rust-clippy/pull/10343)
* [clippy: change `unusual_byte_groupings` to only require byte groupings of equal size](https://github.com/rust-lang/rust-clippy/pull/10353)
* [clippy: do not base `map_entry` lint suggestion on expanded code](https://github.com/rust-lang/rust-clippy/pull/10346)
* [clippy: fix `needless_return` incorrect suggestion when returning if sequence](https://github.com/rust-lang/rust-clippy/pull/10345)
* [clippy: ignore synthetic type parameters for `extra_unused_type_parameters`](https://github.com/rust-lang/rust-clippy/pull/10338)
* [clippy: liberate late-bound regions rather than erasing them in `needless_pass_by_value`](https://github.com/rust-lang/rust-clippy/pull/10328)
* [rust-analyzer: add v7 metadata support to rust-analyzer](https://github.com/rust-lang/rust-analyzer/pull/14153)
* [rust-analyzer: don't assume VSCode internal commands in the server](https://github.com/rust-lang/rust-analyzer/pull/14147)
* [rust-analyzer: support UTF-32 position encoding](https://github.com/rust-lang/rust-analyzer/pull/14141)
* [rust-analyzer: adjust binding mode inlay hints to render better with @ patterns](https://github.com/rust-lang/rust-analyzer/pull/14157)
* [rust-analyzer: bring back hovering call parens for return type info](https://github.com/rust-lang/rust-analyzer/pull/14160)
* [rust-analyzer: don't expand macros in the same expansion tree after overflow](https://github.com/rust-lang/rust-analyzer/pull/14122)
* [rust-analyzer: don't trigger postfix completion in `if` block which has an `else` block](https://github.com/rust-lang/rust-analyzer/pull/14123)
* [rust-analyzer: search raw identifiers without prefix](https://github.com/rust-lang/rust-analyzer/pull/14144)
* [rust-analyzer: trigger call info for more completions of signature having things](https://github.com/rust-lang/rust-analyzer/pull/14149)

### Rust Compiler Performance Triage

Overall a fairly positive week, with few noise-related regressions or
improvements and many benchmarks showing significant improvements. The one
large regression is limited to documentation builds and has at least a partial
fix already planned.

Other wins this week include an average [improvement][memopt] of around 1% in
maximum memory usage of optimized builds, and a 2% average [reduction][sizeopt]
in compiled binary sizes. These are fairly significant wins for these metrics.

[memopt]: https://perf.rust-lang.org/?start=9bb6e60d1f1360234aae90c97964c0fa5524f141&end=3fee48c161a48b0c142d3998fff56faee96bd56c&absolute=false&stat=max-rss&kind=percentfromfirst
[sizeopt]: https://perf.rust-lang.org/?start=9bb6e60d1f1360234aae90c97964c0fa5524f141&end=3fee48c161a48b0c142d3998fff56faee96bd56c&absolute=false&stat=size%3Alinked_artifact&kind=percentfromfirst

Triage done by **@simulacrum**.
Revision range: [9bb6e60..3fee48c1](https://perf.rust-lang.org/?start=9bb6e60d1f1360234aae90c97964c0fa5524f141&end=3fee48c161a48b0c142d3998fff56faee96bd56c&absolute=false&stat=instructions%3Au)

3 Regressions, 3 Improvements, 3 Mixed; 2 of them in rollups
45 artifact comparisons made in total

[Full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-02-21.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [rustdoc: search by macro when query ends with `!`](https://github.com/rust-lang/rust/pull/108143)
* [disposition: merge] [Stabilize rustdoc `--test-run-directory`](https://github.com/rust-lang/rust/pull/103682)
* [disposition: merge] [Treat `str` as containing `[u8]` for auto trait purposes](https://github.com/rust-lang/rust/pull/107941)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [Stabilize may_dangle](https://github.com/rust-lang/rfcs/pull/3390)
* [new] [Add a `[lints]` table to `Cargo.toml`](https://github.com/rust-lang/rfcs/pull/3389)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-02-22 - 2023-03-22 🦀

### Virtual

* 2023-02-23 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Tock, a Rust based Embedded Operating System**](https://www.meetup.com/charlottesville-rust-meetup/events/291248593/)
* 2023-02-23 | Virtual (Kassel, DE) | [Java User Group Hessen](https://www.meetup.com/java-user-group-hessen-jugh/)
    * [**Eine Einführung in Rust (Stefan Baumgartner)**](https://www.meetup.com/java-user-group-hessen-jugh/events/290346591/)
* 2023-02-23 | Virtual (México City, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Rust: ¿por qué es una opción adecuada para implantar Blockchain?**](https://www.meetup.com/rust-mx/events/291456677/)
* 2023-02-24 | Virtual (Tunis, TN) | [Rust Meetup Tunisia](https://www.meetup.com/rust-tunisia/)
    * [**Rust Meetup Tunisia - Volume I, Number II**](https://www.meetup.com/rust-tunisia/events/291534817/)
* 2023-02-28 | Virtual (Berlin, DE) | [Open Tech School Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/290852327/)
* 2023-02-28 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust Nation - What we learnt**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/291338734/)
* 2023-02-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfcdblc/)
* 2023-02-28 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/291437669/)
* 2023-03-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - Michael Baykov on Category Theory & Argument Parsing**](https://www.meetup.com/indyrs/events/qwtdjsyfcfbcb/)
* 2023-03-02 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 30th Edition**](https://www.meetup.com/rust-linz/events/291483339/)
* 2023-03-07 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfcfbkb/)
* 2023-03-08 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsryfcfblb/)
* 2023-03-11 | Virtual | [Rust GameDev](https://discord.gg/yNtPTb2)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2)
* 2023-03-14 | Virtual (Berlin, DE) | [Berlin.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2023/03/14/rust-hack-and-learn.html)
* 2023-03-15 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Injecting Rust Hooks into a 1999 game binary (unsafe)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/291354288/)
* 2023-03-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/wqchctyfcfbtb/)
* 2023-03-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/vdhxgsyfcfbcc/)

### Asia

* 2023-03-04 | Kyoto, JP | [Kansai Rust](https://www.meetup.com/kansai-rust/)
    * [**Fn vs FnMut vs FnOnce**](https://www.meetup.com/kansai-rust/events/291614614/)

### Europe

* 2023-02-23 | Bordeaux, FR | [DedoTalk](https://www.meetup.com/dedotalk/)
    * [**#1 DedoTalk 🎙️ : Rust pour un développeur Python**](https://www.meetup.com/dedotalk/events/291199962/)
* 2023-02-23 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust metup #33**](https://www.meetup.com/copenhagen-rust-community/events/291288154/)
* 2023-02-23 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna)
    * [**Rust Meetup Revived with an Exciting Exploration of Ownership!**](https://www.meetup.com/rust-vienna/events/291465732/)
* 2023-02-28 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/291437669/)
* 2023-02-28 | Nijmegen, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Regular track: Rust at RU**](https://www.meetup.com/rust-nederland/events/291489123/)
    * [**Student track: Rust at RU**](https://www.meetup.com/rust-nederland/events/291488539/)
* 2023-03-01 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/events)
    * [**Rust traits for Fn and profit**](https://www.meetup.com/rustcologne/events/291774935/)
* 2023-03-02 | Barcelona, ES | [BcnRust](https://bcnrust.github.io/)
    * [**9th BcnRust Meetup: Full Stack**](https://www.meetup.com/es-ES/bcnrust/events/291754590/)
* 2023-03-02 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #32**](https://www.meetup.com/rust-wroclaw/events/291776357/)
* 2023-03-07 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake**](https://www.meetup.com/bratislava-rust-meetup-group/events/291657555/)   
* 2023-03-09 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**Rust Meetup #7**](https://www.meetup.com/rust-basel/events/291228934/)
* 2023-03-09 | Delft, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Regular track: Embedded Rust**](https://www.meetup.com/rust-nederland/events/291401965/)
    * [**Student track: Embedded Rust**](https://www.meetup.com/rust-nederland/events/291401778/)
* 2023-03-09 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #2**](https://www.meetup.com/fr-FR/rust-lyon/events/291727241/)
* 2023-03-15 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Walk around Embedded World Exhibition**](https://www.meetup.com/rust-noris/events/291623203/)

### North America

* 2023-02-23 | Mountain View, CA, US | [Mountain View Rust Study Group](https://www.meetup.com/rust-study-group/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/rust-study-group/events/291623636/)
* 2023-03-01 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/291619816/)
* 2023-03-09 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Trails, Triumphs, & Travails of Yet-Another-Database-Crate with PJ and Food!**](https://www.meetup.com/utah-rust/events/rrwbctyfcfbmb/)

### Oceania

* 2023-02-23 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**February Meetup**](https://www.meetup.com/rust-brisbane/events/291377036/)
* 2023-02-28 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**February Meetup**](https://www.meetup.com/rust-canberra/events/291278417/)
* 2023-03-01 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**🦀 Lightning Talks - We are back!**](https://www.meetup.com/rust-sydney/events/291265163/)

### South America

* 2023-02-22 | Montevideo, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay/)
    * [**Hands on: Lifetimes**](https://www.meetup.com/rust-uruguay/events/291386143/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/10nmtew/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> It’s *enjoyable* to write Rust, which is maybe kind of weird to say, but it’s just the language is fantastic. It’s fun. You feel like a magician, and that never happens in other languages.

– [Parker Timmerman cited in a TechnologyReview article](https://www.technologyreview.com/2023/02/14/1067869/rust-worlds-fastest-growing-programming-language/)

Thanks to [robin](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1373) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/119s0gv/this_week_in_rust_483/)</small>
