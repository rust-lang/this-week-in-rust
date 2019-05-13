Title: This Week in Rust 286
Number: 286
Date: 2019-05-14
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

This week's crate is [select-rustc](https://crates.io/crates/select-rustc), a crate for conditional compilation according to rustc version. Thanks to [ehsanmok](https://users.rust-lang.org/t/crate-of-the-week/2704/531) for the suggestion!

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

235 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-04-29..2019-05-06

* [Stop `-O`/`-C opt-level` and `-g`/`-C debuginfo` conflicting](https://github.com/rust-lang/rust/pull/60426)
* [The Genesis of Generic Germination](https://github.com/rust-lang/rust/pull/53645)
* [Avoid repeated interning of static strings](https://github.com/rust-lang/rust/pull/60467)
* [Suggest `try_into` when possible](https://github.com/rust-lang/rust/pull/60159)
* [Suggest using an inclusive range instead of an exclusive range when the endpoint overflows by 1](https://github.com/rust-lang/rust/pull/60330)
* [Search for incompatible universes in borrow errors](https://github.com/rust-lang/rust/pull/60327)
* [Constrain all regions in the concrete type for an opaque type](https://github.com/rust-lang/rust/pull/60449)
* [Const propagation refactoring](https://github.com/rust-lang/rust/pull/60457)
* [Implement `BorrowMut<str>` for `String`](https://github.com/rust-lang/rust/pull/60404)
* [Stabilize vectored IO](https://github.com/rust-lang/rust/pull/60334)
* [Stabilize `str::as_mut_ptr`](https://github.com/rust-lang/rust/pull/60356)
* [Add `Option::flatten` and `Into<Option<_>> for Option<Option<_>>`](https://github.com/rust-lang/rust/pull/60256)
* [cargo: Add some help with updating the registry in offline mode](https://github.com/rust-lang/cargo/pull/6871)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2689: Introduce "compiler-team contributors"](https://github.com/rust-lang/rfcs/pull/2689).
* [RFC 2363: Allow arbitrary enums to have explicit discriminants](https://github.com/rust-lang/rfcs/pull/2363).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Symbol Mangling v2](https://github.com/rust-lang/rfcs/pull/2603).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Document the order of {Vec,VecDeque,String}::retain](https://github.com/rust-lang/rust/pull/60396).
* [disposition: merge] [const-stabilize NonNull::dangling and NonNull::cast](https://github.com/rust-lang/rust/pull/60244).
* [disposition: merge] [std: Derive `Default` for `io::Cursor`](https://github.com/rust-lang/rust/pull/60234).
* [disposition: merge] [Implement DoubleEndedIterator for CaseMappingIter](https://github.com/rust-lang/rust/pull/60112).
* [disposition: merge] [Stabilize the core::array module and reexport in std (for TryFromSliceError)](https://github.com/rust-lang/rust/issues/60014).
* [disposition: merge] [Tracking issue for DoubleEndedIterator::nth_back](https://github.com/rust-lang/rust/issues/56995).

## New RFCs

* [Add key and value methods to DebugMap](https://github.com/rust-lang/rfcs/pull/2696).

# Upcoming Events

### Europe

* [May  9. Wrocław, PL - Rust Wroclaw Meetup #10](https://www.meetup.com/Rust-Wroclaw/events/260858425/).
* [May  9. Berlin, DE - Rust+GNOME 2019 Hackfest#5](https://wiki.gnome.org/Hackfests/Rust2019).
* [May 14. Barcelona, ES - BcnRust Meetup](https://www.meetup.com/BcnRust/events/261043339/).
* [May 15. Berlin, DE - Rust and Rust Berlin Birthday Party](https://www.meetup.com/opentechschool-berlin/events/gkkttqyzhbtb/).
* [May 15. Helsinki, FI - Rust meetup May](https://www.meetup.com/Finland-Rust-Meetup/events/260939025/).
* [May 15. Stuttgart, DE - Workshop: Ownership, Borrowing & Lifetimes](https://www.meetup.com/Rust-Community-Stuttgart/events/261050644/).
* [May 20. Karlsruhe, DE - Rust Hack & Learn](https://www.meetup.com/Rust-Hack-Learn-Karlsruhe/events/261106439/).
* [May 22. Erlangen, DE - Rust Franken Meetup #0](https://www.meetup.com/Rust-NERF/events/261101152/).
* [May 22. Hamburg, DE - Rust Hack & Learn May 2019](https://www.meetup.com/Rust-Meetup-Hamburg/events/260454690/).
* [May 23. Paris, FR - Rust Paris meetup #45](https://www.meetup.com/Rust-Paris/events/260925527/).

### North America

* [May  9. Lehi, UT, US - Meetup #12: Happy Anniversary, Utah Rust](https://www.meetup.com/utah-rust/events/261148910/).
* [May  9. San Diego, US - San Diego Rust May Meetup](https://www.meetup.com/San-Diego-Rust/events/260763786/).
* [May  9. Arlington, US - Rust DC — Mid-month Rustful](https://www.meetup.com/RustDC/events/260559957).
* [May  9. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyzhbmb/).
* [May 14. Seattle, US - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/nzfspqyzhbsb/).
* [May 15. Mexico City, MX - Rust MX - Taller de desarrollo con Rocket](https://www.meetup.com/Rust-MX/events/261254479/).
* [May 15. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/fzqqwqyzhbtb/).
* [May 22. Ann Arbor, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/vsncvqyzhbdc/).
* [May 22. Mesa, AZ, US - Desert Rust - Rust: Backend Web Frameworks](https://www.meetup.com/Desert-Rustaceans/events/mkwgvqyzhbdc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Blockchain Runtime Engineer at Parity, Berlin, DE or remote](https://www.parity.io/jobs/#berlin-blockchain-runtime-engineer).
* [Senior Software Engineer, Backend - Rust at Karken, Remote](https://jobs.lever.co/kraken/4c864c8f-bde6-443d-b521-dd90df0e9105).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> A compile_fail test that fails to fail to compile is also a failure.

[David Tolnay in the try-build README](https://github.com/dtolnay/trybuild/blob/f4abe7607480e74db1905800ea858bab145c3740/README.md)

Llogiq is pretty self-congratulatory for picking this awesome quote.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
