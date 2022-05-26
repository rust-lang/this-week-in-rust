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

### Foundation

### Newsletters

### Project/Tooling Updates

* [What’s New in IntelliJ Rust for 2022.1](https://blog.jetbrains.com/rust/2022/05/19/what-s-new-in-intellij-rust-for-2022-1/)
* [rust-analyzer changelog #130](https://rust-analyzer.github.io/thisweek/2022/05/23/changelog-130.html)
* [Fornjot (code-first CAD in Rust) - Weekly Dev Log - 2022-W20](https://www.fornjot.app/blog/weekly-dev-log/2022-w20/)
* [Slint (UI crate) weekly update](https://slint-ui.com/thisweek/2022-05-23.html)
* [Apache Arrow has released version 8.0.0 of the DataFusion in-process SQL query engine](https://arrow.apache.org/blog/2022/05/16/datafusion-8.0.0/)
* [Redust: a new Redis client](https://www.reddit.com/r/rust/comments/ux9ry5/redust_a_new_redis_client/)
* [[libblendinfo] Return information from Rust crate to C library](https://www.janwalter.org/jekyll/rust/libblendinfo/2022/05/10/libblendinfo.html)

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

### Research

### Miscellaneous

* [Developer survey: JavaScript and Python reign, but Rust is rising](https://www.infoworld.com/article/3661248/developer-survey-javascript-and-python-reign-but-rust-is-rising.html)
* [DE] [Programmiersprache Rust 1.61 kann Programme aussagekräftig beenden](https://www.heise.de/news/Programmiersprache-Rust-1-61-kann-Programme-aussagekraeftig-beenden-7101438.html)

## Crate of the Week

<!-- COTW goes here -->

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

<!-- Rust updates go here -->

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

--

* *No RFCs issued a call for testing this week.*
* *No RFCs were approved this week.*
* *No RFCs entered Final Comment Period this week.*
* *No Tracking Issues or PRs entered Final Comment Period this week.*
* *No New or Updated RFCs were created this week.*

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

<!--

New jobs can be posted here.

They should be of the form:

**Company Name**

* [Job Title (Location)](https://example.com/my-job-link)

-->

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

<!-- QOTW goes here -->

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
