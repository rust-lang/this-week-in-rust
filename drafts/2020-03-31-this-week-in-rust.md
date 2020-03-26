Title: This Week in Rust 332
Number: 332
Date: 2020-03-31
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

This week's crates is [flume](https://github.com/zesterer/flume), a fast multi-producer single-consumer channel.

Thanks to [Vikrant](https://users.rust-lang.org/t/crate-of-the-week/2704/741) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Announcing Rust IPFS, and a call for contributors](https://blog.ipfs.io/2020-03-18-announcing-rust-ipfs/).
* [The RustConf 2020 CFP is now open](https://cfp.rustconf.com/events/rustconf-2020). We'd love to hear from you at RustConf!
* [This Week in Rust is looking for a new maintainer](https://blog.rust-lang.org/inside-rust/2020/03/13/twir-new-lead.html).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

380 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-03-16..2020-03-23

* [use generator resume arguments in the async/await lowering](https://github.com/rust-lang/rust/pull/69033) (async on core!)
* [async: smaller and more correct generator codegen](https://github.com/rust-lang/rust/pull/69814)
* [implement a feature for a sound specialization subset](https://github.com/rust-lang/rust/pull/68970)
* [`#[track_caller]` in traits](https://github.com/rust-lang/rust/pull/69251)
* [add `#[rustc_layout(debug)]`](https://github.com/rust-lang/rust/pull/69901)
* [parser: recover on `for<'a> |...| body` closures](https://github.com/rust-lang/rust/pull/70209)
* [resolve: print import chains on privacy errors](https://github.com/rust-lang/rust/pull/69811)
* [resolve: do not resolve visibilities on proc macro definitions twice](https://github.com/rust-lang/rust/pull/70233)
* [ast: compress `AttrId` from `usize` to `u32`](https://github.com/rust-lang/rust/pull/70215)
* [fix type of const params in associated types](https://github.com/rust-lang/rust/pull/70223)
* [revised span-to-lines conversion to produce an empty vec on `DUMMY_SP`](https://github.com/rust-lang/rust/pull/70199)
* [rustc: use `LocalDefId` instead of `DefId` in `TypeckTables`](https://github.com/rust-lang/rust/pull/70119)
* [update the mir inline costs](https://github.com/rust-lang/rust/pull/69934)
* [handle `ConstKind::Unresolved` after monomorphizing](https://github.com/rust-lang/rust/pull/70249)
* [perf(dep_graph): avoid allocating a set on when the number reads are small](https://github.com/rust-lang/rust/pull/69778)
* [don't unwind when hitting the macro expansion recursion limit](https://github.com/rust-lang/rust/pull/69497)
* [miri: detect UB: overflow in `copy`/`write_bytes`](https://github.com/rust-lang/miri/pull/1248)
* [allow calculating the layout behind a pointer](https://github.com/rust-lang/rust/pull/69079)
* [make `std::sync::Arc` compatible with ThreadSanitizer](https://github.com/rust-lang/rust/pull/65097)
* [proc_macro_harness: use item header spans for errors](https://github.com/rust-lang/rust/pull/70266)
* [implement `zeroed` and `uninitialized` with `MaybeUninit`](https://github.com/rust-lang/rust/pull/69922)
* [return `NonZeroU64` from `ThreadId::as_u64`](https://github.com/rust-lang/rust/pull/70240)
* [`BTreeMap`: remove shared root](https://github.com/rust-lang/rust/pull/70111)
* [hashbrown: use `NonNull` for the Bucket pointer](https://github.com/rust-lang/hashbrown/pull/148)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2843: Add llvm_asm! and deprecate asm!](https://github.com/rust-lang/rfcs/pull/2843).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for `{f32,f64}::approx_unchecked_to` methods](https://github.com/rust-lang/rust/issues/67058).
* [disposition: merge] [Allow obtaining &mut OsStr](https://github.com/rust-lang/rust/pull/70048).
* [disposition: merge] [impl From<[T; N]> for Vec<T>](https://github.com/rust-lang/rust/pull/68692).
* [disposition: merge] [Implement Hash for Infallible](https://github.com/rust-lang/rust/pull/70281).

## New RFCs

* [Access to traits' associated functions and constants from trait objects](https://github.com/rust-lang/rfcs/pull/2886).
* [Allow specifying dependencies for individual artifacts](https://github.com/rust-lang/rfcs/pull/2887).
* [`ForbiddenValue` trait to enable more optimizations](https://github.com/rust-lang/rfcs/pull/2888).

# Upcoming Events

### Online

* [Mar 28. Stockholm, SE - Stockholm Rust - Rust Discord Hangout - Social Social Distancing](https://www.meetup.com/Stockholm-Rust/events/269572409/).
* [Apr  1. Johannesburg, ZA - Johannesburg Rust Meetup - Remote coffee and chat about Rust](https://www.meetup.com/Johannesburg-Rust-Meetup/events/269648606/).
* [Apr  6. Auckland, NZ - Rust AKL - Using C Libraries in Rust](https://www.meetup.com/rust-akl/events/266876539/).
* [Apr  9. San Diego, CA, US - San Diego Rust - April 2020 Meetup](https://www.meetup.com/San-Diego-Rust/events/269639205/).

### North America

* [Mar 31. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmybcfbpc/).
* [Apr  1. Indianapolis, IN, US - Indy.rs - WebAssembly 101](https://www.meetup.com/indyrs/events/dtqwprybcgbcb/).
* [Apr  9. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybcgbmb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Infrastructure Engineer at Aleph Alpha, Heidelberg, Germany](https://aleph-alpha.de/sw_engineer.html?language=de).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust is funny because in one sense it's hard and clunky. However, it's only ever *precisely as hard and clunky as it needs to be*. Everywhere something can be made more concise, or readable, or convenient, without sacrificing any control, it has been. Anytime something is hard or inconvenient, it's because the underlying domain really is exactly that hard or inconvenient.
>
> Contrast this with other languages, which are often clunky when they don't need to be and/or "easy" when they shouldn't be.

– [brundolf on Hacker News](https://news.ycombinator.com/item?id=22609082)

Thanks to [pitdicker](https://users.rust-lang.org/t/twir-quote-of-the-week/328/837) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
