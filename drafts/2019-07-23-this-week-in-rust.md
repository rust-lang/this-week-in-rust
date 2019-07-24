Title: This Week in Rust 296
Number: 296
Date: 2019-07-23
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

* [U.S. House Committee on Financial Services hearing: Why was the Rust language chosen](https://www.c-span.org/video/?c4808083/rust-language-chosen)?
* [Microsoft Security Response Center: Why Rust for safe systems programming](https://msrc-blog.microsoft.com/2019/07/22/why-rust-for-safe-systems-programming/).
* [Compiler speed has improved 30-40% across the board year-to-date, with some projects seeing 45%+ improvements](https://www.reddit.com/r/rust/comments/cezxjn/compiler_speed_has_improved_3040_across_the_board/).
* [Perils of constructors](https://matklad.github.io/2019/07/16/perils-of-constructors.html).
* [Notes on a smaller Rust](https://boats.gitlab.io/blog/post/notes-on-a-smaller-rust/).
* [Announcing heim project](https://svartalf.info/posts/2019-07-17-announcing-heim-project/).
* [Gotham — from start to Heroku](https://blog.codeship.com/gotham-from-start-to-heroku/).
* [How to write Full Stack Rust code](https://www.steadylearner.com/blog/read/How-to-write-Full-Stack-Rust-code).
* [Stream combinators implemented using for await syntax](https://www.reddit.com/r/rust/comments/cbvhq9/stream_combinators_implemented_using_for_await/).

# Crate of the Week

This week's crate is [overloadable](https://crates.io/crates/overloadable), a crate to provides you with the capabilities to overload your functions in a similar style to C# or C++, including support for meta attributes, type parameters and constraints, and visibility modifiers
Thanks to [Stevensonmt](https://users.rust-lang.org/t/crate-of-the-week/2704/585) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [RustFest Barcelona - Call for Proposals is open](https://blog.rustfest.eu/cfp-for-barcelona).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

235 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-07-08..2019-07-15

* [Use const generics for array impls [part 1]](https://github.com/rust-lang/rust/pull/62435)
* [Update to LLVM 9](https://github.com/rust-lang/llvm-project/pull/19) (ongoing work)
* [Update LLVM: apply patch necessary for ThinLTO on RISC-V](https://github.com/rust-lang/rust/pull/62463)
* [Use visitor for `#[structural_match]` check](https://github.com/rust-lang/rust/pull/62339)
* [Normalize projections appearing in `impl Trait`](https://github.com/rust-lang/rust/pull/62221)
* [typeck: Merge opaque type inference logic](https://github.com/rust-lang/rust/pull/62090)
* [Fact generation for liveness calculations in Polonius](https://github.com/rust-lang/rust/pull/60266)
* [Add key and value methods to DebugMap](https://github.com/rust-lang/rust/pull/60458)
* [Add an AtomicCell abstraction](https://github.com/rust-lang/rust/pull/62577)
* [Add messages to `Option`'s and `Result`'s `must_use` annotation for `is_*`](https://github.com/rust-lang/rust/pull/62431)
* [Prevent Vec::drain_filter from double dropping on panic](https://github.com/rust-lang/rust/pull/61224)
* [Add `impl<T> FromIterator<T> for Arc/Rc<[T]>`](https://github.com/rust-lang/rust/pull/61953)
* [Add Iterator::partition_in_place() and is_partitioned()](https://github.com/rust-lang/rust/pull/62278)
* [Use `fold` in `Iterator::last` default implementation](https://github.com/rust-lang/rust/pull/62481)
* [rustc guide: Add humor appendix](https://github.com/rust-lang/rustc-guide/pull/350)

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

* [disposition: merge] [SIMD vectors in FFI](https://github.com/rust-lang/rfcs/pull/2574).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Deprecate `try!` macro](https://github.com/rust-lang/rust/pull/62672).
* [disposition: merge] [Stablize Euclidean Modulo (feature euclidean_division)](https://github.com/rust-lang/rust/pull/61884).
* [disposition: merge] [Tracking issue for unstable `--json-rendered` flag](https://github.com/rust-lang/rust/issues/60987).

## New RFCs

* [Fetching cargo registry tokens from external processes](https://github.com/rust-lang/rfcs/pull/2730).
* [General floating point formatting in Debug with `{:g?}`](https://github.com/rust-lang/rfcs/pull/2729).
* [Stabilize volatile copy and set functions](https://github.com/rust-lang/rfcs/pull/2728).

# Upcoming Events

### Africa

* [Aug  7. Johannesburg, ZA - Johannesburg Rust Meetup - futures](https://www.meetup.com/Johannesburg-Rust-Meetup/events/dgqmbryzlbkb/).

### Asia Pacific

* [Jun 25. Seoul, KR - Seoul Rust Meetup, Hapjeong](https://www.meetup.com/Rust-Seoul-Meetup/events/srxvzqyzkbfc/).
* [Aug 10. Singapore, SG - Rust Meetup](https://www.eventbrite.com/e/rust-meetup-tickets-65358532129).
* [Aug  5. Auckland, NZ - Rust AKL August - Rust usage in Firefox](https://www.meetup.com/rust-akl/events/259480991/).

### Europe

* [Aug  7. Erlangen, DE - Rust Franken Meetup #1](https://www.meetup.com/Rust-NERF/events/263163435/).
* [Aug  7. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gkkttqyzlbkb/).

### North America

* [Jul 30. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyzkbnc/).
* [Aug  7. Portland, OR, US - PDXRust - Trees = Boxes + Enums + Iterators](https://www.meetup.com/PDXRust/events/263383260/).
* [Aug  7. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/fzqqwqyzlbkb/).
* [Aug  8. San Diego, CA, US - San Diego Rust August Meetup](https://www.meetup.com/San-Diego-Rust/events/263267320/).
* [Aug  8. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyzlblb/).
* [Aug  7. Atlanta, GA, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/kkzkxqyzlbkb/).
* [Aug  7. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyzlbkb/).

### South America

* [Jul 27. Sao Paulo, BR - Rust SP - Encontro Julho 2019](https://www.meetup.com/Rust-Sao-Paulo-Meetup/events/262488375).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Infrastructure Engineer at Neuralink, San Francisco, US](https://jobs.lever.co/neuralink/efb72bf6-5a55-434e-a0fd-9197f8485b55).
* [Rust Developer at Red Sift, London, UK or remote](https://www.reddit.com/r/rust/comments/cf7hf1/red_sift_looking_for_a_rust_developerc/).
* [Blockchain Runtime Engineer at Parity, Berlin, DE or remote](https://www.parity.io/jobs/#berlin-blockchain-runtime-engineer).
* [Software Engineer at TenX, Singapore](https://tenx.workable.com/jobs/689264).
* [Senior Software Engineer (Rust) at IOTA, Remote](https://iota.bamboohr.com/jobs/view.php?id=90).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust is 5 languages stacked on top of each other, except that instead of ending up like 5 children under a trenchcoat, they end up like the power rangers.

– [reuvenpo on /r/rust](https://reddit.com/r/rust/comments/cb49lb/coworker_rust_doesnt_offer_anything_c_doesnt/etdddwt/)

Thanks to [Jelte Fennema](https://users.rust-lang.org/t/twir-quote-of-the-week/328/666) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
