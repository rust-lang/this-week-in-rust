Title: This Week in Rust 652
Number: 652
Date: 2026-05-20
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

* [Project goals update — April 2026 (end of 2025H2)](https://blog.rust-lang.org/2026/05/18/project-goals-2026-04/)
* [Program management update — April 2026](https://blog.rust-lang.org/inside-rust/2026/05/13/program-management-update--april-2026/)

### Foundation

### Newsletters

* [This Month in Rust OSDev: April 2026](https://rust-osdev.com/this-month/2026-04/)

### Project/Tooling Updates

* [Tonic is joining the gRPC project](https://luciofranco.com/blog/tonic-joins-grpc/)
* [Toasty 0.6.0 - What is new?](https://tokio.rs/blog/2026-05-15-announcing-toasty-0-6-0)
* [ex_ratatui](https://hexdocs.pm/ex_ratatui): Elixir bindings for ratatui via Rustler NIFs.
* [OmniScope](https://medium.com/@jinhopers/in-depth-llvm-ir-how-omniscope-tracks-ownership-across-languages-2919e418ca61): A Cross-Language LLVM IR Static Analyzer Targeting Unsafe/FFI Boundariesby
* [citum](https://citum.org/): a new Rust citation processor and associated tools.
* [cargo-crap: Finding Untested Complexity in AI-Generated Rust Code](https://minikin.me/blog/cargo-crap)
* [What the Graph Owes](https://aimdb.dev/blog/graph-owes): Connectors That Drive Outputs
* [swpui: a TUI for case-aware search and replace](https://beeb.li/blog/introducing-swpui)
* [kache 0.3.0: zero-copy efficient worktree compilation](https://kunobi.ninja/blog/kache-update)


### Observations/Thoughts

* [Scaling Rust codebases: Lessons learned organizing large projects and managing errors](https://kerkour.com/rust-organize-large-projects-code-error-handling)
* [Migrating from Go to Rust](https://corrode.dev/learn/migration-guides/go-to-rust/)
* [Why I built wrkflw](https://blog.gokuls.in/posts/why-i-built-wrkflw.html)
* [video] [Rust's God Mode](https://www.youtube.com/watch?v=VIsKIzFz_zA)
* [video] [How Rust engineered the perfect async runtime](https://www.youtube.com/watch?v=FUg1y-yv6cs)

### Rust Walkthroughs

* [5× faster fast_blur in image-rs](https://apas.tel/blog/optimizing-image-rs-blur)
* [Finding the Time Part 2 - Rust Async and the Arm Generic Timer](https://thejpster.org.uk/blog/blog-2026-05-17/)
* [Parsing Godot .tres files and walking the resource graph](https://assethoard.com/blog/parsing-godot-tres-files)
* [Rust x GBA: Setup and Pixels](https://jonahnestrick.com/blog/rust-gba-tutorial-1/)
* [Learn Rust Lifetimes by Building a Generic LRU Cache](https://blog.sheerluck.dev/posts/learn-rust-lifetimes-by-building-a-lru-cache/)
* [How to benchmark Rust code with Gungraun](https://bencher.dev/learn/benchmarking/rust/gungraun/)
* [Book: An Introduction to Programming, using ECS & EBP in Rust](https://root-11.github.io/intro-book/)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [cargo-crap](https://github.com/minikin/cargo-crap), a cargo subcommand to calculate the Change Risk Anti-Patterns metric for a crate.

Despite a lamentable lack of suggestions, llogiq is pleased with his choice.

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

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [**Scientific Computing in Rust 2026**](https://scientificcomputing.rs/2026/submit-talk)| 2026-06-05 | Virtual | 2026-07-08 - 2026-07-10

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

369 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-05-12..2026-05-19

#### Compiler
* [add Swift function call ABI](https://github.com/rust-lang/rust/pull/155815)
* [implement pinned drop sugar](https://github.com/rust-lang/rust/pull/156452)

#### Library
* [`map_try_insert` changes](https://github.com/rust-lang/rust/pull/155360)
* [implement `OsStr::split_at`](https://github.com/rust-lang/rust/pull/156444)
* [implement `into_array` for `Vec<T>`](https://github.com/rust-lang/rust/pull/156234)
* [move `std::io::Cursor` to `core::io`](https://github.com/rust-lang/rust/pull/156428)
* [move `std::io::util` to `core::io`](https://github.com/rust-lang/rust/pull/156431)
* [widen the result of `widening_mul`](https://github.com/rust-lang/rust/pull/156644)

#### Cargo
* [`clean`: respect `build.target` config for `clean -p`](https://github.com/rust-lang/cargo/pull/16988)
* [`diag`: Consolidate verify/run diagnostics passes](https://github.com/rust-lang/cargo/pull/16989)
* [`diag`: Report deferred diagnostics like other diagnostics](https://github.com/rust-lang/cargo/pull/16994)
* [`diag`: Pull in the parse pass](https://github.com/rust-lang/cargo/pull/17008)
* [`lints`: Avoid compiling where possible](https://github.com/rust-lang/cargo/pull/17007)
* [drop `-Zunstable-options` for `rustdoc --emit`](https://github.com/rust-lang/cargo/pull/17002)

#### Rustdoc
* [stabilize `--emit` flag](https://github.com/rust-lang/rust/pull/146220)
* [correctly handle associated items in rustdoc macro expansion](https://github.com/rust-lang/rust/pull/156587)
* [correctness & perf improvements to link-to-definition](https://github.com/rust-lang/rust/pull/156413)
* [properly support macros with multiple kinds](https://github.com/rust-lang/rust/pull/152449)

#### Clippy
* [fix `duration_suboptimal_units` for small literals](https://github.com/rust-lang/rust-clippy/pull/16922)
* [fix arithmetic side effects false positive](https://github.com/rust-lang/rust-clippy/pull/17011)

#### Rust-Analyzer
* [add diagnostic for E0029](https://github.com/rust-lang/rust-analyzer/pull/22347)
* [add diagnostic for E0614](https://github.com/rust-lang/rust-analyzer/pull/22380)
* [add diagnostic for E0638](https://github.com/rust-lang/rust-analyzer/pull/22355)
* [add handler for E0040](https://github.com/rust-lang/rust-analyzer/pull/22378)
* [encode the name instead of index in `EnumVariantId`](https://github.com/rust-lang/rust-analyzer/pull/22329)
* [fix assist `qualify_path` loses path segment](https://github.com/rust-lang/rust-analyzer/pull/22354)
* [add param on result methods for `replace_method_eager_lazy`](https://github.com/rust-lang/rust-analyzer/pull/22335)
* [complete `ref_match` in macro](https://github.com/rust-lang/rust-analyzer/pull/22399)
* [fully support pattern types](https://github.com/rust-lang/rust-analyzer/pull/22368)
* [handle usages in macro for `extract_function`](https://github.com/rust-lang/rust-analyzer/pull/22344)
* [no complete module colons before exists colons](https://github.com/rust-lang/rust-analyzer/pull/22386)
* [no lint unsized adt `self_ty` missing bounded assoc](https://github.com/rust-lang/rust-analyzer/pull/22363)
* [not complete same name inherent deref methods](https://github.com/rust-lang/rust-analyzer/pull/22376)
* [only ref match non-unknown value items](https://github.com/rust-lang/rust-analyzer/pull/22367)
* [show Run lens for fn main in bench targets](https://github.com/rust-lang/rust-analyzer/pull/22357)
* [handle `TyKind::{Pat,UnsafeBinder}` in `has_drop_glue`](https://github.com/rust-lang/rust-analyzer/pull/22384)
* [implement `pattern_type` macro](https://github.com/rust-lang/rust-analyzer/pull/22082)
* [method-resolution: emit error for method calls with illegal Sized bound](https://github.com/rust-lang/rust-analyzer/pull/22372)
* [migrate `inline_call` assist to SyntaxFactory](https://github.com/rust-lang/rust-analyzer/pull/22352)
* [perf: provide access to `RootDatabase`'s `LineIndex` for the proc macro protocol](https://github.com/rust-lang/rust-analyzer/pull/22191)
* [show `const` in the signature help if applicable](https://github.com/rust-lang/rust-analyzer/pull/22358)
* [show `unsafe` in the signature help if applicable](https://github.com/rust-lang/rust-analyzer/pull/22381)

### Rust Compiler Performance Triage

<!-- Perf results go here -->


### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Cargo RFC for min publish age](https://github.com/rust-lang/rfcs/pull/3923)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Removing the unstable ptx linker flavor](https://github.com/rust-lang/compiler-team/issues/990)
* [Create a new Tier 3 target: `powerpc64le-unknown-none`](https://github.com/rust-lang/compiler-team/issues/988)
* [Optimize `repr(Rust)` enums by omitting tags in more cases involving uninhabited variants.](https://github.com/rust-lang/compiler-team/issues/922)
* [Proposal for a dedicated test suite for the parallel frontend](https://github.com/rust-lang/compiler-team/issues/906)
* [Promote tier 3 riscv32 ESP-IDF targets to tier 2](https://github.com/rust-lang/compiler-team/issues/864)
* [Proposal for Adapt Stack Protector for Rust](https://github.com/rust-lang/compiler-team/issues/841)

*No Items entered Final Comment Period this week for
[Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Documentation interpolation](https://github.com/rust-lang/rfcs/pull/3962)

## Upcoming Events

Rusty Events between 2026-05-20 - 2026-06-17 🦀

### Virtual
* 2026-05-20 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Mouse Control with Rust**](https://www.meetup.com/vancouver-rust/events/313572925/)
* 2026-05-20 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/548kbqhl)
* 2026-05-21 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**May, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873203/)
* 2026-05-21 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455929/)
* 2026-05-21 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Part #4 - Capsule coding in QEMU!**](https://www.meetup.com/charlottesville-rust-meetup/events/314477948/)
* 2026-05-26 | Virtual (Cardiff, GB) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Hybrid event with Rust Dortmund!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/314820642/)
* 2026-05-26 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254781/)
* 2026-05-26 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Seeing Into Your Code - A Practical Guide to Tracing in Rust**](https://www.meetup.com/women-in-rust/events/313506048/)
* 2026-05-27 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/9v7hv2g1)
* 2026-06-02 | Virtual | [libp2p Events](https://luma.com/libp2p)
    * [**rust-libp2p Open Maintainers Call**](https://luma.com/ukfh0mcf)
* 2026-06-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/314691782/)
* 2026-06-04 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455930/)
* 2026-06-04 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345241/)
* 2026-06-07 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/314095285/)
* 2026-06-09 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254780/)
* 2026-06-10 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/3bcnx1jb)
* 2026-06-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/rdhhptyjcjbvb/)
* 2026-06-02 | Virtual | [libp2p Events](https://luma.com/libp2p)
    * [**rust-libp2p Open Maintainers Call**](https://luma.com/pegz5x4h)
* 2026-06-17 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/ekws5nr4)
* 2026-06-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/314000478/)

### Asia
* 2026-06-02 | Beijing, CN | [Voice AI and Rust Meetup (Rust for AI, lowcoderust.com)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**AI Agents and Open Source LLM (Call for Speakers)**](https://www.meetup.com/wasm-rust-meetup/events/314750465/)

### Europe
* 2026-05-18 - 2026-05-23 | Utrecht, NL | [RustWeek 2026](https://2026.rustweek.org/)
    * [**RustWeek 2026**](https://2026.rustweek.org/)
* 2026-05-21 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam)
    * [**RustWeek Hackathon**](https://www.meetup.com/rust-nederland/events/314301699/)
* 2026-05-22 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam/events/)
    * [**Walking Tour around Utrecht**](https://www.meetup.com/rust-nederland/events/314770275/)
* 2026-05-22 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam)
    * [**Bike tour around Utrecht**](https://www.meetup.com/rust-nederland/events/314523659/)
* 2026-05-26 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - Agentic Programming - May**](https://www.meetup.com/rust-dortmund/events/314522781/)
* 2026-05-26 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester May Code Night**](https://www.meetup.com/rust-manchester/events/314452972/)
* 2026-05-26 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/events/)
    * [**Motorized blinds, and replacing Docker, in Rust!**](https://www.meetup.com/rust-trondheim/events/314711434/)
* 2026-05-28 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/events/)
    * [**LDN Talks May Community Showcase**](https://www.meetup.com/rust-london-user-group/events/314846861/)
* 2026-05-29 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/314396588/)
* 2026-06-03 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin/events/)
    * [**Join us live and INPERSON for Rust 261**](https://www.meetup.com/rust-dublin/events/314689875/)
* 2026-06-03 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 06 2026**](https://luma.com/4bmlc7qd)
* 2026-06-11 | Switzerland, CH | [PostTenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-06-16 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Interactive: Everything is Open Source**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813937/)

### North America
* 2026-05-20 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Mouse Control with Rust**](https://www.meetup.com/vancouver-rust/events/313572925/)
* 2026-05-20 | San Francisco, CA, US | [Bay Area Rust Meetup](https://luma.com/bayarearust)
    * [**Bay Area Rust Meetup**](https://luma.com/9j3q5ejl)
* 2026-05-21 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**May, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873203/)
* 2026-05-21 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: "Boring File Storage" & "Indie News Feed Optimization"**](https://www.meetup.com/rust-nyc/events/314783868/)
* 2026-05-21 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Community Meetup**](https://www.meetup.com/music-city-rust-developers/events/314359076/)
* 2026-05-23 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Allston Rust Lunch, May 23**](https://www.meetup.com/bostonrust/events/314480534/)
* 2026-05-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/314209662/)
* 2026-05-28 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539319/)
* 2026-05-28 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust in Embedded & Autonomous Systems at Parallel Systems in DTLA**](https://www.meetup.com/rust-los-angeles/events/314218564/)
* 2026-05-28 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/314716463/)
* 2026-05-30 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Central Cambridge Rust Lunch, May 30**](https://www.meetup.com/bostonrust/events/314480537/)
* 2026-06-04 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Testing, Coverage, Tracey & Mutations**](https://www.meetup.com/stl-rust/events/314106244/)
* 2026-06-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Boston Common Rust Lunch, June 6**](https://www.meetup.com/bostonrust/events/314480539/)
* 2026-06-11 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Utah Rust June Meetup**](https://www.meetup.com/utah-rust/events/314696643/)
* 2026-06-11 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust/events/)
    * [**San Diego Rust June Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/313721899/)
* 2026-06-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcjbvb/)

### Oceania
* 2026-05-26 | Barton, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**May Meetup**](https://www.meetup.com/rust-canberra/events/314050576/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> Posts like this are useful for those of us who like to help, and who work on rustc to make it more helpful, by letting us learn about what kinds of mistakes people make.

– [Kevin Reid on rust-users](https://users.rust-lang.org/t/slightly-surprising-behavior-of-a-while-loop/140117/5)

Thanks to [firebits.io](https://users.rust-lang.org/t/crate-of-the-week/2704/1605) for the suggestion!

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
