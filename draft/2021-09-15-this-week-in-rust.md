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

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

## Crate of the Week

Sadly, we had no nominations this week. Still, in the spirit of not leaving you without some neat rust code, I give you [gradient](https://github.com/mazznoer/gradient-rs), a command line tool to extract gradients from SVG, display and manipulate them.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

300 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-08-30..2021-09-06

* [introduce `let...else`](https://github.com/rust-lang/rust/pull/87688) 
* [update const generics feature gates](https://github.com/rust-lang/rust/pull/88369)
* [allow `~const` bounds on trait assoc functions](https://github.com/rust-lang/rust/pull/88418)
* [emit specific warning to clarify that `#[no_mangle]` should not be applied on foreign statics or functions](https://github.com/rust-lang/rust/pull/86376)
* [fix 2021 dyn suggestion that used code as label](https://github.com/rust-lang/rust/pull/88657)
* [warn when `[T; N].into_iter()` is ambiguous in the new edition](https://github.com/rust-lang/rust/pull/88503)
* [detect bare blocks with type ascription that were meant to be a struct literal](https://github.com/rust-lang/rust/pull/88598)
* [use right span in prelude collision suggestions with macros](https://github.com/rust-lang/rust/pull/88501)
* [improve structured tuple struct suggestion](https://github.com/rust-lang/rust/pull/88631)
* [move global analyses from lowering to resolution](https://github.com/rust-lang/rust/pull/88597)
* [`fmt::Formatter::pad`: don't call `chars().count()` more than one time](https://github.com/rust-lang/rust/pull/88560)
* [add `carrying_add`, `borrowing_sub`, `widening_mul`, `carrying_mul` methods to integers](https://github.com/rust-lang/rust/pull/85017)
* [stabilize `UnsafeCell::raw_get`](https://github.com/rust-lang/rust/pull/88551)
* [stabilize `Iterator::intersperse`](https://github.com/rust-lang/rust/pull/88548)
* [stabilize `std::os::unix::fs::chroot`](https://github.com/rust-lang/rust/pull/88177)
* [compiler-builtins: optimize `memcpy`, `memmove` and `memset`](https://github.com/rust-lang/compiler-builtins/pull/405)
* [futures: add `TryStreamExt::try_forward`, remove `TryStream` bound from `StreamExt::forward`](https://github.com/rust-lang/futures-rs/pull/2469)
* [futures: correcting overly restrictive lifetimes in vectored IO](https://github.com/rust-lang/futures-rs/pull/2484)
* [cargo: stabilize 2021 edition](https://github.com/rust-lang/cargo/pull/9800)
* [cargo: improve error message when unable to initialize git index repo](https://github.com/rust-lang/cargo/pull/9869)
* [clippy: add the `derivable_impls` lint](https://github.com/rust-lang/rust-clippy/pull/7570)
* [rustdoc: clean up handling of lifetime bounds](https://github.com/rust-lang/rust/pull/88604)
* [rustdoc: don't panic on ambiguous inherent associated types](https://github.com/rust-lang/rust/pull/88573)
* [rustdoc: box `GenericArg::Const` to reduce enum size](https://github.com/rust-lang/rust/pull/88574)
* [rustdoc: display associated types of implementors](https://github.com/rust-lang/rust/pull/88490)

### Rust Compiler Performance Triage

A busy week, with lots of mixed changes, though in the end only a few were deemed significant enough to report here.

Triage done by **@pnkfelix**.
Revision range: [fe379..69c4a](https://perf.rust-lang.org/?start=fe37929e4cba2c5c21e6805805769630c736bc3d&end=69c4aa2901ffadf69deaf91b2f90604bcbc2eb36&absolute=false&stat=instructions%3Au)

3 Regressions, 1 Improvements, 3 Mixed; 0 of them in rollups
57 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-09-07.md).

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

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> In Rust, soundness is never just a convention.

– [@H2CO3 on rust-users](https://users.rust-lang.org/t/rationale-behind-fn-fnmut-and-fnonce-design/64355/11)

Thanks to [Riccardo D'Ambrosio](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1097) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
