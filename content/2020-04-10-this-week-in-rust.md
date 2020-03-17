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

* [An alternative to the builder pattern, init struct pattern](https://xaeroxe.github.io/init-struct-pattern/).
* [Bottlerocket, a new Linux-based OS, uses Rust for almost all new components](https://aws.amazon.com/blogs/aws/bottlerocket-open-source-os-for-container-hosting/).
* [Parsing Library in Rust pt. 1](https://blog.frondeus.pl/parser-1/).
* [Yes, I am still learning Rust](https://llogiq.github.io/2020/03/07/learning.html).
* [I have just made my first open source contribution to rust-analyzer](https://avishay.dev/2020/03/04/oss-contribution/).
* [Nannou update - WebGPU, capturing frames and more](https://nannou.cc/posts/nannou_v0.13).
* [A C# programmer examines Rust - Part 1](https://treit.github.io/programming,/rust,/c%23/2020/03/06/StartingRust.html).
* [My exploration of Rust and .NET](https://ericsink.com/entries/dotnet_rust.html).
* [cfg(doctest) is stable and you should use it](https://blog.guillaume-gomez.fr/articles/2020-03-07+cfg%28doctest%29+is+stable+and+you+should+use+it).
* [ExpressJS vs Actix-Web: performance and running cost comparison](https://medium.com/@maxsparr0w/performance-of-node-js-compared-to-actix-web-37f20810fb1a).
* [rust-analyzer changelog 15](https://rust-analyzer.github.io/thisweek/2020/03/09/changelog-15.html).
* [This month in Rust gamedev 7 - February 2020](https://rust-gamedev.github.io/posts/newsletter-007/).
* [The 2020 RustConf CFP is Now Open](https://blog.rust-lang.org/2020/03/10/rustconf-cfp.html).

# Crate of the Week

This week's crates is [plotly](https://github.com/igiagkiozis/plotly), a plotly.js-backed plotting library.

Thanks to [Ioannis Giagkiozis](https://users.rust-lang.org/t/crate-of-the-week/2704/736) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [offst: Add Android builds to CI](https://github.com/freedomlayer/offst/issues/271). Offst is a decentralized payment system.

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

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Amend Rc/Arc::from_raw() docs regarding unsafety](https://github.com/rust-lang/rust/pull/68099).
* [disposition: merge] [`is_x86_feature_detected!("avx512f")` fails to build on nightly](https://github.com/rust-lang/rust/issues/68905).
* [disposition: merge] [Implement `Copy` for `IoSlice`](https://github.com/rust-lang/rust/pull/69403).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Asia Pacific

* [Mar 21-22. Kuala Lumpur, MY - Rust Workshop + Mini-Hackathon](https://docs.google.com/forms/d/e/1FAIpQLScSe4xQycs5i3PtEtR9GAj4vdkWUhwW3v0BiTQFpps4l7PgIA/viewform) [Postponed].

### Europe

* [Mar 19. Warsaw, PL - Rust Warsaw 4](https://www.meetup.com/Rust-Warsaw/events/269164365/).
* [Mar 19. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gztznrybcfbzb/).
* [Mar 26. Edinburgh, GB - Rust Edinburgh - Rust Meetup Short Talk Night](https://www.meetup.com/rust-edi/events/267810816).

### North America

* [Mar 18. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybcfbxb/).
* [Mar 19. Montreal, QC, CA - Rust Montréal - RustMTL March 2020](https://www.meetup.com/Rust-Montreal/events/269117625/).
* [Mar 23. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybcfbfc/).
* [Mar 25. Portland, OR, US - PDXRust - PDX Rust Hack Night](https://www.meetup.com/PDXRust/events/269072568/).
* [Mar 25. Houston, TX, US - Houston Linux Users Group - Rust Study Group](https://www.facebook.com/events/469382520642102).
* [Mar 25. Ann Arbor, MI, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/zdfscrybcfbhc/).
* [Mar 31. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmybcfbpc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Infrastructure Engineer at Aleph Alpha, Heidelberg, Germany](https://aleph-alpha.de/sw_engineer.html?language=de).
* [Kopernikus Automotive GmbH](http://kopernikusauto.com) is [looking for Rust Developers](https://www.reddit.com/r/rust/comments/eyw94s/official_rrust_whos_hiring_thread_for_jobseekers/fk08z9g).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I have no idea how to debug Rust, because in 2 years of Rust, I haven't had that type of low level bug.

– [papaf on hacker news](https://news.ycombinator.com/item?id=22514233)

Thanks to [zrk](https://users.rust-lang.org/t/twir-quote-of-the-week/328/826) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/fhcuec/this_week_in_rust_329/).</small>
