Title: This Week in Rust 452
Number: 452
Date: 2022-07-20
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
* [Announcing Rust 1.62.1](https://blog.rust-lang.org/2022/07/19/Rust-1.62.1.html)

### Foundation

### Newsletters

### Project/Tooling Updates
* [rust-analyzer changelog #138](https://rust-analyzer.github.io/thisweek/2022/07/18/changelog-138.html)
* [Release Rustlings 5.0.0](https://github.com/rust-lang/rustlings/releases/tag/5.0.0)
* [Rust on Espressif chips - 15-07-2022](https://mabez.dev/blog/posts/esp-rust-15-07-2022/)
* [Fornjot (code-first CAD in Rust) - Weekly Release - 2022-W29](https://www.fornjot.app/blog/weekly-release/2022-w29/)
* [HexoSynth 2022 - Devlog #5 - Signal Monitors and HexoTK Bugfixing](https://m8geil.de/posts/hexosynth-5/)
* [What's new in SeaORM 0.9.0](https://www.sea-ql.org/SeaORM/blog/2022-07-17-whats-new-in-0.9.0/)
* [Slint UI crate weekly updates](https://slint-ui.com/thisweek/2022-07-18.html)

### Observations/Thoughts
* [How to speed up the Rust compiler in July 2022](https://nnethercote.github.io/2022/07/20/how-to-speed-up-the-rust-compiler-in-july-2022.html)
* [How to setup a Wasm API for a CHIP-8 emulator](https://monadical.com/posts/wasm-chip8.html)
* [async let - A new concurrency primitve?](https://conradludgate.com/posts/async-let)
* [Extending SQLite with Rust to support Excel files as virtual tables](https://sergey.khabibullin.com/sqlite-extensions-in-rust/)
* [Pandas vs Polar - A look at performance](https://studioterabyte.nl/en/blog/polars-vs-pandas)
* [Improving “Extract Function” in Rust Analyzer  - Dorian Listens](https://dorianlistens.com/2022/07/improving-extract-function-in-rust-analyzer/)
* [Advice for the next dozen Rust GUIs](https://raphlinus.github.io/rust/gui/2022/07/15/next-dozen-guis.html)
* [When rustc explodes](https://fasterthanli.me/articles/when-rustc-explodes)

### Rust Walkthroughs
* [How Rust manages memory using ownership and borrowing](https://stackoverflow.blog/2022/07/14/how-rust-manages-memory-using-ownership-and-borrowing/)
* [Integrating a Rust module into an Android app](https://blog.logrocket.com/integrating-rust-module-android-app/)
* [Futuristic Rust: context emulation, part 2](https://haibane-tenshi.github.io/rust-contexts2/)
* [Elegant and performant recursion in Rust](https://recursion.wtf/posts/rust_schemes/)
* [DE] [Kommentar: Rust im Linux-Kernel – handeln statt jubeln!](https://www.heise.de/meinung/Kommentar-Rust-im-Linux-Kernel-handeln-statt-jubeln-7179268.html)
* [video] [Stop writing Rust](https://www.youtube.com/watch?v=Z3xPIYHKSoI)

### Research

### Miscellaneous
* [Rust Education Workshop 2022 Call For Participation](https://rust-edu.org/workshop/)

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

<!-- Perf results go here -->

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

* [Rolling co-lead roles for T-compiler](https://github.com/rust-lang/rfcs/commit/2c687cd9712ed4cfe24ac042c12f55b0c908be72)
* [Mention about removal of crate visibility specifier](https://github.com/rust-lang/rfcs/commit/e56963f86d252989bc8b3b55d3a454b9d93ede46)
* [Fixed the associated syntax error in the associated type Trait Headers](https://github.com/rust-lang/rfcs/commit/45d66ff0c063f14371b49caa6dea5d62aa7c9d7e)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: resolve crates.io source replacement ambiguity](https://github.com/rust-lang/rfcs/pull/3289)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [session: stabilize split debuginfo on linux](https://github.com/rust-lang/rust/pull/98051)
* [disposition: merge] [do not mark interior mutable shared refs as dereferenceable](https://github.com/rust-lang/rust/pull/98017)
* [disposition: close] [Tracking issue for `#![register_attr]`](https://github.com/rust-lang/rust/issues/66080)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2022-07-20 - 2022-08-17 🦀

### Virtual

* 2022-07-13 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydckbrb/)
* 2022-07-13 | Malaysia, MY | [Rust Malaysia Meetup](https://rust-malaysia.github.io/meetup/)
    * [**Rust Malaysia Meetup**](https://forms.gle/rFzwUjh5YT1pVci6A)
* 2022-07-14 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydckbsb/)
* 2022-07-14 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/San-Diego-Rust/)
    * [**San Diego Rust July 2022 Tele-Meetup**](https://www.meetup.com/San-Diego-Rust/events/287024976/)
* 2022-07-16 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydckbbc/)
* 2022-07-19 | Sydney, NSW, AU | [Rust Australia](https://github.com/RustAU)
    * [**Rust Lightning Talks**](https://github.com/RustAU/Virtual)
* 2022-07-19 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydckbzb/)
* 2022-07-20 | Cardiff, UK | [Rust and C++ Cardiff ](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust Book Discussion - Building a Multithreaded Web Server**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/287121101/)
* 2022-07-21 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydckbcc/)
* 2022-07-26 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/Dallas-Rust/events/jqxqwrydckbjc/)
* 2022-07-27 | Cardiff, UK | [Rust and C++ Cardiff ](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Using Rust to read the Little Schemer**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/287121637/)
* 2022-07-29 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/Minneapolis-Rust-Meetup/)
    * [**Beginner Rust Open "Office Hours"**](https://www.meetup.com/Minneapolis-Rust-Meetup/events/286993342/)
* 2022-07-31 | Seattle, WA, US | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Intro to Monads for Rustaceans**](https://www.meetup.com/Seattle-Rust-Meetup/events/286692243/)
* 2022-08-02 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydclbdb/)
* 2022-08-03 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydclbfb/)
* 2022-08-03 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydclbfb/)

### Europe

* 2022-07-20 | Wrocław, PL | [Rust Warsaw](https://www.meetup.com/rust-warsaw/)
    * [**Rust Warsaw #5**](https://www.meetup.com/rust-warsaw/events/287093615/)
* 2022-07-21 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #27**](https://www.meetup.com/rust-wroclaw/events/287023750/)
* 2022-06-22 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Developers Amsterdam Group**](https://www.meetup.com/rust-amsterdam-group/events/287019877/)

### North America

* 2022-07-13 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydckbrb/)
* 2022-07-13 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/287009519/)
* 2022-07-14 | Columbus, IL | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydckbsb/)
* 2022-07-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydckbzb/)
* 2022-07-26 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**First Official Meetup - July 2022**](https://www.meetup.com/atx-rustaceans/events/285878081/)
* 2022-08-10 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydclbnb/)

### Oceania

* 2022-07-19 | Phillip, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**Canberra July Meetup**](https://www.meetup.com/rust-canberra/events/286884699/)
* 2022-07-28 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**July Meetup**](https://www.meetup.com/rust-brisbane/events/286889804/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org


<!--

Rust Jobs:

TWiR has removed the jobs posting section. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

<!-- QOTW goes here -->

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
