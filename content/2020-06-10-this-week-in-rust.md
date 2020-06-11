Title: This Week in Rust 342
Number: 342
Date: 2020-06-10
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

There is no *This Week in Rust* podcast this week, next week's episode will cover both this week and next week.

Check out [this week's *This Week in Rust Podcast*](https://rustacean-station.org/episode/017-twir-341-342/)

# Updates from Rust Community

## News & Blog Posts

* [Announcing Rust 1.44.0](https://blog.rust-lang.org/2020/06/04/Rust-1.44.0.html)
* [RustConf 2020 Registration is Open](https://rustconf.com/)
* [Enumerating monitors in Rust using Win32 API](https://patriksvensson.se/2020/06/enumerating-monitors-in-rust-using-win32-api/)
* [Rust cli app integrated with slack](https://bprog.github.io/rust_slack_bot/)
* [Hack week: miniCouchDB in Rust](https://www.garrensmith.com/blogs/mini-couch-hack-week)
* [Zero To Production #1: toolchains, IDEs, CI](https://www.lpalmieri.com/posts/2020-06-06-zero-to-production-1-setup-toolchain-ides-ci/)
* [From Rust to Svelte, what tech stack will I use](https://hugodaniel.pt/posts/browser-tech-stack-2020/)
* [Graph & Tree Traversals in Rust](https://sachanganesh.com/programming/graph-tree-traversals-in-rust/)
* [Programming languages: Rust enters top 20 popularity rankings for the first time](https://www.zdnet.com/article/programming-languages-rust-enters-top-20-popularity-rankings-for-the-first-time/)
* [A Rust SentencePiece implementation](https://guillaume-be.github.io/2020-05-30/sentence_piece)
* [Rust Things I Miss in C](https://people.gnome.org/~federico/blog/rust-things-i-miss-in-c.html)
* [So What's Up with Microsoft's (and Everyone Else's) Love of Rust?](https://visualstudiomagazine.com/articles/2020/06/02/rust-love.aspx?m=1)
* [Why the developers who use Rust love it so much](https://stackoverflow.blog/2020/06/05/why-the-developers-who-use-rust-love-it-so-much/?cb=1)
* [The Story of Tail Call Optimizations in Rust](https://dev.to/seanchen1991/the-story-of-tail-call-optimizations-in-rust-35hf)
* [Taking the Unhappy Path with Result, Option, unwrap and ? operator in Rust](https://dev.to/codeprototype/taking-the-unhappy-path-with-result-option-unwrap-and-operator-in-rust-482b)
* [This Month in Rust OSDev (May 2020)](https://rust-osdev.com/this-month/2020-05/)
* [This Month in Rust GameDev #10 - May 2020](https://rust-gamedev.github.io/posts/newsletter-010/)
* [This month in rustsim #11 (April - May 2020)](https://www.rustsim.org/blog/2020/06/01/this-month-in-rustsim/)
* [Rust in Blockchain Newsletter #12 - ZK-Rustups](https://rustinblockchain.org/newsletters/2020-06-03-zk-rustups/)
* [Memory-Safety Challenge Considered Solved? An Empirical Study with All Rust CVEs](https://arxiv.org/abs/2003.03296)
* [Creating Your Own Programming Language with Rust](https://createlang.rs/)
* [rust-analyzer changelog #28](https://rust-analyzer.github.io/thisweek/2020/06/08/changelog-28.html)
* [Getting started with Rust/WinRT](https://kennykerr.ca/2020/06/05/getting-started-with-rust-winrt/)
* [Chinese] [Simple sorting algorithms in Rust](https://www.bilibili.com/read/cv4991161)
* [Indonesian] [Berbagai alasan melakukan Programming dalam Rust](https://dev.to/rizki96/berbagai-alasan-melakukan-programming-dalam-rust-p67)
* [slides] [Rust in 15 Minutes](https://geigerkind.github.io/training_presentations/rust_in_15_minutes/presentation.html)
* [video] [Rust Web development | Boilerplate free with Rocket](https://youtu.be/tjH0Mye8U_A)
* [video] [Educational Rust Live Coding - Building a web app - Part 4](https://www.youtube.com/watch?v=Dj8i3rM8FIQ)
* [video] [Iterators - Rust](https://www.youtube.com/watch?time_continue=1&v=HZftwxCIXqE&feature=emb_logo)
* [video] [Browser computation with WebAssembly Live Stream](https://www.twitch.tv/occupy_paul_st)
* [video] [Jonathan Teaches Jason Rust!](https://www.youtube.com/watch?v=Y5-ZgxfQvpc)

# Crate of the Week

This week's crate is [cargo-spellcheck](https://github.com/drahnr/cargo-spellcheck), a cargo subcommand to spell-check your docs.

Thanks to [Bernhard Schuster](https://users.rust-lang.org/t/crate-of-the-week/2704/777) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

* [maud: Add an on-by-default "unstable" feature](https://github.com/lambda-fairy/maud/issues/187)
* [maud: Change all uses of Span::def_site to Span::mixed_site](https://github.com/lambda-fairy/maud/issues/190)
* [maud: Document "Allow arbitrary attribute syntax in class and ID names"](https://github.com/lambda-fairy/maud/issues/167)
* [maud: Migrate to the third-party `quote!` macro](https://github.com/lambda-fairy/maud/issues/191)

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.


If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

350 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-06-01..2020-06-08

* [InstCombine: don't optimize `&mut *x` into `x`](https://github.com/rust-lang/rust/pull/72820)
* [add `-Z span-debug` to allow for easier debugging of proc macros](https://github.com/rust-lang/rust/pull/72799)
* [avoid setting wrong obligation cause span of associated type mismatch](https://github.com/rust-lang/rust/pull/72807)
* [be more careful around `ty::Error` in generators](https://github.com/rust-lang/rust/pull/72764)
* [fulfill: try using `SmallVec` or `Box` for `stalled_on`](https://github.com/rust-lang/rust/pull/72776)
* [`impl AsRef<[T]> for vec::IntoIter<T>`](https://github.com/rust-lang/rust/pull/72583)
* [chalk: get ready for the first publish](https://github.com/rust-lang/chalk/pull/483)
* [free `default()` forwarding to `Default::default()`](https://github.com/rust-lang/rust/pull/73001)
* [stabilize `std::io::Buf{Reader, Writer}::capacity`](https://github.com/rust-lang/rust/pull/72924)
* [add associated consts `MIN`/`MAX` for `Wrapping<Int>`](https://github.com/rust-lang/rust/pull/72891)
* [de-promote Duration::from_secs](https://github.com/rust-lang/rust/pull/71796)
* [compiler-builtins: manually patch ret instruction for LVI](https://github.com/rust-lang/compiler-builtins/pull/359)
* [cargo: add environment variables to identify the binary and crate name](https://github.com/rust-lang/cargo/pull/8270)
* [cargo: allow Windows dylibs without dll suffix](https://github.com/rust-lang/cargo/pull/8310)
* [cargo: better error message when passing in relative path to `Workspace::new`](https://github.com/rust-lang/cargo/pull/8321)
* [cargo: don't hash executable filenames on apple platforms](https://github.com/rust-lang/cargo/pull/8329)
* [cargo: support `{prefix}` and `{lowerprefix}` markers in `config.json` `dl` key](https://github.com/rust-lang/cargo/pull/8267)
* [cargo: warn if using hash in git URL](https://github.com/rust-lang/cargo/pull/8297)
* [cargo: reset lockfile information between resolutions](https://github.com/rust-lang/cargo/pull/8274)
* [crates.io: fix issue where crates.io allowed the plus sign in crate names](https://github.com/rust-lang/crates.io/pull/2551)
* [docs.rs: print a backtrace for crates which fail to build](https://github.com/rust-lang/docs.rs/pull/823)
* [rustfmt: pick up comments between visibility modifier and item name](https://github.com/rust-lang/rustfmt/pull/4239)
* [rustfmt: preserve Markdown line breaks in inner and outer block doc comments](https://github.com/rust-lang/rustfmt/pull/4233)
* [rustfmt: use rewrite buffer to determine if comment should be on a newline](https://github.com/rust-lang/rustfmt/pull/4229)
* [rustfmt: feat: conditionally allow unstable opts on stable/beta](https://github.com/rust-lang/rustfmt/pull/4228)

## Rust Compiler Performance Triage

* [2020-06-09](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020.md#2020-06-09)

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

* [disposition: merge] [`impl ToSocketAddrs for (String, u16)`](https://github.com/rust-lang/rust/pull/73007)
* [disposition: merge] [Stabilize `Option::zip`](https://github.com/rust-lang/rust/pull/72938)
* [disposition: merge] [Stabilize `vec::Drain::as_slice`](https://github.com/rust-lang/rust/pull/72584)
* [disposition: merge] [Add raw_ref macros](https://github.com/rust-lang/rust/pull/72279)
* [disposition: merge] [Print environment variables accessed by rustc as special comments into depinfo files](https://github.com/rust-lang/rust/pull/71858)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [June 10. Wroclaw, PL - Remote - Rust Wroclaw Meetup #20](https://www.meetup.com/Rust-Wroclaw/events/271034483/)
* [June 11. Berlin, DE - Remote - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybcjbpb/)
* [June 11. San Diego, CA, US - Remote - San Diego Rust Meetup](https://www.meetup.com/San-Diego-Rust/events/270938860/)
* [June 18. Zurich, CH - Remote - Embedded Rust Update: probe.rs](https://www.meetup.com/Rust-Zurich/events/271020472/)
* [June 18. Turin, IT - Remote - Rust Turin Study Group](https://community.mozilla.org/events/gruppo-di-studio-di-rust-2/)

### North America
* [June 11. Columbus, OH, US - Columbus Rust Society Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybcjbpb/)
* [June 11. Lehi, UT, US - Utah Rust - Lightning Talks](https://www.meetup.com/utah-rust/events/269263282/)
* [June 17. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybcjbwb/)
* [June 18. Durham, NC - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybcjbdc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs
* [Pop! OS Software Engineer at System76, Remote US-based](https://system76.com/careers#software-engineer-pop-os)
* [Software Engineer at Vehicle.Software](https://vehicle.software/careers/) (second listing on page)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> You don't declare lifetimes. Lifetimes come from the shape of your code, so to change what the lifetimes are, you must change the shape of the code.

– [Alice Ryhl on rust-users](https://users.rust-lang.org/t/lifetime-of-a-returned-iterator/43732/2)

Thanks to [RustyYato](https://users.rust-lang.org/t/twir-quote-of-the-week/328/883) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/h0mt9k/this_week_in_rust_342/)</small>
