Title: This Week in Rust 488
Number: 488
Date: 2023-03-29
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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
* [Announcing Rust 1.68.2 | Rust Blog](https://blog.rust-lang.org/2023/03/28/Rust-1.68.2.html)
* [Announcing Rust 1.68.1 | Rust Blog](https://blog.rust-lang.org/2023/03/23/Rust-1.68.1.html)

### Foundation

### Newsletters

### Project/Tooling Updates
* [rust-analyzer changelog #174](https://rust-analyzer.github.io/thisweek/2023/03/27/changelog-174.html)
* [IntelliJ Rust Changelog #191](https://intellij-rust.github.io/2023/03/27/changelog-191.html)
* [Fornjot (code-first CAD in Rust) - Weekly Release - Some Good Progress](https://www.fornjot.app/blog/weekly-release/2023-w13/)
* [Announcing `scoped-trace`.](https://jack.wrenn.fyi/blog/scoped-trace/)
* [dfdx v0.11.0: cuda accelerated deep learning](https://coreylowman.github.io/2023/03/15/release-0.11.0.html)
* [Rust Search Extension v1.10.0 has been released](https://rust.extension.sh/changelog/#v1-10-0-2023-03-25)
* [Introducing Kobold](https://maciej.codes/2023-03-23-kobold.html)
* [autometrics-rs 0.3: Defining Service-Level Objectives (SLOs) in Rust Source Code](https://fiberplane.com/blog/autometrics-rs-0-3-defining-service-level-objectives-in-rust-source-code)
* [Rocket's 3rd v0.5 Release Candidate - Rocket Web Framework](https://rocket.rs/v0.5-rc/news/2023-03-23-version-0.5-rc.3/)
* [Seven Tasks with Rust and Egui](https://github.com/Rust-Ninja-Sabi/rust-egui-seven-tasks)
* [video] [Rust Releases! Rust 1.68.1](https://www.youtube.com/watch?v=TTOH-_bZlYY)

### Observations/Thoughts
* [STV-rs: Single Transferable Vote implementation in Rust](https://gendignoux.com/blog/2023/03/27/single-transferable-vote.html)
* [STM32F4 Embedded Rust at the PAC: Creating Hardware Abstractions with embedded-hal](https://apollolabsblog.hashnode.dev/stm32f4-embedded-rust-at-the-pac-creating-hardware-abstractions-with-embedded-hal)
* [Rust's Golden Rule](https://steveklabnik.com/writing/rusts-golden-rule)
* [Linearity and Control](https://blog.yoshuawuyts.com/linearity-and-control/)
* [Zig And Rust](https://matklad.github.io/2023/03/26/zig-and-rust.html)
* [Generators](https://without.boats/blog/generators/)
* [The AsyncIterator interface](https://without.boats/blog/async-iterator/)
* [A Proposal for Safe Window Handles](https://notgull.github.io/safe-windows/)

### Rust Walkthroughs
* [Building a Fibonacci Heap](https://www.kurtlawrence.info/blog/building-a-fibonacci-heap)
* [Embedded Rust on BBC Micro Bit: unlocking Vec and HashMap](https://gitlab.com/cyril-marpaud/microbit_vec_hashmap)
* [STM32F4 Embedded Rust at the PAC: Creating Hardware Abstractions](https://apollolabsblog.hashnode.dev/stm32f4-embedded-rust-at-the-pac-creating-hardware-abstractions)
* [State Machine testing with Proptest](https://tzemanovic.gitlab.io/posts/state-machine-testing-with-proptest/)
* [Linear Types One-Pager](https://blog.yoshuawuyts.com/linear-types-one-pager/)
* [Writing a Linux executable from scratch with x86_64-unknown-none and Rust](https://vulns.xyz/2023/03/linux-executable-from-scratch-with-x86_64-unknown-none-rust/)
* [Efficient, Extensible, Expressive: Typed Tagless Final Interpreters in Rust](https://getcode.substack.com/p/efficient-extensible-expressive-typed)
* [video] [Env Config Option](https://youtu.be/r6niPhmgxRY)

### Research
* [Tree Borrows - A new aliasing model for Rust](https://perso.crans.org/vanille/treebor)

### Miscellaneous
* [video] [I built my own memory profiler (in Rust, on Linux)](https://youtu.be/DpnXaNkM9_M)
* [video] [Build Universal Libraries with Rust](https://www.youtube.com/watch?v=uKlHwko36c4)
* [video] [How to Learn Rust](https://www.youtube.com/watch?v=2hXNd6x9sZs)
* [video] [How Bevy FlyCam Works](https://www.youtube.com/watch?v=d1agtogutHA)
* [video] [Why I Swaped to Fixed Point Numbers in my game #UTDTG #devlog 2](https://www.youtube.com/watch?v=-YDg8WpJmHw)
* [video] [Make iterators 10X better with itertools](https://www.youtube.com/watch?v=qY9j4dRaMjU)

## Crate of the Week

<!-- COTW goes here -->

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

<!-- Rust updates go here -->

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Tracking Issue for IsTerminal / is_terminal](https://github.com/rust-lang/rust/issues/98070)
* [disposition: merge] [Initial support for return type notation (RTN)](https://github.com/rust-lang/rust/pull/109010)
* [disposition: merge] [Tracking Issue for Option::is_some_and and Result::is_{ok,err}_and](https://github.com/rust-lang/rust/issues/93050)
* [disposition: merge] [Tracking Issue for "C-unwind ABI", RFC 2945](https://github.com/rust-lang/rust/issues/74990)
* [disposition: merge] [rustdoc: run more HIR validation to mirror rustc](https://github.com/rust-lang/rust/pull/108576)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC: sigstore and cargo/crates.io](https://github.com/rust-lang/rfcs/pull/3403)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-03-29 - 2023-04-26 🦀

### Virtual

* 2023-03-22 | Virtual (Richmond, VA, US) | [Rustaceans RVA](https://www.meetup.com/rustaceans-rva/)
    * [**Rustaceans RVA - March Meetup**](https://www.meetup.com/rustaceans-rva/events/291963911/)
* 2023-03-27 | Virtual | [Rust Formal Methods Interest Group](https://www.eventbrite.com/cc/rfmig-87969)
    * [**Flux: Ergonomic Verification of Rust Programs with Liquid Types**](https://www.eventbrite.com/e/flux-ergonomic-verification-of-rust-programs-with-liquid-types-tickets-577742873487?aff=ebdssbonlinesearch)
* 2023-03-28 | Virtual (Berlin, DE) | [Berline.rs - OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/292113239/)
* 2023-03-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfcfblc/)
* 2023-03-28 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Crack code interview problems in Rust: S2 Ep3**](https://www.meetup.com/microsoft-reactor-redmond/events/291677113/)
* 2023-03-29 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Writing your own rust 'book' with mdBook**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/291892487/)
* 2023-03-31 | Virtual (Tunis, TN) | [Rust Tunisia](https://www.meetup.com/rust-tunisia/)
    * [**Rust Meetup Tunisia - Volume I, Number III**](https://www.meetup.com/rust-tunisia/events/292402446/)
* 2023-04-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfcgbgb/)
* 2023-04-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/291967741/)
* 2023-04-05 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcgbhb/)
* 2023-04-11 | Virtual (Berlin, DE) | [Berline.rs - OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/292236794/)
* 2023-04-11 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsyfcgbpb/)
* 2023-04-11 | Virtual | [Rust Live](https://www.eventbrite.com/cc/rust-live-1876809)
    * [**Rust Live: Asynchronous Rust**](https://www.eventbrite.com/e/rust-live-asynchronous-rust-tickets-575865518267?aff=ebdssbonlinesearch&keep_tld=1)
* 2023-04-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—Introducing duplicate! and the peculiarities of proc macros**](https://www.meetup.com/rustdc/events/291830834/)
* 2023-04-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/-0)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/lqkkctyfcgbzb/)

### Europe

* 2023-03-28 | Zurich, CH | [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**High performance concurrent data structures in Rust - March Meetup**](https://www.meetup.com/de-DE/rust-zurich/events/291449557/)
* 2023-03-29 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #57**](https://www.meetup.com/rust-paris/events/291963747/)
* 2023-04-04 | Berlin, DE | [Berline.rs](https://berline.rs)
    * [**Rust and Tell - Goodbye👋 Edition**](https://berline.rs/2023/04/04/rust-and-tell-goodbye-edition.html)
* 2023-04-06 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #3**](https://www.meetup.com/fr-FR/rust-lyon/events/292283973/)
* 2023-04-19 | Zurich, CH | [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**sett: data encryption and transfer made easy(ier)**](https://www.meetup.com/de-DE/rust-zurich/events/292151879/)
* 2023-04-20 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**First Rust Bern Meetup!**](https://www.meetup.com/de-DE/rust-bern/events/292206056/)

### Asia
* 2023-04-08 | Kyoto, JP | [Kansai Rust](https://www.meetup.com/kansai-rust/)
    * [**Demystifying Closures**](https://www.meetup.com/kansai-rust/events/292202435/) 
* 2023-04-08 | Beijing, China | [Rust Chinese Group](https://www.meetup.com/rust-chinese-group/)
    * [**Rust Meetup Beijing**](https://www.meetup.com/rust-chinese-group/events/292379002/) 
* 2023-04-12 | Kuala Lumpur, MY | [Rust Malaysia](https://rust-malaysia.github.io/meetup/); [Telegram](https://t.me/golangmalaysia)
    * [**Rust Meetup Malaysia April 2023: How far is Dioxus from React? by Ivan Tham**](https://www.google.com/calendar/event?eid=MWI0bWJzY21qZTI2NWsyZDgzOG0xb2JkdTkgYXBkOXZtYmMyMmVnZW5tdHU1bDZjNWpiZmNAZw&ctz=America/Los_Angeles) | [Map](https://goo.gl/maps/w2ogftac6mqpBbvt5)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs
<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

<!-- QOTW goes here -->

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
