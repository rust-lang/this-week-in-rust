Title: This Week in Rust 283
Number: 283
Date: 2019-04-23
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
* [Building a plugin system using Rust and Wasmer](https://wiredforge.com/blog/wasmer-plugin-pt-1) ([part 2](https://wiredforge.com/blog/wasmer-plugin-pt-2))

# Crate of the Week

This week's crate is [color-backtrace](https://github.com/athre0z/color-backtrace), a crate to give panic backtraces more information (and some color, too). Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/518) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* The [CFP for Colorado Gold Rust](https://cfp.cogoldrust.com/events/cogoldrust-2019) is open now. The organizers are also looking for volunteers to help people draft talk proposals. If you can help out [send them an email](mailto:coloradogoldrust@gmail.com) or DM them on Twitter at [@COGoldRust](https://twitter.com/cogoldrust).
* The [CFP for "Everything in Rust" in COSCUP](https://blog.coscup.org/2019/04/2019-cfp-open.html#rust) is open now. Rust Taiwan Community is looking for speakers. COSCUP is one of the biggest open source conferences in Asia and takes place in Taipei, Taiwan.
* [discussion] [tetra: Is the OpenGL layer safe](https://github.com/17cupsofcoffee/tetra/issues/117)? Tetra is a simple 2D game framework written in Rust.
* [easy] [ruma-api-macros: Look into removing trait import names in generated code](https://github.com/ruma/ruma-api-macros/issues/16).
* [medium] [ruma-api-macros: Properly handle all possible run time panics](https://github.com/ruma/ruma-api-macros/issues/5).
* [hard] [ruma-api-macros: Figure out how to remove uses of Tokens::append_all](https://github.com/ruma/ruma-api-macros/issues/4).
* [org-rs: Implement parse_objects function](https://github.com/org-rs/org-rs/issues/8).
* [org-rs: Implement affiliated keywords](https://github.com/org-rs/org-rs/issues/11).
* [easy] [futures-jsonrpc: Test coverage](https://github.com/vlopes11/futures-jsonrpc/issues/1).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

221 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-04-15..2019-04-22

* [Implement event filtering for self-profiler](https://github.com/rust-lang/rust/pull/59915)
* [Continue evaluating after missing main](https://github.com/rust-lang/rust/pull/59903)
* [Point at try `?` on errors affecting the err match arm of the desugared code](https://github.com/rust-lang/rust/pull/60064)
* [Make const parameters enforce no variance constraints](https://github.com/rust-lang/rust/pull/60058)
* [save-analysis: Use serde instead of libserialize to dump JSON data](https://github.com/rust-lang/rust/pull/60053)
* [Fix ICE on const evaluation of const method](https://github.com/rust-lang/rust/pull/60048)
* [Specialize `nth_back()` for `Bytes`, `Fuse` and `Enumerate`](https://github.com/rust-lang/rust/pull/60023)
* [Fix the max value of `usize` on 16-bit platforms](https://github.com/rust-lang/rust/pull/60013)
* [Fix `LinkedList` invalidating mutable references](https://github.com/rust-lang/rust/pull/60072)
* [Allow multiple args to `dbg!(..)`](https://github.com/rust-lang/rust/pull/59826)
* [Add `must_use` annotations to `Result::is_ok` and `is_err`](https://github.com/rust-lang/rust/pull/59648)
* [chalk: Remove coinductive_traits from `ProgramEnvironment`](https://github.com/rust-lang/chalk/pull/213)
* [chalk: Simplify crate structure](https://github.com/rust-lang/chalk/pull/215)
* [cargo: Treat HTTP/2 stream errors as spurious network errors](https://github.com/rust-lang/cargo/pull/6861)
* [cargo: Validate registry token before operations that require it](https://github.com/rust-lang/cargo/pull/6854)
* [cargo: Pass `OsStr`/`OsString` args through to the process spawned by cargo run](https://github.com/rust-lang/cargo/pull/6849)
* [rustdoc: Remove `default` keyword from re-exported trait methods](https://github.com/rust-lang/rust/pull/59978)

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

* [disposition: postpone] [Simplify visibility grammar](https://github.com/rust-lang/rfcs/pull/2640).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [[Stabilization] Future APIs](https://github.com/rust-lang/rust/issues/59725).
* [disposition: merge] [Enable NLL migrate mode on the 2015 edition](https://github.com/rust-lang/rust/pull/59114).
* [disposition: merge] [Tracking issue for vectored IO support](https://github.com/rust-lang/rust/issues/58452).

## New RFCs

* [Discriminant bits](https://github.com/rust-lang/rfcs/pull/2684).

# Upcoming Events

### Africa

* [May  2. Johannesburg, ZA - Johannesburg meetup - Everybody Borrows](https://www.meetup.com/Johannesburg-Rust-Meetup/events/gpxrtqyzhbcb/).

### Asia Pacific

* [Apr 20. Beijing, CN - RustCon Asia](https://rustcon.asia/).
* [Apr 20. Taipei, TW - Rust Taiwan Meetup](https://www.facebook.com/events/400895290642737/).
* [Apr 24. Tokyo, JP - Tokyo Rust Meetup](https://rust.connpass.com/event/125666/).

### Europe

* [Apr 25. Toulouse, FR - Mon premier service web en Rust](https://www.meetup.com/Toulouse-Rust-Meetup/events/260218832).
* [Apr 25. Paris, FR - Rust Paris meetup #44](https://www.meetup.com/Rust-Paris/events/260443108/).
* [Apr 25. Brno, CZ - Rust Brno meetup](https://rust-brno.github.io/).
* [Apr 26. Stuttgart, DE - Rust Meetup #2](https://gettogether.community/rust-stuttgart/)
* [Apr 26. Berlin, DE - Oxidize Berlin Conference](https://oxidizeconf.com/).
* [Apr 30. London, UK - Rust London User Group - LDN Talks](https://www.meetup.com/Rust-London-User-Group/events/260565918/).
* [Apr 30. Vienna, AT - Rust Meetup](https://www.meetup.com/Rust-Vienna/events/260693863/).
* [May  1. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gkkttqyzhbcb/).
* [May  2. Munich, DE - Rust Munich - Rust libp2p](https://www.meetup.com/rust-munich/events/259984522/).
* [May  9. Wrocław, PL - Rust Wroclaw Meetup #10](https://www.meetup.com/Rust-Wroclaw/events/260858425/).

### North America

* [Apr 22. Durham, US - Triangle Rustaceans](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzgbdc/).
* [Apr 23. Chicago, US - Chicago Rust Meetup - Discussion: Better Method Chaining in Rust](https://www.meetup.com/Chicago-Rust-Meetup/events/260321118).
* [Apr 24. Sacramento, US - Hands-on Rust](https://www.meetup.com/Rust-Sacramento/events/260347016/).
* [Apr 24. Ann Arbor, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/vsncvqyzgbgc/).
* [Apr 24. Boston, US - Boston Rust Meetup at VMWare](https://www.meetup.com/BostonRust/events/259966076/).
* [Apr 25. San Francisco, US - WebAssembly SF - Let's talk about Rust and a microkernel @ Cloudflare](https://www.meetup.com/wasmsf/events/260288977/).
* [Apr 30. Dallas, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyzgbnc/).
* [May  1. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/hjrwvqyzhbcb/).
* [May  1. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/lgtvsqyzhbcb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at ANIXE, Wrocław, PL](https://anixe.bamboohr.co.uk/jobs/view.php?id=72).
* [Database Engine Developer at Parity, Berlin, DE](https://www.parity.io/jobs/#berlin-database-engine-developer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
