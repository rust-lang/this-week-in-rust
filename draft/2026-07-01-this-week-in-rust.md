Title: This Week in Rust 658
Number: 658
Date: 2026-07-01
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

### Foundation
* [Rust Foundation Trusted Training Program Launches, Giving Learners a Mark of Quality to Trust](https://rustfoundation.org/media/rust-foundation-trusted-training-program-launches-giving-learners-a-mark-of-quality-to-trust/)


### Newsletters
* [Scientific Computing in Rust #19 (June 2026)](https://scientificcomputing.rs/monthly/2026-06)


### Project/Tooling Updates
* [Introducing Test That!: A rich test assertion library for Rust from the original author of GoogleTest Rust](https://hovinen.me/announcements/2026/06/24/introducing-test-that.html)
* [Inside RSSH: one Rust crate, three binaries, and the Tauri lessons along the way](https://github.com/shihuili1218/rssh/blob/main/docs/article_arch_en.md)
* [Rustty 1.38 – accessibility & keyboard nav](https://github.com/Aleixenandros/Rustty/releases/tag/v1.38.0)
* [GuardianDB 0.17.0: Secure namespaces, Iroh 1.0, and the arrival of the ODM](https://www.willsearch.com.br/blog/2026/06/25/guardiandb-0-17-0-secure-namespaces-iroh-1-0-and-the-arrival-of-the-odm/)
* [Building a real-time voice-agent runtime in Rust: no GIL, one binary, 2,000 calls a box](https://dev.to/iam_suriyan_b9078a5b3a553/building-a-real-time-voice-agent-runtime-in-rust-no-gil-one-binary-2000-calls-a-box-12ko)
* [AimDB: Bring Your Own Connector](https://aimdb.dev/blog/aimdb-bring-your-own-connector)
* [kache 0.8.0: zero-copy restores on Windows (ReFS)](https://github.com/kunobi-ninja/kache/releases/tag/v0.8.0)
* [Warbell — a castle-defense action-RPG built with Bevy 0.19](https://miskibin.github.io/warbell/)


[I built a macOS FTP client entirely in Rust - no Electron, no webview](https://dev.to/gregorymc86/i-built-a-macos-ftp-client-entirely-in-rust-no-electron-no-webview-2a8i)

### Observations/Thoughts
* [The Unglamorous Side of Rust Web Development](https://blog.jetbrains.com/rust/2026/06/25/rust-web-development-2026/)
* [How I Found Out 52% of My Knowledge Graph Was Duplicates (and What I Did About It)](https://dev.to/ernesto_arias_148b35bc25d/-how-i-found-out-52-of-my-knowledge-graph-was-duplicates-and-what-i-did-about-it-3coh)
* [A Novel Approach to Rust Error Handling](https://jtjlehi.github.io/2026/06/25/novel-rust-error-handling.html)
* [We put a Redis server inside our runtime](https://encore.dev/blog/redis-runtime)
* [High-performance Rust: Understanding and eliminating memory fragmentation](https://kerkour.com/rust-high-performance-memory-fragmentation-allocations)
* [AI and worktrees are filling our disks: kache storage, measured](https://kunobi.ninja/blog/kache-storage-worktrees)


[Designing a cross-platform terminal memory visualizer in Rust](https://dev.to/sicklefire/designing-a-cross-platform-terminal-memory-visualizer-in-rust-2365)

[Your Rust Service Isn't Leaking — It Could Be the Allocator](https://pranitha.dev/posts/rust-and-memory-allocators)

### Rust Walkthroughs
* [Measure, Don't Guess: Building viser, a Content-Adaptive Video Encoding Optimizer in Rust](https://medium.com/@vbasky/measure-dont-guess-building-viser-a-content-adaptive-video-encoding-optimizer-in-rust-7675edd6943a)
* [Learn SQL and SQLx by Building a Book Library CLI in Rust](https://blog.sheerluck.dev/posts/learn-sql-and-sqlx-by-building-a-book-library-cli-in-rust/)
* [series] [Reasoning About Async Rust with State Machines](https://aibodh.com/posts/async-rust-chapter-2-what-async-fn-compiles-into/)
* [The C to Rust Migration Book](https://mainmatter.com/c-to-rust-migration-book/)


### Research

### Miscellaneous

## Crate of the Week

This week's crate is [deconvolution](https://github.com/pbkx/deconvolution), a image deconvolution and restoration library.

Thanks to [pbkx](https://users.rust-lang.org/t/crate-of-the-week/2704/1621) for the self-suggestion!

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

[multicalc - good first issues](https://github.com/kmolan/multicalc-rust/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->
* [AimDB - Add minimal example: hello-single-latest](https://github.com/aimdb-dev/aimdb/issues/93)
* [AimDB - Wire `.transform()` and `.transform_join()` into stage profiling](https://github.com/aimdb-dev/aimdb/issues/109)
* [edid-info - Increase test coverage with real EDID data](https://github.com/SzilvasiPeter/edid-info/issues/1)
* [edid-info - Finalize CTA-861 extension implementation](https://github.com/SzilvasiPeter/edid-info/issues/2)
* [edid-info - Support additional EDID extension block types](https://github.com/SzilvasiPeter/edid-info/issues/3)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

426 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-06-23..2026-06-30

#### Compiler
* [drop the full-crate AST walk in `check_unused`](https://github.com/rust-lang/rust/pull/157996)
* [make `stable_crate_ids` reads lock-free after crate loading](https://github.com/rust-lang/rust/pull/158185)
* [rework lint pass running](https://github.com/rust-lang/rust/pull/158239)
* [simplify some `proc_macro` things](https://github.com/rust-lang/rust/pull/157271)

#### Library
* [add `io::ErrorKind::TooManyOpenFiles`](https://github.com/rust-lang/rust/pull/158326)
* [expand `OptionFlatten`'s iterator methods](https://github.com/rust-lang/rust/pull/153097)
* [move `std::io::Error` into `core`](https://github.com/rust-lang/rust/pull/155625)
* [optimize network address parser](https://github.com/rust-lang/rust/pull/158053)

#### Cargo
* [add `-Zhint-msrv` flag](https://github.com/rust-lang/cargo/pull/17106)

#### Clippy
* [`filter_map_next`: clean-up, overhaul suggestions](https://github.com/rust-lang/rust-clippy/pull/17237)
* [`chunks_exact_to_as_chunks`: Prevent syntactically invalid suggestions](https://github.com/rust-lang/rust-clippy/pull/17318)
* [`chunks_exact_to_as_chunks`: Use correct method name in message](https://github.com/rust-lang/rust-clippy/pull/17317)
* [`chunks_exact_to_as_chunks`: Pick iter method depending on mut-ness](https://github.com/rust-lang/rust-clippy/pull/17316)
* [`non_ascii_literal`, `invisible_characters`: don't suggest a fix on raw strings](https://github.com/rust-lang/rust-clippy/pull/17302)
* [create a single `ConstEvalCtxt` in `expr_eagerness`](https://github.com/rust-lang/rust-clippy/pull/17228)
* [detect new range types in `higher::Range`](https://github.com/rust-lang/rust-clippy/pull/17299)
* [do not trigger `manual_option_zip` when map receiver is a lazy evaluated expression](https://github.com/rust-lang/rust-clippy/pull/17270)
* [enhance `needless_late_init` to cover grouped assignments](https://github.com/rust-lang/rust-clippy/pull/16746)
* [fix: `borrow_as_ptr` is triggered on generated code](https://github.com/rust-lang/rust-clippy/pull/17257)

#### Rust-Analyzer
* [add diagnostic for E0596](https://github.com/rust-lang/rust-analyzer/pull/22466)
* [add fixes add '.await' for `type_mismatch`](https://github.com/rust-lang/rust-analyzer/pull/22645)
* [crash on lowering consts with associated types](https://github.com/rust-lang/rust-analyzer/pull/22646)
* [crash when hovering on anonymous consts](https://github.com/rust-lang/rust-analyzer/pull/22640)
* [only run `Drop::drop` when implemented](https://github.com/rust-lang/rust-analyzer/pull/22582)
* [mark `inline_convert_while_ascii()` as `unsafe`](https://github.com/rust-lang/rust-analyzer/pull/22633)
* [switch out lsp-types for gen-lsp-types](https://github.com/rust-lang/rust-analyzer/pull/22115)

### Rust Compiler Performance Triage

Overall, the week was fairly neutral, with no meaningful shift on most benchmarks on any of our statistics.

Triage done by **@simulacrum**.
Revision range: [8b6558a0..7dc2c162](https://perf.rust-lang.org/?start=8b6558a02b2774acfb25cf15e199467c37ba7490&end=7dc2c162b9c197aaa76a6f9e7534569537830a01&absolute=false&stat=instructions%3Au)

2 Regressions, 1 Improvement, 7 Mixed; 5 of them in rollups
34 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2026/2026-06-29.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Tracking Issue for LocalKey/Cell::update](https://github.com/rust-lang/rust/issues/143989)
* [Tracking Issue for `{str, [T], Path}::trim_prefix` and `{str, [T]}::trim_suffix`](https://github.com/rust-lang/rust/issues/142312)
* [Stabilize c-variadic function definitions](https://github.com/rust-lang/rust/pull/155697)
* [Tracking Issue for layout information behind pointers](https://github.com/rust-lang/rust/issues/69835)
* [Fix feature gate for `repr(simd)`](https://github.com/rust-lang/rust/pull/158523)
* [reat no_mangle_generic_items as hard error instead of lint warning](https://github.com/rust-lang/rust/pull/154585)
* [Lint against invalid POSIX symbol definitions](https://github.com/rust-lang/rust/pull/158522)
* [Fix `overflowing_literals` lint with repeated negation](https://github.com/rust-lang/rust/pull/158302)
* [stabilize `extern "custom"`](https://github.com/rust-lang/rust/pull/158504)
* [Don't escape U+FF9E and U+FF9F in `escape_debug_ext`](https://github.com/rust-lang/rust/pull/158057)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Decouple `BackendRepr` from ABI alignment](https://github.com/rust-lang/compiler-team/issues/1007)
* [MCP: Stabilization strategy for rustc parallel frontend](https://github.com/rust-lang/compiler-team/issues/1005)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Fields must fit in the type, even for repr(Rust)](https://github.com/rust-lang/reference/pull/2166)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [RFC: Associated const underscore](https://github.com/rust-lang/rfcs/pull/3527)
* [Add `extern "custom"`](https://github.com/rust-lang/rfcs/pull/3980)

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Opsem extension proposal: atomic volatile accesses](https://github.com/rust-lang/unsafe-code-guidelines/issues/615)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Method chain as item](https://github.com/rust-lang/rfcs/pull/3977)
* [Add `extern "custom"`](https://github.com/rust-lang/rfcs/pull/3980)

## Upcoming Events

Rusty Events between 2026-07-01 - 2026-07-29 🦀

### Virtual
* 2026-07-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/315210366/)
* 2026-07-02 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455932/)
* 2026-07-02 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Learning Game Development the Hard Way with Rust and Bevy**](https://www.meetup.com/charlottesville-rust-meetup/events/315211402/)
* 2026-07-02 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345243/)
* 2026-07-04 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-07-05 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/314095287/)
* 2026-07-07 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315060981/)
* 2026-07-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254778/)
* 2026-07-15 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314233743/)
* 2026-07-16 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**July, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314520812/)
* 2026-07-16 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/312045926/)
* 2026-07-19 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/314329045/)
* 2026-07-21 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Learning Rust as First Programming Language**](https://www.meetup.com/women-in-rust/events/315102297/)
* 2026-07-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/315279653/)
* 2026-07-28 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254777/)

### Asia
* 2026-07-18 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**July 2026 Rustacean Meetup**](https://hasgeek.com/rustbangalore/july-2026-rustacean-meetup/)

### Europe
* 2026-07-01 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn)
    * [**Rust in July: Vecs and Strings and Slices, Oh My!**](https://www.meetup.com/rustcologne/events/315404678/)
* 2026-07-01 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester June Talks**](https://www.meetup.com/rust-manchester/events/315200163/)
* 2026-07-01 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Building a file system from scratch**](https://www.meetup.com/oxford-rust-meetup-group/events/315409335/)
* 2026-07-02 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Bevy, Bits, & Cats (Rust July Talks)**](https://www.meetup.com/rust-and-friends/events/314941098/)
* 2026-07-02 | Enschede, NL | [Baseflow Tech Meetups](https://www.meetup.com/dutch-rust-meetup)
    * [**AI Summit**](https://www.meetup.com/baseflow-tech-meetups/events/315099547/)
* 2026-07-08 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Join us live and INPERSON for Rust 262**](https://www.meetup.com/rust-dublin/events/315150327/)
* 2026-07-09 | Switzerland, CH | [PostTenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-07-21 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Supercharge Rust funcs with implicit arguments and context-generic programming**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816470/)
* 2026-07-23 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/315484101/)
* 2026-07-23 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group)
    * [**Rama modular service framework for Rust**](https://www.meetup.com/london-rust-project-group/events/315366453/)
* 2026-07-23 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #87**](https://www.meetup.com/rust-paris/events/315309633/)

### North America
* 2026-07-02 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Git is easy?**](https://www.meetup.com/stl-rust/events/315103359/)
* 2026-07-04 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Boston University Rust Lunch, July 4**](https://www.meetup.com/bostonrust/events/315225861/)
* 2026-07-09 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Utah Rust July Meetup**](https://www.meetup.com/utah-rust/events/314696647/)
* 2026-07-11 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**MIT Rust Lunch, July 11**](https://www.meetup.com/bostonrust/events/315225865/)
* 2026-07-15 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314233743/)
* 2026-07-16 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**July, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314520812/)
* 2026-07-18 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**North End Rust Lunch, July 18**](https://www.meetup.com/bostonrust/events/315225872/)
* 2026-07-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/314997214/)
* 2026-07-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyjckbdc/)
* 2026-07-22 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust in Distributed Systems with Flight Science!**](https://www.meetup.com/rust-los-angeles/events/315376271/)
* 2026-07-25 | Brooklyn, NY, US | [Flower](https://flowercomputer.com/)
    * [**BOG-A-THON 2**](https://partiful.com/e/Vq9fyDNCMSO7ia4ulK5b)

### Oceania
* 2026-07-21 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**July Meetup**](https://www.meetup.com/rust-canberra/events/315307280/)
* 2026-07-23 | Perth, AU | [Rust Perth Meetup Group](https://www.meetup.com/perth-rust-meetup-group)
    * [**Rust Perth: July Meetup!**](https://www.meetup.com/perth-rust-meetup-group/events/315451138/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> I *do* rather hope anyone using `-Zllvm-target-features` or any stabilized form thereof would know that they are getting a conversation with the dragon directly and they should mind their words carefully if they do not wish to be barbecued by it and served over a nice plate of iron filings.

– [workingjubilee on rust zulip](https://rust-lang.zulipchat.com/#narrow/channel/233931-t-compiler.2Fmajor-changes/topic/Add.20.60-Zllvm-target-feature.60.20target.20.2Amodif.E2.80.A6.20compiler-team.23994/near/606147265)

Thanks to [Tomáš Šedovič](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1784) for the suggestion!

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

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
