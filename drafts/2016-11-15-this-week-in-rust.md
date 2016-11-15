Title: This Week in Rust 156
Number: 156
Date: 2016-11-15
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## novemb.rs

[novemb.rs](http://novemb.rs), the distributed Rust hackfest, is happening this weekend. If you would like to participate, please refer to the website for a local meetup or for a chat to get in contact with other Rustceans. Note that if you want to attend a meetup,  you should check on Friday for most up-to-date information.

## Blog Posts

## Other Weeklies from Rust Community

## New Crates

* [nautilus-extension](https://github.com/talklittle/nautilus-extension-rs) – Safe bindings to libnautilus-extension

# Crate of the Week

*No crate was selected for CotW.*

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [git-series: Highlight trailing whitespace](https://github.com/git-series/git-series/issues/31).
* [easy] [git-series: Support rebase --exec](https://github.com/git-series/git-series/issues/24).
* [easy] [servo: Make parse functions implement from Parse trait in style](https://github.com/servo/servo/issues/14101).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

140 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-10-31..2016-11-07

* [Cargo: Use a single profile set per workspace](https://github.com/rust-lang/cargo/pull/3249).
* [rustc: Add knowledge of Windows subsystems](https://github.com/rust-lang/rust/pull/37501).
* [Replace all uses of SHA-256 with BLAKE2b](https://github.com/rust-lang/rust/pull/37439).
* [Rust download: Recommend MSVC ABI](https://github.com/rust-lang/rust-www/pull/602).
* [Cargo: Ignore `panic` configuration for test/bench profiles](https://github.com/rust-lang/cargo/pull/3175).
* [Add or and or_else for ordering](https://github.com/rust-lang/rust/pull/37054).
* [rustbuild: support MIPS host builds](https://github.com/rust-lang/rust/pull/37625).
* [Add `-Z hir-stats` for collecting statistics on HIR and AST](https://github.com/rust-lang/rust/pull/37583).
* [Shrink `hir::Expr` slightly](https://github.com/rust-lang/rust/pull/37577).
* [rustbuild: Rewrite user-facing interface](https://github.com/rust-lang/rust/pull/37521).
* [Cargo: Expose rustc cfg values to build scripts](https://github.com/rust-lang/cargo/pull/3243).
* [Don't reuse RandomState seeds](https://github.com/rust-lang/rust/pull/37470).
* [Add `.wrapping_offset()` methods](https://github.com/rust-lang/rust/pull/37422).
* [Reduce the number of bytes hashed by IchHasher](https://github.com/rust-lang/rust/pull/37427).
* [Add impls for `&Wrapping`. Also `Sum`, `Product` impls for both `Wrapping` and `&Wrapping`](https://github.com/rust-lang/rust/pull/37356).
* [Prevent exhaustive matching of Ordering to allow for future extension](https://github.com/rust-lang/rust/pull/37351).
* [Add Iterator trait TrustedLen to enable better FromIterator / Extend](https://github.com/rust-lang/rust/pull/37306).
* [Add `unwrap_or_default` method to `Result`](https://github.com/rust-lang/rust/pull/37299).
* [Implement `RefUnwindSafe` for atomic types](https://github.com/rust-lang/rust/pull/37178).
* [Detect extra region requirements in impls](https://github.com/rust-lang/rust/pull/37167).
* [Add conversions from `io:ErrorKind` to `io::Error`](https://github.com/rust-lang/rust/pull/37037).
* [Optimize ObligationForest's NodeState handling](https://github.com/rust-lang/rust/pull/36993).
* [hashmap: Store hashes as usize internally](https://github.com/rust-lang/rust/pull/36595).

## New Contributors

* Dmitry Gritsay
* leonardo.yvens
* Marcin Fatyga
* Martin Glagla
* Matwey V. Kornilov
* pweyck

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week!*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [A process for establishing the Rust roadmap](https://github.com/rust-lang/rfcs/pull/1728).
* [Introduce a new type `MoveCell<T>` in `std::cell`](https://github.com/rust-lang/rfcs/pull/1659).

## New RFCs

* [Conditional dependencies](https://github.com/rust-lang/rfcs/pull/1787).
* [Create a separate libc_types crate for basic C types](https://github.com/rust-lang/rfcs/pull/1783).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

PRs:

* [Customising Rustfmt](https://github.com/rust-lang-nursery/fmt-rfcs/pull/33).

Ready for PR:

* [Comments](https://github.com/rust-lang-nursery/fmt-rfcs/issues/17).
* [Simple blocks, `{ ... }`](https://github.com/rust-lang-nursery/fmt-rfcs/issues/21).

Final comment period:

* [Statements](https://github.com/rust-lang-nursery/fmt-rfcs/issues/11).
* [Imports (`use`)](https://github.com/rust-lang-nursery/fmt-rfcs/issues/24).

Other notable issues:

* [block vs visual indentation](https://github.com/rust-lang-nursery/fmt-rfcs/issues/8).
* [function declarations](https://github.com/rust-lang-nursery/fmt-rfcs/issues/39).
* [match](https://github.com/rust-lang-nursery/fmt-rfcs/issues/34).

# Upcoming Events

* [11/9. Rust Boulder/Denver Monthly Meeting](https://www.meetup.com/Rust-Boulder-Denver/events/235031836/).
* [11/10. Columbus Rust Society](https://www.meetup.com/columbus-rs/events/234855067/).
* [11/14. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/234725296/).
* [11/15. Rust Philippines: Introduction to Rust Programming Language](http://www.rustph.tech/rust-101-session-for-november-2016/).
* [11/16. Rust User Group Cologne - Open Source Audio Meetup](http://rust.cologne/2016/11/16/audio-meetup.html).
* [11/16. London Rust meetup #10](https://www.meetup.com/Rust-London-User-Group/events/234999144/).
* [11/16. Rust LA Monthly Meetup - Hack Night](https://www.meetup.com/Rust-Los-Angeles/events/234998313/).
* [11/16. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [11/16. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [11/19 - 11/20. ./novemb.rs Code Sprint](http://novemb.rs/).
* [11/19 - 11/20. Rust Cologne - Weekend Special: Novemb.rs Code Sprint sponsored by Mozilla](https://www.meetup.com/RustCologne/events/235374218/).
* [11/19 - 11/20. L'événement du Logiciel Libre à Toulouse](https://2016.capitoledulibre.org/programme.html).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# fn work(on: RustProject) -> Money

* [Senior Rust Developer at OneSignal Mountain View, CA](http://onesignal.applytojob.com/apply/supk2g/Senior-Rust-Developer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week


> I want to paint you a picture of a utopia in which Rust has expanded to become the fabric of the entire classical computing world, where the possibilities of what we can achieve are not shackled to the decaying dreams of computer science past. In this perfect utopia you have invented the perfect model for managing your computer's sci-fi hardware, perfectly free from the legacy of Unix and Windows. And you need the perfect language to write it in. Everywhere you look is legacy: C, C++, Java; the stacks get bigger and bigger, cruft all the way down.

> The only shining light is Rust. Those Rustaceans have been chipping away the cruft, distilling their platform to only the essence of bits and bytes, while also expanding its expressive power toward legendary elegance. Rust doesn't want to tell you how to build your system. Rust wants to serve you, to fulfill your dreams, on your terms. For your ambitions, Rust is the only reasonable choice in a world filled with compromises.

— [brson on Refactoring std for ultimate portability](https://internals.rust-lang.org/t/refactoring-std-for-ultimate-portability/4301).

Thanks to [Japaric](https://users.rust-lang.org/users/japaric) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
