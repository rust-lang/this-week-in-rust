Title: This Week in Rust 394
Number: 394
Date: 2021-06-09
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

* [Filecoin Forest update](https://medium.com/chainsafe-systems/back-into-the-forest-983a4344ffe9)
* [Mina Protocol update](https://medium.com/chainsafe-systems/realizing-the-mina-vision-in-rust-453f6f522205)

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts
* [Behavior inheritance in Rust](https://abadcafe.wordpress.com/2021/01/08/behavior-inheritance-in-rust/)

### Rust Walkthroughs
* [Creating an NPM package written in Rust](https://popcornpaws.medium.com/creating-an-npm-package-written-in-rust-ce02f7c55458)
* [ZH] [series] [Build front-end web apps with Yew and WebAssembly in Rust -part 1: crates (Rust 和 Wasm 的融合，使用 yew 构建 WebAssembly 标准的 web 前端（1）- 起步及 crate 选择)](https://blog.budshome.com/budshome/rust-he-wasm-de-rong-he-,shi-yong-yew-gou-jian-webassembly-biao-zhun-de-web-qian-duan-(1)--qi-bu-ji-crate-xuan-ze)
* [ZH] [series] [Build front-end web apps with Yew and WebAssembly in Rust -part 2: Components & Routers (Rust 和 Wasm 的融合，使用 yew 构建 WebAssembly 标准的 web 前端（2）- 组件和路由)](https://blog.budshome.com/budshome/rust-he-wasm-de-rong-he-,shi-yong-yew-gou-jian-webassembly-biao-zhun-de-web-qian-duan-(2)--zu-jian-he-lu-you)

### Miscellaneous

* [From Julia to Rust](https://miguelraz.github.io/blog/juliatorust/) 

## Crate of the Week

This week's crate is [cargo-sort](https://github.com/DevinR528/cargo-sort), a cargo subcommand to sort your `Cargo.toml`'s dependencies and workspace members.

Thanks to [jplatte](https://users.rust-lang.org/t/crate-of-the-week/2704/921) for the nomination

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

* [Backroll-rs is looking for contributors](https://www.reddit.com/r/rust/comments/npnl1p/help_wanted_with_backrollrs_new_networking_library/)

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

267 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-05-31..2021-06-07

* [BPF target support](https://github.com/rust-lang/rust/pull/79608)
* [support for force-warns](https://github.com/rust-lang/rust/pull/85788)
* [improve debugging experience for enums on windows-msvc](https://github.com/rust-lang/rust/pull/85292)
* [parser: ensure that all nonterminals have tokens after parsing](https://github.com/rust-lang/rust/pull/84995)
* [don't suggest unsized indirection in where-clauses](https://github.com/rust-lang/rust/pull/85979)
* [rustc: allow safe `#[target_feature]` on wasm](https://github.com/rust-lang/rust/pull/84988)
* [always go through the `expn_that_defined` query](https://github.com/rust-lang/rust/pull/86002)
* [perf: miscellaneous inlining improvements](https://github.com/rust-lang/rust/pull/85892)
* [perf: only compute the trait map once](https://github.com/rust-lang/rust/pull/85905)
* [stabilize `vecdeque_binary_search`](https://github.com/rust-lang/rust/pull/83362)
* [update standard library for `IntoIterator` implementation of arrays](https://github.com/rust-lang/rust/pull/85930)
* [clippy: don't warn about `cfg!(..)` as a constant in assertions](https://github.com/rust-lang/rust-clippy/pull/7319)
* [clippy: fix `needless_collect` with binding shadowing](https://github.com/rust-lang/rust-clippy/pull/7289)
* [clippy: add lint `manual_str_repeat`](https://github.com/rust-lang/rust-clippy/pull/7265)

### Rust Compiler Performance Triage

Busy week, with several reverted PRs due to performance regressions, but overall a positive week.

Triage done by **@simulacrum**.
Revision range: [cdbe288..1160cf8](https://perf.rust-lang.org/?start=cdbe2888979bb8797b05f0d58a6f6e60753983d2&end=1160cf864f2a0014e3442367e1b96496bfbeadf4&absolute=false&stat=instructions%3Au)

3 Regressions, 3 Improvements, 5 Mixed

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-06-01.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: 2021 Edition](https://github.com/rust-lang/rfcs/pull/3085)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: Supertrait item shadowing](https://github.com/rust-lang/rfcs/pull/2845)
* [disposition: merge] [Type-changing struct update syntax](https://github.com/rust-lang/rfcs/pull/2528)
* [disposition: merge] [RFC: Introduce concat_bytes!() to join [u8] and byte str analogous to concat! for str](https://github.com/rust-lang/rfcs/pull/2509)
* [disposition: merge] [RFC: Overconstraining and omitting unsafe in impls of unsafe trait methods](https://github.com/rust-lang/rfcs/pull/2316)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Re-add support for parsing (and pretty-printing) inner-attributes in match body](https://github.com/rust-lang/rust/pull/85193)

### New RFCs

* [Switch from travis to github actions.](https://github.com/rust-lang/rfcs/pull/3136)

## Upcoming Events

### Online

* [June 8, 2021, Seattle, WA, US - Monthly meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccjblb/)
* [June 10, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [June 16, 2021, Vancouver, BC, CA - Rust in Mozilla's Data Platform - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/fqpkjsyccjbvb/)
* [June 17, 2021, Denver, CO, US - Learning Rust as a Python/Javascript developer by Juhis - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/277575285/)

### North America

* [June 9, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/qxqdgryccjbmb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**NZXT**

* [Senior Software Engineer for CAM (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=259)
* [Senior Software Engineer for Streaming Software (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=317)

**Tweede golf**

* [Lead Developer Embedded Rust (Nijmegen, NL)](https://tweedegolf.nl/vacatures/2/lead-developer-embedded-rust)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> As the tradeoffs in software engineering change over time, so does the ideal solution. Some 40 years ago when the first C standards were written down, by people no less competent than those that work on Rust today, the design of the language and the list of behaviours not defined likely made much more sense in context of back then than they do right now. It is not all that unlikely that some years down the line the choices made by Rust won't make all that much of sense as they do today, too.

– [Simonas on rust-internals](https://users.rust-lang.org/t/why-deference-maybeuninit-unint-as-mut-ptr-is-safe/60344/19)

Thanks to [Kill The Mule](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1055) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
