Title: This Week in Rust 467
Number: 467
Date: 2022-11-02
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

### Official
* [Generic associated types to be stable in Rust 1.65](https://blog.rust-lang.org/2022/10/28/gats-stabilization.html)

### Foundation
* [Project Grants Application Deadline Extended to November 7, 2022](https://foundation.rust-lang.org/news/project-grants-application-deadline-extended-to-november-7-2022/)
* [Q3 2022 Recap: A summary from Rebecca Rumbul](https://foundation.rust-lang.org/news/q3-2022-update-a-recap-from-rebecca-rumb/)

### Project/Tooling Updates
* [Announcing async-backtrace](https://tokio.rs/blog/2022-10-announcing-async-backtrace)
* [This Month in hyper: October 2022](https://seanmonstar.com/post/699744578823127040/this-month-in-hyper-october-2022)
* [rust-analyzer changelog #153](https://rust-analyzer.github.io/thisweek/2022/10/31/changelog-153.html)
* [rustc_codegen_gcc: Progress Report #17](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-17)
* [From Fuzzing to Proof: Using Kani with the Bolero Property-Testing Framework](https://model-checking.github.io/kani-verifier-blog/2022/10/27/using-kani-with-the-bolero-property-testing-framework.html)
* [dfdx v0.10.0 - deep learning now with even more const generic goodness](https://coreylowman.github.io/2022/10/30/release-0.10.0.html)
* [SeaQuery 0.27.0 - A dynamic query builder for SeaORM](https://www.sea-ql.org/blog/2022-10-31-whats-new-in-seaquery-0.27.0/)
* [Slint 0.3.1 - GUI framework in Rust](https://slint-ui.com/thisweek/2022-10-31.html)
* [headtail: head & tail simultaneously, written in Rust](https://agile-perception.ghost.io/headtail-head-tail-simultaneously/)

### Observations/Thoughts
* [Mini-post: the role of Rust's teams](https://www.ncameron.org/blog/mini-post-the-role-of-rusts-teams/)
* [Do we need a "Rust Standard"?](https://blog.m-ou.se/rust-standard/)
* [Rustdoc doctests need fixing](https://swatinem.de/blog/fix-rustdoc/)
* [Ranged integers](https://cohost.org/oli-obk/post/148312-ranged-integers)
* [Ranged integers via subtypes](https://cohost.org/oli-obk/post/165584-ranged-integers-via)
* [The stable HashMap trap](https://morestina.net/blog/1843/the-stable-hashmap-trap)
* [Const Syntax](https://blog.yoshuawuyts.com/const-syntax/)
* [How to speed up the Rust compiler in October 2022](https://nnethercote.github.io/2022/10/27/how-to-speed-up-the-rust-compiler-in-october-2022.html)
* [Speeding up the Rust compiler without changing its code](https://kobzol.github.io/rust/rustc/2022/10/27/speeding-rustc-without-changing-its-code.html)
* [Exploring technologies at Weborama: The Rust language](https://medium.com/weborama/introduction-to-the-rust-programming-language-4dbf8f3016de)
* [Lock-free webserver using channels in Rust](https://www.kurtlawrence.info/blog/lock-free-webserver-using-channels-in-rust)
* [video] [A Cellular Automaton with Rust and Bevy - Diemo Heuer](https://www.youtube.com/watch?v=rOFlUvMfOHQ)
* [video] [AMD Hypervisor with Rust - Matthias Heiden](https://www.youtube.com/watch?v=7igpsgCZJY4)
* [video] [Boxes, Heaps, and Stacks - Tim McNamara](https://www.youtube.com/watch?v=DEE1GKMbtgw)
* [video] [Rustberry Pi: Baby-steps in Embedded Rust - Lisa Passing](https://www.youtube.com/watch?v=IgC2HvBesms)
* [video] [Workshop: Rust for Artists - Lisa Passing](https://www.youtube.com/watch?v=jB7aJDAvSuo)
* [video] [Intro to Tower and the Service Trait - Stefan Baumgartner](https://www.youtube.com/watch?v=z78_RnUPnpY)
* [video] [Rust Before Main - Ryan Levick](https://www.youtube.com/watch?v=q8irLfXwaFM)
* [video] [How does the detour crate work?](https://fasterthanli.me/videos/how-does-the-detour-crate-work)

### Rust Walkthroughs
* [Implementing Rayon’s Parallel Iterators](https://geo-ant.github.io/blog/2022/implementing-parallel-iterators-rayon/)
* [Tower, Episode 1: your Service as a Function](https://heikoseeberger.de/2022/10/21/2022-10-21-tower-1/)
* [Rust microservices in server-side WebAssembly](https://blog.logrocket.com/rust-microservices-server-side-webassembly/)
* [4 Simple Steps for Creating a Platform Agnostic Driver in Rust](https://apollolabsblog.hashnode.dev/4-simple-steps-for-creating-a-platform-agnostic-driver-in-rust)
* [video] [Work on gpu library](https://www.youtube.com/watch?v=P5lab7t1hVM)
* [Fathomable Rust Macros](https://geeklaunch.net/blog/fathomable-rust-macros/)
* [Property-Based Testing in Rust with Arbitrary](https://www.greyblake.com/blog/property-based-testing-in-rust-with-arbitrary/)

### Research
* [Static Information Flow Control Made Simpler](https://arxiv.org/abs/2210.12996)
* [Unsafe's Betrayal: Abusing Unsafe Rust in Binary Reverse Engineering toward Finding Memory-safety Bugs via Machine Learning](https://arxiv.org/abs/2211.00111)

### Miscellaneous
* [Rust könnte offizielle Spezifikation bekommen](https://www.golem.de/news/programmierung-rust-koennte-offizielle-spezifikation-bekommen-2210-169286.html)

- [video] [Programming Veloren - the multiplayer voxel RPG written in Rust](https://www.youtube.com/watch?v=bT2SeYXpQm8)

## Crate of the Week

This week's crate is [type\_description](https://crates.io/crates/type_description), a crate to make types discoverable for users by explaining them in a way that a user can understand without knowing implementation details.

Thanks to [musicmatze](https://users.rust-lang.org/t/crate-of-the-week/2704/1119) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

433 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-10-24..2022-10-31

* [add a tier 3 target for the Sony PlayStation 1](https://github.com/rust-lang/rust/pull/102689)
* [add flag to forbid recovery in the parser](https://github.com/rust-lang/rust/pull/103544)
* [allow `impl Fn() -> impl Trait` in return position](https://github.com/rust-lang/rust/pull/93582)
* [prevent foreign Rust exceptions from being caught](https://github.com/rust-lang/rust/pull/102721)
* [add diagnostic for calling a function with the same name with unresolved Macro](https://github.com/rust-lang/rust/pull/103140)
* [add suggestions for unsafe impl error codes](https://github.com/rust-lang/rust/pull/103283)
* [change terminology for assoc method suggestions when they are not called](https://github.com/rust-lang/rust/pull/103350)
* [diagnostic derives: allow specifying multiple alternative suggestions](https://github.com/rust-lang/rust/pull/103209)
* [diagnostics: do not suggest static candidates as traits to import](https://github.com/rust-lang/rust/pull/103550)
* [emit a nicer error on `impl Self {`](https://github.com/rust-lang/rust/pull/103609)
* [emit proper error when casting to `dyn*`](https://github.com/rust-lang/rust/pull/103699)
* [enable varargs support for calling conventions other than C or cdecl](https://github.com/rust-lang/rust/pull/97971)
* [even nicer errors from `assert_unsafe_precondition`](https://github.com/rust-lang/rust/pull/103035)
* [filter candidates in pick probe for diagnostics](https://github.com/rust-lang/rust/pull/103415)
* [fix `unreachable_pub` suggestion for enum with fields](https://github.com/rust-lang/rust/pull/103338)
* [name the `impl Trait` in region bound suggestions](https://github.com/rust-lang/rust/pull/103416)
* [remap early bound lifetimes in return-position `impl Trait` in traits too](https://github.com/rust-lang/rust/pull/103608)
* [remove extra type error after missing semicolon error](https://github.com/rust-lang/rust/pull/103444)
* [libtest: do fewer passes and generally be more efficient when filtering tests](https://github.com/rust-lang/rust/pull/103689)
* [libtest: sort tests at compile time, not at startup](https://github.com/rust-lang/rust/pull/99939)
* [suggest type annotation for local statement initialed by ref expression](https://github.com/rust-lang/rust/pull/102951)
* [miri: implement `ptr_mask` intrinsic](https://github.com/rust-lang/miri/pull/2624)
* [miri: implement thread parking for Windows](https://github.com/rust-lang/miri/pull/2630)
* [miri: stacked Borrows:  make scalar field retagging the default](https://github.com/rust-lang/miri/pull/2636)
* [miri: support timeouts with monotonic clocks even when isolation is enabled](https://github.com/rust-lang/miri/pull/2631)
* [miri: test on windows-gnu target](https://github.com/rust-lang/miri/pull/2621)
* [use `br` instead of `switch` in more cases](https://github.com/rust-lang/rust/pull/103331)
* [perf improvements for effective visibility calculating](https://github.com/rust-lang/rust/pull/103158)
* [introduce UnordMap, UnordSet, and UnordBag (MCP 533)](https://github.com/rust-lang/rust/pull/102698)
* [llvm-16: don't initialize removed legacy passes](https://github.com/rust-lang/rust/pull/103549)
* [codegen\_gcc: add missing register class conversion for inline asm](https://github.com/rust-lang/rustc_codegen_gcc/pull/232)
* [codegen\_gcc: fix gcc build instructions](https://github.com/rust-lang/rustc_codegen_gcc/pull/238)
* [make `CStr::from_ptr` `const`](https://github.com/rust-lang/rust/pull/102961)
* [make `core::mem::copy` `const`](https://github.com/rust-lang/rust/pull/100006)
* [`poll_fn` and Unpin:  fix pinning](https://github.com/rust-lang/rust/pull/102737)
* [stabilize `Option::unzip()`](https://github.com/rust-lang/rust/pull/98204)
* [stabilize `arbitrary_enum_discriminant,` take 2](https://github.com/rust-lang/rust/pull/95710)
* [stabilize `duration_checked_float`](https://github.com/rust-lang/rust/pull/102271)
* [simd: vectors of pointers](https://github.com/rust-lang/portable-simd/pull/287)
* [stdarch: fix undefined behavior in SSE4.2 test](https://github.com/rust-lang/stdarch/pull/1341)
* [cargo: add Accept-Encoding request header to enable compression](https://github.com/rust-lang/cargo/pull/11292)
* [cargo: fix confusing error messages when using `-Zsparse-registry`](https://github.com/rust-lang/cargo/pull/11283)
* [cargo: improve the error message if `publish` is `false` or empty list](https://github.com/rust-lang/cargo/pull/11280)
* [cargo: report crate size on package and publish](https://github.com/rust-lang/cargo/pull/11270)
* [rustdoc: add support for incoherent impls on structs and traits](https://github.com/rust-lang/rust/pull/103746)
* [rustdoc: do not filter out cross-crate `Self:  Sized` bounds](https://github.com/rust-lang/rust/pull/103254)
* [rustdoc: don't mark `Box<T>` as `Iterator`, `Read`, etc](https://github.com/rust-lang/rust/pull/103432)
* [clippy: `bool_to_int_with_if` do not lint in const context](https://github.com/rust-lang/rust-clippy/pull/9738)
* [clippy: `option_if_let_else` do not lint if any arm has guard](https://github.com/rust-lang/rust-clippy/pull/9747)
* [clippy: `question_mark` don't lint on `if let Err` with `else`](https://github.com/rust-lang/rust-clippy/pull/9722)
* [clippy: `use_self` fix FP when trait impl defined in macro](https://github.com/rust-lang/rust-clippy/pull/9704)
* [clippy: `use_self` fix suggestion when full path to struct was given](https://github.com/rust-lang/rust-clippy/pull/9726)
* [clippy: add lint for confusing use of `^` instead of `.pow`](https://github.com/rust-lang/rust-clippy/pull/9506)
* [clippy: add lint to tell about the `let else` pattern](https://github.com/rust-lang/rust-clippy/pull/8437)
* [clippy: add new lint `seek_to_start_instead_of_rewind`](https://github.com/rust-lang/rust-clippy/pull/9667)
* [clippy: add new lint `seek_from_current`](https://github.com/rust-lang/rust-clippy/pull/9681)
* [clippy: ensure `new_ret_no_self` is not fired if `impl Trait<Self>` is returned](https://github.com/rust-lang/rust-clippy/pull/9733)
* [clippy: fix `bool_to_int_with_if` false positive with `if let`](https://github.com/rust-lang/rust-clippy/pull/9714)
* [clippy: fix `needless_borrow` false positive](https://github.com/rust-lang/rust-clippy/pull/9674) & [fix another `needless_borrow` false positive](https://github.com/rust-lang/rust-clippy/pull/9711)
* [clippy: fix the `string-extend-chars` suggestion on slice](https://github.com/rust-lang/rust-clippy/pull/9741)
* [clippy: make ignored internally mutable types for `mutable-key` configurable](https://github.com/rust-lang/rust-clippy/pull/9692)
* [clippy: move `uninlined_format_args` to pedantic](https://github.com/rust-lang/rust-clippy/pull/9728)
* [clippy: remove note mentioning configuration changes need cargo clean](https://github.com/rust-lang/rust-clippy/pull/9717)
* [clippy: warn when `clippy::restriction` is enabled via the command line](https://github.com/rust-lang/rust-clippy/pull/9755)
* [rust-analyzer: clicking the status bar item stops and starts the server](https://github.com/rust-lang/rust-analyzer/pull/13510)
* [rust-analyzer: type inference for generic associated types](https://github.com/rust-lang/rust-analyzer/pull/13494)
* [rust-analyzer: disregard type variable expectation for if expressions](https://github.com/rust-lang/rust-analyzer/pull/13523)
* [rust-analyzer: don't respond with an error when requesting a shutdown while starting](https://github.com/rust-lang/rust-analyzer/pull/13476)
* [rust-analyzer: fix standard flycheck command not being executed in the workspace it is being invoked for](https://github.com/rust-lang/rust-analyzer/pull/13478)
* [rust-analyzer: test all generic args for trait when finding matching impl](https://github.com/rust-lang/rust-analyzer/pull/13475)

### Rust Compiler Performance Triage

Noise continues to make triaging a bit tedious. We've become good at identifying noise, but we may need to invest in trying to reduce it or automate some of the triaging needed to identify it. In terms of performance, this week ending up being positive albeit with improvements only outweighing regressions by a little. Some of the largest improvements were in reverts of previous regressions as well.

Triage done by **@rylev**.
Revision range: [629a414d..822f8](https://perf.rust-lang.org/?start=629a414d7ba4caa3ca28b0a46c478e2ecb4c0059&end=822f8c22f540b12f296d844ad5bf39aaa47bfeb4&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 1.7%  | [0.2%, 7.9%]   | 28    |
| Regressions ❌ <br /> (secondary)  | 1.7%  | [0.2%, 7.0%]   | 97    |
| Improvements ✅ <br /> (primary)   | -1.2% | [-4.6%, -0.2%] | 73    |
| Improvements ✅ <br /> (secondary) | -1.3% | [-2.6%, -0.3%] | 61    |
| All ❌✅ (primary)                 | -0.4% | [-4.6%, 7.9%]  | 101   |

13 Regressions, 9 Improvements, 5 Mixed; 9 of them in rollups
41 artifact comparisons made in total

See [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-11-02.md) for details.

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* [Have `cargo publish` block until crate is published](https://github.com/rust-lang/cargo/issues/9507#issuecomment-1296058029)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Add lang-team advisors team](https://github.com/rust-lang/rfcs/pull/3327)
* [disposition: merge] [Support upcasting of dyn Trait values](https://github.com/rust-lang/rfcs/pull/3324)
* [disposition: close] [Short Macro Invocation Syntax: m!123 and m!"abc"](https://github.com/rust-lang/rfcs/pull/3267)
* [disposition: merge] [crates.io token scopes](https://github.com/rust-lang/rfcs/pull/2947)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Use `token::Lit` in `ast::ExprKind::Lit`.](https://github.com/rust-lang/rust/pull/102944)
* [disposition: merge] [Make `Sized` coinductive, again](https://github.com/rust-lang/rust/pull/100386)
* [disposition: merge] [Stabilize const char convert](https://github.com/rust-lang/rust/pull/102470)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [Add `exhaustive_match_output` RFC](https://github.com/rust-lang/rfcs/pull/3340)
* [new] [Update the RFC template](https://github.com/rust-lang/rfcs/pull/3339)
* [new] [Style evolution](https://github.com/rust-lang/rfcs/pull/3338)
* [new] [MaybeDangling](https://github.com/rust-lang/rfcs/pull/3336)

## Upcoming Events

Rusty Events between 2022-11-02 - 2022-11-30 🦀

### Virtual

* 2022-11-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcpbdb/)
* 2022-11-02 | Virtual (Redmond, WA, US / San Francisco, SF, US / New York, NY, US / Toronto, CA / London, UK) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Getting Started with Rust: From Java Dev to Rust Developer**](https://www.meetup.com/microsoft-reactor-redmond/events/288475833/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/288475838/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/288475839/) | [**Toronto Mirror**](https://www.meetup.com/microsoft-reactor-toronto/events/288475836/) | [**London Mirror**](https://www.meetup.com/microsoft-reactor-london/events/288475832/) 
* 2022-11-08 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/289211414/)
* 2022-11-08 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydcpblb/)
* 2022-11-08 | Virtual (Rostock, DE) | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/289352420/)
* 2022-11-08 | Virtual (Stockholm, SE) | [Func Prog Sweden](https://www.meetup.com/func-prog-sweden/)
    * [**Tenth Func Prog Sweden MeetUp 2022 – Online (with "Ready for Rust" by Erik Dörnenburg)**](https://www.meetup.com/func-prog-sweden/events/288323896/)
* 2022-11-09 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Introduction to Rust Atomics**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/289052285/)
* 2022-11-09 | Virtual (Darmstadt, DE) | [betterCode](https://www.bettercode.eu/)
    * [**betterCode Rust**](https://rust.bettercode.eu/)
* 2022-11-09 | Virtual (Malaysia, MY) | [Rust Malaysia](https://forms.gle/zWXcMDAnnibiL4ni9)
    * [**Rust Meetup November 2022 - a couple of lightning talks**](https://discord.gg/9Xj8H2EXTD)
* 2022-11-10 | Virtual (Budapest, HU) | [HWSW free!](https://www.meetup.com/hwswfree/)
    * [**RUST! RUST! RUST! meetup (online formában!)**](https://www.meetup.com/hwswfree/events/289044458/)
* 2022-11-10 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #19**](https://www.meetup.com/rust-noris/events/hlvbvsydcpbnb/)
* 2022-11-12 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://www.google.com/url?q=https%3A%2F%2Fdiscord.gg%2FyNtPTb2&sa=D&ust=1666661760000000&usg=AOvVaw13uHY9m-8bJJwgeP58VS8l)
* 2022-11-15 | Virtual (Nairobi, KE / New York, NY, US)| [Data Umbrella Africa](https://www.meetup.com/data-umbrella-africa2/)
    * [**Online: Introduction to Rust Programming**](https://www.meetup.com/data-umbrella-africa2/events/289308825/) | [**New York Mirror**](https://www.meetup.com/data-umbrella/events/289308172/)
* 2022-11-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc//)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/289015883/)
* 2022-11-16 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/tqvhxsydcpbvb/)
* 2022-11-17 | Virtual (Amsterdam, NL) | [ITGilde Tech-Talks](https://www.meetup.com/itgilde-cooperatie-amsterdam-unix-linux-meetups)
    * [**Introduction “Rust” an ITGilde Tech Talk delivered by Pascal van Dam**](https://www.meetup.com/itgilde-cooperatie-amsterdam-unix-linux-meetups/events/289167373/)
* 2022-11-17 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydcpbwb/)
* 2022-11-21 | Virtual (Paris, FR) | [Meetup Paris - École Supérieure de Génie Informatique (ESGI)](https://www.meetup.com/meetup-paris-ecole-superieur-du-genie-informatique)
    * [**Découverte de WebAssembly**](https://www.meetup.com/meetup-paris-ecole-superieur-du-genie-informatique/events/289112753/)
* 2022-11-24 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 27th Edition**](https://www.meetup.com/rust-linz/events/289251460/)
* 2022-11-29 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcpbmc/)
* 2022-11-30 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 3 - hybrid**](https://www.meetup.com/rust-munich/events/289065390/)

### Asia

* 2022-11-08 | Bangkok, TH | [Tech@Agoda](https://www.meetup.com/techatagoda/)
    * [**Rustacean Bangkok 5.0.0**](https://www.meetup.com/techatagoda/events/289329464/)

### Europe

* 2022-11-16 | Paris, FR | [Stockly](https://www.eventbrite.fr/o/stockly-42274765293)
    * [**Rust Meetup in Paris - hosted by Stockly**](https://www.eventbrite.fr/e/rust-meetup-in-paris-hosted-by-stockly-tickets-444152621447?aff=ebdssbdestsearch)
* 2022-11-23 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Initial Meet and Greet Rust meetup**](https://www.meetup.com/bratislava-rust-meetup-group/events/289028178/)
* 2022-11-24 | København, DK | [Copenhagen Rust Group](https://cph.rs/)
    * [**Hack Night #31**](https://www.meetup.com/copenhagen-rust-meetup-group/events/288179132/)
* 2022-11-30 | Amsterdam, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Rust in Critical Infrastructure**](https://www.meetup.com/rust-nederland/events/289204146/)
* 2022-11-30 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 3 - hybrid**](https://www.meetup.com/rust-munich/events/289065390/)

### North America

* 2022-11-10 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/events/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcpbnb/)
* 2022-11-15 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcpbtb/)

### Oceania

* 2022-11-09 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**RustAU Sydney - Last physical for 2022 !**](https://www.meetup.com/rust-sydney/events/289061840/)
* 2022-11-22 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**November Meetup**](https://www.meetup.com/rust-canberra/events/288615873/)

### South America

* 2022-11-05 | São Paulo, SP, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup/)
    * [**Rust-SP meetup Outubro 2022**](https://www.meetup.com/rust-sao-paulo-meetup/events/289176073/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org


<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/xldzbl/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> I'm getting more convinced that Rust code is generally going to end up faster than C++ code every day I work on optimizations.
>
> Strong immutability and no-alias guarantees are a game-changer and we've only really begun to scratch the surface of what can be done.

– [Patrick Walton on twitter](https://twitter.com/pcwalton/status/1585709621955526657?s=20&t=Ij5ODVKJERjZXOEh2Nzplg)

llogiq is exceedingly pleased with his suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/ykp3c9/this_week_in_rust_467/)</small>
