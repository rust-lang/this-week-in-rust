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

* [Writing an OS in Rust: async/await](https://os.phil-opp.com/async-await/).
* [rustc-dev-guide overview](https://blog.rust-lang.org/inside-rust/2020/03/26/rustc-dev-guide-overview.html).
* [Tokio: Reducing tail latencies with automatic cooperative task yielding](https://tokio.rs/blog/2020-04-preemption/).
* [Introducing R2, a router in Rust](https://r2.rs/blog/).
* [Goodbye, docs team](https://blog.rust-lang.org/inside-rust/2020/03/27/goodbye-docs-team.html).
* [Traits working group 2020 sprint 1 summary](https://blog.rust-lang.org/inside-rust/2020/03/28/traits-sprint-1.html).
* [Specs and Legion, two very different approaches to ECS](https://csherratt.github.io/blog/posts/specs-and-legion/).
* [Reducing memory consumption in librsvg, part 4: compact representation for Bézier paths](https://people.gnome.org/~federico/blog/reducing-memory-consumption-in-librsvg-4.html).
* [State machines in Rust](https://blog.yoshuawuyts.com/state-machines/).
* [Macros vs rename](https://rust-analyzer.github.io/blog/2020/03/30/macros-vs-rename.html).
* [More advanced aspects of pattern matching in Rust](https://notes.iveselov.info/programming/refs-and-pattern-matching-in-rust).
* [What constitutes a vulnerability](https://boats.gitlab.io/blog/post/vulnerabilities/)?
* [On generics and associated types](https://thomashartmann.dev/blog/on-generics-and-associated-types/).
* [rust-analyzer changelog 18](https://rust-analyzer.github.io/thisweek/2020/03/30/changelog-18.html).
* [IntelliJ Rust changelog 119](https://intellij-rust.github.io/2020/03/30/changelog-119.html).
* [Rust in blockchain 10](https://rustinblockchain.org/newsletters/2020-04-01-keep-calm-and-hack-more/).

# Crate of the Week

This week's crates is [async-recursion](https://github.com/dcchut/async-recursion), a macro to allow recursion in async functions.

Thanks to [Zicklag](https://users.rust-lang.org/t/crate-of-the-week/2704/744) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [rust: Implement conversion traits for usize/isize together with a portability lint](https://github.com/rust-lang/rust/issues/70460).
* [This Week in Rust is looking for a new maintainer](https://blog.rust-lang.org/inside-rust/2020/03/13/twir-new-lead.html). (If you're interested, please [fill this form](https://docs.google.com/forms/d/e/1FAIpQLScJUYdkpLwQc_4zn3oxpVLdT3IHtKlYEBAzd6lgSgE3vsTOtA/viewform)).
* [The RustConf 2020 CFP is now open](https://cfp.rustconf.com/events/rustconf-2020). We'd love to hear from you at RustConf!
* [dotenv-linter is looking for contributors](https://twitter.com/mgrachev/status/1241700876244434951).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

468 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-03-23..2020-03-30

* [permit negative impls for non-auto traits](https://github.com/rust-lang/rust/pull/68004)
* [parser: recover on `...` as a pattern, suggesting `..`](https://github.com/rust-lang/rust/pull/70417)
* [clean up debugging options](https://github.com/rust-lang/rust/pull/70297)
* [evaluate repeat expression lengths as late as possible](https://github.com/rust-lang/rust/pull/69981)
* [fix cycle error when emitting suggestion for mismatched `fn` type](https://github.com/rust-lang/rust/pull/69936)
* [fix smaller issues with invalid placeholder type errors](https://github.com/rust-lang/rust/pull/70369)
* [fix incorrect pattern warning "unreachable pattern"](https://github.com/rust-lang/rust/pull/70413)
* [account for bad placeholder types in where clauses](https://github.com/rust-lang/rust/pull/70294)
* [tweak chained operators diagnostic](https://github.com/rust-lang/rust/pull/69878)
* [remove const eval loop detector](https://github.com/rust-lang/rust/pull/70087)
* [correctly normalize constants](https://github.com/rust-lang/rust/pull/70319)
* [perf: avoid allocating a set on dep graph when the number reads are small](https://github.com/rust-lang/rust/pull/69778)
* [refactor object file handling](https://github.com/rust-lang/rust/pull/70384)
* [`#[track_caller]` on `core::ops::`{`Index`, `IndexMut`}](https://github.com/rust-lang/rust/pull/70234)
* [add `Result<Result<T, E>, E>::flatten -> Result<T, E>`](https://github.com/rust-lang/rust/pull/70140)
* [add copy bound to atomic & numeric intrinsics](https://github.com/rust-lang/rust/pull/70101)
* [ASCII methods on `OsStr`](https://github.com/rust-lang/rust/pull/69937)
* [add `Wake` trait for safe construction of `Waker`s](https://github.com/rust-lang/rust/pull/68700)
* [`impl From<[T; N]> for Vec<T>`](https://github.com/rust-lang/rust/pull/68692)
* [`fold_self` and `try_fold_self` for Iterators](https://github.com/rust-lang/rust/pull/65222)
* [fix `TryEnterCriticalSection` return type](https://github.com/rust-lang/rust/pull/70510)
* [regex: add fast path for `c_char`](https://github.com/rust-lang/regex/pull/658)
* [regex: improve allocation of `escape_into`](https://github.com/rust-lang/regex/pull/655)

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

* [disposition: merge] [Tracking issue for `{f32,f64}::to_int_unchecked` methods](https://github.com/rust-lang/rust/issues/67058).

## New RFCs

* [Add a `trustme` keyword to define unsafe code blocks](https://github.com/rust-lang/rfcs/pull/2893).
* [documentation-in-code-block-in-documentation](https://github.com/rust-lang/rfcs/pull/2894).
* [YieldSafe auto trait](https://github.com/rust-lang/rfcs/pull/2890).

# Upcoming Events

### Online

* [Apr  6. Auckland, NZ - Rust AKL - Using C Libraries in Rust](https://www.meetup.com/rust-akl/events/266876539/).
* [Apr  9. San Diego, CA, US - San Diego Rust - April 2020 Meetup](https://www.meetup.com/San-Diego-Rust/events/269639205/).
* [Apr  8. Zurich, CH - Rust Zurich - PyO3 and Rust - an experimental online meetup](https://www.meetup.com/Rust-Zurich/events/269771239/).

### North America

* [Apr  9. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybcgbmb/).
* [Apr 15. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybcgbtb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Infrastructure Engineer at Aleph Alpha, Heidelberg, Germany](https://aleph-alpha.de/sw_engineer.html?language=de).
* [Backend Engineer at Kraken Bitcoin Exchange, remote or in Oregon](https://www.glassdoor.com/job-listing/backend-engineer-rust-kraken-bitcoin-exchange-JV_KO0,21_KE22,45.htm?jl=2913415229).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Meta-Comment: I started this topic as someone completely uninvolved in the rust project. It's very reassuring seeing the nature of the response. Even knowing how fantastic the Rust community is, I was still prepared to be met with at least a small element of condescension given the nature of this issue. I haven't felt any sense of it. It's amazing. Anyone that has impact on the community culture deserves credit: This sort of experience doesn't come from nowhere. It comes from a long history of many people nudging things in the right direction.
> Thank you.

– [Ben on Zulip](https://rust-lang.zulipchat.com/#narrow/stream/122653-zulip/topic/new-user.20friction.20from.20stream.20naming.20conventions/near/191422121)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/842) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
