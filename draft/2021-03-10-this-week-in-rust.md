Title: This Week in Rust 381
Number: 381
Date: 2021-03-10
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

No newsletters this week.

### Official

### Project/Tooling Updates

### Observations/Thoughts

* [Serverless Rust Testing](https://www.peakscale.com/serverless-rust/)

### Rust Walkthroughs
* [OS in Rust: Incorporate VGA buffer: Part-6](https://blog.knoldus.com/os-in-rust-incorporate-vga-buffer-part-6/)

### Miscellaneous

# Crate of the Week

This week's crate is [camino](https://crates.io/crates/camino), a library with UTF-8 coded paths mimicking `std::os::Path`'s API.

Thanks to [piegames](https://users.rust-lang.org/t/crate-of-the-week/2704/886) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

*No calls for participation this week*

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

402 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-02-22..2021-03-01

* [implement -Z hir-stats for nested foreign items](https://github.com/rust-lang/rust/pull/82258)
* [suggest character encoding is incorrect when encountering random null bytes](https://github.com/rust-lang/rust/pull/81856)
* [suggest `return`ing tail expressions that match return type](https://github.com/rust-lang/rust/pull/81769)
* [improve suggestion for tuple struct pattern matching errors](https://github.com/rust-lang/rust/pull/81235)
* [improve error message when found type is deref of expected](https://github.com/rust-lang/rust/pull/82364)
* [AST: remove some unnecessary boxes](https://github.com/rust-lang/rust/pull/82321)
* [apply lint restrictions from renamed lints](https://github.com/rust-lang/rust/pull/82620)
* [remove storage markers if they won't be used during code generation](https://github.com/rust-lang/rust/pull/78360)
* [remove many `RefCell`s from `DocContext`](https://github.com/rust-lang/rust/pull/82305)
* [prevent computing Item attributes twice](https://github.com/rust-lang/rust/pull/82265)
* [new mir-opt pass to simplify gotos with const values](https://github.com/rust-lang/rust/pull/80475)
* [add an impl of `Error` on `Arc<impl Error>`](https://github.com/rust-lang/rust/pull/80553)
* [make `ptr::write` const](https://github.com/rust-lang/rust/pull/81167)
* [make `char` and `u8` methods const](https://github.com/rust-lang/rust/pull/82078)
* [slight perf improvement on `char::to_ascii_lowercase`](https://github.com/rust-lang/rust/pull/81837)
* [stabilize `str_split_once`](https://github.com/rust-lang/rust/pull/81940)
* [specialize `slice::fill` with `Copy` type and `u8`/`i8`/`bool`](https://github.com/rust-lang/rust/pull/81874)
* [futures: `future::SelectAll::into_inner`](https://github.com/rust-lang/futures-rs/pull/2363)
* [futures: `futures_util::stream::SelectAll::push` should use `&self`](https://github.com/rust-lang/futures-rs/pull/2293)
* [cargo: run rustdoc doctests relative to the workspace](https://github.com/rust-lang/cargo/pull/9105)
* [cargo: throw error if `CARGO_TARGET_DIR` is an empty string](https://github.com/rust-lang/cargo/pull/8939)
* [cargo: add support for `[env]` section in .cargo/config.toml](https://github.com/rust-lang/cargo/pull/9175)
* [cargo: make it more clear which module is being tested when running cargo test](https://github.com/rust-lang/cargo/pull/9195)

## Rust Compiler Performance Triage

Quiet week, a couple regressions and several nice improvements.

Triage done by **@simulacrum**.
Revision range: [301ad8..edeee](https://perf.rust-lang.org/?start=301ad8a4fa3ea56fb980443b7997c8f9d72dd717&end=edeee915b1c52f97411e57ef6b1a8bd46548a37a&absolute=false&stat=instructions%3Au)

2 Regressions, 3 Improvements, 0 Mixed

0 of them in rollups

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)


* [RFC - cargo templates](https://github.com/rust-lang/rfcs/pull/2922)
* [rfc: make cargo install extensible](https://github.com/rust-lang/rfcs/pull/2376)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Deprecate `doc(include)`](https://github.com/rust-lang/rust/pull/82539)
* [disposition: merge] [Implement Extend and FromIterator for OsString](https://github.com/rust-lang/rust/pull/82121)
* [disposition: merge] [Allow specifying alignment for functions](https://github.com/rust-lang/rust/pull/81234)
* [disposition: close] [resolve: allow super in module in block to refer to block items](https://github.com/rust-lang/rust/pull/79309)

## New RFCs

* [A new prelude for the 2021 edition](https://github.com/rust-lang/rfcs/pull/3090)

# Upcoming Events

### Online
* [March 4, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprryccfbgb/)
* [March 9, Saarbücken, Saarland, DE - Meetup: 9u16 (virtual) - Rust Saar](https://www.meetup.com/de-DE/Rust-Saar/events/276401469/)
* [March 9, Buffalo, NY, US - Buffalo Rust User Group - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/276717842/)
* [March 9, Seattle, WA, US - Monthly meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccfbmb/)
* [March 10, New York, NY, US - Seemingly Dark Magic with Rust Types with Nikolai Vazquez - Rust NYC](https://www.meetup.com/Rust-NYC/events/276666844/)
* [March 11, Columbus, OH, US - Monthly Meeting - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgryccfbpb/)
* [March 13th, DE - Chemnitzer Linux Tage - Talk on Rust and its ecosystem](https://chemnitzer.linux-tage.de/2021/en/programm/beitrag/135)
* [March 16, Washington, DC, US - Rust and Tell Lightning Talks - Rust DC](https://www.meetup.com/RustDC/events/kcfpzryccfbpb/)
* [March 17, Vancouver, BC, US - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/npqfbsyccfbwb/)
* [March 25. Barcelona, ES - BcnRust Meetup](https://www.meetup.com/es-ES/BcnRust/events/276796209/).

### North America
* [March 10, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/qxqdgryccfbnb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**e.ventures**
* [Rust software engineer (Remote, the Americas)](https://old.reddit.com/r/rust/comments/lhvipu/official_rrust_whos_hiring_thread_for_jobseekers/)

**Launchbadge, LLC.**
* [Rust software engineer (Citrus Heights, CA, US)](https://www.ziprecruiter.com/jobs/launchbadge-5e5a2369/rust-software-engineer-72eb7f1b)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*


**Parity**

* [Rust Performance Engineer (Remote)](https://www.parity.io/apply/?gh_jid=4385365003)
* [Rust P2P Network Engineer (Remote)](https://www.parity.io/apply/?gh_jid=4347843003)
* [and several other Rust Positions](https://www.parity.io/jobs/#jobs)

# Quote of the Week

> It's a great example of the different attitudes of C/C++ and Rust: In C/C++ something is correct when someone can use it correctly, but in Rust something is correct when someone can't use it incorrectly.

– [/u/Janohard on /r/rust](https://www.reddit.com/r/rust/comments/lt4u85/const_generics_mvp_hits_beta/goyg3v4/)

Thanks to [Vlad Frolov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1007) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
