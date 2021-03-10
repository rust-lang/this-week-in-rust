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

This week's crate is [Sorceress](https://crates.io/crates/sorceress), a Rust environment for sound synthesis and algorithmic composition.

Thanks to [Zelda Hessler](https://users.rust-lang.org/t/crate-of-the-week/2704/887) for the suggestion!

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

369 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-03-01..2021-03-08

* [upgrade to LLVM 12](https://github.com/rust-lang/rust/pull/81451)
* [backport some LLVM compile-time improvements](https://github.com/rust-lang/rust/pull/82783)
* [add natvis for `Result`, `NonNull`, `CString`, `CStr`, and `Cow`](https://github.com/rust-lang/rust/pull/82557)
* [change error about unknown attributes to a warning](https://github.com/rust-lang/rust/pull/82702)
* [shrink the size of Rvalue by 16 bytes](https://github.com/rust-lang/rust/pull/82727)
* [move check only relevant in error case out of critical path](https://github.com/rust-lang/rust/pull/82738)
* [add `assert_matches!` macro](https://github.com/rust-lang/rust/pull/82770)
* [generalize `Write` impl for `Vec<u8>` to `Vec<u8, A>`](https://github.com/rust-lang/rust/pull/82862)
* [avoid unnecessary `Vec` construction in `BufReader`](https://github.com/rust-lang/rust/pull/82728)
* [improve `slice.binary_search_by()`'s best-case performance to O(1)](https://github.com/rust-lang/rust/pull/74024)
* [add {`BTreeMap`, `HashMap`}`::try_insert`](https://github.com/rust-lang/rust/pull/82764)
* [hashbrown: add `try_insert`](https://github.com/rust-lang/hashbrown/pull/247)
* [cargo: fix `filter_platform` to run on targets other than x86](https://github.com/rust-lang/cargo/pull/9246)
* [make rustdoc lints a tool lint instead of built-in](https://github.com/rust-lang/rust/pull/80527)

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

**Launchbadge, LLC.**
* [Rust software engineer (Citrus Heights, CA, US)](https://www.ziprecruiter.com/jobs/launchbadge-5e5a2369/rust-software-engineer-72eb7f1b)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*


**Parity**

* [Rust Performance Engineer (Remote)](https://www.parity.io/apply/?gh_jid=4385365003)
* [Rust P2P Network Engineer (Remote)](https://www.parity.io/apply/?gh_jid=4347843003)
* [and several other Rust Positions](https://www.parity.io/jobs/#jobs)

# Quote of the Week

> it's funny, every time I run into a baffling borrow error, it's preventing me from committing a real, serious mistake
>
> but it can take some thinking to figure out what exactly that mistake is..
>
> sometimes the borrow checker feels like a wise sage on a mountain giving advice in riddles lol

– [Jarrett on discord](https://discord.com/channels/442252698964721669/443150878111694848/817890654779605009)

Thanks to [Daniel H-M](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1012) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
