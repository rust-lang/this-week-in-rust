Title: This Week in Rust 306
Number: 306
Date: 2019-10-01
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

- [Adventures in Motion Control: FPS Counter](http://adventures.michaelfbryan.com/posts/fps-counter/)
- [gfx-rs 2019 update](https://gfx-rs.github.io/2019/10/01/update.html)

# Crate of the Week

This week's crate is [pin-project](https://crates.io/crates/pin-project), a proc-macro-derive for ergonomic and safe `Pin` projections.

Thanks to [Krishna Sannasi](https://users.rust-lang.org/t/crate-of-the-week/2704/636) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

*No issues were proposed for CfP*.

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

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: postpone] [Existential types with external definition](https://github.com/rust-lang/rfcs/pull/2492).
* [disposition: postpone] [Custom self types](https://github.com/rust-lang/rfcs/pull/2362).
* [disposition: postpone] [Cargo versioning](https://github.com/rust-lang/rfcs/pull/2182).
* [disposition: close] [Project-based Examples for Cargo Projects](https://github.com/rust-lang/rfcs/pull/2517).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize macros in some more positions](https://github.com/rust-lang/rust/pull/63931).
* [disposition: merge] [Stabilize `Option::as_deref` and `Option::as_deref_mut`](https://github.com/rust-lang/rust/pull/64708).
* [disposition: merge] [Stabilize mem::take (mem_take)](https://github.com/rust-lang/rust/pull/64716).
* [disposition: merge] [Tracking issue for RFC 2008: Future-proofing enums/structs with #[non_exhaustive] attribute](https://github.com/rust-lang/rust/issues/44109).
* [disposition: merge] [Support repr(simd) on ADTs containing a single array field](https://github.com/rust-lang/rust/pull/63531).
* [disposition: merge] [syntax: Support modern attribute syntax in the `meta` matcher](https://github.com/rust-lang/rust/pull/63674).
* [disposition: merge] [convert `\r\n` -> `\n` in include_str! macro](https://github.com/rust-lang/rust/pull/63681).
* [disposition: close] [Regression: : cannot determine resolution for the attribute macro `test`](https://github.com/rust-lang/rust/issues/56375).
* [disposition: close] [Expose Linux syscall interface](https://github.com/rust-lang/rust/pull/63745).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Africa

* [Oct  2. Johannesburg, ZA - Johannesburg Rust Meetup - futures (part 2)](https://www.meetup.com/Johannesburg-Rust-Meetup/events/dgqmbryznbdb/).

### Asia Pacific

* [Sep 28. Taipei, TW - Rust Taiwan Meetup](https://www.facebook.com/events/2110177005945081/).
* [Oct  7. Auckland, NZ - Rust AKL - Introduction to Rust (session 2 of 3)](https://www.meetup.com/rust-akl/events/259481147/).

### Europe

* [Sep 26. Turin, IT - Mozilla Torino - Gruppo di studio Rust](https://www.meetup.com/Mozilla-Torino/events/264748662).
* [Sep 26. London, GB - Rust London User Group - Hack 'n Learn September 2019](https://www.meetup.com/it-IT/Rust-London-User-Group/events/264999149/).
* [Oct  1. Göteborg, SE - Rust Gbg — Golden October Rust 2019](https://www.meetup.com/rustgbg/events/264957575/).
* [Oct  2. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryznbdb/).
* [Oct  2. Cologne, DE - Rust Cologne - Open Space](https://www.meetup.com/RustCologne/events/tnrnbryznbdb/).
* [Oct  3. Toulouse, FR - Rust talks at DevFest Toulouse 2019](https://devfesttoulouse.fr/).
* [Oct  4. Toulouse, FR - Toulouse Rust Meetup - Future<Output = Rust>](https://www.meetup.com/Toulouse-Rust-Meetup/events/264780064).
* [Oct  4. Darmstadt, DE - Hacktoberfest for Rustaceans](https://www.meetup.com/Rust-Rhein-Main/events/265052778).
* [Oct  5. Kharkiv, UA - PeerLab Kharkiv #Rust: AsyncIO](https://dou.ua/calendar/28904/).
* [Oct  9. Zagreb, HR - impl Zagreb for Rust: Rust, FFmpeg i TensorFlow](https://www.meetup.com/Zagreb-Rust-Meetup/events/265307360/).
* [Oct 10. Helsinki, FI - Finland Rust-lang Group - October meetup](https://www.meetup.com/Finland-Rust-Meetup/events/265091401/).
* [Oct 10. Warsaw, PL - Rust Warsaw - reboot](https://www.meetup.com/Rust-Warsaw/events/265091321/).

### North America

* [Sep 26. New York, NY - Local Native: A Decentralized Cross-platform App Developed with Rust](https://www.meetup.com/Rust-NYC/events/264849068/).
* [Oct  1. Toronto, ON, CA - Rust Toronto - Rust for the Web](https://www.meetup.com/Rust-Toronto/events/264727074/).
* [Oct  2. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyznbdb/).
* [Oct  2. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryznbdb/).
* [Oct  2. Boston, MA, US - Boston Rust Meetup at VMware](https://www.meetup.com/BostonRust/events/264555065/).
* [Oct  5. Cleveland, OH, US - Cleveland RustBridge](https://coffee.dev/rustbridge).
* [Oct  8. Detroit, MI, US - Detroit Rust - Diving into Rust web frameworks](https://www.meetup.com/detroitrust/events/265090754/).
* [Oct  9. Atlanta, GA, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgryznbmb/).
* [Oct 10. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgryznbnb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [2D Graphics Programmer at Dungeonfog, Vienna, AT (Remote available)](https://www.dungeonfog.com/about/job-offers/).
* [Principal Software Engineer at Microsoft, Redmond, WA, US](https://twitter.com/ryan_levick/status/1171830191804551168).
* [Multiple Rust positions at Parity, Berlin, DE (Remote available)](https://www.parity.io/jobs/).
* [Software Engineer at 3DSignals, Kfar Saba, IL](https://3dsig.com/positions/software-engineer/).
* [Senior Engineer at Ditto at San Francisco, US (Remote available)](https://twitter.com/Adam_Fish/status/1173672751271268352).
* [Multiple Rust jobs at Matter Labs (Berlin, Kiev, remote](https://medium.com/matter-labs/software-engineering-jobs-at-matter-labs-c456d01b2a02)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

Sadly, there were no nominations this week.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
