Title: This Week in Rust 644
Number: 644
Date: 2026-03-25
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/main/CONTRIBUTING.md).

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
* [What we heard about Rust's challenges](https://blog.rust-lang.org/2026/03/20/rust-challenges/)
* [Security advisory for Cargo](https://blog.rust-lang.org/2026/03/21/cve-2026-33056/)

### Foundation
* [Canonical Joins the Rust Foundation as a Gold Member](https://rustfoundation.org/media/canonical-joins-the-rust-foundation-as-a-gold-member/)

### Newsletters
* [Scientific Computing in Rust #16 (March 2026)](https://scientificcomputing.rs/monthly/2026-03)

### Project/Tooling Updates
* [Ferox - A native PostgreSQL client in Rust](https://dev.to/frkdrgt/i-built-a-postgresql-client-in-rust-because-dbeaver-was-eating-my-ram-1eg1)
* [Introducing dial9: A flight recorder for Tokio](https://tokio.rs/blog/2026-03-18-dial9)
* [Zellij 0.44: native Windows support, new Rust APIs, remote sessions](https://zellij.dev/news/remote-sessions-windows-cli/)
* [vigil-rs: A Rust Service Supervisor for Containers with PID 1 handling](https://blog.none.at/blog/2026/2026-03-21-virgil/)
* [Fyrox 1.0.0](https://fyrox.rs/blog/post/fyrox-game-engine-1-0-0/)
* [Edge.js: running Node.js safely in a WebAssembly sandbox with Wasmer and WASIX](https://wasmer.io/posts/edgejs-safe-nodejs-using-wasm-sandbox)
* [Bookokrat v0.3.8: A terminal EPUB / PDF Book Reader now supports DJVU](https://github.com/bugzmanov/bookokrat/releases/tag/v0.3.8)
* [flodl v0.1.5: benchmarking Rust vs PyTorch on 7 models — up to 30% faster with 3-20x tighter variance](https://flodl.dev/blog/benchmarks)
* [Zero-copy Protobuf and ConnectRPC for Rust](https://dev.to/iainmcgin/zero-copy-protobuf-and-connectrpc-for-rust-1m3e)
* [mtp-rs: pure-Rust MTP library, up to 4x faster than libmtp](https://www.veszelovszki.com/a/mtp-rs/)
* [video] [Batty: Supervised Agent Execution for Software Teams — Demo](https://www.youtube.com/watch?v=2wmBcUnq0vw)
* [indxr v0.2.0: A fast codebase indexer and MCP server for AI coding agents](https://github.com/bahdotsh/indxr/releases/tag/v0.2.0)
* [halloy 2026.5 - desktop IRC client with IRCv3 capabilities](https://github.com/squidowl/halloy/releases/tag/2026.5)

### Observations/Thoughts
* [Deadlocking a Tokio Mutex without Holding a Lock](https://www.e6data.com/blog/deadlocking-tokio-mutex-without-holding-lock)
* [The Good, the Bad, and the Leaky: jemalloc, bumpalo, and mimalloc in meilisearch](https://blog.kerollmops.com/the-good-the-bad-and-the-leaky-jemalloc-bumpalo-and-mimalloc-in-meilisearch)
* [Maximally minimal view types](https://smallcultfollowing.com/babysteps/blog/2026/03/21/view-types-max-min/)
* [Matching Puzzle Pieces and Disappointing Benchmarks](https://llogiq.github.io/2026/03/20/case.html)
* [What If Traits Carried Values](https://nadrieril.github.io/blog/2026/03/22/what-if-traits-carried-values.html)
* [An effect notation based on with-clauses and blocks](https://blog.yoshuawuyts.com/a-with-based-effect-notation)
* [Rust threads on the GPU](https://www.vectorware.com/blog/threads-on-gpu)
* [Elaborating Rust Traits to Dictionary-Passing Style](https://nadrieril.github.io/blog/2026/03/20/dictionary-passing-style.html)

### Rust Walkthroughs
* [ZK snarks for rust developer part 2/8](https://rustarians.com/roots-of-unity/)
* [Let's see Paul Allen's SIMD CSV parser](https://chunkofcoal.com/posts/simd-csv/)
* [Building an LSP Server with Rust is surprisingly easy and fun](https://codeinput.com/blog/lsp-server)
* [An Incoherent Rust](https://www.boxyuwu.blog/posts/an-incoherent-rust/)
* [Building pentest devices with Rust and ESP32 microcontrollers](https://kerkour.com/rust-esp32-pentest)
* [Rust in Intersec's lib-common, Part 1: Integrating Rust in a C Build System](https://techtalk.intersec.com/2026/03/rust-in-lib-common-part-1-integrating-rust-in-a-waf-based-c-build-system/)

## Crate of the Week

This week's crate is [noq](https://github.com/n0-computer/noq), a general purpose implementation of the [QUIC transport protocol](https://www.rfc-editor.org/rfc/rfc9000.html) in pure rust.

Thanks to [Brendan O'Brien](https://users.rust-lang.org/t/crate-of-the-week/2704/1569) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

*No calls for testing were issued this week by
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Rustup](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) or
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

* [Wild Almonds — Appimage fails to start](https://github.com/opeolluwa/almonds/issues/75)
* [Wild Almonds — implement workspace for user_preference](https://github.com/opeolluwa/almonds/issues/116)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

* [**EuroRust**](https://sessionize.com/eurorust-2026/) | CFP open until 2026-04-27 | Barcelona, Spain | 2026-10-14 - 2026-10-17
* [**NDC Techtown 2026**](https://pretalx.com/oxidize-conference-2026-2025/cfp) | CFP open until 2026-05-03 | Kongsberg, Norway | 2026-09-21 - 2026-09-24

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

433 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-03-17..2026-03-24

#### Compiler
* [fix some suggestions of the `for-loops-over-fallibles` lint](https://github.com/rust-lang/rust/pull/153913)
* [guard patterns: lowering to THIR](https://github.com/rust-lang/rust/pull/153828)
* [introduce `#[diagnostic::on_move(message)]`](https://github.com/rust-lang/rust/pull/150935)
* [make `par_slice` consistent with single-threaded execution](https://github.com/rust-lang/rust/pull/153768)
* [privacy: fix type privacy holes in RPITITs](https://github.com/rust-lang/rust/pull/152543)

#### Library
* [add APIs for dealing with titlecase](https://github.com/rust-lang/rust/pull/122668)
* [add `is_disconnected` functions to mpsc and mpmc channels](https://github.com/rust-lang/rust/pull/153170)
* [implement `BinaryHeap::as_mut_slice`](https://github.com/rust-lang/rust/pull/154011)
* [make `OsString::truncate` a no-op when `len > current_len`](https://github.com/rust-lang/rust/pull/152998)
* [optimize 128-bit integer formatting](https://github.com/rust-lang/rust/pull/154077)
* [optimize `BTreeMap::append()` using CursorMut](https://github.com/rust-lang/rust/pull/153107)
* [`vec::Drain::fill`: avoid reference to uninitialized memory](https://github.com/rust-lang/rust/pull/154138)
* [unstable impl of `From<{i64, u64}> for f128`](https://github.com/rust-lang/rust/pull/154012)

#### Cargo
* [clean: Validate that `target_dir` is not a file](https://github.com/rust-lang/cargo/pull/16765)
* [`cli`: Add support for completing `--config` argument values with `native-completions`](https://github.com/rust-lang/cargo/pull/16249)
* [`cli`: complete `--config` and `--color` before command](https://github.com/rust-lang/cargo/pull/16780)
* [`compile`: Make build.warnings ignore non-local deps](https://github.com/rust-lang/cargo/pull/16760)
* [fix `symlink_and_directory` when running in a long target dir name](https://github.com/rust-lang/cargo/pull/16775)
* [detect circular publish dependency cycle in workspace publish](https://github.com/rust-lang/cargo/pull/16722)
* [fix fetching non-standard git refspecs on non-github repos](https://github.com/rust-lang/cargo/pull/16768)
* [warn when installing with a non-default toolchain](https://github.com/rust-lang/cargo/pull/16131)

#### Clippy
* [add `BinaryHeap::pop_if()` to `manual_pop_if`](https://github.com/rust-lang/rust-clippy/pull/16734)
* [fix `collapsible_match` false positive when the pat binding is moved or mutated](https://github.com/rust-lang/rust-clippy/pull/16708)
* [perf: `manual_is_ascii_check,` remove 822 million instructions](https://github.com/rust-lang/rust-clippy/pull/16755)

#### Rust-Analyzer
* [add `ops::AddAssign` implement for IndentLevel](https://github.com/rust-lang/rust-analyzer/pull/20601)
* [add applicable on LetExpr for `unwrap_tuple`](https://github.com/rust-lang/rust-analyzer/pull/20600)
* [add applicable on let-else branch for `unwrap_block`](https://github.com/rust-lang/rust-analyzer/pull/21473)
* [add auto trait name for `generate_trait_from_impl`](https://github.com/rust-lang/rust-analyzer/pull/20299)
* [add fixes for `non_exhaustive_let` diagnostic](https://github.com/rust-lang/rust-analyzer/pull/20762)
* [add mapping to syntax factory constructor methods](https://github.com/rust-lang/rust-analyzer/pull/21832)
* [add nested lifetime support for `add_lifetime_to_type`](https://github.com/rust-lang/rust-analyzer/pull/20628)
* [add partial selection for `merge_imports`](https://github.com/rust-lang/rust-analyzer/pull/20566)
* [add wrap multiple attr for `wrap_unwrap_cfg_attr`](https://github.com/rust-lang/rust-analyzer/pull/20625)
* [change `test_name` placeholder to `executable_arg`](https://github.com/rust-lang/rust-analyzer/pull/21395)
* [complete block .let in closure expression](https://github.com/rust-lang/rust-analyzer/pull/21756)
* [offer `'add_braces'` on bin-expr assignment](https://github.com/rust-lang/rust-analyzer/pull/21850)
* [offer on let-expr for `inline_local_variable`](https://github.com/rust-lang/rust-analyzer/pull/21775)
* [fix asm sym operand parsing for parenthesized expr fragments](https://github.com/rust-lang/rust-analyzer/pull/21588)
* [fix indent for `convert_closure_to_fn`](https://github.com/rust-lang/rust-analyzer/pull/20594)
* [fix indent for `trait_impl_redundant_assoc_item`](https://github.com/rust-lang/rust-analyzer/pull/20681)
* [fix not applicable on empty `struct` for `no_such_field`](https://github.com/rust-lang/rust-analyzer/pull/20614)
* [fix other predicate for `replace_is_method_with_if_let_method`](https://github.com/rust-lang/rust-analyzer/pull/21787)
* [fix postfix completion indentation compensation](https://github.com/rust-lang/rust-analyzer/pull/21324)
* [fix tuple `struct` pat expected type](https://github.com/rust-lang/rust-analyzer/pull/21333)
* [add `ident_pat` qualifier to fully fn param](https://github.com/rust-lang/rust-analyzer/pull/21768)
* [don't add a second semicolon after postfix completions](https://github.com/rust-lang/rust-analyzer/pull/21839)
* [fill match arms on last comma and empty expr](https://github.com/rust-lang/rust-analyzer/pull/21822)
* [fix overlap edit on record to tuple assist uses self](https://github.com/rust-lang/rust-analyzer/pull/21817)
* [incorrect flychecks with multiple workspaces](https://github.com/rust-lang/rust-analyzer/pull/21709)
* [offer on const like path-expr for `'extract_variable'`](https://github.com/rust-lang/rust-analyzer/pull/21809)
* [replace TODO placeholders in next-solver IrPrint with proper formatting](https://github.com/rust-lang/rust-analyzer/pull/21779)
* [implement signature type inference](https://github.com/rust-lang/rust-analyzer/pull/21823)
* [improve tmp iterator variable name for `convert_for_to_while_let`](https://github.com/rust-lang/rust-analyzer/pull/20979)
* [migrate `convert_from_to_tryfrom` assist to SyntaxEditor API](https://github.com/rust-lang/rust-analyzer/pull/21843)
* [project json compatibility improvements](https://github.com/rust-lang/rust-analyzer/pull/21423)
* [project json compatibility improvements](https://github.com/rust-lang/rust-analyzer/pull/21423)
* [remove doc comments for `generate_trait_from_impl`](https://github.com/rust-lang/rust-analyzer/pull/20407)
* [remove outdated TODO](https://github.com/rust-lang/rust-analyzer/pull/21845)
* [remove the mapping for `expr_underscore` from the syntax factory constructor](https://github.com/rust-lang/rust-analyzer/pull/21848)
* [replace direct usage of make with syntax factory and migrate assist to syntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/21847)
* [support WhileExpr and ForExpr for `add_label_to_loop`](https://github.com/rust-lang/rust-analyzer/pull/20984)
* [support more runnable kinds in project JSON](https://github.com/rust-lang/rust-analyzer/pull/21424)

### Rust Compiler Performance Triage


Lot of mixed results this week. One big regression from [#152931](https://github.com/rust-lang/rust/pull/152931) makes the results look pretty negative, but otherwise the week was fairly quiet.

Triage done by **@panstromek**.
Revision range: [5b61449e..6f22f613](https://perf.rust-lang.org/?start=5b61449ed85a670f1dd3fca6a8c759ee0b451b66&end=6f22f61305478df09f9a4523743f85d9f558c3d7&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 1.0%  | [0.1%, 4.2%]   | 27    |
| Regressions ❌ <br /> (secondary)  | 0.2%  | [0.0%, 0.6%]   | 36    |
| Improvements ✅ <br /> (primary)   | -0.1% | [-0.2%, -0.1%] | 3     |
| Improvements ✅ <br /> (secondary) | -0.3% | [-2.8%, -0.0%] | 14    |
| All ❌✅ (primary)                 | 0.9%  | [-0.2%, 4.2%]  | 30    |


1 Regression, 1 Improvement, 4 Mixed; 1 of them in rollups
32 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/75edcf889ba56f439f91a3c576388d9969dc5a16/triage/2026/2026-03-24.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: map_or_default in Option and Result](https://github.com/rust-lang/rfcs/pull/3148)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Tracking Issue for fN::BITS](https://github.com/rust-lang/rust/issues/151073)
* [Fallback `{float}` to `f32` when `f32: From<{float}>` and add `impl From<f16> for f32`](https://github.com/rust-lang/rust/pull/139087)
* [Tracking Issue for tcp_deferaccept](https://github.com/rust-lang/rust/issues/119639)
* [Tracking Issue for #138068: Add `Result::map_or_default` and `Option::map_or_default`](https://github.com/rust-lang/rust/issues/138099)
* [Merge `fabsf16/32/64/128` into `fabs::<F>`](https://github.com/rust-lang/rust/pull/153834)
* [1.95 beta regression: "malformed feature attribute input"](https://github.com/rust-lang/rust/issues/153764)
* [Never break between empty parens](https://github.com/rust-lang/rust/issues/152761)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [feat: add frame-pointers profile option](https://github.com/rust-lang/cargo/pull/16742)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)

* [Emit retags in codegen](https://github.com/rust-lang/compiler-team/issues/958)
* [Optimize `repr(Rust)` enums by omitting tags in more cases involving uninhabited variants.](https://github.com/rust-lang/compiler-team/issues/922)
* [Proposal for a dedicated test suite for the parallel frontend](https://github.com/rust-lang/compiler-team/issues/906)
* [Promote tier 3 riscv32 ESP-IDF targets to tier 2](https://github.com/rust-lang/compiler-team/issues/864)
* [Proposal for Adapt Stack Protector for Rust](https://github.com/rust-lang/compiler-team/issues/841)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Guarantee alignment of fixed-width integer primitives](https://github.com/rust-lang/reference/pull/2205)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [Propose the Rust Foundation Maintainer fund](https://github.com/rust-lang/rfcs/pull/3931)
* [Avoid linting `unreachable_code` on `todo!()`](https://github.com/rust-lang/rfcs/pull/3928)

## Upcoming Events

Rusty Events between 2026-03-25 - 2026-04-22 🦀

### Virtual
* 2026-03-25 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 03 2026**](https://luma.com/vq9w8q0w)
* 2026-03-26 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455925/)
* 2026-03-31 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens)
    * [**Web development using axum in Rust - part 1**](https://www.meetup.com/code-mavens/events/313944077/)
* 2026-04-01 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/me4jwgxu)
* 2026-04-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/313656388/)
* 2026-04-02 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345237/)
* 2026-04-04 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-04-09 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455926/)
* 2026-04-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254784/)
* 2026-04-14 | Virtual (London, GB) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/313506013/)
* 2026-04-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Nushell**](https://www.meetup.com/vancouver-rust/events/313471712/)
* 2026-04-19 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/313846195/)
* 2026-04-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/313846195/)

### Asia
* 2026-03-28 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/events/)
    * [**Rust Delhi Meetup #13**](https://www.meetup.com/rustdelhi/events/313777790/)
* 2026-04-17 | Bangalore, IN, [Rust India](https://rustindia.org/)
    * [**Rust India Workshop**](https://rustindia.org/schedule)
* 2026-04-18 | Bangalore, IN, [Rust India](https://rustindia.org/)
    * [**Rust India Conference**](https://rustindia.org/schedule)

### Europe
* 2026-03-25 | Dresden, DE | [Rust Dresden](https://github.com/rust-dresden)
    * [**First Meetup**](https://github.com/rust-dresden/rust-dresden/discussions/7)
* 2026-03-26 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust meetup #66 sponsored by Adapt!**](https://www.meetup.com/copenhagen-rust-community/events/313833635/)
* 2026-03-26 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #84**](https://www.meetup.com/rust-paris/events/313646981/)
* 2026-03-27 | Paris, FR | [Rust in Paris](https://www.rustinparis.com/)
    * [**Rust in Paris**](https://www.rustinparis.com/)
* 2026-03-28 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #25**](https://www.meetup.com/stockholm-rust/events/313749232/)
* 2026-04-01 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/313783250/)
* 2026-04-01 | Edinburgh, GB | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (evening pub)**](https://www.meetup.com/rust-and-friends/events/313898254/)
* 2026-04-01 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Rust/ACCU meetup.**](https://www.meetup.com/oxford-rust-meetup-group/events/312664491/)
* 2026-04-02 | London, GB | [Rust London User Group](https://www.meetup.com/rust-london-user-group/events/)
    * [**LDN Talks Spring Community Showcase**](https://www.meetup.com/rust-london-user-group/events/313816694/)
* 2026-04-03 | Edinburgh, GB | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/313898258/)
* 2026-04-07 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #15 @ letsboot**](https://www.meetup.com/rust-basel/events/313765547/)
* 2026-04-09 | Geneva, CH | [Rust Meetup Geneva](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-04-09 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust talks @ AutoStore – "Patterns for Event Driven Systems" and "Rust + WASM"**](https://www.meetup.com/rust-oslo/events/313806765/)
* 2026-04-21 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Native GUIs with Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813853/)

### North America
* 2026-03-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/313653030/)
* 2026-03-25 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC's Digital Asset Adoption Special**](https://www.meetup.com/rust-nyc/events/313713085/)
* 2026-03-26 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/311228658/)
* 2026-03-28 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Chinatown Rust Lunch, Mar 28**](https://www.meetup.com/bostonrust/events/313883686/)
* 2026-04-02 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/313543900/)
* 2026-04-02 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**SIUE Cruft Crawler with LLM**](https://www.meetup.com/stl-rust/events/313482094/)
* 2026-04-04 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Winter Hill Rust Lunch, Apr 4**](https://www.meetup.com/bostonrust/events/313883689/)
* 2026-04-09 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust/events/)
    * [**San Diego Rust April Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/313721879/)
* 2026-04-11 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Brookline Rust Lunch, Apr 11**](https://www.meetup.com/bostonrust/events/313883710/)
* 2026-04-14 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Sharpening Your Rust Skills for Job Interviews**](https://www.meetup.com/charlottesville-rust-meetup/events/313262215/)
* 2026-04-16 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**April, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873177/)
* 2026-04-18 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Harvard Square Rust Lunch, Apr 18**](https://www.meetup.com/bostonrust/events/313883701/)
* 2026-04-20 - 2026-04-22 | Portland, OR | [Tokio](https://tokio.rs/)
    * [**TokioConf 2026**](https://www.tokioconf.com/)
* 2026-04-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/313918531/)
* 2026-04-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyjcgbdc/)

### Oceania
* 2026-03-26 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**TBD March Meetup**](https://www.meetup.com/rust-melbourne/events/313471749/)

### South America
* 2026-04-11 | Argentina, AR | [Oxidar Org](https://luma.com/user/oxidar)
    * [**Oxidar.org Hackaton - Snakear - ¡Veni a hackear con Rust!**](https://luma.com/5f51ey45)
* 2026-04-17 | Rio de Janeiro, BR | [Meetups Rust RJ](https://luma.com/calendar/cal-z65k0aMSe7DaqKv)
    * [**Meetup Rust RJ**](https://luma.com/ce46pl7z)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1rmra27/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Code does not become better out of thin air just because you rewrite it in #rustlang. 

– [allp on mastodon](https://mastodon.online/@alip/116275090869947511)

Despite a third week gone by without a suggestion, llogiq is unrelenting in his quest to find a quote worth your while.

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1s3uoo0/this_week_in_rust_644/)</small>
