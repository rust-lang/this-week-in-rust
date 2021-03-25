Title: This Week in Rust 383
Number: 383
Date: 2021-03-24
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

### Official
* [Building a shared vision for Async Rust](https://blog.rust-lang.org/2021/03/18/async-vision-doc.html)
* [Inside] [1.51.0 pre-release testing](https://blog.rust-lang.org/inside-rust/2021/03/23/1.51.0-prerelease.html)
* [Foundation] [Introducing Bobby Holley](https://foundation.rust-lang.org/posts/2021-03-18-introducing-bobby-holley/)
* [Foundation] [Introducing Tyler Mandry](https://foundation.rust-lang.org/posts/2021-03-18-introducing-tyler-mandry/)

### Newsletters

### Project/Tooling Updates
* [rust-analyzer Changelog #69](https://rust-analyzer.github.io/thisweek/2021/03/22/changelog-69.html)

### Observations/Thoughts
* [The biggest threat to Rust's sustainability](https://kerkour.com/blog/the-biggest-threat-to-rust-sustainability/)
* [Safe Systems Programming in Rust](https://cacm.acm.org/magazines/2021/4/251364-safe-systems-programming-in-rust/fulltext)
* [The GPIO war: macro bunkers for typestate explosions](https://www.ecorax.net/macro-bunker-1/)
* [A look back at asynchronous Rust](https://tomaka.medium.com/a-look-back-at-asynchronous-rust-d54d63934a1c)
* [Rust as an App Language](https://www.ikiapps.com/programming/2019/07/26/is-rust-good-for-apps)
* [How often does Rust change?](https://steveklabnik.com/writing/how-often-does-rust-change)
* [Serving a single file over HTTP with Rust and Go](https://vadosware.io/post/serving-a-single-file-over-http-with-rust-and-go/)
* [The Case for the Typestate Pattern - The Typestate Pattern itself](https://www.novatec-gmbh.de/en/blog/the-case-for-the-typestate-pattern-the-typestate-pattern-itself/)
* [ZH] [Rust in Embedded World](https://zhuanlan.zhihu.com/p/352945545)
* [video] [Cheaply writing a fast interpreter - Neil Mitchell](https://youtu.be/V8dnIw3amLA)

### Rust Walkthroughs
* [How to execute shellcodes from memory in Rust](https://kerkour.com/blog/rust-execute-from-memory/)
* [Beginner's Guide to Rust Pattern Matching](https://doma-dev.medium.com/pattern-matching-in-rust-and-other-imperative-languages-7cf1c6abf4a1)
* [Writing a PostgreSQL Pretty Printer in Rust: Part 1.5](https://blog.urth.org/2021/03/21/writing-a-postgres-sql-pretty-printer-in-rust-part-1-5/)
* [Calling .NET APIs from Rust](https://ericsink.com/entries/lousygrep.html)
* [The magical applications of Zero-Sized Types in Rust](https://www.hardmo.de/article/2021-03-14-zst-proof-types.md)
* [Rust and Rocket - Zero to Deploy](https://dev.to/alexmercedcoder/rust-rocket-zero-to-deploy-578o)
* [Learning to Fly: Let's simulate evolution in Rust! (pt 3)](https://pwy.io/en/posts/learning-to-fly-pt3/)
* [Building an OpenStreetMap app in Rust, Part VI](https://blogg.bekk.no/building-an-openstreetmap-app-in-rust-part-vi-8cfc8c3f7798)
* [Tutorial: Writing a tiny Entity Component System in Rust](https://ianjk.com/ecs-in-rust/)
* [Implementing RAII guards in Rust](https://aloso.github.io/2021/03/18/raii-guards.html)
* [JP] [[TECH] Actix web で HttpOnly な Cookie を設定する 🍪](https://dev.to/nikaera/tech-actix-web-httponly-cookie-182)
* [video] [The four-part "Overview of the Rust Programming Language" for beginners is now complete](https://www.youtube.com/watch?v=gesNaLkUJeA&list=PLP2yfE2-FXdQmXLvrQ5QN64enbF_KCYQW)

### Papers and Research Projects

### Miscellaneous
* [Streaming using Bastion & Kafka - near consumer native correlation](https://vertexclique.com/streaming-with-bastion-and-kafka/)
* [Performance comparison: counting words in Python, Go, C++, C, AWK, Forth, and Rust](https://benhoyt.com/writings/count-words/)
* [Linus Torvalds on where Rust will fit into Linux](https://www.zdnet.com/article/linus-torvalds-on-where-rust-will-fit-into-linux/)

# Crate of the Week

This week's crate is [egg](https://egraphs-good.github.io), a project using e-graphs to provide a new way to build program optimizers and synthesizers.

Thanks to [Daniel Nugent](https://users.rust-lang.org/t/crate-of-the-week/2704/891) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [dotenv-linter has many good first issues](https://github.com/dotenv-linter/dotenv-linter/issues/390)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

389 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-03-15..2021-03-22

* [enable mutable noalias for LLVM >= 12](https://github.com/rust-lang/rust/pull/82834) (Fingers crossed)
* [allow registering tool lints with `register_tool`](https://github.com/rust-lang/rust/pull/83216)
* [more precise spans for HIR paths](https://github.com/rust-lang/rust/pull/83092)
* [`const_evaluatable_checked`: stop eagerly erroring in `is_const_evaluatable`](https://github.com/rust-lang/rust/pull/82707)
* [miri: improve error message of calling unsupported non-"C"/"system"-ABI foreign function](https://github.com/rust-lang/miri/pull/1745)
* [make source-based code coverage compatible with MIR inlining](https://github.com/rust-lang/rust/pull/83080)
* [stabilize `or_patterns` (RFC 2535, 2530, 2175)](https://github.com/rust-lang/rust/pull/79278)
* [stabilize `feature(osstring_ascii)`](https://github.com/rust-lang/rust/pull/80193)
* [stabilize `slice::IterMut::as_slice`](https://github.com/rust-lang/rust/pull/82771)
* [stabilize `assoc_char_funcs` and `assoc_char_consts`](https://github.com/rust-lang/rust/pull/82919)
* [implement `String::remove_matches`](https://github.com/rust-lang/rust/pull/71780)
* [add a check for ASCII characters in `to_upper` and `to_lower`](https://github.com/rust-lang/rust/pull/81358)
* [fix invalid slice access in `String::retain`](https://github.com/rust-lang/rust/pull/82554)
* [constify copy related functions](https://github.com/rust-lang/rust/pull/83091)
* [add `as_str` method for split whitespace str iterators](https://github.com/rust-lang/rust/pull/82570)
* [`Vec::dedup_by` optimization](https://github.com/rust-lang/rust/pull/82191)
* [fix overflowing length in `Vec<ZST>` to `VecDeque`](https://github.com/rust-lang/rust/pull/83244)
* [implement `TrustedLen` and `TrustedRandomAccess` for `Range<integer>`, `array::IntoIter`, `VecDequeue`'s iterators](https://github.com/rust-lang/rust/pull/81607)
* [cargo: allow cargo update to operate with the `--offline` flag](https://github.com/rust-lang/cargo/pull/9279)
* [cargo: refactor feature handling, and improve error messages](https://github.com/rust-lang/cargo/pull/9290)
* [rustdoc: reduce GC work during search](https://github.com/rust-lang/rust/pull/83077)
* [rustfmt: fix issue 'double spaces between struct field prefix and identity when using long attributes](https://github.com/rust-lang/rustfmt/pull/4747)

## Rust Compiler Performance Triage

Added two benchmarks over the past week to the perf suite - diesel and stm32f4,
which are intended to add to the level of tracking for rustdoc and, for both, a
focus on compiler trait machinery.

Performance results for this week are mixed, but overall largely positive.

Triage done by **@simulacrum**.
Revision range: [861872b..f24ce9b0](https://perf.rust-lang.org/?start=861872bc453bde79b83ff99d443d035225f10e87&end=f24ce9b0140d9be5a336954e878d0c1522966bb8&absolute=false&stat=instructions%3Au)

3 Regressions, 3 Improvements, 4 Mixed

0 of them in rollups

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Adds must_not_suspend_lint RFC](https://github.com/rust-lang/rfcs/pull/3014)
* [adds async stream rfc](https://github.com/rust-lang/rfcs/pull/2996)
* [Make the authors field optional](https://github.com/rust-lang/rfcs/pull/3052)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: Declarative macro metavariable expressions](https://github.com/rust-lang/rfcs/pull/3086)
* [RFC: Hidden trait implementations](https://github.com/rust-lang/rfcs/pull/2529)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize bufreader_seek_relative](https://github.com/rust-lang/rust/pull/82992)
* [disposition: merge] [Add IEEE 754 compliant fmt/parse of -0, infinity, NaN](https://github.com/rust-lang/rust/pull/78618)
* [disposition: merge] [Implement indexing slices with pairs of `core::ops::Bound<usize>`](https://github.com/rust-lang/rust/pull/77704)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [March 25, Barcelona, ES - BcnRust Meetup](https://www.meetup.com/es-ES/BcnRust/events/276796209/).
* [March 30, Munich, DE - Rust Remote #6 - Rust Munich](https://www.meetup.com/rust-munich/events/276424952)
* [March 30, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccfbnc/)
* [April 1, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprryccgbcb/)
* [April 6, Buffalo, NY, US - Buffalo Rust User Group - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/276717867/)

### Asia Pacific

* [March 31, Wellington, NZ - Coffee - Rust Wellington](https://www.meetup.com/Rust-Wellington/events/277104604/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**MongoDB**

* [Senior Rust Engineer, Database Experience (Remote, North America)](https://www.mongodb.com/careers/jobs/3005375)

**IONQ**

* [Senior Software Engineer, Quantum Operating Systems (Remote)](https://ionq.bamboohr.com/jobs/view.php?id=44)

**Pondurance**

* [Software Data Engineer - National (Remote)](https://pondurance-llc.prismhr-hire.com/job/216824/software-data-engineer-national)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> This is just to say,  
> I have rebased  
> the feature branch  
> opened against  
> master
> 
> and which  
> you might have been  
> already working  
> on fixing
> 
> Forgive me,  
> the diff was so trivial  
> so minor  
> so smol

– [Jubilee on rust-lang zulip](https://rust-lang.zulipchat.com/#narrow/stream/257879-project-portable-simd/topic/2021-03-08.20Meeting/near/231384678)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1020) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
