Title: This Week in Rust 401
Number: 401
Date: 2021-07-28
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

### Project/Tooling Updates

- [Updates from the Tinyverse - July 2021](https://tinyverse.substack.com/p/updates-from-the-tinyverse-july-2021)

### Observations/Thoughts

### Rust Walkthroughs

* [Awesome Unstable Rust Features](https://lazy.codes/posts/awesome-unstable-rust-features/)
* [video] [How to create an AWS Lambda in rust](https://www.youtube.com/watch?v=PmtwtK6jyLc)

### Miscellaneous

## Crate of the Week

This week's crate is [loadstone](https://crates.io/crates/loadstone), a bare-metal bootloader for embedded systems.

Thanks to [Andres O. Vela](https://users.rust-lang.org/t/crate-of-the-week/2704/940) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

287 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-07-19..2021-07-26

* [recognize bounds on impls as const bounds](https://github.com/rust-lang/rust/pull/87273)
* [warn on inert attributes used on bang macro invocation](https://github.com/rust-lang/rust/pull/87296)
* [better diagnostics with mismatched types due to implicit static lifetime](https://github.com/rust-lang/rust/pull/87244)
* [improve `SortedMap::get_by_key_enumerated` more](https://github.com/rust-lang/rust/pull/86429)
* [`VecMap::get_value_matching` should return just one element](https://github.com/rust-lang/rust/pull/86410)
* [don't create references to uninitialized data in `List::from_arena`](https://github.com/rust-lang/rust/pull/87268)
* [miri: better errors for negative out-of-bounds offsets](https://github.com/rust-lang/miri/pull/1853)
* [miri: better ptr-out-of-bounds errors](https://github.com/rust-lang/rust/pull/87224)
* [MIR opt: separate constant predecessors of a switch](https://github.com/rust-lang/rust/pull/85646)
* [stabilize `into_parts()` and `into_error()`](https://github.com/rust-lang/rust/pull/87175)
* [stabilize `impl From<[(K, V); N]> for HashMap` (and friends)](https://github.com/rust-lang/rust/pull/84111)
* [remove `Option` from `BufWriter`](https://github.com/rust-lang/rust/pull/87171)
* [implement `TrustedLen` for `Flatten`/`FlatMap` if the `U: IntoIterator == [T; N]`](https://github.com/rust-lang/rust/pull/87168)
* [add `Stdin::lines`, `Stdin::split` forwarder methods](https://github.com/rust-lang/rust/pull/86847)
* [add support for custom allocator in `VecDeque`](https://github.com/rust-lang/rust/pull/86595)
* [hashbrown: make rehashing and resizing less generic](https://github.com/rust-lang/hashbrown/pull/282)
* [hashbrown: inline small functions](https://github.com/rust-lang/hashbrown/pull/283)
* [clippy: add check if ty `has_escaping_bound_vars` in `zero_sized_map_values` lint](https://github.com/rust-lang/rust-clippy/pull/7470)
* [clippy: improve conflicting rlibs error again](https://github.com/rust-lang/rust-clippy/pull/7495)

### Rust Compiler Performance Triage

Mostly quiet week; improvements outweighed regressions.

Triage done by **@simulacrum**.
Revision range: [9a27044f4..5aff6dd](https://perf.rust-lang.org/?start=9a27044f42ace9eb652781b53f598e25d4e7e918&end=5aff6dd07a562a2cba3c57fc3460a72acb6bef46&absolute=false&stat=instructions%3Au)

1 Regressions, 4 Improvements, 0 Mixed; 0 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-07-13.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: let-else statements](https://github.com/rust-lang/rfcs/pull/3137)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Stabilize Cargo's weak-dep-features and namespaced-features.](https://github.com/rust-lang/rfcs/pull/3143)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize arbitrary_enum_discriminant](https://github.com/rust-lang/rust/pull/86860)
* [disposition: close] [regression: infallible residual could not convert error](https://github.com/rust-lang/rust/issues/86831)
* [disposition: merge] [Allow reifying intrinsics to fn pointers.](https://github.com/rust-lang/rust/pull/86699)
* [disposition: merge] [Commit to not supporting IPv4-in-IPv6 addresses](https://github.com/rust-lang/rust/pull/86335)
* [disposition: merge] [Stabilize core::task::if_ready!](https://github.com/rust-lang/rust/pull/81050)
* [disposition: close] [Implement RFC 2500 Needle API (Part 1)](https://github.com/rust-lang/rust/pull/76901)

### New RFCs

* [Scoped threads in the standard library, take 2](https://github.com/rust-lang/rfcs/pull/3151)

## Upcoming Events

### Online

* [July 21, 2021, Vancouver, BC, CA - Rust Adoption at Huawei - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsycckbcc/)
* [July 22, 2021, Tokyo, JP - Rust LT Online#4 - Rust JP](https://www.youtube.com/watch?v=oK0iJz7XF3Y)
* [July 22, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [July 27, 2021, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrycckbkc/)
* [August 3, 2021, Buffalo, NY, US - Buffalo Rust User Group, First Tuesdays - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/jxfdjsycclbfb/)

### North America

* [July 27, 2021, Chicago, IL, US - Rust in production at Tempus - Chicago Rust Meetup](https://www.meetup.com/Chicago-Rust-Meetup/events/279131036)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Rhebo GmbH**
* [Rust Software Engineer - Focus Networks (DE)](https://germantechjobs.de/en/jobs/Rhebo-GmbH-Softwareentwickler-Rust---Schwerpunkt-Netzwerk-wmd)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> We were able to verify the safety of Rust's type system and thus show how Rust automatically and reliably prevents entire classes of programming errors

– [Ralf Jung on Eureka Alert Science News](https://www.eurekalert.org/pub_releases/2021-07/su-cs071521.php)

Thanks to [Henrik Tougaard](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1084) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
