Title: This Week in Rust 348
Number: 348
Date: 2020-07-21
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

## News & Blog Posts

* [crates.io security advisory](https://blog.rust-lang.org/2020/07/14/crates-io-security-advisory.html)
* [Announcing Rust 1.45.0](https://blog.rust-lang.org/2020/07/16/Rust-1.45.0.html)
* [Traits working group 2020 sprint 3 summary](https://blog.rust-lang.org/inside-rust/2020/07/17/traits-sprint-3.html)
* [Best Programming Languages to Learn in 2020(Rust Lang)](https://frogtok.com/useful-and-best-programming-languages-to-learn-in-2020/)
* [Learn how to make a Sokoban game in Rust!](https://sokoban.iolivia.me/c01-00-intro.html)
* [Clear explanation of Rust’s module system](http://www.sheshbabu.com/posts/rust-module-system/)
* [Checking status of Rust features](https://notes.iveselov.info/programming/checking-status-of-rust-features)
* [Programming Servo: just hanging around](https://medium.com/programming-servo/programming-servo-just-hanging-around-d8f660c33df2?source=friends_link&sk=6efbf13743aec335bd11834c2df71783)
* [Efficient representation of Ultimate Tic Tac Toe using Rust](https://www.minimax.dev/docs/ultimate/efficient-representation/)
* [Next Algorithm: Backtracking into the n Queens Problem](https://rust.graystorm.com/2020/07/16/next-algorithm-backtracking-into-the-n-queens-problem/)
* [RSoC: improving drivers and kernel - part 3 (largely io_uring)](https://redox-os.org/news/io_uring-3/)
* [Writing an asynchronous MQTT Broker in Rust - Part 2](https://hassamuddin.com/blog/rust-mqtt/ping-pong/)
* [Rust Closures: Returning `impl Fn` for `move` closures](https://notes.iveselov.info/programming/rust-closures-combining-move-and-fn)
* [Writing a kernel driver with Rust](https://not-matthias.github.io/kernel-driver-with-rust/)
* [Rewriting FORTRAN Software In Rust](https://mckeogh.tech/post/shallow-water/)
* [Building and debugging a high-throughput daemon in Rust](https://brokenco.de/2020/07/15/high-throughput-in-rust.html)
* [Why even unused data needs to be valid](https://www.ralfj.de/blog/2020/07/15/unused-data.html)
* [Three Architectures for a Responsive IDE](https://rust-analyzer.github.io/blog/2020/07/20/three-architectures-for-responsive-ide.html)
* [Packaging & Vending Production Rust Software - Windows](https://ebbflow.io/blog/vending-win)
* [Async Rust, but less intimidating](https://dev.to/dotxlem/async-rust-but-less-intimidating-2c13)
* [Rust Analyzer Changelog #34](https://rust-analyzer.github.io/thisweek/2020/07/20/changelog-34.html)
* [Benchmarking gRPC in Rust & Go](https://medium.com/@Rustling_gopher/benchmarking-grpc-in-rust-go-184545e7688a)
* [Efficient Parsing of JSON Record Sets in Rust](https://dev.to/virtualkirill/how-to-write-a-queue-in-rust-12m9)
* [Function Overloading in Rust](https://medium.com/swlh/function-overloading-in-rust-d591aff64a03)
* [Geometric Constraint Solvers Part 1: Algebraic Expressions](http://adventures.michaelfbryan.com/posts/constraints-part-1-expressions/?utm_source=reddit&utm_medium=social&utm_campaign=constraint-solver-1-expression-trees)
* [Haskell::From(Rust) I: Infix Notation and Currying](https://seanchen1991.github.io/posts/haskell-from-rust-i/)
* [How to Write a Queue in Rust](https://dev.to/virtualkirill/how-to-write-a-queue-in-rust-12m9)
* [Implementing a Workflow Graph](https://milchdealer.github.io/2020/07/19/Implementing-a-workflow-graph.html)
* [Implementing `flat_map` in Rust](https://www.eltonpinto.me/blog/posts/implementing_flatmap_in_rust/)
* [IntelliJ Rust 0.3: New Macro Expansion Engine](https://blog.jetbrains.com/clion/2020/07/intellij-rust-0-3-new-macro-expansion-engine/)
* [Low Level Stuff pt. 1 - Booting to 'Hello Rust!'](https://micouy.github.io/posts/low-level-pt-1/)
* [My Bet on Rust has been Vindicated](https://nbsoftsolutions.com/blog/my-bet-on-rust-has-been-vindicated.html)
* [The Next Steps for Single Ownership and RAII](https://vale.dev/blog/raii-next-steps)
* [Popol: Minimal Non-Blocking I/O with Rust](https://cloudhead.io/popol/)
* [Rust vs Go in Backend Web Development](https://qvault.io/2020/07/17/rust-vs-go-in-backend-web-development/)
* [Shipping Const Generics in 2020](https://without.boats/blog/shipping-const-generics/)
* [Two Beautiful Rust Programs](https://matklad.github.io//2020/07/15/two-beautiful-programs.html)
* [Japanese] [TCPが遅すぎる？QUICを使おう！](https://medium.com/nttlabs/quic-with-rust-9cf9b44596ad)
* [Spanish] [Rust para embebidos](https://dev.to/iddar/rust-para-embebidos-4agn)
* [Portuguese] [Aprendendo Rust: 03 - Variáveis](https://dev.to/pehdepano/aprendendo-rust-03-variaveis-57a8)
* [Portuguese] [video] [Curso Rust 🦀 - Aula 4 - strings, match, sorteio, quizz](https://www.twitch.tv/videos/681897847)
* [video] [Boiled Down Crate 🦀: OnceCell](https://www.youtube.com/watch?v=YBG8QTO8fNI&feature=youtu.be)
* [video] [Rust: What is Ownership and Borrowing](https://www.youtube.com/watch?v=79phqVpE7cU)

# Crate of the Week

This week's crate is [pre](https://github.com/aticu/pre), a library for declaring and checking the assurance of precondition, useful for unsafe functions.

Thanks to [Zicklag](https://users.rust-lang.org/t/crate-of-the-week/2704/792) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [zbus is looking for contributors](https://gitlab.freedesktop.org/zeenix/zbus/-/issues)
* [just: Add extensible recipe and justfile attributes](https://github.com/casey/just/issues/604)
* [libpnet: Segfault in icmp send](https://github.com/libpnet/libpnet/issues/449)
* [rust: fs::remove_dir_all rarely succeeds for large directories on window](https://github.com/rust-lang/rust/issues/29497)


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

* [2020-07-21](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-07-21.md).
  A disastrous week. At least 7 regressions. 3 improvements. Lots of murkiness due to rollups.

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
* [July 27 - August 8. Rusty Days Virtual Rust Conference](https://rusty-days.org/)

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

* [RUST Senior Software Engineer - Backend at LogDNA (Remote, US)](https://www.linkedin.com/jobs/cap/view/1922843992/?pathWildcard=1922843992&trk=mcm)
* [Rust Engineer at Elektron (Gothenburg, SE)](https://www.elektron.se/rust-engineer/)
* [Software Engineer - Systems and Analytics at Noibu (Ottawa, ON, CA)](https://www.indeedjobs.com/jobs/4261e5785229cb748d43?from=snippet)
* [Software Engineer - Data Collection at Noibu (Ottawa, ON, CA)](https://www.indeedjobs.com/jobs/99f93d6ff0f763d6c0c8?from=snippet)
* [Senior Backend Engineer at OneSignal (San Mateo, CA, US)](https://onesignal.com/careers/9a60a245-06d9-4e2a-82fb-da5e1e9d22d8)
* [Rust Developer at OnePassword (Remote, US or CA)](https://jobs.lever.co/1password/0623888f-0125-41b9-9902-eae8cfeae0c3)
* [Systems Engineer at Findora (Menlo Park, CA, US)](https://jobs.lever.co/findora/88502a0d-a86d-4cd2-b0b7-8625a107b02b)
* [Senior Systems Engineer at Findora (Menlo Park, CA, US)](https://jobs.lever.co/findora/e89e2e02-622c-41da-a14d-c12d854a25b5)
* [Rust Developer at ESR Labs (München, DE)](https://www.esrlabs.com/careers/position/?jobPostingId=7156225)
* [french] [Rust Instructor - University of Paris 8 (Paris, FR)](https://twitter.com/p4bl0/status/1283723397478973440)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> `unsafe` Rust is all about flirting with UB but never giving in.

– [Ralf Jung on Zulip](https://rust-lang.zulipchat.com/#narrow/stream/136281-t-lang.2Fwg-unsafe-code-guidelines/topic/Language.20UB.20vs.20library.20UB/near/204212193)

Thanks to [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/903) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust]()small>
