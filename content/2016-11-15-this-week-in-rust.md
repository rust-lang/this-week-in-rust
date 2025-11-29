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

## News & Blog Posts

* 🎈🎉 [Announcing Rust 1.13](https://blog.rust-lang.org/2016/11/10/Rust-1.13.html). 🎉🎈
* [Schemers: Build a Scheme Interpreter in Rust - a tutorial for Rust beginners](https://mgattozzi.github.io/2016/11/08/scheme-input.html).
* [Rapid prototyping C applications with Rust](https://rust-leipzig.github.io/cargo/2016/11/13/rapid-prototyping-c-applications/).
* [Associated type constructors, part 4: Unifying ATC and HKT](http://smallcultfollowing.com/babysteps/blog/2016/11/09/associated-type-constructors-part-4-unifying-atc-and-hkt/).
* [Parallel iterators, part 3: Consumers](http://smallcultfollowing.com/babysteps/blog/2016/11/14/parallel-iterators-part-3-consumers/).
* [A quick tour of Rust’s Type System part 1: Sum Types (a.k.a. Tagged Unions)](https://tonyarcieri.com/a-quick-tour-of-rusts-type-system-part-1-sum-types-a-k-a-tagged-unions).
* [The fastest template engine in the West](https://lambda.xyz/blog/maud-is-fast/).
* [Implementing Finite Automata in Rust (Part 2)](https://apanatshka.github.io/compsci/2016/11/12/implementing-finite-automata-part-2/).
* [Using Rust for ‘scripting’](http://www.chriskrycho.com/2016/using-rust-for-scripting.html).

## novemb.rs

[novemb.rs](http://novemb.rs), the distributed Rust hackfest, is happening this weekend. If you would like to participate, please refer to the website for a local meetup or for a chat to get in contact with other Rustceans. Note that if you want to attend a meetup,  you should check on Friday for most up-to-date information.

## Other Weeklies from Rust Community

* [This week in Rust docs 30](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-30). Updates from the Rust documentation team.
* [These weeks in Servo 83](https://blog.servo.org/2016/11/14/twis-83/). Servo is a prototype web browser engine written in Rust.
* [This week in TiKV 2016-11-14](http://weekly.pingcap.com/2016/11/14/tidb-weekly/#weekly-update-in-tikv). TiKV is a distributed Key-Value database.

# Crate of the Week

*No crate was selected for CotW.*

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [less easy] [rust: Separate foreign items in HIR](https://github.com/rust-lang/rust/issues/37713).
* [less easy] [rust: Separate trait items from trait](https://github.com/rust-lang/rust/issues/37712).
* [easy] [rust: docs: Explain why/when `.lines()` returns an error](https://github.com/rust-lang/rust/issues/37744).
* [easy] [git-series: Highlight trailing whitespace](https://github.com/git-series/git-series/issues/31).
* [easy] [git-series: Support rebase --exec](https://github.com/git-series/git-series/issues/24).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

152 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-11-07..2016-11-14

* [Stabilize `..` in tuple (struct) patterns](https://github.com/rust-lang/rust/pull/36843).
* [Partially stabilize RFC 1506 "Clarify relationships between ADTs"](https://github.com/rust-lang/rust/pull/36868).
* [Macro parser performance improvements and refactoring](https://github.com/rust-lang/rust/pull/37701).
* [Add `{into,from}_raw` to Rc and Arc](https://github.com/rust-lang/rust/pull/37192).
* [Replace FNV with a faster hash function](https://github.com/rust-lang/rust/pull/37229).
* [Support `#[macro_reexport]`ing custom derives](https://github.com/rust-lang/rust/pull/37542).
* [Replace syntax's SmallVector with AccumulateVec](https://github.com/rust-lang/rust/pull/37551).
* [Add support for ARMv5TE architecture](https://github.com/rust-lang/rust/pull/37615).
* [std: Derive `Default` for `Duration`](https://github.com/rust-lang/rust/pull/37699).

## New Contributors

* abhijeetbhagat
* Angelo Polo
* Arthur Silva
* Havvy
* Josh Driver
* Juan Gomez
* karpinski
* meh
* Nicolas B. Pierron
* oldmanmike
* Trotter Cashion

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1728: A process for establishing the Rust roadmap](https://github.com/rust-lang/rfcs/pull/1728).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Require documentation for all new features](https://github.com/rust-lang/rfcs/pull/1636).
* [Add two functions, `ptr::read_unaligned` and `ptr::write_unaligned`, which allows reading/writing to an unaligned pointer](https://github.com/rust-lang/rfcs/pull/1725).

## New RFCs

* [Introduce `Option::<&T>::borrowed`](https://github.com/rust-lang/rfcs/pull/1792).
* [`AsCell` conversion from `&mut T` to `&Cell<T>`](https://github.com/rust-lang/rfcs/pull/1789).

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

* [11/16. Rust User Group Cologne - Open Source Audio Meetup](http://rust.cologne/2016/11/16/audio-meetup.html).
* [11/16. London Rust meetup #10](https://www.meetup.com/Rust-London-User-Group/events/234999144/).
* [11/16. Rust LA Monthly Meetup - Hack Night](https://www.meetup.com/Rust-Los-Angeles/events/234998313/).
* [11/16. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [11/16. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [11/17. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [11/19 - 11/20. ./novemb.rs Code Sprint](http://novemb.rs/).
* [11/19 - 11/20. Rust Cologne - Weekend Special: Novemb.rs Code Sprint sponsored by Mozilla](https://www.meetup.com/RustCologne/events/235374218/).
* [11/19 - 11/20. L'événement du Logiciel Libre à Toulouse](https://2016.capitoledulibre.org/programme.html).
* [11/19 - 11/20. Rust Vilnius - Boot Camp Rust](https://www.meetup.com/Rust-in-Vilnius/events/234293479/).
* [11/21. Rust Paris meetup #34](https://www.meetup.com/Rust-Paris/events/235570335/).
* [11/22. Kaspersky CoLaboratory: Moscow Rust Meetup](https://events.kaspersky.com/event/rust2).
* [11/23. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [11/23. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# fn work(on: RustProject) -> Money

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Now higher-kinded types especially are one of those PL topics that sound forebodingly complex and kind of abstract (like monads). But once you learn what it is, you realize it’s actually relevant to your life (unlike monads).

— [@nikomatsakis invoking the M word in his blog post](http://smallcultfollowing.com/babysteps/blog/2016/11/02/associated-type-constructors-part-1-basic-concepts-and-introduction/).

Thanks to [Japaric](https://users.rust-lang.org/users/japaric) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
