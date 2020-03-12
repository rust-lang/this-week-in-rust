Title: This Week in Rust 329
Number: 329
Date: 2020-03-10
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

* [My exploration of Rust and .NET](https://ericsink.com/entries/dotnet_rust.html).


# Crate of the Week

This week's crates is [plotly](https://github.com/igiagkiozis/plotly), a plotly.js-backed plotting library.

Thanks to [Ioannis Giagkiozis](https://users.rust-lang.org/t/crate-of-the-week/2704/736) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [The RustConf 2020 CFP is now open](https://cfp.rustconf.com/events/rustconf-2020). We'd love to hear from you at RustConf!

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

302 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-03-02..2020-03-09

* [permit attributes on 'if' expressions](https://github.com/rust-lang/rust/pull/69201)
* [const limit for CTFE](https://github.com/rust-lang/rust/pull/67260)
* [invoke `OptimizerLastEPCallbacks` in `PreLinkThinLTO`](https://github.com/rust-lang/rust/pull/69665)
* [fix a leak in `DiagnosticBuilder::into_diagnostic`](https://github.com/rust-lang/rust/pull/69628)
* [when encountering an Item in a pat context, point at the item def](https://github.com/rust-lang/rust/pull/67741)
* [improve linking of crates with circular dependencies](https://github.com/rust-lang/rust/pull/69371)
* [mir-interpret: add method to read wide strings from memory](https://github.com/rust-lang/rust/pull/69326)
* [stabilize `assoc_int_consts` associated int/float constants](https://github.com/rust-lang/rust/pull/68952)
* [add `Layout::dangling()` to return a well-aligned `NonNull<u8>`](https://github.com/rust-lang/rust/pull/69794)
* [fix & test leak of some `BTreeMap` nodes on panic during `into_iter`](https://github.com/rust-lang/rust/pull/69776)
* [hashbrown: add `HashMap::get_key_value_mut`](https://github.com/rust-lang/hashbrown/pull/145)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Cargo report future-incompat](https://github.com/rust-lang/rfcs/pull/2834).
* [disposition: merge] [Add llvm_asm! and deprecate asm!](https://github.com/rust-lang/rfcs/pull/2843).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Permit attributes on 'if' expressions](https://github.com/rust-lang/rust/pull/69201).
* [disposition: merge] [mem::zeroed/uninit: panic on types that do not permit zero-initialization](https://github.com/rust-lang/rust/pull/66059).

## New RFCs

* [Inline assembly](https://github.com/rust-lang/rfcs/pull/2873).
* [Rust-lang org GitHub access policy](https://github.com/rust-lang/rfcs/pull/2872).
* [Clarifying UnwindSafe](https://github.com/rust-lang/rfcs/pull/2871).

# Upcoming Events

### Asia Pacific

* [Mar 21-22. Kuala Lumpur, MY - Rust Workshop + Mini-Hackathon](https://docs.google.com/forms/d/e/1FAIpQLScSe4xQycs5i3PtEtR9GAj4vdkWUhwW3v0BiTQFpps4l7PgIA/viewform).

### Europe

* [Mar  9. Karlsruhe, DE - Rust Meetup](https://www.meetup.com/Rust-Hack-Learn-Karlsruhe/events/268299172/)
* [Mar 11. Oslo, NO - Rust Oslo - Lightning talks](https://www.meetup.com/Rust-Oslo/events/268738879).
* [Mar 12. Turin, IT - Mozilla Torino - Gruppo di studio Rust alla Torino hacknight](https://www.meetup.com/Mozilla-Torino/events/268822794).
* [Mar 19. Warsaw, PL - Rust Warsaw 4](https://www.meetup.com/Rust-Warsaw/events/269164365/).
* [Mar 19. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gztznrybcfbzb/).

### North America

* [Mar 10. Denver, CO, US - Rust Boulder/Denver - March Meetup](https://www.meetup.com/Rust-Boulder-Denver/events/267834799/).
* [Mar 10. Redmond, WA, US - Seattle Rust Meetup - Monthly meetup in Redmond](https://www.meetup.com/Seattle-Rust-Meetup/events/prbtdrybcfbnb/).
* [Mar 11. Atlanta, GA, US - Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgrybcfbpb/).
* [Mar 11. Houston, TX, US - Houston Linux Users Group - Rust Study Group](https://www.facebook.com/events/469382520642102).
* [Mar 11. Portland, OR, US - PDXRust - Rust Lightning Talks! By You](https://www.meetup.com/PDXRust/events/269055813/).
* [Mar 12. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybcfbqb/).
* [Mar 12. Lehi, UT, US - Utah Rust - The Blue Pill: Rust on Microcontrollers](https://www.meetup.com/utah-rust/events/268567961/).
* [Mar 12. San Diego, CA, US - San Diego Rust - March Meetup](https://www.meetup.com/San-Diego-Rust/events/269191953/).
* [Mar 18. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybcfbxb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Infrastructure Engineer at Aleph Alpha, Heidelberg, Germany](https://aleph-alpha.de/sw_engineer.html?language=de).
* [Kopernikus Automotive GmbH](http://kopernikusauto.com) is [searching for Rust Developers.](https://www.reddit.com/r/rust/comments/eyw94s/official_rrust_whos_hiring_thread_for_jobseekers/fk08z9g?utm_source=share&utm_medium=web2x)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I have no idea how to debug Rust, because in 2 years of Rust, I haven't had that type of low level bug.

– [papaf on hacker news](https://news.ycombinator.com/item?id=22514233)

Thanks to [zrk](https://users.rust-lang.org/t/twir-quote-of-the-week/328/826) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
