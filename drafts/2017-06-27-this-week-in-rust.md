Title: This Week in Rust 188
Number: 188
Date: 2017-06-27
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

* [Rusoto](https://github.com/rusoto/rusoto), an AWS SDK, has a [codegen walkthrough](https://matthewkmayer.github.io/blag/public/post/rusoto-codegen/) that follows code generation from JSON files to Rust code.  [Codegen walkthrough, part two](https://matthewkmayer.github.io/blag/public/post/rusoto-codegen-part-two/).

# Crate of the Week

This week's crate is [strum](https://crates.io/crates/strum), a crate that allows you to derive stringify and parse operations for your enums. Thanks to [lucab](https://users.rust-lang.org/u/lucab) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Guard types should implement more traits](https://github.com/rust-lang/rust/issues/24372).
* [walkdir: Add Error docs to methods that return Result](https://github.com/BurntSushi/walkdir/issues/24).
* [walkdir: Default to generating constified enums, rather than generating Rust enums](https://github.com/servo/rust-bindgen/issues/758).
* [walkdir: Change OsString args in sort_by to OsStr](https://github.com/BurntSushi/walkdir/issues/44).
* [walkdir: Remove re-export of is_same_file](https://github.com/BurntSushi/walkdir/issues/43).
* [walkdir: Document why unwraps won't fail](https://github.com/BurntSushi/walkdir/issues/42).
* [walkdir: Make skip_current_dir and filter_entry inherent methods](https://github.com/BurntSushi/walkdir/issues/40).
* [walkdir: Link references to std in docs](https://github.com/BurntSushi/walkdir/issues/39).
* [walkdir: Add build badges to Cargo.toml](https://github.com/BurntSushi/walkdir/issues/35).
* [walkdir: Implement Debug for WalkDir, Iter and IterFilterEntry](https://github.com/BurntSushi/walkdir/issues/34).
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

* Marco Castelluccio
* Thomas Lively
* Wonwoo Choi

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1974: Prepare global allocators for stabilization](https://github.com/rust-lang/rfcs/pull/1974).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: postpone] [Allow `extern crate` to take a list of crates](https://github.com/rust-lang/rfcs/pull/1875).
* [disposition: merge] [Stabilize drop order](https://github.com/rust-lang/rfcs/pull/1857).
* [disposition: merge] [Specify `#[repr(transparent)]`](https://github.com/rust-lang/rfcs/pull/1758).
* [disposition: merge] [Conversions from `&mut T` to `&Cell<T>`](https://github.com/rust-lang/rfcs/pull/1789).
* [disposition: merge] [Tiered browser support policy for Rust's web content](https://github.com/rust-lang/rfcs/pull/1985).

## New RFCs

* [Experimentally add coroutines to Rust](https://github.com/rust-lang/rfcs/pull/2033).
* [Add a try_with method to LocalKey, replacing the existing but unstable state method](https://github.com/rust-lang/rfcs/pull/2030)

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

We're making good progress and the style is coming together. If you want to see the style in practice, check out [our example](https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/example/lists.rs) or use the [Integer32 Playground](https://play.integer32.com/) and select 'Proposed RFC' from the 'Format' menu. Be aware that implementation is work in progress.

Issues in final comment period:

* [[macro_use] on the same line as crate](https://github.com/rust-lang-nursery/fmt-rfcs/issues/83)
* [trait bounds](https://github.com/rust-lang-nursery/fmt-rfcs/issues/80)
* [Specify rules for breaking long `where` conditions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/75)
* [Single-line `where`](https://github.com/rust-lang-nursery/fmt-rfcs/issues/74)
* [Combining openings and closings](https://github.com/rust-lang-nursery/fmt-rfcs/issues/61)

Good first issues:

We're happy to mentor these, please reach out to us in #rust-style if you'd like to get involved

* [paths](https://github.com/rust-lang-nursery/fmt-rfcs/issues/69)
* [simple expressions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/68)
* [assignment and assignment operators](https://github.com/rust-lang-nursery/fmt-rfcs/issues/67)
* [control flow](https://github.com/rust-lang-nursery/fmt-rfcs/issues/62)

# Upcoming Events

* [Jun 21. Rust Meetup Dresden](https://www.meetup.com/Mozilla-Community-Dresden/events/240188745/).
* [Jun 21. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jun 21. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jun 24. RainOfRust Camp Ahmedabad, Gujarat](https://reps.mozilla.org/e/rainofrust-camp-ahmedabad-gujarat/).
* [Jun 24. Rust Workshop Bangalore - Rain of Rust](https://reps.mozilla.org/e/rust-workshop-rain-of-rust/).
* [Jun 27. Rust Zurich - June Meetup](https://www.meetup.com/de-DE/Rust-Zurich/events/240752079/).
* [Jun 27. Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/240812524/).
* [Jun 27. Let's Rust - Hyderabad](https://reps.mozilla.org/e/let-s-rust/).
* [Jun 27. RainOfRust Camp Patan, Gujarat](https://reps.mozilla.org/e/rainofrust-camp-patan-gujarat/).
* [Jun 28. Boston Rust - Tutorial Bug-fixing Hackathon](https://www.meetup.com/BostonRust/events/240244837/).
* [Jun 28. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/240365553/).
* [Jun 28. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jun 28. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jun 29. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Jun 29. Rust  Durham, NC - Welcome to Rust! Introductions and Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/240942367/).
* [Jun  3. Rust Prague Meetup](https://www.meetup.com/rust-prague/events/240884817/).
* [Jul  4. Rust Utrecht - Rust Workshop](https://www.meetup.com/Rust-Utrecht/events/240660834/).
* [Jul  5. Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmywkbhb/).
* [Jul  5. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jul  5. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Research Engineer - Servo at Mozilla](https://careers.mozilla.org/position/gh/727971).
* [Tor: Summer 2017 Internship to Create a Bridge Bandwidth Scanner](https://blog.torproject.org/blog/summer-2017-internship-create-bridge-bandwidth-scanner).
* Student Research Assistant for developing Clippy in Karlsruhe (contact oliver.schneider \at kit.edu).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

<pre>impl<T> Clone for T {
  fn clone(&self) -> T {
    unsafe { std::ptr::read(self) }
  }
}</pre>

— [@horse_rust on twitter](https://twitter.com/horse_rust/status/876205034999996417).

Thanks to [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/407) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
