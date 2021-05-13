Title: This Week in Rust 390
Number: 390
Date: 2021-05-12
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
* [The Plan for the Rust 2021 Edition](https://blog.rust-lang.org/2021/05/11/edition-2021.html)
* [Announcing Rust 1.52.1](https://blog.rust-lang.org/2021/05/10/Rust-1.52.1.html)
* [Announcing Rust 1.52.0](https://blog.rust-lang.org/2021/05/06/Rust-1.52.0.html)

### Newsletters
* [This Month in Rust GameDev #21 - April 2021](https://gamedev.rs/news/021/)
* [RiB Newsletter #23 - Rewriting in Rust?](https://www.reddit.com/r/rust/comments/n5xhku/rib_newsletter_23_rewriting_in_rust/)

### Project/Tooling Updates
* [rust-analyzer Changelog #76](https://rust-analyzer.github.io/thisweek/2021/05/10/changelog-76.html)
* [Knurling-rs changelog #24: three releases, `defmt-test` supports more items and a nasty issue](https://ferrous-systems.com/blog/knurling-changelog-24/)
* [This Week In TensorBase 2](https://tensorbase.io/thisweek/2021-05-10-tw_2/)
* [Naga shader translation benchmark](https://gfx-rs.github.io/2021/05/09/dota2-msl-compilation.html)
* [Announcing egui 0.12 - the simple GUI library](https://www.reddit.com/r/rust/comments/n9f6vt/announcing_egui_012_the_simple_gui_library/)
* [GCC Rust Weekly Status Report 14](https://thephilbert.io/2021/05/10/gcc-rust-weekly-status-report-14/)
* [Integrating Rust Into the Android Open Source Project](https://security.googleblog.com/2021/05/integrating-rust-into-android-open.html)
* [A New AWS SDK for Rust - Alpha Launch](https://aws.amazon.com/blogs/developer/a-new-aws-sdk-for-rust-alpha-launch/)
* [Gleam v0.15 released!](https://gleam.run/news/gleam-v0.15-released/)
* [IsomorphicDB. The Road ahead](https://isomorphicdb.io/blog/2021/05/11/IsomorphicDB-The-Road-ahead/)
* [My first Rust crate, pretend](https://sfietkonstantin.github.io/2021/05/10/First-Crate-Pretend.html)

### Observations/Thoughts
* [Rust on the Frontend and Backend](https://blog.abor.dev/p/moonzoon)
* [Why we should consider Rust for Embedded Developement?](https://blog.knoldus.com/why-rust-for-embedded-development/)
* [Aiming for idiomatic Rust](https://shane-o.dev/blog/aiming-for-idiomatic-rust)
* [Rust on the Frontend and Backend](https://blog.abor.dev/p/moonzoon)
* [Optimizing 7000 CPUs Away with Rust](https://medium.com/tenable-techblog/optimizing-700-cpus-away-with-rust-dc7a000dbdb2)
* [Improving our development confidence and productivity with Bors](https://www.fluvio.io/blog/2021/05/bors-confident-merges/)
* [Increasing the level of parallelism in DataFusion 4.0](https://medium.com/@danilheres/increasing-the-level-of-parallelism-in-datafusion-4-0-d2a15b5a2093)
* [My second impression of Rust and why I think it's the best general-purpose language!](https://deepu.tech/my-second-impression-of-rust/)
* [Incrementally porting a small Python project to Rust](https://blog.waleedkhan.name/port-python-to-rust/)
* [video] [Rust Verification Workshop 2021 - Ferrite: A Rust EDSL for Message-passing Protocol Verification](https://youtu.be/6dcf3tOPOwo)
* [video] [Rust Verification Workshop 2021 - RustBelt: A Quick Dive into the Abyss](https://youtu.be/iAs0gZ8o0oQ)
* [video] [Rust Verification Workshop 2021 - Polonius](https://youtu.be/H54VDCuT0J0)
* [video] [Rust Verification Workshop 2021 - Rust Interest in safety- and mission-critical environments](https://youtu.be/_DM36e2A9dg)
* [video] [Rust Verification Workshop 2021 - Leveraging Compiler Intermediate Representation for Multi- and Cross-Language Verification](https://youtu.be/0DcIn7kiNxM)

### Rust Walkthroughs
* [BABE – Consensus Algorithm and How to Implement it](https://blog.knoldus.com/babe-consensus-algorithm-and-how-to-implement-it-in-our-runtime/)
* [Pallets in Substrate and using them in runtime.](https://blog.knoldus.com/pallets-in-substrate-and-using-them-in-runtime/)
* [How to deploy Rust web app with DigitalOcean](https://dev.to/steadylearner/how-to-deploy-rust-web-app-with-digitalocean-h2o)
* [Learning Rust #4: Parsing JSON with strong types](https://dev.to/hamatti/learning-rust-4-parsing-json-with-strong-types-575m)
* [How to use Rust web framework Warp](https://dev.to/steadylearner/how-to-use-rust-warp-web-framework-2b4e)
* [Ray Tracing in One Weekend](https://misterdanb.github.io/raytracinginrust/)
* [Procedural Macros: Error handling](https://blog.turbo.fish/proc-macro-error-handling/)
* [Speeding up algorithms with arena allocators](https://mnwa.medium.com/speeding-up-algorithms-with-arena-allocators-d72d06f23607)
* [Building a REST and Web Socket API with Actix and Rust](https://agmprojects.com/blog/building-a-rest-and-web-socket-api-with-actix.html)
* [Building Rust binaries in CI that work with older GLIBC](https://kobzol.github.io/rust/ci/2021/05/07/building-rust-binaries-in-ci-that-work-with-older-glibc.html)
* [ZH] [series] [Build GraphQL services based on Async Rust using tide + async-graphql + mongodb (基于 Async Rust 构建 GraphQL 服务，使用 tide + async-graphql + mongodb) - Part 4](https://blog.budshome.com/budshome/gou-jian-rust-yi-bu-graphql-fu-wu-:ji-yu-tide-+-async-graphql-+-mongodb(4)--bian-geng-fu-wu-,yi-ji-di-er-ci-zhong-gou)
* [ZH] [series] [Build GraphQL services based on Async Rust using actix-web + async-graphql + rbatis + postgresql / mysql (基于 actix-web + async-graphql + rbatis + postgresql / mysql 构建异步 Rust GraphQL 服务) - Part 3](https://blog.budshome.com/budshome/ji-yu-actix-web-+-async-graphql-+-rbatis-+-postgresql---mysql-gou-jian-yi-bu-rust-graphql-fu-wu-(3)---zhong-gou)
* [video] [Graphs in Rust: What is a Graph? Representing them in Rust](https://youtu.be/3DLrUNbKhjQ)
* [video] [Rust Verification Workshop 2021 - Peeking at compiler-internal data (for fun and profit)](https://youtu.be/SKmd5A-1cSE)
* [video] [Rust Verification Workshop 2021 - Verifying that Rust programs don't crash](https://youtu.be/vMGilPbIotw)
* [video] [Rust Verification Workshop 2021 - crux-mir: Symbolic testing for Rust](https://youtu.be/0mocaSR9f_M)
* [video] [Rust Verification Workshop 2021 - Rustv: Semi-automatic Verification of Unsafe Rust Programs](https://youtu.be/bikmlNlwAYo)
* [video] [Rust Verification Workshop 2021 - Towards Automatic Verification of Unsafe Rust with Constrained Horn Solvers](https://youtu.be/yJQZ7sG8xSM)
* [video] [Rust Verification Workshop 2021 - Prusti - Deductive Verification for Rust](https://youtu.be/C9TTioH5JUg)
* [video] [Rust Verification Workshop 2021 - Creusot: A prototype tool for verification of Rust software](https://youtu.be/b8sBtmzq0FM)
* [video] [Rust Verification Workshop 2021 - hacspec: succinct, executable, verifiable specifications for high-assurance cryptography](https://youtu.be/k7_BcWwvz7k)

### Miscellaneous
* [Building Rust binaries in CI that work with older GLIBC](https://kobzol.github.io/rust/ci/2021/05/07/building-rust-binaries-in-ci-that-work-with-older-glibc.html)
* [Innovating with Rust](https://aws.amazon.com/blogs/opensource/innovating-with-rust/)
* [What can C++ do that Rust can't? (2021 edition)](https://www.reddit.com/r/rust/comments/n7rjfk/what_can_c_do_that_rust_cant_2021_edition/)
* [video] [The Rust Borrow Checker—A Deep Dive @ Rust DC, April 20, 2021 w/ Nell Shamrell-Harrington](https://youtu.be/Ys7ma3au5m0)

# Crate of the Week

This week's crate is [tokio-console](https://github.com/tokio-rs/console), a "top"-like utility to view your tasks run.

Thanks to [Simon Farnsworth](https://users.rust-lang.org/t/crate-of-the-week/2704/910) for the nomination

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [advent_of_code_traits - Improve the efficiency of the default `run` method](https://github.com/drmason13/advent_of_code_traits/issues/3)
* [compress-tools-rs - Cannot statically compile on Windows due to linking failure, exit code 1120](https://github.com/OSSystems/compress-tools-rs/issues/57)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

324 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-05-03..2021-05-10

* [fix duplicate unknown lint errors](https://github.com/rust-lang/rust/pull/85053)
* [improve diagnostics for functions in `struct` definitions](https://github.com/rust-lang/rust/pull/76808)
* [ensure failing promoteds in const/static bodies are handled correctly](https://github.com/rust-lang/rust/pull/85112)
* [implement Native link modifiers](https://github.com/rust-lang/rust/pull/83507) (RFC [#2951](https://rust-lang.github.io/rfcs/2951-native-link-modifiers.html))
* [deduplicate native libs before they are passed to the linker](https://github.com/rust-lang/rust/pull/84794)
* [retain data in vectorized registers for longer](https://github.com/rust-lang/rust/pull/84915)
* [only compute Obligation `cache_key` once in register_obligation_at](https://github.com/rust-lang/rust/pull/84923)
* ["const" initialized thread locals in rustc](https://github.com/rust-lang/rust/pull/84833)
* [streamline `try_start` code](https://github.com/rust-lang/rust/pull/84806)
* [simplify `chdir` implementation and minimize unsafe block](https://github.com/rust-lang/rust/pull/84712)
* [optimize `BufWriter`](https://github.com/rust-lang/rust/pull/79930)
* [provide `io::Seek::rewind`](https://github.com/rust-lang/rust/pull/85146)
* [stablize {`HashMap`, `BTreeMap`}`::into_`{`keys`, `values`}](https://github.com/rust-lang/rust/pull/84328)
* [futures: add FuturesUnordered::into_iter, make iter_pin_ref public](https://github.com/rust-lang/futures-rs/pull/2423)
* [regex: fix compilation with pattern feature](https://github.com/rust-lang/regex/pull/772)
* [datafrog: speed up Relation::merge](https://github.com/rust-lang/datafrog/pull/29)
* [cargo: improve two error messages](https://github.com/rust-lang/cargo/pull/9472)
* [rustdoc: fix source code line number display and make it clickable again](https://github.com/rust-lang/rust/pull/85148)
* [compiletest: add `needs-unwind` and beginning of support for testing `panic=abort` std](https://github.com/rust-lang/rust/pull/84734)
* [clippy: fix stack overflow issue in `redundant_pattern_matching`](https://github.com/rust-lang/rust-clippy/pull/7170)
* [clippy: fix `eval_order_dependence` async false positive](https://github.com/rust-lang/rust-clippy/pull/7174)
* [clippy: fix `unused_unit` macro false positive](https://github.com/rust-lang/rust-clippy/pull/7167)
* [clippy: fix `needless_quesiton_mark` false positive](https://github.com/rust-lang/rust-clippy/pull/7165)
* [clippy: for `to_*` variant don't lint in trait impl taking `self` when non-`Copy` type](https://github.com/rust-lang/rust-clippy/pull/7182)
* [clippy: fix `unnecessary_filter_map` false positive](https://github.com/rust-lang/rust-clippy/pull/7175)
* [clippy needless_collect: Lint cases with type annotations for indirect usage and recognize `BinaryHeap`](https://github.com/rust-lang/rust-clippy/pull/7163)

## Rust Compiler Performance Triage

Not much change overall - both regressions and improvements were all minor, apart from the 2x compile-time improvement for libcore from PR [#83278](https://github.com/rust-lang/rust/issues/83278).

Triage done by **@pnkfelix**.
Revision range: [7a0f..382f](https://perf.rust-lang.org/?start=7a0f1781d04662041db5deaef89598a8edd53717&end=382f748f23979e37e3e012b090e5a0313463f182&absolute=false&stat=instructions%3Au)

2 Regressions, 3 Improvements, 0 Mixed
0 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-05-11.md).

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Reserved prefixes in the 2021 edition](https://github.com/rust-lang/rfcs/pull/3101)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [A new prelude for the 2021 edition (trait-only edition)](https://github.com/rust-lang/rfcs/pull/3114)
* [disposition: postpone] [Allow Overloading || and &&](https://github.com/rust-lang/rfcs/pull/2722)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [stabilize const_fn_unsize](https://github.com/rust-lang/rust/pull/85078)
* [disposition: merge] [FCP poll for ErrorKind::OutOfMemory](https://github.com/rust-lang/rust/issues/84916)
* [disposition: merge] [impl FromStr for proc_macro::Literal](https://github.com/rust-lang/rust/pull/84717)
* [disposition: merge] [stabilize member constraints](https://github.com/rust-lang/rust/pull/84701)
* [disposition: merge] [Uplift the invalid_atomic_ordering lint from clippy to rustc](https://github.com/rust-lang/rust/pull/84039)
* [disposition: merge] [Stabilize "RangeFrom" patterns](https://github.com/rust-lang/rust/pull/83918)
* [disposition: merge] [Stabilize extended_key_value_attributes](https://github.com/rust-lang/rust/pull/83366)
* [disposition: merge] [Add functions `Duration::try_from_secs_{f32, f64}`](https://github.com/rust-lang/rust/pull/82179)

## New RFCs

* [Scrape code examples from examples/ directory for Rustdoc](https://github.com/rust-lang/rfcs/pull/3123)

# Upcoming Events

### Online
* [May 12, Online - Rust Meetup May 2021 - Rust Malaysia](https://docs.google.com/forms/d/e/1FAIpQLSf_hz-ZDwYEhVmIH0uzJ0uH41aXWZ_zRDsI0XENpfkKHvh_Jg/viewform)
* [May 13, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/2021/05/13/rust-hack-and-learn.html)
* [May 15 - June 7, Online - Solana Season Hackathon - Registration open now](https://twitter.com/solana/status/1387411221717176323?s=20)
* [May 17, 2021, Cardiff, UK - Rust and Cpp Cardiff :: v2.0 - Rust and C++ Cardiff](https://secure.meetup.com/register/?referrer_n=event&referrer_i=278002832&ctx=ref)
* [May 18, 2021, Washington, DC, US - Mid-month Rustful: rust4ml - Rust DC](https://www.meetup.com/RustDC/events/ntvrgsycchbxb)
* [May 19, 2021, Vancouver, BC - Rust Study/Hack/Hang-out night - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zppkjsycchbzb/)
* [May 20, 2021, Online - Go vs Rust | Round table discussion](https://rustlab.it/en/rust-vs-go/)
* [May 20, 2021, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrycchbhc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Paige**

* [Senior Software Engineer, Visualization (Remote, Europe)](https://boards.greenhouse.io/paige/jobs/5210311002)

**Impero**

* [Full Stack Developer (Denmark + Remote)](https://www.linkedin.com/jobs/view/2493233249/)

**Zimpler**

* [Rust Developer (Gothenburg, SE)](https://careers.zimpler.com/jobs/1170476-rust-developer-to-zimpler)

**Yat Labs**

* [Senior Rust Developer (Remote)](https://www.arbeitnow.com/view/senior-rust-developer-tari-71761)

**Kollider**

* [Junior Backend Engineer (Remote)](https://kollider.homerun.co/junior-backend-engineer/en)
* [Senior Backend Engineer (Remote)](https://kollider.homerun.co/senior-backend-engineer/en)
* [DevOps Engineer (Remote)](https://kollider.homerun.co/devops-engineer/en)

**TrueLayer**

* [Rust Backend Engineer (London, UK)](https://apply.workable.com/truelayer/j/D07759DAF6/)
* [Rust Backend Engineer (Milan, Italy)](https://apply.workable.com/truelayer/j/F13E839E3B/)
* [Rust Engineering Lead (London, UK)](https://apply.workable.com/truelayer/j/3B78A6F6F4/)
* [Rust Engineering Lead (Milan, Italy)](https://apply.workable.com/truelayer/j/8D8D56C09E/)

**Aleph Alpha**

* [Several Rust Engineering Positions (Heidelberg, DE)](https://aleph-alpha.de/career)

**Kraken**

* [Several Rust Engineering Positions (Remote)](https://jobs.lever.co/kraken?team=Engineering)

**ChainSafe**

* [Several Rust Engineering Positions (Remote)](https://chainsafe.io/careers)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> You won’t appreciate Rust unless you spend few weeks building something in it. The initial steep learning curve could be frustrating or challenging depending on how you see it, but once past that it’s hard not to love it. It’s a toddler with superpowers after all 💗

– [Deepu K Sasidharan on their blog](https://deepu.tech/my-second-impression-of-rust)

Thanks to [Phlopsi](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1047) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/nb4s14/this_week_in_rust_390/)</small>
