Title: This Week in Rust 320
Number: 320
Date: 2020-01-07
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

# Crate of the Week

This week's crate is [sqlx](https://crates.io/crates/sqlx), a modern SQL client library.

Thanks to [Jan Riemer](https://users.rust-lang.org/t/crate-of-the-week/2704/698) for the suggestions!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

*No issues were proposed for CfP*.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

207 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-12-30..2019-01-06

* [use function attribute "frame-pointer" instead of "no-frame-pointer-elim"](https://github.com/rust-lang/rust/pull/67748)
* [parser: reduce diversity in error handling mechanisms](https://github.com/rust-lang/rust/pull/67744)
* [allocate HIR on an arena 4/4](https://github.com/rust-lang/rust/pull/67032)
* [improve some `Drop`-related error messages](https://github.com/rust-lang/rust/pull/67823)
* [add symbol normalization for `proc_macro_server`](https://github.com/rust-lang/rust/pull/67702)
* [suggest calling method when first argument is `self`](https://github.com/rust-lang/rust/pull/66913)
* [implement uncommon_codepoints lint](https://github.com/rust-lang/rust/pull/67810)
* [perf: don't recurse into types that do not need normalizing](https://github.com/rust-lang/rust/pull/67808)
* [revert "Add IntoFuture trait and await support" for performance reasons](https://github.com/rust-lang/rust/pull/67768)
* [ensure that we process projections during MIR inlining](https://github.com/rust-lang/rust/pull/67796)
* [miri: update panic machinery to match `#[track_caller]` changes](https://github.com/rust-lang/miri/pull/1137)
* [fix ICE involving calling `Instance.ty` during const evaluation](https://github.com/rust-lang/rust/pull/67800)
* [no longer promote non-pattern const functions](https://github.com/rust-lang/rust/pull/67531)
* [update the barrier cache during ARM EHABI unwinding](https://github.com/rust-lang/rust/pull/67779)
* [add `Iterator::try_find`](https://github.com/rust-lang/rust/pull/63177)
* [cargo: fix CARGO_TARGET_triple_LINKER environment variable](https://github.com/rust-lang/cargo/pull/7763)
* [crates.io: allow multiple keywords in crate search](https://github.com/rust-lang/crates.io/pull/1543)

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

*No RFCs are currently in final comment period.*

## New RFCs

* [-C export-executable-symbols](https://github.com/rust-lang/rfcs/pull/2841).

# Upcoming Events

### Asia Pacific

* [Jan  8. Kuala Lumpur, MY - Rust Meetup January 2019](https://docs.google.com/forms/d/e/1FAIpQLScb1MoYvLE4hfUlUKzg4LJHNI6Abw41hRIQGyBVVIAcwvdGfQ/viewform).

### Europe

* [Jan  8. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgrybccblb/).
* [Jan  9. Lisbon, PT - Rust Lisbon - Live Jan 2020](https://www.meetup.com/Rust-Lisbon/events/266629066/).
* [Jan 10. Darmstadt, DE - Rust Rhein-Main - 1st 2020 Rhein-Main Rust Meetup](https://www.meetup.com/Rust-Rhein-Main/events/267158461/).
* [Jan 16. Turin, IT - Mozilla Torino - Gruppo di studio Rust](https://www.meetup.com/Mozilla-Torino/events/267292439).
* [Jan 17. Stuttgart, DE - Rust Hack and Learn](https://www.meetup.com/de-DE/Rust-Community-Stuttgart/events/267764516).
* [Jan 22. Wrocław, PL - Rust Wrocław Meetup #16](https://www.meetup.com/Rust-Wroclaw/events/267514337/).
* [Jan 23. Warsaw, PL - Rust Warsaw 3](https://www.meetup.com/Rust-Warsaw/events/267525144/).

### North America

* [Jan  6. Houston, TX, US - Houston Linux Users Group - Rust Study Group](https://www.facebook.com/events/469382520642102).
* [Jan  7. Denver, CO, US - Rust Boulder/Denver - Rust Meetup: January](https://www.meetup.com/Rust-Boulder-Denver/events/267240914/).
* [Jan  8. Atlanta, GA, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgrybccblb/).
* [Jan  8. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/qgvxlrybccblb/).
* [Jan  8. Portland, OR, US - PDXRust - C-Side Tourism: Using C libraries from Rust](https://www.meetup.com/PDXRust/events/266938349/).
* [Jan  9. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybccbmb/).
* [Jan  9. San Diego, CA, US - San Diego Rust January 2020 Meetup](https://www.meetup.com/San-Diego-Rust/events/267242856/).
* [Jan  9. Lehi, UT, US - Utah Rust - January 2020 Regular Meetup](https://www.meetup.com/utah-rust/events/265905282/).
* [Jan  9. Arlington, VA, US - Rust DC — Mid-month Rustful](https://www.meetup.com/RustDC/events/266235306).
* [Jan 14. Seattle, WA, US - Seattle Rust Meetup - Physical Computing Workshop](https://www.meetup.com/Seattle-Rust-Meetup/events/267538087/).

### South America

* [Jan 18. Sao Paulo, BR - Rust SP - Encontro Janeiro 2020](https://www.meetup.com/Rust-Sao-Paulo-Meetup/events/266858154/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> *relatively speaking*, my rust programs are like Leonardo DiCaprio in the Revenant, killing grizzly bears with their bare hands, dying and being frozen into a giant ice cubes then, surprise!, they're actually alive.
>
> they can handle a lot, they tend to experience far fewer bugs that come around days or weeks after going into production.
>
> my python programs, otoh, are like William Henry Harrison. Inauguration day! exciting! kind of chilly out here. uh oh -- pneumonia ... dang it!

– [Jonathan Strong on reddit](https://www.reddit.com/r/rust/comments/ehup6r/reddit_on_rust/fcma8y2/,,,)

Thanks to [Jan Riemer](https://users.rust-lang.org/t/twir-quote-of-the-week/328/769) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
