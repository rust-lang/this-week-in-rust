Title: This Week in Rust 100
Number: 100
Date: 2015-10-12
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/wiki/Note-guide-for-new-contributors).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

This week's edition was edited by: [nasa42](https://github.com/nasa42), [brson](https://github.com/brson), and [llogiq](https://github.com/llogiq).


# Updates from Rust Community

## News & Blog Posts

* [Safety = Freedom](https://llogiq.github.io/2015/10/05/safety-freedom.html).
* [This week in Redox 1](http://www.redox-os.org/news/this-week-in-redox-1/). Redox is an upcoming OS written in Rust.
* [This week in Servo 36](http://blog.servo.org/2015/10/05/twis-36/).
* [Conference report: Friendly, diverse Rust Camp](http://techwhirl.com/conference-report-friendly-diverse-rust-camp/).
* [Neither necessary nor sufficient](https://ruudvanasseldonk.com/2015/10/06/neither-necessary-nor-sufficient) - why Rust shipped without GC.
* [Rustfmt-ing Rust](http://www.ncameron.org/blog/rustfmt-ing-rust/).
* [Virtual structs part 4: Extended enums and thin traits](http://smallcultfollowing.com/babysteps/blog/2015/10/08/virtual-structs-part-4-extended-enums-and-thin-traits/).
* [Macros](http://www.ncameron.org/blog/macros/). Plans on an overhaul of the syntax extension and macro systems in Rust.
* [Lints that collect data per crate](https://llogiq.github.io/2015/10/09/lint-data.html).
* [Stuff the identity function does (in Rust)](https://bluss.github.io/rust/fun/2015/10/11/stuff-the-identity-function-does/).
* [Rust in detail: Writing scalable chat service from scratch](https://nbaksalyar.github.io/2015/07/10/writing-chat-in-rust.html).
* [Formalizing Rust](https://www.ralfj.de/blog/2015/10/12/formalizing-rust.html).

## Notable New Crates & Projects

* [Plumbum](https://github.com/srijs/rust-plumbum). Conduit-like data processing library for Rust.
* [rust-keepass](https://github.com/raymontag/rust-keepass). Crate to use KeePass databases in Rust.
* [Rusoto](https://github.com/DualSpark/rusoto). AWS client libraries for Rust.
* [fired](http://www.redox-os.org/blog/fired-fast-init-system/). Fast init system for Redox OS.
* [Rust Release Explorer](https://ashleygwilliams.github.io/rust-release-explorer/).
* [cargo-edit](https://github.com/killercup/cargo-edit). A utility for adding cargo dependencies from the command line.
* [Rust MetroHash](https://github.com/arthurprs/metrohash-rs). Rust implementation of MetroHash - a high quality, high performance hash algorithm.
* [rose_tree](https://github.com/mitchmindtree/rose_tree-rs). An implementation of the rose tree data structure for Rust.
* [daggy](https://github.com/mitchmindtree/daggy). A directed acyclic graph data structure for Rust.
* [hdfs-rs](https://github.com/hyunsik/hdfs-rs). libhdfs binding and wrapper APIs for Rust.
* [cargo.el](https://github.com/attichacker/cargo.el). Emacs minor mode for Cargo.

# Updates from Rust Core

102 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-10-05..2015-10-12

## Notable changes

* [Turn on MIR construction unconditionally](https://github.com/rust-lang/rust/pull/28748).
* [Make function pointers implement traits for up to 12 parameters](https://github.com/rust-lang/rust/pull/28560).
* [Implement `Read`, `BufRead`, `Write` and `Seek` for `Cursor<Box<[u8]>>`](https://github.com/rust-lang/rust/pull/27897).
* [Shrink metadata size](https://github.com/rust-lang/rust/pull/28521). libcore.rlib reduced from 19121 kiB to 15934 kiB - 20% win.
* [Implement RFC 1238: nonparametric dropck](https://github.com/rust-lang/rust/pull/28861).
* [Integer parsing should accept leading plus](https://github.com/rust-lang/rust/pull/28826).
* [Add `AsRef`/`AsMut` impls to `Box`/`Rc`/`Arc`](https://github.com/rust-lang/rust/pull/28811).
* [rustc: Don't lint about isize/usize in FFI](https://github.com/rust-lang/rust/pull/28779).
* [rustc: Improve the dep-info experience](https://github.com/rust-lang/rust/pull/28768).
* [Cargo: Fix `make` and `make doc` on Mac OS X 10.11 El Capitan](https://github.com/rust-lang/cargo/pull/1903).
* [Book: Add documentation on custom allocators](https://github.com/rust-lang/rust/pull/28869).
* [Book: New "Syntax Index" chapter](https://github.com/rust-lang/rust/pull/28926).
* [rust.vim: Add rudimentary support for the playpen](https://github.com/rust-lang/rust.vim/pull/16).
* [sublime-rust: Major changes](https://github.com/rust-lang/sublime-rust/pull/65). (PR has a list of all changes).

## New Contributors

* Carlos Liam
* Chris C Cerami
* Craig Hills
* Cristi Cobzarenco
* Daniel Carral
* Daniel Keep
* glendc
* Joseph Caudle
* J. Ryan Stinnett
* Kyle Robinson Young
* panicbit
* Yoshito Komatsu

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [1228: Place left arrow syntax (`place <- expr`)](https://github.com/rust-lang/rfcs/pull/1228).
* [1260: Allow a re-export for `main`](https://github.com/rust-lang/rfcs/pull/1260).
* [Amend #911 const-fn to allow unsafe const functions](https://github.com/rust-lang/rfcs/pull/1245).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen

* [Allow overlapping implementations for marker traits](https://github.com/rust-lang/rfcs/pull/1268).
* [Ordered ranges 2.0](https://github.com/rust-lang/rfcs/pull/1254). Replace `range(Bound::Excluded(&min), Bound::Included(&max))` with `range().ge(&min).lt(&max)`.
* [Stabilize OS string to bytes conversions](https://github.com/rust-lang/rfcs/pull/1255).
* [Implement `.drain(range)` and `.drain()` respectively as appropriate on collections.](https://github.com/rust-lang/rfcs/pull/1257).

## New RFCs

* [Add a string-like API to the `OsString` and `OsStr` types](https://github.com/rust-lang/rfcs/pull/1309).

# Upcoming Events

* [10/13. San Diego Rust Meetup #9](http://www.meetup.com/San-Diego-Rust/events/225389095/).
* 10/14. RustBerlin Hack and Learn.
* [10/19. Rust Paris](http://www.meetup.com/Rust-Paris).
* [10/20. Rust Hack and Learn Hamburg](http://www.meetup.com/Rust-Meetup-Hamburg/events/225865482/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*No jobs listed for this week. Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate of the Week

The Crate of this week is [Conrod](https://crates.io/crates/conrod), a simple to use intermediate-mode GUI library written in pure Rust. Thanks to [7zf](https://www.reddit.com/user/7zf).

Conrod forgos the traditional "wrap the default GUI for each operating system/desktop environment" approach and uses OpenGL to present a simple, but color- as well as powerful GUI in its own style. There are a good number of widgets implemented already, and writing GUI code with conrod is a breeze thanks to the very streamlined interface.

# Quote of the Week

>> War is Unsafe
>>
>> Freedom is Safety
>>
>> Ignorance is Type-checked
>
> (The new trifecta for rust-lang.org)

— [killercup on /r/rust](https://www.reddit.com/r/programming/comments/3nom7j/safety_freedom/cvqdt1z).

Thanks to [ruudva](https://users.rust-lang.org/users/ruudva) for the tip. [Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
