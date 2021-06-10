Title: This Week in Rust 394
Number: 394
Date: 2021-06-09
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Official
* [Announcing Rustup 1.24.3](https://blog.rust-lang.org/2021/06/08/Rustup-1.24.3.html)

### Newsletters
* [This Month in Rust OSDev (May 2021)](https://rust-osdev.com/this-month/2021-05/)

### Project/Tooling Updates
* [Filecoin Forest update](https://medium.com/chainsafe-systems/back-into-the-forest-983a4344ffe9)
* [Mina Protocol update](https://medium.com/chainsafe-systems/realizing-the-mina-vision-in-rust-453f6f522205)
* [rust-analyzer Changelog #80](https://rust-analyzer.github.io/thisweek/2021/06/07/changelog-80.html)
* [IntelliJ Rust Changelog #148](https://intellij-rust.github.io/2021/06/07/changelog-148.html)
* [Rust/C++ Interop in the Android Platform](https://security.googleblog.com/2021/06/rustc-interop-in-android-platform.html)
* [Rocket v0.5 Release Candidate](https://rocket.rs/v0.5-rc/news/2021-06-09-version-0.5-rc.1/)
* [This Week In TensorBase 6](https://tensorbase.io/thisweek/2021-06-09-tw_6/)
* [Dotenv-linter v3.1.0: Overview of key changes](https://dotenv-linter.github.io/#/whats_new/v310)

### Observations/Thoughts
* [Untapped potential in Rust's type system](https://www.jakobmeier.ch/blogging/Untapped-Rust.html)
* [Idiomatic Rust - Binary Search Extended](https://c-hirsch.de/2020-05-30-idiomatic-rust-binary-search-extended/)
* [Designing Rust bindings for REST APIs](https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is)
* [Rust zero-cost abstractions in action](https://medium.com/ingeniouslysimple/rust-zero-cost-abstraction-in-action-9e4e2f8bf5a)
* [Translating Quake 3 into Rust](https://immunant.com/blog/2020/01/quake3/)
* [First impressions of Rust programming on Solana](https://brson.github.io/2021/06/08/rust-on-solana)
* [Optimizing Pairing-Based Cryptography: Montgomery Arithmetic in Rust](https://research.nccgroup.com/2021/06/09/optimizing-pairing-based-cryptography-montgomery-arithmetic-in-rust/)
* [My second cup of Rust](https://blog.frankel.ch/start-rust/2/)
* [A Goose In The Clouds: Load Testing At Scale](https://www.tag1consulting.com/blog/goose-clouds-load-testing-scale)
* [Walking through "The Java Tutorials" with Rust](https://rust-java-tutorials.netlify.app/blog/)
* [Behavior inheritance in Rust](https://abadcafe.wordpress.com/2021/01/08/behavior-inheritance-in-rust/)
* [audio] [Building with Rust: Ralf Jung on GhostCell and working as a PL researcher](https://anchor.fm/building-with-rust/episodes/Building-with-Rust-Ralf-Jung-on-GhostCell-and-Working-as-a-PL-Researcher-e12auje)

### Rust Walkthroughs
* [Rust Derive Macro Guide](https://github.com/imbolc/rust-derive-macro-guide)
* [Calibration From Scratch Using Rust: Part 1 of 3](https://www.tangramvision.com/blog/calibration-from-scratch-using-rust-part-1-of-3)
* [Calibration From Scratch Using Rust: Part 2 of 3](https://www.tangramvision.com/blog/calibration-from-scratch-using-rust-part-2-of-3)
* [Calibration From Scratch Using Rust: Part 3 of 3](https://www.tangramvision.com/blog/calibration-from-scratch-using-rust-part-3-of-3)
* [From Julia to Rust](https://miguelraz.github.io/blog/juliatorust/)
* [Rust from a JavaScript perspective](https://blogs.harvard.edu/kapolos/rust-from-a-javascript-perspective/)
* [!#[no_std] with WASI is more complicated than I thought it would be](https://dev.to/thepuzzlemaker/nostd-with-wasi-is-more-complicated-than-i-thought-it-would-be-14j7)
* [Rust - What made it "click" for me (Ownership and memory models)](https://deavid.wordpress.com/2021/06/06/rust-what-made-it-click-for-me-ownership-memory-internals/)
* [Creating an NPM package written in Rust](https://popcornpaws.medium.com/creating-an-npm-package-written-in-rust-ce02f7c55458)
* [Rise and Shine: Putting the nRF52840 to sleep, and waking it back up](https://tweedegolf.nl/blog/57/rise-and-shine-putting-the-nrf52840-to-sleep-and-waking-back-up)
* [Iterator producing iterator in Rust is really helpful.](https://blog.knoldus.com/iterator-producing-iterator-in-rust-is-really-helpful/)
* [Speed up your Rust CI with cache image and Buildkit](https://blog.erebe.dev/blog/speed-up-your-ci-with-buildkit/)
* [ZH] [series] [Build front-end web apps with Yew and WebAssembly in Rust -part 1: crates (Rust 和 Wasm 的融合，使用 yew 构建 WebAssembly 标准的 web 前端（1）- 起步及 crate 选择)](https://blog.budshome.com/budshome/rust-he-wasm-de-rong-he-,shi-yong-yew-gou-jian-webassembly-biao-zhun-de-web-qian-duan-(1)--qi-bu-ji-crate-xuan-ze)
* [ZH] [series] [Build front-end web apps with Yew and WebAssembly in Rust -part 2: Components & Routers (Rust 和 Wasm 的融合，使用 yew 构建 WebAssembly 标准的 web 前端（2）- 组件和路由)](https://blog.budshome.com/budshome/rust-he-wasm-de-rong-he-,shi-yong-yew-gou-jian-webassembly-biao-zhun-de-web-qian-duan-(2)--zu-jian-he-lu-you)
* [video] [Rust Beginners 5 - Tuples](https://youtu.be/gZMet9Vi7_A)

### Research
* [collection] [Automatic Rust verification tools (2021)](https://alastairreid.github.io/automatic-rust-verification-tools-2021/)

### Miscellaneous
* [QUIC Version 1 is live on Cloudflare](https://blog.cloudflare.com/quic-version-1-is-live-on-cloudflare/)
* [What are the most "professional" crates?](https://www.reddit.com/r/rust/comments/nsvyxq/what_are_the_most_professional_crates/)
* [What's your favourite under-rated Rust crate and why?](https://www.reddit.com/r/rust/comments/nuq1ix/whats_your_favourite_underrated_rust_crate_and_why/)
* [It's not much, but I graduated from middle-school today with Rust as my language of choice](https://www.reddit.com/r/rust/comments/nrin1u/its_not_much_but_i_graduated_from_middleschool/)
* [From Julia to Rust](https://miguelraz.github.io/blog/juliatorust/) 

## Crate of the Week

This week's crate is [cargo-sort](https://github.com/DevinR528/cargo-sort), a cargo subcommand to sort your `Cargo.toml`'s dependencies and workspace members.

Thanks to [jplatte](https://users.rust-lang.org/t/crate-of-the-week/2704/921) for the nomination

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

* [Backroll-rs is looking for contributors](https://www.reddit.com/r/rust/comments/npnl1p/help_wanted_with_backrollrs_new_networking_library/)

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

267 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-05-31..2021-06-07

* [BPF target support](https://github.com/rust-lang/rust/pull/79608)
* [support for force-warns](https://github.com/rust-lang/rust/pull/85788)
* [improve debugging experience for enums on windows-msvc](https://github.com/rust-lang/rust/pull/85292)
* [parser: ensure that all nonterminals have tokens after parsing](https://github.com/rust-lang/rust/pull/84995)
* [don't suggest unsized indirection in where-clauses](https://github.com/rust-lang/rust/pull/85979)
* [rustc: allow safe `#[target_feature]` on wasm](https://github.com/rust-lang/rust/pull/84988)
* [always go through the `expn_that_defined` query](https://github.com/rust-lang/rust/pull/86002)
* [perf: miscellaneous inlining improvements](https://github.com/rust-lang/rust/pull/85892)
* [perf: only compute the trait map once](https://github.com/rust-lang/rust/pull/85905)
* [stabilize `vecdeque_binary_search`](https://github.com/rust-lang/rust/pull/83362)
* [update standard library for `IntoIterator` implementation of arrays](https://github.com/rust-lang/rust/pull/85930)
* [clippy: don't warn about `cfg!(..)` as a constant in assertions](https://github.com/rust-lang/rust-clippy/pull/7319)
* [clippy: fix `needless_collect` with binding shadowing](https://github.com/rust-lang/rust-clippy/pull/7289)
* [clippy: add lint `manual_str_repeat`](https://github.com/rust-lang/rust-clippy/pull/7265)

### Rust Compiler Performance Triage


Some good improvements, and a few regressions. No large changes.

Triage done by **@simulacrum**.
Revision range: [1160cf..a50d721](https://perf.rust-lang.org/?start=1160cf864f2a0014e3442367e1b96496bfbeadf4&end=a50d72158e08e02cfc051b863017bdbd2c45b637&absolute=false&stat=instructions%3Au)

3 Regressions, 3 Improvements, 1 Mixed; 1 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-06-08.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: 2021 Edition](https://github.com/rust-lang/rfcs/pull/3085)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: Supertrait item shadowing](https://github.com/rust-lang/rfcs/pull/2845)
* [disposition: merge] [Type-changing struct update syntax](https://github.com/rust-lang/rfcs/pull/2528)
* [disposition: merge] [RFC: Introduce concat_bytes!() to join [u8] and byte str analogous to concat! for str](https://github.com/rust-lang/rfcs/pull/2509)
* [disposition: merge] [RFC: Overconstraining and omitting unsafe in impls of unsafe trait methods](https://github.com/rust-lang/rfcs/pull/2316)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Re-add support for parsing (and pretty-printing) inner-attributes in match body](https://github.com/rust-lang/rust/pull/85193)

### New RFCs

* [Switch from travis to github actions.](https://github.com/rust-lang/rfcs/pull/3136)

## Upcoming Events

### Online

* [June 8, 2021, Seattle, WA, US - Monthly meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccjblb/)
* [June 10, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [June 15, 2021, Washington, DC, US - In-kernel, fast-path packet processing with AF_XDP - Rust DC](https://www.meetup.com/RustDC/events/vdhxgsyccjbtb)
* [June 16, 2021, Vancouver, BC, CA - Rust in Mozilla's Data Platform - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/fqpkjsyccjbvb/)
* [June 17, 2021, Denver, CO, US - Learning Rust as a Python/Javascript developer by Juhis - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/277575285/)

### North America

* [June 9, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/qxqdgryccjbmb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Tweede golf**

* [Lead Developer Embedded Rust (Nijmegen, NL)](https://tweedegolf.nl/vacatures/2/lead-developer-embedded-rust)

**NZXT**

* [Senior Software Engineer for CAM (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=259)
* [Senior Software Engineer for Streaming Software (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=317)

**Kollider**

* [Junior Backend Engineer (Remote)](https://kollider.homerun.co/junior-backend-engineer/en)
* [Senior Backend Engineer (Remote)](https://kollider.homerun.co/senior-backend-engineer/en)
* [DevOps Engineer (Remote)](https://kollider.homerun.co/devops-engineer/en)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> As the tradeoffs in software engineering change over time, so does the ideal solution. Some 40 years ago when the first C standards were written down, by people no less competent than those that work on Rust today, the design of the language and the list of behaviours not defined likely made much more sense in context of back then than they do right now. It is not all that unlikely that some years down the line the choices made by Rust won't make all that much of sense as they do today, too.

– [Simonas on rust-internals](https://users.rust-lang.org/t/why-deference-maybeuninit-unint-as-mut-ptr-is-safe/60344/19)

Thanks to [Kill The Mule](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1055) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
