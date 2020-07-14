Title: This Week in Rust 347
Number: 347
Date: 2020-07-14
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*]()

# Updates from Rust Community

## News & Blog Posts

* [Announcing Rustup 1.22.1](https://blog.rust-lang.org/2020/07/08/Rustup-1.22.1.html)
* [Lang team design meeting: path to membership](https://blog.rust-lang.org/inside-rust/2020/07/09/lang-team-path-to-membership.html)
* [Perspective on Rust Community Moderation](https://www.reddit.com/r/rust/comments/hnfnti/where_is_the_rust_community_allowed_to_talk_about/fxf65nf/)
* [Programming Servo: the bird’s-eyes view](https://medium.com/programming-servo/programming-servo-the-birds-eyes-view-201d28220b9a?source=friends_link&sk=b8610f254bf6faf8b81c81729c1b3498)
* [Faster Integer Parsing (Rust port)](https://rust-malaysia.github.io/code/2020/07/11/faster-integer-parsing.html)
* [Learning Rust: Let's Build a Parser](https://codeandbitters.com/lets-build-a-parser/)
* [nnnoiseless: porting audio code from C to rust](https://jneem.github.io/nnnoiseless/)
* [The Soul of a New Debugger](https://nbaksalyar.github.io/2020/07/12/soul-of-a-new-debugger.html)
* [String interners in Rust](https://dev.to/cad97/string-interners-in-rust-797)
* [Target Feature vs Target CPU for Rust](https://www.nickwilcox.com/blog/target_cpu_vs_target_feature/)
* [Rust and it's Orphan Rules](https://blog.mgattozzi.dev/orphan-rules/)
* [Async Interview #8: Stjepan Glavina](https://smallcultfollowing.com/babysteps/blog/2020/07/09/async-interview-8-stjepan-glavina/)
* [Using RabbitMQ in Rust](https://www.zupzup.org/rmq-in-rust/)
* [Creating a custom target](https://rust-embedded.github.io/embedonomicon/custom-target.html)
* [Statically Sized Higher-kinded Polymorphism](http://blog.ielliott.io/sized-hkts/)
* [Getting in and out of trouble with Rust futures](https://fasterthanli.me/articles/getting-in-and-out-of-trouble-with-rust-futures)
* [Rust Analyzer Changelog #33](https://rust-analyzer.github.io/thisweek/2020/07/13/changelog-33.html)
* [IntelliJ Rust Changelog #126](https://intellij-rust.github.io/2020/07/13/changelog-126.html)
* [This Month in Rust OSDev (June 2020)](https://rust-osdev.com/this-month/2020-06/)
* [Cross-compiling Rust from ARM to x86-64](https://burgers.io/cross-compile-rust-from-arm-to-x86-64)
* [Writing Rust NIFs for Elixir With Rustler](https://simplabs.com/blog/2020/06/25/writing-rust-nifs-for-elixir-with-rustler/)
* [Geometric Constraint Solvers Part 1: Algebraic Expressions](http://adventures.michaelfbryan.com/posts/constraints-part-1-expressions/)
* [A View of Async Memory Access in Rust](https://blog.haoxp.xyz/posts/async-memory-access/)
* [Building Canrun: A Statically Typed Logic Programming Library for Rust (Part 1)](https://esimmler.com/building-canrun-part-1/)
* [Hunting down a non-determinism-bug in our Rust Wasm build](https://dev.to/gnunicorn/hunting-down-a-non-determinism-bug-in-our-rust-wasm-build-4fk1)
* [Implementing the Clipper Chip Cipher in Rust](https://blog.yossarian.net/2020/03/09/Implementing-the-Clipper-chip-cipher-in-Rust)
* [Linux Developers May Discuss Allowing Rust Code Within the Kernel](https://www.phoronix.com/scan.php?page=news_item&px=Linux-Plumbers-2020-Rust)
* [Machine Learning in Rust and WebAssembly](https://www.secondstate.io/articles/machine-learning/)
* [Moving from TypeScript to Rust/WebAssembly](https://nicolodavis.com/blog/typescript-to-rust/)
* [RSoC: Improving Drivers and Kernel - Part 1 (Largely io_uring)](https://www.redox-os.org/news/io_uring-1/)
* [Rust and WebAssembly from Scratch: Hello World with Strings](https://depth-first.com/articles/2020/07/07/rust-and-webassembly-from-scratch-hello-world-with-strings/)
* [Rust for JavaScript Developers - Pattern Matching and Enums](http://www.sheshbabu.com/posts/rust-for-javascript-developers-pattern-matching-and-enums/)
* [Rust is Surprisingly Good as a Server Language](https://stu2b50.dev/posts/rust-is-surpris76171)
* [Some Learnings from Implementing a Normalizing Rust Representer](https://dev.to/seanchen1991/some-learnings-from-implementing-a-normalizing-rust-representer-2l12)
* [Subclassing GTK Widgets in Rust](https://www.figuiere.net/technotes/notes/tn002/)
* [spanish] [Rust es orientado a objeto?](https://emanuelpeg.blogspot.com/2020/07/rust-es-orientado-objeto.html#.XwsegbMr_EQ.reddit)
* [audio] [Fuzzing Rust with Shnatsel Podcast](https://medium.com/@social_62682/fuzzing-rust-with-shnatsel-podcast-e1fa0dbc28a)
* [video] [Rust Loops Part 1: loop, break, continue](https://www.youtube.com/watch?v=zVX2qJFHDXA&feature=youtu.be)
* [video] [Hitting A Bug in the Rust Compiler - While Writing A Boring Link Checker](https://www.youtube.com/watch?time_continue=4&v=DArJCR0HDL8&feature=emb_logo)
* [video] [Two Sum Problem - Leet Code + Rust](https://www.youtube.com/watch?v=CMlHbAGkXjA&list=PLK_g1a_cAfaZuTXzDoQUAFEHCalKSCv9G&index=2)
* [video] [Rust + WebAssembly - EdgeXR @ Netlight](https://www.youtube.com/watch?v=dmbqpg5BuBY)
* [video] [Rust Meetup - Adding WASM Support to a Native Application (GameDev Edition)](https://www.youtube.com/watch?v=7YQGwb4_AvA)

# Crate of the Week

This week's crate is [suckit](https://github.com/skallwar/suckit), a tool to recursively download a website.

Thanks to [Martin Schmidt](https://users.rust-lang.org/t/crate-of-the-week/2704/786) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [rust_gpiozero is looking for contributors](https://github.com/rahul-thakoor/rust_gpiozero/issues/11)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

308 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-06-29..2020-07-06

* [add `format_args_capture` feature](https://github.com/rust-lang/rust/pull/73670)
* [don't implement Fn* traits for `#[target_feature]` functions](https://github.com/rust-lang/rust/pull/73306)
* [fix wasm32 being broken due to a NodeJS version bump](https://github.com/rust-lang/rust/pull/73885)
* [handle `macro_rules!` tokens consistently across crates](https://github.com/rust-lang/rust/pull/73569)
* [implement `slice_strip` feature](https://github.com/rust-lang/rust/pull/73414)
* [make `likely` and `unlikely` const, gated by feature `const_unlikely`](https://github.com/rust-lang/rust/pull/73778)
* [optimise fast path of checked_ops with `unlikely`](https://github.com/rust-lang/rust/pull/73938)
* [provide more information on duplicate lang item error.](https://github.com/rust-lang/rust/pull/73449)
* [remove `TypeckTables::empty(None)` and make hir_owner non-optional.](https://github.com/rust-lang/rust/pull/73751)
* [remove unnecessary release from Arc::try_unwrap](https://github.com/rust-lang/rust/pull/74025)
* [serialize all foreign `SourceFile`s into proc-macro crate metadata](https://github.com/rust-lang/rust/pull/73706)
* [stabilize `#[track_caller]`.](https://github.com/rust-lang/rust/pull/72445)
* [use WASM's saturating casts if they are available](https://github.com/rust-lang/rust/pull/73724)
* [use `Span`s to identify unreachable subpatterns in or-patterns](https://github.com/rust-lang/rust/pull/73973)
* [Update the rust-lang/llvm-project submodule to include AVR fixes recently merged](https://github.com/rust-lang/rust/pull/73658)
* [mir-opt: Fix mis-optimization and other issues with the SimplifyArmIdentity pass](https://github.com/rust-lang/rust/pull/73949)
* [added `.collect()` into `String` from `Box<str>`](https://github.com/rust-lang/rust/pull/72688)
* [impl `From<char>` for `String`](https://github.com/rust-lang/rust/pull/73466)
* [linker: create `GNU_EH_FRAME` header by default when producing ELFs](https://github.com/rust-lang/rust/pull/73564)
* [resolve: disallow labelled breaks/continues through closures/async blocks](https://github.com/rust-lang/rust/pull/73726)
* [ship rust analyzer](https://github.com/rust-lang/rust/pull/72978)
* [chalk: add type outlives goal](https://github.com/rust-lang/chalk/pull/551)
* [chalk: allow printing lifetime placeholders](https://github.com/rust-lang/chalk/pull/557)
* [chalk: support for ADTs](https://github.com/rust-lang/chalk/pull/524)
* [hashbrown: add RawTable::erase and remove](https://github.com/rust-lang/hashbrown/pull/171)
* [hashbrown: expose RawTable::try_with_capacity](https://github.com/rust-lang/hashbrown/pull/174)
* [hashbrown: improve RawIter re-usability](https://github.com/rust-lang/hashbrown/pull/175)
* [libc: add a bunch of constants and functions which were missing on Android](https://github.com/rust-lang/libc/pull/1795)
* [libc: add more WASI libc definitions.](https://github.com/rust-lang/libc/pull/1811)
* [libc: declare `seekdir` and `telldir` for WASI.](https://github.com/rust-lang/libc/pull/1804)
* [stdarch: fix or equals integer comparisons](https://github.com/rust-lang/stdarch/pull/872)
* [cargo: write GNU tar files, supporting long names.](https://github.com/rust-lang/cargo/pull/8453)
* [crates.io: use default branch alias instead of "master"](https://github.com/rust-lang/crates.io/pull/2601)
* [clippy: added restriction lint: pattern-type-mismatch](https://github.com/rust-lang/rust-clippy/pull/4841)
* [clippy: suggest `Option::map_or`(`_else`) for `if let Some { y } else { x }`](https://github.com/rust-lang/rust-clippy/pull/5301)
* [rustfmt: do not duplicate const keyword on parameters](https://github.com/rust-lang/rustfmt/pull/4294)
* [rustfmt: do not remove fn headers (e.g., async) on extern fn items](https://github.com/rust-lang/rustfmt/pull/4291)
* [rustfmt: pick up comments between trait where clause and open block](https://github.com/rust-lang/rustfmt/pull/4292)

## Rust Compiler Performance Triage

* [2020-07-14](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-07-14.md). Twelve revisions checked. Zero regressions. One improvement.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Remove trait `LengthAtMost32`](https://github.com/rust-lang/rust/pull/74060)
* [disposition: merge] [Stabilize control-flow-guard codegen option](https://github.com/rust-lang/rust/pull/73893)
* [disposition: merge] [Impl Default for ranges](https://github.com/rust-lang/rust/pull/73197)
* [disposition: merge] [Tracking issue for `core::{f32,f64}::consts::TAU` ](https://github.com/rust-lang/rust/issues/66770)

## New RFCs

* [Opt-in Stable Trait VTables](https://github.com/rust-lang/rfcs/pull/2955)

# Upcoming Events

### Online
* [July 14. Dallas, TX, US - Dallas Rust - Second Tuesday](https://www.meetup.com/Dallas-Rust/events/mzzfsrybckbsb/)
* [July 16. Turin, IT - Rust Italia - Gruppo di studio di Rust](https://community.mozilla.org/events/gruppo-di-studio-di-rust-3/)
* [July 27 - August 8. Rusty Days Virtual Rust Conference](https://rusty-days.org/)

### North America
* [July 15. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybckbtb/)
* [July 27. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybckbkc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Software Engineer - Backend at LogDNA (Remote, US)](https://boards.greenhouse.io/logdna/jobs/4703358002)
* [Senior Security Engineer at LogDNA (Remote, US)](https://boards.greenhouse.io/logdna/jobs/4702074002)
* [Rust Engineer at Elektron (Gothenburg, Sweden)](https://www.elektron.se/rust-engineer/)
* [Open Source Engineer (Remote) at Embark Studios (Stockholm)](https://www.embark-studios.com/jobs/910166-open-source-engineer)
* [Software Engineer (Remote) at Embark Studios (Stockholm)](https://www.embark-studios.com/jobs/910166-open-source-engineer)
* [dutch] [Vacature software engineer (Rust / Python) at Dreamsolution (Netherlands)](https://www.dreamsolution.nl/2020-07-03-vacature-rust-python-software-engineer.html)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust is like a futuristic laser gun with an almost AI-like foot detector that turns the safety on when it recognises your foot.

– [u/goofbe on reddit](https://www.reddit.com/r/rust/comments/hiyfhq/linus_torvalds_the_kernel_team_is_looking_at/fwk12r6/)

Thanks to [Synek317](https://users.rust-lang.org/t/twir-quote-of-the-week/328/898) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/hnkws3/this_week_in_rust_346/)</small>
