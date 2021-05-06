Title: This Week in Rust 389
Number: 389
Date: 2021-05-05
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

### Project/Tooling Updates
* [One Third Of 2021](https://isomorphicdb.io/blog/2021/05/05/One-Third-of-2021/)

* [This Week In TensorBase 1](https://tensorbase.io/thisweek/2021-05-01-tw_1/)
* [gfx/wgpu releases 0.8](https://gfx-rs.github.io/2021/04/30/release-0.8.html)
* [Last Month in Flott (Motion Control Toolkit in Rust) - May 2021](https://flott-motion.org/news/last-month-in-flott-may-2021/)

### Observations/Thoughts
[Making Generative Art with Rust: interview with Alexis André](https://blog.abor.dev/p/making-generative-art-with-rust)

[Compilers as Teachers](https://ferrous-systems.com/blog/compilers-as-teachers/)

### Rust Walkthroughs
* [Late Night Confessions — Building a Website Using Rust, Rocket, Diesel, and Askama - Part 3](https://medium.com/perimeterx/late-night-confessions-building-a-website-using-rust-rocket-diesel-and-askama-part-3-3f9b7d511bde)

* [ZH] [series] [Build GraphQL services based on Async Rust using tide + async-graphql + mongodb (基于 Async Rust 构建 GraphQL 服务，使用 tide + async-graphql + mongodb) - Part 3](https://blog.budshome.com/budshome/gou-jian-rust-yi-bu-graphql-fu-wu-:ji-yu-tide-+-async-graphql-+-mongodb(3)--zhong-gou)
* [ZH] [series] [Build GraphQL services based on Async Rust using actix-web + async-graphql + rbatis + postgresql / mysql (基于 actix-web + async-graphql + rbatis + postgresql / mysql 构建异步 Rust GraphQL 服务) - Part 2](https://blog.budshome.com/budshome/ji-yu-actix-web-+-async-graphql-+-rbatis-+-postgresql---mysql-gou-jian-yi-bu-rust-graphql-fu-wu-(2)---cha-xun-fu-wu)


* Make a [Counter](https://kas-gui.github.io/tutorials/counter.html) and [Calculator](https://kas-gui.github.io/tutorials/calculator.html) with KAS GUI

### Papers/Research Projects

### Miscellaneous

[Using GDB and `defmt` to debug embedded programs](https://ferrous-systems.com/blog/gdb-and-defmt/)

# Crate of the Week

This week's crate is [display_utils](https://docs.rs/display_utils), a library with `Display`able structs to make string manipulation easier.

Thanks to [kangalioo](https://users.rust-lang.org/t/crate-of-the-week/2704/908) for the nomination

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

322 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-04-26..2021-05-03

* [adds feature-gated `#[no_coverage]` function attribute, to fix derived Eq `0` coverage](https://github.com/rust-lang/rust/pull/84562)
* [give a better error when `std` or `core` are missing](https://github.com/rust-lang/rust/pull/84450)
* [suggestion for unit enum variant when matched with a pattern](https://github.com/rust-lang/rust/pull/84818)
* [avoid generating `QueryMap::extend` for each key type](https://github.com/rust-lang/rust/pull/84805)
* [remove dead code in `rustc_session::Options`](https://github.com/rust-lang/rust/pull/84802)
* [move `iter_results` to `dyn FnMut` rather than a generic](https://github.com/rust-lang/rust/pull/84719)
* [miri: throw UB if f*_fast intrinsic called with non-finite value](https://github.com/rust-lang/miri/pull/1785)
* [miri: use `harness = false` instead of `#![feature(custom_test_frameworks)]`](https://github.com/rust-lang/miri/pull/1784)
* [LLVM: don't merge thread_local constants with non-thread_local constants](https://github.com/rust-lang/llvm-project/pull/105)
* [be stricter about rejecting LLVM reserved registers in asm!](https://github.com/rust-lang/rust/pull/84658)
* [stabilize `vec_extend_from_within`](https://github.com/rust-lang/rust/pull/84642)
* [stabilize `ordering_helpers`](https://github.com/rust-lang/rust/pull/84523)
* [override `clone_from` method for `PathBuf` and `OsString`](https://github.com/rust-lang/rust/pull/84615)
* [simplify `Mutex::into_inner`](https://github.com/rust-lang/rust/pull/84650)
* [`i8` and `u8::to_string()` specialisation](https://github.com/rust-lang/rust/pull/82576)
* [reuse `sys::unix::cmath` on other platforms](https://github.com/rust-lang/rust/pull/84522)
* [add `ErrorKind::OutOfMemory`](https://github.com/rust-lang/rust/pull/84744)
* [add `std::os::unix::fs::chroot` to change the root directory of the current process](https://github.com/rust-lang/rust/pull/84716)
* [inline most raw socket, fd and handle conversions](https://github.com/rust-lang/rust/pull/84541)
* [socket2: allow for niche optimization on Unix platforms](https://github.com/rust-lang/socket2/pull/222)
* [regex: fix lazy DFA false quits on ASCII text](https://github.com/rust-lang/regex/pull/768)
* [regex: update to latest memchr + upgrade to Rust 2018 + bump MSRV to Rust 1.41](https://github.com/rust-lang/regex/pull/767)
* [cargo: add report subcommand](https://github.com/rust-lang/cargo/pull/9438)
* [cargo: show transfer rate when fetching/updating registry index](https://github.com/rust-lang/cargo/pull/9395)
* [rustdoc: remove unnecessary `provided_trait_methods` field from Impl](https://github.com/rust-lang/rust/pull/84463)
* [rustdoc: shrink `doctree::Module`](https://github.com/rust-lang/rust/pull/84763)
* [datafrog: micro-optimize `binary_search`](https://github.com/rust-lang/datafrog/pull/30)
* [clippy: fix a false-positive inside const fn in `comparison_chain`](https://github.com/rust-lang/rust-clippy/pull/7118)
* [clippy: `implicit_return` improvements](https://github.com/rust-lang/rust-clippy/pull/6951)
* [clippy: `while_immutable_cond`: check condition for mutation](https://github.com/rust-lang/rust-clippy/pull/7144)
* [clippy: fix false negative in `iter_cloned_collect` with a large array](https://github.com/rust-lang/rust-clippy/pull/7138)

## Rust Compiler Performance Triage

Quiet week, no significant changes.

Triage done by **@simulacrum**.
Revision range: [537544..7a0f178](https://perf.rust-lang.org/?start=537544b1061467ee4b74ef7f552fab3d513e5caf&end=7a0f1781d04662041db5deaef89598a8edd53717&absolute=false&stat=instructions%3Au)

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-05-04.md).

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
* [May 11, Saarbücken, Saarland, DE - Meetup: 11u16 (virtual) - Rust Saar](https://www.meetup.com/de-DE/Rust-Saar/events/277607432/)
* [May 15 - June 7, Online - Solana Season Hackathon - Registration open now](https://twitter.com/solana/status/1387411221717176323?s=20)
* [May 20, 2021, Online - Go vs Rust | Round table discussion](https://rustlab.it/en/rust-vs-go/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Paige**

* [Senior Software Engineer, Visualization (Remote, Europe)](https://boards.greenhouse.io/paige/jobs/5210311002)

**Netlify**

* [Senior Backend Engineer (Go/Rust) (Remote or San Francisco, CA, US)](https://boards.greenhouse.io/netlify/jobs/5054144002)

**e.ventures**

* [Rust backend engineer (Remote, the Americas)](https://old.reddit.com/r/rust/comments/mfstaz/official_rrust_whos_hiring_thread_for_jobseekers/gspq9v1/)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Using R or Numpy is like driving around in a sports car. You just turn the wheel, press the pedals, and burn rubber. Rust (and other systems languages) are like getting a spaceship. You can go places and do things that you never dreamt of in a car. They are harder to pilot, but the possibilities seem unlimited! With the Rust ecosystem still in development, it feels like parts of your spaceship come in boxes of parts labeled "some assembly required".

– [Erik Rose on rust-users](https://users.rust-lang.org/t/rust-for-data-first-problems/58887/16)

Thanks to [Phlopsi](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1047) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
