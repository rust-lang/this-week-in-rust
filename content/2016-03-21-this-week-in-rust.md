Title: This Week in Rust 123
Number: 123
Date: 2016-03-21
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

* [discussion] [What, in your opinion is the worst thing about Rust?](https://www.reddit.com/r/rust/comments/4b5rfi/what_in_your_opinion_is_the_worst_thing_about_rust/).
* [The epic story of Dropbox’s exodus from the amazon cloud empire (feat. Rust)](http://www.wired.com/2016/03/epic-story-dropboxs-exodus-amazon-cloud-empire/).
* [Servo is going to have an alpha release in June: Servo + browser.html](https://www.reddit.com/r/rust/comments/4aec34/servo_is_going_to_have_an_alpha_release_in_june/).
* [Learn you a Rust II - references and borrowing](http://pro.theta.eu.org/2016/03/18/lyar-borrows.html).
* [Mozilla looks towards the IoT for Rust](https://www.voxxed.com/blog/2016/03/rust/).
* [Working with C unions in Rust FFI](http://hermanradtke.com/2016/03/17/unions-rust-ffi.html).
* [pdf] [Fuzzing the Rust typechecker using CLP](https://www.cs.ucsb.edu/~benh/research/papers/dewey15fuzzing.pdf).
* [This week in Servo 55](http://blog.servo.org/2016/03/14/twis-55/).

## Notable New Crates & Project Updates

* [Gtk-rs released 0.0.7 with major changes](http://gtk-rs.org/blog/2016/03/15/forget-everything-you-knew-gtk-0.0.7.html).
* [RustType 0.2 - Now with dynamic GPU font caching](https://github.com/dylanede/rusttype/blob/205def21e370e35e2b860eb6f086fda749e57df8/CHANGELOG.md).
* [libs.rs](http://libs.rs). A catalogue of Rust community's awesomeness.
* [dryad.so.1](http://github.com/m4b/dryad) - an experimental x86-64 ELF dynamic linker, written entirely in Rust (and some asm)
* [Termion](https://github.com/Ticki/termion) (previously called libterm) now supports 256 colour mode, asynchronous key events, special key events, and password input.

# Summer of Code Projects

Hi students! Looking for an awesome summer project in Rust? Look no further! Chris Holcombe from Canonical is an experienced Google Summer of Code mentor and has a project to implement CephX protocol decoding. [Check it out here](https://wiki.ubuntu.com/GoogleSoC2016/Ideas#Decode_CephX_Protocol).

Servo is also accepting GoSC project submissions under the Mozilla banner. See if any of the [project ideas](https://wiki.mozilla.org/Community:SummerOfCode16#Servo) appeal to you and read the [advice for applications](https://wiki.mozilla.org/Community:SummerOfCode16#Application_Advice).

Servo also has [a project](https://teams.railsgirlssummerofcode.org/projects/104-servo) in Rails Girls Summer of Code. nom is [also participating](https://teams.railsgirlssummerofcode.org/projects/78-nom).

# Crate of the Week

This week's Crate of the Week is [tempfile](https://crates.io/crates/tempfile), a crate that does exactly what it says on the tin. Thanks to [Steven Allen](https://users.rust-lang.org/users/stebalien) for the suggestion!

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [Rust: OsString could implement the Default trait](https://github.com/rust-lang/rust/issues/32385).
* [easy] [Rust: Backtrace contains function names with MIR, but not on MSVC](https://github.com/rust-lang/rust/issues/32384).
* [less easy] [Servo: Write a tool that reports unnecessary crate dependencies](https://github.com/servo/servo/issues/9256).
* [easy] [Servo/Saltfs: Trim down the symlinks to ARM cross-compilation binaries](https://github.com/servo/saltfs/issues/252).
* [easy] [`cargo add`: Target specifications](https://github.com/killercup/cargo-edit/issues/13).
* [easy] [`cargo list`: More tests](https://github.com/killercup/cargo-edit/issues/16).
* [easy/mentored] [`multipart`: create sample projects](https://github.com/cybergeek94/multipart/issues/29)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

103 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-03-14..2016-03-21

## Notable changes

* [RFC 1210 impl specialization has landed!](https://github.com/rust-lang/rust/pull/30652) Yay!
* [MIR is now able to bootstrap itself...into orbit!](https://github.com/rust-lang/rust/pull/32080)
* [typestrong const integers](https://github.com/rust-lang/rust/pull/30587) (breaking change)
* [Fast floating point algebra](https://github.com/rust-lang/rust/pull/32256)
* [#derive now uses intrinsics::unreachable](https://github.com/rust-lang/rust/pull/32250)
* [*Map-Entries now have a `.key()` method](https://github.com/rust-lang/rust/pull/32248)
* [Rustc platform intrinsics went on a slimming diet](https://github.com/rust-lang/rust/pull/32236)
* [`rustc --test -q`: Shorter test output](https://github.com/rust-lang/rust/pull/31887)
* [LLVM assertions disabled on ARM to workaround codegen bug](https://github.com/rust-lang/rust/pull/32361)
* [Another LLVM update](https://github.com/rust-lang/rust/pull/32337)
* [Fix an ICE in region inference](https://github.com/rust-lang/rust/pull/32332)
* [No *into_ascii methods after all](https://github.com/rust-lang/rust/pull/32314) (destabilized and removed because of inference regression)
* [Warn, not err on inherent overlaps](https://github.com/rust-lang/rust/pull/32309) (for now, this will become an error in later versions)
* [coercions don't kill rustc anymore](https://github.com/rust-lang/rust/pull/32306)
* [const_eval now correctly handles right shifts](https://github.com/rust-lang/rust/pull/32285)
* [Fix overflow when subtracting Instant](https://github.com/rust-lang/rust/pull/32273)

## New Contributors

* Daniel J Rollins
* pravic
* Stu Black
* tiehuis
* Todd Lucas

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1479: Unix socket support in the standard library](https://github.com/rust-lang/rfcs/pull/1479).
* [RFC 1498: Add octet-oriented interface to `std::net::Ipv6Addr`](https://github.com/rust-lang/rfcs/pull/1498).
* [RFC 1434: Implement a method, `contains()`, for `Range`, `RangeFrom`, and `RangeTo`, checking if a number is in the range.](https://github.com/rust-lang/rfcs/pull/1434).
* [RFC 1432: Add a `replace_slice` method to `Vec<T>` and `String` which removes a range of elements, and replaces it in place with a given sequence of values](https://github.com/rust-lang/rfcs/pull/1432).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Unsafe expressions](https://github.com/rust-lang/rfcs/pull/1346).
* [Prevent unstable items from causing name resolution conflicts with downstream code](https://github.com/rust-lang/rfcs/pull/1321).
* [Add support for naked functions](https://github.com/rust-lang/rfcs/pull/1201).
* [Expand the current pub/non-pub categorization of items with the ability to say "make this item visible solely to a (named) module tree"](https://github.com/rust-lang/rfcs/pull/1422).

## New RFCs

* [Allow fields in traits that map to lvalues in `impl`'ing type](https://github.com/rust-lang/rfcs/pull/1546).
* [`as_millis` function on `std::time::Duration`](https://github.com/rust-lang/rfcs/pull/1547).
* [Add `global_asm!` for module-level inline assembly](https://github.com/rust-lang/rfcs/pull/1548).

# Upcoming Events

* [3/22. Bay Area Rust: Rust, Data, and Science](http://www.meetup.com/Rust-Bay-Area/events/229107276/).
* [3/23. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [3/31. Tokyo Rust Meetup #4](http://www.meetup.com/Tokyo-Rust-Meetup/events/229260081/).
* [4/15. Frankfurt/Main Rust Lint Workshop](http://www.meetup.com/de-DE/Rust-Rhein-Main/events/229564640/?eventId=229564640)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [PhD and postdoc positions](http://plv.mpi-sws.org/rustbelt/) at MPI-SWS.
* [Rust developer](http://rust.jobboard.io/jobs/125594-rust-developer-at-the-blackbird) at The Blackbird.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> <phaazon\> ok
> <phaazon\> and what trait is from from?
> <jix\> From :D
> <phaazon\> ok :D
> <phaazon\> so from is from From
> <phaazon\> nice!

— [phaazon on #rust](https://botbot.me/mozilla/rust/2016-03-20/?msg=62542397&page=11).

Thanks to [Thomas Winwood](https://users.rust-lang.org/users/ketsuban) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
