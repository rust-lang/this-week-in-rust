Title: This Week in Rust 325
Number: 325
Date: 2020-02-11
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## News & Blog Posts

* [Debugging Rust in VSCode (in 2020)](https://jason-williams.co.uk/debugging-rust-in-vscode).

# Crate of the Week

This week's crate is [faux](https://github.com/nrxus/faux), a trait-less mocking library for Rust.

Thanks to [Vikrant](https://users.rust-lang.org/t/crate-of-the-week/2704/715) for the suggestions!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [serde_cbor crate is looking for maintainers](https://github.com/pyfisch/cbor/issues/179).
* [Diesel: Looking for persons willing to do some code review on submitted PRs](https://github.com/diesel-rs/diesel/issues/1186).
* [time: Implement function returning the local UTC offset](https://github.com/time-rs/time/issues/203#issuecomment-581175875). Looking for a code review of linked gist.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

291 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-01-27..2020-02-03

* [detect use-after-scope bugs with AddressSanitizer](https://github.com/rust-lang/rust/pull/68572)
* [add support for Control Flow Guard on Windows](https://github.com/rust-lang/rust/pull/68180)
* [add support for enabling the LLVM time-trace feature](https://github.com/rust-lang/rust/pull/68720)
* [suggest defining type parameter when appropriate](https://github.com/rust-lang/rust/pull/68447)
* [do not suggest duplicate bounds](https://github.com/rust-lang/rust/pull/68763)
* [parser: avoid re-wrapping NtItem](https://github.com/rust-lang/rust/pull/68769)
* [parser: syntactically allow `self` in all `fn` contexts](https://github.com/rust-lang/rust/pull/68764)
* [check_match: extract common logic](https://github.com/rust-lang/rust/pull/68571)
* [rustc_span: return an `impl Iterator` instead of a `Vec` from `macro_backtrace`](https://github.com/rust-lang/rust/pull/68407)
* [use `BufWriter` for emitting MIR](https://github.com/rust-lang/rust/pull/68460)
* [change opt-level from 2 back to 3](https://github.com/rust-lang/rust/pull/67878)
* [shrink `Nonterminal`](https://github.com/rust-lang/rust/pull/67340)
* [avoid exponential behaviour when relating types](https://github.com/rust-lang/rust/pull/68772)
* [deduplicate types in the generator witness](https://github.com/rust-lang/rust/pull/68672)
* [add an early-exit to `QueryNormalizer::fold_ty`](https://github.com/rust-lang/rust/pull/68606)
* [add `raw-addr-of` variant to `mir_raw_fat_ptr`](https://github.com/rust-lang/rust/pull/68778)
* [optimize `core::ptr::align_offset`](https://github.com/rust-lang/rust/pull/68787)
* [move numeric consts to associated consts (step 1)](https://github.com/rust-lang/rust/pull/68325)
* [add `Iterator::map_while`](https://github.com/rust-lang/rust/pull/66577)
* [add `BTreeMap::remove_entry`](https://github.com/rust-lang/rust/pull/68378)
* [stabilize `debug_map_key_value`](https://github.com/rust-lang/rust/pull/68200)
* [stabilize `ptr::slice_from_raw_parts`(`_mut`)](https://github.com/rust-lang/rust/pull/68234)
* [stabilize `core::iter::once_with()`](https://github.com/rust-lang/rust/pull/68800)
* [futures: allow async-await macros to be used without std](https://github.com/rust-lang/futures-rs/pull/1891)
* [cargo: swap std::sync::mpsc channel with crossbeam_channel](https://github.com/rust-lang/cargo/pull/7844)
* [cargo: stabilize config-profile](https://github.com/rust-lang/cargo/pull/7823)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

*No issues are currently in final comment period.*

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Europe

* [Feb  7. Darmstadt, DE - Rust Rhein-Main - Rust Meetup – Show Your Project](https://www.meetup.com/Rust-Rhein-Main/events/268145620/).
* [Feb 11. Zurich, CH - Rust Zurich - From cargo to kubernetes and back-up - February Meetup](https://www.meetup.com/Rust-Zurich/events/267790109/).
* [Feb 12. Moscow, RU - Rust Moscow February 2019 Meetup](https://www.meetup.com/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/268190420/).
* [Feb 13. Munich, DE - Rust Munich - Lightning~ish Talks - First Edition](https://www.meetup.com/rust-munich/events/266865499/).
* [Feb 19. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgrybcdbzb/).
* [Feb 20. Turin, IT - Mozilla Torino - Gruppo di studio Rust](https://www.meetup.com/Mozilla-Torino/events/268060855).
* [Feb 21. Stuttgart, DE - Rust Community Stuttgart - Rust Hack and Learn](https://www.meetup.com/Rust-Community-Stuttgart/events/268416708/).

### North America

* [Feb 12. Houston, TX, US - Houston Linux Users Group - Rust Study Group](https://www.facebook.com/events/469382520642102).
* [Feb 12. Portland, OR, US - PDXRust - WASM: Run Rust in the browser](https://www.meetup.com/PDXRust/events/267797263/).
* [Feb 13. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybcdbrb/).
* [Feb 13. San Diego, CA, US - San Diego Rust February 2020 Meetup](https://www.meetup.com/San-Diego-Rust/events/268129845/).
* [Feb 13. Arlington, VA, US - Rust DC — Mid-month Rustful](https://www.meetup.com/RustDC/events/268293591).
* [Feb 18. Redmond, WA, US - Seattle Rust Meetup - Monthly meetup in Redmond](https://www.meetup.com/Seattle-Rust-Meetup/events/prbtdrybcdbpb/).
* [Feb 19. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybcdbzb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> People argue about the color of a bike shed because even though it's a meaningless decision - it's still a decision that has to be made. The null choice is a very bad choice - if you don't paint the shed it'll rust. And there is no "default color" so you can't just say "just color it" - you have to pick a color.
>
> Even seen someone argue about the **pattern** of the shed's paint? No. The pattern is not any more meaningful than the color, but unlike the color - there is a null choice. There is a default. Solid paint. And because there is a default, no one even thinks about using something else because why are you wasting company time and money on a pattern for a bike shed?
>
> From my personal experience, when there is a default and the default is good enough, nobody bikesheds how to derive from the default. They only discuss it when there is a concrete problem with the default, where is doesn't fit your needs for whatever reason. And when you do have a concrete reason to derive from the default - you will derive from the default. Because you have to. And if the library does not support it - you'll switch the library.
>
> Because you have to.

– [/u/someboddy on /r/rust](https://www.reddit.com/r/rust/comments/exbbes/argh_is_googles_opinionated_derivebased_argument/fgdxvt7)

Thanks to [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/804) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
