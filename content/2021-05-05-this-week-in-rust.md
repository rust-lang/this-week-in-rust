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

### Official
* [Announcing Rustup 1.24.1](https://blog.rust-lang.org/2021/04/29/Rustup-1.24.1.html)
* [Inside] [Rustup 1.24.0 release incident report for 2021-04-27](https://blog.rust-lang.org/inside-rust/2021/04/28/rustup-1.24.0-incident-report.html)
* [Inside] [1.52.0 pre-release testing](https://blog.rust-lang.org/inside-rust/2021/05/04/1.52.0-prerelease.html)
* [Inside] [Core Team Update: May 2021](https://blog.rust-lang.org/inside-rust/2021/05/04/core-team-update.html)
* [Foundation] [Q12021 Membership Update](https://foundation.rust-lang.org/posts/2021-04-29-membership-update/)

### Newsletters
* [This Month in Rust OSDev (April 2021)](https://rust-osdev.com/this-month/2021-04/)

### Project/Tooling Updates
* [One Third Of 2021](https://isomorphicdb.io/blog/2021/05/05/One-Third-of-2021/)
* [This Week In TensorBase 1](https://tensorbase.io/thisweek/2021-05-01-tw_1/)
* [gfx/wgpu releases 0.8](https://gfx-rs.github.io/2021/04/30/release-0.8.html)
* [Last Month in Flott (Motion Control Toolkit in Rust) - May 2021](https://flott-motion.org/news/last-month-in-flott-may-2021/)
* [rust-analyzer Changelog #75](https://rust-analyzer.github.io/thisweek/2021/05/03/changelog-75.html)
* [GCC Rust Monthly Report #5 April 2021](https://thephilbert.io/2021/05/03/gcc-rust-monthly-report-5-april-2021/)
* [RustCrypto Release Announcements (`aead`, `cipher`, `crypto`, `elliptic-curve`, `ecdsa`, and more)](https://users.rust-lang.org/t/rustcrypto-release-announcements/59149)

### Observations/Thoughts
* [How Rust makes Rayon's data parallelism magical](https://developers.redhat.com/blog/2021/04/30/how-rust-makes-rayons-data-parallelism-magical/)
* [Making Generative Art with Rust](https://blog.abor.dev/p/making-generative-art-with-rust)
* [An Incomplete Explanation of the Proc Macro That Saved Me 4000 Lines of Rust](https://mbuffett.com/posts/incomplete-macro-walkthrough/)
* [The most underrated but useful Rust standard library type](https://dev.to/thepuzzlemaker/the-most-underrated-but-useful-rust-standard-library-type-59b1)
* [Guaranteed unique; Or, why dogfooding can be taxing.](https://dev.to/ecton/guaranteed-unique-or-why-dogfooding-can-be-taxing-2gcn)
* [The Great Rewriting in Rust](https://deprogrammaticaipsum.com/the-great-rewriting-in-rust/)
* [A story about async Rust and using !Send types](https://procmarco.netlify.app/blog/2021-05-04-a-story-about-async-rust-and-using-send-types/)
* [Compilers as Teachers](https://ferrous-systems.com/blog/compilers-as-teachers/)
* [Making Generative Art with Rust: interview with Alexis André](https://blog.abor.dev/p/making-generative-art-with-rust)
* [Building on the Shoulders of Giants: Combining TensorFlow and Rust](https://www.crowdstrike.com/blog/how-crowdstrike-combines-tensorflow-and-rust-for-performance/)
* [Let's make everything iterable - Iterate over pagination result in the Rest API](https://0x709394.me/Let's-make%20everything%20iterable)
* [video] [Interview with Niko Matsakis, Co-lead of the Rust language team](https://youtu.be/alD0l_8W9Sc)

### Rust Walkthroughs
* [Introduction to gRPC in Rust](https://dev.to/rkudryashov/introduction-to-grpc-in-rust-4dgg)
* [I2C on the Pinephone](https://dev.to/pcvonz/i-c-on-the-pinephone-5090)
* [Late Night Confessions - Building a Website Using Rust, Rocket, Diesel, and Askama - Part 3](https://dev.to/pxjohnny/late-night-confessions-building-a-website-using-rust-rocket-diesel-and-askama-part-3-46i9)
* [Rust ownership and borrows - Fighting the borrow-checker](https://dev.to/daaitch/rust-ownership-and-borrows-fighting-the-borrow-checker-4ea3)
* [Running Rust on Android](https://blog.svgames.pl/article/running-rust-on-android)
* [Using GDB and defmt to debug embedded programs](https://ferrous-systems.com/blog/gdb-and-defmt/)
* [Data Modelling in Rust](https://phazer99.blogspot.com/2021/05/data-modelling-in-rust.html)
* [Data Modelling in Rust Continued](https://phazer99.blogspot.com/2021/05/data-modelling-in-rust-continued.html)
* [Embedding Rust code in Java Jar for distribution](https://www.fluvio.io/blog/2021/05/java-client/)
* [Late Night Confessions — Building a Website Using Rust, Rocket, Diesel, and Askama - Part 3](https://medium.com/perimeterx/late-night-confessions-building-a-website-using-rust-rocket-diesel-and-askama-part-3-3f9b7d511bde)
* Make a [Counter](https://kas-gui.github.io/tutorials/counter.html) and [Calculator](https://kas-gui.github.io/tutorials/calculator.html) with KAS GUI
* [ZH] [series] [Build GraphQL services based on Async Rust using tide + async-graphql + mongodb (基于 Async Rust 构建 GraphQL 服务，使用 tide + async-graphql + mongodb) - Part 3](https://blog.budshome.com/budshome/gou-jian-rust-yi-bu-graphql-fu-wu-:ji-yu-tide-+-async-graphql-+-mongodb(3)--zhong-gou)
* [ZH] [series] [Build GraphQL services based on Async Rust using actix-web + async-graphql + rbatis + postgresql / mysql (基于 actix-web + async-graphql + rbatis + postgresql / mysql 构建异步 Rust GraphQL 服务) - Part 2](https://blog.budshome.com/budshome/ji-yu-actix-web-+-async-graphql-+-rbatis-+-postgresql---mysql-gou-jian-yi-bu-rust-graphql-fu-wu-(2)---cha-xun-fu-wu)
* [video] [Crust of Rust: Dispatch and Fat Pointers](https://youtu.be/xcygqF5LVmM)
* [video] [Ockam | OSS Community Call | April 2021](https://www.youtube.com/watch?v=ndujK8lTTVY)

### Papers/Research Projects
* [New Variant of Buer Loader Written in Rust](https://www.proofpoint.com/us/blog/threat-insight/new-variant-buer-loader-written-rust)
* [Is it possible to write overhead-free cyclic data-structures in safe, stable Rust?](https://www.reddit.com/r/rust/comments/n420cg/is_it_possible_to_write_overheadfree_cyclic/)

### Miscellaneous
* [Rust's Most Unrecognized Contributor](https://brson.github.io/2021/05/02/rusts-most-unrecognized-contributor)
* [Facebook Joins the Rust Foundation](https://developers.facebook.com/blog/post/2021/04/29/facebook-joins-rust-foundation/)
* [A brief history of Rust at Facebook](https://engineering.fb.com/2021/04/29/developer-tools/rust/)
* [rustc performance improvement from rust 1.46 to 1.51](https://www.reddit.com/r/rust/comments/n2lh7z/rustc_performance_improvement_from_rust_146_to_151/)
* [Microsoft joins Bytecode Alliance to advance WebAssembly - aka the thing that lets you run compiled C/C++/Rust code in browsers](https://www.theregister.com/2021/04/28/microsoft_bytecode_alliance/)
* [Using GDB and `defmt` to debug embedded programs](https://ferrous-systems.com/blog/gdb-and-defmt/)


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

* [paru - Add -P --stats](https://github.com/Morganamilo/paru/issues/357)

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

* [Target tier policy](https://github.com/rust-lang/rfcs/pull/2803)
* [add const-ub RFC](https://github.com/rust-lang/rfcs/pull/3016)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [A new prelude for the 2021 edition (trait-only edition)](https://github.com/rust-lang/rfcs/pull/3114)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [FCP poll for ErrorKind::OutOfMemory](https://github.com/rust-lang/rust/issues/84916)
* [disposition: merge] [impl FromStr for proc_macro::Literal](https://github.com/rust-lang/rust/pull/84717)
* [disposition: merge] [rustdoc: Make "rust code block is empty" and "could not parse code block" warnings a lint (`INVALID_RUST_CODEBLOCKS`)](https://github.com/rust-lang/rust/pull/84587)
* [disposition: merge] [Uplift the invalid_atomic_ordering lint from clippy to rustc](https://github.com/rust-lang/rust/pull/84039)
* [disposition: merge] [Stabilize "RangeFrom" patterns](https://github.com/rust-lang/rust/pull/83918)
* [disposition: merge] [Stabilize extended_key_value_attributes](https://github.com/rust-lang/rust/pull/83366)

## New RFCs

* [RFC: Preview for Unstable Features](https://github.com/rust-lang/rfcs/pull/3120)
* [Rust-lang crate ownership policy](https://github.com/rust-lang/rfcs/pull/3119)

# Upcoming Events

### Online
* [May 6, New York, NY, US - Rust Lightning Talks - Rust NYC](https://www.meetup.com/Rust-NYC/events/277822386)
* [May 11, Seattle, WA, US - Monthly meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrycchbpb/)
* [May 11, Saarbücken, Saarland, DE - Meetup: 11u16 (virtual) - Rust Saar](https://www.meetup.com/de-DE/Rust-Saar/events/277607432/)
* [May 12, Online - Rust Meetup May 2021 - Rust Malaysia](https://docs.google.com/forms/d/e/1FAIpQLSf_hz-ZDwYEhVmIH0uzJ0uH41aXWZ_zRDsI0XENpfkKHvh_Jg/viewform)
* [May 15 - June 7, Online - Solana Season Hackathon - Registration open now](https://twitter.com/solana/status/1387411221717176323?s=20)
* [May 17, 2021, Cardiff, UK - Rust and Cpp Cardiff :: v2.0 - Rust and C++ Cardiff](https://secure.meetup.com/register/?referrer_n=event&referrer_i=278002832&ctx=ref)
* [May 20, 2021, Online - Go vs Rust | Round table discussion](https://rustlab.it/en/rust-vs-go/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Fiberplane**

* [Rust Engineer (Remote)](https://fiberplane.dev/careers/rust-engineer/)

**Paige**

* [Senior Software Engineer, Visualization (Remote, Europe)](https://boards.greenhouse.io/paige/jobs/5210311002)

**Netlify**

* [Senior Backend Engineer (Go/Rust) (Remote or San Francisco, CA, US)](https://boards.greenhouse.io/netlify/jobs/5054144002)

**e.ventures**

* [Rust backend engineer (Remote, the Americas)](https://old.reddit.com/r/rust/comments/mfstaz/official_rrust_whos_hiring_thread_for_jobseekers/gspq9v1/)

**ConsenSys**

* [Rust Software Engineer (Protocol Engineering)](https://arbeitnow.com/view/rust-software-engineer-protocol-engineering-consensys-459183)

**Spacemesh**

* [Rust Developer (Remote)](https://spacemesh.io/rust-developer/)

**DEX Labs**

* [Senior Software Engineer – Full-Stack (Remote)](https://dex-labs.breezy.hr/p/49c5370a8473)

**Kollider**

* [Junior Backend Engineer (Remote)](https://kollider.homerun.co/junior-backend-engineer/en)
* [Senior Backend Engineer (Remote)](https://kollider.homerun.co/senior-backend-engineer/en)
* [DevOps Engineer (Remote)](https://kollider.homerun.co/devops-engineer/en)

**Ockam**

* [Multiple Rust Engineering Positions](https://www.ockam.io/team#open-roles)

**Kraken**

* [Several Rust Engineering Positions](https://jobs.lever.co/kraken?team=Engineering)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Using R or Numpy is like driving around in a sports car. You just turn the wheel, press the pedals, and burn rubber. Rust (and other systems languages) are like getting a spaceship. You can go places and do things that you never dreamt of in a car. They are harder to pilot, but the possibilities seem unlimited! With the Rust ecosystem still in development, it feels like parts of your spaceship come in boxes of parts labeled "some assembly required".

– [Erik Rose on rust-users](https://users.rust-lang.org/t/rust-for-data-first-problems/58887/16)

Thanks to [Phlopsi](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1047) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/n5xua0/this_week_in_rust_389/)</small>
