Title: This Week in Rust 360
Number: 360
Date: 2020-10-14
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
* [Inside] [Announcing Rust 1.47.0](https://blog.rust-lang.org/2020/10/08/Rust-1.47.html)

### Newsletters
* [This Month in Rust GameDev #14](https://rust-gamedev.github.io/posts/newsletter-014/)

### Tooling
* [Two videos about rust code coverage in VSCode](https://www.marcoieni.com/2020/10/2-videos-about-rust-code-coverage-in-vscode/)

### Observations/Thoughts
* [Rust after the honeymoon](http://dtrace.org/blogs/bmc/2020/10/11/rust-after-the-honeymoon/)

### Learn Simple Rust
* [Iterators in Rust](https://blog.thoughtram.io/iterators-in-rust/)
* [Learn Rust Together Part 5: Structs and Enums!](https://www.youtube.com/watch?v=Iy5pvVPZT50)
* [Learn Rust by building the game Snake](https://blog.scottlogic.com/2020/10/08/lets-build-snake-with-rust.html)

### Learn More Rust
* [Deploying a Rust HTTP server to DigitalOcean App Platform](https://pretired.dazwilkin.com/posts/201008/)
* [ZH] [Build a Shoot 'em up game with framework Amethyst](https://yodalee.me/2020/06/introduction/)
* [Supercharge your Electron apps with Rust](https://blog.logrocket.com/supercharge-your-electron-apps-with-rust/)

### Project Updates

### Miscellaneous

# Call for Blog Posts

The Rust Core Team wants input from the community!
If you haven't already, [read the official blog](https://blog.rust-lang.org/2020/09/03/Planning-2021-Roadmap.html) and submit a blog post - it will show up here!
Here are the wonderful submissions since the call for blog posts:

# Crate of the Week

This week's crate is [paste](https://crates.io/crates/paste), a macro to concatenate identifiers (which would otherwise be nightly only).

Thanks to [mark-i-m](https://users.rust-lang.org/t/crate-of-the-week/2704/825) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

* [Create a Rust-Client for SirixDB](https://dev.to/sirixdb/create-a-rust-client-during-hacktoberfest-5al4)

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

409 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-10-05..2020-10-12

* [resolve: improve "try using the enum's variant"](https://github.com/rust-lang/rust/pull/77341)
* [Fix `LitKind`'s byte buffer to use refcounted slice](https://github.com/rust-lang/rust/pull/77560)
* [Replace `(Body, DefId)` with `Body` where possible](https://github.com/rust-lang/rust/pull/77552)
* [perf: `UninhabitedEnumBranching` avoid n²](https://github.com/rust-lang/rust/pull/77597)
* [Fix span for unicode escape suggestion](https://github.com/rust-lang/rust/pull/77587)
* [Implement `advance_by`, `advance_back_by` for `iter::Chain`](https://github.com/rust-lang/rust/pull/77594)
* [Add `PartialEq` impls for `Vec` ↔ `slice`](https://github.com/rust-lang/rust/pull/74194)
* [stdsimd: Use xor to implement `Neg::neg` for floats](https://github.com/rust-lang/stdsimd/pull/31)

## Rust Compiler Performance Triage

* [2020-10-13](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-10-13.md):
0 Regressions, 3 Improvements, 3 Mixed

Overall, fairly busy week, but without major regressions that need to be addressed.

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-10-13.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [Destructuring assignment](https://github.com/rust-lang/rfcs/pull/2909)
* [RFC: impl-only glob imports](https://github.com/rust-lang/rfcs/pull/2782)
* [RFC: Permit _ in type aliases](https://github.com/rust-lang/rfcs/pull/2524)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [stabilize union with 'ManuallyDrop' fields and 'impl Drop for Union'](https://github.com/rust-lang/rust/pull/77547)
* [disposition: merge] [Set up CI for aarch64-apple-darwin](https://github.com/rust-lang/rust/pull/75991)
* [disposition: merge] [Promote aarch64-pc-windows-msvc to Tier 2 Development Platform](https://github.com/rust-lang/rust/pull/75914)
* [disposition: merge] [resolve: Do not put nonexistent crate `meta` into prelude](https://github.com/rust-lang/rust/pull/75802)
* [disposition: merge] [Tracking issue for slice_partition_at_index](https://github.com/rust-lang/rust/issues/55300)

## New RFCs

* [Add 0000-vecdeque-binary-search.md: Binary search fns for VecDeque](https://github.com/rust-lang/rfcs/pull/2997)

# Upcoming Events

### Online
* [October 7. Johannesburg, ZA - Johannesburg Rust Meetup - Monthly Joburg Rust Chat!](https://www.meetup.com/Johannesburg-Rust-Meetup/events/273455489/)
* [October 7. Dublin, IE - Rust Dublin - October Remote Meetup](https://www.meetup.com/Rust-Dublin/events/273014329/)
* [October 7. Indianapolis, IN, US - Indy.rs - Indy.rs - with Social Distancing](https://www.meetup.com/indyrs/events/jhfstrybcnbkb/)
* [October 8. Linz, AT - Rust Linz - Rust Meetup Linz](https://www.meetup.com/de-DE/Rust-Linz/events/271857253/)
* [October 8. San Diego, CA, US - San Diego Rust - San Diego Rust October 2020 Tele-Meetup](https://www.meetup.com/San-Diego-Rust/events/273486967/)
* [October 12 - 18. RustLab](https://www.rustlab.it/agenda)
* [October 13. Saabrücken, DE - Rust-Saar Meetup - `4u16`](https://www.meetup.com/Rust-Saar/events/273252813/)
* [Octover 15. Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprrybcnbtb/)
* [October 20. Denver, CO, US - Rust Denver - Data Science with Rust](https://www.meetup.com/Rust-Boulder-Denver/events/272996842/)
* [October 21. Vancover, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/cxrtxrybcnbcc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs
* [Lead Software Engineer at Stake Technologies (SG, Remote)](https://angel.co/company/stake-technologies/jobs/1005201-lead-software-engineer)
* [Software Engineer at Stake Technologies (SG, Remote)](https://angel.co/company/stake-technologies/jobs/1005914-software-engineer)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Just because Rust allows you to write super cool non-allocating zero-copy algorithms safely, doesn’t mean every algorithm you write should be super cool, zero-copy and non-allocating.

- [trentj on rust-users](https://users.rust-lang.org/t/feeling-rust-is-so-difficult/29962/15)

Thanks to [Nixon Enraght-Moony](https://users.rust-lang.org/t/twir-quote-of-the-week/328/948) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/iu3ge0/this_week_in_rust_356/)</small>
