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

No papers/research projects this week.

### Official

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts
* [Rust Notebooks with Evcxr: Interview with David Lattimore](https://blog.abor.dev/p/evcxr)
* [Costs of iterators and Zero Cost Abstractions in Rust](https://github.com/mike-barber/rust-zero-cost-abstractions/blob/main/README.md) (with video)

### Rust Walkthroughs
* [Creating privacy-preserving signatures in Rust with BBS+](https://github.com/ockam-network/ockam/blob/develop/implementations/rust/ockam/signature_bbs_plus/GUIDE.md)
* [Late Night Confessions — Building a Website Using Rust, Rocket, Diesel, and Askama - Part 2](https://medium.com/perimeterx/late-night-confessions-building-a-website-using-rust-rocket-diesel-and-askama-part-2-fc87c463e8f3)
* [ZH] [series] [Build GraphQL services based on Async Rust using tide + async-graphql + mongodb (基于 Async Rust 构建 GraphQL 服务，使用 tide + async-graphql + mongodb) - Part 2](https://blog.budshome.com/budshome/gou-jian-rust-yi-bu-graphql-fu-wu-:ji-yu-tide-+-async-graphql-+-mongodb(2)--cha-xun-fu-wu)
* [ZH] [series] [Build GraphQL services based on Async Rust using actix-web + async-graphql + rbatis + postgresql / mysql (基于 actix-web + async-graphql + rbatis + postgresql / mysql 构建异步 Rust GraphQL 服务) - Part 1](https://blog.budshome.com/budshome/ji-yu-actix-web-+-async-graphql-+-rbatis-+-postgresql---mysql-gou-jian-yi-bu-rust-graphql-fu-wu---qi-bu-ji-crate-xuan-ze)
* [video] [What is Rust and why is it important?](https://youtu.be/kdv1EBk6Xgc)

### Miscellaneous

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

*No calls for participation this week*

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

* [try_trait_v2: A new design for the ? desugaring](https://github.com/rust-lang/rfcs/pull/3058)


## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: Reserved prefixes in the 2021 edition](https://github.com/rust-lang/rfcs/pull/3101)
* [disposition: merge] [add const-ub RFC](https://github.com/rust-lang/rfcs/pull/3016)
* [disposition: merge] [Target tier policy](https://github.com/rust-lang/rfcs/pull/2803)
* [disposition: postpone] [RFC: Custom DSTs](https://github.com/rust-lang/rfcs/pull/2594)
* [disposition: postpone] [Enum variant types](https://github.com/rust-lang/rfcs/pull/2593)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Cautiously add IntoIterator for arrays by value](https://github.com/rust-lang/rust/pull/84147)
* [disposition: merge] [Stabilize Duration::MAX](https://github.com/rust-lang/rust/pull/84120)
* [disposition: merge] [Stabilize `impl From<[(K, V); N]`> for HashMap`](https://github.com/rust-lang/rust/pull/84111)
* [disposition: merge] [Allow setting `target_family` to multiple values, and implement `target_family="wasm"`](https://github.com/rust-lang/rust/pull/84072)
* [disposition: close] [Exiting a process calls exit() which isn’t thread-safe](https://github.com/rust-lang/rust/issues/83994)
* [disposition: merge] [Stabilize `:pat_param` but leave :pat2021 gated](https://github.com/rust-lang/rust/pull/83386)
* [disposition: merge] [parser: Remove support for inner attributes on non-block expressions](https://github.com/rust-lang/rust/pull/83312)
* [disposition: merge] [Update BARE_TRAIT_OBJECT and ELLIPSIS_INCLUSIVE_RANGE_PATTERNS to errors in Rust 2021](https://github.com/rust-lang/rust/pull/83213)
* [disposition: merge] [Tracking Issue for vec_extend_from_within](https://github.com/rust-lang/rust/issues/81656)
* [disposition: merge] [Tracking Issue for 'ordering helpers'](https://github.com/rust-lang/rust/issues/79885)
* [disposition: merge] [Tracking issue for array::from_ref and array::from_mut](https://github.com/rust-lang/rust/issues/77101)
* [disposition: merge] [Tracking Issue for {HashMap,BTreeMap}::into_{keys,values}](https://github.com/rust-lang/rust/issues/75294)
* [disposition: merge] [Tracking issue for x86 bittest intrinsics](https://github.com/rust-lang/rust/issues/59414)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [April 21, Vancouver, BC, CA - Rust Study/Hack/Hang-out night - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/npqfbsyccgbcc/)
* [April 27, Berlin, DE - Rust and Tell - Rust Berlin](https://www.meetup.com/Rust-Berlin/events/277590271)
* [April 27, London, UK - LDN Virtual Talks Apr 2021 - Red Badger Takeover - Rust London User Group](https://www.meetup.com/Rust-London-User-Group/events/277520645/)
* [April 27, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccgbkc/)
* [April 28, Online - Ockam Open Source Community Call - Live coding walkthrough of building end-to-end encrypted communication in Rust](https://github.com/ockam-network/ockam/discussions/1303)
* [May 3, 2021, Online - Cloud Native Rust Day](https://events.linuxfoundation.org/cloud-native-rust-day/)
* [May 4, 2021, Online - Cloud Native WASM Day](https://events.linuxfoundation.org/cloud-native-wasm-day/)
* [May 4, Buffalo, NY, US - Buffalo Rust User Group, Tues May 4th - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/277402612/)

### Europe
* [April 21, Moscow, RU - Monthly Meetup - Rust Moscow](https://www.meetup.com/ru-RU/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/277259838/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Paige**

* [Senior Software Engineer, Visualization (Remote, Europe)](https://boards.greenhouse.io/paige/jobs/5210311002)

**Parity Technologies**

* [Blockchain Developer - Consensus (Remote)](https://grnh.se/2dd887b13us)
* [Numerous other Rust engineering openings](https://www.parity.io/jobs/)

**Kollider**

* [Junior Backend Engineer (Remote)](https://kollider.homerun.co/junior-backend-engineer/en)
* [Senior Backend Engineer (Remote)](https://kollider.homerun.co/senior-backend-engineer/en)
* [DevOps Engineer (Remote)](https://kollider.homerun.co/devops-engineer/en)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> this error message is UNREAL

– [Ash 2X3 on Twitter](https://twitter.com/ash2x3/status/1384986537167892483)

Thanks to [Nixon Enraght-Moony](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1046) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
