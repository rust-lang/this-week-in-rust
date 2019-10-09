Title: This Week in Rust 307
Number: 307
Date: 2019-10-08
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

[Coding nRF52 with Rust and Apache Mynewt on Visual Studio Code](https://medium.com/@ly.lee/coding-nrf52-with-rust-and-apache-mynewt-on-visual-studio-code-9521bcba6004?source=friends_link&sk=bb4e2523b922d0870259ab3fa696c7da).
- [Adventures in Motion Control: The Communications System Part 1](http://adventures.michaelfbryan.com/posts/comms-part-1/)

# Crate of the Week

This week's crate is [pin-project](https://crates.io/crates/pin-project), a proc-macro-derive for ergonomic and safe `Pin` projections.

Thanks to [Krishna Sannasi](https://users.rust-lang.org/t/crate-of-the-week/2704/636) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [vector: Update `stdin` source to use a thread instead of using the tokio version](https://github.com/timberio/vector/issues/932). Vector is a high-performance observability data router.
* [mundane: Build docs.rs documentation with all features enabled](https://github.com/google/mundane/issues/21). Mundane is a Rust cryptography library backed by BoringSSL.
* [rustfm-scrobble: Issues for Hacktoberfest](https://github.com/bobbo/rustfm-scrobble/issues?q=is%3Aissue+is%3Aopen+label%3AHacktoberfest). rustfm-scrobble is a Last.fm scrobble API library in Rust.
* [Tokamak is looking for maintainers](https://github.com/vertexclique/tokamak/issues/91). Tokamak is a Rust IDE for Atom.
* [Bastion is looking for maintainers](https://github.com/bastion-rs/bastion/issues/32). Bastion is a fault-tolerant runtime for Rust applications

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

278 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-09-23..2019-09-30

* [Rest In Peace, AST borrowck (2012-2019)](https://github.com/rust-lang/rust/pull/64790)
* [Fix double panic when printing query stack during an ICE](https://github.com/rust-lang/rust/pull/64799)
* [or-patterns: Push `PatKind/PatternKind::Or` at top level to HIR & HAIR](https://github.com/rust-lang/rust/pull/64508)
* [Fix format macro expansions spans to be macro-generated](https://github.com/rust-lang/rust/pull/64793)
* [Remove blanket silencing of "type annotation needed" errors](https://github.com/rust-lang/rust/pull/64746)
* [Include message on tests that should panic but do not](https://github.com/rust-lang/rust/pull/64745)
* [Point at definition when misusing ADT](https://github.com/rust-lang/rust/pull/64691)
* [Account for tail expressions when pointing at return type](https://github.com/rust-lang/rust/pull/64802)
* [On obligation errors point at the unfulfilled binding when possible](https://github.com/rust-lang/rust/pull/64151)
* [Fix coherence checking for impl trait in type aliases](https://github.com/rust-lang/rust/pull/63934)
* [Propagate `types.err` in locals further to avoid spurious knock-down errors](https://github.com/rust-lang/rust/pull/64674)
* [check_match: Improve diagnostics for `let A = 2;` with `const A: i32 = 3`](https://github.com/rust-lang/rust/pull/64859)
* [Point at enclosing match when expecting `()` in arm](https://github.com/rust-lang/rust/pull/64825)
* [Add a cycle detector for generic `Graph`s and `mir::Body`s](https://github.com/rust-lang/rust/pull/64622)
* [Add const-eval support for SIMD types, insert, and extract](https://github.com/rust-lang/rust/pull/64738)
* [Implement dataflow-based const validation](https://github.com/rust-lang/rust/pull/64470)
* [Optimize `try_eval_bits` to avoid layout queries](https://github.com/rust-lang/rust/pull/64673)
* [Even more `ObligationForest` improvements](https://github.com/rust-lang/rust/pull/64627)
* [A more explanatory thread local storage panic message](https://github.com/rust-lang/rust/pull/64481)
* [Stabilize `str::len`, `[T]::len` and `str::as_bytes` as const fn](https://github.com/rust-lang/rust/pull/63770)
* [Reserve `impl<T> From<!> for T`](https://github.com/rust-lang/rust/pull/62661)
* [Remove manual unrolling from `slice::Iter`(`Mut`)`::try_fold`](https://github.com/rust-lang/rust/pull/64600)
* [compiler-builtins: Implement bcmp](https://github.com/rust-lang/compiler-builtins/pull/315)
* [cargo: Improve test output with `--quiet`](https://github.com/rust-lang/cargo/pull/7446)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2523: `#[cfg(accessible(..) / version(..))]`](https://github.com/rust-lang/rfcs/pull/2523).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Minimum supported Rust version](https://github.com/rust-lang/rfcs/pull/2495).
* [disposition: close] [Project-based Examples for Cargo Projects](https://github.com/rust-lang/rfcs/pull/2517).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for RFC 2008: Future-proofing enums/structs with #[non_exhaustive] attribute](https://github.com/rust-lang/rust/issues/44109).
* [disposition: merge] [Stabilize `Option::as_deref` and `Option::as_deref_mut`](https://github.com/rust-lang/rust/pull/64708).
* [disposition: merge] [Stabilize mem::take (mem_take)](https://github.com/rust-lang/rust/pull/64716).
* [disposition: merge] [Stabilize `slice::repeat` (feature `repeat_generic_slice`)](https://github.com/rust-lang/rust/pull/64877).

## New RFCs

* [RFC 1201 ammendments: Naked function corrections](https://github.com/rust-lang/rfcs/pull/2774).

# Upcoming Events

### Asia Pacific

* [Oct  7. Auckland, NZ - Rust AKL - Introduction to Rust (session 2 of 3)](https://www.meetup.com/rust-akl/events/259481147/).

### Europe

* [Oct  4. Toulouse, FR - Toulouse Rust Meetup - Future<Output = Rust>](https://www.meetup.com/Toulouse-Rust-Meetup/events/264780064).
* [Oct  4. Darmstadt, DE - Hacktoberfest for Rustaceans](https://www.meetup.com/Rust-Rhein-Main/events/265052778).
* [Oct  5. Kharkiv, UA - PeerLab Kharkiv #Rust: AsyncIO](https://dou.ua/calendar/28904/).
* [Oct  9. Zagreb, HR - impl Zagreb for Rust: Rust, FFmpeg i TensorFlow](https://www.meetup.com/Zagreb-Rust-Meetup/events/265307360/).
* [Oct 10. Helsinki, FI - Finland Rust-lang Group - October meetup](https://www.meetup.com/Finland-Rust-Meetup/events/265091401/).
* [Oct 10. Warsaw, PL - Rust Warsaw - reboot](https://www.meetup.com/Rust-Warsaw/events/265091321/).
* [Oct 16. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryznbvb/).

### North America

* [Oct  5. Cleveland, OH, US - Cleveland RustBridge](https://coffee.dev/rustbridge).
* [Oct  8. Detroit, MI, US - Detroit Rust - Diving into Rust web frameworks](https://www.meetup.com/detroitrust/events/265090754/).
* [Oct  9. Atlanta, GA, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgryznbmb/).
* [Oct 10. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgryznbnb/).
* [Oct 10. Arlington, VA, US - Rust DC — Mid-month Rustful](https://www.meetup.com/RustDC/events/264768938).
* [Oct 16. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryznbvb/).
* [Oct 18 & 19. Dayton, OH, US - Rust Belt Rust](https://www.rust-belt-rust.com/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Official /r/rust "Who's Hiring" thread for job-seekers and job-offerers [Rust 1.38]](https://www.reddit.com/r/rust/comments/d9l79d/official_rrust_whos_hiring_thread_for_jobseekers/).
* [Software engineer (New grad) at Embark Studios, Stockholm, SE](https://www.embark-studios.com/jobs/278026-software-engineer-new-grad).
* [Multiple Rust jobs at Matter Labs (Berlin, Kiev, remote](https://medium.com/matter-labs/software-engineering-jobs-at-matter-labs-c456d01b2a02).
* [Rust internship at Tsuru Capital, Tokyo, JP](https://www.reddit.com/r/rust/comments/db7910/job_rust_internship_in_japan/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

Sadly, there were no nominations this week.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
