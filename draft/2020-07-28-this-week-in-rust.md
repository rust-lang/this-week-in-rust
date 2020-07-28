Title: This Week in Rust 349
Number: 349
Date: 2020-07-28
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*](https://rustacean-station.org/episode/023-twir-348/)

# Updates from Rust Community

### Official

* [Opening up the Core Team agenda](https://blog.rust-lang.org/inside-rust/2020/07/27/opening-up-the-core-team-agenda.html)
* [Rust's CI is Moving to GitHub Actions](https://blog.rust-lang.org/inside-rust/2020/07/23/rust-ci-is-moving-to-github-actions.html)

### Tooling

* [IntelliJ Rust Changelog #127](https://intellij-rust.github.io/2020/07/27/changelog-127.html)
* [Rust Analyzer Changelog #35](https://rust-analyzer.github.io/thisweek/2020/07/27/changelog-35.html)

### Observations/Thoughts

* [An Inside Look at the Rust Programming Language](https://about.gitlab.com/blog/2020/07/21/rust-programming-language/)
* [Notes on A Smaller Rust](https://without.boats/blog/notes-on-a-smaller-rust/)
* [Rust at FP Complete, 2020 Update](https://www.fpcomplete.com/insights/rust-at-fpco-2020/)
* [Why the Rust Module System Might Be Hard to Understand](https://dev.to/dotxlem/why-the-rust-module-system-might-be-hard-to-understand-2l)
* [Giving Rust Another Shot in 2020](https://sharpend.io/giving-rust-another-shot-in-2020/)

### Learn

* [Rust Explained using Easy English](https://github.com/Dhghomon/easy_rust)
* [(A Few) Advanced Variable Types in Rust](https://rust.graystorm.com/2020/07/20/a-few-advanced-variable-types-in-rust/)
* [Tutorial for Tokio and async Rust](https://tokio.rs/tokio/tutorial)
* [An introduction to Data-Oriented Design in Rust](http://jamesmcm.github.io/blog/2020/07/25/intro-dod/#en)
* [Learning Rust by Converting Python to Rust](https://towardsdatascience.com/learning-rust-by-converting-python-to-rust-259e735591c6)
* [Writing a file system from scratch in Rust](https://blog.carlosgaldino.com/writing-a-file-system-from-scratch-in-rust.html)
* [Memory safety in Rust - part 1](https://hashrust.com/blog/memory-safey-in-rust-part-1/)
* [Analyzing terabytes of GitHub Archive data with Rusoto and Serde](https://matthewkmayer.github.io/blag/public/post/rusty-von-humboldt/)
* [Sizedness in Rust](https://github.com/pretzelhammer/rust-blog/blob/master/posts/sizedness-in-rust.md)
* [This July in my Database project written in Rust](https://alex-dukhno.github.io/2020-07-26-This-July-in-my-Database-project-written-in-rust/)
* [Cell, RefCell, and Interior Mutability in Rust](https://badboi.dev/rust/2020/07/17/cell-refcell.html)
* [CLI Autocompletion Algorithm in Rust](https://dev.to/yujiri8/cli-autocompletion-algorithm-in-rust-47jl)
* [Compressing Authority](https://dev.to/johndriscoll/compressing-authority-1kph)
* [From C# to Rust: Code Basics](https://dev.to/sebnilsson/from-c-to-rust-code-basics-40cj)
* [Hello World with Rust and WebAssembly](https://blog.nodraak.fr/2020/07/rust-wasm-2-hello-world/)
* [Interactive Chord Diagrams (Data Analysis with Rust)](https://shahinrostami.com/posts/programming/rust-notebooks/chord-diagrams/)
* [Some (Number) of Ways to Calculate a Fibonacci Number in Rust](https://dev.to/jculverhouse/some-number-of-ways-to-calculate-a-fibonacci-number-in-rust-d78)
* [Writing A Simple Query System in Rust](https://pnevyk.github.io/posts/query-system-in-rust/)
* [JSONB Data with Rust and Diesel](https://vasilakisfil.social/blog/2020/05/09/rust-diesel-jsonb/)
* [Enum or Trait Object](https://www.possiblerust.com/guide/enum-or-trait-object)
* [Parallel Stream Processing with Rayon](https://morestina.net/blog/1432/parallel-stream-processing-with-rayon)
* [Compile Time CUDA Device Checking in Rust](https://m-decoster.github.io/2020/07/24/compile-time-cuda/)
* [Async/Await for AVR with Rust](https://lights0123.com/blog/2020/07/25/async-await-for-avr-with-rust/)
* [Making a Game in 48 hours with Rust and WebAssembly](https://ianjk.com/rust-gamejam/)
* [PT] [Aprendendo Rust: 04 - Comentários, documentação e tipos de variáveis primitivas](https://dev.to/pehdepano/aprendendo-rust-04-comentarios-documentacao-e-tipos-de-variaveis-primitivas-1jb6)
* [UK] [Пошук в глибину на прикладі задачі Ханойської вежі](https://dev.to/yaroslavpodorvanov/-3n3b)
* [video] [Utility AI (with Rust example) - How-To](https://www.youtube.com/watch?v=M0Sx_M61ILU&feature=youtu.be)
* [video] [Writing A Guide to Slice Data in Rust](https://www.twitch.tv/videos/691303613)
* [video] [Learning Rust GameDev Patterns](https://www.twitch.tv/videos/691311447)
* [video] [Run any web container for free on Google CloudRun (example in rust/warp)](https://www.youtube.com/watch?v=SMTVwISbQtE)
* [video] [Technologieplauscherl: Memory Ownership in C# and Rust](https://www.youtube.com/watch?v=20GNFE0462w)
* [video] [FLTK Rust Tutorial: Create a media player using the vlc crate](https://www.youtube.com/watch?time_continue=289&v=enxqU3bhCEs&feature=emb_logo)
* [video] [Rust at Speed - Building A Fast Concurrent Database](https://youtu.be/s19G6n0UjsM)
* [FR] [video] [Développement Web Rust & Rocket](https://www.youtube.com/playlist?list=PLMWEEzYqZ0ekOG6_G4q_GXPpVHWrIH--x)
* [PT] [video] [Hoje sai o data tables em RUST](https://www.twitch.tv/videos/688423082)

### Project Updates

* [Ballista Distributed Compute: One Year Later](https://andygrove.io/2020/07/ballista-one-year-on/)
* [Cross-compiling to Redox using Nix](https://www.redox-os.org/news/redox-plus-nix-0/)
* [Public Announcement: You Can Now Debug Programs Using GDB on Redox OS](https://www.redox-os.org/news/public-announcement-gdb/)
* [Rocket Can Now Compile on Stable Rust](https://www.reddit.com/r/rust/comments/hviz2q/rocket_can_now_compile_on_stable_rust/)

### Miscellaneous

* [ANSSI - Programming Rules to Develop Secure Applications with Rust](https://www.ssi.gouv.fr/uploads/2020/06/anssi-guide-programming_rules_to_develop_secure_applications_with_rust-v1.0.pdf)
* [Rust is now a top 20 language in all of the 5 most major language popularity listings](https://www.reddit.com/r/rust/comments/hz7dfp/rust_is_now_a_top_20_language_in_all_of_the_5/)
* [Under the Hood of Linkerd's State of the Art Rust Proxy, Linkerd2-proxy](https://linkerd.io/2020/07/23/under-the-hood-of-linkerds-state-of-the-art-rust-proxy-linkerd2-proxy/)
* [Performance Comparison: Rust vs PyO3 vs Python](https://medium.com/the-innovation/performance-comparison-rust-vs-pyo3-vs-python-6480709be8d)
* [Optimising Rust: Clockwise Triangles](https://wapl.es/rust/2020/07/25/optimising-with-cmp-and-ordering.html)

# Crate of the Week

This week's crate is [pre](https://github.com/aticu/pre), a library for declaring and checking the assurance of precondition, useful for unsafe functions.

Thanks to [Zicklag](https://users.rust-lang.org/t/crate-of-the-week/2704/792) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.


If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

394 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-07-13..2020-07-20

* [do not try fetching the ancestors of errored trait impls](https://github.com/rust-lang/rust/pull/74516)
* [only skip impls of foreign unstable traits](https://github.com/rust-lang/rust/pull/74534)
* [don't assign `()` to `!` MIR locals](https://github.com/rust-lang/rust/pull/74411)
* [some `Symbol` related improvements](https://github.com/rust-lang/rust/pull/74357)
* [use `ArrayVec` in `SparseBitSet`](https://github.com/rust-lang/rust/pull/74310)
* [change `SymbolName::name` to a `&str`](https://github.com/rust-lang/rust/pull/74214)
* [enforce the static symbol order](https://github.com/rust-lang/rust/pull/74203)
* [reduce the amount of interning and `layout_of` calls in const eval](https://github.com/rust-lang/rust/pull/74202)
* [add `Arguments::as_str()`](https://github.com/rust-lang/rust/pull/74056)
* [`impl Index<RangeFrom> for CStr`](https://github.com/rust-lang/rust/pull/74021)
* [add (unchecked) indexing methods to raw (and NonNull) slices](https://github.com/rust-lang/rust/pull/73986)
* [make some `Option` methods const](https://github.com/rust-lang/rust/pull/73930)
* [use `step_unchecked` more liberally in range iter impls](https://github.com/rust-lang/rust/pull/73490)
* [add `core::task::ready!` macro](https://github.com/rust-lang/rust/pull/70817)
* [backtrace: use noop backends on Miri](https://github.com/rust-lang/backtrace-rs/pull/360)
* [stdarch: update and revamp wasm32 SIMD intrinsics](https://github.com/rust-lang/stdarch/pull/874)
* [stdarch: implement AVX512f floating point comparisons](https://github.com/rust-lang/stdarch/pull/869)
* [stdarch: constify all x86 `rustc_args_required_const` intrinsics](https://github.com/rust-lang/stdarch/pull/876)
* [make `unreachable_unchecked` a const fn](https://github.com/rust-lang/rust/pull/74459)
* [cargo: fix freshness checks for build scripts on renamed dirs](https://github.com/rust-lang/cargo/pull/8497)
* [crates.io: generate API tokens with a secure RNG, store hashed](https://github.com/rust-lang/crates.io/pull/2637)
* [add Ayu theme to rustdoc](https://github.com/rust-lang/rust/pull/71237)
* [clippy: `unnecessary_sort_by`: avoid linting if key borrows](https://github.com/rust-lang/rust-clippy/pull/5756)

## Rust Compiler Performance Triage

* [2020-07-28](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-07-28.md).
  2 regressions, 1 improvement, none in rollups.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: 'C unwind' ABI](https://github.com/rust-lang/rfcs/pull/2945)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize core::future::{pending,ready}](https://github.com/rust-lang/rust/pull/74328)
* [disposition: merge] [Make more primitive integer methods const](https://github.com/rust-lang/rust/pull/73858)
* [disposition: merge] [Derive common traits for panic::Location](https://github.com/rust-lang/rust/pull/73583)

## New RFCs

* [Add `oneof` configuration predicate to support exclusive features](https://github.com/rust-lang/rfcs/pull/2962)
* [RFC: Promote aarch64-unknown-linux-gnu to a Tier-1 Rust target](https://github.com/rust-lang/rfcs/pull/2959)
* [Add Drop::poll_drop_ready for asynchronous destructors](https://github.com/rust-lang/rfcs/pull/2958)
* [Stabilize Cargo's new feature resolver](https://github.com/rust-lang/rfcs/pull/2957)
* [Add the partial-closure-args RFC](https://github.com/rust-lang/rfcs/pull/2956)

# Upcoming Events

### Online
* [July 23. Berlin, DE - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybckbfc/)
* [July 27 - August 2. Rusty Days Virtual Rust Conference](https://rusty-days.org/)
* [August 6, Rust Meetup Linz](https://www.meetup.com/de-DE/Rust-Linz/events/271857182/)

### North America
* [July 27. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybckbkc/)
* [July 28. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/nppvrrybckblc/)

### Asia Pacific
* [August 3. Auckland, NZ - Rust ALK - Rust Meetup](https://www.meetup.com/rust-akl/events/266876693/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer Backend (m/f/d) Rust at Snapview GmbH ( München, DE)](https://stackoverflow.com/jobs/415671/software-engineer-backend-m-f-d-rust-snapview-gmbh)
* [Backend Engineer at Spruce (Remote)](https://docs.google.com/document/d/1ZrvfGtgVGh63ezpiaerc95SFR64cGU4ftBJENczdvAs)
* [Backend Engineer - Rust at Kraken (Remote)](https://jobs.lever.co/kraken/4019a818-4a7b-46ef-9225-c53c7a7f238c)
* [Backend Engineer - Data Processing - Rust at Kraken (Remote)](https://jobs.lever.co/kraken/246f7fd2-000a-4f61-8f53-b1cc783d51cb)
* [Senior Backend Engineer - Rust at Kraken (Remote)](https://jobs.lever.co/kraken/4c864c8f-bde6-443d-b521-dd90df0e9105)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> `unsafe` Rust is all about flirting with UB but never giving in.

– [Ralf Jung on Zulip](https://rust-lang.zulipchat.com/#narrow/stream/136281-t-lang.2Fwg-unsafe-code-guidelines/topic/Language.20UB.20vs.20library.20UB/near/204212193)

Thanks to [HeroicKatora](https://users.rust-lang.org/t/twir-quote-of-the-week/328/913) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/hvjf4i/this_week_in_rust_348/)</small>
