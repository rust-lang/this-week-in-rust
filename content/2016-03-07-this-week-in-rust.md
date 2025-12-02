Title: This Week in Rust 121
Number: 121
Date: 2016-03-07
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

* 🎈🎉 [Announcing Rust 1.7](http://blog.rust-lang.org/2016/03/02/Rust-1.7.html). 🎉🎈
* [Memory Management in Rust and Swift](https://medium.com/@itchyankles/memory-management-in-rust-and-swift-8ecda3cdf5b7).
* [Rust and Swift](http://www.chriskrycho.com/rust-and-swift.html). An ongoing series of blog posts comparing Rust and Swift.
* [Lambda architecture with differential dataflow](http://www.poumeyrol.fr/2016/02/29/Query2-in-differential-dataflow/). Part #7 of a series about a BigData in Rust experiment.
* [How to use Eco](http://blog.piston.rs/2016/03/04/how-to-use-eco/). Eco is a tool for reasoning about breaking changes in Rust ecosystems.
* [Clippy: Linting as a Service](http://www.bashy.io/news/2016/03/05/clippy-linting-as-a-service/).
* [ASM.JS code using Rust and Emscripten](http://ashleysommer.com.au/how-to/articles/asm-js-code-using-rust-and-emscripten).
* [video] [Ferris makes emulators: Episode 7](https://youtu.be/0GtccLHA9hc). Recorded stream of Ferris developing a N64 emulator in Rust (also on [Twitch](http://www.twitch.tv/ferrisstreamsstuff/profile)).
* [Nice errors in LALRPOP](http://smallcultfollowing.com/babysteps/blog/2016/03/02/nice-errors-in-lalrpop/). LALRPOP is an LR(1) parser generator for Rust.
* [This week in Servo 53](https://blog.servo.org/2016/02/29/twis-53/).

## Notable New Crates & Project Updates

* [Announcing Leaf 0.2](https://github.com/autumnai/leaf/blob/063ce978004b8bf4b7fc7481fdd58fed0515cfd8/RELEASE.md) - one of the fastest Machine Intelligence Frameworks that exist today.
* RFC on [rewriting the data structure and API](https://github.com/servo/rust-url/pull/176) for rust-url.
* [Oxipng](https://github.com/shssoichiro/oxipng). Lossless PNG compression optimiser written in Rust.
* [Caesar](https://github.com/Postage/caesar). A drop-in replacement for the Rust standard library TCP listener with TLSv1.2 enabled.
* [rust-stm](https://github.com/Marthog/rust-stm). A Rust library that implements software transactional memory.
* [concatenation_benchmarks-rs](https://github.com/hoodie/concatenation_benchmarks-rs). Comparing ways to concatenate strings in Rust.
* [rosc](https://github.com/klingtnet/rosc). An implementation of the OSC 1.0 (Open Sound Control) protocol in pure Rust.
* [Rsoundio](https://github.com/klingtnet/rsoundio). A wrapper for libsoundio, a cross-platform realtime audio in- and output library.
* [Releasing ndarray 0.4](http://bluss.github.io/rust/2016/03/06/ndarray-0.4/). ndarray is a Rust library providing an n-dimensional array.

# Summer of Code Projects

Hi students! Looking for an awesome summer project in Rust? Look no further! Chris Holcombe from Canonical is an experienced Google Summer of Code mentor and has a project to implement CephX protocol decoding. [Check it out here](https://wiki.ubuntu.com/GoogleSoC2016/Ideas#Decode_CephX_Protocol).

Servo is also accepting GoSC project submissions under the Mozilla banner. See if any of the [project ideas](https://wiki.mozilla.org/Community:SummerOfCode16#Servo) appeal to you and read the [advice for applications](https://wiki.mozilla.org/Community:SummerOfCode16#Application_Advice).

Servo also has [a project](https://teams.railsgirlssummerofcode.org/projects/104-servo) in Rails Girls Summer of Code. nom is [also participating](https://teams.railsgirlssummerofcode.org/projects/78-nom).

# Crate of the Week

This week's Crate of the Week is [preferences](https://crates.io/crates/preferences) which does the right thing to your program's preferences on all common operating systems.

Thanks to [llogiq](https://users.rust-lang.org/users/llogiq) for the suggestion.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Easy] [cargo-add accepts package names with mismatching hyphen/underscore without conversion](https://github.com/killercup/cargo-edit/issues/51).
* [Hard] [`cargo add`: Infer crate name from path/git repo](https://github.com/killercup/cargo-edit/issues/14).
* [Easy] [`cargo add`: Target specifications](https://github.com/killercup/cargo-edit/issues/13).
* [Easy] [`cargo list`: More tests](https://github.com/killercup/cargo-edit/issues/16).
* [Easy] [Rust: -C suggestions to use llc for details is annoying](https://github.com/rust-lang/rust/issues/30961).
* [Easy] [Rust: JSON errors with suggestions are incomplete](https://github.com/rust-lang/rust/issues/30701).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

113 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-02-29..2016-03-07

## Notable changes

* [rustdoc: Split out rustdoc pass to strip private imports](https://github.com/rust-lang/rust/pull/32055). No more private imports in rustdoc.
* [Improve the `check_pat_enum` logic](https://github.com/rust-lang/rust/pull/32039). rustc will catch missing fields in pattern matches again.
* [Truncate i8-s to i1-s when loading constants](https://github.com/rust-lang/rust/pull/32032). Regression fix for LLVM type errors with bools.
* [Use `drop_in_place` in `Vec` and `VecDeque`](https://github.com/rust-lang/rust/pull/32012). VecDeques now drop faster.
* [Extend compiletest to support revisions, incremental tests](https://github.com/rust-lang/rust/pull/32007).
* [Use raw pointer casts for `slice`, `str`'s `.as_ptr()`](https://github.com/rust-lang/rust/pull/31999). `slice.as_ptr` generates simpler LLVM IR.
* [Update libc](https://github.com/rust-lang/rust/pull/31996).
* [Changed `std::pattern::Pattern` impl on `&'a &'a str` to `&'a &'b str`](https://github.com/rust-lang/rust/pull/31989). Patterns can now be pointers-to-pointers to str with different lifetimes.
* [Fix compiling libstd with emscripten target](https://github.com/rust-lang/rust/pull/31985). libstd can now be compiled with emscripten.
* [Use HIR map instead of tcx in constant evaluator](https://github.com/rust-lang/rust/pull/31962). One less internal compiler error with trait consts.
* [Implement RFC 1461](https://github.com/rust-lang/rust/pull/31945). Move some net2 functionality into libstd.
* [std: Stabilize APIs for the 1.8 release](https://github.com/rust-lang/rust/pull/31928).
* [Give `Name`s to positional fields and merge them with named fields](https://github.com/rust-lang/rust/pull/31937). Positional fields are now named starting from "0", "1", ...
* [Use `box` syntax in `vec!` macro](https://github.com/rust-lang/rust/pull/31797). `vec![..]` no longer copies from the stack.
* [[MIR] Some initial zeroing implementation work](https://github.com/rust-lang/rust/pull/31430). Zeroing on drop.
* [Add `AsciiExt::into_ascii_{upper,lower}case`](https://github.com/rust-lang/rust/pull/31335).
* [Implement RFC 1192 inclusive ranges](https://github.com/rust-lang/rust/pull/30884).

## New Contributors

* ashleysommer
* Evan
* Geoff Catlin
* gohyda
* John Talling
* srinivasreddy
* ubsan
* vegai

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week!*.

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Add a `replace_slice` method to `Vec<T>` and `String` which removes a range of elements, and replaces it in place with a given sequence of values](https://github.com/rust-lang/rfcs/pull/1432).
* [Implement a method, `contains()`, for `Range`, `RangeFrom`, and `RangeTo`, checking if a number is in the range.](https://github.com/rust-lang/rfcs/pull/1434).
* [Unix socket support in the standard library](https://github.com/rust-lang/rfcs/pull/1479).
* [Add octet-oriented interface to `std::net::Ipv6Addr`](https://github.com/rust-lang/rfcs/pull/1498).

## New RFCs

* [Allow generic `const`s and `static`s with the obvious syntax and semantics](https://github.com/rust-lang/rfcs/pull/1520).
* [Specifying that `<T as Clone>::clone(&t)` where `T: Copy` should be equivalent to `ptr::read(&t)`](https://github.com/rust-lang/rfcs/pull/1521).
* [Add a initial, minimal form of `impl Trait`](https://github.com/rust-lang/rfcs/pull/1522).
* [Custom dynamically sized types for Rust](https://github.com/rust-lang/rfcs/pull/1524).
* [Add workspaces to Cargo](https://github.com/rust-lang/rfcs/pull/1525).
* [Add a variation to the entry API for maps that doesn't store the search key in the `Entry`](https://github.com/rust-lang/rfcs/pull/1533).

# Upcoming Events

* [3/9. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [3/9. Rust Boulder/Denver Monthly Meeting](http://www.meetup.com/Rust-Boulder-Denver/).
* [3/10. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [3/11. Darmstadt Rust Table of Regulars](http://www.meetup.com/de-DE/Rust-Rhein-Main/events/228665878/).
* [3/14. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [3/17. London Rust Meetup](http://www.meetup.com/Rust-London-User-Group/events/229413056/).
* [3/21. Rust Paris](http://www.meetup.com/Rust-Paris)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [PhD and postdoc positions](http://plv.mpi-sws.org/rustbelt/) at MPI-SWS.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No QotW were selected for this week.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
