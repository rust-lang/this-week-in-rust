Title: This Week in Rust 444
Number: 444
Date: 2022-05-25
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the Linked Page](https://example.com/my_article)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official

* [Announcing Rust 1.61.0 | Rust Blog](https://blog.rust-lang.org/2022/05/19/Rust-1.61.0.html)

### Project/Tooling Updates

* [What’s New in IntelliJ Rust for 2022.1](https://blog.jetbrains.com/rust/2022/05/19/what-s-new-in-intellij-rust-for-2022-1/)
* [rust-analyzer changelog #130](https://rust-analyzer.github.io/thisweek/2022/05/23/changelog-130.html)
* [Fornjot (code-first CAD in Rust) - Weekly Dev Log - 2022-W20](https://www.fornjot.app/blog/weekly-dev-log/2022-w20/)
* [Slint (UI crate) weekly update](https://slint-ui.com/thisweek/2022-05-23.html)
* [Apache Arrow has released version 8.0.0 of the DataFusion in-process SQL query engine](https://arrow.apache.org/blog/2022/05/16/datafusion-8.0.0/)
* [Redust: a new Redis client](https://www.reddit.com/r/rust/comments/ux9ry5/redust_a_new_redis_client/)
* [[libblendinfo] Return information from Rust crate to C library](https://www.janwalter.org/jekyll/rust/libblendinfo/2022/05/10/libblendinfo.html)
* [This week in Databend #43: A Modern Cloud Data Warehouse for Everyone](https://weekly.databend.rs/2022-05-25-databend-weekly/)
* [This week in Fluvio #34: the programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0034/)

### Observations/Thoughts

* [Rust: A Critical Retrospective](https://www.bunniestudios.com/blog/?p=6375)
* [Building a Cloud Database from Scratch: Why We Moved from C++ to Rust](https://singularity-data.com/blog/building-a-cloud-database-from-scratch-why-we-moved-from-cpp-to-rust/)
* [How we use Rust, SQLx and Rocket for Oso Cloud](https://www.osohq.com/post/rust-rocket-sqlx)
* [Fixing Memory Leaks in Rust](https://onesignal.com/blog/solving-memory-leaks-in-rust/)
* [Crash Reporting in Rust](https://jake-shadle.github.io/crash-reporting/)
* [Rust Environment and Docker Build with Nix Flakes](https://johns.codes/blog/rust-enviorment-and-docker-build-with-nix-flakes)
* [BonsaiDb performance update: A deep-dive on file synchronization](https://bonsaidb.io/blog/durable-writes/)
* [Rust Docker Tutorial](https://tutorialedge.net/rust/rust-docker-tutorial/)

### Rust Walkthroughs

* [Optimizing the size of your Rust binaries](https://kerkour.com/optimize-rust-binary-size)
* [Testing and building your Rust project with GitHub Actions](https://kerkour.com/rust-github-actions-ci-cd)
* [Rust-raspberrypi-OS-tutorials: Tutorial 19 - Kernel Heap](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/19_kernel_heap#readme)
* [video] [Deref and Drop Traits](https://www.youtube.com/watch?v=Nlc3HdVyaNg)

### Miscellaneous

* [Developer survey: JavaScript and Python reign, but Rust is rising](https://www.infoworld.com/article/3661248/developer-survey-javascript-and-python-reign-but-rust-is-rising.html)
* [DE] [Programmiersprache Rust 1.61 kann Programme aussagekräftig beenden](https://www.heise.de/news/Programmiersprache-Rust-1-61-kann-Programme-aussagekraeftig-beenden-7101438.html)

## Crate of the Week

This week's crate is [rustdoc-types](https://docs.rs/rustdoc-types), a crate with types to deserialize Rustdoc's JSON output.

Thanks to [Nixon Enraght-Moony](https://users.rust-lang.org/t/crate-of-the-week/2704/1061) for the self-ish suggestion. 

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

363 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-05-16..2022-05-23

* [recover when resolution did not resolve lifetimes](https://github.com/rust-lang/rust/pull/97236)
* [add new lint to enforce whitespace after keywords](https://github.com/rust-lang/rust/pull/97179)
* [lint single-use lifetimes during AST resolution](https://github.com/rust-lang/rust/pull/96833)
* [fix misleading "cannot infer type for type parameter" error](https://github.com/rust-lang/rust/pull/97109)
* [improve `u32 as char` cast diagnostic](https://github.com/rust-lang/rust/pull/97169)
* [suggest dereferencing non-lval mutable reference on assignment](https://github.com/rust-lang/rust/pull/94639)
* [add a query for checking whether a function is an intrinsic](https://github.com/rust-lang/rust/pull/97012)
* [types with reachable constructors are reachable](https://github.com/rust-lang/rust/pull/97096)
* [miri: adjust diagnostics assertion so we don't ICE in setup](https://github.com/rust-lang/miri/pull/2141)
* [miri: initial work on Miri permissive-exposed-provenance](https://github.com/rust-lang/miri/pull/2059)
* [miri: make `allow_data_races_*` public and use it during `EnvVars::cleanup`](https://github.com/rust-lang/miri/pull/2142)
* [remove quadratic behaviour from `-Zunpretty=hir-tree`](https://github.com/rust-lang/rust/pull/97223)
* [clean up derived obligation creation](https://github.com/rust-lang/rust/pull/96892)
* [correctly deal with user type ascriptions in pat](https://github.com/rust-lang/rust/pull/96515)
* [rustc\_parse: move AST -> `TokenStream` conversion logic to `rustc_ast`](https://github.com/rust-lang/rust/pull/97251)
* [stabilize `Ipv6Addr::to_ipv4_mapped`](https://github.com/rust-lang/rust/pull/96906)
* [stabilize `array_from_fn`](https://github.com/rust-lang/rust/pull/94119)
* [add convenience byte offset/check align functions to pointers](https://github.com/rust-lang/rust/pull/95643)
* [add functions to un-poison `Mutex` and `RwLock`](https://github.com/rust-lang/rust/pull/96422)
* [improve codegen of `String::retain` method](https://github.com/rust-lang/rust/pull/96605)
* [change `NonNull::as_uninit_*` to take self by value (as opposed to reference), matching primitive pointers](https://github.com/rust-lang/rust/pull/96100)
* [remove unneeded null pointer asserts in `ptr2int` casts](https://github.com/rust-lang/rust/pull/97188)
* [make `ptr::invalid` not the same as a regular `int2ptr` cast](https://github.com/rust-lang/rust/pull/97219)
* [use pointers in `cell::{Ref,RefMut}` to avoid `noalias`](https://github.com/rust-lang/rust/pull/97027)
* [portable SIMD: add `Mask::cast`](https://github.com/rust-lang/portable-simd/pull/251)
* [backtrace: make Miri backtraces work with `#[global_allocator]`](https://github.com/rust-lang/backtrace-rs/pull/462)
* [hashbrown: add function for getting access to map `table: RawTable<(K, V), A>` field](https://github.com/rust-lang/hashbrown/pull/335)
* [cargo: add unstable `rustc-check-cfg` build script output](https://github.com/rust-lang/cargo/pull/10539)
* [cargo: restore proper error for crate not in local reg](https://github.com/rust-lang/cargo/pull/10683)
* [rustdoc: reduce `clean::Type` size](https://github.com/rust-lang/rust/pull/93963)
* [rustdoc: resolve some more doc links early 2](https://github.com/rust-lang/rust/pull/96713)
* [rustfmt: import_granularity: Don't normalize imports with comments](https://github.com/rust-lang/rustfmt/pull/5311)
* [clippy: fix `cmp_owned` on copy types](https://github.com/rust-lang/rust-clippy/pull/8807)
* [clippy: improve "unknown field" error messages](https://github.com/rust-lang/rust-clippy/pull/8823)
* [clippy: lint indirect usages in `disallowed_methods`](https://github.com/rust-lang/rust-clippy/pull/8852)
* [clippy: `dbg_macro` tolerates use of `dbg!` in test items](https://github.com/rust-lang/rust-clippy/pull/8838)
* [clippy: add suggestions to `rc_clone_in_vec_init`](https://github.com/rust-lang/rust-clippy/pull/8814)
* [rust-analyzer: fix inference when pattern matching a tuple field with a wildcard](https://github.com/rust-lang/rust-analyzer/pull/12355)
* [rust-analyzer: generate enum variant assist](https://github.com/rust-lang/rust-analyzer/pull/12334)
* [rust-analyzer: add "cargo clippy" task preset](https://github.com/rust-lang/rust-analyzer/pull/12326)
* [rust-analyzer: implement inlay hint tooltips](https://github.com/rust-lang/rust-analyzer/pull/12285)
* [rust-analyzer: improve docs generation assist](https://github.com/rust-lang/rust-analyzer/pull/12303)
* [rust-analyzer: add a "Add attribute" assist](https://github.com/rust-lang/rust-analyzer/pull/12296)
* [rust-analyzer: don't swallow build script errors](https://github.com/rust-lang/rust-analyzer/pull/12329)
* [rust-analyzer: fix broken async callback in join lines](https://github.com/rust-lang/rust-analyzer/pull/12342)
* [rustup: don't send logging to stdout](https://github.com/rust-lang/rustup/pull/2985)

### Rust Compiler Performance Triage

Overall a positive week for non-incremental performance (roughly 0.5% faster),
however, some >1% regressions on multiple incremental benchmarks, primarily due to
[#95563](https://github.com/rust-lang/rust/pull/95563), which will hopefully be
investigated in the coming weeks.

Triage done by **@simulacrum**.
Revision range: [7355d971..43d9f3](https://perf.rust-lang.org/?start=7355d971a954ed63293e4191f6677f60c1bc07d9&end=43d9f3859e0204e764161ee085a360274b5f3e9a&absolute=false&stat=instructions%3Au)

2 Regressions, 5 Improvements, 4 Mixed; 0 of them in rollups
57 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-05-24.md)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Modify MIR building to drop repeat expressions with length zero](https://github.com/rust-lang/rust/pull/95953)
* [disposition: merge] [Tracking Issue for `{array, slice}::{from_ref, from_mut}` as const fn](https://github.com/rust-lang/rust/issues/90206)
* [disposition: merge] [Remove migrate borrowck mode](https://github.com/rust-lang/rust/pull/95565)
* [disposition: merge] [Remove label/lifetime shadowing warnings](https://github.com/rust-lang/rust/pull/96296)
* [disposition: merge] [Stabilize the bundle native library modifier](https://github.com/rust-lang/rust/pull/95818)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [RFC: Add more support for fallible allocations in Vec](https://github.com/rust-lang/rfcs/pull/3271)
* [notice] [Mention RFC 1201 was superseded by RFC 2972](https://github.com/rust-lang/rfcs/pull/3270)
* [new] [RFC: Add a scalable representation to allow support for scalable vectors](https://github.com/rust-lang/rfcs/pull/3268)
* [new] [Macro Shorthand: Make m!123 identical to m!(123)](https://github.com/rust-lang/rfcs/pull/3267)

## Upcoming Events

Rusty Events between 2022-05-25 - 2022-06-22 🦀

### Virtual

* 2022-05-25 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/qwgrssydchbhc/)
* 2022-05-26 | Linz, AT | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Rust Meetup Linz - 22th Edition**](https://www.meetup.com/Rust-Linz/events/286006468/)
* 2022-05-31 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/Dallas-Rust/events/jqxqwrydchbpc/)
* 2022-06-01 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcjbcb/)
* 2022-06-01 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydcjbcb/)
* 2022-06-07 | Beijing, CN | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/Wasm-Rust-Meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/Wasm-Rust-Meetup/events/jbfnrsydcjbkb/)
* 2022-06-07 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/Buffalo-Rust-Meetup/events/xgmfssydcjbkb/)
* 2022-06-08 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcjblb/)
* 2022-06-09 | Dublin, IE | [Rust Dublin](https://www.meetup.com/Rust-Dublin/)
    * [**Verus — Verified Rust for low-level systems code**](https://www.meetup.com/Rust-Dublin/events/286018947/)
* 2022-06-09 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydcjbmb/)
* 2022-06-09 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/San-Diego-Rust/)
    * [**San Diego Rust June 2022 Tele-Meetup**](https://www.meetup.com/San-Diego-Rust/events/285952122/)
* 2022-06-09 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/swgrssydcjbmb/)
* 2022-06-14 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/Dallas-Rust/events/vqtjcsydcjbsb/)
* 2022-06-15 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydcjbtb/)
* 2022-06-15 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Nutshell**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydcjbtb/)
* 2022-06-21 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydcjbcc/)

### North America

* 2022-05-25 | New York, NY, US | [Rust NYC](https://www.meetup.com/Rust-NYC/)
    * [**May Lightning Talks: State machines for sync, BonsaiDB, WASM Cloudflare Workers**](https://www.meetup.com/Rust-NYC/events/285925838/)
* 2022-05-31 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/Minneapolis-Rust-Meetup/)
    * [**Happy Hour and Planning (everyone welcome)**](https://www.meetup.com/Minneapolis-Rust-Meetup/events/285954876/)
* 2022-06-01 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/285973465/)
* 2022-06-08 | Atlanta, GA, US | [Rust ATL](https://www.meetup.com/Rust-ATL/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/Rust-ATL/events/pczdssydcjblb/)
* 2022-06-09 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcjbmb/)
* 2022-06-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person **](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcjbcc/)

### Europe

* 2022-05-30 | London, UK | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**LDN Talks May 2022 *Quickwit Takeover* RSVP @Skills Matter**](https://www.meetup.com/Rust-London-User-Group/events/285740296/)
* 2022-05-31 | Rome, IT | [Rust Roma](https://www.meetup.com/Rust-Roma/)
    * [**When Rocket is fueled by Diesel #Aperitech**](https://www.meetup.com/Rust-Roma/events/285587293/)
* 2022-06-09 | Oslo, NO | [Rust Oslo](https://www.meetup.com/Rust-Oslo/)
    * [**Introduction to axum - An ergonomic and modular web framework by David Pedersen**](https://www.meetup.com/Rust-Oslo/events/286006378/)

### Oceania

* 2022-05-26 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/Rust-Brisbane/)
    * [**May Meetup**](https://www.meetup.com/Rust-Brisbane/events/285665676/)
* 2022-06-17 | Melbourne, VI, AU | [Rust Melbourne](https://www.meetup.com/Rust-Melbourne/)
    * [**June 2022 Meetup**](https://www.meetup.com/Rust-Melbourne/events/285962368/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Element**

* [Software Engineer - VoIP (Go/Rust) (London, UK or Remote)](https://apply.workable.com/elementio/j/5BD58AFB6F/)

**Kidsloop**

* [Senior Audio/ Video Backend Engineer (Seoul, KR)](https://kidsloop.bamboohr.com/jobs/view.php?id=397)

**SixtyFPS GmbH**

* [Software Developer for the Slint UI Toolkit (remote/EU)](https://slint-ui.com/careers.html)

<!--

New jobs can be posted here.

They should be of the form:

**Company Name**

* [Job Title (Location)](https://example.com/my-job-link)

-->
**Bionaut Labs**

* [Embedded Rust Engineer - Senior/Principal (Remote US, Los Angeles, CA, US)](https://www.indeed.com/cmp/Bionaut-Labs-1/jobs?jk=2d4ddec4bed66bc1)

**Stockly**

* [Back-end developer - Engine (Rust, gRPC, PostgreSQL) (Paris, FR)](https://www.welcometothejungle.com/fr/companies/stockly/jobs/back-end-developer-engine-team-rust-grpc-postgresql_paris)
* [Back-end developer (Rust, gRPC, PostgreSQL) (Paris, FR)](https://www.welcometothejungle.com/fr/companies/stockly/jobs/back-end-developer-rust-grpc-postgresql_paris)

**Tempus Ex**

* [Several full-time Rust positions available (San Francisco, CA, US, Atlanta, GA, US, Austin, TX, US, and Remote)](https://tempus-ex.com/careers)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> This is the difference in approaches of the two languages. In C++ if the code is vulnerable, the blame is on the programmer. In Rust if the code is vulnerable, Rust considers it a failure of the language, and takes responsibility to stop even “bad” programmers from writing vulnerable code. I can’t stress enough how awesome it is that I can be a careless fool, and still write perfectly robust highly multi-threaded code that never crashes.

– [kornel on lobste.rs](https://lobste.rs/s/wiavtb/rust_critical_retrospective#c_jkfhpb) (with a [caveat from ZiCog](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1244) that Rust does *not* guarantee freedom from all vulnerabilities!)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1243) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/uxx7w8/this_week_in_rust_444/)</small>
