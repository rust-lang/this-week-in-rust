Title: This Week in Rust 346
Number: 346
Date: 2020-07-07
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

* [Announcing Rustup 1.22.0](https://blog.rust-lang.org/2020/07/06/Rustup-1.22.0.html)
* [Ownership of the standard library implementation](https://blog.rust-lang.org/inside-rust/2020/07/02/Ownership-Std-Implementation.html)
* [Compiler Team 2020-2021 Roadmap Meeting Minutes](https://rust-lang.github.io/compiler-team/minutes/design-meeting/2020-05-29-Roadmap-2020-2021/)
* [Back to old tricks ..(or, baby steps in Rust)](https://donsbot.wordpress.com/2020/07/04/back-to-old-tricks-or-baby-steps-in-rust/)
* [Small strings in Rust](https://fasterthanli.me/articles/small-strings-in-rust)
* [Choosing a Rust web framework, 2020 edition](https://www.lpalmieri.com/posts/2020-07-04-choosing-a-rust-web-framework-2020-edition/)
* [Writing Interpreters in Rust: a Guide](https://pliniker.github.io/post/rust-hosted-langs/)
* [Transpiling A Kernel Module to Rust: The Good, the Bad and the Ugly](https://immunant.com/blog/2020/06/kernel_modules/)
* [Bad Apple!! and how I wrote a Rust video player for Task Manager!!](https://www.azabani.com/2020/06/29/bad-apple-for-taskmgr.html)
* [Boa release v0.9 and make use of Rust's measureme](https://boa-dev.github.io/2020/07/03/boa-release-09.html)
* [RiB (Rust in Blockchain) Newsletter #13](https://rustinblockchain.org/newsletters/2020-07-01-stuck-inside-hacking-away/)
* [7 Things I learned from Porting a C Crypto Library to Rust](https://sharpend.io/7-things-I-learned-from-porting-a-c-crypto-library-to-rust/)
* [AWS Lambda with Rust](https://blog.knoldus.com/aws-lambda-with-rust/)
* [Writing a winning 4K intro in Rust](https://www.codeslow.com/2020/07/writing-winning-4k-intro-in-rust.html)
* [Ringbahn II: the central state machine](https://without.boats/blog/ringbahn-ii/)
* [Bastion floating on Tide - Part 2](https://blog.bastion.rs/2020/06/14/bastion-floating-on-tide-part-2.html)
* [Porting Godot Games To Rust (Part 1)](https://paytonrules.com/post/games-in-rust-with-godot-part-one/)
* [Image decay as a service](https://fasterthanli.me/articles/image-decay-as-a-service)
* [IntelliJ Rust Changelog #125](https://intellij-rust.github.io/2020/06/29/changelog-125.html)
* [Abstracting away correctness](https://fasterthanli.me/articles/abstracting-away-correctness)
* [Rendering in Rust](https://www.zerotoga.me/dev/renderinginrust)
* [Super hero Rust fuzzing](https://blog.firosolutions.com/2020/07/superhero-rust-fuzzing/)
* [What Is a Dangling Pointer?](https://medium.com/swlh/what-is-a-dangling-pointer-2773d49cf86c)
* [Simple Rocket Web Framework Tutorial | POST Request](https://frogtok.com/simple-rocket-web-framework-tutorial-part-2in/)
* [Adventures of OS - System Calls](http://osblog.stephenmarz.com/ch7.html)
* [Allocation API, Allocators and Virtual Memory](https://notes.iveselov.info/programming/allocation-api-and-allocators)
* [Cargo [features] explained with examples](https://dev.to/rimutaka/cargo-features-explained-with-examples-194g)
* [Concurrency Patterns in Embedded Rust](https://ferrous-systems.com/blog/embedded-concurrency-patterns/)
* [Getting started with WebAssembly and Rust](https://blog.logrocket.com/getting-started-with-webassembly-and-rust/)
* [How to Write a Stack in Rust](https://dev.to/virtualkirill/how-to-write-a-stack-in-rust-3d4o)
* [Implementing WebSockets in Rust](http://subhojit777.in/implementing-websockets-in-rust/)
* [rust-analyzer changelog 32](https://rust-analyzer.github.io/thisweek/2020/07/06/changelog-32.html)
* [Rust for JavaScript Developers - Functions and Control Flow](http://www.sheshbabu.com/posts/rust-for-javascript-developers-functions-and-control-flow/)
* [Rust: The New LLVM](https://willcrichton.net/notes/rust-the-new-llvm/)
* [Using Rust and WebAssembly to Process Pixels from a Video Feed](https://dev.to/fallenstedt/using-rust-and-webassembly-to-process-pixels-from-a-video-feed-4hhg)
* [WebAssembly with Rust and React (Using create-react-app)](https://dev.to/lokesh007/webassembly-with-rust-and-react-using-create-react-app-67)
* [Portuguese] [Aprendendo Rust: 01 - Hello World](https://dev.to/pehdepano/aprendendo-rust-01-hello-world-35p4)
* [audio] [Mun](https://rustacean-station.org/episode/020-mun/)
* [audio] [Rust and machine learning #3 with Alec Mocatta (Ep. 109)](https://datascienceathome.com/rust-and-machine-learning-3-with-alec-mocatta-ep-109/)
* [video] [Authentication Service in Actix - Part 1: Configuration](https://www.youtube.com/watch?v=AH2P7Vc0N9s)
* [video] [Rust FLTK gui tutorial](https://www.youtube.com/watch?v=ygP4egJtmzw)

# Crate of the Week

This week's crate is [suckit](https://github.com/skallwar/suckit), a tool to recursively download a website.

Thanks to [Martin Schmidt](https://users.rust-lang.org/t/crate-of-the-week/2704/786) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

*No issues were proposed for CfP*.

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

* [2020-07-07](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020.md#2020-07-07). One unimportant regression on a rollup; six improvements, two on rollups.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: Add a new `#[instruction_set(...)]` attribute for supporting per-function instruction set changes](https://github.com/rust-lang/rfcs/pull/2867)
* [Inline `const` expressions and patterns](https://github.com/rust-lang/rfcs/pull/2920)
* [Inline assembly](https://github.com/rust-lang/rfcs/pull/2873)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [stabilize const mem::forget](https://github.com/rust-lang/rust/pull/73887)
* [disposition: merge] [Stabilize casts and coercions to `&[T]` in const fn](https://github.com/rust-lang/rust/pull/73862)
* [disposition: merge] [mv std libs to std/](https://github.com/rust-lang/rust/pull/73265)
* [disposition: merge] [Stabilize `transmute` in constants and statics but not const fn](https://github.com/rust-lang/rust/pull/72920)
* [disposition: merge] [Stabilize const_type_id feature](https://github.com/rust-lang/rust/pull/72488)
* [disposition: merge] [Accept tuple.0.0 as tuple indexing (take 2)](https://github.com/rust-lang/rust/pull/71322)

## New RFCs

* [RFC: IndexGet and IndexSet](https://github.com/rust-lang/rfcs/pull/2953)

# Upcoming Events

### Online
* [July 9. Berlin, DE - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybckbmb/)
* [July 9. San Diego, CA, US - July 2020 Tele-Meetup](https://www.meetup.com/San-Diego-Rust/events/271680644/)
* [July 13. Seattle, WA, US - Seattle Rust Meetup - Monthly Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrybckbsb/)
* [July 16. Turin, IT - Rust Italia - Gruppo di studio di Rust](https://community.mozilla.org/events/gruppo-di-studio-di-rust-3/)

### North America
* [July 8. Atlanta, GA, US - Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgrybckblb/)
* [July 9. Lehi, UT, US - Utah Rust - The Blue Pill: Rust on Microcontrollers](https://www.meetup.com/utah-rust/events/268567961/)
* [July 15. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybckbtb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at 1Password, Remote (US or Canada)](https://jobs.lever.co/1password/0623888f-0125-41b9-9902-eae8cfeae0c3)
* [Security Engineer at 1Password, Remote (US or Canada)](https://jobs.lever.co/1password/23444f56-c83b-4c75-85cf-64305c335e78)
* [Part-time Backend Engineer at Tagnifi, Remote (North America)](https://github.com/tagnifi/job-descriptions/issues/1)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust is like a futuristic laser gun with an almost AI-like foot detector that turns the safety on when it recognises your foot.

– [u/goofbe on reddit](https://www.reddit.com/r/rust/comments/hiyfhq/linus_torvalds_the_kernel_team_is_looking_at/fwk12r6/)

Thanks to [Synek317](https://users.rust-lang.org/t/twir-quote-of-the-week/328/898) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/hisn3e/this_week_in_rust_345/)</small>
