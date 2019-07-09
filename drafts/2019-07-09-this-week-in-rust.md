Title: This Week in Rust 294
Number: 294
Date: 2019-07-09
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
* [Retrieve DHCP Logs using Rust Programming](https://blog.knoldus.com/retrieve-dhcp-logs-using-rust-programming/)

# Crate of the Week

This week's crate is [aljabar](https://github.com/maplant/aljabar), an extremely generic linear algebra libary. Thanks to [Vikrant](https://users.rust-lang.org/t/crate-of-the-week/2704/574) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [TiKV: Replace Debug formatting with Display formatting in errors, panics and logs](https://github.com/tikv/tikv/issues/4960).
* [Goblin: Crash on malformed ELF file](https://github.com/m4b/goblin/issues/120). Goblin is an impish, cross-platform binary parsing crate, written in Rust.

gfx-rs introduces the [contributor-friendly](https://github.com/gfx-rs/gfx/issues?q=is%3Aissue+is%3Aopen+label%3Acontributor-friendly) label for issues that are appropriately inviting to new members:

* [Should have a way of telling whether backend is supported](https://github.com/gfx-rs/gfx/issues/2783).
* [A comparative table of HAL features available per backend](https://github.com/gfx-rs/gfx/issues/2547).
* (hal) [Consider removing typed command buffers, pools and queues](https://github.com/gfx-rs/gfx/issues/2862).
* (metal) [Try Kudzu instead of storage-map](https://github.com/gfx-rs/gfx/issues/2860).
* (dx12) [Support VertexIndex and InstanceIndex semantics](https://github.com/gfx-rs/gfx/issues/2589).
* (dx12) [Pipeline caches](https://github.com/gfx-rs/gfx/issues/2877).

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

* [RFC 2359: Finalize syntax for slice patterns with subslices](https://github.com/rust-lang/rfcs/pull/2359).
* [RFC 2707: Make `..` a pattern syntactically](https://github.com/rust-lang/rfcs/pull/2707).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Permit impl Trait in type aliases](https://github.com/rust-lang/rfcs/pull/2515).
* [disposition: merge] [Add key and value methods to DebugMap](https://github.com/rust-lang/rfcs/pull/2696).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize todo macro](https://github.com/rust-lang/rust/pull/61879).
* [disposition: merge] [Add `impl<T> FromIterator<T> for Arc/Rc<[T]>`](https://github.com/rust-lang/rust/pull/61953).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Asia Pacific

* [Jul  7. Tokyo, JP - Tokyo Rust Meetup - Rust LT #6](https://rust.connpass.com/event/133657/).
* [Jul 10. Petaling Jaya, MY - Rust Meetup July 2019](https://docs.google.com/forms/d/e/1FAIpQLSeyDIRlKFE0h4gJ8cxL6tz_3G4p7k4okZZBNhGbuitlOqBJOg/viewform).

### Europe

* [Jul  9. Göteborg, SE - Rust Gbg — July 2019](https://www.meetup.com/rustgbg/events/262786615/).
* [Jul 10. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gkkttqyzkbnb/).
* [Jul 18. Turin, IT - Mozilla Torino - Gruppo di studio Rust](https://www.meetup.com/Mozilla-Torino/events/258593192).

### North America

* [Jul  9. Redmond, WA, US - Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gfnncryzkbmb/).
* [Jul 10. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/fzqqwqyzkbnb/).
* [Jul 11. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyzkbpb/).
* [Jul 11. San Diego, CA, US - San Diego Rust July Meetup](https://www.meetup.com/San-Diego-Rust/events/262650307/).
* [Jul 17. San Francisco, CA, US - Rust in Blockchain San Francisco - In Rust We Trust](https://www.meetup.com/Rust-in-Blockchain-San-Francisco/events/262773260/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Blockchain Runtime Engineer at Parity, Berlin, DE or remote](https://www.parity.io/jobs/#berlin-blockchain-runtime-engineer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Python and Go pick up your trash for you. C lets you litter everywhere, but throws a fit when it steps on your banana peel. Rust slaps you and demands that you clean up after yourself.

– [Nicholas Hahn on his blog](http://www.nicolas-hahn.com/python/go/rust/programming/2019/07/01/program-in-python-go-rust/)

Thanks to [UtherII](https://users.rust-lang.org/t/twir-quote-of-the-week/328/662) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
