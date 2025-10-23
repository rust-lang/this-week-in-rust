Title: This Week in Rust 622
Number: 622
Date: 2025-10-22
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

Want TWIR in your inbox? [Subscribe here](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the linked Page](https://example.com/my_article)

If you add a link to a non-text content please prefix it with `[video]` or `[audio]`:

* [video] [Title of the linked video](https://example.com/my_video_article)
* [audio] [Title of the linked audio file](https://example.com/my_podcast)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official

### Foundation

### Newsletters
* [Scientific Computing in Rust #11 (October 2025)](https://scientificcomputing.rs/monthly/2025-10)

### Project/Tooling Updates
* [Slint 1.14 Released](https://slint.dev/blog/slint-1.14-released)
* [Danube Messaging - new topic persistence architecture (Wal + Cloud)](https://danube-docs.dev-state.com/architecture/persistence/)
* [SierraDB: A Distributed Event Store Built in Rust](https://tqwewe.com/blog/building-sierradb/)
* [Announcing C2Rust v0.21](https://immunant.com/blog/2025/10/c2rust_release/)

### Observations/Thoughts
* [Python and Rust interoperability](https://medium.com/google-cloud/python-and-rust-interoperability-a-walkthrough-for-building-a-high-performance-mcp-server-56c04e4b651b)
* [Controlled Destruction in Rust: Towards Async Drop and Safer Resource Management](https://smallcultfollowing.com/babysteps/blog/2025/10/21/move-destruct-leak/)
* [Everybody's so Creative!](https://daymare.net/blogs/everbody-so-creative/)
* [How we organized the Rust Clippy feature freeze](https://blog.goose.love/posts/organizing-a-feature-freeze/)
* [audio] [Netstack.FM Episode 10 – zerocopy with Joshua Liebow-Feeser](https://netstack.fm/#episode-10)

### Rust Walkthroughs
* [Real-Time Results in a Federated Query Engine](https://blog.vega.io/posts/partial_stream/)
* [Axum: Multi-tenancy (with Hexarch) and Abstracting the Repository](https://crustyengineer.com/blog/axum-multi-tenancy-abstract-repository-layer/)
* [Fixing rust-lang stdarch issues in LLVM - Blog - Tweede golf](https://tweedegolf.nl/en/blog/196/fixing-rust-lang-stdarch-issues-in-llvm)
* [Rust unit testing: spies and dummy test doubles](https://jorgeortiz.dev/posts/rust_unit_testing_test_doubles_spy/)
* [Axum Backend Series: Refresh Token Rotation and Reuse Detection](https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-refresh-token-rotation/)
* [Vibe Validation with Lean, ChatGPT-5, & Claude 4.5: Nine Rules for Proving (Rust) Algorithms Correct Without Knowing Formal Methods (Part 1)](https://medium.com/@carlmkadie/vibe-validation-with-lean-chatgpt-5-claude-4-5-part-1-c57b430b3d7a)
* [A Typed Evaluator in Rust](https://rvarago.github.io/typed-evaluator-in-rust/)
* [Zero Cost Composition and the Power of GATs](https://orxfun.github.io/orxfun-notes/#/zero-cost-composition-2025-10-15)
* [Integration Testing Rust Binary Crates](https://www.unwoundstack.com/blog/integration-testing-rust-binaries.html)
* [video] [Build with Naz : How to speed up Rust compiler for different workflows](https://www.youtube.com/watch?v=hpGDCbO31Rg)

### Research

### Miscellaneous
* [September 2025 Rust Jobs Report](https://filtra.io/rust/jobs-report/sep-25)

## Crate of the Week

This week's crate is [extend\_mut](https://docs.rs/extend_mut), a library to safely extend the lifetime of an exclusive reference under some constraints.P

Thanks to [Oleksandr Babak](https://users.rust-lang.org/t/crate-of-the-week/2704/1482) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

[Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing)
* [Tracking Issue for cargo-script RFC 3424](https://github.com/rust-lang/cargo/issues/12207)
  * [Testing Steps](https://github.com/rust-lang/cargo/issues/12207#issuecomment-3412997290)


* *No calls for testing were issued this week by
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

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

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

369 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-10-14..2025-10-21

#### Compiler
* [add a `!=` check to `ChunkedBitSet::union`](https://github.com/rust-lang/rust/pull/147619)
* [bitset cleanups](https://github.com/rust-lang/rust/pull/147630)
* [`deduced_param_attrs`: check Freeze on monomorphic types](https://github.com/rust-lang/rust/pull/147695)
* [deny-by-default never type lints](https://github.com/rust-lang/rust/pull/146167)
* [improve error message for ambiguous numeric types in closure parameters](https://github.com/rust-lang/rust/pull/147577)
* [remove boxes from AST list elements](https://github.com/rust-lang/rust/pull/146221)
* [`TaskDeps` improvements](https://github.com/rust-lang/rust/pull/147508)
* [`unused_must_use`: Don't warn on `Result<(), Uninhabited>` or `ControlFlow<Uninhabited, ()>`](https://github.com/rust-lang/rust/pull/147382)
* [use regular Vec in BitSet](https://github.com/rust-lang/rust/pull/147644)

#### Library
* [const `mem::drop`](https://github.com/rust-lang/rust/pull/147708)
* [constify basic Clone impls](https://github.com/rust-lang/rust/pull/146976)
* [iter repeat: panic on last](https://github.com/rust-lang/rust/pull/147258)
* [stabilise `rotate_left` and `rotate_right` in `[_]` as `const fn` items](https://github.com/rust-lang/rust/pull/146841)
* [stabilize `rwlock_downgrade` library feature](https://github.com/rust-lang/rust/pull/143191)

#### Cargo
* [`check`: Fix suggested command for bin package](https://github.com/rust-lang/cargo/pull/16127)
* [`script`: Remove name sanitiztion outside what is strictly required](https://github.com/rust-lang/cargo/pull/16120)
* [`script`: Tweak cargo script build-dir / target-dir](https://github.com/rust-lang/cargo/pull/16086)

#### Rustdoc
* [search: stringdex 0.0.2](https://github.com/rust-lang/rust/pull/147660)
* [fix passes order so intra-doc links are collected after stripping passes](https://github.com/rust-lang/rust/pull/147809)

#### Clippy
* [`empty_enum`: don't lint if all variants happen to be `cfg`-d out](https://github.com/rust-lang/rust-clippy/pull/15911)
* [`option_option`: split part of diagnostic message into help message](https://github.com/rust-lang/rust-clippy/pull/15870)
* [`unnecessary_safety_comment` Some fixes regarding comments above attributes](https://github.com/rust-lang/rust-clippy/pull/15678)
* [allow `explicit_write` in tests](https://github.com/rust-lang/rust-clippy/pull/15862)
* [dereference argument of `manual_div_ceil()` if needed](https://github.com/rust-lang/rust-clippy/pull/15706)
* [`manual_rotate`: also recognize non-consts](https://github.com/rust-lang/rust-clippy/pull/15402)
* [overhaul `mutex_{atomic,integer}`](https://github.com/rust-lang/rust-clippy/pull/15632)

#### Rust-Analyzer
* [parser: Don't error on frontmatter](https://github.com/rust-lang/rust-analyzer/pull/20854)
* [improve fixture support](https://github.com/rust-lang/rust-analyzer/pull/20855)
* [fix invalid RestPat for `convert_tuple_struct_to_named_struct`](https://github.com/rust-lang/rust-analyzer/pull/20880)
* [fix missing RestPat for `convert_named_struct_to_tuple_struct`](https://github.com/rust-lang/rust-analyzer/pull/20872)
* [don't make `convert_to_guarded_return` applicable on `let-else`](https://github.com/rust-lang/rust-analyzer/pull/20838)
* [fix `signature_help` to proto conversion creating invalid utf16 offsets](https://github.com/rust-lang/rust-analyzer/pull/20876)
* [support `break` with value in completions](https://github.com/rust-lang/rust-analyzer/pull/20673)
* [support `else` blocks with `!` return type in `convert_to_guarded_return`](https://github.com/rust-lang/rust-analyzer/pull/20758)
* [support `match` inside `if` in `pull_assignment_up`](https://github.com/rust-lang/rust-analyzer/pull/20772)
* [migrate more stuff to the next solver](https://github.com/rust-lang/rust-analyzer/pull/20841)
* [migrate variance to the next solver and remove lint allows from its stuff](https://github.com/rust-lang/rust-analyzer/pull/20867)
* [rip Chalk out of the codebase 🎉](https://github.com/rust-lang/rust-analyzer/pull/20873)
* [support underscore suffix parameter hide inlayHints](https://github.com/rust-lang/rust-analyzer/pull/20858)
* [use `FileId::MAX` for id assertion in `PathInterner::intern`](https://github.com/rust-lang/rust-analyzer/pull/20757)

### Rust Compiler Performance Triage

Fairly busy week, with lots of mixed results. However, overall we ended with a
slight improvement on average.

Triage done by **@simulacrum**.
Revision range: [956f47c3..4068bafe](https://perf.rust-lang.org/?start=956f47c32f1bd97b22cd702d7ccf78f0f0d42c34&end=4068bafedd8ba724e332a5221c06a6fa531a30d2&absolute=false&stat=instructions%3Au)

2 Regressions, 5 Improvements, 10 Mixed; 5 of them in rollups

39 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025/2025-10-20.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Tracking Issue for NEON fp16 intrinsics](https://github.com/rust-lang/rust/issues/136306)
* [Change `Location<'_>` lifetime to `'static` in `Panic[Hook]Info`](https://github.com/rust-lang/rust/pull/146561)
* [Tracking Issue for `substr_range` and related methods](https://github.com/rust-lang/rust/issues/126769)
* [repr(transparent): do not consider repr(C) types to be 1-ZST](https://github.com/rust-lang/rust/pull/147185)
* [Don't require `T: RefUnwindSafe` for `vec::IntoIter<T>: UnwindSafe`](https://github.com/rust-lang/rust/pull/145665)
* [Stabilize -Zno-jump-tables into -Cjump-tables=bool](https://github.com/rust-lang/rust/pull/145974)
* [Tracking issue for alloc_layout_extra](https://github.com/rust-lang/rust/issues/55724)
* [Add warn-by-default lint for visibility on `const _` declarations](https://github.com/rust-lang/rust/pull/147136)
* [Tracking Issue for `debug_closure_helpers`](https://github.com/rust-lang/rust/issues/117729)
* [fully deprecate the legacy integral modules](https://github.com/rust-lang/rust/pull/146882)
* [Tracking Issue for `fmt_from_fn`](https://github.com/rust-lang/rust/issues/146705)
* [Make `IoSlice` and `IoSliceMut` methods unstably const](https://github.com/rust-lang/rust/pull/144090)
* [Tracking Issue for `VecDeque::pop_front_if` & `VecDeque::pop_back_if`](https://github.com/rust-lang/rust/issues/135889)
* [disposition: unspecified] [[std][BTree] Fix behavior of `::append` to match documentation, `::insert`, and `::extend`](https://github.com/rust-lang/rust/pull/145628)
* [Impls and impl items inherit `dead_code` lint level of the corresponding traits and trait items](https://github.com/rust-lang/rust/pull/144113)
* [Document MaybeUninit bit validity](https://github.com/rust-lang/rust/pull/140463)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Move unreachable code lint from HIR type check to a proper lint](https://github.com/rust-lang/compiler-team/issues/931)
* [Rename `//@ add-core-stubs` to `//@ add-minicore`](https://github.com/rust-lang/compiler-team/issues/930)
* [Move annotation for profiling compiler-generated moves and copies.](https://github.com/rust-lang/compiler-team/issues/928)
* [Use `llvm-bitcode-linker` as the default linker for nvptx64-nvidia-cuda](https://github.com/rust-lang/compiler-team/issues/927)

[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period)
* [Delegate GSoC money spending to the t-mentorship team](https://github.com/rust-lang/leadership-council/issues/232)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2025-10-22 - 2025-11-19 🦀

### Virtual
* 2025-10-22 | Virtual (Boulder, CO, US) | [Boulder Elixir](https://www.meetup.com/boulder-elixir/events/)
    * [**Integrating Elixir and Apache DataFusion with Rustler**](https://www.meetup.com/boulder-elixir/events/310996627/)
* 2025-10-22 | Virtual (Buenos Aires, AR) | [[Net-Baires] Comunidad de .NET en Buenos Aires](https://www.meetup.com/es-ES/net-baires/)
    * [**Rust para devs .NET | Community Standup #10**](https://www.meetup.com/es-ES/net-baires/events/311365783/)
* 2025-10-23 | Hybrid (Seattle/Bellevue, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**October, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351020/)
* 2025-10-23 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046641/)
* 2025-10-23 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/zyc3touy)
* 2025-10-26 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109171/)
* 2025-10-28 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361444/)
* 2025-10-30 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/t8yovmmm)
* 2025-11-01 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763868657)
* 2025-11-02 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109173/)
* 2025-11-05 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 2025-11-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/311574520/)
* 2025-11-05 | Virtual | [Ardan Labs](https://www.eventbrite.com/o/ardan-labs-7092394651)
    * [**Mastering Error Handling in Rust: From Panics to thiserror & anyhow**](https://www.eventbrite.com/e/mastering-error-handling-in-rust-from-panics-to-thiserror-anyhow-tickets-1849030121869)
* 2025-11-06 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305646021/)
* 2025-11-06 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/xkd84gfz)
* 2025-11-09 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109175/)
* 2025-11-10 || [BetterCode](https://www.bettercode.eu/)
    * $[**betterCode() Industrielle Anwendungen mit Rust**](https://dev.events/conferences/better-code-rust-i6inve6t)
* 2025-11-11 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361536/)
* 2025-11-11 | Virtual (London, GB) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/311068632/)
* 2025-11-13 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/yhe1xrhe)
* 2025-11-13 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/310849154/)
* 2025-11-16 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109181/)
* 2025-11-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/310002262/)
* 2025-11-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/309926564/)

### Asia
* 2025-11-15 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**November 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/november-2025-rustacean-meetup//)

### Europe
* 2025-10-23 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (evening pub)**](https://www.meetup.com/rust-and-friends/events/311501254/)
* 2025-10-24 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/311501249/)
* 2025-10-28 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester October Code Night**](https://www.meetup.com/rust-manchester/events/307919171/)
* 2025-10-29 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund/events/)
    * [**Rust Dortmund Meetup October 2025**](https://www.meetup.com/rust-dortmund/events/311251545/)
* 2025-10-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #62 sponsored by Google!**](https://www.meetup.com/copenhagen-rust-community/events/311405044/)
* 2025-10-30 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Prague (October 2025)**](https://www.meetup.com/rust-prague/events/310967094/)
* 2025-11-01 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #19**](https://www.meetup.com/stockholm-rust/events/311582259/)
* 2025-11-02 - 2025-11-04 | Florence, IT | [Rustlab 2025](https://rustlab.it/)
    * $[**Rustlab 2025**](https://rustlab.it/)
* 2025-11-03 | Bern, CH | [Guild42](https://www.meetup.com/it-IT/guild42ch/)
    * [**Moving out of systems programming into Kubernetes: is it time to adopt Rust ?**](https://www.meetup.com/it-IT/guild42ch/events/307260207/)
* 2025-11-04 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester November Talk**](https://www.meetup.com/rust-manchester/events/310921632/)
* 2025-11-04 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/events/)
    * [**Optimizing matrix multiplication and building Python packages with Rust**](https://www.meetup.com/rust-trondheim/events/311595023/)
* 2025-11-05 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Rust Girona Hack & Learn 11 2025**](https://luma.com/xl8ob0tn)
* 2025-11-05 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310601872/)
* 2025-11-05 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Rust/ACCU meetup.**](https://www.meetup.com/oxford-rust-meetup-group/events/nnrkttyhcpbhb/)
* 2025-11-06 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #11**](https://www.meetup.com/rust-gdansk/events/310924266/)
* 2025-11-12 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944050/)
* 2025-11-13 | Geneva, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2025-11-13 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #80**](https://www.meetup.com/rust-paris/events/311461594/)
* 2025-11-18 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592257/)

### North America
* 2025-10-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310457307/)
* 2025-10-23 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Year In Review**](https://www.meetup.com/music-city-rust-developers/events/304333267/)
* 2025-10-23 | Hybrid (Seattle/Bellevue, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**October, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351020/)
* 2025-10-23 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust)
    * [**October Rust Meetup: A Special Presentation and Monthly Meetups are Back!**](https://www.meetup.com/spokane-rust/events/311346444/)
* 2025-10-25 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Porter Square Rust Lunch, Oct 25**](https://www.meetup.com/bostonrust/events/310983712/)
* 2025-10-25 | Dallas, TX, US | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**In-person Meetup with Collin College Software Engineering Club!**](https://www.meetup.com/dallasrust/events/311562607/)
* 2025-10-28 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/311603282/)
* 2025-10-29 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Scalable static analysis: confronting the halting problem**](https://www.meetup.com/rust-nyc/events/311541108/)
* 2025-10-30 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/308675988/)
* 2025-10-30 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/311273832/)
* 2025-11-01 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Chinatown Rust Lunch, Nov 1**](https://www.meetup.com/bostonrust/events/311039492/)
* 2025-11-06 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**SIUE students on wasm 3D animations**](https://www.meetup.com/stl-rust/events/307251982/)
* 2025-11-08 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Winter Hill Rust Lunch, Nov 8**](https://www.meetup.com/bostonrust/events/311039501/)
* 2025-11-13 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Ipmap: Building Desktop Apps with Tauri**](https://www.meetup.com/utah-rust/events/311613658/)
* 2025-11-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/308865806/)

### Oceania
* 2025-10-22 | Perth, AU | [Rust Perth Meetup Group](https://www.meetup.com/perth-rust-meetup-group)
    * [**October Meetup**](https://www.meetup.com/perth-rust-meetup-group/events/310847099/)
* 2025-10-28 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**October Meetup**](https://www.meetup.com/rust-canberra/events/311234237/)

### South America
* 2025-10-22 | Montevideo, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay/events/)
    * [**Rust Uruguay meetup de Octubre**](https://www.meetup.com/rust-uruguay/events/311475675/)
* 2025-10-25 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP na Amazon Web Services**](https://www.meetup.com/rust-sao-paulo-meetup/events/311084440/)
* 2025-10-30 | Florianopolis, BR | [Rust Brasil](https://luma.com/calendar/cal-iOloL5ZqswCO5Mm)
    * [**Rust Floripa**](https://luma.com/lky7an18)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> There used to be recurring questions about mod vs use in the user forum, until I've added a note to the error message [...] and I think it largely solved the problem

– [Kornel on rust-internals](https://internals.rust-lang.org/t/curly-brace-support-for-mod/23437/51)

Thanks to [Noratrieb](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1722) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
