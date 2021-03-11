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

### Official
* [Planning the Rust 2021 Edition](https://blog.rust-lang.org/inside-rust/2021/03/04/planning-rust-2021.html)

### Newsletters
* [This Month in Rust GameDev #19 - February 2021](https://rust-gamedev.github.io/posts/newsletter-019/)
* [This Month in Rust OSDev (February 2021)](https://rust-osdev.com/this-month/2021-02/)
* [RiB Newsletter #21 - More Rust and blockchains](https://www.reddit.com/r/rust/comments/lxdw8j/rib_newsletter_21_more_rust_and_blockchains/)

### Project/Tooling Updates
* [rust-analyzer Changelog #67](https://rust-analyzer.github.io/thisweek/2021/03/08/changelog-67.html)
* [rust-gpu v0.3](https://github.com/EmbarkStudios/rust-gpu/releases/tag/v0.3.0)
* [cargo-generate 0.6.0](https://github.com/cargo-generate/cargo-generate/releases/tag/v0.6.0)
* [Automatically generate APIs for datasets using Rust, Arrow and Datafusion](https://roapi.github.io/docs/index.html)

### Observations/Thoughts
* [Serverless Rust Testing](https://www.peakscale.com/serverless-rust/)
* [Rust's Type System is Turing-Complete](https://sdleffler.github.io/RustTypeSystemTuringComplete/)
* [Why I rewrote my Rust keyboard firmware in Zig: consistency, mastery, and fun](https://kevinlynagh.com/rust-zig/)
* [Calling a private Rust function from outside of its module](https://tim.mcnamara.nz/post/644942576528523264/calling-a-private-rust-function-from-outside)
* [Safe, Flexible Aliasing with Deferred Borrows](https://cfallin.org/pubs/ecoop2020_defborrow.pdf)
* [Evaluating Memory Models for Graph-Like Data Structures in the Rust Programming Language: Performance and Usability](http://www.diva-portal.org/smash/record.jsf?pid=diva2%3A1463648&dswid=1332)
* [Rust's async isn't f#@king colored!](https://www.hobofan.com/blog/2021-03-10-rust-async-colored/)
* [Rusts Universes](https://aloso.github.io/2021/03/10/rusts-universes.html)
* [How I significantly improved Giganotes app performance using Rust](https://www.reddit.com/r/rust/comments/m14agf/how_i_significantly_improved_giganotes_app/)

### Rust Walkthroughs
* [OS in Rust: Incorporate VGA buffer: Part-6](https://blog.knoldus.com/os-in-rust-incorporate-vga-buffer-part-6/)
* [Writing a 3D Shooter using rg3d - #1 - Character Controller](https://rg3d.rs/tutorials/2021/03/05/tutorial1.html)
* [Writing a 3D Shooter using rg3d - #2 - Weapons](https://rg3d.rs/tutorials/2021/03/09/tutorial2.html)
* [Writing Pong in Rust for my OS Written in Rust](https://blog.stephenmarz.com/2021/02/22/writing-pong-in-rust/)
* [Replacing FastAPI with Rust: Part 6 - AWS Lambda](https://dev.to/dbanty/replacing-fastapi-with-rust-part-6-aws-lambda-inm)
* [Creating an Iterator in Rust](https://aloso.github.io/2021/03/09/creating-an-iterator)
* [Building an OpenStreetMap app in Rust, Part V](https://blogg.bekk.no/building-an-openstreetmap-app-in-rust-part-v-f14831e13e61)
* [video] [Learning Rust: Structs and Traits](https://youtu.be/tYfA5rjrhqk)
* [video] [Answers to StackOverflow's top Rust programming questions explained](https://youtu.be/Flf4ezLWw1E)
* [video] [(Live Coding) Learning Timely Dataflow](https://youtu.be/z2m1Y4nj7s8)
* [FA] [video] [Rust in Farsi ツ](https://www.youtube.com/channel/UCIXThmX_uAp2cJTR-hTjdcA)

### Papers and Research Projects

* [Creusot is a tool for deductive verification of Rust code](https://github.com/xldenis/creusot)
* [egg, a Rust library for e-graphs and equality saturation](https://egraphs-good.github.io/)

### Miscellaneous
* [Debian running on Rust coreutils](https://sylvestre.ledru.info/blog/2021/03/09/debian-running-on-rust-coreutils)
* [A friend and I made pipes-rs, a Rust clone of pipes.sh](https://www.reddit.com/r/rust/comments/lxil6d/a_friend_and_i_made_pipesrs_a_rust_clone_of/)
* [Half of curl's vulnerabilities are C mistakes](https://daniel.haxx.se/blog/2021/03/09/half-of-curls-vulnerabilities-are-c-mistakes/)
* [video] [USENIX Enigma 2021 - Quantifying Memory Unsafety and Reactions to It](https://youtu.be/drfXNB6p6nI)

# Crate of the Week

This week's crate is [Sorceress](https://crates.io/crates/sorceress), a Rust environment for sound synthesis and algorithmic composition.

Thanks to [Zelda Hessler](https://users.rust-lang.org/t/crate-of-the-week/2704/887) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

[Our own "Papers and Research Projects" section needs filling!](https://github.com/rust-lang/this-week-in-rust/)

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

A generally positive albeit quiet week though many of the perf improvements were gaining performance back from previous regressions. We'll need to continue to keep an eye on rollups as there were two that caused small performance changes.

Triage done by **@rylev**.
Revision range: [edeee..86187](https://perf.rust-lang.org/?start=edeee915b1c52f97411e57ef6b1a8bd46548a37a&end=861872bc453bde79b83ff99d443d035225f10e87&absolute=false&stat=instructions%3Au)

1 Regression, 4 Improvements, 1 Mixed

2 of them in rollups

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
* [March 18, Manchester, UK - Rust Manchester Opening Night - Rust Manchester](https://www.meetup.com/rust-manchester/events/276567843/)
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

* [Rust software engineer (Remote, the Americas)](https://old.reddit.com/r/rust/comments/lhvipu/official_rrust_whos_hiring_thread_for_jobseekers/gn3p99j/)

**Launchbadge, LLC.**

* [Rust software engineer (Citrus Heights, CA, US)](https://www.ziprecruiter.com/jobs/launchbadge-5e5a2369/rust-software-engineer-72eb7f1b)

**Manta Network**

* [Backend/Distributed Systems Engineer (Rust) (Remote)](https://angel.co/company/manta-network/jobs/1182625-backend-distributed-systems-engineer-rust)

**Tweede golf**

* [Lead Developer Embedded Rust (Nijmegen, NL)](https://tweedegolf.nl/vacatures/2/lead-developer-embedded-rust)
* [Embedded Rust Engineer (Nijmegen, NL)](https://tweedegolf.nl/vacatures/11/medior-embedded-engineer)

**Parity**

* [Rust Performance Engineer (Remote)](https://www.parity.io/apply/?gh_jid=4385365003)
* [Rust P2P Network Engineer (Remote)](https://www.parity.io/apply/?gh_jid=4347843003)
* [and several other Rust Positions](https://www.parity.io/jobs/#jobs)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

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
