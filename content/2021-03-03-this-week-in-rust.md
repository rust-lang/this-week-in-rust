Title: This Week in Rust 380
Number: 380
Date: 2021-03-03
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
* [Const generics MVP hits beta!](https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html)
* [Inside] [Lang team March update](https://blog.rust-lang.org/inside-rust/2021/03/03/lang-team-mar-update.html)

### Project/Tooling Updates
* [IntelliJ Rust Changelog #142](https://intellij-rust.github.io/2021/03/01/changelog-142.html)
* [rust-analyzer changelog #66](https://rust-analyzer.github.io/thisweek/2021/03/01/changelog-66.html)
* [Knurling-rs changelog #18](https://ferrous-systems.com/blog/knurling-changelog-18/)
* [Last Month in Flott - March 2021](https://flott-motion.org/news/last-month-in-flott-march-2021/)
* [RampMaker 0.2 - Stepper Motor Acceleration Ramp Generator](https://flott-motion.org/news/ramp-maker-0-2/)

### Observations/Thoughts
* [Why we built the core auth library in Rust (interview with CTO of Oso)](https://console.dev/qa/oso-sam-scott/)
* [Data Manipulation: Pandas vs Rust](https://able.bio/haixuanTao/data-manipulation-pandas-vs-rust--1d70e7fc)
* [Achieving warp speed with Rust](https://gist.github.com/little-dude/674de61df7f48547bdcc5bbe2860563d)
* [Evolution of Kube](https://clux.github.io/probes/post/2021-02-28-kube-evolution/)
* [Temporal RDO update optimization](https://dev.to/luzero/temporal-rdo-update-optimization-2pf1)
* [Introducing The Calypso Chronicles](https://dev.to/thepuzzlemaker/introducing-the-calypso-chronicles-1ff8)
* [Rust: Beware of Escape Sequences\n](https://d3lm.medium.com/rust-beware-of-escape-sequences-85ec90e9e243#ee0e-58229fc84d02)
* [Introducing Rustybot (part 3 of n)](https://objectdisoriented.evokewonder.com/posts/introducing-rustybot-part-3/)
* [Delete Cargo Integration Tests](https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html)

### Rust Walkthroughs
* [C++ to Rust - or how to render your mindset](https://jduchniewicz.com/posts/2021/02/c-to-rust-or-how-to-render-your-mindset/)
* [Generic `impl` blocks are kinda like macros...](https://dev.to/somedood/generic-impl-blocks-are-kinda-like-macros-1aa0)
* [Make a Back-End Number Guessing Game with Rust](https://dev.to/xinnks/make-a-back-end-number-guessing-game-with-rust-1hkj)
* [Captures in closures and async blocks](https://www.fpcomplete.com/blog/captures-closures-async/)
* [Testing a driver crate](https://ferrous-systems.com/blog/test-driver-crate/)
* [Using Rust for AWS Lambdas](https://beanseverywhere.xyz/blog/rust-lambda)
* [Always-On Benchmarking in Rust](https://medium.com/edge-node-engineering/always-on-benchmarking-in-rust-d23f2bac1c1d)
* [Building an OpenStreetMap app in Rust, Part IV](https://blogg.bekk.no/building-an-openstreetmap-app-in-rust-part-iv-d82ebc245381)
* [Solving Advent of Code 2020 in under a second](https://timvisee.com/blog/solving-aoc-2020-in-under-a-second/)
* [The Case for the Typestate Pattern - Introducing Algebraic Data Types](https://www.novatec-gmbh.de/en/blog/the-case-for-the-typestate-pattern-introducing-algebraic-data-types/)
* [DE] [Weniger Frust mit Rust](https://www.golem.de/news/programmiersprachen-weniger-frust-mit-rust-2102-154243.html)
* [video] [Learning Rust: Procedural Macros](https://youtu.be/9IbYBl48eTQ)
* [video] [Rust proxy server with Warp and Hyper](https://youtu.be/eIllaNZisiU)

### Miscellaneous
* [Ferrocene Part 3: The Road to Rust in mission- and safety-critical](https://ferrous-systems.com/blog/ferrocene-update-three-the-road/)
* [How our AWS Rust team will contribute to Rust's future successes](https://aws.amazon.com/blogs/opensource/how-our-aws-rust-team-will-contribute-to-rusts-future-successes/)
* [Librsvg, Rust, and non-mainstream architectures](https://people.gnome.org/~federico/blog/librsvg-rust-and-non-mainstream-architectures.html)
* [Always-On Benchmarking In Rust](https://medium.com/edge-node-engineering/always-on-benchmarking-in-rust-d23f2bac1c1d)
* [DE] [Weniger Frust mit Rust](https://www.golem.de/news/programmiersprachen-weniger-frust-mit-rust-2102-154243.html)

# Crate of the Week

This week's crate is [camino](https://crates.io/crates/camino), a library with UTF-8 coded paths mimicking `std::os::Path`'s API.

Thanks to [piegames](https://users.rust-lang.org/t/crate-of-the-week/2704/886) for the suggestion!

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

402 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-02-22..2021-03-01

* [implement -Z hir-stats for nested foreign items](https://github.com/rust-lang/rust/pull/82258)
* [suggest character encoding is incorrect when encountering random null bytes](https://github.com/rust-lang/rust/pull/81856)
* [suggest `return`ing tail expressions that match return type](https://github.com/rust-lang/rust/pull/81769)
* [improve suggestion for tuple struct pattern matching errors](https://github.com/rust-lang/rust/pull/81235)
* [improve error message when found type is deref of expected](https://github.com/rust-lang/rust/pull/82364)
* [AST: remove some unnecessary boxes](https://github.com/rust-lang/rust/pull/82321)
* [apply lint restrictions from renamed lints](https://github.com/rust-lang/rust/pull/82620)
* [remove storage markers if they won't be used during code generation](https://github.com/rust-lang/rust/pull/78360)
* [remove many `RefCell`s from `DocContext`](https://github.com/rust-lang/rust/pull/82305)
* [prevent computing Item attributes twice](https://github.com/rust-lang/rust/pull/82265)
* [new mir-opt pass to simplify gotos with const values](https://github.com/rust-lang/rust/pull/80475)
* [add an impl of `Error` on `Arc<impl Error>`](https://github.com/rust-lang/rust/pull/80553)
* [make `ptr::write` const](https://github.com/rust-lang/rust/pull/81167)
* [make `char` and `u8` methods const](https://github.com/rust-lang/rust/pull/82078)
* [slight perf improvement on `char::to_ascii_lowercase`](https://github.com/rust-lang/rust/pull/81837)
* [stabilize `str_split_once`](https://github.com/rust-lang/rust/pull/81940)
* [specialize `slice::fill` with `Copy` type and `u8`/`i8`/`bool`](https://github.com/rust-lang/rust/pull/81874)
* [futures: `future::SelectAll::into_inner`](https://github.com/rust-lang/futures-rs/pull/2363)
* [futures: `futures_util::stream::SelectAll::push` should use `&self`](https://github.com/rust-lang/futures-rs/pull/2293)
* [cargo: run rustdoc doctests relative to the workspace](https://github.com/rust-lang/cargo/pull/9105)
* [cargo: throw error if `CARGO_TARGET_DIR` is an empty string](https://github.com/rust-lang/cargo/pull/8939)
* [cargo: add support for `[env]` section in .cargo/config.toml](https://github.com/rust-lang/cargo/pull/9175)
* [cargo: make it more clear which module is being tested when running cargo test](https://github.com/rust-lang/cargo/pull/9195)

## Rust Compiler Performance Triage

Quiet week, a couple regressions and several nice improvements.

Triage done by **@simulacrum**.
Revision range: [301ad8..edeee](https://perf.rust-lang.org/?start=301ad8a4fa3ea56fb980443b7997c8f9d72dd717&end=edeee915b1c52f97411e57ef6b1a8bd46548a37a&absolute=false&stat=instructions%3Au)

2 Regressions, 3 Improvements, 0 Mixed

0 of them in rollups

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)


* [RFC - cargo templates](https://github.com/rust-lang/rfcs/pull/2922)
* [rfc: make cargo install extensible](https://github.com/rust-lang/rfcs/pull/2376)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Deprecate `doc(include)`](https://github.com/rust-lang/rust/pull/82539)
* [disposition: merge] [Implement Extend and FromIterator for OsString](https://github.com/rust-lang/rust/pull/82121)
* [disposition: merge] [Allow specifying alignment for functions](https://github.com/rust-lang/rust/pull/81234)
* [disposition: close] [resolve: allow super in module in block to refer to block items](https://github.com/rust-lang/rust/pull/79309)

## New RFCs

* [A new prelude for the 2021 edition](https://github.com/rust-lang/rfcs/pull/3090)

# Upcoming Events

### Online
* [March 4, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprryccfbgb/)
* [March 9, Saarbücken, Saarland, DE - Meetup: 9u16 (virtual) - Rust Saar](https://www.meetup.com/de-DE/Rust-Saar/events/276401469/)
* [March 9, Buffalo, NY, US - Buffalo Rust User Group - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/276717842/)
* [March 9, Seattle, WA, US - Monthly meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccfbmb/)
* [March 10, New York, NY, US - Seemingly Dark Magic with Rust Types with Nikolai Vazquez - Rust NYC](https://www.meetup.com/Rust-NYC/events/276666844/)
* [March 11, Columbus, OH, US - Monthly Meeting - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgryccfbpb/)
* [March 13th, DE - Chemnitzer Linux Tage - Talk on Rust and its ecosystem](https://chemnitzer.linux-tage.de/2021/en/programm/beitrag/135)
* [March 16, Washington, DC, US - Rust and Tell Lightning Talks - Rust DC](https://www.meetup.com/RustDC/events/kcfpzryccfbpb/)
* [March 17, Vancouver, BC, US - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/npqfbsyccfbwb/)

### North America
* [March 10, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/qxqdgryccfbnb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Crown**

* [Rust Software Engineer (Karlsruhe, DE)](https://www.crown.de/lang/en/company/career/software-developer-rust.html)


**Polymath**

* [Sr. Back End Developer (Remote)](https://polymath.bamboohr.com/jobs/view.php?id=80&source=aWQ9NQ%3D%3D)

**Tweede golf**

* [Lead Developer Embedded Rust (Nijmegen, NL)](https://tweedegolf.nl/vacatures/2/lead-developer-embedded-rust)
* [Embedded Rust Engineer (Nijmegen, NL)](https://tweedegolf.nl/vacatures/11/medior-embedded-engineer)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> It's a great example of the different attitudes of C/C++ and Rust: In C/C++ something is correct when someone can use it correctly, but in Rust something is correct when someone can't use it incorrectly.

– [/u/Janohard on /r/rust](https://www.reddit.com/r/rust/comments/lt4u85/const_generics_mvp_hits_beta/goyg3v4/)

Thanks to [Vlad Frolov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1007) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://this-week-in-rust.org/blog/2021/03/03/this-week-in-rust-380/)</small>
