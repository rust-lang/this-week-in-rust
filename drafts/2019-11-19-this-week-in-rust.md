Title: This Week in Rust 313
Number: 313
Date: 2019-11-19
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

### #Rust2020

Find all #Rust2020 posts at [Read Rust](https://readrust.net/rust-2020/).

# Crate of the Week

This week's crate is [wasmtime](https://github.com/bytecodealliance/wasmtime), a standalone JIT-style runtime for WebAssembly.

Thanks to [Josh Triplett](https://users.rust-lang.org/t/crate-of-the-week/2704/671) for the suggestions!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [crates.io: carols10cents will be mentoring multiple issues for the month of November & December](https://github.com/rust-lang/crates.io/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+label%3AE-mentor).
* [simdeez: Create SIMD sin/cos/log etc functions as in agner fog's vector libraries](https://github.com/jackmott/simdeez/issues/17).
* [Spirit: Tokio 0.2 and hyper 0.13 support](https://github.com/vorner/spirit/issues/45).
* [Spirit: Support for slog](https://github.com/vorner/spirit/issues/46).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

252 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-11-11..2019-11-18

* [add a callback that allows compiler consumers to override queries](https://github.com/rust-lang/rust/pull/66297)
* [update LLVM submodule](https://github.com/rust-lang/rust/pull/66318)
* [expand source_util macros with def-site context](https://github.com/rust-lang/rust/pull/66349)
* [improve errors after re rebalance coherence](https://github.com/rust-lang/rust/pull/66253)
* [move `Session` fields to `CrateStore`](https://github.com/rust-lang/rust/pull/66334)
* [improve non-exhaustiveness handling in usefulness checking](https://github.com/rust-lang/rust/pull/66330)
* [refactor integer range handling in the usefulness algorithm](https://github.com/rust-lang/rust/pull/66326)
* [remove some stack frames from `.async` calls](https://github.com/rust-lang/rust/pull/66398)
* [avoid hashing the key twice in `get_query()`](https://github.com/rust-lang/rust/pull/66013)
* [don't warn labels beginning with `_` on unused_labels lint](https://github.com/rust-lang/rust/pull/66419)
* [only include "already existing ..." comment in gitignore on conflict](https://github.com/rust-lang/cargo/pull/7570)
* [suggest borrowing when it would satisfy an unmet trait bound](https://github.com/rust-lang/rust/pull/65456)
* [fully integrate derive helpers into name resolution](https://github.com/rust-lang/rust/pull/64694)
* [push `ast::{ItemKind, ImplItemKind}::OpaqueTy` hack down into lowering](https://github.com/rust-lang/rust/pull/66197)
* [add a HIR pass to check consts for `if`, `loop`, etc.](https://github.com/rust-lang/rust/pull/66170)
* [fix MIR lowering evaluation order and soundness bug](https://github.com/rust-lang/rust/pull/65608)
* [split `ConstValue` into two enums](https://github.com/rust-lang/rust/pull/66233)
* [fix two OOM issues related to `ConstProp`](https://github.com/rust-lang/rust/pull/66394)
* [make dataflow-based const qualification the canonical one](https://github.com/rust-lang/rust/pull/66385)
* [miri: use new isize_max method in FS accesses](https://github.com/rust-lang/miri/pull/1056)
* [miri panic_unwind: fix hack for SEH platforms](https://github.com/rust-lang/rust/pull/66466)
* [make chalk-rust-ir generic over type-family](https://github.com/rust-lang/chalk/pull/284)
* [chalk: refactor fold](https://github.com/rust-lang/chalk/pull/283)
* [chalk: implement `zip_binders` and add some `dyn Trait`/`impl Trait` tests](https://github.com/rust-lang/chalk/pull/282)
* [add `Result::map_or`](https://github.com/rust-lang/rust/pull/66292)
* [fix `HashSet::union` performance](https://github.com/rust-lang/rust/pull/66280)
* [add raw ptr variant of `UnsafeCell::get`](https://github.com/rust-lang/rust/pull/66248)
* [proposal for `BTree`{`Map`, `Set`}`::`{`min`, `max`}](https://github.com/rust-lang/rust/pull/65637)
* [make the semantics of `Vec::truncate(_)` consistent with slices](https://github.com/rust-lang/rust/pull/64432)
* [libc: add support for making functions `const`](https://github.com/rust-lang/libc/pull/1536)
* [cargo: Don't panic when parsing `/proc/stat`](https://github.com/rust-lang/cargo/pull/7580)
* [stabilize rustdoc theme options](https://github.com/rust-lang/rust/pull/54733)
* [rustup build: make clippy faster by using checking before that operation](https://github.com/rust-lang/rustup/pull/2122)
* [rustup: retry downloads](https://github.com/rust-lang/rustup/pull/2121)
* [rustup: fix/improve human-readable units](https://github.com/rust-lang/rustup/pull/2043)
* [measureme: only use 48 bits for encoding timestamps and 32 bits for encoding thread IDs in `RawEvent` in order to make it smaller](https://github.com/rust-lang/measureme/pull/86)

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

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize `!` in Rust 1.41.0](https://github.com/rust-lang/rust/pull/65355).
* [disposition: merge] [Stabilize Result::map_or_else](https://github.com/rust-lang/rust/pull/66322).

## New RFCs

* [Generic parameters in derive](https://github.com/rust-lang/rfcs/pull/2811).
* [Adding is_zero() to core::time::Duration](https://github.com/rust-lang/rfcs/pull/2814).

# Upcoming Events

### Europe

* [Nov 14. Zurich, CH - Rust Zurich - RustFest Decompression Zürich](https://www.meetup.com/Rust-Zurich/events/265593126/).
* [Nov 14. Moscow, RU - Rust Moscow November 2019 Meetup](https://www.meetup.com/ru-RU/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/266184946/).
* [Nov 14. Zagreb, HR - impl Zagreb for Rust - Rust Meetup 201911: Proceduralni makroi](https://www.meetup.com/Zagreb-Rust-Meetup/events/266226748).
* [Nov 15. Barcelona, ES - Rust GTK/GStreamer Workshop at Linux Application Summit 2019](https://www.meetup.com/Barcelona-Free-Software/events/265596417/).
* [Nov 19-21, Rome, Italy - Weekly Rust course at "La Sapienza" University](https://lugsapienza.altervista.org/corsorust-nov2019).
* [Nov 21. Turin, IT - Mozilla Torino - Gruppo di studio Rust](https://www.meetup.com/Mozilla-Torino/events/265961100).
* [Nov 27. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryzpbkc/).

### North America

* [Nov 14. San Diego, CA, US - San Diego Rust November Meetup](https://www.meetup.com/San-Diego-Rust/events/265981542/).
* [Nov 14. Lehi, UT, US - Utah Rust - November 2019 Regular Meetup](https://www.meetup.com/utah-rust/events/265905259/).
* [Nov 14. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgryzpbsb/).
* [Nov 14. Montreal, QC, CA - Montreal Rust Meetup - November 2019 RustMTL: November Common Traits & Causal Profiling](https://www.meetup.com/Rust-Montreal/events/prvrjryzpbqb/).
* [Nov 14. Arlington, VA, US - Rust DC — Mid-month Rustful](https://www.meetup.com/RustDC/events/265769078).
* [Nov 20. Portland, OR, US - PDXRust - Hack Night](https://www.meetup.com/PDXRust/events/265998640/).
* [Nov 20. Ann Arbor, MI, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/zdfscryzpbkc/).
* [Nov 20. Chicago, IL, US - Chicago Rust Meetup - Constructing a Repl(like) from scratch](https://www.meetup.com/Chicago-Rust-Meetup/events/266237535/).
* [Nov 25. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzpbhc/).
* [Nov 26. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyzpbjc/).
* [Nov 27. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryzpbkc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

This week, we have two quotes:

> Telling a programmer there's already a library to do X is like telling a songwriter there's already a song about love.

– [PeteCordell on twitter](https://twitter.com/petecordell/status/428542622844477441), as [quoted in a recent Rust Gamedev meetup](https://www.youtube.com/watch?v=lpOg2nl3kr0)

> Well a Museum purpose is also memory safety, I guess.

– [/u/xav_19 on /r/rust](https://www.reddit.com/r/rust/comments/dxh6pg/why_is_trpl_sold_in_the_gift_shop_at_the_spy/f7r8d3k?utm_source=share&utm_medium=web2x) commenting on a post asking why "The Rust Programming Language" is sold in Washington D.C.'s spy museum's gift shop

Thanks to [Matthieu M.](https://users.rust-lang.org/t/twir-quote-of-the-week/328/737) and [ZiCog](https://users.rust-lang.org/t/twir-quote-of-the-week/328/739) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
