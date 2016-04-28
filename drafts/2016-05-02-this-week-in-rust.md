Title: This Week in Rust 128
Number: 128
Date: 2016-05-02
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

* [Optimizing Matrix Multiplication in Rust.](http://www.suchin.co/2016/04/25/Matrix-Multiplication-In-Rust-Pt-1/)


## Notable New Crates & Project Updates


# Crate of the Week

This week's Crate of the Week is [owning_ref](https://crates.io/crates/owning_ref), which contains a reference type that can carry its owner with it. Thanks to [Diwic](https://users.rust-lang.org/users/diwic) for the suggestion!

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Add error explanations for all error codes](https://github.com/rust-lang/rust/issues/32777).
* [easy] [servo/saltfs: buildbot steps need description and descriptionDone](https://github.com/servo/saltfs/issues/337).
* [easy] [rust: rustbuild: Add a tidy check to ensure Cargo.lock updates are checked in](https://github.com/rust-lang/rust/issues/32901).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

186 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-04-11..2016-04-25

## Notable changes

* [`pub(restricted)` (RFC 1422) implemented](https://github.com/rust-lang/rust/pull/32875)
* [Warn on type parameter defaults](https://github.com/rust-lang/rust/pull/32817), this will become an error in the future
* [RFC #1494 (amendment to #550) implemented](https://github.com/rust-lang/rust/pull/32945), allows blocks to follow types/paths in macro patterns
* [`de-`/`encode()` methods no longer break Serialization deriving](https://github.com/rust-lang/rust/pull/32908)
* [Macro hygiene bugs fixed](https://github.com/rust-lang/rust/pull/32923)
* [Avoid crashing due to duplicate external items](https://github.com/rust-lang/rust/pull/32970)
* [Fix multiple glob import](https://github.com/rust-lang/rust/pull/32814)
* [MIR debuginfo mostly works](https://github.com/rust-lang/rust/pull/32952) ([etc.](https://github.com/rust-lang/rust/pull/32803))
* [MIR Blocks no longer require END_BLOCK](https://github.com/rust-lang/rust/pull/33030)
* [MIR now has LLVM-agnostic type layout](https://github.com/rust-lang/rust/pull/32939)
* [Register duplicate item symbols anyway](https://github.com/rust-lang/rust/pull/32946)
* [Resolve compiler performance regression fixed](https://github.com/rust-lang/rust/pull/33064)
* [Syntax: Import prefixes are now paths](https://github.com/rust-lang/rust/pull/33044)
* [Don't report errors in constants at every use site](https://github.com/rust-lang/rust/pull/32877)
* [Handle over-aligned realloc failures on UNIX](https://github.com/rust-lang/rust/pull/32997)
* [String::truncate goes to greater lengths to not panic](https://github.com/rust-lang/rust/pull/32977)
* [`BinaryHeap::append(..)`](https://github.com/rust-lang/rust/pull/32987)
* [Faster `is_char_boundary()` with bit twiddling](https://github.com/rust-lang/rust/pull/32862)
* [Fixed `BufRead` overrun on `Take`](https://github.com/rust-lang/rust/pull/32855)
* [`Default` for `RwLock`, `Mutex`, `CondVar`, `CStr`, `Path`](https://github.com/rust-lang/rust/pull/32785)
* [Cargo can now use multiple git user names](https://github.com/rust-lang/cargo/pull/2584)
* [Removed the (apparently broken) `std::net::IPV6_V6ONLY` feature](https://github.com/rust-lang/rust/pull/33124)
* [Handle `DefId`s and extern crates before lowering the AST to HIR](https://github.com/rust-lang/rust/pull/33089)
* [Compiletest now uses JSON output](https://github.com/rust-lang/rust/pull/33020)
* [`VecDeque::contains(_)` and `LinkedList::contains(_)` implemented](https://github.com/rust-lang/rust/pull/32951)
* [Rust now bootstraps from previous stable instead of snapshots](https://github.com/rust-lang/rust/pull/32942)
* [`impl From<Vec<T>>` and `Into<Vec<T>>` for `VecDeque<T>`](https://github.com/rust-lang/rust/pull/32866)
* [`BTree::append(_)` implemented](https://github.com/rust-lang/rust/pull/32466)

## New Contributors

* Alec S
* Andrey Tonkih
* c4rlo
* David Hewitt
* David Tolnay
* Deepak Kannan
* Gigih Aji Ibrahim
* jocki84
* Jonathan Turner
* Kaiyin Zhong
* Lukas Kalbertodt
* Lukas Pustina
* Maxim Samburskiy
* Raph Levien
* rkjnsn
* Sander Maijers
* Szabolcs Berecz

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1543: Add more integer atomic types](https://github.com/rust-lang/rfcs/pull/1543).
* [RFC 1510: Add a new crate-type, cdylib](https://github.com/rust-lang/rfcs/pull/1510).
* [RFC 1535: Stabilize the `-C overflow-checks` command line argument](https://github.com/rust-lang/rfcs/pull/1535).
* [RFC 1440: Allow Drop types in statics/const functions](https://github.com/rust-lang/rfcs/pull/1440).
* [RFC 1399: Add `#[repr(pack = "N")]`](https://github.com/rust-lang/rfcs/pull/1399).
* [Amend RFC 1228 with operator fixity and precedence](https://github.com/rust-lang/rfcs/pull/1319).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Add `#[repr(align = "N")]`](https://github.com/rust-lang/rfcs/pull/1358).
* [Proposal for thread affinity](https://github.com/rust-lang/rfcs/pull/1480).
* [Specifying that `<T as Clone>::clone(&t)` where `T: Copy` should be equivalent to `ptr::read(&t)`](https://github.com/rust-lang/rfcs/pull/1521).
* [Add workspaces to Cargo](https://github.com/rust-lang/rfcs/pull/1525).
* [Add `TryFrom` and `TryInto` traits](https://github.com/rust-lang/rfcs/pull/1542).
* [`as_millis` function on `std::time::Duration`](https://github.com/rust-lang/rfcs/pull/1547).

## New RFCs

* [`FusedIterator` marker trait and `iter::Fuse` specialization](https://github.com/rust-lang/rfcs/pull/1581).
* [Add `parse_generics!` and `parse_where!` macros](https://github.com/rust-lang/rfcs/pull/1583).
* [Macros by example 2.0. A replacement for `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1584).
* [Define a best practices procedure for making bug fixes in the compiler](https://github.com/rust-lang/rfcs/pull/1589).
* [Add a `lifetime` specifier to `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1590/files).

# Upcoming Events

* [4/26. Rust Bay Area: Securing OSes and Apps with Rust, seL4, and SGX](http://www.meetup.com/Rust-Bay-Area/events/230271083/).
* 4/27. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [5/4. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [5/9. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [5/13. Rust Meetup Darmstadt](http://www.meetup.com/de-DE/Rust-Rhein-Main/events/230396961/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [PhD and postdoc positions](http://plv.mpi-sws.org/rustbelt/) at MPI-SWS.
* [Software Engineer](http://www.coturnix.fr/en/#join) at Coturnix.
* [Senior full stack developer](http://onesignal.applytojob.com/apply/gpSzt4/Senior-Full-Stack-Developer) at OneSignal.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

>> Cow is still criminally underused in a lot of code bases
>
> I suggest we make a new slogan to remedy this:
> "To err is human, to moo bovine."
> (I may or may not have shamelessly stolen this from [this bug report](https://bugs.launchpad.net/ubuntu/+source/apt/+bug/56125/comments/63))

[so_you_like_donuts on reddit](https://www.reddit.com/r/rust/comments/4fc6m4/from_str_to_cow/d27rv0f).

Thanks to [killercup](https://users.rust-lang.org/users/killercup) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
