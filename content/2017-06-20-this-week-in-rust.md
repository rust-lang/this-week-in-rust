Title: This Week in Rust 187
Number: 187
Date: 2017-06-20
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

* [RustConf 2017 tickets are now available](https://twitter.com/rustconf/status/874697180778565632).
* [encoding_rs, a new character encoding conversion library written in Rust, has landed in nightly Firefox](https://twitter.com/hsivonen/status/875043685544718336).
* [Bugs you'll probably only have in Rust](https://gankro.github.io/blah/only-in-rust/).
* [Why not to use Rust](https://llogiq.github.io/2017/06/16/no-rust.html).
* [Algorithms cookbook in Rust](https://github.com/EbTech/rust-algorithms).
* [Implementing cooperative multitasking in Rust](https://gmorenz.gitlab.io/coop.html).
* [Switching from C++ to Rust](http://psychopath.io/switching-from-c-to-rust/).
* [Graphics by squares: a gfx-rs tutorial](https://suhr.github.io/gsgt/).
* [Rustfmt releases](https://users.rust-lang.org/t/rustfmt-releases/11357). There are some significant changes happening to Rustfmt. Here is what you need to know.
* [System programming in Rust: beyond safety](https://blog.acolyer.org/2017/06/14/system-programming-in-rust-beyond-safety/).
* [Writing a LALR(1) parser generator in Rust](https://medium.com/@DmitrySoshnikov/rust-lalr-1-parser-generator-a623b64164e4).
* [hyper v0.11 is released](http://seanmonstar.com/post/161786147642/hyper-v011).
* [This week in Redox 22](https://www.redox-os.org/news/this-week-in-redox-22/).

# Crate of the Week

This week's crate is [include_dir](https://crates.io/crates/include_dir), a crate that lets you include entire directory contents in your binary – like `include_str!`, but on steroids. Thanks to [Michael Bryan](https://users.rust-lang.org/u/Michael-F-Bryan) for the suggestion!

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

122 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-06-12..2017-06-19

* [overflow-check `str` index by inclusive (`...`) ranges](https://github.com/rust-lang/rust/pull/42428)
* [float `min`/`max` is now pure Rust](https://github.com/rust-lang/rust/pull/42430)
* [`Ord::`{`min`, `max`}](https://github.com/rust-lang/rust/pull/42496)
* [allocation-less `Display` for `Path` and `OsStr`](https://github.com/rust-lang/rust/pull/42613)
* [suggest `==` on inadvertent assignment in `if` conditions](https://github.com/rust-lang/rust/pull/42649)
* [omit trait errors implied by other errors](https://github.com/rust-lang/rust/pull/41840)
* [save-analysis is now JSON only](https://github.com/rust-lang/rust/pull/42650)
* [`collections` is back](https://github.com/rust-lang/rust/pull/42720)
* [fix type inference ICE due to missing obligations](https://github.com/rust-lang/rust/pull/42659)
* [fix fn pointer coercion ICE](https://github.com/rust-lang/rust/pull/42735)
* [use custom cargo/rustc paths when parsing flags](https://github.com/rust-lang/rust/pull/42695)
* [cargo stores API tokens in separate, user-private file](https://github.com/rust-lang/cargo/pull/3978)

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
