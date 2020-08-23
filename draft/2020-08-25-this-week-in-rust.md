Title: This Week in Rust 353
Number: 353
Date: 2020-08-25
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/emberian/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/emberian/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/emberian/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*](https://rustacean-station.org/episode/028-twir-352/)

# Updates from Rust Community

### Official

### Tooling

### Newsletters

### Observations/Thoughts

### Learn Standard Rust

### Learn More Rust

* [PL] [CrabbyBird #0 Pierwsza przygoda z Rustem i Godotem](https://postacnormalna.pl/pierwsza-przygoda-z-rustem-i-godotem/)

### Project Updates

### Miscellaneous

# Crate of the Week

This week's crate is [cargo-c](https://github.com/lu-zero/cargo-c), a cargo subcommand to build and install C-ABI compatibile dynamic and static libraries.

Thanks to [Zicklag](https://users.rust-lang.org/t/crate-of-the-week/2704/799) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

345 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-08-10..2020-08-17

* [use existing `infcx` when emitting trait impl diagnostic](https://github.com/rust-lang/rust/pull/75363)
* [detect JS-style `===` and `!==` and recover](https://github.com/rust-lang/rust/pull/75321)
* [detect likely `for foo of bar` JS syntax](https://github.com/rust-lang/rust/pull/75320)
* [move stack size check to `const_eval` machine](https://github.com/rust-lang/rust/pull/75338)
* [add `array` lang item and `[T; N]::map(f: FnMut(T) -> S)`](https://github.com/rust-lang/rust/pull/75212)
* [remove branch in optimized `is_ascii`](https://github.com/rust-lang/rust/pull/74562)
* [constified `str::from_utf8_unchecked`](https://github.com/rust-lang/rust/pull/75157)
* [hard way to respect `BTreeMap`'s minimum node length](https://github.com/rust-lang/rust/pull/75105)
* [BTreeMap: purge innocent use of `into_kv_mut`](https://github.com/rust-lang/rust/pull/75195)
* [hashbrown: implement `FusedIterator` and `size_hint` for `DrainFilter`](https://github.com/rust-lang/hashbrown/pull/188)
* [rustdoc: don't print "const" keyword on non-nightly build if `rustc_const_unstable` is used on the item](https://github.com/rust-lang/rust/pull/74936)

## Rust Compiler Performance Triage

* [2020-08-17](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-08-17.md).
  4 regressions, 3 improvements, 4 mixed bags.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge][Stabilize Range[Inclusive]::is_empty](https://github.com/rust-lang/rust/pull/75132)
* [disposition: merge][ stabilize ptr_offset_from](https://github.com/rust-lang/rust/pull/74238)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [August 19. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out Night](https://www.meetup.com/Vancouver-Rust/events/vcgsvrybclbzb/)
* [August 20. RustConf](https://rustconf.com/)

### North America
* [August 25. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/nppvrrybclbhc/)

### Asia Pacific
* [August 18. Seoul, KR - Rust Meetup - Rust last 6 months review (also available on Zoom)](https://www.meetup.com/Rust-Seoul-Meetup/events/qfkdvrybclbxb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> As Dave Herman always told me, “macros are for when you run out of language”. If you still have language left—and Rust gives you a lot of language—use the language first.

- [Patrick Walton on twitter](https://twitter.com/pcwalton/status/1294676975575896064)

Thanks to [Nixon Enraght-Moony](https://users.rust-lang.org/t/twir-quote-of-the-week/328/926) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust]()</small>
