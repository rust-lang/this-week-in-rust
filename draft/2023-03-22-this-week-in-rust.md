Title: This Week in Rust 487
Number: 487
Date: 2023-03-22
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

### Foundation
* [Welcoming Software Engineer Adam Harvey to the Rust Foundation Team](https://foundation.rust-lang.org/news/welcoming-software-engineer-adam-harvey-to-the-rust-foundation-team/)
* [New SLSA++ Survey Reveals Real-World Developer Approaches to Software Supply Chain Security](https://foundation.rust-lang.org/news/new-slsa-survey-reveals-real-world-developer-approaches-to-software-supply-chain-security/)

### Newsletters
* [This Month in Rust OSDev: February 2023 | Rust OSDev](https://rust-osdev.com/this-month/2023-02/)

### Project/Tooling Updates
* [Typst starts its public beta test and goes open source](https://typst.app/blog/2023/beta-oss-launch)
* [Klint: Compile-time Detection of Atomic Context Violations for Kernel Rust Code](https://www.memorysafety.org/blog/gary-guo-klint-rust-tools/)
* [rust-analyzer changelog #173](https://rust-analyzer.github.io/thisweek/2023/03/20/changelog-173.html)
* [Gitea 1.19.0 is released](https://blog.gitea.io/2023/03/gitea-1.19.0-is-released/)
* [Fornjot (code-first CAD in Rust) - Weekly Release - Finished!](https://www.fornjot.app/blog/weekly-release/2023-w12/)
* [activitypub-federation 0.4.0: Major rewrite with improvements to usability and documentation](https://docs.rs/activitypub_federation)

### Observations/Thoughts
* [Temporary lifetimes](https://smallcultfollowing.com/babysteps/blog/2023/03/15/temporary-lifetimes/)
* [Must move types](https://smallcultfollowing.com/babysteps/blog/2023/03/16/must-move-types/)
* [Defer blocks and async drop](https://www.ncameron.org/blog/defer-blocks-and-async-drop/)
* [A template proposal for adopting Rust at work](https://tim.mcnamara.nz/post/711821310255349760/template-proposal-to-adopt-rust)
* [Patterns & Abstractions](https://without.boats/blog/patterns-and-abstractions/)
* [Const as an auto trait](https://without.boats/blog/const-as-an-auto-trait/)
* [Item Patterns And Struct Else](https://blog.nilstrieb.dev/posts/item-patterns-and-struct-else/)
* [Why use Rust on the backend?](https://blog.adamchalmers.com/why-rust-on-backend/)
* [The Importance of Logging](https://www.thecodedmessage.com/posts/logging/)
* [audio] [Cargo Limit with Alexander Lopatin :: Rustacean Station](https://rustacean-station.org/episode/alexander-lopatin/)
* [video] [The Truth about Rust/WebAssembly Performance](https://www.youtube.com/watch?v=4KtotxNAwME)


[AsRef vs Borrow trait (ft. ChatGPT)](https://rusty-ferris.pages.dev/blog/asref-vs-borrow-trait/)

### Rust Walkthroughs
* [STM32F4 Embedded Rust at the PAC: Creating Hardware Abstractions](https://apollolabsblog.hashnode.dev/stm32f4-embedded-rust-at-the-pac-creating-hardware-abstractions)
* [STM32F4 Embedded Rust at the PAC: GPIO Interrupts](https://apollolabsblog.hashnode.dev/stm32f4-embedded-rust-at-the-pac-gpio-interrupts)
* [video] [Setting up CI and property testing for a Rust crate](https://www.youtube.com/watch?v=xUH-4y92jPg)

* [Build your own Counting Bloom Filter](https://www.arunma.com/2023/03/19/build-your-own-counting-bloom-filter-in-rust/)

### Research
* [Verus: Verifying Rust Programs using Linear Ghost Types](https://arxiv.org/abs/2303.05491)
* [Ownership guided C to Rust translation](https://arxiv.org/abs/2303.10515)
* [Optimizing a parser/compiler with data-oriented design: a case study](https://tweedegolf.nl/en/blog/88/data-oriented-design)

### Miscellaneous
* [Bringing Rust to the Xen Project](https://xcp-ng.org/blog/2023/03/17/bringing-rust-to-the-xen-project/)

* [The birth of a package manager [written in Rust ;)]](https://ochagavia.nl/blog/the-birth-of-a-package-manager/)

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

A mixed week, with some nice wins, but also at least two PR's that were
subsequently reverted, such as the upgrade to LLVM 16. We do want to
note PR [#108944][], which cut down on crate metadata, binary sizes, and
was an overall win on execution time for many benchmarks.

[#108944]: https://github.com/rust-lang/rust/pull/108944

Triage done by **@pnkfelix**.
Revision range: [00587489..ef03fda3](https://perf.rust-lang.org/?start=0058748944abb3282aba0e0a74823c6411703565&end=ef03fda339923e659d3d3ca3321de887316d2807&absolute=false&stat=instructions%3Au)

1 Regressions, 4 Improvements, 11 Mixed; 2 of them in rollups
37 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-03-21.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: result_ffi_guarantees](https://github.com/rust-lang/rfcs/pull/3391)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Initial support for return type notation (RTN)](https://github.com/rust-lang/rust/pull/109010)
* [disposition: merge] [rustdoc: add support for type filters in arguments and generics](https://github.com/rust-lang/rust/pull/108629)
* [disposition: merge] [rustdoc: run more HIR validation to mirror rustc](https://github.com/rust-lang/rust/pull/108576)
* [disposition: merge] [Add a builtin FnPtr trait that is implemented for all function pointers](https://github.com/rust-lang/rust/pull/108080)
* [disposition: merge] [Clarify stability guarantee for lifetimes in enum discriminants](https://github.com/rust-lang/rust/pull/104299)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-03-22 - 2023-04-19 🦀

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

### Asia
* 2023-04-08 | Kyoto, JP | [Kansai Rust](https://www.meetup.com/kansai-rust/)
    * [**Demystifying Closures**](https://www.meetup.com/kansai-rust/events/292202435/) 
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
