Title: This Week in Rust 619
Number: 619
Date: 2025-10-01
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
* [The next Rust All Hands](https://blog.rust-lang.org/inside-rust/2025/09/30/all-hands-2026/)
* [This Development-cycle in Cargo: 1.90](https://blog.rust-lang.org/inside-rust/2025/10/01/this-development-cycle-in-cargo-1.90/)

### Newsletters
* [The Embedded Rustacean Issue #55](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-55)

### Project/Tooling Updates
* [Bevy 0.17](https://bevy.org/news/bevy-0-17/)
* [Linting Rust code in the kernel](https://lwn.net/SubscriberLink/1038750/6aa9769e0b875235/)
* [Fast UDP I/O for Firefox in Rust](https://max-inden.de/post/fast-udp-io-in-firefox/)
* [genedex: A Small and Fast FM-Index for Rust](https://github.com/feldroop/genedex)
* [blogr v0.3.0 - Fast, lightweight static site generator with built in newsletter manager](https://github.com/bahdotsh/blogr/releases/tag/v0.3.0)
* [Feedr v0.2.0 - A feature-rich terminal-based RSS feed reader with a clean and intuitive TUI](https://github.com/bahdotsh/feedr/releases/tag/v0.2.0)
* [Glues v0.8.1 adds a browser TUI, proxy support, and redb storage](https://github.com/gluesql/glues/releases/tag/v0.8.1)
* [Role Based Access Control in SeaORM 2.0](https://www.sea-ql.org/blog/2025-09-30-sea-orm-rbac/)

### Observations/Thoughts
* [On Choosing Rust](https://endler.dev/2025/choosing-rust/)
* [The expression problem and Rust](https://purplesyringa.moe/blog/the-expression-problem-and-rust/)
* [Case Study: How Proton uses Rust to build cross-platform applications for millions of people](https://kerkour.com/proton-apps-rust)
* [The Game Engine that would not have been made without Rust](https://blog.vermeilsoft.com/2025-09-rust-game-engine)
* [Introducing Rust To The Automotive Stack: A Conversation With Julius Gustavsson Of Volvo Cars](https://filtra.io/rust/interviews/volvo-sep-25)
* [Rust in Paris 2025 – Full Talks Playlist](https://www.youtube.com/playlist?list=PLiOc9_WF8i8vVy5Qn6I9TxkBjsofzLcaH)
* [video] [How to Optimize Rust for Slowness: Inspired by New Turing Machine Results](https://www.youtube.com/watch?v=ec-ucXJ4x-0)

### Rust Walkthroughs
* [Under the hood: Vec\<T\>](https://marma.dev/articles/2025/under-the-hood-vec-t)
* [Axum Backend Series: Implement JWT Access Token](https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-jwt-access-token/)
* [Level Up your Rust pattern matching](https://blog.cuongle.dev/p/level-up-your-rust-pattern-matching)
* [video] [Sguaba: Type-safe spatial math in Rust](https://www.youtube.com/watch?v=kESBAiTYMoQ)

### Miscellaneous
* [Cloudflare just got faster and more secure, powered by Rust](https://blog.cloudflare.com/20-percent-internet-upgrade/)
* [Rust: Who, What and Why for ESA SW PA Workshop](https://ferrous-systems.com/blog/rust-who-what-why/)

## Crate of the Week

This week's crate is [blogr](https://github.com/bahdotsh/blogr), a fast, lightweight static site generator.

Thanks to [Gokul](https://users.rust-lang.org/t/crate-of-the-week/2704/1472) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

[Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

* [FR: Add a --fail-fast option to libtest](https://github.com/rust-lang/rust/issues/142859)
  * [Testing instructions](https://github.com/rust-lang/rust/issues/142859#issuecomment-3339090064)

*No calls for testing were issued this week by
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

* [Diesel - Feedback on an All About Select guide](https://github.com/diesel-rs/diesel/discussions/4786)
* [Diesel - Incomplete explanation for PgConnection loading modes](https://github.com/diesel-rs/diesel/issues/4764)
* [Diesel - FromSqlRow derive macro fails when in scope where Ok is not the one from the standard library](https://github.com/diesel-rs/diesel/issues/4787)
* [Diesel - fk_related_tables not working correctly with two foreign keys to same table](https://github.com/diesel-rs/diesel/issues/4780)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->

* [**TokioConf 2026**](https://tokio.rs/blog/2025-09-26-announcing-tokio-conf-cfp)| 2025-12-08 | Portland, Oregon, USA | 2026-04-20

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

473 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-09-23..2025-09-30

#### Compiler
* [add an attribute to check the number of lanes in a SIMD vector after monomorphization](https://github.com/rust-lang/rust/pull/146667)
* [add panic=immediate-abort](https://github.com/rust-lang/rust/pull/146317)
* [skip stack overflow handler for panic=immediate-abort](https://github.com/rust-lang/rust/pull/147090)
* [allow `&raw [mut | const]` for union field in safe code](https://github.com/rust-lang/rust/pull/141469)
* [debuginfo: add an unstable flag to write split DWARF to an explicit directory](https://github.com/rust-lang/rust/pull/146376)
* [detect tuple structs that are unconstructable due to re-export](https://github.com/rust-lang/rust/pull/133477)
* [do not compute optimized MIR if code does not type-check](https://github.com/rust-lang/rust/pull/147092)
* [do not materialise X in `[X; 0]` when X is unsizing a const](https://github.com/rust-lang/rust/pull/145277)
* [improve diagnostics for empty attributes](https://github.com/rust-lang/rust/pull/146653)
#### Library
* [BTreeMap: don't leak allocators when initializing nodes](https://github.com/rust-lang/rust/pull/146859)
* [constify {`Mutex`, `RwLock`, `ReentrantLock`}`::data_ptr`](https://github.com/rust-lang/rust/pull/146904)
* [constify Default on Nanoseconds](https://github.com/rust-lang/rust/pull/146979)
* [constify `{float}::total_cmp()`](https://github.com/rust-lang/rust/pull/146818)
* [unstably constify float `mul_add` methods](https://github.com/rust-lang/rust/pull/146735)
* [non-panicking `Vec::try_remove`](https://github.com/rust-lang/rust/pull/146293)
* [fix infinite recursion in `Path::eq` with String](https://github.com/rust-lang/rust/pull/146958)
* [implement `hostname`](https://github.com/rust-lang/rust/pull/146937)
* [remove most `#[track_caller]` from allocating Vec methods](https://github.com/rust-lang/rust/pull/147042)
#### Cargo
* [config: combine key error context into one](https://github.com/rust-lang/cargo/pull/16004)
* [shell: Use a distinct style for transient status](https://github.com/rust-lang/cargo/pull/16019)
* [add retry for `git fetch` failures in `CARGO_NET_GIT_FETCH_WITH_CLI` path](https://github.com/rust-lang/cargo/pull/16016)
* [better error message for rust version incompatibility](https://github.com/rust-lang/cargo/pull/16021)
* [sparse URLs in `TomlLockfileSourceId`](https://github.com/rust-lang/cargo/pull/15990)
* [use `host-tuple` for host target subsitution](https://github.com/rust-lang/cargo/pull/16003)
#### Rustdoc
* [Add support for associated items in "jump to def" feature](https://github.com/rust-lang/rust/pull/135771)
* [add rustdoc `doc_cfg` features](https://github.com/rust-lang/rust/pull/138907) (RFC [#3631](https://rust-lang.github.io/rfcs/3631-rustdoc-cfgs-handling.html))
* [search: use the same ID for entry and path to same item](https://github.com/rust-lang/rust/pull/147045)
* [hide `#[repr]` if it isn't part of the public ABI](https://github.com/rust-lang/rust/pull/116882)
* [put the toolbar on the all item index](https://github.com/rust-lang/rust/pull/147047)
#### Clippy
* [`double_parens`: add structured suggestions, fix bug](https://github.com/rust-lang/rust-clippy/pull/15420)
* [`filter_next`: check for `filter().next_back()`](https://github.com/rust-lang/rust-clippy/pull/15748)
* [`collapsible`(`_else`)`_if`: respect `#[expect]` on inner `if`](https://github.com/rust-lang/rust-clippy/pull/15647)
* [`let_unit_value`: create the suggestion "differentially"](https://github.com/rust-lang/rust-clippy/pull/15788)
* [`new_without_default`: if `new` has `#[cfg]`, copy that onto `impl Default`](https://github.com/rust-lang/rust-clippy/pull/15720)
* [`or_fun_call`: respect MSRV for `Result::unwrap_or_default` suggestion](https://github.com/rust-lang/rust-clippy/pull/15756)
* [`should_implement_trait`: only suggest traits that are in the prelude](https://github.com/rust-lang/rust-clippy/pull/15776)
* [`unnecessary_mut_passed`: retain parens around the arguments](https://github.com/rust-lang/rust-clippy/pull/15731)
* [check for proc macros from within `explicit_deref_methods` and do not lint on proc macro expansions](https://github.com/rust-lang/rust-clippy/pull/15628)
* [fix `new_without_default` false positive on private type with trait impl](https://github.com/rust-lang/rust-clippy/pull/15782)
* [overhaul `mut_mut`](https://github.com/rust-lang/rust-clippy/pull/15417)
* [refactor `module_style`](https://github.com/rust-lang/rust-clippy/pull/15469)
* [rename `unchecked_duration_subtraction` to `unchecked_time_subtraction` and check for `Duration - Duration`](https://github.com/rust-lang/rust-clippy/pull/13800)
#### Rust-Analyzer
* [add `all`, `any` and `not` completions in `#[cfg]`](https://github.com/rust-lang/rust-analyzer/pull/20760)
* [add `cfg_attr` predicate completion](https://github.com/rust-lang/rust-analyzer/pull/20604)
* [add applicable on bang `!` for `apply_demorgan`](https://github.com/rust-lang/rust-analyzer/pull/20599)
* [add const parameter keyword completion](https://github.com/rust-lang/rust-analyzer/pull/20729)
* [add let-chain support for `convert_to_guarded_return`](https://github.com/rust-lang/rust-analyzer/pull/20598)
* [allow `&raw` {`mut`, `const`} for union field](https://github.com/rust-lang/rust-analyzer/pull/19867)
* [fix "Replace match with if let" not to trigger when invalid transformations occur](https://github.com/rust-lang/rust-analyzer/pull/20543)
* [fix SCIP panicking due to salsa not attaching](https://github.com/rust-lang/rust-analyzer/pull/20735)
* [fix applicable on if-let-chain for `invert_if`](https://github.com/rust-lang/rust-analyzer/pull/20736)
* [fix expand rest pattern in tuple and slice pattern](https://github.com/rust-lang/rust-analyzer/pull/20731)
* [fix precedence parenthesis for `replace_arith_op`](https://github.com/rust-lang/rust-analyzer/pull/20611)
* [don't turn unused variables into raw identifier](https://github.com/rust-lang/rust-analyzer/pull/20742)
* [implement fallback properly](https://github.com/rust-lang/rust-analyzer/pull/20721)
* [support negative integer literals in const generics in declarative macros](https://github.com/rust-lang/rust-analyzer/pull/20745)

### Rust Compiler Performance Triage

A relatively quiet week. Most of the improvements are to doc builds, driven by
continued packing of the search index in rustdoc-search: stringdex update with
more packing [#147002](https://github.com/rust-lang/rust/pull/147002) and
simplifications to doc(cfg) in Implement RFC 3631: add rustdoc doc_cfg features
[#138907](https://github.com/rust-lang/rust/pull/138907).

Triage done by **@simulacrum**.
Revision range: [ce4beebe..8d72d3e1](https://perf.rust-lang.org/?start=ce4beebecb77821734079cff47d8af08f9f27f11&end=8d72d3e1e96f58ca10059a6bb6e8aecba4a0e9cd&absolute=false&stat=instructions%3Au)

1 Regressions, 6 Improvements, 4 Mixed; 2 of them in rollups
29 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025/2025-09-28.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [Tracking Issue for `const_slice_rotate`](https://github.com/rust-lang/rust/issues/143812)
* [implement Extend<{Group, Literal, Punct, Ident}> for TokenStream](https://github.com/rust-lang/rust/pull/145722)
* [Stabilize `char_max_len`](https://github.com/rust-lang/rust/pull/145610)
* [Add `From` impls for wrapper types](https://github.com/rust-lang/rust/pull/146013)
* [prefer alias candidates for sizedness + auto trait goals](https://github.com/rust-lang/rust/pull/144064)
* [Tracking Issue for `NonZero<u*>::div_ceil`](https://github.com/rust-lang/rust/issues/132968)
* [Tracking issue for release notes of #146410: Iterator repeat: no infinite loop for `last` and `count`](https://github.com/rust-lang/rust/issues/146660)
* [Stabilize `rwlock_downgrade` library feature](https://github.com/rust-lang/rust/pull/143191)
* [Prevent downstream `impl DerefMut for Pin<LocalType>`](https://github.com/rust-lang/rust/pull/145608)
* [Forbid freely casting lifetime bounds of dyn-types](https://github.com/rust-lang/rust/pull/136776)
* [Fix accidental type inference in array coercion](https://github.com/rust-lang/rust/pull/140283)
* [docs(style): Specify the frontmatter style](https://github.com/rust-lang/rust/pull/145617)
* [core: simplify `Extend` for tuples](https://github.com/rust-lang/rust/pull/138799)

*No Items entered Final Comment Period this week for
  [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) or
  [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2025-10-01 - 2025-10-29 🦀

### Virtual
* 2025-10-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyhcnbcb)
* 2025-10-02 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/ekgdex6j)
* 2025-10-04 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763858627)
* 2025-10-05 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311062530/)
* 2025-10-07 | Virtual (Beijing, CN) | [WebAssembly and Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**Monthly WasmEdge Community Meeting, the runtime for LLM/AGI**](https://www.meetup.com/wasm-rust-meetup/events/310831771/)
* 2025-10-09 - 2025-10-10 | Hybrid (Paris, FR) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2025**](https://eurorust.eu/schedule/)
* 2025-10-09 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046639/)
* 2025-10-09 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/jotnli2g)
* 2025-10-12 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109160/)
* 2025-10-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361534/)
* 2025-10-15 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731034/)
* 2025-10-16 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/o8fh3fh7)
* 2025-10-16 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/306046668/)
* 2025-10-18 | Virtual (Gdansk, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto/)
    * [**[BEZPŁATNIE] Programowanie w języku Rust**](https://www.meetup.com/stacja-it-trojmiasto/events/310935164/)
* 2025-10-19 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109167/)
* 2025-10-21 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/311068625/)
* 2025-10-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/310002307/)
* 2025-10-23 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046641/)
* 2025-10-23 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/zyc3touy)
* 2025-10-26 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109171/)
* 2025-10-28 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361444/)

### Asia
* 2025-10-02 | Seoul, KR | [Seoul Rust (Programming Language) Meetup](https://www.meetup.com/rust-seoul-meetup)
    * [**Seoul Rust Meetup**](https://www.meetup.com/rust-seoul-meetup/events/310824483/)
* 2025-10-04 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**October 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/october-2025-rustacean-meetup/)
* 2025-10-08 | Kuala Lumpur, MY | [Rust Malaysia](https://t.me/rustlangmalaysia)
    * [**Malaysia Rust Meetup**](https://docs.google.com/forms/d/e/1FAIpQLScESY4eHc5lzZznAHZmFxI85CYaOKCYTQASRwXxC2y0KpI6zw/viewform)
* 2025-10-09 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup)
    * [**Building Pocket-Sized Terminal UIs with Rust**](https://www.meetup.com/tokyo-rust-meetup/events/310899137/)
* 2025-10-20 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust October 2025 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/310628902/)

### Europe
* 2025-10-01 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn)
    * [**Rust in October: Undefined Rust**](https://www.meetup.com/rustcologne/events/311209846/)
* 2025-10-01 | Olomouc, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia)
    * [**4. Rust Moravia Meetup (In the capital!)**](https://www.meetup.com/rust-moravia/events/310743282/)
* 2025-10-01 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Building AI chatbots with Webassembly, Rust, and Leptos**](https://www.meetup.com/oxford-rust-meetup-group/events/311170808/)
* 2025-10-01 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1686673127729)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1686673127729)
* 2025-10-02 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin on location 🏳️‍🌈 - Edition 007**](https://www.meetup.com/rust-berlin/events/311202886/)
* 2025-10-02 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310062134/)
* 2025-10-08 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 10 2025**](https://luma.com/8u55jo0h)
* 2025-10-08 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #79**](https://www.meetup.com/rust-paris/events/310424476/)
* 2025-10-08 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944041/)
* 2025-10-09 - 2025-10-10 | Hybrid (Paris, FR) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2025**](https://eurorust.eu/schedule/)
* 2025-10-14 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #13 @ letsboot**](https://www.meetup.com/rust-basel/events/310827834/)
* 2025-10-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/311035141/)
* 2025-10-21 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592252/)
* 2025-10-21 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group)
    * [**Rust in Surgery: Powering the Data Pipelines**](https://www.meetup.com/london-rust-project-group/events/310813952/)
* 2025-10-21 | Bergen, No | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Meetup #01 @ Zrch**](https://www.meetup.com/bergen-rust-new-technology/events/311153821/)
* 2025-10-28 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester October Code Night**](https://www.meetup.com/rust-manchester/events/307919171/)

### North America
* 2025-10-01 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Web3 Developer Meetup**](https://www.meetup.com/rust-los-angeles/events/311243690/)
* 2025-10-02 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
    * [**October Monthly Social**](https://www.meetup.com/rust-montreal/events/311242811/)
* 2025-10-02 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/311004898)
* 2025-10-02 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**🚁 Rust in Flight: Lessons from Designing a 3D‑Printed Quadcopter with Embedded**](https://www.meetup.com/stl-rust/events/310279407/)
* 2025-10-04 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**North End Rust Lunch, Oct 4**](https://www.meetup.com/bostonrust/events/310983705/)
* 2025-10-09 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Aya the Beholder: Writing an eBPF Firewall with the Aya Crate**](https://www.meetup.com/utah-rust/events/311145663/)
* 2025-10-15 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731034/)
* 2025-10-16 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/311012947/)
* 2025-10-21 | San Francisco, CA, US | [Vara & Gear](https://luma.com/events-by-vara-gear)
    * [**Rust Workshop by Vara Network**](https://luma.com/kbs2os1c)
* 2025-10-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/308284343/)
* 2025-10-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310457307/)
* 2025-10-23 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Year In Review**](https://www.meetup.com/music-city-rust-developers/events/304333267/)
* 2025-10-25 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Porter Square Rust Lunch, Oct 25**](https://www.meetup.com/bostonrust/events/310983712/)

### Oceania
* 2025-10-22 | Perth, AU | [Rust Perth Meetup Group](https://www.meetup.com/perth-rust-meetup-group)
    * [**October Meetup**](https://www.meetup.com/perth-rust-meetup-group/events/310847099/)
* 2025-10-28 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**October Meetup**](https://www.meetup.com/rust-canberra/events/311234237/)

### South America
* 2025-10-08 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Octubre Async - Escribimos un Runtime desde Cero!**](https://www.meetup.com/rust-argentina/events/311276950/)
* 2025-10-25 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP na Amazon Web Services**](https://www.meetup.com/rust-sao-paulo-meetup/events/311084440/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1nknaii/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> I must personally extend my condolences to those who forgot they chose in the past to annoy their future self.

– [@workingjubilee on github](https://github.com/rust-lang/rust/issues/145936#issuecomment-3322104583)

Thanks to [Riking](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1719) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1nvpd2x/this_week_in_rust_619/)</small>
