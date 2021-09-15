Title: This Week in Rust 408
Number: 408
Date: 2021-09-15
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Official

### Newsletters

### Project/Tooling Updates

* [Announcing Sycamore v0.6.0: Faster and faster with plenty of fixes and features…](https://sycamore-rs.netlify.app/news/announcing-v0.6.0)
* [SixtyFPS (GUI crate) weekly report 12th of September](https://sixtyfps.io/thisweek/2021-09-13.html)
* [This week in Databend #7](https://datafuselabs.github.io/weekly/2021-09-15-databend-weekly/)

### Observations/Thoughts

### Rust Walkthroughs
* [An experimental Diesel-based CRUD for Rocket](https://tweedegolf.nl/blog/61/an-experimental-diesel-based-crud-for-rocket)
* [Rust cli example #2: Ferris hunts errors](https://dev.to/uggla/rust-cli-example-2-ferris-hunts-errors-116b)
* [video] [FLTK Rust: a new basics tutorial](https://www.youtube.com/watch?v=S1NSsHZs6hI)

### Miscellaneous

* [Native Rust support on Cloudflare Workers](https://blog.cloudflare.com/workers-rust-sdk/)
* [Launching Porta Ecosystem Grants](https://portanetwork.medium.com/launching-porta-ecosystem-grants-to-grow-its-network-7f82262d4260)

## Crate of the Week

This week's crate is [qcell](https://github.com/uazu/qcell), with a type that works like a compile-time `RefCell`.

Thanks to [Soni L.](https://users.rust-lang.org/t/crate-of-the-week/2704/952) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [OpenSUSE Rust Usage Survey](https://survey.opensuse.org/)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

278 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-09-06..2021-09-13

* [fix ICE for functions with more than 65535 arguments](https://github.com/rust-lang/rust/pull/88733)
* [detect stricter constraints on gats where clauses in impls vs trait](https://github.com/rust-lang/rust/pull/88336)
* [ignore derived `Clone` and `Debug` implementations during dead code analysis](https://github.com/rust-lang/rust/pull/85200)
* [fix non-capturing closure return type coercion](https://github.com/rust-lang/rust/pull/88147)
* [suggest items be borrowed in `for i in items[x..]`](https://github.com/rust-lang/rust/pull/88578)
* [suggest wrapping expr in parentheses on invalid unary negation](https://github.com/rust-lang/rust/pull/88757)
* [improve error message when `_` is used for in/inout `asm` operands](https://github.com/rust-lang/rust/pull/88209)
* [emit suggestion when passing byte literal to `format!` macro](https://github.com/rust-lang/rust/pull/87441)
* [use smaller spans for some structured suggestions](https://github.com/rust-lang/rust/pull/87915)
* [use more correct span data in `for` loop desugaring](https://github.com/rust-lang/rust/pull/88214)
* [use `FxHashSet` instead of `Vec` for well formed tys](https://github.com/rust-lang/rust/pull/88771)
* [`mmap` the incremental data instead of reading it](https://github.com/rust-lang/rust/pull/83214)
* [`BTreeMap`/`BTreeSet::from_iter`: use bulk building to improve the performance](https://github.com/rust-lang/rust/pull/88448)
* [add `proc_macro::Span::`{`before`, `after`}](https://github.com/rust-lang/rust/pull/86165)
* [hashbrown: `insert_unique_unchecked` operation](https://github.com/rust-lang/hashbrown/pull/293)
* [clippy: add new lint `iter_not_returning_iterator`](https://github.com/rust-lang/rust-clippy/pull/7610)

### Rust Compiler Performance Triage

Fairly busy week, with some large improvements on several benchmarks. Several
larger rollups landed, in part due to recovery from a temporary CI outage,
and continued CI trouble since then. This is likely the cause for the
somewhat unusual presence of rollups in our results.

Triage done by **@simulacrum**.
Revision range: [69c4aa290..9f85cd6](https://perf.rust-lang.org/?start=69c4aa2901ffadf69deaf91b2f90604bcbc2eb36&end=9f85cd6f2ab2769c16e89dcdddb3e11d9736b351&absolute=false&stat=instructions%3Au)

2 Regressions, 2 Improvements, 4 Mixed; 2 of them in rollups

31 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-09-14.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: close] [Proposal: Else clauses for for and while loops](https://github.com/rust-lang/rfcs/pull/3163)
* [disposition: merge] [Scrape code examples from examples/ directory for Rustdoc](https://github.com/rust-lang/rfcs/pull/3123)
* [disposition: merge] [Rust-lang crate ownership policy](https://github.com/rust-lang/rfcs/pull/3119)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Deprecate array::IntoIter::new](https://github.com/rust-lang/rust/pull/88611)
* [disposition: merge] [Partially stabilize array_methods](https://github.com/rust-lang/rust/pull/88353)
* [disposition: merge] [Tracking issue Iterator map_while](https://github.com/rust-lang/rust/issues/68537)

### New RFCs

* [Add RFC float-next-up-down](https://github.com/rust-lang/rfcs/pull/3173)

## Upcoming Events

### Online

* [September 8, 2021, Denver, CO, US - Rust Q&A - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/279407152/)
* [September 14, 2021, Seattle, WA, US - Monthly Meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccmbsb/)
* [September 15, 2021, Vancouver, BC, CA - Considering Rust - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsyccmbtb/)
* [September 16, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [September 18, 2021, Tokyo, JP - Rust.Tokyo 2021](https://rust.tokyo/)

### North America

* [September 8, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsyccmblb/)
* [September 9, 2021, Pleasant Grove, UT, US - Rusty Engine: A 2D game engine for learning Rust with Nathan Stocks (and Pizza)](https://www.meetup.com/utah-rust/events/280470653/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Indeed**

* [Staff Backend Rust Engineer (Remote)](https://www.indeed.jobs/jobs/staff-backend-rust-engineer-27316/)

**Enso**

* [Senior Rust Developer (Remote)](https://github.com/enso-org/hiring/blob/main/people/senior-rust-developer.md)

**SmartThings**

* [Senior Software Engineer  (Minneapolis, MN, US)](https://smartthings.pinpointhq.com/jobs/22298?utm_medium=internal_referral&utm_source=pinpoint&utm_term=jkypj)

**DEMV Systems**

* [PHP / Rust Backend Entwickler in (Hamburg, DE or Remote) (m/w/d) ](https://arbeitnow.com/view/php-rust-backend-entwickler-in-hamburg-oder-remote-mwd-demv-systems-gmbh-480199)

**Kollider**

* [Junior Backend Engineer (Remote)](https://kollider.homerun.co/junior-backend-engineer/en)

**Polar Sync**

* [Principal/Senior Software Engineer - Rust/C++](https://polarsync.breezy.hr/p/0c1d3630d39d)

**Kraken**

* [Backend Engineer - Rust - Core Backend (Remote)](https://jobs.lever.co/kraken/4019a818-4a7b-46ef-9225-c53c7a7f238c)
* [Backend Engineer, Kraken Futures - Rust (Remote)](https://jobs.lever.co/kraken/fe1e07f4-6d7c-4f65-9a8f-27cf3b3fd2b1)
* [Senior Banking Engineer - Rust (Remote)](https://jobs.lever.co/kraken/2863623f-13c9-4f50-992d-7c25736a60f9)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Edition!

– [Niko and Daphne Matsakis on YouTube](https://www.youtube.com/watch?v=q0aNduqb2Ro)

Thanks to [mark-i-m](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1102) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
