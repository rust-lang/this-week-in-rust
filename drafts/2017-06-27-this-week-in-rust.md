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

* [Announcing the Talk Help section](https://users.rust-lang.org/t/announcing-the-talk-help-section/11461). The "Talk Help" section in [TRPLF](https://users.rust-lang.org/) is for discussing conference or meetup talks in all stages, from idea, over proposal to the post-talk feedback.
* [A Rust view on Effective Modern C++](https://mainisusuallyafunction.blogspot.in/2017/06/a-rust-view-on-effective-modern-c.html).
* [The highs and lows of Rust (2017)](https://www.jimmycuadra.com/posts/the-highs-and-lows-of-rust-2017/).
* [Get Started with Rust, WebAssembly, and Webpack](https://medium.com/@ianjsikes/get-started-with-rust-webassembly-and-webpack-58d28e219635).
* [Six months of rustc performance (2016-12 ~ 2017-05)](https://www.reddit.com/r/rust/comments/6iusjx/six_months_of_rustc_performance_201612_201705/).
* [Looping on a member variable without mutably borrowing self](http://blog.ssokolow.com/archives/2017/06/23/rust-looping-on-a-member-variable-without-mutably-borrowing-self/).
* [Implementing Huffman algorithm in Rust](http://sireliah.com/niusy/rust_huffman/).
* [A whirlwind tour of rustdoc](https://quietmisdreavus.net/code/2017/06/21/a-whirlwind-tour-of-rustdoc/).
* [Tock OS: RustConf tutorial preview and SITP retreat](https://www.tockos.org/blog/2017/talking-tock-22/).
* [rustup 1.5.0](https://users.rust-lang.org/t/rustup-1-5-0-released/11529) is released with support for 'rust-toolchain' file.
* [This week in Rust docs 61](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-61).
* [This week in Redox 23](https://redox-os.org/news/this-week-in-redox-23/).
* [GSoC project: Making Redox self-hosting - status report 1](https://redox-os.org/news/gsoc-self-hosting-1/).
* [podcast] [Request for Explanation #0 - What the hell](https://request-for-explanation.github.io/podcast/ep0-what-the-hell/index.html). This week's topic is [RFC 2005](https://github.com/rust-lang/rfcs/blob/master/text/2005-match-ergonomics.md) "Match Ergonomics Using Default Binding Modes".

# Crate of the Week

This week's crate is [include_dir](https://crates.io/crates/include_dir), a crate that lets you include entire directory contents in your binary – like `include_str!`, but on steroids. Thanks to [Michael Bryan](https://users.rust-lang.org/u/Michael-F-Bryan) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

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
