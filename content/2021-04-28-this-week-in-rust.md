Title: This Week in Rust 388
Number: 388
Date: 2021-04-28
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

No newsletters this week.

### Official
* [Announcing Rustup 1.24.0](https://blog.rust-lang.org/2021/04/27/Rustup-1.24.0.html)
* [Inside] [Congrats to compiler team member Aaron Hill](https://blog.rust-lang.org/inside-rust/2021/04/26/aaron-hill-compiler-team.html)
* [Foundation] [Introducing Josh Stone](https://foundation.rust-lang.org/posts/2021-04-22-introducing-josh-stone/)
* [Foundation] [Introducing Lars Bergstrom](https://foundation.rust-lang.org/posts/2021-04-22-introducing-lars-bergstrom/)

### Project/Tooling Updates
* [rust-analyzer Changelog #74](https://rust-analyzer.github.io/thisweek/2021/04/26/changelog-74.html)
* [IntelliJ Rust Changelog #146](https://intellij-rust.github.io/2021/04/26/changelog-146.html)
* [Lemmy Release v0.11.0](https://lemmy.ml/post/61856)
* [simdutf v0.1.1 - A small step for semver, one giant leap for performance](https://www.reddit.com/r/rust/comments/mz44xi/simdutf_v011_a_small_step_for_semver_one_giant/)
* [The playable demo of Outer Wonders, our cute, colorful and Rust-powered puzzle game, is live on itch.io for Windows and Linux! Thank you Rust community for creating such awesome tools!](https://www.reddit.com/r/rust/comments/mx3enm/the_playable_demo_of_outer_wonders_our_cute/)

### Observations/Thoughts
* [Rust Notebooks with Evcxr: Interview with David Lattimore](https://blog.abor.dev/p/evcxr)
* [Costs of iterators and Zero Cost Abstractions in Rust](https://github.com/mike-barber/rust-zero-cost-abstractions/blob/main/README.md) (with video)
* [Move Constructors in Rust: Is it possible?](https://mcyoung.xyz/2021/04/26/move-ctors/)
* [basedrop: a garbage collector for real-time audio in rust](https://glowcoil.com/posts/basedrop/)
* [Road to TurboWish; Part 1: Goals](http://blog.pnkfx.org/blog/2021/04/26/road-to-turbowish-part-1-goals/)
* [Road to TurboWish; Part 2: Stories](http://blog.pnkfx.org/blog/2021/04/27/road-to-turbowish-part-2-stories/)
* [The Pains of Path Parsing](https://www.fpcomplete.com/blog/pains-path-parsing/)
* [If you could re-design Rust from scratch today, what would you change?](https://www.reddit.com/r/rust/comments/my3ipa/if_you_could_redesign_rust_from_scratch_today/)
* [Red & blue functions are actually a good thing](https://blainehansen.me/post/red-blue-functions-are-actually-good/)

### Rust Walkthroughs
* [Creating privacy-preserving signatures in Rust with BBS+](https://github.com/ockam-network/ockam/blob/develop/implementations/rust/ockam/signature_bbs_plus/GUIDE.md)
* [Late Night Confessions — Building a Website Using Rust, Rocket, Diesel, and Askama - Part 2](https://medium.com/perimeterx/late-night-confessions-building-a-website-using-rust-rocket-diesel-and-askama-part-2-fc87c463e8f3)
* [How to Build a Custom Test Harness in Rust](https://www.fluvio.io/blog/2021/04/rust-custom-test-harness/)
* [Introduction to gRPC in Rust](https://romankudryashov.com/blog/2021/04/grpc-rust/)
* [Writing a Postgres SQL Pretty Printer in Rust: Part 2](https://blog.urth.org/2021/04/24/writing-a-postgres-sql-pretty-printer-in-rust-part-2/)
* [series] [A journey into Rust](https://dev.to/basman/series/12170)
* [ZH] [series] [Build GraphQL services based on Async Rust using tide + async-graphql + mongodb (基于 Async Rust 构建 GraphQL 服务，使用 tide + async-graphql + mongodb) - Part 2](https://blog.budshome.com/budshome/gou-jian-rust-yi-bu-graphql-fu-wu-:ji-yu-tide-+-async-graphql-+-mongodb(2)--cha-xun-fu-wu)
* [ZH] [series] [Build GraphQL services based on Async Rust using actix-web + async-graphql + rbatis + postgresql / mysql (基于 actix-web + async-graphql + rbatis + postgresql / mysql 构建异步 Rust GraphQL 服务) - Part 1](https://blog.budshome.com/budshome/ji-yu-actix-web-+-async-graphql-+-rbatis-+-postgresql---mysql-gou-jian-yi-bu-rust-graphql-fu-wu---qi-bu-ji-crate-xuan-ze)
* [video] [What is Rust and why is it important?](https://youtu.be/kdv1EBk6Xgc)
* [video] [Using Iterator::collect in Rust](https://youtu.be/ECwy6s_h7T8)
* [video] [RustConf 2020 - Macros for a More Productive Rust by jam1garner](https://youtu.be/HN6EUcnGN1s)

### Papers/Research Projects
* [A DSL embedded in Rust](https://kyleheadley.github.io/PHDWebsite/traitlang-IFL18-camready.pdf)
* [Energy Efficiency across Programming Languages](https://greenlab.di.uminho.pt/wp-content/uploads/2017/09/paperSLE.pdf)

### Miscellaneous
* [An Interview With Linus Torvalds: Linux and Git](https://www.tag1consulting.com/blog/interview-linus-torvalds-linux-and-git)
* [Programming languages: JavaScript has most developers but Rust is the fastest growing](https://www.zdnet.com/google-amp/article/programming-languages-javascript-has-most-developers-but-rust-is-the-fastest-growing/)
* [Parts of Tensorboard are being rewritten in Rust for a 100x to 400x speedup](https://www.reddit.com/r/rust/comments/mzlg5s/parts_of_tensorboard_are_being_rewritten_in_rust/)
* [Are we finally about to gain guaranteed Tail Calls in Rust?](https://www.reddit.com/r/rust/comments/my6k5i/are_we_finally_about_to_gain_guaranteed_tail/)
* [GitHub Actions best practices for Rust projects](https://www.fluvio.io/blog/2021/04/github-actions-best-practices/)

# Crate of the Week

This week's crate is [cargo-rr](https://github.com/danielzfranklin/cargo-rr), a cargo subcommand to use the time-traveling rr debugger on our code.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/905) for the nomination

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

[jsonschema-rs: User-defined validation for the format keyword](https://github.com/Stranger6667/jsonschema-rs/issues/158)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

350 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-04-19..2021-04-26

* [use LLVM's new saturating float-to-int intrinsics](https://github.com/rust-lang/rust/pull/84339)
* [enable sanitizers for `x86_64-unknown-linux-musl`](https://github.com/rust-lang/rust/pull/84126)
* [add coverage to `continue` statements](https://github.com/rust-lang/rust/pull/84295)
* [further split up `const_fn` feature flag](https://github.com/rust-lang/rust/pull/84310)
* [various const parameter defaults improvements](https://github.com/rust-lang/rust/pull/84299)
* [tweak trait not `use d suggestion](https://github.com/rust-lang/rust/pull/84499)
* [on stable, suggest removing `#![feature]` for features that have been stabilized](https://github.com/rust-lang/rust/pull/83722)
* [improve diagnostics for function passed when a type was expected](https://github.com/rust-lang/rust/pull/84520)
* [add suggestion to "use break" when attempting to implicit-break a loop](https://github.com/rust-lang/rust/pull/84516)
* [suggest `.as_ref()` on borrow error involving `Option`/`Result`](https://github.com/rust-lang/rust/pull/84353)
* [implement a lint that highlights all moves larger than a configured limit](https://github.com/rust-lang/rust/pull/83519)
* [introduce `CompileMonoItem` `DepNode`](https://github.com/rust-lang/rust/pull/84123)
* [cautiously add `IntoIterator` for arrays by value](https://github.com/rust-lang/rust/pull/84147)
* [stabilize `Duration::MAX`](https://github.com/rust-lang/rust/pull/84120)
* [stabilize `core::array::`{`from_ref`, `from_mut`} in 1.53.0](https://github.com/rust-lang/rust/pull/84105)
* [implement `TrustedRandomAccess` for `Take` iterator adapter](https://github.com/rust-lang/rust/pull/83990)
* [format `Struct { .. }` on one line even with `{:#?}`](https://github.com/rust-lang/rust/pull/84390)
* [added `CharIndices::offset` function](https://github.com/rust-lang/rust/pull/82585)
* [improve rebuilding behaviour of `BinaryHeap::retain`](https://github.com/rust-lang/rust/pull/78681)
* [hashbrown: add an `allocator()` getter to `HashMap` and `HashSet`](https://github.com/rust-lang/hashbrown/pull/257)
* [libz: disable forced zlib vendoring on musl](https://github.com/rust-lang/libz-sys/pull/78)
* [cargo: some changes to rustdoc fingerprint checking](https://github.com/rust-lang/cargo/pull/9404)
* [rustdoc: remove most fields from `ExternalCrate`](https://github.com/rust-lang/rust/pull/84457)
* [clippy: refactor MSRV aliases](https://github.com/rust-lang/rust-clippy/pull/7137)
* [clippy: finish MSRV for `cloned_instead_of_copied`](https://github.com/rust-lang/rust-clippy/pull/7134)
* [clippy: `manual_unwrap_or`: fix invalid code suggestion due to macro expansion](https://github.com/rust-lang/rust-clippy/pull/7136)
* [clippy: `cloned_instead_of_copied` MSRV](https://github.com/rust-lang/rust-clippy/pull/7129)
* [clippy: add `flat_map_option` lint](https://github.com/rust-lang/rust-clippy/pull/7101)
* [clippy: `unused_io_amount` detects `.read().ok()?`](https://github.com/rust-lang/rust-clippy/pull/7100)
* [clippy: add lint to check for boolean comparison in assert macro calls](https://github.com/rust-lang/rust-clippy/pull/7083)

## Rust Compiler Performance Triage

It's always nice to have a week without any regressions and 2 small improvements 🎉🎉.

Triage done by **@rylev**.
Revision range: [6df26f8..537544](https://perf.rust-lang.org/?start=6df26f897cffb2d86880544bb451c6b5f8509b2d&end=537544b1061467ee4b74ef7f552fab3d513e5caf&absolute=false&stat=instructions%3Au)

0 Regressions, 2 Improvements, 0 Mixed

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: Reserved prefixes in the 2021 edition](https://github.com/rust-lang/rfcs/pull/3101)
* [disposition: postpone] [Enum variant types](https://github.com/rust-lang/rfcs/pull/2593)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Add default search path to `Target::search()`](https://github.com/rust-lang/rust/pull/83800)
* [disposition: merge] [parser: Remove support for inner attributes on non-block expressions](https://github.com/rust-lang/rust/pull/83312)
* [disposition: merge] [Tracking Issue for {HashMap,BTreeMap}::into_{keys,values}](https://github.com/rust-lang/rust/issues/75294)

## New RFCs

* [Add bitfields support](https://github.com/rust-lang/rfcs/pull/3113)
* [A new prelude for the 2021 edition (trait-only edition)](https://github.com/rust-lang/rfcs/pull/3114)

# Upcoming Events

### Online
* [April 28, Online - Ockam Open Source Community Call - Live coding walkthrough of building end-to-end encrypted communication in Rust](https://github.com/ockam-network/ockam/discussions/1303)
* [May 3, 2021, Online - Cloud Native Rust Day](https://events.linuxfoundation.org/cloud-native-rust-day/)
* [May 4, 2021, Online - Cloud Native WASM Day](https://events.linuxfoundation.org/cloud-native-wasm-day/)
* [May 4, 2021, Dublin, IE - Rust Dublin May Remote Meetup - Rust Dublin](https://www.meetup.com/Rust-Dublin/events/277860218/)
* [May 4, Buffalo, NY, US - Buffalo Rust User Group, Tues May 4th - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/277402612/)
* [May 11, Seattle, WA, US - Monthly meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrycchbpb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Collabora**

* [Rust developer/promoter (Remote)](https://arbeitnow.com/view/rust-developerpromoter-collabora-36866)

**Paige**

* [Senior Software Engineer, Visualization (Remote, Europe)](https://boards.greenhouse.io/paige/jobs/5210311002)

**Confio GmbH**

* [Rust Engineer at Confio GmbH (Remote)](https://jobs.gohire.io/confio-gmbh-ggtjivjy/rust-engineer-39453/)

**CoBloX**

* [Software Engineer (Remote)](https://comit.network/blog/2021/03/01/we-are-hiring/)

**Gattaca**

* [Software Engineer - Rust & Python](https://gattaca.com/jobspec.html)

**Wallaroo**

* [Senior Client Solutions Engineer (Remote)](https://wallaroo.breezy.hr/p/47862ae31c91-senior-client-solutions-engineer-remote-even-after-covid)

**Parity Technologies**

* [Blockchain Developer - Consensus (Remote)](https://grnh.se/2dd887b13us)
* [Numerous other Rust engineering openings](https://www.parity.io/jobs/)

**Kollider**

* [Junior Backend Engineer (Remote)](https://kollider.homerun.co/junior-backend-engineer/en)
* [Senior Backend Engineer (Remote)](https://kollider.homerun.co/senior-backend-engineer/en)
* [DevOps Engineer (Remote)](https://kollider.homerun.co/devops-engineer/en)

**Chainflip**

* [Rust / C++ Backend Engineer (Berlin, DE)](https://angel.co/company/chainflip/jobs/1162345-rust-c-backend-engineer)
* [Security Engineer (Berlin, DE)](https://angel.co/company/chainflip/jobs/1293957-security-engineer)
* [Junior/Mid Backend Rust/C++ Developer (Melbourne, AU)](https://angel.co/company/chainflip/jobs/1305439-junior-mid-backend-rust-c-developer)

**Kraken**

* [Several Rust Engineering Positions](https://jobs.lever.co/kraken?team=Engineering)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> this error message is UNREAL

– [Ash 2X3 on Twitter](https://twitter.com/ash2x3/status/1384986537167892483)

Thanks to [Nixon Enraght-Moony](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1046) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/n0t8mm/this_week_in_rust_388/)</small>
