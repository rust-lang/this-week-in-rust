Title: This Week in Rust 293
Number: 293
Date: 2019-07-02
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

This week's crate is [aljabar](https://github.com/maplant/aljabar), an extremely generic linear algebra libary. Thanks to [Vikrant](https://users.rust-lang.org/t/crate-of-the-week/2704/574) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Call for Proposals is open for Rust Belt Rust 2019 until 1 July](http://cfp.rust-belt-rust.com/).
* [Winit 0.20, the state of windowing in Rust, and a request for help](https://users.rust-lang.org/t/winit-0-20-the-state-of-windowing-in-rust-and-a-request-for-help/29485).
* [Request for implementation](https://github.com/dtolnay/request-for-implementation/) - Crates that don't exist but should. Suggest your own design and someone will pick it up.
* [miniz_oxide: Some Panics cause segfaults in jemalloc when called from flate2](https://github.com/Frommi/miniz_oxide/issues/14).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

196 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-06-24..2019-07-01

* [Clean up query cache code](https://github.com/rust-lang/rust/pull/59722)
* [Don't ICE on mutable zst slices](https://github.com/rust-lang/rust/pull/62094)
* [syntax: Remove `ast::Guard`](https://github.com/rust-lang/rust/pull/62075)
* [Always parse `async unsafe fn` + properly ban in 2015](https://github.com/rust-lang/rust/pull/62241)
* [Call out explicitly that general read needs to be called with an initialized buffer](https://github.com/rust-lang/rust/pull/62102)
* [Fix error counting](https://github.com/rust-lang/rust/pull/62055)
* [Use ecx for const-prop local storage](https://github.com/rust-lang/rust/pull/62012)
* [Fix HIR visit order](https://github.com/rust-lang/rust/pull/61572)
* [Extend the `#[must_use]` lint to boxed types](https://github.com/rust-lang/rust/pull/62228)
* [Extend the `#[must_use]` lint to arrays](https://github.com/rust-lang/rust/pull/62235)
* [Clean up MIR drop generation](https://github.com/rust-lang/rust/pull/61872)
* [Use a more efficient iteration order for backward dataflow](https://github.com/rust-lang/rust/pull/62063)
* [Use a more efficient iteration order for forward dataflow](https://github.com/rust-lang/rust/pull/62062)
* [save-analysis: Use buffered writes](https://github.com/rust-lang/rust/pull/62164)
* [Remove `FnBox`](https://github.com/rust-lang/rust/pull/62043)
* [rustdoc: Remove unused derives and variants](https://github.com/rust-lang/rust/pull/62224)
* [rustdoc: Prevent panic when sysroot cannot be computed](https://github.com/rust-lang/rust/pull/61459)
* [backtrace: More improvements to gimli support](https://github.com/rust-lang/backtrace-rs/pull/217)
* [rustup: Switch to `std::fs::read_to_string`](https://github.com/rust-lang/rustup.rs/pull/1906)

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

* [disposition: merge] [Finalize syntax for slice patterns with subslices](https://github.com/rust-lang/rfcs/pull/2359).
* [disposition: merge] [Make `..` a pattern syntactically](https://github.com/rust-lang/rfcs/pull/2707).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize `type_alias_enum_variants` in Rust 1.37.0](https://github.com/rust-lang/rust/pull/61682).

## New RFCs

* [RFC for Vec::append_from_within()](https://github.com/rust-lang/rfcs/pull/2714).

# Upcoming Events

### Africa

* [Jul  3. Johannesburg, ZA - Johannesburg Rust Meetup - informal discussions on topics related to the language](https://www.meetup.com/Johannesburg-Rust-Meetup/events/dgqmbryzkbfb/).

### Asia Pacific

* [Jun 29. Taipei, TW - Rust Taiwan Meetup](https://www.facebook.com/events/2824830874225735/).
* [Jul  1. Auckland, NZ - Rust AKL - WASM - Implementing a scalable omiscient debugger in Rust](https://www.meetup.com/rust-akl/events/259480968/).
* [Jul  7. Tokyo, JP - Tokyo Rust Meetup - Rust LT #6](https://rust.connpass.com/event/133657/).
* [Jul 10. Petaling Jaya, MY - Rust Meetup July 2019](https://docs.google.com/forms/d/e/1FAIpQLSeyDIRlKFE0h4gJ8cxL6tz_3G4p7k4okZZBNhGbuitlOqBJOg/viewform).

### Europe

* [Jun 28-29. Firenze, IT - RustLab 2019](https://www.rustlab.it/).
* [Jul 10. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gkkttqyzkbnb/).

### North America

* [Jul  3. Atlanta, GA, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/kkzkxqyzkbfb/).
* [Jul  3. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyzkbfb/).
* [Jul  9. Redmond, WA, US - Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gfnncryzkbmb/).
* [Jul 10. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/fzqqwqyzkbnb/).
* [Jul 11. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyzkbpb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Infrastructure Engineer at Blue Origin, Kent, WA](https://blueorigin.wd5.myworkdayjobs.com/en-US/BlueOrigin/job/Kent-WA/Software-Infrastructure-Engineer_R661)
* [Senior Software Engineer (Rust) at IOTA, Remote](https://iota.bamboohr.com/jobs/view.php?id=90).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Python and Go pick up your trash for you. C lets you litter everywhere, but throws a fit when it steps on your banana peel. Rust slaps you and demands that you clean up after yourself.

– [Nicholas Hahn on his blog](http://www.nicolas-hahn.com/python/go/rust/programming/2019/07/01/program-in-python-go-rust/)

Thanks to [UtherII](https://users.rust-lang.org/t/twir-quote-of-the-week/328/662) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
