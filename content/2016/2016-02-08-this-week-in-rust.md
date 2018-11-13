Title: This Week in Rust 117
Number: 117
Date: 2016-02-08
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

This week's edition was edited by: [Vikrant](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).


# Updates from Rust Community

## News & Blog Posts

* [Everything you need to know about cross compiling Rust programs](https://github.com/japaric/rust-cross).
* [Deep learning in Rust: Baby steps](https://medium.com/@tedsta/deep-learning-in-rust-7e228107cccc).
* [BigData & Rust, part 3](http://www.poumeyrol.fr/2016/02/01/Lets-optimize/). A comparison of data serialisation formats.
* [Rust's memory management for C/C++ programmers](http://blog.zgtm.de/1).
* [How Stateful cheats at analysis](https://erickt.github.io/blog/2016/01/28/stateful/). Stateful is a Rust experimental syntax extension for generators and more.
* [This week in Servo 49](http://blog.servo.org/2016/02/01/twis-49/).
* [This Week in Amethyst 3](https://thisweekinamethyst.wordpress.com/2016/02/01/twia-3/). Amethyst is a data-oriented game engine written in Rust.

## Notable New Crates & Project Updates

* [PSA: If you used these particular cargo nightlies, you need to reupload published crates](https://github.com/rust-lang/cargo/issues/2326).
* [Diesel has added support for SQLite3](https://github.com/sgrif/diesel/commit/44e40bc90a769d1539ff40afc034b514a3df5e75).
* [RustType](https://github.com/dylanede/rusttype). A pure Rust alternative to libraries like FreeType.
* [crates.fyi](https://github.com/onur/cratesfyi) - Documentation generator for crates released in crates.io.
* [Pippin](https://github.com/dhardy/pippin). A distributed-merge capable database for many small objects.
* [Pijul](https://pijul.org/). Distributed version control system written in Rust.
* [Zipper](https://github.com/forticulous/zipper). ZIP file parser written in Rust.

# Updates from Rust Core

121 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-02-01..2016-02-08

## Notable changes

* [Don't make atomic loads and stores volatile](https://github.com/rust-lang/rust/pull/30962).
* [Emscripten support take 2 (with bonus i686-unknown-linux-musl)](https://github.com/rust-lang/rust/pull/30629).
* [Remove MutexGuard::map, as it is not safe in combination with Condvar](https://github.com/rust-lang/rust/pull/31428).
* [Avoid quadratic growth of functions due to cleanups](https://github.com/rust-lang/rust/pull/31390).
* [Add `Cow::from` for `Vec` and slices](https://github.com/rust-lang/rust/pull/31386).
* [Abort on stack overflow instead of re-raising SIGSEGV](https://github.com/rust-lang/rust/pull/31333).
* [Fix the armv7 linux target](https://github.com/rust-lang/rust/pull/31331).
* [Allow `#[repr(i32)]` for univariant enum](https://github.com/rust-lang/rust/pull/31232).
* [Add Illumos support](https://github.com/rust-lang/rust/pull/31078).

## New Contributors

* Alexander Lopatin
* Brandon W Maister
* Nikita Baksalyar
* Paul Smith
* Prayag Verma
* qpid
* Reeze Xia
* Ryan Thomas
* Sandeep Datta
* Sean Leffler

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 243: Trait-based exception handling](https://github.com/rust-lang/rfcs/pull/243).
* [RFC 1445: Restrict constants in patterns](https://github.com/rust-lang/rfcs/pull/1445).
* [RFC 1359: Add `CommandExt::{exec, before_exec}`](https://github.com/rust-lang/rfcs/pull/1359).
* [Amendment to RFC 1270: Describe actual implementation](https://github.com/rust-lang/rfcs/pull/1423).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Deprecate type aliases in `std::os::*::raw`](https://github.com/rust-lang/rfcs/pull/1415).
* [Add `retain_mut` to `Vec` and `VecDeque`](https://github.com/rust-lang/rfcs/pull/1353).
* [Rust Language Server (IDE support)](https://github.com/rust-lang/rfcs/pull/1317).
* [Add a `SharedSender` to `std::sync::mpsc` that implements `Sync`](https://github.com/rust-lang/rfcs/pull/1299).
* [Propose a design for _specialization_, which permits multiple `impl` blocks to apply to the same type/trait](https://github.com/rust-lang/rfcs/pull/1210).
* [Let specified generic type parameter lists be abbreviated](https://github.com/rust-lang/rfcs/pull/1196).
* [Add a `IndexAssign` trait that allows overloading "indexed assignment" expressions like `a[b] = c`](https://github.com/rust-lang/rfcs/pull/1129).

## New RFCs

* [Add a new `kind=better_static` that is used to link static libraries by passing them to the linker](https://github.com/rust-lang/rfcs/pull/1489).
* [Permit the `..` pattern fragment in more contexts](https://github.com/rust-lang/rfcs/pull/1492).

# Upcoming Events

* [2/10. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [2/10. Rust Boulder/Denver Monthly Meeting](http://www.meetup.com/Rust-Boulder-Denver/).
* [2/11. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [2/12. Embedded Rust Workshop Frankfurt](http://www.meetup.com/de-DE/Rust-Rhein-Main/events/228170051/).
* [2/15. Rust Paris](http://www.meetup.com/Rust-Paris).
* [2/15. Rust Sydney Meetup](http://www.meetup.com/Rust-Sydney/events/228043858/).
* [2/16. San Diego Rust: Eat– Drink– Rust! Downtown Rust Meetup](http://www.meetup.com/San-Diego-Rust/events/228573576/).
* [2/17. Rust Los Angeles Monthly Meetup](http://www.meetup.com/Rust-Los-Angeles/events/228104697/).
* [2/17. Rust Berlin: Leaf and Collenchyma](http://www.meetup.com/Rust-Berlin/events/227321071/).
* [2/18. Rust Hack and Learn Hamburg @ 4=1](http://www.meetup.com/Rust-Meetup-Hamburg/events/228502426/?rv=ea1&_af=event&_af_eid=228502426&https=off).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Research Engineer - Servo](https://careers.mozilla.org/en-US/position/ozy21fwU) at Mozilla.
* [Senior Research Engineer - Rust](https://careers.mozilla.org/en-US/position/o0H41fww) at Mozilla.
* [PhD and postdoc positions](http://plv.mpi-sws.org/rustbelt/) at MPI-SWS.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate of the Week

The week's Crate of the Week is [roaring](https://crates.io/crates/roaring), the Rust
version of Prof. D. Lemire's compressed bitmap data structure. I can personally
attest that both the Rust and Java versions compare very favorably in both
speed and size to other bit sets and are easy to use.

Thanks to [polyfractal](https://users.rust-lang.org/users/polyfractal) for the
suggestion.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704
