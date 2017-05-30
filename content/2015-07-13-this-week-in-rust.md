Title: This Week in Rust 87
Date: 2015-07-13
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

This week's edition was edited by: Brian Anderson, Vikrant Chaudhary

# From the Blogosphere

* [Rust in Detail: Writing Scalable Chat Service from Scratch](https://nbaksalyar.github.io/2015/07/10/writing-chat-in-rust.html).
* [Reading Rust Function Signatures](http://hoverbear.org/2015/07/10/reading-rust-function-signatures/). How to read function signatures and extract information from them.
* [Holy std::borrow::Cow](https://llogiq.github.io/2015/07/09/cow.html), also [Redux](https://llogiq.github.io/2015/07/09/cow.html): Llogiq investigates Cow usage and implementation.
* [Collecting Results from Collections](http://hoverbear.org/2015/07/08/a-useful-error-pattern/). Use `.collect()` to transform `Vec<Result<()>>` into `Result<Vec<()>>`.
* [Easier libc in Rust](https://mobiarch.wordpress.com/2015/07/03/easy-libc-in-rust/). An index of some of the most commonly used libc calls and their higher level wrappers.
* [Importing C constants: Proof of Concept](http://vojtech.kral.hk/en/rust-importing-c-constants-proof-of-concept/). A proof-of-concept rustc plugin that imports C macro constants from C include files at compile time.
* [A Pythonist getting Rusty these days... (Part 2)](https://wafflespeanut.github.io/blog/2015/07/08/a-pythonist-getting-rusty-these-days-dot-dot-dot-part-2/). Rust from a Python developer's perspective (part 2).
* [Converting longitude and latitude coordinates into BNG coordinates](http://sensitivecities.com/rust-python-ffi-bng-EN.html).
* [ArcadeRS - making a simple game in Rust](https://jadpole.github.io/2015/185/arcaders-1-0/).
* [opinion] [Why Go and Rust are Competitors](http://www.doxsey.net/blog/why-go-and-rust-are-competitors/).

# New Releases & Project Updates

* [stdx](https://github.com/brson/stdx). Curated collection of well-regarded Rust crates.
* [ipc-channel](https://github.com/pcwalton/ipc-channel). A multiprocess drop-in replacement for Rust channels.
* [rocket](https://github.com/aochagavia/rocket).  A toy game in Rust, using Piston.
* [forkjoin](https://github.com/faern/forkjoin). A work stealing fork-join parallelism library for Rust.
* [capsize](https://github.com/softprops/capsize). Conversions between units of capacity.
* [porthole](https://github.com/softprops/porthole). A tiny rust crate for resolving the next available network port.

# What's cooking on nightly?

122 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-07-06..2015-07-13

* [bluss](https://github.com/bluss) updated [substring search to use the Two Way algorithm](https://github.com/rust-lang/rust/pull/26327). 
* [1.2 beta will issue warnings about code that will break](https://github.com/rust-lang/rust/pull/26829) when [RFC 1156](https://github.com/rust-lang/rfcs/blob/master/text/1156-adjust-default-object-bounds.md) is implemented. This approved breaking change will fix a major wart and is thought to break no real code.
* [Linux installation will try harder to set up the dynamic linker](https://github.com/rust-lang/rust-installer/pull/41), fixing a bad first-run issue where Fedora systems can't run rustc out of the box.
* The `#[prelude_import]` attribute, which is employed by rustc to perform [dark](https://github.com/rust-lang/rust/blob/6a3b385cbd6b9044b4447da96aad066e8b257ddf/src/libsyntax/std_inject.rs#L164) and [mysterious](https://github.com/rust-lang/rust/blob/6a3b385cbd6b9044b4447da96aad066e8b257ddf/src/librustc_resolve/build_reduced_graph.rs#L292-L294) acts, but is not supposed to be stable. This is not known to break real code.
* `rustc` on Windows now [looks in the registry](https://github.com/rust-lang/rust/pull/26741) to find the location of the MSVC linker.
* Inspired by some [poor I/O performance on the forums](https://users.rust-lang.org/t/reading-from-stdin-performance/2025), bluss dug into the problem [and pulled out some big improvements in zero-filling](https://github.com/rust-lang/rust/pull/26849) that greatly improve the performance of `Vec::resize` and `Read::read_to_end`.
* GuillaumeGomez [added a host](https://github.com/rust-lang/rust/pull/26742) of [new error explanations](https://github.com/rust-lang/rust/pull/26879).
* dotdash got some [huge improvements in the performance of `PartialEq` for slices](https://github.com/rust-lang/rust/pull/26884).
* `rustc` now uses [LLVM to write archive files where possible](https://github.com/rust-lang/rust/pull/26926). Eventually this will eliminate the compiler's dependency on the `ar` utility.
* [Add `String::into_boxed_slice` and `Box<str>::into_string`](https://github.com/rust-lang/rust/pull/26931).
* Dave Huseby added [initial support for FreeBSD on i686](https://github.com/rust-lang/rust/pull/26959). Rust has long worked on 64-bit FreeBSD.
* Simon [updated the gedit syntax highlighter](https://github.com/rust-lang/gedit-config/pull/8).

# New Contributors

* Alex HotShot Newman
* Christian Weinz
* Esption
* Georg Brandl
* Jesús Espino
* jethrogb

# Approved RFCs

* [RFC 1058: Replace `slice.tail()`, `slice.init()` with new methods `slice.split_first()`, `slice.split_last()`](https://github.com/rust-lang/rfcs/blob/master/text/1058-slice-tail-redesign.md).
* [RFC 1102: Rename `connect` to `join`](https://github.com/rust-lang/rfcs/blob/master/text/1102-rename-connect-to-join.md).

# Final Comment Period

Every week the teams announce a 'final comment period' for RFCs and
key PRs which are reaching a decision. Express your opinions
now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen+updated%3A2015-07-06..2015-07-13

* [Add a method `lines_any` to `BufRead`](https://github.com/rust-lang/rust/pull/26743)
* [RFC for creation of `IntoRaw{Fd, Socket, Handle}` trait to complement `AsRaw*`](https://github.com/rust-lang/rfcs/pull/1174)
* [RFC: Expand the std::net module](https://github.com/rust-lang/rfcs/pull/1158)
* [RFC: Allow re-exporting associated items with `use`](https://github.com/rust-lang/rfcs/pull/1150)
* [Allow macros in types](https://github.com/rust-lang/rfcs/pull/873)

# New RFCs

* [RFC for inclusive ranges with ...](https://github.com/rust-lang/rfcs/pull/1192)
* [RFC: Prevent lint changes being a breaking change](https://github.com/rust-lang/rfcs/pull/1193)
* [RFC: Add item recovery collection APIs](https://github.com/rust-lang/rfcs/pull/1194)
* [ordered query API](https://github.com/rust-lang/rfcs/pull/1195)
* [RFC for allowing eliding more type parameters](https://github.com/rust-lang/rfcs/pull/1196)
* [pretty print `Debug` of tuples, tuple structs and enum variants in a single line](https://github.com/rust-lang/rfcs/pull/1198)
* [SIMD groundwork](https://github.com/rust-lang/rfcs/pull/1199)
* [RFC: Add `cargo install`](https://github.com/rust-lang/rfcs/pull/1200)
* [Add support for naked functions](https://github.com/rust-lang/rfcs/pull/1201)

# Upcoming Events

* [7/15. Tokyo Rust Monthly Meetup](https://rust.doorkeeper.jp/events/27068).
* [7/15. Rust Los Angeles Monthly Meetup](http://www.meetup.com/Rust-Los-Angeles/events/223341178).
* [7/20. Rust Paris](http://www.meetup.com/Rust-Paris).
* [7/22. Columbus Rust Society](http://www.meetup.com/columbus-rs/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# Quote of the Week

*I think if someone placed the Rust and Go community in a room and asked them to fight, we'd probably just all order pizza and geek out over languages.* — [Manish Goregaokar](https://www.reddit.com/r/rust/comments/3cj69b/why_go_and_rust_are_competitors/csw5t5v)

Thanks to [msiemens](https://users.rust-lang.org/users/msiemens) for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
