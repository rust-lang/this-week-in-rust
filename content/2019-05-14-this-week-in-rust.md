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

* 🎈🎉 [Announcing Rust 1.34.2](https://blog.rust-lang.org/2019/05/14/Rust-1.34.2.html). 🎉🎈
* [Security advisory for the standard library](https://blog.rust-lang.org/2019/05/13/Security-advisory.html).
* [Update on the CI investigation](https://internals.rust-lang.org/t/update-on-the-ci-investigation/10056).
* [2D graphics on modern GPU](https://raphlinus.github.io/rust/graphics/gpu/2019/05/08/modern-2d.html).
* [Rust patterns: enums instead of booleans](http://blakesmith.me/2019/05/07/rust-patterns-enums-instead-of-booleans.html).
* [Asymmetric multi-processing on microcontrollers with μAMP](https://blog.japaric.io/microamp/).
* [State of machine learning in Rust](https://ehsanmkermani.com/2019/05/13/state-of-machine-learning-in-rust/).

# Crate of the Week

This week's crate is [panic-never](https://github.com/japaric/panic-never), a crate to make every panic a link-time error. Thanks to [ehsanmok](https://users.rust-lang.org/t/crate-of-the-week/2704/544) for the suggestion!

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

190 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-05-06..2019-05-13

* [Implement built-in `.await` syntax](https://github.com/rust-lang/rust/pull/60586) (Hooray!)
* [Remove the old await! macro](https://github.com/rust-lang/rust/pull/60675)
* [Cleanup the .await HIR lowering with .stmt(..)](https://github.com/rust-lang/rust/pull/60733)
* [Revert "Disable big-endian simd in swap_nonoverlapping_bytes"](https://github.com/rust-lang/rust/pull/60588)
* [syntax: introduce unescape module](https://github.com/rust-lang/rust/pull/60261)
* [syntax_pos: Optimize symbol interner pre-filling slightly](https://github.com/rust-lang/rust/pull/60700)
* [Keep original literal tokens in AST](https://github.com/rust-lang/rust/pull/60679)
* [Tweak `Symbol` and `InternedString`](https://github.com/rust-lang/rust/pull/60659)
* [Use `Symbol` more](https://github.com/rust-lang/rust/pull/60630)
* [Better IO buffer when validating dist hashes](https://github.com/rust-lang/rustup.rs/pull/1845)
* [Remove `hir::ExprKind::If`](https://github.com/rust-lang/rust/pull/59288)
* [Optimize HIR map](https://github.com/rust-lang/rust/pull/60246)
* [Fix HIR printing of existential type](https://github.com/rust-lang/rust/pull/60694)
* [Const-stabilize `NonNull::dangling` and `NonNull::cast`](https://github.com/rust-lang/rust/pull/60244)
* [std: Derive `Default` for `io::Cursor`](https://github.com/rust-lang/rust/pull/60234)
* [cargo: Stabilize offline mode](https://github.com/rust-lang/cargo/pull/6934)
* [cargo: Always include `Cargo.toml` when packaging](https://github.com/rust-lang/cargo/pull/6925)
* [Implement the Cargo half of pipelined compilation](https://github.com/rust-lang/cargo/pull/6883)
* [rustup: More progress bars](https://github.com/rust-lang/rustup.rs/pull/1842)

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

> The big gorilla 3D game framework. Apparently it actually works.

[SimonHeath on Amethyst](https://wiki.alopex.li/AGuideToRustGraphicsLibraries2019)

Thanks to [Magnus Larsen](https://users.rust-lang.org/t/twir-quote-of-the-week/328/640) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/bowbaj/this_week_in_rust_286/).</small>
