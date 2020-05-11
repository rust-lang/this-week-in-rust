Title: This Week in Rust 338
Number: 338
Date: 2020-05-12
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

* [Announcing Rust 1.43.1](https://blog.rust-lang.org/2020/05/07/Rust.1.43.1.html)
* [Rust concurrency: the single-writer principle.](https://medium.com/@polyglot_factotum/rust-concurrency-the-single-writer-principle-applied-aada2cdc6fb0?source=friends_link&sk=cafc8dcf8babf4ec95b1b62ccde7e54b)
* [This Month in Rust OSDev (April 2020)](https://rust-osdev.com/this-month/2020-04/)
* [A no_std Rust binary](https://fasterthanli.me/blog/2020/a-no-std-rust-binary/)
* [Notes on io_uring](https://boats.gitlab.io/blog/post/io-uring/)
* [no_std async/await - soon on stable](https://ferrous-systems.com/blog/stable-async-on-embedded/)
* ["try fn" without special-casing Result ](https://dev.to/cad97/try-fn-without-special-casing-result-4m5b)
* [time_it: A case study in Rust macros](https://notes.iveselov.info/programming/time_it-a-case-study-in-rust-macros)
* [Dynamic stylesheets and Yew](https://conradludgate.com/posts/yew-css/)
* [A practical introduction to async programming in Rust](http://jamesmcm.github.io/blog/2020/05/06/a-practical-introduction-to-async-programming-in-rust/#en)
* [Collecting many errors from an iterator of Results](https://tarquin-the-brave.github.io/blog/posts/collecting-all-the-errors/).
* [AssemblyLift: a Framework for running WebAssembly on AWS Lambda](https://dev.to/akkoro/assemblylift-a-framework-for-running-webassembly-on-aws-lambda-2m96)
* [Auto-currying Rust Functions](https://peppe.rs/posts/auto-currying_rust_functions/)
* [Converting bits to integers in Rust using generics](https://dev.to/citizen_stig/converting-bits-to-integers-in-rust-using-generics-2nfg)
* [Magnifying Glasses for Rust Assembly](https://www.justanotherdot.com/posts/magnifying-glasses-for-rust-assembly.html)
* [More Rust and Load Balancer Adventures](https://medium.com/@bparli/more-rust-and-load-balancer-adventures-fad07f4fb095) 
* [Porting North Korean Dictionaries with Rust](https://digitalnk.com/blog/2020/05/08/porting-north-korean-dictionaries-with-rust/)
* [Restart accel project, GPGPU Framework for Rust: 0.3.0 Release](https://users.rust-lang.org/t/restart-accel-project-gpgpu-framework-for-rust-0-3-0-release/42087)
* [Rust Compile Times Are Not Slow](https://wiki.alopex.li/RustCompileTimesAreNotSlow)
* [Rust Performance Is Getting Hurt On LLVM 10 With Noticeably Longer Build Times](https://www.phoronix.com/scan.php?page=news_item&px=Rust-Hurt-On-LLVM-10)
* [Rust verification tools](https://alastairreid.github.io/rust-verification-tools/)
* [Series Announcement - Zero to Production in Rust](https://www.lpalmieri.com/posts/2020-05-10-announcement-zero-to-production-in-rust/)
* [Super Simple Disk Benchmark written in rust](https://www.d34dl0ck.me/super-simple-disk-benchmark-written-in-rust.html#super-simple-disk-benchmark-written-in-rust)
* [TERMINAL UI FOR GIT WRITTEN IN RUST](https://blog.extrawurst.org/general/2020/05/08/gitui-release.html)
* [Using Rust to Power Python Importing With oxidized_importer](https://gregoryszorc.com/blog/2020/05/10/using-rust-to-power-python-importing-with-oxidized_importer/)
* [What I Learned Contributing to Rust-Analyzer](https://dev.to/bnjjj/what-i-learned-contributing-to-rust-analyzer-4c7e)
* [What’s the difference between a Rust char and a Go rune?](https://www.christianfscott.com/rust-chars-vs-go-runes/)
* [Writing A Wayland Compositor In Rust](https://wiki.alopex.li/WritingAWaylandCompositorInRust)
* [Yak shaving conditional compilation in Rust](https://bitshifter.github.io/2020/05/07/conditional-compilation-in-rust/)
* [YEW Tutorial: 03 Services and all](https://dev.to/davidedelpapa/yew-tutorial-03-services-and-all-28ef)
* [video] [Building a simple GraphQL API with Actix and Juniper](https://youtu.be/7v7ERnrC4fo)
* [video] [Named Field Init in C, C++20, Zig, Rust, & D](https://www.youtube.com/watch?v=c-NyXKbqmQc)

# Crate of the Week

This week's crate is [WinRT-rs](https://github.com/microsoft/winrt-rs), Microsoft™'s official WinRT API for Rust.

Thanks to [JLalu](https://users.rust-lang.org/t/crate-of-the-week/2704/767) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [s3rename: Atomic renames and asynchronous destructors](https://github.com/jamesmcm/s3rename/issues/16)
* [sedregex: Add support for translation commands y/ and tr/](https://gitlab.com/mexus/sedregex/-/issues/4)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

372 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-04-27..2020-05-04

* implement RFC [#2523](https://rust-lang.github.io/rfcs/2523-cfg-path-version.html), [`#[cfg(version(..))]`](https://github.com/rust-lang/rust/pull/71314)
* [have the per-query caches store the results on arenas](https://github.com/rust-lang/rust/pull/70674)
* [avoid duplicating code for each query](https://github.com/rust-lang/rust/pull/69808)
* [forbid `dyn Trait` in patterns](https://github.com/rust-lang/rust/pull/71038)
* [fix wrong argument in autoderef process](https://github.com/rust-lang/rust/pull/71627)
* [suggest `into` instead of `try_into` if possible with int types](https://github.com/rust-lang/rust/pull/71617)
* [tweak some suggestions in `rustc_resolve`](https://github.com/rust-lang/rust/pull/71438)
* [add message for resolution failure because wrong namespace](https://github.com/rust-lang/rust/pull/71419)
* [point at the return type on `.into()` failure caused by `?`](https://github.com/rust-lang/rust/pull/71409)
* [suggest `;` or assignment to drop borrows in tail exprs](https://github.com/rust-lang/rust/pull/71217)
* [on type mismatch involving associated type, suggest constraint](https://github.com/rust-lang/rust/pull/71108)
* [minimize parameter of `coerce_borrowed_pointer`](https://github.com/rust-lang/rust/pull/71524)
* [remove some `Vec` allocations to improve performance](https://github.com/rust-lang/rust/pull/71268)
* [allow `Unreachable` terminators unconditionally in const-checking](https://github.com/rust-lang/rust/pull/71691)
* [allow `Downcast` projections unconditionally in const-checking](https://github.com/rust-lang/rust/pull/71688)
* [added MIR constant propagation of Scalars into function call arguments](https://github.com/rust-lang/rust/pull/71697)
* [Miri: unleash all feature gates](https://github.com/rust-lang/rust/pull/71631)
* [use existing framework for backward dataflow analyses](https://github.com/rust-lang/rust/pull/71006)
* [add Read/Write::can_read/write_vectored](https://github.com/rust-lang/rust/pull/67841)
* [add `RefCell::take`](https://github.com/rust-lang/rust/pull/71398)
* [`slice::fill`: use `T` instead of generic arg](https://github.com/rust-lang/rust/pull/71165)
* [`Vec` `drop` and `truncate`: drop using raw slice `*mut [T]`](https://github.com/rust-lang/rust/pull/71148)
* [hashbrown: mark `RawTable::par_iter` `unsafe`](https://github.com/rust-lang/hashbrown/pull/157)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2585: FC for unsafe blocks in unsafe fn](https://github.com/rust-lang/rfcs/pull/2585)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.


### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are in final comment period this week*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for RFC 2432, "Allow `if` and `match` in constants"](https://github.com/rust-lang/rust/issues/49146)
* [disposition: merge] [Tracking issue for std::sync::Once poisoning](https://github.com/rust-lang/rust/issues/33577)

## New RFCs

* [Inline `const` expressions and patterns](https://github.com/rust-lang/rfcs/pull/2920)
* [Add the `experimental_keywords` ability](https://github.com/rust-lang/rfcs/pull/2919)
* [sigil-option-notation](https://github.com/rust-lang/rfcs/pull/2918)

# Upcoming Events

### Online

* [May 20. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybchbbc/).

### North America

* [May 6. Johannesburg, ZA - Johannesburg meetup](https://www.meetup.com/Johannesburg-Rust-Meetup)
* [May  6. Portland, OR, US - PDXRust - NES Emulation in Rust](https://www.meetup.com/PDXRust/events/269165311/).
* [May  6. Indianapolis, IN, US - Indy.rs - Rust Meetup](https://www.meetup.com/indyrs/events/dtqwprybchbjb/).
* [May 6. Atlanta, GA, US - Rust Atlanta Meetup](https://www.meetup.com/Rust-ATL/)
* [May 11. Seattle, WA, US - Seattle Rust Meetup](http://www.meetup.com/Seattle-Rust-Meetup/)
* [May 13. Denver, CO, US Rust Boulder/Denver Monthly Meeting](https://www.meetup.com/Rust-Boulder-Denver/)
* [May 13. Vancouver, BC, CA - Vancouver Rust Meetup](https://www.meetup.com/Vancouver-Rust/events/)
* [May 14. Berlin, DE - Berlin Rust Hack and Learn](https://berline.rs/)
* [May 14. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybchbsb/).
* [May 14. Lehi, UT, US - Utah Rust Monthly Meetup](https://www.meetup.com/utahrust)
* [May 14. San Diego, CA, US - San Diego Rust](http://meetu.ps/c/2vF0G/4DXV4/a)
* [May 19. Paris, FR - Rust Paris](https://www.meetup.com/Rust-Paris)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I love Rust like I love Dark Souls.  
> It's difficult, but fair. I can not praise enough the software developers that realize proper errors are vastly superior to extensive docs.

– [seph-reed on Hacker News](https://news.ycombinator.com/item?id=23032636)

Thanks to [Armando Pérez Marqués](https://users.rust-lang.org/t/twir-quote-of-the-week/328/864) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [srikwit](https://github.com/srikwit), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust]().</small>
