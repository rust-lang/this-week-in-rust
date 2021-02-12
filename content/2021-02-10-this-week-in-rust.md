Title: This Week in Rust 377
Number: 377
Date: 2021-02-10
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

* [Foundation] [Hello World!](https://foundation.rust-lang.org/posts/2021-02-08-hello-world/)
* [Inside] [1.50.0 pre-release testing](https://blog.rust-lang.org/inside-rust/2021/02/09/1.50.0-prerelease.html)

### Newsletters
* [This Month in Rust GameDev #18 - January 2021](https://rust-gamedev.github.io/posts/newsletter-018/)
* [This Month in Rust OSDev (January 2021)](https://rust-osdev.com/this-month/2021-01/)

### Project/Tooling Updates
* [rust-analyzer Changelog #63](https://rust-analyzer.github.io/thisweek/2021/02/08/changelog-63.html)
* [Launching wasm.rs: a collection of crates, a community](https://yrashk.medium.com/launching-wasm-rs-a-collection-of-crates-a-community-4344d2ba75b3)
* [A Memory Safe TLS Module for the Apache HTTP Server](https://www.abetterinternet.org/post/memory-safe-tls-apache/)

### Observations/Thoughts
* [Benchmarking Tokio Tasks and Goroutines](https://www.reddit.com/r/rust/comments/lg0a7b/benchmarking_tokio_tasks_and_goroutines/)
* [A Better Rust Profiler](https://matklad.github.io/2021/02/10/a-better-profiler.html)
* [An unsafe tour of Rust's Send and Sync](https://nyanpasu64.github.io/blog/an-unsafe-tour-of-rust-s-send-and-sync/)
* [Improving texture atlas allocation in WebRender](https://nical.github.io/posts/etagere.html)

### Rust Walkthroughs
* [Async Rust: Futures, Tasks, Wakers; Oh My!](https://msarmi9.github.io/posts/async-rust/)
* [Rust for Haskell Programmers!](https://mmhaskell.com/rust)
* [Rust CLI Game of Life tutorial - Part 1](https://dev.to/jbarszczewski/rust-cli-game-of-life-tutorial-part-1-57pp)
* [Where everything went wrong...](https://msirringhaus.github.io/Where-everything-went-wrong/)
* [Rust for Clojurists](https://gist.github.com/oakes/4af1023b6c5162c6f8f0)
* [ES] [El formato RON: Rusty Object Notation](https://blog.adrianistan.eu/formato-ron-rusty-object-notation)
* [video] [1Password Developer Fireside Chat: Introduction to Rust Macros](https://youtu.be/Lh262L63asA)
* [video] [Dynamic vs Static Dispatch in Rust](https://youtu.be/tM2r9HD4ivQ)

### Miscellaneous
* [Congratulations, Rustaceans, on the creation of the Rust Foundation!](https://aws.amazon.com/blogs/opensource/congratulations-rustaceans-on-the-creation-of-the-rust-foundation/)
* [Microsoft joins Rust Foundation](https://cloudblogs.microsoft.com/opensource/2021/02/08/microsoft-joins-rust-foundation/)
* [Google joins the Rust Foundation](https://opensource.googleblog.com/2021/02/google-joins-rust-foundation.html)
* [Mozilla Welcomes the Rust Foundation](https://blog.mozilla.org/blog/2021/02/08/mozilla-welcomes-the-rust-foundation/)
* [Trusted Programming - Our Rust Mission at Huawei](https://trusted-programming.github.io/2021-02-07/index.html)
* [YSK: VSCode's most recent update fixed a quirk in Rust workflows](https://www.reddit.com/r/rust/comments/lgccv5/ysk_vscodes_most_recent_update_fixed_a_quirk_in/)
* [curl supports rustls](https://daniel.haxx.se/blog/2021/02/09/curl-supports-rustls/)
* [Architecting Artichoke Ruby: A Modular Ruby implementation written in Rust](https://github.com/artichoke/artichoke/blob/21045d8f1086c669dd428a3b5bdcc4a58e13acec/ARCHITECTURE.md)
* [video] [Interview with Ashley Williams, Rust Foundation Interim Executive Director (Part 1)](https://youtu.be/h-LoPr5553o)

# Crate of the Week

This week's crate is [threadIO](https://crates.io/crates/thread_io), a crate that makes disk IO in a background thread easy and elegant.

Thanks to [David Andersen](https://users.rust-lang.org/t/crate-of-the-week/2704/881) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

Fuchsia has several issues available:

* [[netstack3] ARP: Add tests for ARP on a broadcast medium](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=34979)
* [[netstack3] Migrate all transport protocols to trait associated types](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=48364)
* [[netstack3] Split IpProto into Ipv4Proto and Ipv6NextHeader](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=47454)
* [[net-types] Add common prefix length calculation for IP addresses](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=47008)
* [[netstack3] Make sure ICMP messages are not sent in response to non-initial fragment packets](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=21432)
* [[internet-checksum] Clarify documentation around odd byte lengths](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=69355)
* [[netstack3] IP fragment reassembly vulnerable to FragmentSmack](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=50830)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

384 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-02-01..2021-02-08

* [add AArch64 big-endian and ILP32 targets](https://github.com/rust-lang/rust/pull/81455)
* [improve handling of spans around macro result parse errors](https://github.com/rust-lang/rust/pull/81608)
* [identify unreachable subpatterns more reliably](https://github.com/rust-lang/rust/pull/80632)
* [fix issues with move closures and mutability](https://github.com/rust-lang/rust/pull/80092)
* [const_evaluatable: consider sub-expressions to be evaluatable](https://github.com/rust-lang/rust/pull/81577)
* [introduce future-compatibility warning for forbidden lint groups](https://github.com/rust-lang/rust/pull/81556)
* [`Box` the biggest `ast::ItemKind` variants](https://github.com/rust-lang/rust/pull/81405)
* [improve error message for disallowed ptr-to-int casts in const eval](https://github.com/rust-lang/rust/pull/81779)
* [typeck: emit structured suggestions for tuple struct syntax](https://github.com/rust-lang/rust/pull/81737)
* [faster few span methods](https://github.com/rust-lang/rust/pull/81735)
* [fix bug with `assert!()` calling the wrong edition of `panic!()`](https://github.com/rust-lang/rust/pull/81647)
* [make `Allocator` object-safe](https://github.com/rust-lang/rust/pull/81730)
* [add Frames Iterator for Backtrace](https://github.com/rust-lang/rust/pull/81022)
* [add `Vec::extend_from_within` method under `vec_extend_from_within` feature gate](https://github.com/rust-lang/rust/pull/79015)
* [`BTreeMap`: make `Ord` bound explicit, compile-test its absence](https://github.com/rust-lang/rust/pull/81610)
* [implement `TrustedLen` for `Fuse<I: TrustedLen>`](https://github.com/rust-lang/rust/pull/81599)
* [rename `Iterator::fold_first` to `reduce` and stabilize it](https://github.com/rust-lang/rust/pull/79805)
* [stabilize the `Wake` trait](https://github.com/rust-lang/rust/pull/74304)
* [stabilize `peekable_next_if`](https://github.com/rust-lang/rust/pull/80011)
* [stabilize poison API of `Once`, rename `poisoned()`](https://github.com/rust-lang/rust/pull/81745)
* [stabilize remaining integer methods as `const fn`](https://github.com/rust-lang/rust/pull/80962)
* [futures-rs: avoid `once_cell` in static wakers](https://github.com/rust-lang/futures-rs/pull/2332)
* [hashbrown: implement `From<HashMap<T, ()>>` for `HashSet<T>`](https://github.com/rust-lang/hashbrown/pull/235)
* [cargo: fix panic with doc collision orphan](https://github.com/rust-lang/cargo/pull/9142)
* [cargo: fix env/cfg set for `cargo test` and `cargo run`](https://github.com/rust-lang/cargo/pull/9122)
* [make rustdoc respect `--error-format short` in doctests](https://github.com/rust-lang/rust/pull/81675)
* [clippy: fix `let_underscore_drop` false positive](https://github.com/rust-lang/rust-clippy/pull/6682)
* [clippy: fix `let_and_return` false positive](https://github.com/rust-lang/rust-clippy/pull/6659)
* [clippy: don't trigger `exhaustive_structs` for structs with private fields](https://github.com/rust-lang/rust-clippy/pull/6661)
* [clippy: add new lint `missing_panics_doc`](https://github.com/rust-lang/rust-clippy/pull/6523)
* [compiletest: Add option to emit compiler stderr per bitwidth](https://github.com/rust-lang/rust/pull/81817)

## Rust Compiler Performance Triage

*No triage report this week*

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Rust 2021 Roadmap](https://github.com/rust-lang/rfcs/pull/3037)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [adds async stream rfc](https://github.com/rust-lang/rfcs/pull/2996)
* [RFC: add the Freeze trait to libcore/libstd](https://github.com/rust-lang/rfcs/pull/2944)
* [Generic Pointer to Field](https://github.com/rust-lang/rfcs/pull/2708)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Allow leading | anywhere we allow or-patterns](https://github.com/rust-lang/rust/issues/81415)
* [disposition: merge] [`impl PartialEq<Punct> for char`; symmetry for #78636](https://github.com/rust-lang/rust/pull/80595)
* [disposition: merge] [Add an impl of Error on `Arc<impl Error>`.](https://github.com/rust-lang/rust/pull/80553)
* [disposition: merge] [Add `NotSupported` to `std::io::ErrorKind`](https://github.com/rust-lang/rust/pull/78880)
* [disposition: close] [Tracking issue for `Option::expect_none(msg)` and `unwrap_none()`](https://github.com/rust-lang/rust/issues/62633)

## New RFCs

* [Add named path bases to cargo](https://github.com/rust-lang/rfcs/pull/3074)

# Upcoming Events

### Online
* [February 11, Washington, DC, US - Let's learn to Rust nice with others - Rust DC](https://www.meetup.com/RustDC/events/275569653)
* [February 11, San Diego, CA, US - February 2021 Tele-Meetup - San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/276272745/)
* [February 17, Vancouver, BC, CA - Rust Study/Hack/Hang-out night - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/npqfbsyccdbwb/)
* [February 18, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprryccdbxb/)
* [February 23, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccdbfc/)


### North America
* [February 11, Columbus, OH, US - Monthly Meeting - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgryccdbpb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Backend Engineer at TrueLayer (London, UK)](https://apply.workable.com/truelayer/j/D07759DAF6/)
* [Rust Backend Engineer at TrueLayer (Milan, Italy)](https://apply.workable.com/truelayer/j/F13E839E3B/)
* [Rust Technical Lead at TrueLayer (London, UK)](https://apply.workable.com/truelayer/j/3B78A6F6F4/)
* [Rust Technical Lead at TrueLayer (Milan, Italy)](https://apply.workable.com/truelayer/j/8D8D56C09E/)
* [Senior Software Engineer (Rust & C++) at NZXT (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=259)
* [Rust Software Engineer at JetASAP (Remote US or CA)](https://angel.co/company/jetasap/jobs/1178965-rust-software-engineer)
* [Backend Engineer - Rust at Kraken (Remote)](https://jobs.lever.co/kraken/4019a818-4a7b-46ef-9225-c53c7a7f238c)
* [Backend Engineer, Kraken Futures - Rust at Kraken (Remote)](https://jobs.lever.co/kraken/fe1e07f4-6d7c-4f65-9a8f-27cf3b3fd2b1)
* [Rust API SDET at Kraken (Remote)](https://jobs.lever.co/kraken/5ec9958a-529c-4bae-89b3-0d1a104cbd81)
* [Rust Engineer, Desktop GUI - Cryptowatch at Kraken (Remote)](https://jobs.lever.co/kraken/2442ee5c-56b6-4a73-a477-8cdda2b218d5)
* [Senior Banking Engineer - Rust at Kraken (Remote)](https://jobs.lever.co/kraken/2863623f-13c9-4f50-992d-7c25736a60f9)
* [Software Engineer - Trading Technology (Rust) at Kraken (Remote)](https://jobs.lever.co/kraken/4485f672-dc5f-4e49-a10b-2b0399e28a8d)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> The main theme of Rust *is not* systems programming, speed, or memory safety - it's moving runtime problems to compile time. Everything else is incidental. This is an invaluable quality of any language, and is something Rust greatly excels at.

– [/u/OS6aDohpegavod4 on /r/rust](https://www.reddit.com/r/rust/comments/leki5o/advantages_of_building_a_crud_web_application_in/gmfq2w9/)

Thanks to [Chris](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1001) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/lhe7yi/this_week_in_rust_377/)</small>
