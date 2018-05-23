Title: This Week in Rust 235
Number: 235
Date: 2018-05-22
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

* [The Rust compiler is getting faster](https://blog.mozilla.org/nnethercote/2018/05/17/the-rust-compiler-is-getting-faster/).
* [Rand 0.5.0 released with quite significant changes over 0.4](https://www.reddit.com/r/rust/comments/8l95zk/rand_050_released/).
* [Cannoli: A Python Compiler Written Entirely in Rust](https://github.com/joncatanio/cannoli).
* [Auto reloading development web servers with systemfd/listenfd](https://www.reddit.com/r/rust/comments/8kpea2/ann_auto_reloading_development_web_servers_with/).
* [Migrating to Actix Web from Rocket for stability](https://nbsoftsolutions.com/blog/migrating-to-actix-web-from-rocket-for-stability).
* [Improving SmallVec's speed by 60% and why that shouldn't matter to you](http://troubles.md/posts/improving-smallvec/).
* [Validating UTF-8 strings using as little as 0.7 cycles per byte](https://lemire.me/blog/2018/05/16/validating-utf-8-strings-using-as-little-as-0-7-cycles-per-byte/).
* [Swapping and dropping to accelerate Rust in a benchmark](https://barrielle.cedeela.fr/research_page/dropping-drops.html).
* [Building a Datalog engine in under 300 lines of Rust](https://github.com/frankmcsherry/blog/blob/master/posts/2018-05-19.md).
* [Porting Rust benchmarks to Criterion](https://llogiq.github.io/2018/05/18/criterion.html).
* [Compile time prevention of SQL-injections in Rust](https://polyfloyd.net/post/compile-time-prevention-of-sql-injections/).
* [The Embedded Working Group newsletter 5](https://internals.rust-lang.org/t/the-embedded-working-group-newsletter-5/7536).
* [This week in Rust docs 106](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-106).
* [podcast] [CoRecursive w/ Adam Bell - 013](https://corecursive.com/013-rust-and-with-jim-blandy). Rust and bitter C++ developers with Jim Blandy, one of the authors of Programming Rust.
* [podcast] [Rusty Spike Podcast - episode 29](https://rusty-spike.blubrry.net/2018/05/17/episode-29-may-16-2018/). Square, Amazon, the 1.26 release, Cloudflare, Rust’s birthday, and three-fold improvements.

# Crate of the Week

This week's crate is [Askama](https://crates.io/crates/askama), a Jinja-like type-safe compiled templating engine. Thanks to [Icefoxen](https://users.rust-lang.org/u/Icefoxen) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [rustc-guide](https://github.com/rust-lang-nursery/rustc-guide) is a project to write a short guide about how the rust compiler works, and it needs your help. There are some [easier issues](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3AEasy), [issues which might require a bit of investigation/code reading](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3AMedium), and [issues which probably require some advanced knowledge or a lot of time](https://github.com/rust-lang-nursery/rustc-guide/issues?utf8=%E2%9C%93&q=is%3Aissue+is%3Aopen+label%3AHard).
* [annotate-snippets](https://github.com/zbraniecki/annotate-snippets-rs) - a crate for code snippets visual annotations (think: rustc error display) released 0.1 and is looking for code review, testing, and feedback.
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

153 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-05-07..2018-05-14

* [set PrepareForThinLTO flag when using ThinLTO](https://github.com/rust-lang/rust/pull/50684)
* [typeck: fix ICE with struct update syntax](https://github.com/rust-lang/rust/pull/50643)
* [typeck: save the index of private fields](https://github.com/rust-lang/rust/pull/50693)
* [use `SmallVec` for `DepNodeIndex` within `dep_graph`](https://github.com/rust-lang/rust/pull/50565)
* [inline `Span` methods](https://github.com/rust-lang/rust/pull/50564)
* [don't use Lock for heavily accessed `CrateMetadata::cnum_map`](https://github.com/rust-lang/rust/pull/50532)
* [do not silently truncate offsets for `read_at`/`write_at` on emscripten](https://github.com/rust-lang/rust/pull/50634)
* [fix `panic` for binaries built during tests](https://github.com/rust-lang/cargo/pull/5513)
* [fix volatile_store and nontemporal_store](https://github.com/rust-lang/rust/pull/50648)
* [rustc: leave space for fields of uninhabited types to allow partial initialization](https://github.com/rust-lang/rust/pull/50622)
* [rustc: don't trip an assertion for enums with present but uninhabited variants](https://github.com/rust-lang/rust/pull/50735)
* [rustc: allow an edition's feature on that edition](https://github.com/rust-lang/rust/pull/50663)
* [rustc: include semicolon when removing `extern crate`](https://github.com/rust-lang/rust/pull/50670)
* [improve single-use and zero-use lifetime lints](https://github.com/rust-lang/rust/pull/50440)
* [prevent infinite recursion of modules](https://github.com/rust-lang/rust/pull/50305)
* [fix self referential impl Trait substitutions](https://github.com/rust-lang/rust/pull/50694)
* [macros: Add a 'literal' fragment specifier](https://github.com/rust-lang/rust/pull/49835)
* [rename Pin to PinMut, and some more breaking changes](https://github.com/rust-lang/rust/pull/50497)
* [stabilize macro_lifetime_matcher](https://github.com/rust-lang/rust/pull/50385)
* [don't allocate when creating an empty BTree](https://github.com/rust-lang/rust/pull/50352)
* [only lookup types in one interner](https://github.com/rust-lang/rust/pull/50332)
* [idiom lints for removing `extern crate`](https://github.com/rust-lang/rust/pull/50260)
* [added missing implementation hint](https://github.com/rust-lang/rust/pull/50161)
* [make `String::new()` const](https://github.com/rust-lang/rust/pull/50460)
* [turn `ManuallyDrop::new` into a constant function](https://github.com/rust-lang/rust/pull/50148)
* [std: avoid `ptr::copy` if unnecessary in `vec::Drain`](https://github.com/rust-lang/rust/pull/50575)
* [add fn `into_inner(self) -> (Idx, Idx)` to RangeInclusive](https://github.com/rust-lang/rust/pull/50574)
* [./x.py test should be able to run individual tests](https://github.com/rust-lang/rust/pull/49729)

## New Contributors

* bstrie
* Daniel Mueller
* George Burton
* Jane Lusby
* Kyle Stachowicz
* Mikela
* Robin Krahl
* SHA Miao

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Keyword unreservations (pure, sizeof, alignof, offsetof)](https://github.com/rust-lang/rfcs/pull/2421).
* [disposition: merge] [Unstable features accidentally usable on the Stable release chanel are still unstable](https://github.com/rust-lang/rfcs/pull/2405).
* [disposition: merge] [`#[used]` static variables](https://github.com/rust-lang/rfcs/pull/2386).
* [disposition: merge] [Allow `if let` guards in `match` expressions](https://github.com/rust-lang/rfcs/pull/2294).

## New RFCs

* [Reserve `f(a = b)` in Rust 2018](https://github.com/rust-lang/rfcs/pull/2443).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [May 24. Madrid, ES - Cuarto meetup de MadRust](https://www.meetup.com/MadRust/events/gfrdspyxhbgc/).
* **[May 27. Paris, FR - RustFest Paris 2018](https://paris.rustfest.eu/)**.
* [May 27. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxhbkc/).
* [May 29. Dallas, US - Last Tuesday Meetup](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxhbmc/).
* [May 30/31. Berlin, DE - Rust/WASM course around JSConf.EU](https://ti.to/asquera-event-ug/rust-wasm-wwwtf-2018/).
* [May 30. Berlin, DE - Berlin Mozilla Meetup - Rust Hack and Learn](https://www.meetup.com/Berlin-Mozilla-Meetup/events/tvmmslyxhbnc/).
* [May 30. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [May 30. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/ckwdlpyxhbnc/).
* [May 30. Milano, IT - Rust Exercises](https://www.meetup.com/rust-language-milano/events/250868847/).
* [Jun  2. Florianópolis, BR - 1º Encontro Rust Floripa](https://www.meetup.com/rustfloripa/events/xvglrpyxjbdb/).
* [Jun  3. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxhbbc/).
* [Jun  5. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Jun  5. Johannesburg, ZA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxjbhb/).
* [Jun  6. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Jun  6. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Jun  6. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/cpvshpyxjbjb/).
* [Jun  6. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxjbjb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
