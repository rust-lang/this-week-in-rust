Title: This Week in Rust 245
Number: 245
Date: 2018-07-31
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

* [What is Rust 2018](https://blog.rust-lang.org/2018/07/27/what-is-rust-2018.html)?
* [Announcing the js-sys crate](https://rustwasm.github.io/2018/07/26/announcing-the-js-sys-crate.html).
* [Learn how to build OpenGL renderer with Rust](https://old.reddit.com/r/rust/comments/92abkg/rust_and_opengl_from_scratch_a_blog_post_series/).
* [A snapshot of Rust's popularity in July 2018](http://www.jonathanturner.org/2018/07/snapshot-of-rust-popularity.html).
* [Pointers are complicated, or: What's in a byte](https://www.ralfj.de/blog/2018/07/24/pointers-and-bytes.html)?
* [Version selection in Cargo](https://aturon.github.io/2018/07/25/cargo-version-selection/).
* [A "rustup target" example: Using a Mac to cross-compile Linux binaries](http://timryan.org/2018/07/27/cross-compiling-linux-binaries-from-macos.html).
* [Rust concurrency patterns: Natural born pipelines](https://medium.com/@polyglot_factotum/rust-concurrency-patterns-natural-born-pipelines-4d599e7612fc).
* [Build a sequence-based recommender system in Rust](https://maciejkula.github.io/2018/07/27/recommending-books-with-rust/).
* [Programming Servo: How to match](https://medium.com/programming-servo/programming-servo-how-to-match-b76c43f76fe6).
* [My experience with the Rust 2018 preview](https://boats.gitlab.io/blog/post/my-experience-with-rust-2018/).
* [Writing a front-end WebAssembly framework in Rust: lessons learned](https://medium.com/@robert.balicki_2494/writing-a-front-end-webassembly-framework-in-rust-lessons-learned-7cc48ed27d96).
* [2018 edition end of week post (2018-07-27)](https://internals.rust-lang.org/t/2018-edition-end-of-week-post-2018-07-27/8078).
* [The Embedded WG newsletter 8](https://internals.rust-lang.org/t/the-embedded-working-group-newsletter-8/8089).
* [Rust 2018 release schedule and extended beta](https://internals.rust-lang.org/t/rust-2018-release-schedule-and-extended-beta/8076).

# Crate of the Week

This week's crate is [Taizen](https://github.com/NerdyPepper/taizen), a wikipedia browser for your terminal. Thanks to [nasa42](https://users.rust-lang.org/t/crate-of-the-week/2704/419) for suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Help test out the 2018 module system changes](https://internals.rust-lang.org/t/help-test-out-the-2018-module-system-changes/8047).
* [exercism.io needs Rust mentors](https://users.rust-lang.org/t/exercism-io-needs-mentors/19222).
* [jsonwebtoken: Add ES* family](https://github.com/Keats/jsonwebtoken/issues/21).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

158 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-07-23..2018-07-31

* [try to fix an ICE](https://github.com/rust-lang/rust/pull/52673)
* [abort if a promoted fails to be const evaluable and its runtime checks didn't trigger](https://github.com/rust-lang/rust/pull/52571)
* [allow declaring existential types inside blocks](https://github.com/rust-lang/rust/pull/52645)
* [do not overwrite child def-id in place but rather remove/insert](https://github.com/rust-lang/rust/pull/52546)
* [format linker args in a way that works for gcc and ld](https://github.com/rust-lang/rust/pull/52654)
* [rustc: implement tokenization of nested items](https://github.com/rust-lang/rust/pull/52618)
* [buffer NLL errors](https://github.com/rust-lang/rust/pull/52566)
* [don't match on region kinds when reporting NLL errors](https://github.com/rust-lang/rust/pull/52617)
* [NLL: improve the "fully elaborated type" case in region errors](https://github.com/rust-lang/rust/pull/52648)
* [NLL: use better spans in some errors](https://github.com/rust-lang/rust/pull/52678)
* [NLL: make temp for each candidate in `match` arm](https://github.com/rust-lang/rust/pull/52733)
* [NLL: fix some things for bootstrap](https://github.com/rust-lang/rust/pull/52830)
* [suggest underscore when using dashes in crate name](https://github.com/rust-lang/rust/pull/52740)
* [suggest fix when encountering different mutability from impl to trait](https://github.com/rust-lang/rust/pull/52702)
* [do a basic sanity check for all constant values](https://github.com/rust-lang/rust/pull/51361)
* [tweak the raw_identifiers lints in 2018](https://github.com/rust-lang/rust/pull/52722)
* [change ManuallyDrop<T> to a lang item](https://github.com/rust-lang/rust/pull/52711)
* [don't use NonNull::dangling as sentinel value in Rc, Arc](https://github.com/rust-lang/rust/pull/52637)
* [add unaligned volatile intrinsics](https://github.com/rust-lang/rust/pull/52391)
* [`impl PartialEq+Eq for BuildHasherDefault`](https://github.com/rust-lang/rust/pull/52402)
* [`impl Executor for Box<E: Executor>`](https://github.com/rust-lang/rust/pull/52674)
* [`impl std::ops::Try for std::task::Poll`](https://github.com/rust-lang/rust/pull/52721)
* [`impl Send & Sync for JoinHandle`](https://github.com/rust-lang/rust/pull/52759)
* [make `memrchr` use `align_offset`](https://github.com/rust-lang/rust/pull/52744)
* [stablize Redox Unix Sockets](https://github.com/rust-lang/rust/pull/52656)
* [don't `format!()` string literals](https://github.com/rust-lang/rust/pull/52805)
* [`cargo -Zcompile-progress`: use the target name in the progress bar when building a test/binary](https://github.com/rust-lang/cargo/pull/5828)
* [rustdoc: rework how default passes are chosen](https://github.com/rust-lang/rust/pull/52751)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Closures Capture Disjoint Fields](https://github.com/rust-lang/rfcs/pull/2229).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for `:vis` macro matcher](https://github.com/rust-lang/rust/issues/41022).
* [disposition: merge] [Tracking issue for RFC 2093: Infer `T: 'x` outlives requirements on structs](https://github.com/rust-lang/rust/issues/44493).
* [disposition: merge] [Tracking issue for RFC 2151, Raw Identifiers](https://github.com/rust-lang/rust/issues/48589).
* [disposition: merge] [Tracking issue for `ToOwned::clone_into` (`toowned_clone_into`)](https://github.com/rust-lang/rust/issues/41263).
* [disposition: merge] [Calculate capacity when collecting into Option and Result](https://github.com/rust-lang/rust/pull/52910).

## New RFCs

* [Teach `concat!()` to join `[u8]` and byte `str`](https://github.com/rust-lang/rfcs/pull/2509).

# Upcoming Events

### Online

* [Aug  8. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Aug 14. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Aug 15. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Aug 15. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Africa

* [Aug  7. Johannesburg, ZA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxlbkb/).

### Europe

* [Aug  8. Berlin, DE - Binding to Rust from everything](https://www.meetup.com/Rust-Berlin/events/252872742/).
* [Aug  8. Berlin, DE - OpenTechSchool - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/xkdlvpyxlblb/).
* [Aug 10. Frankfurt, DE - Rhein-Main Rust Meetup (with Special Guest)](https://www.meetup.com/Rust-Rhein-Main/events/253311151).
* [Aug 16. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxlbvb/).

### North America

* [Aug  5. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxlbhb/).
* [Aug  8. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/dqldspyxlblb/).
* [Aug  9. Arlington, US - Rust DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/252742624).
* [Aug  9. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxlbmb/).
* [Aug  9. Lehi, UT - Utah Rust - Monthly Meeting #3](https://www.meetup.com/utahrust/events/252760018/).
* [Aug 12. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxlbqb/).
* [Aug 13. Seattle, US - Monthly Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/pkggvpyxlbrb/).
* **[Aug 17. Portland, US - RustConf 2018](http://rustconf.com/).**

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Engineer at Commure, Inc. (San Francisco, Boston, Montreal)](https://www.reddit.com/r/rust/comments/92e67g/commure_healthcare_software_startup_hiring_rust/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust is more restrictive, indeed. But only in the sense that a car with seatbelts is more restrictive than one without: both reach the same top speed, but only one of them will save you in a bad day 😊

– [Felix91gr on rust-users](https://users.rust-lang.org/t/which-language-gives-users-more-control-c-or-rust/19034/8).

Thanks to [Jules Kerssemakers](https://users.rust-lang.org/u/juleskers) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
