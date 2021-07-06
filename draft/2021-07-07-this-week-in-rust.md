Title: This Week in Rust 398
Number: 398
Date: 2021-07-07
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
* [Inside] [What the Error Handling Project Group is Working Towards](https://blog.rust-lang.org/inside-rust/2021/07/01/What-the-error-handling-project-group-is-working-towards.html)

### Project/Tooling Updates
* [Linux Rust Support Patches](https://lore.kernel.org/lkml/20210704202756.29107-1-ojeda@kernel.org/)
* [Intellij Rust Changelog #150](https://intellij-rust.github.io/2021/07/05/changelog-150.html)
* [Rust Analyzer Changelog #84](https://rust-analyzer.github.io/thisweek/2021/07/05/changelog-84.html)
* [GCC Rust Monthly Report #7 June 2021](https://thephilbert.io/2021/07/05/gcc-rust-monthly-report-7-june-2021/)
* [The future of Buck](https://developers.facebook.com/blog/post/2021/07/01/future-of-buck)
* [ChainSafe's Rust implementation of Mina spec Community Spotlight (video)](https://youtu.be/NaxZR-LDc_g)
* [MoonZoon Dev News (5): Chat example, MoonZoon Cloud](https://dev.to/martinkavik/moonzoon-dev-news-5-chat-example-moonzoon-cloud-5de4)
* [Fluvio: The Programmable Data Platform](https://www.infinyon.com/blog/2021/06/introducing-fluvio/)
* [butido - a Linux Package Building Tool in Rust](https://beyermatthias.de/butido-a-linux-package-building-tool-in-rust)

### Observations/Thoughts
* [Walking through "The Java Tutorials" with Rust - 'What Is an Interface?' and specialization](https://rust-java-tutorials.netlify.app/blog/6-interfaces/)
* [Rust and Tinyverse](https://tinyverse.substack.com/p/rust-and-tinyverse)
* [Speeding Up the Webcola Graph Viz Library with Rust + WebAssembly](https://cprimozic.net/blog/speeding-up-webcola-with-webassembly/)

### Rust Walkthroughs
* [Rust on the front-end](https://blog.frankel.ch/start-rust/5/)
* [Deploy a Rust Web App With Rocket](https://www.koyeb.com/tutorials/deploy-a-rust-web-app-with-rocket)
* [A TCP Proxy in 30 lines of Rust](https://zmedley.com/tcp-proxy.html)
* [Manage WebAssembly Apps in WasmEdge Using Docker Tools](https://www.secondstate.io/articles/manage-webassembly-apps-in-wasmedge-using-docker-tools/)
* [Rust Concept Clarification: Deref vs AsRef vs Borrow vs Cow](https://dev.to/zhanghandong/rust-concept-clarification-deref-vs-asref-vs-borrow-vs-cow-13g6)
* [Thread safety and Learning in Rust](https://dev.to/onesignal/thread-safety-and-learning-in-rust-1p83)
* [REST API Wrapper with Rust](https://dev.to/rogertorres/rest-api-wrapper-with-rust-mk4)
* [video] [Building a Web Application with Rust - Part VII - Auth Middleware](https://www.youtube.com/watch?v=NEyUq5AVF2U)
* [video] [Building a Web Application with Rust - Part VIII - Containerization](https://www.youtube.com/watch?v=iEZAnmVX7yk)
* [video] [Rust & Bevy Tutorial - Building a Game From Scratch (Part 1)](https://www.youtube.com/watch?v=Yb3vInxzKGE)
* [video] [Creating a Chat Server with async Rust and Tokio](https://www.youtube.com/watch?v=4DqP57BHaXI)
* [video] [1Password Developer Fireside Chat: Dive into Async & Futures in Rust](https://www.youtube.com/watch?v=HrxwOUVzyDU)

### Miscellaneous

## Crate of the Week

This week's crate is [hypergraph](https://github.com/yamafaktory/hypergraph), graph data structure implementation where edges can join arbitrary numbers of vertices.

Thanks to [Davy Duperron](https://users.rust-lang.org/t/crate-of-the-week/2704/929) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

284 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-06-21..2021-06-28

* [fix type checking of return expressions outside of function bodies](https://github.com/rust-lang/rust/pull/86206)
* [add `future_prelude_collision` lint](https://github.com/rust-lang/rust/pull/85707)
* [do not emit alloca for ZST locals with multiple assignments](https://github.com/rust-lang/rust/pull/86166)
* [fix panic-safety in specialized `Zip::next_back`](https://github.com/rust-lang/rust/pull/86452)
* [add `io::Cursor::`{`remaining`, `remaining_slice`, `is_empty`}](https://github.com/rust-lang/rust/pull/86037)
* [make `fmt::Arguments::as_str` unstably const](https://github.com/rust-lang/rust/pull/86655)
* [cargo: unify weak and namespaced features](https://github.com/rust-lang/cargo/pull/9574)
* [rustdoc: properly render higher-ranked trait bounds](https://github.com/rust-lang/rust/pull/84814)
* [rustdoc: do not list impl when trait has doc(hidden)](https://github.com/rust-lang/rust/pull/86513)
* [rustdoc: render `<Self as X>::Y` type casts properly across crate bounds](https://github.com/rust-lang/rust/pull/86449)
* [rustdoc: staggered layout for module contents on mobile](https://github.com/rust-lang/rust/pull/85651)
* [clippy: add suspicious group](https://github.com/rust-lang/rust-clippy/pull/7350)

### Rust Compiler Performance Triage

A fairly mixed week with improvements and regressions mostly balancing themselves out. The highlight of this week is we have now started to adopt a new performance triage process which will label PRs that introduce performance regressions with the `perf-regression` label. Authors and/or reviewers are expected to justify their performance regression either by a short summary of why the change is worth it despite the regression or by creating an issue to follow-up on the regression.

We hope this process will lead to better compiler performance in the long term.

Triage done by **@rylev**.
Revision range: [5a78340..9a27044](https://perf.rust-lang.org/?start=5a7834050f3a0ebcd117b4ddf0bc1e8459594309&end=9a27044f42ace9eb652781b53f598e25d4e7e918&absolute=false&stat=instructions%3Au)

2 Regressions, 3 Improvements, 2 Mixed
1 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-07-06.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: let-else statements](https://github.com/rust-lang/rfcs/pull/3137)
* [disposition: merge] [RFC: I/O Safety](https://github.com/rust-lang/rfcs/pull/3128)
* [disposition: merge] [`#[derive(Default)]` on enums with a `#[default]` attribute](https://github.com/rust-lang/rfcs/pull/3107)
* [disposition: close] [New RFC: Collection Transmute](https://github.com/rust-lang/rfcs/pull/2756)
* [disposition: close] [RFC: Add delete and delete_by methods to Iterator](https://github.com/rust-lang/rfcs/pull/2475)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize bindings_after_at](https://github.com/rust-lang/rust/pull/85305)
* [disposition: merge] [Tracking Issue for std::io::Seek::rewind()](https://github.com/rust-lang/rust/issues/85149)
* [disposition: merge] [Stabilize `impl From<[(K, V); N]>` for HashMap (and friends)](https://github.com/rust-lang/rust/pull/84111)
* [disposition: merge] [Stabilize "RangeFrom" patterns in 1.55](https://github.com/rust-lang/rust/pull/83918)
* [disposition: merge] [Tracking Issue for feature(string_drain_as_str) - string::Drain::as_str()](https://github.com/rust-lang/rust/issues/76905)

### New RFCs

* [Candidate Target Policy](https://github.com/rust-lang/rfcs/pull/3145)

## Upcoming Events

### Online

* [July 6, 2021, Buffalo, NY, US - Buffalo Rust User Group, First Tuesdays - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/jxfdjsycckbjb/)
* [July 7, 2021, Denver, CO, US - End-to-end Encrypted Messaging in Rust, with Ockam by Mrinal Wadhwa - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/277633525/)
* [July 13, 2021, Seattle, WA, US - Monthly meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrycckbrb/)
* [July 14, 2021, Malaysia - Rust Meetup July 2021 - Golang Malaysia, feat Rustlang, Erlang, Haskelllang and `.*-?(lang|script)\`](https://docs.google.com/forms/d/e/1FAIpQLSdoVbexvU3TZox1D9yLKPUggeTuih7TEDR6eaFQGTEgJtXZ5g/viewform)
* [July 14, 2021, Dublin, IE - Rust Dublin July Remote Meetup - Rust Dublin](https://www.meetup.com/Rust-Dublin/events/278698763/)

### North America

* [July 14, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/qxqdgrycckbsb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**StructionSite**

* [Backend Engineer - AI Pipeline (Remote)](https://jobs.lever.co/structionsite/3eecbb4d-427b-4e99-87fd-89533b9e7510)

**ChainSafe Systems**

* [Rust Developer (Remote)](https://jobs.smartrecruiters.com/ChainSafeSystemsInc/743999739358248-rust-developer)

**InfinyOn**

* [Senior Rust Engineer(Remote))](https://infinyon.zohorecruit.com/jobs/Careers/619885000000428015/Senior-Rust-Software-Engineer?source=CareerSite)

**Merantix**

* [Senior Software Engineer (Data Infrastructure) (Berlin, DE)](https://arbeitnow.com/view/senior-software-engineer-data-infrastructure-merantix-125225)

**NORICS GmbH**

* [Softwareentwickler (m/w/d) (Norden, DE)](https://www.norics.de/job)

**NZXT**

* [Senior Software Engineer for CAM (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=259)
* [Senior Software Engineer for Streaming Software (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=317)

**Esturary**

* [Several positions available (New York, NY, US and Columbus, OH, US)](https://estuary.dev/careers)

**Kraken**

* [Several positions available (Remote)](https://jobs.lever.co/kraken?team=Engineering)

**Subspace Network**

* [Several positions available (Remote)](https://jobs.lever.co/subspacelabs)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> When a panic has a payload that's an object which needs Drops,
> And the panic hits a catch_unwind for unexpected stops
> Before if its Drop panicked we'd just crash to your desktops,
> Now the payload gets forgotten, and you'd better grab some mops!

– [Josh Triplett on twitter](https://twitter.com/josh_triplett/status/1407776002973986819)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1069) for the self-suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
