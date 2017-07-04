Title: This Week in Rust 189
Number: 189
Date: 2017-07-04
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

* [Sep 30. RustFest Zürich](http://zurich.rustfest.eu/).

# Friends of the Forest

Our community likes to recognize people who have made outstanding contributions
to the Rust Project, its ecosystem, and its community. These people are
'friends of the forest'. The community team has been lax in making nominations for
this on a regular basis, but we hope to get back on track!

Today's featured friend of the forest is [Mark Simulacrum](https://github.com/Mark-Simulacrum). As of Friday, June 23, Mark
has made sure that [all 2,634 open issues on the rust-lang/rust repo](https://github.com/rust-lang/rust/issues) have a label!
Thank you, Mark, for this heroic effort!

# Crate of the Week

This week's crate is [strum](https://crates.io/crates/strum), a crate that allows you to derive stringify and parse operations for your enums. Thanks to [lucab](https://users.rust-lang.org/u/lucab) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [CFP for RustFest Zürich](https://cfp.rustfest.eu/events/rustfest-ch).
* [Rust libz blitz status update 2017-06-20](https://internals.rust-lang.org/t/rust-libz-blitz/5184/80). Contribution opportunities are available.
* [rustup: Fix 'show' displaying UNC paths on windows](https://github.com/rust-lang-nursery/rustup.rs/issues/1177).
* [easy] [rustup: Fix 'show' when active toolchain is not installed](https://github.com/rust-lang-nursery/rustup.rs/issues/1178).
* [easy] [rustup: Add a reqwest backend to rustup](https://github.com/rust-lang-nursery/rustup.rs/issues/1183).
* [easy] [rustup: Teach hyper/rustls HTTP backends to resume partial downloads](https://github.com/rust-lang-nursery/rustup.rs/issues/1181).
* [walkdir: Add Error docs to methods that return Result](https://github.com/BurntSushi/walkdir/issues/24).
* [walkdir: Document why unwraps won't fail](https://github.com/BurntSushi/walkdir/issues/42).
* [walkdir: Link references to std in docs](https://github.com/BurntSushi/walkdir/issues/39).
* [walkdir: Correct errors in WalkDir type docs](https://github.com/BurntSushi/walkdir/issues/32).
* [walkdir: Document that `Iter` and `IterFilterEntry` are the result of trait methods](https://github.com/BurntSushi/walkdir/issues/31).
* [walkdir: Add links to other walkdir items in WalkDirIterator docs](https://github.com/BurntSushi/walkdir/issues/30).
* [walkdir: Add links to other walkdir items in Iter and IterFilterEntry docs](https://github.com/BurntSushi/walkdir/issues/28).
* [walkdir: Add links to other walkdir items in DirEntry docs](https://github.com/BurntSushi/walkdir/issues/27).
* [walkdir: Add example for content_first](https://github.com/BurntSushi/walkdir/issues/26).
* [rust-cookbook: Use `filter_entry` in walkdir examples](https://github.com/brson/rust-cookbook/issues/182).
* [easy] [rust-bindgen: Default to generating constified enums, rather than generating Rust enums](https://github.com/servo/rust-bindgen/issues/758).
* [less-easy] [rust-bindgen: Rewrite `is_unsized` as either a graph traversal or fix-point analysis](https://github.com/servo/rust-bindgen/issues/768).
* [less-easy] [rust-bindgen: Rewrite `can_derive_debug` as either a graph traversal or fix-point analysis](https://github.com/servo/rust-bindgen/issues/767).
* [less-easy] [rust-bindgen: Rewrite `can_derive_copy[_in_array]` as either a graph traversal or fix-point analysis](https://github.com/servo/rust-bindgen/issues/766).
* [less-easy] [rust-bindgen: Rewrite `has_vtable` checks as either graph traversal or fix-point analysis](https://github.com/servo/rust-bindgen/issues/765).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

94 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-06-19..2017-06-26

* [MIR dataflow for borrows](https://github.com/rust-lang/rust/pull/39409) (this broke nightly, alas)
* [make `break` break just the loop, not type inference](https://github.com/rust-lang/rust/pull/42634) (but break clippy in the process...)
* [enable `#[thread_local]` for Windows](https://github.com/rust-lang/rust/pull/42687)
* [`SyncSender` now implements `Sync`](https://github.com/rust-lang/rust/pull/42397)
* [integrate jobserver for parallel codegen](https://github.com/rust-lang/rust/pull/42682)
* [`compile_error!("...")` macro](https://github.com/rust-lang/rust/pull/42620)
* [implement `Display`, `Debug` for *`Guard` types](https://github.com/rust-lang/rust/pull/42822)
* [`wasm32-experimental-emscripten` target](https://github.com/rust-lang/rust/pull/42571)
* [more readable multiline message for `assert_eq!(..)`](https://github.com/rust-lang/rust/pull/42541)
  (also update [test infrastructure](https://github.com/rust-lang/cargo/pull/4181)
  and [cargo](https://github.com/rust-lang/cargo/pull/4196) for multiline messages)
* [allocator integration](https://github.com/rust-lang/rust/pull/42313)
* [fix memory eating bug on name resolution](https://github.com/rust-lang/rust/pull/42728)
* [avoid exponential blowup in `is_representable`](https://github.com/rust-lang/rust/pull/42751)
* [cherrypick LLVM stack coloring improvement](https://github.com/rust-lang/rust/pull/42750)
* [avoid inlining unwind calls](https://github.com/rust-lang/rust/pull/42771)
* [color for rustbuild errors](https://github.com/rust-lang/rust/pull/42804)

## New Contributors

* Casey Rodarmor
* Chris MacNaughton
* Giles Cope
* Leonardo Yvens
* Nick Whitney
* slo
* Squirrel

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Experimentally add coroutines to Rust](https://github.com/rust-lang/rfcs/pull/2033).
* [disposition: merge] [Tiered browser support policy for Rust's web content](https://github.com/rust-lang/rfcs/pull/1985).
* [disposition: merge] [Specify `#[repr(transparent)]`](https://github.com/rust-lang/rfcs/pull/1758).
* [disposition: merge] [Conversions from `&mut T` to `&Cell<T>`](https://github.com/rust-lang/rfcs/pull/1789).
* [disposition: merge] [Stabilize drop order](https://github.com/rust-lang/rfcs/pull/1857).
* [disposition: postpone] [Allow `extern crate` to take a list of crates](https://github.com/rust-lang/rfcs/pull/1875).

## New RFCs

* [Add `is_aligned` intrinsic](https://github.com/rust-lang/rfcs/pull/2043).
* [Put the RFCs repo under license terms](https://github.com/rust-lang/rfcs/pull/2044).
* [Allow a break not only out of `loop`, but of labelled blocks with no loop](https://github.com/rust-lang/rfcs/pull/2046).
* [Extend Rust target specification to follow more closely LLVM triple specification](https://github.com/rust-lang/rfcs/pull/2048).
* [Zero-Sized References](https://github.com/rust-lang/rfcs/pull/2040).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

The RFC style is now the default style in Rustfmt - try it out and let us know what you think!

Issues in final comment period:

* [Combining openings and closings](https://github.com/rust-lang-nursery/fmt-rfcs/issues/61)

An interesting issue:

* [Define short](https://github.com/rust-lang-nursery/fmt-rfcs/issues/47)

Good first issues:

We're happy to mentor these, please reach out to us in #rust-style if you'd like to get involved

* [paths](https://github.com/rust-lang-nursery/fmt-rfcs/issues/69)
* [simple expressions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/68)
* [assignment and assignment operators](https://github.com/rust-lang-nursery/fmt-rfcs/issues/67)
* [control flow](https://github.com/rust-lang-nursery/fmt-rfcs/issues/62)

# Upcoming Events

* [Jun 28. Boston Rust - Tutorial Bug-fixing Hackathon](https://www.meetup.com/BostonRust/events/240244837/).
* [Jun 28. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/240365553/).
* [Jun 28. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jun 28. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jun 29. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Jun 29. Rust  Durham, NC - Welcome to Rust! Introductions and Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/240942367/).
* [Jun  3. Rust Prague Meetup](https://www.meetup.com/rust-prague/events/240884817/).
* [Jul  4. Rust Utrecht - Rust Workshop](https://www.meetup.com/Rust-Utrecht/events/240660834/).
* [Jul  5. Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmywkbhb/).
* [Jul  5. Rust User Group Cologne - Live Coding](https://www.meetup.com//RustCologne/events/240619280/).
* [Jul  5. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jul  5. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jul  7. Rust Toronto - Game Development in Rust](https://www.meetup.com/Rust-Toronto/events/240585994/).
* [Jul 10. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/240751286/).
* [Jul 12. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/241046172/).
* [Jul 12. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jul 12. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jul 13. Columbus Rust Society - Monthly Meetup](https://www.meetup.com/columbus-rs/events/240698982/).
* [Jul 13. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Software Engineer - Remote working available](http://www.headresourcing.com/software-engineer-c-c-rust-remote-working-available-bbbh28438-1496919540).
* [Senior Research Engineer - Servo at Mozilla](https://careers.mozilla.org/position/gh/727971).
* [Tor: Summer 2017 Internship to Create a Bridge Bandwidth Scanner](https://blog.torproject.org/blog/summer-2017-internship-create-bridge-bandwidth-scanner).
* Student Research Assistant for developing Clippy in Karlsruhe (contact oliver.schneider \at kit.edu).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Regarding the C++ discussion, when I started programming the only viable oss version control system was cvs. It was horrible, but better than nothing. Then subversion was created and it was like a breath of fresh air, because it did the same thing well. Then alternatives exploded and among them git emerged as this amazing, amazing game-changer because it changed the whole approach to version control, enabling amazing things.
>
> To me, Rust is that git-like game-changer of systems programming languages because it changes the whole approach, enabling amazing things.


— [Nathan Stocks on TRPLF](https://users.rust-lang.org/t/blog-why-not-to-use-rust/11388/13).

Thanks to [Aleksey Kladov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/412) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
