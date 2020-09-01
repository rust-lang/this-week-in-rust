Title: This Week in Rust 354
Number: 354
Date: 2020-09-02
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/emberian/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/emberian/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/emberian/this-week-in-rust/pulls).

# Updates from Rust Community

### Official

### Tooling

### Newsletters

### Observations/Thoughts

### Learn Standard Rust

### Learn More Rust

### Project Updates

### Miscellaneous

# Crate of the Week

This week's crate is [GlueSQL](https://github.com/gluesql/gluesql), a SQL database engine written in Rust with WebAssembly support.

Thanks to [Taehoon Moon](https://users.rust-lang.org/t/crate-of-the-week/2704/807) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

326 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-08-24..2020-08-31

* [point to a move-related span when pointing to closure upvars](https://github.com/rust-lang/rust/pull/75933)
* [abort when foreign exceptions are caught by `catch_unwind`](https://github.com/rust-lang/rust/pull/70212)
* [new pass to optimize `if` conditions on integrals to switches on the integer](https://github.com/rust-lang/rust/pull/75370)
* [suggest `mem::forget` if `mem::ManuallyDrop::new` isn't used](https://github.com/rust-lang/rust/pull/75912)
* [improve error message when typo is made in `format!`](https://github.com/rust-lang/rust/pull/75779)
* [allow reallocation to different alignment in `AllocRef`](https://github.com/rust-lang/rust/pull/75687)
* [add some avx512f intrinsics for mask, rotation, shift](https://github.com/rust-lang/stdarch/pull/884)
* [make some `Ordering` methods const](https://github.com/rust-lang/rust/pull/75463)
* [stabilize {`Range`, `RangeInclusive`}`::is_empty`](https://github.com/rust-lang/rust/pull/75132)
* [get rid of bounds check in `slice::chunks_exact()` and related functions](https://github.com/rust-lang/rust/pull/75936)
* [stdarch: avx512](https://github.com/rust-lang/stdarch/pull/887)
* [hashbrown: make `with_hasher` functions const fn](https://github.com/rust-lang/hashbrown/pull/195)
* [hashbrown: implement `replace_entry_with`](https://github.com/rust-lang/hashbrown/pull/190)
* [clippy: add a lint for an async block/closure that yields a type that is itself awaitable](https://github.com/rust-lang/rust-clippy/pull/5909)
* [use `rustc_lexer` for rustdoc syntax highlighting](https://github.com/rust-lang/rust/pull/75775)
* [report an ambiguity if both modules and primitives are in scope for intra-doc links](https://github.com/rust-lang/rust/pull/75815)
* [rustdoc: improve rendering of crate features via `doc(cfg)`](https://github.com/rust-lang/rust/pull/75330)
* [docs.rs: separate metadata parsing into a library](https://github.com/rust-lang/docs.rs/pull/1000)

## Rust Compiler Performance Triage

* [2020-08-24](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-08-24.md):
  1 regression, 4 improvements.
  
  This week included a major speedup on optimized builds of real-world crates (up to 5%) as a result of the [upgrade to LLVM 11](https://github.com/rust-lang/rust/pull/73526#issuecomment-679374070).

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.


### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Update the intra-doc link RFC to match the implementation](https://github.com/rust-lang/rfcs/pull/2975)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge][Stabilize deque_make_contiguous](https://github.com/rust-lang/rust/pull/74559)
* [disposition: merge][Tracking issue for hint::spin_loop (renamed sync::atomic::spin_loop_hint)](https://github.com/rust-lang/rust/issues/55002)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [September 2. Johannesburg, ZA - Monthly Joburg Rust Chat! - Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/272786420/)
* [September 2. Dublin, IE - Rust Dublin September Remote Meetup](https://www.meetup.com/Rust-Dublin/events/272781420/?action=rsvp&response=yes)
* [September 2. Indianapolis, IN, US - Indy Rust - Indy.rs - with Social Distancing](https://www.meetup.com/indyrs)
* [September 3. Berlin, DE - Berlin.rs - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybcmbfb/)
* [September 8. Saarbrücken, DE - Rust-Saar Meetup - `3u16.map_err(...)`](https://www.meetup.com/Rust-Saar/events/272522454/)

### North America
* [September 9. Atlanta, GA, US - Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgrybcmbmb/)

### Asia Pacific
* [September 7. Auckland, NZ - Rust AKL - Finally, good Rust IDE support in VSCode: rust-analyzer](https://www.meetup.com/rust-akl/events/266876702/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> When the answer to your question contains the word "variance" you're probably going to have a bad time.

- [trentj on rust-users](https://users.rust-lang.org/t/in-this-mesh-class-whats-wrong-with-my-use-of-lifetimes/47946/4)

Thanks to [Michael Bryan](https://users.rust-lang.org/t/twir-quote-of-the-week/328/937) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust]()</small>
