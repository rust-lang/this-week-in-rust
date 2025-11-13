Title: This Week in Rust 625
Number: 625
Date: 2025-11-12
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
* [Announcing Rust 1.91.1](https://blog.rust-lang.org/2025/11/10/Rust-1.91.1/)

### Newsletters
* [The Embedded Rustacean Issue #58](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-58)
* [This Month in Rust OSDev: October 2025](https://rust-osdev.com/this-month/2025-10/)

### Project/Tooling Updates
* [channels-console - real-time monitoring, metrics and logs for Rust channels](https://github.com/pawurb/channels-console)
* [qstr: Cache-efficient, stack-allocated string types](https://github.com/tindzk/qstr)
* [Announcing Magika 1.0: now faster, smarter, and rebuilt in Rust](https://opensource.googleblog.com/2025/11/announcing-magika-10-now-faster-smarter.html?m=1)
* [Tokuin 0.1.2: Load Testing LLMs from the Terminal](https://noos.blog/posts/tokuin-token-tooling-for-llm-builders/)
* [semver-query: semantic versioning data query tool](https://github.com/zak905/semver-query)
* [SeaORM 2.0: Strongly-Typed Column](https://www.sea-ql.org/blog/2025-11-11-sea-orm-2.0/)
* [LLMs: nanoGPT model in Rust - arrowspace v0.22.0 released](https://www.tuned.org.uk/posts/009_llms_nanogpt_model_in_rust)
* [InterpN: Fast Interpolation](https://jlogan.dev/blog/2025/11/10/2025-11-10-interpn-fast-interpolation/)
* [Tako 0.5.0 road to v1.0.0](https://rust-dd.com/post/tako-v-0-5-0-road-to-v-1-0-0)

### Observations/Thoughts
* [Just call clone (or alias)](https://smallcultfollowing.com/babysteps/blog/2025/11/10/just-call-clone/)
* [Engineering a Rust optimization quiz](https://fasterthanli.me/articles/engineering-a-rust-optimization-quiz)
* [Rust vs. Python: Finding the right balance between speed and simplicity](https://blog.jetbrains.com/rust/2025/11/10/rust-vs-python-finding-the-right-balance-between-speed-and-simplicity/)
* [video] [A Quick Start to Rust Lang](https://www.youtube.com/watch?v=fTXtdbt1PFA)
* [video] [Rust & JavaScript - Jakob Meier - Rust Zürisee November 2024](https://www.youtube.com/watch?v=MEHi7FZjSYw)
* [audio] [Netstack.FM Episode 13 – Inside Ping Proxies with Joseph Dye](https://netstack.fm/#episode-13)

### Rust Walkthroughs
* [Reproachfully Presenting Resilient Recursive Descent Parsing](https://thunderseethe.dev/posts/parser-base/)
* [Rust Hashing Cheat Sheet](https://bd103.github.io/blog/2025-11-10-rust-hashing-cheat-sheet/)

### Miscellaneous
* [Memory Safety for Skeptics](https://queue.acm.org/detail.cfm?id=3773095)

## Crate of the Week

This week's crate is [automesh](https://docs.rs/automesh), a crate for high-performance automatic mesh generation in Rust.

Thanks to [Michael R. Buche](https://users.rust-lang.org/t/crate-of-the-week/2704/1485) for the self-suggestion!

[Please submit your suggestions and votes for neMichael R. Buchext week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

* *No calls for testing were issued this week by
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing),
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
*No Calls for participation were submitted this week.*

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [**TokioConf 2026**](https://tokio.rs/blog/2025-09-26-announcing-tokio-conf-cfp)| CFP closes 2025-12-08 | Portland, Oregon, USA | 2026-04-20

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

409 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-11-04..2025-11-11

#### Compiler
* [add LLVM realtime sanitizer](https://github.com/rust-lang/rust/pull/147935)
* [don't completely reset `HeadUsages`](https://github.com/rust-lang/rust/pull/148649)
* [use annotate-snippets by default on nightly](https://github.com/rust-lang/rust/pull/148188)
* [implement SIMD funnel shifts in const-eval/Miri](https://github.com/rust-lang/rust/pull/147534)
* [recover `[T: N]` as `[T; N]`](https://github.com/rust-lang/rust/pull/148680)

#### Library
* [add Allocator proxy impls for Box, Rc, and Arc](https://github.com/rust-lang/rust/pull/148539)
* [add `extend_front` to VecDeque with specialization like extend](https://github.com/rust-lang/rust/pull/146861)
* [add alignment parameter to `simd_masked_{load,store}`](https://github.com/rust-lang/rust/pull/147355)
* [constify `ControlFlow` methods with unstable bounds](https://github.com/rust-lang/rust/pull/148285)
* [constify `ControlFlow` methods without unstable bounds](https://github.com/rust-lang/rust/pull/148248)
* [constify result unwrap unchecked](https://github.com/rust-lang/rust/pull/148333)
* [optimize path components iteration on platforms that don't have prefixes](https://github.com/rust-lang/rust/pull/148084)
* [stabilize `as_array` in `[_]` and `*const [_]`; stabilise `as_mut_array` in `[_]` and `*mut [_]`](https://github.com/rust-lang/rust/pull/147540)
* [stabilize `vec_deque_pop_if`](https://github.com/rust-lang/rust/pull/145992)
* [stabilize s390x `vector` target feature and `is_s390x_feature_detected!` macro](https://github.com/rust-lang/rust/pull/145656)
* [stop specializing on `Copy`](https://github.com/rust-lang/rust/pull/135634)

#### Cargo
* [`cli`: Refer to commands, not subcommands](https://github.com/rust-lang/cargo/pull/16226)
* [`completions`: don't wrap completion item help in parenthesis](https://github.com/rust-lang/cargo/pull/16215)
* [add native completions for `--package` on various commands](https://github.com/rust-lang/cargo/pull/16210)

#### Rustdoc
* [search: remove broken index special case](https://github.com/rust-lang/rust/pull/148563)
* [properly highlight shebang, frontmatter & weak keywords in source code pages and code blocks](https://github.com/rust-lang/rust/pull/148230)

#### Clippy
* [perf: `manual_is_power_of_two`: perform the `is_integer_literal` check first](https://github.com/rust-lang/rust-clippy/pull/16050)
* [consider type conversion that won't overflow](https://github.com/rust-lang/rust-clippy/pull/15950)
* [don't flag `cfg(test)` as multiple inherent impl](https://github.com/rust-lang/rust-clippy/pull/16041)
* [fix `match_single_binding` suggesting wrongly inside tuple](https://github.com/rust-lang/rust-clippy/pull/15539)
* [fix `missing_asserts_for_indexing` changing `assert_eq` to `assert`](https://github.com/rust-lang/rust-clippy/pull/16040)
* [fix `missing_inline_in_public_items` failing to fulfill `expect` in `--test` build](https://github.com/rust-lang/rust-clippy/pull/15320)
* [fix `mod_module_files` false positive for tests in workspaces](https://github.com/rust-lang/rust-clippy/pull/16048)
* [fix `nonminimal_bool` wrongly unmangled terms](https://github.com/rust-lang/rust-clippy/pull/16017)
* [fix `useless_let_if_seq` false negative when `if` is in the last expr of block](https://github.com/rust-lang/rust-clippy/pull/16063)

#### Rust-Analyzer
* [support rename after adding loop label](https://github.com/rust-lang/rust-analyzer/pull/20985)
* [add block on postfix `.const` completion](https://github.com/rust-lang/rust-analyzer/pull/21003)
* [fix panicking while resolving callable sigs for `AsyncFnMut`](https://github.com/rust-lang/rust-analyzer/pull/20971)
* [handle guards in `replace_if_let_with_match`](https://github.com/rust-lang/rust-analyzer/pull/20542)
* [handle method calls in `apply_demorgan`](https://github.com/rust-lang/rust-analyzer/pull/20973)
* [parse `impl ! {}`](https://github.com/rust-lang/rust-analyzer/pull/20972)
* [move safe computation out of unsafe block](https://github.com/rust-lang/rust-analyzer/pull/20977)
* [perf: only populate public items in dependency symbol index](https://github.com/rust-lang/rust-analyzer/pull/20997)
* [perf: reduce memory usage of symbol index](https://github.com/rust-lang/rust-analyzer/pull/20994)

### Rust Compiler Performance Triage

Mostly quiet week, with the majority of changes coming from the standard
library work towards removal of Copy specialization
([#135634](https://github.com/rust-lang/rust/pull/135634)).

Triage done by **@simulacrum**.
Revision range: [35ebdf9b..055d0d6a](https://perf.rust-lang.org/?start=35ebdf9ba1414456dfe1cb6a6b13ebae80e99734&end=055d0d6aaf937cc11b3d2a5b5725972723b7f3c6&absolute=false&stat=instructions%3Au)

3 Regressions, 1 Improvement, 7 Mixed; 3 of them in rollups
37 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025/2025-11-10.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Pass pointers to `const` in assembly](https://github.com/rust-lang/rfcs/pull/3848)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Warn against calls which mutates an interior mutable const -item](https://github.com/rust-lang/rust/pull/148407)
* [parser/lexer: bump to Unicode 17, use faster unicode-ident](https://github.com/rust-lang/rust/pull/148321)
* [const-eval: fix and re-enable pointer fragment support](https://github.com/rust-lang/rust/pull/148259)
* [Replace OffsetOf by an actual sum of calls to intrinsic.](https://github.com/rust-lang/rust/pull/148151)
* [Stabilize `asm_cfg`](https://github.com/rust-lang/rust/pull/147736)
* [Stabilize `-Zremap-path-scope`](https://github.com/rust-lang/rust/pull/147611)
* [error out when `repr(align)` exceeds COFF limit](https://github.com/rust-lang/rust/pull/142638)

[Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [target tier 3 support for hexagon-unknown-qurt](https://github.com/rust-lang/compiler-team/issues/919)
* [Proposal for a dedicated test suite for the parallel frontend](https://github.com/rust-lang/compiler-team/issues/906)
* [Proposal for Adapt Stack Protector for Rust](https://github.com/rust-lang/compiler-team/issues/841)
* [Give integer literals a sign instead of relying on negation expressions](https://github.com/rust-lang/compiler-team/issues/835)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Introduce `DerefInto` and `DerefMutInto` for RAII access](https://github.com/rust-lang/rfcs/pull/3880)

## Upcoming Events

Rusty Events between 2025-11-12 - 2025-12-10 🦀

### Virtual
* 2025-11-12 | Virtual (Boulder, CO, US) | [Boulder Elixir](https://www.meetup.com/boulder-elixir/events/)
    * [**Integrating Elixir and Apache DataFusion with Rustler**](https://www.meetup.com/boulder-elixir/events/310996627/)
* 2025-11-12 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.github.io)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/yhe1xrhe)
* 2025-11-13 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/310849154/)
* 2025-11-16 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109181/)
* 2025-11-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/310002262/)
* 2025-11-19 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.github.io)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/l2xukapz)
* 2025-11-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/309926564/)
* 2025-11-20 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046642/)
* 2025-11-20 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Tock, a Rust based Operating System Part #1**](https://www.meetup.com/charlottesville-rust-meetup/events/311705915/)
* 2025-11-23 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109183/)
* 2025-11-25 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361446/)
* 2025-11-25 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Data-Intensive Systems in Rust: Safety, Speed, Concurrency**](https://www.meetup.com/women-in-rust/events/311534469/)
* 2025-11-26 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.github.io)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/q5tjirkt)
* 2025-11-30 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109188/)
* 2025-12-02 | Virtual (London, GB) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Advent of Code - Kick Off!**](https://www.meetup.com/women-in-rust/events/311068648/)
* 2025-12-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 2025-12-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/311886445/)
* 2025-12-04 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046643/)
* 2025-12-09 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361537/)

### Africa
* 2025-11-18 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**More on Rust types**](https://www.meetup.com/johannesburg-rust-meetup/events/311726345/)

### Asia
* 2025-11-15 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**November 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/november-2025-rustacean-meetup//)

### Europe
* 2025-11-12 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/events/)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/311583721/)
* 2025-11-12 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944050/)
* 2025-11-13 | Geneva, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2025-11-13 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group/events/)
    * [**Adopting Rust at Lloyds bank (limited capacity)**](https://www.meetup.com/london-rust-project-group/events/311884244/)
* 2025-11-13 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/events/)
    * [**Rust London at Lloyds Banking Group**](https://www.meetup.com/rust-london-user-group/events/311868858/)
* 2025-11-13 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #80**](https://www.meetup.com/rust-paris/events/311461594/)
* 2025-11-14 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Rust Meetup @Magello**](https://www.meetup.com/stockholm-rust/events/311844163/)
* 2025-11-18 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Bevy, Compilers & Algebraic Data Types @ Zrch**](https://www.meetup.com/de-DE/bergen-rust-new-technology/events/311630543)
* 2025-11-18 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592257/)
* 2025-11-19 | Ostrava, CZ | [TechMeetup Ostrava](https://www.meetup.com/techmeetupostrava/events/)
    * [**QA Circle**](https://www.meetup.com/techmeetupostrava/events/311581090/)
* 2025-11-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Social Night**](https://www.meetup.com/rust-aarhus/events/311502123/)
* 2025-11-20 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ Monumental X Zed**](https://www.meetup.com/rust-amsterdam-group/events/311829267/)
* 2025-11-20 | Luzern, CH | [Rust Luzern](https://www.meetup.com/rust-luzern/)
    * [**2025 Rust Talks Luzern #3: Crate Walkthroughs @ Noser Engineering AG**](https://www.meetup.com/rust-luzern/events/311410681/)
* 2025-11-26 | Bergen, NO | [Hubbel kodeklubb](https://www.meetup.com/bergen-html-css-meetup-group/events/)
    * [**Workshop c# - nr 2 av 2 - grunnleggende nivå**](https://www.meetup.com/bergen-html-css-meetup-group/events/311784127/)
* 2025-11-26 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #5 @Source Engineers**](https://www.meetup.com/rust-bern/events/311792568/)
* 2025-11-27 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**19th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/311787736/)
* 2025-11-27 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**November Talks: Exotically Sized Types and ...**](https://www.meetup.com/rust-and-friends/events/311753411/)
* 2025-11-28 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/events/)
    * [**Rust Meetup Prague @ Barclays**](https://www.meetup.com/rust-prague/events/311846118/)
* 2025-12-03 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.github.io)
    * [**Rust Girona Hack & Learn 12 2025**](https://luma.com/8ncu1p8l)
* 2025-12-03 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Rust/ACCU meetup.**](https://www.meetup.com/oxford-rust-meetup-group/events/nnrkttyhcqbfb/)
* 2025-12-10 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2025 / 4 - Hacking Evening**](https://www.meetup.com/rust-munich/events/307105932/)
* 2025-12-10 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944053/)

### North America
* 2025-11-13 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Ipmap: Building Desktop Apps with Tauri**](https://www.meetup.com/utah-rust/events/311613658/)
* 2025-11-13 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Talks: Custom Rust Linting (ast-grep) & Geospatial DataFrame lib (SedonaDB)**](https://www.meetup.com/rust-nyc/events/311904744/)
* 2025-11-13 | Portland, OR, US | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**Rust Coding with AI**](https://www.meetup.com/pdxrust/events/311936952/)
* 2025-11-13 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust/events/)
    * [**San Diego Rust November Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/311876762/)
* 2025-11-16 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Brookline Rust Lunch, Nov 16**](https://www.meetup.com/bostonrust/events/311917229/)
* 2025-11-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/308865806/)
* 2025-11-20 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**November, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351673/)
* 2025-11-20 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/events/)
    * [**Monthly Rust Meetup: November**](https://www.meetup.com/spokane-rust/events/311863560/)
* 2025-11-23 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Inman Rust Lunch & Hackathon, Nov 23**](https://www.meetup.com/bostonrust/events/311917854/)
* 2025-11-26 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310457310/)
* 2025-11-27 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/jqvvttyhcpbkc/)
* 2025-11-29 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Harvard Square Rust Lunch, Nov 29**](https://www.meetup.com/bostonrust/events/311917256/)
* 2025-12-02 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Rust Talk December**](https://www.meetup.com/chicago-rust-meetup/events/311736848/)
* 2025-12-04 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Actix Web Unleashed: Mastering State, Security, and Scalable Handlers in Rust**](https://www.meetup.com/stl-rust/events/311396006/)
* 2025-12-05 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC Unconf 2025: Our Biggest Event Yet!**](https://www.meetup.com/rust-nyc/events/311757146/)
* 2025-12-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Downtown Rust Lunch, Dec 6**](https://www.meetup.com/bostonrust/events/311917263/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1nknaii/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Making your `unsafe` very tiny is sort of like putting caution markings *on* the lethally strong robot arm with no proximity sensors, rather than on the door into the protective cage.

– [Stephan Sokolow on lobste.rs](https://lobste.rs/c/0vkdmo)

Thanks to [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1727) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

This Week in Rust is edited by:
* [nellshamrell](https://github.com/nellshamrell)
* [llogiq](https://github.com/llogiq)
* [ericseppanen](https://github.com/ericseppanen)
* [extrawurst](https://github.com/extrawurst)
* [U007D](https://github.com/U007D)
* [mariannegoldin](https://github.com/mariannegoldin)
* [bdillo](https://github.com/bdillo)
* [opeolluwa](https://github.com/opeolluwa)
* [bnchi](https://github.com/bnchi)
* [KannanPalani57](https://github.com/KannanPalani57)
* [tzilist](https://github.com/tzilist)

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1ovpiqv/this_week_in_rust_625/)</small>
