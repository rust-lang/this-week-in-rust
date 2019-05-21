Title: This Week in Rust 287
Number: 287
Date: 2019-05-21
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

This week we have two crates: [memory-profiles](https://github.com/nokia/memory-profiler), does what it says on the box. [momo](https://github.com/llogiq/momo) is a procedural macro that outlines generic conversions to reduce monomorphized code. Thanks to [ehsanmok](https://users.rust-lang.org/t/crate-of-the-week/2704/549) and llogiq for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Help answer: What are the similarities and differences between C++'s concepts and Rust’s traits](https://users.rust-lang.org/t/twir-call-for-participation/4821/242)?

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

240 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-05-13..2019-05-20

* [Move token tree related lexer state to a separate struct](https://github.com/rust-lang/rust/pull/60763)
* [Stop using gensyms in HIR lowering](https://github.com/rust-lang/rust/pull/60960)
* [Fix more escaping ReScopes](https://github.com/rust-lang/rust/pull/60765)
* [Perform constant propagation into terminators](https://github.com/rust-lang/rust/pull/60745)
* [Do some simple constant propagation in the `ConstProp` pass](https://github.com/rust-lang/rust/pull/60597)
* [Test interaction of unions with non-zero/niche-filling optimization](https://github.com/rust-lang/rust/pull/60590)
* [Forego caching for all participants in cycles, apart from root node](https://github.com/rust-lang/rust/pull/60444)
* [Mark `core::alloc::Layout::from_size_align_unchecked` const](https://github.com/rust-lang/rust/pull/60370)
* [Remove the unstable and deprecated `mpsc_select`](https://github.com/rust-lang/rust/pull/60921)
* [Stabilize core parts of `MaybeUninit`](https://github.com/rust-lang/rust/pull/60445)
* [Stabilize `vecdeque_rotate`](https://github.com/rust-lang/rust/pull/60678)
* [Add entry-like methods to `HashSet`](https://github.com/rust-lang/rust/pull/60894)
* [Add implementations of `last` in terms of `next_back` on a bunch of `DoubleEndedIterators`](https://github.com/rust-lang/rust/pull/60130)
* [Fix display of const generics in rustdoc](https://github.com/rust-lang/rust/pull/60760)
* [rustup: Avoid blocking on `CloseHandle`](https://github.com/rust-lang/rustup.rs/pull/1850)
* [rustc-guide: Add documentation about profile-guided optimization](https://github.com/rust-lang/rustc-guide/pull/318)
* [lint: convert `incoherent_fundamental_impls` into hard error](https://github.com/rust-lang/rust/pull/49799)
* [clippy: Prevent symbocalypse](https://github.com/rust-lang/rust-clippy/pull/4110)
* [crates.io: Fix performance regression on crate search](https://github.com/rust-lang/crates.io/pull/1746)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2603: Symbol Mangling v2](https://github.com/rust-lang/rfcs/pull/2603).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Named custom cargo profiles](https://github.com/rust-lang/rfcs/pull/2678).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for VecDeque::rotate_{left|right} (feature vecdeque_rotate)](https://github.com/rust-lang/rust/issues/56686).
* [disposition: merge] [std: implement `Error` for `Box<dyn Error>`](https://github.com/rust-lang/rust/pull/58974).
* [disposition: merge] [Stabilize ADX, TBM, and SSE4a target features](https://github.com/rust-lang/rust/pull/60109).
* [disposition: merge] [Allow null-pointer-optimized enums in FFI if their underlying representation is FFI safe](https://github.com/rust-lang/rust/pull/60300).
* [disposition: merge] [Stabilize core parts of MaybeUninit](https://github.com/rust-lang/rust/pull/60445).

## New RFCs

* [Deprecate stdlib modules dedicated to numeric constants and move those constants to associated consts](https://github.com/rust-lang/rfcs/pull/2700).
* [Annotate unwind rust](https://github.com/rust-lang/rfcs/pull/2699).

# Upcoming Events

### Asia

* [May 18. Chennai, IN - Rust Chennai - Monthly meetup - May](https://www.meetup.com/mad-rs/events/261443876/).
* [May 25. Taipei, TW - Rust Taiwan Meetup](https://www.facebook.com/events/381254712479005/).
* [May 27. Tokyo, JP - Tokyo Rust Meetup - Rust LT #5](https://rust.connpass.com/event/129406/).

### Europe

* [May 20. Karlsruhe, DE - Rust Hack & Learn](https://www.meetup.com/Rust-Hack-Learn-Karlsruhe/events/261106439/).
* [May 22. Erlangen, DE - Rust Franken Meetup #0](https://www.meetup.com/Rust-NERF/events/261101152/).
* [May 22. Hamburg, DE - Rust Hack & Learn May 2019](https://www.meetup.com/Rust-Meetup-Hamburg/events/260454690/).
* [May 22. Grenoble, FR - FLOSS Grenoble - Rust meetup](https://www.meetup.com/FLOSS-Grenoble/events/261250845/).
* [May 23. Paris, FR - Rust Paris meetup #45](https://www.meetup.com/Rust-Paris/events/260925527/).
* [May 29. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gkkttqyzhbmc/).
* [May 28. Vilnius, LT - Rust Vilnius - Rust Safety and Distributed Consensus](https://www.meetup.com/Rust-in-Vilnius/events/260937510/).

### North America

* [May 22. Ann Arbor, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/vsncvqyzhbdc/).
* [May 22. Mesa, AZ, US - Desert Rust - Rust: Backend Web Frameworks](https://www.meetup.com/Desert-Rustaceans/events/mkwgvqyzhbdc/).
* [May 27. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzhbkc/).
* [May 28. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyzhblc/).
* [May 29. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/fzqqwqyzhbmc/).
* [May 29. Chicago, IL, US - Chicago Rust Meetup - Unsafe Rust](https://www.meetup.com/Chicago-Rust-Meetup/events/260918979).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Lead Software Engineer at SE4, Tokyo, JP](https://www.linkedin.com/jobs/cap/view/1162802661/).
* [Software Engineer - Backend at SISU, San Francisco, US](https://sisu.ai/careers/?gh_jid=4057600002).
* [Software Engineer at TenX, Singapore](https://tenx.workable.com/jobs/689264).
* [Blockchain Runtime Engineer at Parity, Berlin, DE or remote](https://www.parity.io/jobs/#berlin-blockchain-runtime-engineer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Just the presence of well integrated Algebraic Data Types (ADTs) makes an incredible amount of difference. They are used to represent errors in a meaningful and easy to understand way (`Result<T>`), are used to show that a function may or may not return a meaningful value without needing a garbage value (`Option<T>`), and the optional case can even be used to wrap a null pointer scenario in a safe way (Option<Ref<T>> being the closest to a literal translation I think).
>
> That’s just one small feature that permeates the language. Whatever the opposite of a death-of-a-thousand-cuts is, Rust has it.

[tomcatfish on the orange website](https://news.ycombinator.com/item?id=19922344)

Thanks to [PrototypeNM1](https://users.rust-lang.org/t/twir-quote-of-the-week/328/643) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
