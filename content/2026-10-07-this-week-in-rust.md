Title: This Week in Rust 672
Number: 672
Date: 2026-10-07
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
* [Announcing Rust 1.99.0](https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/)
* [Generic Const Args and You](https://blog.rust-lang.org/inside-rust/2026/10/02/generic-const-args-and-you/)
* [video] [Rust Release Changelog - 1.99.0](https://www.youtube.com/watch?v=SLV9AJxnb1U)

### Foundation
* [A Fond Farewell To Three Rust Foundation Colleagues](https://rustfoundation.org/media/a-fond-farewell-to-three-rust-foundation-colleagues/)

### Newsletters
* [This Month in Rust OSDev: September 2026](https://rust-osdev.com/this-month/2026-09/index.html)
* [Rust Trends Issue 83 - NVIDIA Brings Rust to the GPU Kernel](https://rust-trends.com/newsletter/nvidia-brings-rust-to-the-gpu-kernel/)
* [Rust Trends Issue 84 - Google Puts Agents on the Rust Rewrite](https://rust-trends.com/newsletter/google-puts-agents-on-the-rust-rewrite/)

### Project/Tooling Updates

<!-- IMPORTANT NOTE: We are no longer accepting pull request submissions for the Project/Tooling Updates section.
See here for details: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

* [Generate PDFs from Rust with HTML and CSS](https://docs.fullbleed.dev/getting-started/rust/)
* [Catharsis for Noisy Audio: A Pure-Rust Restoration Toolkit with No ffmpeg and No Black Boxes](https://medium.com/@vbasky/catharsis-for-noisy-audio-a-pure-rust-restoration-toolkit-with-no-ffmpeg-and-no-black-boxes-a6c5c38e4c14)
* [Release mold 3.0.0 · rui314/mold](https://github.com/rui314/mold/releases/tag/v3.0.0)

### Observations/Thoughts
* [Rust for CPython (Python Language Summit 2026)](https://blog.python.org/2026/09/language-summit-2026-rust-for-cpython/)
* [Lies, damned lies, and Rust in the TechEmpower Web Framework Benchmarks](https://kerkour.com/rust-techempower-benchmarks)
* [Beyond the `&`](https://lwn.net/SubscriberLink/1096028/7524dbcae1be7205/)
* [The Performance Cost of RwLock in Our Read-Heavy Workload](https://pranitha.dev/posts/rwlock-vs-lockfree/)
* [Shipping JPEG XL in Chrome](https://developer.chrome.com/blog/jpeg-xl-in-chrome)
* [Proving Rust Web Application Correctness with Lean 4](https://medium.com/@Koukyosyumei/proving-rust-web-application-correctness-with-lean-4-8889583f1e15)
* [Hardware-Aware Programming in Rust](https://medium.com/@alan0408yuan/hardware-aware-programming-in-rust-6e68a70c1535?postPublishedType=repub)

### Rust Walkthroughs
* [The Missing Piece in Rust Error Handling](https://mcmah309.github.io/posts/the-missing-piece-in-rust-error-handling/)
* [Compiling the kernel with gccrs](https://lwn.net/Articles/1095553/)
* [Declarative Macros in Rust: A Simple and Practical Introduction](https://sigseis.dev/articles/2026/10/06)
* [A dynamic drone fail-safe system that adapts as the situation changes.](https://www.deepcausality.com/tutorials/dynamic-drone-failsafe/)
* [Build a Burglar Alarm with ESP32-C5 That Sends Telegram Alerts](https://iot.implrust.com/burglar-alarm/index.html)

## Crate of the Week

This week's crate is [karatepe](https://codeberg.org/miroo/karatepe), a statically typed localisation language and library.

Thanks to [miro](https://users.rust-lang.org/t/crate-of-the-week/2704/1690) for the self-suggestion!

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
* [issuerd - Add a French (fr) message bundle for login pages and emails](https://github.com/issuerd/issuerd/issues/1)
* [issuerd - Add proptest suites for issuerd-protocol parsers](https://github.com/issuerd/issuerd/issues/3)
* [issuerd - Add an additional client installation provider (adapter config download format)](https://github.com/issuerd/issuerd/issues/5)
* [ruxen - Expand globs in include](https://github.com/gvozdetsky/ruxen/issues/35)
* [ruxen - Use nginx's status reason phrases everywhere](https://github.com/gvozdetsky/ruxen/issues/33)
* [ruxen - Implement proxy_method](https://github.com/gvozdetsky/ruxen/issues/39)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [**RustWeek 2027**](https://sessionize.com/rustweek-2027/) | CFP closes 2027-01-10 | Utrecht, The Netherlands | Event date: 2027-05-24
* [**TokioConf 2027**](https://tokio.rs/blog/2026-10-06-tokioconf-2027-cfp) | CFP closes 2026-11-30 | Portland, Oregon, USA | 2027-04-26 - 2027-04-27

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

653 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-09-29..2026-10-06

#### Compiler
* [add a single-entry parent `SpanData` cache](https://github.com/rust-lang/rust/pull/163628)
* [add fast path to generalization](https://github.com/rust-lang/rust/pull/163607)
* [optimize Cranelift with PGO](https://github.com/rust-lang/rust/pull/163649)

#### Library
* [add `mul_add_relaxed` methods for floating-point types](https://github.com/rust-lang/rust/pull/151793)
* [add `std::fs::{Home|Media}Dirs`](https://github.com/rust-lang/rust/pull/158936)
* [expose `Rc::is_unique`](https://github.com/rust-lang/rust/pull/163489)
* [stabilize `CStr::display`](https://github.com/rust-lang/rust/pull/163711)
* [stabilize `debug_closure_helpers`](https://github.com/rust-lang/rust/pull/146099)

#### Cargo
* [add new peak memory table to cargo timings enabled via `-Zmem-stats`](https://github.com/rust-lang/cargo/pull/17531)
* [`config`: Proper dotted tuple support with legacy fallback](https://github.com/rust-lang/cargo/pull/17536)
* [git: default to net.git-fetch-with-cli if git is present](https://github.com/rust-lang/cargo/pull/17329)
* [improved testsuite file permissions cleanup](https://github.com/rust-lang/cargo/pull/17547)
* [`lint`: Making the lint name a terminal hyperlink to docs](https://github.com/rust-lang/cargo/pull/17538)
* [`trim-paths`: stabilize `profile.trim-paths`](https://github.com/rust-lang/cargo/pull/17488)
* [use trusted publishing for Cargo crates](https://github.com/rust-lang/cargo/pull/17426)

#### Rustdoc
* [Correctly handle `rustc_allow_incoherent_impl` on primitive methods](https://github.com/rust-lang/rust/pull/163360)
* [Correctly link to (imported) `enum` variants with "jump to def"](https://github.com/rust-lang/rust/pull/163682)
* [Fix how `Deref` items are handled](https://github.com/rust-lang/rust/pull/160915)

#### Rustfmt
* [`items`: format comments after where using `clause_shape` budget](https://github.com/rust-lang/rustfmt/pull/7156)
* [use saturating arithmetics for `adjust_max_width`](https://github.com/rust-lang/rustfmt/pull/7152)

#### Clippy
* [`manual_range_patterns`: support char and byte literal](https://github.com/rust-lang/rust-clippy/pull/17817)
* [`let_unit_value` bail out if initializer is cfg-dependent](https://github.com/rust-lang/rust-clippy/pull/17701)
* [extend `needless_borrowed_reference` to lint mutable ref patterns](https://github.com/rust-lang/rust-clippy/pull/17012)
* [fix exponential-time performance bug in `has_non_owning_mutable_access_inner`](https://github.com/rust-lang/rust-clippy/pull/17807)
* [improve `items_after_test_module`: don't let derive expansions hide trailing items](https://github.com/rust-lang/rust-clippy/pull/17816)
* [new lint: `unnecessary_as_slice`](https://github.com/rust-lang/rust-clippy/pull/16953)
* [optimize msrv calls (again)](https://github.com/rust-lang/rust-clippy/pull/17355)

#### Rust-Analyzer
* [complete 'let' 'letm' in arm expr and closure expr](https://github.com/rust-lang/rust-analyzer/pull/23468)
* [complete turbofish when fn can't infer param](https://github.com/rust-lang/rust-analyzer/pull/23473)
* [do not suggest arg-list in expected callable arg](https://github.com/rust-lang/rust-analyzer/pull/23452)
* [fix `unicode-ident`, take 2](https://github.com/rust-lang/rust-analyzer/pull/23449)
* [add missing HIR database when running unresolved-references](https://github.com/rust-lang/rust-analyzer/pull/23444)
* [complete let in macro when expand at macro stmts](https://github.com/rust-lang/rust-analyzer/pull/22982)
* [don't panic on malformed let-pattern with mismatched or-arm arities](https://github.com/rust-lang/rust-analyzer/pull/23422)
* [generate variant for self in impl](https://github.com/rust-lang/rust-analyzer/pull/23462)
* [improve in-block heuristic check in nested ambiguous](https://github.com/rust-lang/rust-analyzer/pull/23467)
* [name-match ignore leading tailing underscore](https://github.com/rust-lang/rust-analyzer/pull/23456)
* [transform usage path when extract trait to module](https://github.com/rust-lang/rust-analyzer/pull/23441)
* [fixed Implement `opaques_with_sub_unified_hidden_type` for the next-sol…](https://github.com/rust-lang/rust-analyzer/pull/23061)

### Rust Compiler Performance Triage

A relatively quiet week, but a very positive one nonetheless.
Highlights are a 3.1% improvement in rustdoc speed from not using the metadata based crate_hash for rustdoc runs,
a 0.5% improvement from a new fast path in the trait solver,
and a 0.4% improvement from a cache for the parents of `SpanData`.

Triage done by **@JonathanBrouwer**.
Revision range: [c1070d69..cc9a14f7](https://perf.rust-lang.org/?start=c1070d69382b8d2f2eb65119c738a77d9e324c9e&end=cc9a14f721fac5226338c61dcec7d5ab785bde82&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.6%  | [0.4%, 1.0%]    | 12    |
| Regressions ❌ <br /> (secondary)  | 0.4%  | [0.1%, 0.9%]    | 26    |
| Improvements ✅ <br /> (primary)   | -1.1% | [-6.2%, -0.2%]  | 212   |
| Improvements ✅ <br /> (secondary) | -1.9% | [-15.7%, -0.1%] | 208   |
| All ❌✅ (primary)                 | -1.0% | [-6.2%, 1.0%]   | 224   |


4 Regressions, 3 Improvements, 1 Mixed; 5 of them in rollups
33 artifact comparisons made in total

[Full report here](https://github.com/JonathanBrouwer/rustc-perf/blob/b65c7aa3a161fb8590ed262043e822d278f5a7f5/triage/2026/2026-10-05.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/main)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Change default branch to main](https://github.com/rust-lang/rfcs/pull/4012)
* [`f16b` type](https://github.com/rust-lang/rfcs/pull/3983)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Make `std::fs::{File, ReadDir, DirEntry}` always `needs_drop` even when unsupported.](https://github.com/rust-lang/rust/pull/162444)
* [[rustdoc] Add tabs to settings popover](https://github.com/rust-lang/rust/pull/163224)
* [rustc: Stabilize the WebAssembly `wide-arithmetic` feature](https://github.com/rust-lang/rust/pull/160877)
* [Error on non-literal expressions in doc attributes on macro calls](https://github.com/rust-lang/rust/pull/160904)
* [stabilize ptr_cast_slice](https://github.com/rust-lang/rust/pull/162927)
* [Add extra types to `VaArgSafe`](https://github.com/rust-lang/rust/pull/162858)
* [Reject cfg on expressions that cannot be safely removed](https://github.com/rust-lang/rust/pull/159580)
* [fn_addr_eq: we actually can guarantee basically nothing](https://github.com/rust-lang/rust/pull/162140)
* [Stop using dlltool for generating import libraries on MinGW](https://github.com/rust-lang/rust/pull/157712)
* [Stabilize `ptr::try_cast_aligned`](https://github.com/rust-lang/rust/pull/154170)
* [disposition: close] [1.99 beta crater regression: overflow evaluating the requirement](https://github.com/rust-lang/rust/issues/161916)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Move `hir::Param`s from `Body` of functions to `FnDecl`.](https://github.com/rust-lang/compiler-team/issues/1044)
* [MCP: Add -Zasync-panic for binary size](https://github.com/rust-lang/compiler-team/issues/1016)
* [Upstreaming BorrowSanitizer Experimentally in Nightly Rust](https://github.com/rust-lang/compiler-team/issues/1041)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Guarantee that the never type `!` is zero-sized and 1-aligned.](https://github.com/rust-lang/reference/pull/2309)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Add `required-targets` for workspace package selection](https://github.com/rust-lang/rfcs/pull/4013)
* [RFC: unsafe `global_asm`](https://github.com/rust-lang/rfcs/pull/4014)
* [Cromulent `Copy` closure captures](https://github.com/rust-lang/rfcs/pull/4011)
* [RFC for limited crates.io self-service version deletion](https://github.com/rust-lang/rfcs/pull/4015)

<!-- Call for Testing Message (post in GH `issue` and remove `call-for-testing` label) -->
This RFC will appear in the **Call for Testing** section of the next issue (#) of This Week in Rust (TWiR).
You may remove the `call-for-testing` label.  Please feel free to leave the `call-for-testing` label in place if you would like this RFC to appear again in another issue of TWiR.


## Upcoming Events

Rusty Events between 2026-10-07 - 2026-11-04 🦀

### Virtual
* 2026-10-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcnbkb/)
* 2026-10-08 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/315907995/)
* 2026-10-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619617/)
* 2026-10-10 | Virtual (Gdansk, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto)
    * [**[BEZPŁATNIE] Programowanie w języku Rust**](https://www.meetup.com/stacja-it-trojmiasto/events/316381946/)
* 2026-10-10 | Hybrid (Kuala Lumpur, Malaysia) | [Rust Malaysia Meetup](https://discord.gg/Uz88bnZA3B)
    * [**Rust Meetup October 2026**](https://forms.gle/721DxqrPeHXY6omP9)
* 2026-10-11 | Virtual (Bengaluru, India) | [Embedded Rust Discord](https://discord.com/invite/pvYY69PvyS)
    * [**Silicon Sundays 4**](https://discord.gg/t9Cb2gjjq7?event=1553055353149718588)
* 2026-10-13 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254772/)
* 2026-10-14 - 2026-10-17 | Hybrid (Barcelona, ES) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2026**](https://eurorust.eu/)
* 2026-10-18 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/316563013/)
* 2026-10-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/fhvsztyjcnbbc/)
* 2026-10-21 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Disposable Agent Sandboxes in Rust**](https://www.meetup.com/vancouver-rust/events/315210233/)
* 2026-10-22 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/316272609/)
* 2026-10-22 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Rust and the GPU from Native to Web: An Introduction to `wgpu`**](https://luma.com/k1978ath)
* 2026-10-26 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**No Python was harmed: teaching a tiny MCU to learn as it goes**](https://luma.com/1byfc495)
* 2026-10-27 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Fourth Tuesday Rust Bookclub**](https://www.meetup.com/dallasrust/events/310254771/)
* 2026-10-27 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: Reasoning with Async Rust**](https://www.meetup.com/women-in-rust/events/315297195/)
* 2026-10-29 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Creating a hexadecimal editor in Rust**](https://luma.com/q8i3385k)
* 2026-11-01 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/316331277/)
* 2026-11-03 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315773682/)
* 2026-11-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcpbgb/)

### Asia
* 2026-10-09 | Hybrid (Kuala Lumpur, MY) | [Rust Malaysia Meetup](https://discord.gg/Uz88bnZA3B)
    * [**Rust Meetup August 2026**](https://forms.gle/721DxqrPeHXY6omP9)
* 2026-11-03 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**In person Rust November 2026 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/316440864/)

### Europe
* 2026-10-08 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/316564477/)
* 2026-10-08 | Geneva, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-10-14 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust)
    * [**22nd bcnrust session**](https://www.meetup.com/bcnrust/events/316316234/)
* 2026-10-14 - 2026-10-17 | Hybrid (Barcelona, ES) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2026**](https://eurorust.eu/)
* 2026-10-20 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Creating a realtime web Multiuser Dungeon game with Dioxus**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816496/)
* 2026-10-22 | Karlsruhe, DE | [Rust Hack & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe/events/)
    * [**Karlsruhe Rust Hack and Learn Meetup bei BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/316799366/)
* 2026-10-22 | Toulouse, FR | [Rust Toulouse](https://www.meetup.com/rust-community-toulouse)
    * [**Rust Toulouse Meetup - Rust & Python interoperability**](https://www.meetup.com/rust-community-toulouse/events/316880185/)
* 2026-10-27 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Hack Night: Rust meets AI**](https://www.meetup.com/rust-aarhus/events/316796329/)
* 2026-10-31 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #31**](https://www.meetup.com/stockholm-rust/events/316721062/)
* 2026-11-01 - 2026-11-03 | Italy, IN | [RustLab](https://rustlab.it/)
    * [**RustLab - The International Conference on Rust in Italy**](https://rustlab.it/schedule)

### North America
* 2026-10-08 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Lightning Talks N' Chill**](https://www.meetup.com/utah-rust/events/316708351/)
* 2026-10-08 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Zero Knowledge Proofs & GPUs in Rendering**](https://www.meetup.com/rust-nyc/events/316698929/)
* 2026-10-08 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust October Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/316319730/)
* 2026-10-10 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Back Bay Rust Lunch, Oct 10**](https://www.meetup.com/bostonrust/events/316378823/)
* 2026-10-14 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA October: AI & Rust w/ Oxen.AI & Origin Lab!**](https://www.meetup.com/rust-los-angeles/events/315795432/)
* 2026-10-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/315783988/)
* 2026-10-21 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Disposable Agent Sandboxes in Rust**](https://www.meetup.com/vancouver-rust/events/315210233/)
* 2026-10-21 | San Francisco, CA, US | [Bay Area Rust](https://luma.com/bayarearust)
    * [**Bay Area Rust - Embedded Meetup**](https://luma.com/ur4pm34i)
* 2026-10-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/316655403/)

### South America
* 2026-10-08 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**WebApp Ergonomics y Secretos Distribuidos.**](https://www.meetup.com/rust-argentina/events/316664266/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1wzctie/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> There ain't no rules here in Quote of the Week - it's survival of the wittest

– [Simon Buchan on rust-users](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1814?u=llogiq)

Thanks to [Jonas Fassbender](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1815) for the suggestion!

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1x0kagz/this_week_in_rust_672/)</small>
