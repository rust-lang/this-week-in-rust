Title: This Week in Rust 409
Number: 409
Date: 2021-09-22
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Official

### RustConf 2021
* [Project Update: Lang Team by Niko Matsakis](https://youtu.be/ylOpCXI2EMM)
* [Project Update: Libs Team by Mara Bos](https://youtu.be/DnYQKWs_7EA)
* [Move Constructors: Is it Possible? by Miguel Young de la Sota](https://youtu.be/UrDhMWISR3w)
* [The Importance of Not Over-Optimizing in Rust by Lily Mara](https://youtu.be/CV5CjUlcqsw)
* [Identifying Pokemon Cards by Hugo Peixoto](https://youtu.be/BLy_YF4nmqQ)
* [Fuzz Driven Development by Midas Lambrichts](https://youtu.be/qUu1vJNg8yo)
* [Writing the Fastest GDBT Library in Rust by Isabella Tromba](https://youtu.be/D1NAREuicNs)
* [Whoops! I Rewrote it in Rust by Brian Martin](https://youtu.be/m-Qg3OoPIdc)
* [How I Used Rust to Become Extremely Offline by Luke Westby](https://youtu.be/b0I4vP2CP88)
* [Supercharging Your Rust Code With Five Little-Known Attributes by Jackson Lewis](https://youtu.be/8d7DqeYXq7A)
* [Compile-Time Social Coordination by Zac Burns](https://youtu.be/4_Jg-rLDy-Y)
* [Hacking rustc: Contributing to the Compiler by Esteban Kuber](https://youtu.be/9H9SO2u6Q20)
* [This Week in Rust: 400 Issues and Counting! by This Week in Rust's very own Lead Editor Nell Shamrell-Harrington!](https://youtu.be/OZPXhmy-wVw)

### Project/Tooling Updates
* [rust-analyzer Changelog #95](https://rust-analyzer.github.io/thisweek/2021/09/20/changelog-95.html)
* [Rome will be rewritten in Rust](https://rome.tools/blog/2021/09/21/rome-will-be-rewritten-in-rust)
* [SixtyFPS (GUI crate) weekly report 19th of September](https://sixtyfps.io/thisweek/2021-09-20.html)
* [Introducing SeaORM: An async & dynamic ORM for Rust](https://www.sea-ql.org/SeaORM/blog/2021-09-20-introducing-sea-orm)
* [wgpu alliance with Deno](https://gfx-rs.github.io/2021/09/16/deno-webgpu.html)
* [Experimenting Is Underway For Rust Code Within Mesa](https://www.phoronix.com/scan.php?page=news_item&px=Mesa-Rust-2021-Experiment)
* [Goscript is now language feature complete](https://www.reddit.com/r/rust/comments/pqiiw1/goscript_is_now_language_feature_complete/)
* [An update on Memory Safety in Chrome](https://security.googleblog.com/2021/09/an-update-on-memory-safety-in-chrome.html?m=1)
* [This week in Databend #8](https://datafuselabs.github.io/weekly/2021-09-22-databend-weekly/)

### Observations/Thoughts
* [How to use Vectors in Rust?](https://blog.knoldus.com/how-to-use-vectors-in-rust/)
* [Message Passing in Rust Threads is very helpful](https://blog.knoldus.com/message-passing-in-rust-threads-is-very-helpful/)
* [New project: Wheel of Fortune solver! (and Rust is still faster than Python)](https://gregstoll.wordpress.com/2021/09/18/new-project-wheel-of-fortune-solver-and-rust-is-still-faster-than-python/)
* [Rustacean Principles](https://smallcultfollowing.com/babysteps/blog/2021/09/08/rustacean-principles/)
* [Rustacean Principles, Continued](https://smallcultfollowing.com/babysteps//blog/2021/09/16/rustacean-principles-continued/)
* [Full Stack Rust](https://www.justinm.one/blog/2021/09/11/fullstackrust/)
* [Learnability of Rust](https://epage.github.io/blog/2021/09/learning-rust/)
* [The Rust Programming Language for Game Tooling](https://research.activision.com/publications/2021/09/the-rust-programming-language-for-game-tooling)
* [Reality Check for Cloudflare Wasm Workers and Rust](https://nickb.dev/blog/reality-check-for-cloudflare-wasm-workers-and-rust)
* [Rust on the MOS 6502: Beyond Fibonacci](https://gergo.erdi.hu/blog/2021-09-18-rust_on_the_mos_6502__beyond_fibonacci/)
* [Rocket: A Web Framework for Rust](https://tech.marksblogg.com/rocket-rust-web-framework.html)
* [Oxidizing Kraken: Improving Kraken Infrastructure Using Rust](https://blog.kraken.com/post/7964/oxidizing-kraken-improving-kraken-infrastructure-using-rust/)
* [Tricky compatibility issue - recent changes to `std::io::ErrorKind`](https://diziet.dreamwidth.org/9894.html)
* [audio] [Flutter/Rust Roundtable - Flying High with Flutter #24](https://youtu.be/SsZUicpnpVE)
* [audio] [Rustacean Station: From Zero to Production with Luca Palmieri](https://rustacean-station.org/episode/036-luca-palmieri/)
* [audio] [AreWePodcastYet 08 with Jon Ferdinand Ronge Gjenset](https://soundcloud.com/arewepodcastyet/awpy-08-jon-ferdinand-ronge-gjengset)
* [audio] [Rustacean Station: Rust Code Coverage with Daniel McKenna](https://rustacean-station.org/episode/037-daniel-mckenna/)
* [video] [rust4ml @ Rust DC w/ Vlad Orlov](https://youtu.be/FiEbgZlBXhk)
* [video] [In-kernel, fast-path packet processing with AF_XDP @ Rust DC w/ Collins Huff](https://youtu.be/Gv-nG6F_09I)

### Rust Walkthroughs
* [How to end-to-end encrypt all application layer communication](https://github.com/ockam-network/ockam/tree/develop/documentation/use-cases/end-to-end-encrypt-all-application-layer-communication)
* [Rust on ESP32](https://kerkour.com/blog/rust-on-esp32)
* [How to compile Rust and LLVM for ESP32 on a Raspberry Pi](https://kerkour.com/blog/compile-rust-for-esp32-xtensa-on-raspberry-pi-aarch64/)
* [Optimize Rust binaries size with cargo and Semver](https://oknozor.github.io/blog/optimize-rust-binary-size/)
* [Combining Axum, Hyper, Tonic, and Tower for hybrid web/gRPC apps: Part 2](https://www.fpcomplete.com/blog/axum-hyper-tonic-tower-part2)
* [Combining Axum, Hyper, Tonic, and Tower for hybrid web/gRPC apps: Part 3](https://www.fpcomplete.com/blog/axum-hyper-tonic-tower-part3)
* [Combining Axum, Hyper, Tonic, and Tower for hybrid web/gRPC apps: Part 4](https://www.fpcomplete.com/blog/axum-hyper-tonic-tower-part4/)
* [First steps with an Arduino board and Rust](https://hoj-senna.github.io/HShhss/texts/001arduino1.html)
* [Full-stack Rust: A complete tutorial with examples](https://blog.logrocket.com/full-stack-rust-a-complete-tutorial-with-examples/)
* [Designing state machines in Rust](https://dev.to/senyeezus/designing-state-machines-in-rust-252k)
* [Optimizing a JavaScript library with WebAssembly, a failed attempt!](https://dev.to/antoniovdlc/optimising-a-javascript-library-with-webassembly-a-failed-attempt-48le)
* [series] [URL Shortener with Rust, Svelte, & AWS](https://dev.to/mileswatson/series/14520)
* [CN] [String in Rust](https://dev.to/daniel1in/string-in-rust-34d7)
* [video] [From Python to Rust - all episodes](https://www.youtube.com/watch?v=7odJDwhjCXQ&list=PLEIv4NBmh-GsWGE9mY3sF9c5lgh5Z_jLr&ab_channel=BedroomBuilds)
* [video] [1 Problem, 16 Programming Languages (C++ vs Rust vs Haskell vs Python vs APL...)](https://youtu.be/UVUjnzpQKUo)
* [video] [Explaing Rust Analyzer 15: Error Resilient Parsing](https://youtu.be/0HlrqwLjCxA)
* [video] [series] [Rust for Java Developers](https://youtube.com/playlist?list=PL7r-PXl6ZPcD63DS2djSiz4SlXkaTfobc)

### Miscellaneous
* [Do Developers Read Compiler Error Messages?](https://neverworkintheory.org/2021/09/20/do-developers-read-compiler-error-messages.html)
* [Alpine: System change proposal: Rust in main](https://www.reddit.com/r/rust/comments/pqs2dh/alpine_system_change_proposal_rust_in_main/)
* [Rust Adds Support For The Motorola M68000 Processors](https://www.phoronix.com/scan.php?page=news_item&px=Rust-Adds-Motorola-M68k)
* [Godbolt assembly exploring without crate limitations, in Visual Studio Code](https://saveriomiroddi.github.io/Rust-lulz-godbolt-assembly-exploring-without-crate-limitations-in-visual-studio-code)
* [What libraries do you miss from other languages?](https://www.reddit.com/r/rust/comments/pm4xe9/what_libraries_do_you_miss_from_other_languages/)
* [Rust for Rustaceans is off to the printer!](https://www.reddit.com/r/rust/comments/pp0nfa/rust_for_rustaceans_is_off_to_the_printer/)
* [GCC codegen now under rust-lang organization](https://www.reddit.com/r/rust/comments/ppf83d/gcc_codegen_now_under_rustlang_organization/)
* [Rust in Qemu, host support matrix](https://www.reddit.com/r/rust/comments/ppya5y/rust_in_qemu_host_support_matrix/)
* [Chrome - Borrowing Trouble: The Difficulty Of A C++ Borrow Checker](https://docs.google.com/document/u/1/d/e/2PACX-1vSt2VB1zQAJ6JDMaIA9PlmEgBxz2K5Tx6w2JqJNeYCy0gU4aoubdTxlENSKNSrQ2TXqPWcuwtXe6PlO/pub)
* [picture] [My Rust project was featured in one of Europe's biggest computer magazines (heise c't)](https://www.reddit.com/r/rust/comments/pskqhk/media_my_rust_project_was_featured_in_one_of/)
* [video] [Wrote a neat little maze solver. Largest solved so far is 125k x 125k. Here's a smaller 512x512](https://www.reddit.com/r/rust/comments/pl7n8a/media_wrote_a_neat_little_maze_solver_largest/?utm_source=share&utm_medium=web2x&context=3)

## Crate of the Week

This week's crate is [flowistry](https://github.com/willcrichton/flowistry), a VS code extension to visualize data flow in Rust code.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/963) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

256 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-09-13..2021-09-20

* [ARMv6K Nintendo 3DS Tier 3 target added](https://github.com/rust-lang/rust/pull/88529)
* [add initial support for m68k](https://github.com/rust-lang/rust/pull/88321)
* [disable the evaluation cache when in intercrate mode](https://github.com/rust-lang/rust/pull/88994)
* [use a separate interner type for `UniqueTypeId`](https://github.com/rust-lang/rust/pull/87867)
* [accept `m!{ .. }.method()` and `m!{ .. }?` statements](https://github.com/rust-lang/rust/pull/88690)
* [recover from `Foo(a: 1, b: 2)`](https://github.com/rust-lang/rust/pull/88729)
* [emit clearer diagnostics for parens around for loop heads](https://github.com/rust-lang/rust/pull/86422)
* [make diagnostics clearer for `?` operators](https://github.com/rust-lang/rust/pull/86382)
* [improve error message for type mismatch in generator arguments](https://github.com/rust-lang/rust/pull/88911)
* [improve error message for missing trait in trait impl](https://github.com/rust-lang/rust/pull/88894)
* [add a separate error for `dyn Trait` in `const fn`](https://github.com/rust-lang/rust/pull/89021)
* [suggest replacing an inexisting field for an unmentioned field](https://github.com/rust-lang/rust/pull/87960)
* [suggest better place to add call parentheses for method expressions wrapped in parentheses](https://github.com/rust-lang/rust/pull/89055)
* [suggest removing bad parens in `(recv.method)()`](https://github.com/rust-lang/rust/pull/88841)
* [suggest removing `#![feature]` for library features that have been stabilized](https://github.com/rust-lang/rust/pull/89012)
* [don't lint about missing code examples in derived traits](https://github.com/rust-lang/rust/pull/88735)
* [point at argument instead of call for their obligations](https://github.com/rust-lang/rust/pull/88719)
* [reuse existing shared `Lrc` for `MatchImpl` parent](https://github.com/rust-lang/rust/pull/89000)
* [fast reject for `NeedsNonConstDrop`](https://github.com/rust-lang/rust/pull/88965)
* [simplify lazy `DefPathHash` decoding by using an on-disk hash table](https://github.com/rust-lang/rust/pull/82183)
* [avoid codegen for `Result::into_ok` in `lang_start`](https://github.com/rust-lang/rust/pull/88988)
* [use `<[T; N]>::map` in `Sharded` instead of `SmallVec` and unsafe code](https://github.com/rust-lang/rust/pull/89069)
* [introduce a fast path that avoids the `debug_tuple` abstraction when deriving `Debug` for unit-like enum variants](https://github.com/rust-lang/rust/pull/88832)
* [make `UnsafeCell::get_mut` const](https://github.com/rust-lang/rust/pull/88722)
* [`const` `drop`](https://github.com/rust-lang/rust/pull/88558)
* [don't inline `OnceCell` initialization closures](https://github.com/rust-lang/rust/pull/89031)
* [allow `panic!("{}", computed_str)` in `const fn`](https://github.com/rust-lang/rust/pull/88954)
* [fix potential race in `AtomicU64` time monotonizer](https://github.com/rust-lang/rust/pull/89017)
* [futures: fix Unusable `Sink` implementation on `Scan`](https://github.com/rust-lang/futures-rs/pull/2499)
* [clippy: change `while_let_on_iterator` suggestion to use `by_ref()`](https://github.com/rust-lang/rust-clippy/pull/7690)
* [clippy: improve accuracy of `mut_key`](https://github.com/rust-lang/rust-clippy/pull/7640)
* [clippy: new lint: `same_name_method`](https://github.com/rust-lang/rust-clippy/pull/7653)

### Rust Compiler Performance Triage

A nice week: more improvements than regressions.

Triage done by **@pnkfelix**.
Revision range: [9f85cd6f2..7743c9f](https://perf.rust-lang.org/?start=9f85cd6f2ab2769c16e89dcdddb3e11d9736b351&end=7743c9fadd64886d537966ba224b9c20e6014a59&absolute=false&stat=instructions%3Au)

2 Regressions, 4 Improvements, 8 Mixed; ??? of them in rollups

44 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-09-21.md)

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Rust-lang crate ownership policy](https://github.com/rust-lang/rfcs/pull/3119)
* [Scrape code examples from examples/ directory for Rustdoc](https://github.com/rust-lang/rfcs/pull/3123)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: close] [Disable unused_must_use for statically known bools](https://github.com/rust-lang/rust/pull/88028)
* [disposition: merge] [Make #[derive(A, B, ...)] cfg-eval its input only for A, B, ... and stabilize feature(macro_attributes_in_derive_output)](https://github.com/rust-lang/rust/pull/87220)
* [disposition: merge] [Make `*const (), *mut ()` okay for FFI](https://github.com/rust-lang/rust/pull/84267)
* [disposition: merge] [Tracking issue Iterator map_while](https://github.com/rust-lang/rust/issues/68537)

### New RFCs

*No new RFCs were proposed this week.*

## Upcoming Events

### Online

* [September 15, 2021, Vancouver, BC, CA - Considering Rust - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsyccmbtb/)
* [September 16, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [September 18, 2021, Tokyo, JP - Rust.Tokyo 2021](https://rust.tokyo/)
* [September 28, 2021, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccmblc/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Kollider**

* [Junior Backend Engineer (Remote)](https://kollider.homerun.co/junior-backend-engineer/en)

**ChainSafe**
* [Protocol Engineer - Forest (Remote)](https://jobs.smartrecruiters.com/ChainSafeSystemsInc/743999769790643-protocol-engineer-forest-rust-)
* [Rust Engineer - Substrate (Remote)](https://jobs.smartrecruiters.com/ChainSafeSystemsInc/743999769885107-rust-engineer-substrate-)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> the strains of the project have hurt a lot of people over the years and I think maybe the only path to recovery involves getting some distance from it.

– [Graydon Hoare on twitter](https://twitter.com/graydon_pub/status/1437521319722029056)

Thanks to [mmmmib](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1107) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
