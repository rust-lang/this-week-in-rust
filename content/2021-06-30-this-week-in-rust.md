Title: This Week in Rust 397
Number: 397
Date: 2021-06-30
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

* [Foundation] [Announcing our Executive Search](https://foundation.rust-lang.org/posts/2021-06-25-announcing-executive-search/)

### Project/Tooling Updates

* [ChainSafe's Polkadot Index Network Token update #2](https://medium.com/chainsafe-systems/pint-community-update-2-b337ece3f031)
* [Rust Analyzer Changelog #83](https://rust-analyzer.github.io/thisweek/2021/06/28/changelog-83.html)
* [Fang](https://www.badykov.com/rust/2021/06/27/fang/)
* [This Week In Veloren 125](https://veloren.net/devblog-125/)
* [This Week In TensorBase 9](https://tensorbase.io/thisweek/2021-06-30-tw_9/)

### Observations/Thoughts

* [Hosting wasm modules in Rust easily using ‘wasmi’](https://blog.knoldus.com/hosting-wasm-modules-in-rust-easily-using-wasmi/)
* [video] [Why the future of the cloud will be built on Rust](https://www.youtube.com/watch?v=BWL4889RKhU&t=5s)

### Rust Walkthroughs

* [Deserializing Binary Data Files in Rust](https://adventures.michaelfbryan.com/posts/deserializing-binary-data-files/)
* [Type-checked keypaths in Rust](http://www.cmyr.net/blog/keypaths.html)
* [Polymorphism in Rust](https://oswalt.dev/2021/06/polymorphism-in-rust/)
* [Cross Compiling Rust Binaries With GitHub Actions](https://www.rohanjain.in/cargo-cross/)
* [Rust #2: Lifetimes, Owners and Borrowers, OH MY!](https://dev.to/cthutu/rust-2-lifetimes-owners-and-borrowers-oh-my-3fem)
* [series] [Build an API in Rust (Part 3)](https://dev.to/naruhodo/build-an-api-in-rust-part-3-11j1)
* [series] [video] [Building a Web Application with Rust - Part V - HTTP Server with Database Manager](https://www.youtube.com/watch?v=TCUnZVLgNps)
* [series] [video] [Building a Web Application with Rust - Part VI - CRUD API](https://www.youtube.com/watch?v=v7y_Ngn_-AY)
* [video] [Beginner's Series to Rust](https://www.youtube.com/playlist?list=PLlrxD0HtieHjbTjrchBwOVks_sr8EVW1x)
* [video] [Building a multithreaded Flutter + Rust App integrating both with Bloc/Cubit and Rid](https://www.youtube.com/watch?v=PGKBdxOA6Xs&t=1s)
* [video] [Implementing Hazard Pointers in Rust](https://www.youtube.com/watch?v=fvcbyCYdR10)
* [video] [Rust Linz; June 2021 - Tim McNamara - How to learn Rust](https://www.youtube.com/watch?v=sDtQaO5_SOw)

### Miscellaneous

* [Programming Rust, 2nd Edition is Available](https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/)
* [Rust in Action is Available](https://www.manning.com/books/rust-in-action)

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

We only have partial results this week (more details in full report). From the results we have collected, we have one small regression and several improvements.
Also, there was a broad [max-rss regression](https://perf.rust-lang.org/compare.html?start=29cd70d40722930e66a8b726fe58a7bd1d64a22b&end=6b354a13820a444f834a33365ae4a8d97d7d27ce&stat=max-rss) from 11 days ago.
and narrower [max-rss regression](https://perf.rust-lang.org/compare.html?start=406d4a9cc3b9601cf98a07c6c83e0227d64f5d48&end=4573a4a879a8e1f773944a8859e4dcd136138af8&stat=max-rss) from 9 days ago.

Triage done by **@pnkfelix**.
Revision range: [406d4a9..5a78340](https://perf.rust-lang.org/?start=406d4a9cc3b9601cf98a07c6c83e0227d64f5d48&end=5a7834050f3a0ebcd117b4ddf0bc1e8459594309&absolute=false&stat=instructions%3Au)
Revision range: [7c3872e..7ede6e2](https://perf.rust-lang.org/?start=7c3872e6bfd555d2ad753ac1f871db3bd7f2a547&end=7ede6e2a2359c1bb9032baffa4fdafe5633749e3&absolute=false&stat=instructions%3Au)


1 Regressions, 5 Improvements, 0 Mixed; 1 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-06-30.md).

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

**Field 33**

* [Senior Software Engineer Backend - Java/Rust (Berlin, DE)](https://arbeitnow.com/view/senior-software-engineer-backend-javarust-fxm-field-33-55295)

**Georg Fischer**

* [Software Engineer (Remote)](https://www.indeed.com/viewjob?cmp=Georg-Fischer-Signet-LLC&t=Software+Engineer&jk=c5a6c3823ac77bd4)

**Rhebo**

* [Softwareentwickler Rust - Schwerpunkt Netzwerk (Remote)](https://rhebo.com/de/unternehmen/karriere/job/senior-software-entwickler-rust-m-w/)

**ChainSafe Systems**

* [Rust Developer (Remote)](https://jobs.smartrecruiters.com/ChainSafeSystemsInc/743999739358248-rust-developer)

**Mimo**

* [Full-time senior Rust developer (Remote)](https://github.com/mimo-capital/jobs/blob/main/Full-time%20senior%20Rust%20developer.md)

**Anixe**

* [Rust Software Engineer (Wrocław, PL)]()

**NZXT**

* [Senior Software Engineer for CAM (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=259)
* [Senior Software Engineer for Streaming Software (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=317)

**Kollider**

* [Junior Backend Engineer (Remote)](https://kollider.homerun.co/junior-backend-engineer/en)
* [Senior Backend Engineer (Remote)](https://kollider.homerun.co/senior-backend-engineer/en)
* [DevOps Engineer (Remote)](https://kollider.homerun.co/devops-engineer/en)

**Ockam**

* [Multiple Rust Engineering Positions Available (Remote)](https://www.ockam.io/team#open-roles)

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/obarj8/this_week_in_rust_397/)</small>
