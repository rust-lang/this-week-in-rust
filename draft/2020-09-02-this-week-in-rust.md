Title: This Week in Rust 354
Number: 354
Date: 2020-09-02
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/emberian/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/emberian/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/emberian/this-week-in-rust/pulls).

# Updates from Rust Community

### Official
* [Announcing Rust 1.46.0](https://blog.rust-lang.org/2020/08/27/Rust-1.46.0.html)

### Tooling
* [Rust Analyzer Changelog #40](https://rust-analyzer.github.io/thisweek/2020/08/31/changelog-40.html)

### Newsletters

### Observations/Thoughts
* [Should we trust Rust with the future of systems programming?](https://edfloreshz.blog/should-we-trust-rust-with-the-future-of-systems-programming)
* [Optionality in the type systems of Julia and Rust](https://andreaskroepelin.de/blog/sum_types/)
* [Migrating from quick-error to SNAFU: a story on revamped error handling in Rust](https://dev.to/e_net4/migrating-from-quick-error-to-snafu-a-story-on-revamped-error-handling-in-rust-58h9)
* [Go use those Traits](https://prateeknischal.github.io/go-use-those-traits/)
* [Starframe devlog: Architecture](https://moletrooper.github.io/blog/2020/08/starframe-1-architecture/)
* [Event Chaining as a Decoupling Method in Entity-Component-System](https://www.jojolepro.com/blog/2020-08-20_event_chaining/)
* [Rust serialization: What's ready for production today?](https://blog.logrocket.com/rust-serialization-whats-ready-for-production-today/)
* [video] [Is Rust Used Safely by Software Developers?](https://youtu.be/iOBXVOAbpdY)

[rust vs scripted languages; a short case study](https://www.linkedin.com/pulse/rust-goodness-case-study-matthew-sherborne)

### Learn Standard Rust
* [Ownership in Rust, Part 1](https://www.thomascountz.com/2018/07/09/ownership-in-rust-part-1)
* [Ownership in Rust, Part 2](https://www.thomascountz.com/2018/07/11/ownership-in-rust-part-2)
* [Learning Rust 1: Install things and read a file](https://dev.to/jbachhardie/learning-rust-1-install-things-and-read-a-file-5613)
* [Learning Rust 2: A Tiny Database is Born](https://dev.to/jbachhardie/learning-rust-2-a-tiny-database-is-born-eef)
* [That's so Rusty: Ownership](https://dev.to/imaculate3/that-s-so-rusty-ownership-493c)

### Learn More Rust
* [As above, so below: Bare metal Rust generics 2/2](https://www.ecorax.net/as-above-so-below-2/)
* [Halite III Bot Development Kit in Rust](https://sgolem.com/halite-iii-bot-development-kit-in-rust)
* [Zero To Production #3.5: HTML forms, Databases, Integration tests](https://www.lpalmieri.com/posts/2020-08-31-zero-to-production-3-5-html-forms-databases-integration-tests/)
* [Objective-Rust](https://belkadan.com/blog/2020/08/Objective-Rust/)
* [Building web apps with Rust using the Rocket framework](https://blog.logrocket.com/rust-web-apps-using-rocket-framework/)
* [Writing an asynchronous MQTT Broker in Rust - Part 3](https://hassamuddin.com/blog/rust-mqtt/channels/)
* [Multiple Thread Pools in Rust](https://pkolaczk.github.io/multiple-threadpools-rust/)
* [Type-directed metaprogramming in Rust](https://willcrichton.net/notes/type-directed-metaprogramming-in-rust/)
* [Generating static arrays during compile time in Rust](https://dev.to/rustyoctopus/generating-static-arrays-during-compile-time-in-rust-10d8)
* [Let's build a single binary gRPC server-client with Rust in 2020 - Part 2](https://dev.to/tjtelan/let-s-build-a-single-binary-grpc-server-client-with-rust-in-2020-part-2-1i2e)
* [How to use Rust + WebAssembly to Perform Serverless Machine Learning and Data Visualization in the Cloud](https://www.freecodecamp.org/news/rust-webassembly-serverless-tencent-cloud/)
* [Fireworks for your terminal](https://blog.darrien.dev/posts/fireworks-for-your-terminal/)
* [Serverless Data Ingestion with Rust and AWS SES](http://jamesmcm.github.io/blog/2020/08/29/rust-ses/#en)
* [Box Plots at the Olympics](https://datacrayon.com/posts/programming/rust-notebooks/box-plots-at-the-olympics/)
* [Fixing include_bytes!](https://jack.wrenn.fyi/blog/include-transmute/)
* [video] [FLTK Rust: Creating a music player with customized widgets](https://youtu.be/okdFx6tv7ds)

[PL] [CrabbyBird #1 O tym, jak animować postać gracza](https://postacnormalna.pl/animacja-kraboptaka/)

[Generating static arrays during compile time in Rust](https://dev.to/rustyoctopus/generating-static-arrays-during-compile-time-in-rust-10d8)

### Project Updates
* [This August in my database project written in Rust](https://alex-dukhno.github.io/2020-08-29-This-August-in-my-Database-project-written-in-rust-copy/)
* [Using Rust for Kentik's New Synthetic Monitoring Agent](https://www.kentik.com/blog/using-rust-for-kentiks-new-synthetic-network-monitoring-agent/)
* [AWS introduces Bottlerocket: A Rust language-oriented Linux for containers](https://www.zdnet.com/article/aws-introduces-bottlerocket-a-rust-language-oriented-linux-for-containers/)
* [Using `cargo test` for embedded testing with `panic-probe`](https://ferrous-systems.com/blog/cargo-test-with-panic-probe/)
* [Headcrab: August 2020 progress report](https://headcrab.rs/2020/08/31/august-update.html)
* [Rust Analyzer 2020[..6] Financial Report](https://rust-analyzer.github.io/blog/2020/08/20/financial-report.html)

### Miscellaneous
* [Avoid Build Cache Bloat By Sweeping Away Artifacts](https://www.justanotherdot.com/posts/avoid-build-cache-bloat-by-sweeping-away-artifacts.html)
* [const_fn makes it too easy to do mandelbrots](https://www.reddit.com/r/rust/comments/ijpxz2/const_fn_makes_it_too_easy_to_do_mandelbrots/)
* [Linux Developers Continue Evaluating The Path To Adding Rust Code To The Kernel](https://www.phoronix.com/scan.php?page=news_item&px=Linux-Kernel-Rust-Path-LPC2020)
* [Supporting Linux kernel development in Rust](https://lwn.net/SubscriberLink/829858/281103f9c6fd0dc2/)
* [video] [LPC 2020 - LLVM MC](https://youtu.be/FFjV9f_Ub9o)

* [Serverless Data Ingestion with Rust and AWS SES](http://jamesmcm.github.io/blog/2020/08/29/rust-ses/#en)

* [Refactoring Rust Transpiled from C](https://immunant.com/blog/2020/09/transpiled_c_safety/)

# Crate of the Week

This week's crate is [pdf](https://github.com/pdf-rs/pdf), a crate for reading PDF files.

Thanks to [S3bk](https://users.rust-lang.org/t/crate-of-the-week/2704/806) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Database in Rust is looking for contributors](https://github.com/alex-dukhno/database/issues?q=is%3Aopen+is%3Aissue+label%3A%22help+wanted%22)
* [Docs.rs is looking for help adding a changelog](https://github.com/rust-lang/docs.rs/issues/1013)


If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

292 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-08-17..2020-08-24

* [remove fast path in reallocation for same layout sizes](https://github.com/rust-lang/rust/pull/75621)
* [missing doc examples lint improvements](https://github.com/rust-lang/rust/pull/75776)
* [promote missing_fragment_specifier to hard error](https://github.com/rust-lang/rust/pull/75516)
* [polymorphize: if any param in a predicate is used, then all are used](https://github.com/rust-lang/rust/pull/75595)
* [make `OnceCell<T>` transparent to dropck](https://github.com/rust-lang/rust/pull/75648)
* [don't panic in `Vec::shrink_to_fit`](https://github.com/rust-lang/rust/pull/75677)
* [improve codegen for `align_offset`](https://github.com/rust-lang/rust/pull/75600)
* [add `Arc::new_cyclic`](https://github.com/rust-lang/rust/pull/75505)
* [new zeroed slice](https://github.com/rust-lang/rust/pull/75171)
* [make `<*const T>::is_null` const fn](https://github.com/rust-lang/rust/pull/74940)
* [stabilize `ptr_offset_from`](https://github.com/rust-lang/rust/pull/74238)
* [use `min_specialization` in libcore](https://github.com/rust-lang/rust/pull/73565)
* [const floating point bitcasts and classification](https://github.com/rust-lang/rust/pull/72449)
* [compiler-builtins: add mips/mips64 compiler-rt fallbacks so that libgcc is not required](https://github.com/rust-lang/compiler-builtins/pull/341)
+ [pin-utils: deprecate unsafe pin projection macros](https://github.com/rust-lang/pin-utils/pull/33)
* [git2: fix dangling pointer in format_email](https://github.com/rust-lang/git2-rs/pull/614)
* [git2: add support for zlib-ng](https://github.com/rust-lang/git2-rs/pull/612)
* [cargo: remove unnecessary allocations](https://github.com/rust-lang/cargo/pull/8641)
* [rust-bindgen: do generate unnamed enums, as they can be referred to by members and others](https://github.com/rust-lang/rust-bindgen/pull/1882)

## Rust Compiler Performance Triage

* [2020-08-24](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-08-24.md):
  1 regression, 4 improvements.
  
  This week included a major speedup on optimized builds of real-world crates (up to 5%) as a result of the [upgrade to LLVM 11](https://github.com/rust-lang/rust/pull/73526#issuecomment-679374070).

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.


### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Update the intra-doc link RFC to match the implementation](https://github.com/rust-lang/rfcs/pull/2975)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge][Stabilize deque_make_contiguous](https://github.com/rust-lang/rust/pull/74559)
* [disposition: merge][Tracking issue for hint::spin_loop (renamed sync::atomic::spin_loop_hint)](https://github.com/rust-lang/rust/issues/55002)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [September 2. Johannesburg, ZA - Monthly Joburg Rust Chat! - Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/272786420/)
* [September 2. Dublin, IE - Rust Dublin September Remote Meetup](https://www.meetup.com/Rust-Dublin/events/272781420/?action=rsvp&response=yes)
* [September 2. Indianapolis, IN, US - Indy Rust - Indy.rs - with Social Distancing](https://www.meetup.com/indyrs)
* [September 3. Berlin, DE - Berlin.rs - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybcmbfb/)
* [September 8. Saarbrücken, DE - Rust-Saar Meetup - `3u16.map_err(...)`](https://www.meetup.com/Rust-Saar/events/272522454/)
* [September 9. East Coast, US - Rust East Coast Virtual Talks + Q&A](https://www.meetup.com/Rust-NYC/events/272982073/)

### North America
* [September 9. Atlanta, GA, US - Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgrybcmbmb/)

### Asia Pacific
* [September 7. Auckland, NZ - Rust AKL - Finally, good Rust IDE support in VSCode: rust-analyzer](https://www.meetup.com/rust-akl/events/266876702/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust is a very different beast for me. It is a *much* bigger and *much* more capable language. However, I've found that it is, in many ways, a lot more restrictive in how you can approach problems. I frequently find myself being perplexed at how to eloquently solve a problem. When I discover the idiomatic way of doing it I'm usually both blown away by the brilliance of it and a bit disheartened by how difficult it would be to come up with that solution by myself :-).

- [mikekchar on /r/rust](https://reddit.com/r/rust/comments/id8n8d/are_some_of_you_coming_from_javascript_ts/g27d3ni/)

Thanks to [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/931) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust]()</small>
