Title: This Week in Rust 561
Number: 561
Date: 2024-08-21
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

* [This Development-cycle in Cargo: 1.81](https://blog.rust-lang.org/inside-rust/2024/08/15/this-development-cycle-in-cargo-1.81.html)
* [Async Closures MVP: Call for Testing!](https://blog.rust-lang.org/inside-rust/2024/08/09/async-closures-call-for-testing.html)

### Project/Tooling Updates

* [Rerun 0.18 - Exploiting column chunks for faster ingestion and lower memory use](https://rerun.io/blog/column-chunks)
* [`srgn 0.13`: new grep-like search mode for source code](https://github.com/alexpovel/srgn/releases/tag/srgn-v0.13.0)

### Observations/Thoughts

* [Expanding on withoutboat’s pinned places](https://poignardazur.github.io/2024/08/16/pinned-places/)
* [Introducing datafusion-uwheel, A Native DataFusion Optimizer for Time-based Analytics](https://uwheel.rs/post/datafusion_uwheel/)
* [What is a place expression?](https://www.ralfj.de/blog/2024/08/14/places.html)
* [Arenas](https://donsz.nl/blog/arenas/)
* [4 Years of Bevy](https://bd103.github.io/blog/2024-08-18-4-years-of-bevy)
* [Fixing Incorrect Tracing Parent Spans with Futures and JoinSet in Rust](https://chesedo.me/blog/rust-tracing-incorrect-parent-spans-async-futures-joinset/)

### Rust Walkthroughs

* [Rust GUI library via Flutter, done simple](https://cjycode.com/posts/rust-ui-flutter/)
* [Series] [Mastering Dependency Injection in Rust: Using a macro part 1](https://chesedo.me/blog/despatma-a-minimal-macro-for-dependency-injection/)

### Miscellaneous

* [The amazing Rust podcasts that have emerged in 2024](https://tim.mcnamara.nz/post/758930152514306048/the-amazing-rust-podcasts-that-have-emerged-in)
* [FreeBSD considers Rust in the base system](https://lwn.net/SubscriberLink/985210/f3c3beb9ef9c550e/)
* [Incorporating Lua with Livtet](https://www.jacky.wtf/essays/2024/using-lua-with-livtet/)
* [video] [Systems Software for Linux with Rust – Interview with Zeeshan Ali Khan](https://www.youtube.com/watch?v=atE94jvfVuA)
* [Building A Spreadsheet in Rust, WASM, and WebGL](https://filtra.io/rust-quadratic-aug-24)

## Crate of the Week

This week's crate is [discret](https://github.com/discretlib/discret), a graphQL-based peer-to-peer implementation library.

Thanks to [adsalais](https://users.rust-lang.org/t/crate-of-the-week/2704/1332) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* [RFC: Make Cargo respect minimum supported Rust version (MSRV) when selecting dependencies](https://github.com/rust-lang/rfcs/pull/3537)
    - [Testing Steps](https://github.com/rust-lang/cargo/issues/13873)

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

426 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-08-13..2024-08-20

* [add powerpc-unknown-linux-muslspe compile target](https://github.com/rust-lang/rust/pull/127905)
* [`-Znext-solver` caching](https://github.com/rust-lang/rust/pull/128828)
* [`ParamEnvAnd::fully_perform`: we have an `ocx`, use it](https://github.com/rust-lang/rust/pull/129078)
* [`derive(SmartPointer)`: register helper attributes](https://github.com/rust-lang/rust/pull/128925)
* [alloc: add ToString specialization for `&&str`](https://github.com/rust-lang/rust/pull/128759)
* [allow to customize `// TODO`: comment for deprecated safe autofix](https://github.com/rust-lang/rust/pull/127857)
* [check that `#[may_dangle]` is properly applied](https://github.com/rust-lang/rust/pull/129235)
* [cloneToUninit impls](https://github.com/rust-lang/rust/pull/126877)
* [delete debuginfo test suite infra for gdb without Rust support and lldb with Rust support](https://github.com/rust-lang/rust/pull/129218)
* [detect incompatible CI rustc options more precisely](https://github.com/rust-lang/rust/pull/129052)
* [detect multiple crate versions on method not found](https://github.com/rust-lang/rust/pull/128786)
* [emit a warning instead of an error if `--generate-link-to-definition` is used with other output formats than HTML](https://github.com/rust-lang/rust/pull/129050)
* [emit an error for invalid use of the linkage attribute](https://github.com/rust-lang/rust/pull/128989)
* [fix `is_val_statically_known` for floats](https://github.com/rust-lang/rust/pull/129173)
* [fix order of normalization and recursion in const folding](https://github.com/rust-lang/rust/pull/129208)
* [fix problems with assoc expr token collection](https://github.com/rust-lang/rust/pull/128725)
* [fix projections when parent capture is by-ref but child capture is by-value in the `ByMoveBody` pass](https://github.com/rust-lang/rust/pull/129101)
* [fix wrong argument for `get_fn_decl`](https://github.com/rust-lang/rust/pull/129223)
* [fix wrong source location for some incorrect macro definitions](https://github.com/rust-lang/rust/pull/129154)
* [include a copy of `compiler-rt` source in the `download-ci-llvm` tarball](https://github.com/rust-lang/rust/pull/129116)
* [infer async closure args from `Fn` bound even if there is no corresponding `Future` bound on return](https://github.com/rust-lang/rust/pull/129072)
* [only try to modify file times of a writable file on Windows](https://github.com/rust-lang/rust/pull/128977)
* [prevent double panic in query system, improve diagnostics](https://github.com/rust-lang/rust/pull/129271)
* [print more verbose error for commands that capture output](https://github.com/rust-lang/rust/pull/129096)
* [record the correct target type when coercing fn items/closures to pointers](https://github.com/rust-lang/rust/pull/129059)
* [refactor `powerpc64` call ABI handling](https://github.com/rust-lang/rust/pull/128643)
* [remove `print::Pat` from the printing of `WitnessPat`](https://github.com/rust-lang/rust/pull/128965)
* [remove redundant type ops: `Eq`/`Subtype`](https://github.com/rust-lang/rust/pull/129106)
* [return correct `HirId` when finding body owner in diagnostics](https://github.com/rust-lang/rust/pull/129168)
* [rework MIR inlining debuginfo so function parameters show up in debuggers](https://github.com/rust-lang/rust/pull/128861)
* [safe transmute: check lifetimes](https://github.com/rust-lang/rust/pull/129217)
* [special-case alias ty during the delayed bug emission in `try_from_lit`](https://github.com/rust-lang/rust/pull/129042)
* [suggest adding Result return type for associated method in E0277](https://github.com/rust-lang/rust/pull/128084)
* [support reading thin archives in ArArchiveBuilder](https://github.com/rust-lang/rust/pull/128936)
* [switch to using the v2 resolver in most workspaces](https://github.com/rust-lang/rust/pull/128722)
* [unconditionally allow shadow call-stack sanitizer for AArch64](https://github.com/rust-lang/rust/pull/128348)
* [use `FnSig` instead of raw `FnDecl` for `ForeignItemKind::Fn`, fix ICE for `Fn` trait error on safe foreign fn](https://github.com/rust-lang/rust/pull/128792)
* [use `ar_archive_writer` for writing COFF import libs on all backends](https://github.com/rust-lang/rust/pull/129164)
* [use the `enum2$` Natvis visualiser for repr128 C-style enums](https://github.com/rust-lang/rust/pull/128037)
* [when deduplicating unreachable blocks, erase the source information](https://github.com/rust-lang/rust/pull/128628)
* [added "copy" to Debug fmt for copy operands](https://github.com/rust-lang/rust/pull/122551)
* [miri: FD: remove big surrounding RefCell, simplify socketpair](https://github.com/rust-lang/miri/pull/3809)
* [miri: `tls_leak_main_thread_allowed`: make test check `target_thread_local`](https://github.com/rust-lang/miri/pull/3822)
* [miri: add 'project' process guidlines for larger contributions](https://github.com/rust-lang/miri/pull/3807)
* [miri: add epoll EPOLLHUP flag support](https://github.com/rust-lang/miri/pull/3814)
* [miri: add epollerr support](https://github.com/rust-lang/miri/pull/3820)
* [miri: borrow tracking: remove the concept of a call ID](https://github.com/rust-lang/miri/pull/3802)
* [miri: epoll `test_socketpair_read`: explicitly check real and Miri behavior](https://github.com/rust-lang/miri/pull/3825)
* [miri: epoll: iterate through output buffer then fetch an event from ready list](https://github.com/rust-lang/miri/pull/3818)
* [miri: implement epoll shim](https://github.com/rust-lang/miri/pull/3712)
* [miri: implement pipe and pipe2](https://github.com/rust-lang/miri/pull/3815)
* [miri: make unused states of Reserved unrepresentable](https://github.com/rust-lang/miri/pull/3754)
* [miri: set EINVAL for `epoll_wait` maxevent value 0](https://github.com/rust-lang/miri/pull/3824)
* [shrink `TyKind::FnPtr`](https://github.com/rust-lang/rust/pull/128812)
* [stabilize `Ready::into_inner()`](https://github.com/rust-lang/rust/pull/116528)
* [stabilize `asm_const`](https://github.com/rust-lang/rust/pull/128570)
* [stabilize `is_none_or`](https://github.com/rust-lang/rust/pull/129086)
* [stabilize `raw_ref_op` (RFC 2582)](https://github.com/rust-lang/rust/pull/127679)
* [stabilize `std::thread::Builder::spawn_unchecked`](https://github.com/rust-lang/rust/pull/129161)
* [stabilize `unsafe_attributes`](https://github.com/rust-lang/rust/pull/128771)
* [`CommandExt::before_exec`: deprecate safety in edition 2024](https://github.com/rust-lang/rust/pull/125970)
* [`std::fs: get_mode` implementation for all unix](https://github.com/rust-lang/rust/pull/128962)
* [add `#[must_use]` attribute to `Coroutine` trait](https://github.com/rust-lang/rust/pull/129034)
* [implement DoubleEnded and ExactSize for `Take<Repeat>` and `Take<RepeatWith>`](https://github.com/rust-lang/rust/pull/106943)
* [hash Ipv*Addr as an integer](https://github.com/rust-lang/rust/pull/128946)
* [optimize integer `pow` by removing the exit branch](https://github.com/rust-lang/rust/pull/122884)
* [std: refactor UNIX random data generation](https://github.com/rust-lang/rust/pull/128655)
* [add windows-targets crate to std's sysroot](https://github.com/rust-lang/rust/pull/128873)
* [cargo: add `--lockfile-path` flag](https://github.com/rust-lang/cargo/pull/14326)
* [cargo: correct diagnostic for `TomlDebugInfo`](https://github.com/rust-lang/cargo/pull/14413)
* [cargo: doc: convert comments to rustdoc in workspace](https://github.com/rust-lang/cargo/pull/14397)
* [cargo: fix MSRV for workspace .package and .dependencies](https://github.com/rust-lang/cargo/pull/14400)
* [cargo: fix: remove list owners feature of info subcommand](https://github.com/rust-lang/cargo/pull/14418)
* [cargo: implement path-bases (RFC 3529) 1/n: path dep and patch support](https://github.com/rust-lang/cargo/pull/14360)
* [rustdoc: greatly speed up doctests by compiling compatible doctests in one file](https://github.com/rust-lang/rust/pull/126245)
* [rustdoc: remove useless attributes in merged doctest generated code](https://github.com/rust-lang/rust/pull/129192)
* [rustdoc: add possibility to generate rustdoc JSON output to stdout](https://github.com/rust-lang/rust/pull/128963)
* [rustdoc-json: use FxHashMap from `rustdoc_json_types`](https://github.com/rust-lang/rust/pull/129124)
* [rustfmt: ensure that `fn config_path` returns canonicalized paths](https://github.com/rust-lang/rustfmt/pull/6272)
* [clippy: disable `assigning_clones` for tests](https://github.com/rust-lang/rust-clippy/pull/13273)
* [clippy: fix false positive in `explicit_iter_loop` with msrv](https://github.com/rust-lang/rust-clippy/pull/13288)
* [clippy: improve `collapsible_match` error message syntax](https://github.com/rust-lang/rust-clippy/pull/13284)
* [clippy: remove `find_format_arg_expr` AST fallback](https://github.com/rust-lang/rust-clippy/pull/13248)
* [clippy: trigger `string_slice` if expression is reference to `&str`](https://github.com/rust-lang/rust-clippy/pull/13281)
* [rust-analyzer: add scip/lsif flag to exclude vendored libaries](https://github.com/rust-lang/rust-analyzer/pull/17900)
* [rust-analyzer: `min_exhaustive_patterns`](https://github.com/rust-lang/rust-analyzer/pull/17853)
* [rust-analyzer: implement lifetime inferring](https://github.com/rust-lang/rust-analyzer/pull/17595)
* [rust-analyzer: make rust-analyzer work partially when offline](https://github.com/rust-lang/rust-analyzer/pull/17915)
* [rust-analyzer: fix panics for semantic highlighting at startup](https://github.com/rust-lang/rust-analyzer/pull/17932)
* [rust-analyzer: include generics when lowering extern type](https://github.com/rust-lang/rust-analyzer/pull/17925)
* [rust-analyzer: keep comments in `convert_while_to_loop`](https://github.com/rust-lang/rust-analyzer/pull/17928)
* [rust-analyzer: panic when a TAIT exists in a RPIT](https://github.com/rust-lang/rust-analyzer/pull/17924)
* [rust-analyzer: panic while canonicalizing erroneous projection type](https://github.com/rust-lang/rust-analyzer/pull/17882)
* [rust-analyzer: panic while hovering associated function with type annotation on generic param that not inherited from its container type](https://github.com/rust-lang/rust-analyzer/pull/17893)
* [rust-analyzer: properly account for editions in names](https://github.com/rust-lang/rust-analyzer/pull/17905)
* [rust-analyzer: fix wrong BoundVar index when lowering impl trait parameter of parent generics](https://github.com/rust-lang/rust-analyzer/pull/17916)

### Rust Compiler Performance Triage

A fairly noisy week (though most of that has been dropped from this report).
Overall we saw several improvements, and ended the week on a net positive.
Memory usage is down around 1.5-3% over the course of the week, primarily due
to [RawVec polymorphization](https://github.com/rust-lang/rust/pull/126793) and
[CloneToUninit impl expansion](https://github.com/rust-lang/rust/pull/126877).

Triage done by **@simulacrum**.
Revision range: [9cb1998e..4fe1e2bd](https://perf.rust-lang.org/?start=9cb1998ea15e179482504e07cad8fa121e169a32&end=4fe1e2bd5bf5a6f1cb245f161a5e9d315766f103&absolute=false&stat=instructions%3Au)

1 Regressions, 1 Improvements, 3 Mixed; 1 of them in rollups
53 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-08-19.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [crates.io: Crate Deletions](https://github.com/rust-lang/rfcs/pull/3660)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [impl `Default` for `HashMap`/`HashSet` iterators that don't already have it](https://github.com/rust-lang/rust/pull/128711)
* [disposition: merge] [stabilize const_fn_floating_point_arithmetic](https://github.com/rust-lang/rust/pull/128596)
* [disposition: merge] [Non-exhaustive structs may be empty](https://github.com/rust-lang/rust/pull/128934)
* [disposition: merge] [Lint at `deny-by-default` against references to `static mut`](https://github.com/rust-lang/rust/issues/128794)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [feat: Stablize `CARGO_RUSTC_CURRENT_DIR`](https://github.com/rust-lang/cargo/pull/13644)

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Trait method impl restrictions](https://github.com/rust-lang/rfcs/pull/3678)

## Upcoming Events

Rusty Events between 2024-08-21 - 2024-09-18 🦀

### Virtual
* 2024-08-21 | Hybrid - Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Surreal-ORM**](https://www.meetup.com/vancouver-rust/events/298631735/)
* 2024-08-22 | Virtual | [Conf42: Online Tech Events](https://www.meetup.com/conf42/)
    * [**Conf42 Rustlang 2024**](https://www.meetup.com/conf42/events/297266825/)
* 2024-08-22 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897938/)
* 2024-08-22 | Virtual (Karlsruhe, DE) | [Karlsruhe Functional Programmers Group](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/)
    * [**Stammtisch (gemeinsam mit der C++ UG KA): various topics, from C++ to Rust**](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/events/293937801)
* 2024-08-27 | Virtual | [Ardan Labs](https://www.eventbrite.com/o/ardan-labs-7092394651)
    * [**Fearless Concurrency with Rust**](https://www.eventbrite.com/e/fearless-concurrency-with-rust-tickets-934569591807)
* 2024-08-27 | Virtual (Bordeaux, FR) | [Rust Bordeaux](https://www.meetup.com/bordeaux-rust/)
    * [**Live coding - A distance #1**](https://www.meetup.com/bordeaux-rust/events/302570681/)
* 2024-08-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/301585668/)
* 2024-08-27 | Virtual (Tel Aviv, IL) | [Rust in Israel](https://www.meetup.com/rust-in-israel/)
    * [**Declarative macros in Rust (Virtual) - מקרוים בראסט**](https://www.meetup.com/rust-in-israel/events/302327956/)
* 2024-08-28 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Command Line Tools: Implementing wc in Rust (English, Virtual)**](https://www.meetup.com/code-mavens/events/302151487/)
* 2024-08-29 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633267/)
* 2024-08-29 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Rust Source Code Reading: The thousands crate (English)**](https://www.meetup.com/code-mavens/events/302391142/)
* 2024-09-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/302007365/)
* 2024-09-04 | Virtual (Indianapolis, IN, US) | [Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - Typestate Pattern in Rust: With a Strict Builder Example**](https://www.meetup.com/indyrs/events/300328029/)
* 2024-09-05 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897957/)
* 2024-09-05 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820268/)
* 2024-09-10 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346981/)
* 2024-09-10 - 2024-09-13 | Hybrid: Virtual and In-Person (Montreal, QC, CA)| [Rust Conf](https://rustconf.com/)
    * [**Rust Conf 2024**](https://foundation.rust-lang.org/events/rustconf-2024/)
* 2024-09-12 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633268/)
* 2024-09-12 | Virtual (Rotterdam, NL)| [Bevy Game Development](https://www.meetup.com/bevy-game-development/)
    * [**Bevy Meetup #6**](https://www.meetup.com/bevy-game-development/events/302841892/)
* 2024-09-16 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/302827971/)
* 2024-09-18 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Cells**](https://www.meetup.com/vancouver-rust/events/298631736/)
* 2024-09-18 - 2024-09-20 | Hybrid - Virtual and In-Person (Vienna, AT) | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2024**](https://lpc.events/event/18/sessions/186/)

### Africa
* 2024-09-06 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587/)

### Asia
* 2024-08-24 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**August 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/august-2024-rustacean-meetup/)
* 2024-09-09 | Ramat Gan, IL | [Coralogix](https://coralogix.com/)
    * [**Rust as Scale**](https://coralogix.com/rust-coralogix-meetup/)

### Europe
* 2024-08-21 | Cabridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/302574953/)
* 2024-08-21 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Walk'n'Talk around Wöhrder See (+ Burritos)**](https://www.meetup.com/rust-noris/events/302080495/)
* 2024-08-22 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester Talks August**](https://www.meetup.com/rust-manchester/events/302419276/)
* 2024-08-26 | Mainz, DE | [Fachschaft Mathematik+Informatik der JGU Mainz](https://rheinneckar.events/@fsmathe_informatik_mainz@rheinmain.social)
    * [**Ferienkurs Rust**](https://rheinneckar.events/events/3f76f860-75c1-4f3a-810f-03fc0cecb691/)
* 2024-08-27 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/)
    * [**Exploring Slint: A Rust-based UI Toolkit – Mob Programming Session**](https://www.meetup.com/rust-trondheim/events/300991355/)
* 2024-08-28 | Frankfurt (Main), DE | [Rust Rhein Main](https://www.meetup.com/rust-rhein-main)
    * [**Rust Frankfurt WebAssembly**](https://www.meetup.com/rust-rhein-main/events/302738445/)
* 2024-08-29 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299421378/)
* 2024-09-11 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/302833564/)
* 2024-09-18 | Moravia, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/)
    * [**Rust Moravia Meetup (September 2024)**](https://www.meetup.com/rust-moravia/events/301360936)
* 2024-09-18 | Vienna, AT + Virtual | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2024 (Sep 18-20)**](https://lpc.events/event/18/sessions/186/)

### North America
* 2024-08-21 | Hybrid - Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Surreal-ORM**](https://www.meetup.com/vancouver-rust/events/298631735/)
* 2024-08-22 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/302723527/) | [**Alternative Meetup Link**](https://www.meetup.com/mv-rust-meetup/events/302723816/)
* 2024-08-26 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**Boulder Rust Meetup: Learnings and Hangs!**](https://www.meetup.com/boulder-rust-meetup/events/302579817/)
* 2024-08-28 | Austin, TX, US | [Rust ATC](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygclblc/)
* 2024-08-29 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers : placeholder**](https://www.meetup.com/music-city-rust-developers/events/301420110/)
* 2024-09-05 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/302723843/)
* 2024-09-05 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Lifetimes**](https://www.meetup.com/stl-rust/events/hdzdmtygcmbhb/)
* 2024-09-10 - 2024-09-13 | Hybrid: Virtual and In-Person (Montreal, QC, CA)| [Rust Conf](https://rustconf.com/)
    * [**Rust Conf 2024**](https://foundation.rust-lang.org/events/rustconf-2024/)
* 2024-09-18 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Cells**](https://www.meetup.com/vancouver-rust/events/298631736/)

# Oceania
* 2024-08-22 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Dot IX: Diagram Generator + Deep Learning from Scratch in Rust**](https://www.meetup.com/rust-akl/events/302431924/)
* 2024-08-27 | Canberra, ACT, AU | [Canberra Rust User Group (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/301887261/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1ecdzp2/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> I'm trying to round up to next power of two (for fun).

> I know that's perhaps not a lot of fun, but there's [next_power_of_two() ](https://doc.rust-lang.org/std/primitive.u32.html#method.next_power_of_two) on all integer types.

> That is indeed less fun.

– [Edeadlink and Riccardo Borgani on rust-users](https://users.rust-lang.org/t/stupid-or-genius-getting-at-the-bits/116138)

Thanks to [Jonas Fassbender](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1600) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1eyyz3c/this_week_in_rust_561/)</small>
