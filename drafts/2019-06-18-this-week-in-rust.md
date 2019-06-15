Title: This Week in Rust 291
Number: 291
Date: 2019-06-18
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

* [gfx-rs project 5 year anniversary](https://gfx-rs.github.io/2019/06/12/anniversary-5.html).
* [Building Secure Systems using RISC-V and Rust (slides from presentation)](https://content.riscv.org/wp-content/uploads/2019/06/14.05-building_secure_systems-1.pdf)

# Crate of the Week

This week's crate is [uom](https://crates.io/crates/uom), Units of measurement is a crate that does automatic type-safe zero-cost dimensional analysis. Thanks to [ehsanmok](https://users.rust-lang.org/t/crate-of-the-week/2704/562) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [good first issue] [blasoxide: Add more tests for level2 and level3](https://github.com/oezgurmakkurt/blasoxide/issues/3).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

242 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-06-03..2019-06-10

* [Stabilize Cell::from_mut and as_slice_of_cells](https://github.com/rust-lang/rust/pull/61620).
* [Stabilize #![feature(repr_align_enum)] in Rust 1.37.0](https://github.com/rust-lang/rust/pull/61229).
* [Implementation of RFC 2289 (associated_type_bounds)](https://github.com/rust-lang/rust/pull/57428).
* [Import the cargo-vendor subcommand into Cargo](https://github.com/rust-lang/cargo/pull/6869).
* [Support ? Kleene macro operator in 2015](https://github.com/rust-lang/rust/pull/60932).
* [Add std::mem::take as suggested in #61129](https://github.com/rust-lang/rust/pull/61130).
* [Make tuple constructors real const fns](https://github.com/rust-lang/rust/pull/61209).
* [syntax: Remove `Deref` impl from `Token`](https://github.com/rust-lang/rust/pull/61669).
* [Make `i*::signum` a `const fn`](https://github.com/rust-lang/rust/pull/61635).
* [Remove useless allocations in macro_rules follow logic](https://github.com/rust-lang/rust/pull/61646).
* [Minimize use of `#![feature(custom_attribute)]`](https://github.com/rust-lang/rust/pull/61660).
* [parser: Remove `Deref` impl from `Parser`](https://github.com/rust-lang/rust/pull/61616).
* [Change visit api](https://github.com/rust-lang/rust/pull/61554).
* [Don't allow using const fn arguments as "args_required_const"](https://github.com/rust-lang/rust/pull/61536).
* [Support Rvalue::{Ref,Len} and Deref](https://github.com/rust-lang/rust/pull/61532).
* [Fix integer overflow in rotate_left](https://github.com/rust-lang/rust/pull/61454).
* [On TerminatorKind::DropAndReplace still handle unused_mut correctly](https://github.com/rust-lang/rust/pull/61446).
* [Use LLVM intrinsics for floating-point min/max](https://github.com/rust-lang/rust/pull/61408).
* [Add new diagnostic writer using annotate-snippet library](https://github.com/rust-lang/rust/pull/61407).
* [Add Bound::cloned()](https://github.com/rust-lang/rust/pull/61376).
* [Refactor `TypeVariableOrigin`](https://github.com/rust-lang/rust/pull/59331).
* [Replace linear token counting macros with optimized implementation](https://github.com/rust-lang/rust/pull/59600).
* [Suggest using `as_ref` on `*const T`](https://github.com/rust-lang/rust/pull/61444).
* [Re-implement async fn drop order lowering](https://github.com/rust-lang/rust/pull/61413).
* [syntax: Keep token span as a part of `Token`](https://github.com/rust-lang/rust/pull/61541).

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

* [disposition: merge] [Expose the type_name intrinsic](https://github.com/rust-lang/rfcs/issues/1428).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Support `cfg` and `cfg_attr` on generic parameters](https://github.com/rust-lang/rust/pull/61547).
* [disposition: merge] [Stabilize underscore_const_names in 1.37.0](https://github.com/rust-lang/rust/pull/61347).
* [disposition: merge] [Stabilize support for Profile-guided Optimization](https://github.com/rust-lang/rust/pull/61268).

## New RFCs

* [Generic Pointer to Field](https://github.com/rust-lang/rfcs/pull/2708).

# Upcoming Events

### Asia Pacific

* [Jun 15. Chennai, IN - Rust Chennai - Monthly meetup - June](https://www.meetup.com/mad-rs/events/262191938/).
* [Jun 25. Seoul, KR - Seoul Rust Meetup, Hapjeong - Deep dive into Rusts standard library](https://www.meetup.com/Rust-Seoul-Meetup/events/srxvzqyzjbhc/).

### Europe

* [Jun 18. London, UK - Rust London User Group - LDN Talks June 2019](https://www.meetup.com/Rust-London-User-Group/events/262000795/).
* [Jun 26. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gkkttqyzjbjc/).
* [Jun 26. Milano, IT - Rust Language Milano - Fun with Rusty Robots](https://www.meetup.com/rust-language-milano/events/262155219).
* [Jun 28-29. Firenze, IT - RustLab 2019](https://www.rustlab.it/).

### North America

* [Jun 13. San Diego, CA, US - San Diego Rust May Meetup](https://www.meetup.com/San-Diego-Rust/events/261595821/).
* [Jun 13. Arlington, VA, US - Rust DC — Mid-month Rustful](https://www.meetup.com/RustDC/events/261239650).
* [Jun 13. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyzjbrb/).
* [Jun 13. Lehi, UT, US - Utah Rust - Meetup #13: Serde, Serde, and More Serde](https://www.meetup.com/utah-rust/events/262109363).
* [Jun 18. Denver, CO, US - Rust Boulder/Denver - Rust Meetup for June](https://www.meetup.com/Rust-Boulder-Denver/events/259124426/).
* [Jun 19. Mexico City, MX - Rust MX - Reunión junio: Hablemos de Fuchsia OS y WebAssembly](https://www.meetup.com/Rust-MX/events/261739565/).
* [Jun 24. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzjbgc/).
* [Jun 25. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyzjbhc/).
* [Jun 26. Ann Arbor, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/vsncvqyzjbjc/).
* [Jun 26. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/fzqqwqyzjbjc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Blockchain Runtime Engineer at Parity, Berlin, DE or remote](https://www.parity.io/jobs/#berlin-blockchain-runtime-engineer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
