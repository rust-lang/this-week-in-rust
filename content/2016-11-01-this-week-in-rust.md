Title: This Week in Rust 154
Number: 154
Date: 2016-11-01
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## Blog Posts

* [A quantum leap for the web](https://medium.com/mozilla-tech/a-quantum-leap-for-the-web-a3b7174b3c12). [Project Quantum](https://wiki.mozilla.org/Quantum) is an effort to develop Mozilla’s next-generation web engine, with a number of its components written in Rust.
* [Corrode update: support from Mozilla, and new features](http://jamey.thesharps.us/2016/10/corrode-update-support-from-mozilla-and.html). Corrode is a C to Rust translator.
* [Librsvg gets Rusty](https://people.gnome.org/~federico/news-2016-10.html#25). Part of The GNOME Project, librsvg is a library to render SVG files using cairo.
* [Porting a few C functions to Rust](https://people.gnome.org/~federico/news-2016-10.html#28). Follow-up to [Librsvg gets Rusty](https://people.gnome.org/~federico/news-2016-10.html#25).
* [Thoughts on DX: GNOME and Rust](https://siliconislandblog.wordpress.com/2016/10/31/thoughts-on-dx-gnome-and-rust/). Alberto Ruiz on embracing Rust in GNOME libraries.
* [Builder Rust](https://blogs.gnome.org/chergert/2016/10/25/builder-rust/). GNOME Builder IDE now supports Rust via [Rust Language Service](https://github.com/jonathandturner/rls).
* [Exploring possible alternative compiler backend: Cretonne](https://internals.rust-lang.org/t/possible-alternative-compiler-backend-cretonne/4275).
* [Rewinding time with rr & Rust to debug a terrible error message](http://blog.faraday.io/reversing-rust-debugging-a-terrible-error-message-backwards-in-time-with-rr-2/).
* [Elysian: Behind the scenes](https://everyweeks.com/entry/5814c12e6965033a77743f63). Elysian is a [demo](https://en.wikipedia.org/wiki/Demoscene) written in Rust that generates a full audiovisual presentation, realtime, with zero external media files. [Watch the video here](https://www.youtube.com/watch?v=rWwNgVwQG1A).
* [A game in Rust](https://thiolliere.org/jekyll/update/2016/10/29/a-game-in-Rust.html). Developing [Ruga](https://github.com/thiolliere/ruga), a video game made in Rust.
* [WebVR coming to Servo: Part 1](https://blog.mozvr.com/webvr-coming-to-servo-part-1/).
* [Text analysis in Rust - Tokenization](http://nitschinger.at/Text-Analysis-in-Rust-Tokenization/).
* [Hacking on the Rust language — error messages](https://andidog.de/blog/2016-10-28-today-i-learned-2-rust-hacking). Short guide on getting started working on Rust compiler — example of adding a previously missing error code explanation.
* [Creating a Dalvik parser in Rust (Part 1)](http://superanalyzer.rocks/2016/10/18/dalvik-parser-1). First part of a series of articles describing how SUPER is building an Android Dalvik executable file parser.
* [Team JaM in Systems Wonderland](http://jeenalee.com/2016/10/26/donutjs.html). Jeena Lee and Malisa Smith talk about their summer of hacking on Servo and learning Rust as beginners.

## News & Project Updates

* 🎃 [Happy Halloween from Ferris](https://i.imgur.com/mNfEWCA.jpg)! 🎃. (Credit: [carols10cents & kibwen](https://www.reddit.com/r/rust/comments/5a8qca/happy_halloween_from_ferris/)).
* The community team is hosting a hybrid hack event on November 19th/20th centered around the African/European time zones. If you are interested in participating, or supplying topics to work on, please head to the [novemb.rs](http://novemb.rs) website.
* [Spectral fluent assertions 0.5.0 released.](https://github.com/cfrancia/spectral)

## Other Weeklies from Rust Community

* [This week in Rust docs 28](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-28). Updates from the Rust documentation team.
* [This week in TiKV 2016-10-31](http://weekly.pingcap.com/2016/10/31/tidb-weekly/). TiKV is a distributed Key-Value database.

## New Crates

* [app_dirs](https://github.com/AndyBarron/app-dirs-rs). Put your app's data in the right place on every platform.
* [Spongedown](https://ivanceras.github.io/spongedown/) converts markdown to html with support for [svgbob diagrams](https://github.com/ivanceras/svgbobrus).
* [loc](https://github.com/cgag/loc). A tool for counting lines of code. It's a Rust implementation of cloc, but it's more than 100x faster.
* [cargo-update](https://github.com/nabijaczleweli/cargo-update). A cargo subcommand for checking and applying updates to installed executables.
* [rustbreak](https://crates.io/crates/rustbreak) - a daybreak inspired self-contained file database.

# Crate of the Week

* This week's Crate of the Week is [app_dirs](https://github.com/AndyBarron/app-dirs-rs). app_dirs lets you put your app's data in the right place on every platform.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [git-series: Highlight trailing whitespace](https://github.com/git-series/git-series/issues/31).
* [easy] [git-series: Support rebase --exec](https://github.com/git-series/git-series/issues/24).
* [easy] [rust: incr.comp.: Create Test Case for Incr. Comp. Hash for unary and binary expressions](https://github.com/rust-lang/rust/issues/37520).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

88 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-10-24..2016-10-31

* [Add Fuchsia support](https://github.com/rust-lang/rust/pull/37313).
* [Add ArrayVec and AccumulateVec to reduce heap allocations during interning of slices](https://github.com/rust-lang/rust/pull/37270).
* [Deprecate no_debug and custom_derive](https://github.com/rust-lang/rust/pull/37128).
* [Disallow Unsized Enums](https://github.com/rust-lang/rust/pull/37111).
* [Support `Self` in struct expressions and patterns](https://github.com/rust-lang/rust/pull/37035).
* [Make sufficiently old or low-impact compatibility lints deny-by-default](https://github.com/rust-lang/rust/pull/36894).
* [Small improvement to SipHasher](https://github.com/rust-lang/rust/pull/37312).
* [Implement `Iterator::fold` for `.chain()`, `.cloned()`, `.map()` and the `VecDeque` iterators](https://github.com/rust-lang/rust/pull/37315).
* [Use `SmallVector` in `CombineFields::instantiate`](https://github.com/rust-lang/rust/pull/37322).
* [Support `use *;` and `use ::*;`](https://github.com/rust-lang/rust/pull/37367).
* [Avoid more allocations when compiling html5ever](https://github.com/rust-lang/rust/pull/37373).
* [libc: Enable musl with aarch64](https://github.com/rust-lang/libc/pull/435).
* [rustc_typeck: Allow reification from fn item to unsafe ptr](https://github.com/rust-lang/rust/pull/37389).
* [Disable jemalloc on aarch64/powerpc](https://github.com/rust-lang/rust/pull/37392).
* [Replace all uses of SHA-256 with BLAKE2b](https://github.com/rust-lang/rust/pull/37439).
* [Shrink Expr_::ExprInlineAsm](https://github.com/rust-lang/rust/pull/37445).

## New Contributors

* Bunts Thy Unholy
* Federico Mena Quintero
* iirelu
* Konrad Borowski
* loggerhead
* msiglreith
* Pieter Frenssen
* Vadzim Dambrouski
* Zoffix Znet

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1665: Windows subsystem support](https://github.com/rust-lang/rfcs/pull/1665).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [A process for establishing the Rust roadmap](https://github.com/rust-lang/rfcs/pull/1728).
* [Introduce a new type `MoveCell<T>` in `std::cell`](https://github.com/rust-lang/rfcs/pull/1659).

## New RFCs

* [Warn by default when casting a pointer to an integer smaller than usize](https://github.com/rust-lang/rfcs/pull/1782).
* [Add Cargo post-build scripts](https://github.com/rust-lang/rfcs/pull/1777).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

PRs:

* [Customising Rustfmt](https://github.com/rust-lang-nursery/fmt-rfcs/pull/33).

[Issue 17](https://github.com/rust-lang-nursery/fmt-rfcs/issues/17) (comments) is ready for a PR, we'd love someone to help out with that, if you're interested ping someone in #rust-style.

Final comment period:

* [Statements](https://github.com/rust-lang-nursery/fmt-rfcs/issues/11).
* [static mut capitalisation](https://github.com/rust-lang-nursery/fmt-rfcs/issues/20).
* [Simple blocks, `{ ... }`](https://github.com/rust-lang-nursery/fmt-rfcs/issues/21).

New P-high issues:

* [Struct and union declarations](https://github.com/rust-lang-nursery/fmt-rfcs/issues/30).
* [Enums and variants](https://github.com/rust-lang-nursery/fmt-rfcs/issues/31).
* [`type` aliases](https://github.com/rust-lang-nursery/fmt-rfcs/issues/32).

# Upcoming Events

* [11/3. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [11/5. Servo / Rust Hackathon](https://www.meetup.com/de-DE/Mozilla-Meetup-Switzerland/events/234883249/?eventId=234883249).
* [11/8. Introduction to Rust Programming Language, Manila PH](https://www.eventbrite.com/e/introduction-to-rust-programming-language-tickets-28577366673).
* [11/9. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [11/9. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [11/9. Rust Boulder/Denver Monthly Meeting](https://www.meetup.com/Rust-Boulder-Denver/events/235031836/).
* [11/10. Columbus Rust Society](https://www.meetup.com/columbus-rs/events/234855067/).
* [11/14. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/234725296/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# fn work(on: RustProject) -> Money

* [Senior Rust Developer at OneSignal Mountain View, CA](http://onesignal.applytojob.com/apply/supk2g/Senior-Rust-Developer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Every once in a while someone discovers a bug in librsvg that makes it all the way to a CVE security advisory. We've gotten double free()s, wrong casts, and out-of-bounds memory accesses. Recently someone did fuzz-testing with some really pathological SVGs, and found interesting explosions in the library. That's the kind of 1970s bullshit that Rust prevents.

— [Federico on porting librsvg to Rust](https://people.gnome.org/~federico/news-2016-10.html#25).

Thanks to [/u/robinst and /u/cjstevenson1](https://www.reddit.com/r/rust/comments/59d2ql/librsvg_gets_rusty/d97ui7q/) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
