Title: This Week in Rust 391
Number: 391
Date: 2021-05-19
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

### Newsletters

### Project/Tooling Updates

* [This Week In TensorBase 3](https://tensorbase.io/thisweek/2021-05-19-tw_3/)

### Observations/Thoughts
* [Authorization mechanisms in Rust web applications](https://ddtkey.com/blog/authz-mechanisms-in-Rust/)

### Rust Walkthroughs
* [How to make a cryptocurrency Telegram bot with Rust and Teloxide](https://dev.to/steadylearner/how-to-make-a-telegram-bot-with-rust-teloxide-m60)
* [Authorization mechanisms in Rust web applications](https://dev.to/ddtkey/authorization-mechanisms-in-rust-web-applications-27ag)
* [ZH] [series] [Build GraphQL services based on Async Rust using actix-web + async-graphql + rbatis + postgresql / mysql (基于 actix-web + async-graphql + rbatis + postgresql / mysql 构建异步 Rust GraphQL 服务) - Part 4](https://blog.budshome.com/budshome/ji-yu-actix-web-+-async-graphql-+-rbatis-+-postgresql---mysql-gou-jian-yi-bu-rust-graphql-fu-wu-(4)---bian-geng-fu-wu-,yi-ji-xiao-zhong-gou)
* [ZH] [Practice of rendering markdown to HTML in Rust (Rust web 开发中，将 markdown 渲染为 HTML 的实践)](https://blog.budshome.com/budshome/rust-zhong-jiang-markdown-xuan-ran-wei-html)
* [DE] [Speicherverwaltung in Rust](https://jo-so.de/2021-01/Speicher-Rust.html)

### Miscellaneous

* [James Munns on the state and the future of embedded & safety-critical Rust | Emergence Podcast](https://www.youtube.com/watch?v=SNUklwUi_M4)
* [Behind the scenes of 1Password for Linux [using Rust]](https://dteare.medium.com/behind-the-scenes-of-1password-for-linux-d59b19143a23)

# Crate of the Week

This week's crate is [arraygen](https://docs.rs/arraygen), a derive proc macro to generate arrays from structs.

Thanks to [José Manuel Barroso Galindo](https://users.rust-lang.org/t/crate-of-the-week/2704/911) for the nomination

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

333 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-05-10..2021-05-17

* [add auto traits and clone trait migrations for](https://github.com/rust-lang/rust/pull/84730) (RFC [#2229](https://rust-lang.github.io/rfcs/2229-capture-disjoint-fields.html))
* [insignificant destructors for RFC #2229](https://github.com/rust-lang/rust/pull/84152)
* [add `asm!()` support for PowerPC](https://github.com/rust-lang/rust/pull/84732)
* [add `asm!()` support for PowerPC64](https://github.com/rust-lang/rust/pull/85279)
* [add support for const operands and options to `global_asm!`](https://github.com/rust-lang/rust/pull/84107)
* [recover from invalid struct item syntax](https://github.com/rust-lang/rust/pull/84793)
* [fix diagnostic for cross crate private tuple struct constructors](https://github.com/rust-lang/rust/pull/85068)
* [fix suggestions for missing return type lifetime specifiers](https://github.com/rust-lang/rust/pull/85050)
* [suggest adding a type parameter for impls](https://github.com/rust-lang/rust/pull/85041)
* [fix stack overflow when checking for structural recursion](https://github.com/rust-lang/rust/pull/85012)
* [implement span quoting for proc-macros](https://github.com/rust-lang/rust/pull/84278)
* [handle more span edge cases in generics diagnostics](https://github.com/rust-lang/rust/pull/83759)
* [improve diagnostics for GATs](https://github.com/rust-lang/rust/pull/82272)
* [show macro name in 'this error originates in macro' message](https://github.com/rust-lang/rust/pull/82069)
* [store `VariantIdx` to distinguish enum variants](https://github.com/rust-lang/rust/pull/85195)
* [remove `CrateNum` parameter for queries that only work on local crate](https://github.com/rust-lang/rust/pull/85178)
* [adjust target search algorithm for rustlib path](https://github.com/rust-lang/rust/pull/85152)
* [don't suggest adding `'static` lifetime to arguments](https://github.com/rust-lang/rust/pull/85240)
* [improve error message for non-exhaustive matches on non-exhaustive enums](https://github.com/rust-lang/rust/pull/85233)
* [allow `async {}` expressions in const contexts](https://github.com/rust-lang/rust/pull/85353)
* [warn about unused pub fields in non-pub structs](https://github.com/rust-lang/rust/pull/85324)
* [fix unused attributes on `macro_rules`](https://github.com/rust-lang/rust/pull/85312)
* [box `Impl.blanket_impl` to reduce size](https://github.com/rust-lang/rust/pull/85311)
* [`#[inline(always)]` on basic pointer methods](https://github.com/rust-lang/rust/pull/85218)
* [make `unchecked_`{`add`, `sub`, `mul`} inherent methods unstably const](https://github.com/rust-lang/rust/pull/85096)
* [BTree: no longer copy keys and values before dropping them](https://github.com/rust-lang/rust/pull/84904)
* [`str::is_char_boundary` - slight optimization](https://github.com/rust-lang/rust/pull/84751)
* [futures-macro: improve diagnostics on type mismatch](https://github.com/rust-lang/futures-rs/pull/2433)
* [futures: implement `try_chunks`](https://github.com/rust-lang/futures-rs/pull/2438)
* [futures: change `SelectAll` iterators to return stream instead of `StreamFuture`](https://github.com/rust-lang/futures-rs/pull/2431)
* [futures: expose iterators from `SelectAll`](https://github.com/rust-lang/futures-rs/pull/2428)
* [futures: `SelectAll::clear`](https://github.com/rust-lang/futures-rs/pull/2430)
* [futures: `FuturesUnordered::clear`](https://github.com/rust-lang/futures-rs/pull/2415)
* [futures: change `StreamExt::scan` to pass state to closure by value](https://github.com/rust-lang/futures-rs/pull/2427)
* [futures: abortable streams](https://github.com/rust-lang/futures-rs/pull/2410)
* [cargo: improve performance of git status check in `cargo package`](https://github.com/rust-lang/cargo/pull/9478)
* [rustdoc: minimize amount of fake DefIds used in rustdoc](https://github.com/rust-lang/rust/pull/85067)
* [clippy: add `needless_bitwise_bool` lint](https://github.com/rust-lang/rust-clippy/pull/7133)
* [clippy: new lint: `unused_async`](https://github.com/rust-lang/rust-clippy/pull/7225)
* [clippy: move `inconsistent_struct_constructor` to `pedantic`](https://github.com/rust-lang/rust-clippy/pull/7193)
* [clippy: `needless_collect` enhancements](https://github.com/rust-lang/rust-clippy/pull/7188)
* [clippy: `while_let_on_iterator` improvements](https://github.com/rust-lang/rust-clippy/pull/6966)
* [clippy: add `Sized` trait for `wrong_self_convention` lint test](https://github.com/rust-lang/rust-clippy/pull/7222)
* [clippy: `match_single_binding`: fix invalid suggestion when match scrutinee has side effects](https://github.com/rust-lang/rust-clippy/pull/7095)
* [clippy: trigger `wrong_self_convention` only if it has implicit self](https://github.com/rust-lang/rust-clippy/pull/7215)
* [clippy: stop linting else if let pattern in `option_if_let_else` lint](https://github.com/rust-lang/rust-clippy/pull/7216)
* [clippy: fix false positives about generic args](https://github.com/rust-lang/rust-clippy/pull/7223)
* [clippy: fix a `manual_unwrap_or` false positive with deref coercion](https://github.com/rust-lang/rust-clippy/pull/7233)

## Rust Compiler Performance Triage

A lot of noise in the benchmark results this week. We are discussing ([zulip archive](https://zulip-archive.rust-lang.org/247081tcompilerperformance/06104coercionsdebugnoise.html), [live zulip](https://rust-lang.zulipchat.com/#narrow/stream/247081-t-compiler.2Fperformance/topic/coercions-debug.20noise)) how best to update the benchmark set to eliminate the noisy cases that are bouncing around. Beyond that, some large improvements to a few individual benchmarks.

The memory usage ([max-rss](https://perf.rust-lang.org/?start=2021-05-11&end=2021-05-18&absolute=true&stat=max-rss)) seemed largely flat. Except for an upward trend on `tuple-stess` that indicates 4% more memory from a week ago.

Triage done by **@pnkfelix**.
Revision range: [382f..25a2](https://perf.rust-lang.org/?start=382f748f23979e37e3e012b090e5a0313463f182&end=25a277f03df7e44643ddfcc240d034409cb2f505&absolute=false&stat=instructions%3Au)

5 Regressions, 7 Improvements, 2 Mixed
1 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-05-18.md).

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Reserved prefixes in the 2021 edition](https://github.com/rust-lang/rfcs/pull/3101)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [A new prelude for the 2021 edition (trait-only edition)](https://github.com/rust-lang/rfcs/pull/3114)
* [disposition: postpone] [Allow Overloading || and &&](https://github.com/rust-lang/rfcs/pull/2722)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [stabilize const_fn_unsize](https://github.com/rust-lang/rust/pull/85078)
* [disposition: merge] [FCP poll for ErrorKind::OutOfMemory](https://github.com/rust-lang/rust/issues/84916)
* [disposition: merge] [impl FromStr for proc_macro::Literal](https://github.com/rust-lang/rust/pull/84717)
* [disposition: merge] [stabilize member constraints](https://github.com/rust-lang/rust/pull/84701)
* [disposition: merge] [Uplift the invalid_atomic_ordering lint from clippy to rustc](https://github.com/rust-lang/rust/pull/84039)
* [disposition: merge] [Stabilize "RangeFrom" patterns](https://github.com/rust-lang/rust/pull/83918)
* [disposition: merge] [Stabilize extended_key_value_attributes](https://github.com/rust-lang/rust/pull/83366)
* [disposition: merge] [Add functions `Duration::try_from_secs_{f32, f64}`](https://github.com/rust-lang/rust/pull/82179)

## New RFCs

* [Scrape code examples from examples/ directory for Rustdoc](https://github.com/rust-lang/rfcs/pull/3123)

# Upcoming Events

### Online
* [May 12, Online - Rust Meetup May 2021 - Rust Malaysia](https://docs.google.com/forms/d/e/1FAIpQLSf_hz-ZDwYEhVmIH0uzJ0uH41aXWZ_zRDsI0XENpfkKHvh_Jg/viewform)
* [May 13, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/2021/05/13/rust-hack-and-learn.html)
* [May 15 - June 7, Online - Solana Season Hackathon - Registration open now](https://twitter.com/solana/status/1387411221717176323?s=20)
* [May 17, 2021, Cardiff, UK - Rust and Cpp Cardiff :: v2.0 - Rust and C++ Cardiff](https://secure.meetup.com/register/?referrer_n=event&referrer_i=278002832&ctx=ref)
* [May 18, 2021, Washington, DC, US - Mid-month Rustful: rust4ml - Rust DC](https://www.meetup.com/RustDC/events/ntvrgsycchbxb)
* [May 19, 2021, Vancouver, BC - Rust Study/Hack/Hang-out night - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zppkjsycchbzb/)
* [May 20, 2021, Online - Go vs Rust | Round table discussion](https://rustlab.it/en/rust-vs-go/)
* [May 20, 2021, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrycchbhc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Protocol Labs**

* [Research Engineer, CryptoCompute Lab (Remote)](https://www.arbeitnow.com/view/research-engineer-cryptocompute-lab-protocol-labs-444987)

**Amazon Web Services**

* [Applied Scientist (Boston/Cambridge Area, MA, US)](https://www.amazon.jobs/en/jobs/1555897/applied-scientist)

**Paige**

* [Senior Software Engineer, Visualization (Remote, Europe)](https://boards.greenhouse.io/paige/jobs/5210311002)

**ANIXE**

* [Rust Software Engineer (Wrocław, PL)](https://anixe.bamboohr.com/jobs/view.php?id=72)

**NZXT**

* [Senior Software Engineer for CAM (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=259)
* [Senior Software Engineer for Streaming Software (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=317)

**Zondax**

* [Embedded Systems Engineer (C/C++ & Rust)(Remote)](https://zondax.ch/news/embedded-systems-engineer)
* [Software Engineer (Golang / Rust) (Remote)](https://zondax.ch/news/engineers-golang-rust)


**Ockam**

* [Several Rust related positions available](https://www.ockam.io/team#open-roles)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I often think about Rust as a process and community for developing a programming language, rather than as a programming language itself.

– [throwaway894345 on hacker news](https://news.ycombinator.com/item?id=27120691)

Thanks to [Krishna Sundarram](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1050) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
