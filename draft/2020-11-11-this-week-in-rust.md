Title: This Week in Rust 364
Number: 364
Date: 2020-11-11
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
* [Inside] [Exploring PGO for the Rust compiler](https://blog.rust-lang.org/inside-rust/2020/11/11/exploring-pgo-for-the-rust-compiler)

### Newsletters
* [This Month in Rust OSDev (October 2020)](https://rust-osdev.com/this-month/2020-10/)
* [This Month in Rust GameDev #15 - October 2020](https://rust-gamedev.github.io/posts/newsletter-015/)

### Tooling
* [Rust-Analyzer Changelog #50](https://rust-analyzer.github.io/thisweek/2020/11/09/changelog-50.html)
* [Develop & Debug Your Rust Application Top Of Kubernetes With OKteto](https://okteto.com/blog/getting-started-with-okteto-and-rust/)

### Observations/Thoughts
* [Rust Ray Tracer, an Update (and SIMD)](https://siliconsprawl.com/2020/11/06/simd-ray-tracer.html)
* [Rust emit=asm Can Be Misleading](https://siliconsprawl.com/2020/11/09/rust-emit-asm.html)
* [A survey into static analyzers configurations: Clippy for Rust, part 1](https://medium.com/monocodus/a-survey-into-static-analyzers-configurations-clippy-for-rust-part-1-5de50fd9326)
* [Why Rust is the Future of Game Development](https://thefuntastic.com/blog/why-rust-is-the-future-game-dev?)
* [Rust as a productive high-level language](https://omarabid.com/rust-high-level-language)
* [40 millisecond bug](https://vorner.github.io/2020/11/06/40-ms-bug.html)
* [Postfix macros in Rust](https://gist.github.com/est31/8d0465997ea920c5ba917cbbf80a822d)
* [A Quick Tour of Trade-offs Embedding Data in Rust](https://nickb.dev/blog/a-quick-tour-of-trade-offs-embedding-data-in-rust)
* [Why Developers Love Rust](https://ibraheem.ca/posts/why-devs-love-rust)

### Rust Walkthroughs
* [Make a Language - Part Nine: Function Calls](https://arzg.github.io/lang/9/)
* [Building a Weather Station UI](https://blog.kdubovikov.ml/articles/rust/ui/weather-station-ui)
* [Getting Graphical Output from our Custom RISC-V Operating System in Rust](https://blog.stephenmarz.com/2020/11/11/risc-v-os-using-rust-graphics/)
* [Build your own: GPG](https://andrewhalle.github.io/build-your-own/gpg)
* [Rpi 4 meets Flutter and Rust](https://dev.to/charliefoxtrot/rpi-4-meets-flutter-and-rust-23ma)
* [AWS Lambda + Rust](https://dev.to/rad_val_/aws-lambda-rust-292g)
* [Orchestration in Rust](https://dev.to/elasticrash/orchestration-in-rust-549b)
* [Rocket Tutorial 02: Minimalist API](https://dev.to/davidedelpapa/rocket-tutorial-02-minimalist-api-2kl6)
* [Get simple IO stats using Rust (throughput, ...)](https://dev.to/martichou/get-simple-io-stats-using-rust-throughput-47m4)
* [Type-Safe Discrete Simulation in Rust](https://dev.to/elshize/type-safe-discrete-simulation-in-rust-3n7d)
* [series] [A Gemini Client in Rust](https://dev.to/krowemoh/series/9524)
* [Postfix macros in Rust](https://gist.github.com/est31/8d0465997ea920c5ba917cbbf80a822d)
* [Processing a Series of Items with Iterators in Rust](https://blog.knoldus.com/processing-a-series-of-items-with-iterators-in-rust/)
* [Compilation of Active Directory Logs Using Rust](https://blog.knoldus.com/compilation-of-active-directory-logs-using-rust/)
* [FR] [The Rust Programming Language (translated in French)](https://jimskapt.github.io/rust-book-fr/)

### Project Updates
* [New doc comment handling in rustdoc](https://blog.guillaume-gomez.fr/articles/2020-11-11+New+doc+comment+handling+in+rustdoc)

### Miscellaneous
* [Rust vs Go](https://bitfieldconsulting.com/golang/rust-vs-go)
* [Learn Assembly by Writing Entirely Too Many Brainf*ck Compilers in Rust](https://github.com/pretzelhammer/rust-blog/blob/master/posts/too-many-brainfuck-compilers.md)

# Crate of the Week

This week's crate is [postfix-macros](https://github.com/est31/postfix-macros), a clever hack to allow postfix macros in stable Rust.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/841) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [time-rs: The call to `localtime_r` may be unsound](https://github.com/time-rs/time/issues/293)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

333 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-11-02..2020-11-09

* [Implement destructuring assignment for tuples](https://github.com/rust-lang/rust/pull/78748)
* [reverse binding order in matches to allow the subbinding of copyable fields in bindings after `@`](https://github.com/rust-lang/rust/pull/78638)
* [Fix unreachable sub-branch detection in or-patterns](https://github.com/rust-lang/rust/pull/78167)
* [Transform post order walk to an iterative approach](https://github.com/rust-lang/rust/pull/78607)
* [Compile rustc crates with the initial-exec TLS model](https://github.com/rust-lang/rust/pull/78201)
* [Make some `std::io` functions `const`](https://github.com/rust-lang/rust/pull/78811)
* [Stabilize `Poll::is_ready` and `is_pending` as const](https://github.com/rust-lang/rust/pull/76227)
* [Stabilize `hint::spin_loop`](https://github.com/rust-lang/rust/pull/76097)
* [Simplify the implementation of `Cell::get_mut`](https://github.com/rust-lang/rust/pull/78735)
* [futures: Add `StreamExt::cycle`](https://github.com/rust-lang/futures-rs/pull/2252)
* [futures: Add `TryStreamExt::try_buffered`](https://github.com/rust-lang/futures-rs/pull/2245)
* [cargo: Avoid some extra downloads with new feature resolver](https://github.com/rust-lang/cargo/pull/8823)

## Rust Compiler Performance Triage

* [2020-11-10](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-11-10.md):
1 Regression, 2 Improvements, 2 mixed

A mixed week with improvements still outweighing regressions. Perhaps the biggest highlight was the move to compiling rustc crates [with the initial-exec TLS model](https://github.com/rust-lang/rust/pull/78201) which results in fewer calls to `_tls_get_addr` and thus faster compile times.

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-11-10.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)
* [disposition: merge] [Stabilize clamp](https://github.com/rust-lang/rust/pull/77872)
* [disposition: merge] [[android] Add support for android's file descriptor ownership tagging to libstd.](https://github.com/rust-lang/rust/pull/74860)
* [disposition: merge] [Implement Error for &(impl Error)](https://github.com/rust-lang/rust/pull/75180)
* [disposition: merge] [Add checking for no_mangle to unsafe_code lint](https://github.com/rust-lang/rust/pull/72209)
* [disposition: merge] [Tracking issue for methods converting `bool` to `Option<T>`](https://github.com/rust-lang/rust/issues/64260)

## New RFCs
* [User/ardavis/checked cfg](https://github.com/rust-lang/rfcs/pull/3013)
* [add const-ub RFC](https://github.com/rust-lang/rfcs/pull/3016)
* [Adds `must_not_await_lint` RFC](https://github.com/rust-lang/rfcs/pull/3014)

# Upcoming Events

### Online
* [November 12, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprrybcpbqb/)
* [November 12, Washington, DC, US - Mid-month Rustful—How oso built a runtime reflection system for Rust - Rust DC](https://www.meetup.com/RustDC/events/273813659)
* [November 12, Lehi, UT, US - WASM, Rust, and the State of Async/Await - Utah Rust](https://www.meetup.com/utah-rust/events/273757338/)
* [November 18, Vancouver, BC, CA - Rust Study/Hack/Hang-out night - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/npqfbsybcpbxb/)
* [November 24, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrybcpbgc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [2 Backend engineers, Core Banking at Truelayer (Milan, IT)](https://apply.workable.com/truelayer/j/BD023B950B) [also contact](https://twitter.com/algo_luca/status/1324763252560191490)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*
# Quote of the Week

> There are no bad programmers, only insufficiently advanced compilers

– [Esteban Kuber on twitter](https://twitter.com/ekuber/status/1319476290395664384)

Thanks to [Nixon Enraght-Moony](https://users.rust-lang.org/t/twir-quote-of-the-week/328/957) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/joxy7n/this_week_in_rust_363/)</small>
