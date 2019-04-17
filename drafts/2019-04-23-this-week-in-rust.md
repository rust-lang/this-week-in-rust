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

# Crate of the Week

This week's crate is [sendfd](https://github.com/Standard-Cognition/sendfd), a simple way to send file descriptors over UNIX sockets. Thanks to [Léo Gaspard](https://users.rust-lang.org/t/crate-of-the-week/2704/514) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* The [CFP for Colorado Gold Rust](https://cfp.cogoldrust.com/events/cogoldrust-2019) is open now. The organizers are also looking for volunteers to help people draft talk proposals. If you can help out [send them an email](mailto:coloradogoldrust@gmail.com) or DM them on Twitter at [@COGoldRust](https://twitter.com/cogoldrust).
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

241 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-04-08..2019-04-15

* [save-analysis: Pull associated type definition using `qpath_def`](https://github.com/rust-lang/rust/pull/59894)
* [Recover from missing semicolon based on the found token](https://github.com/rust-lang/rust/pull/59866)
* [Increase `Span` from 4 bytes to 8 bytes](https://github.com/rust-lang/rust/pull/59693)
* [proc_macro: Stop using LEB128 for RPC](https://github.com/rust-lang/rust/pull/59820)
* [Remove duplicated redundant spans](https://github.com/rust-lang/rust/pull/59896)
* [Mark variables captured by reference as mutable correctly](https://github.com/rust-lang/rust/pull/59708)
* [Suggest removing `?` to resolve type errors](https://github.com/rust-lang/rust/pull/59859)
* [Make duplicate matcher bindings a hard error](https://github.com/rust-lang/rust/pull/59858)
* [Improvement for comparision against fn](https://github.com/rust-lang/rust/pull/59798)
* [Suggest importing macros from the crate root](https://github.com/rust-lang/rust/pull/59784)
* [Function arguments should never get promoted](https://github.com/rust-lang/rust/pull/59724)
* [miri: Implement non-deterministc mode](https://github.com/rust-lang/miri/pull/683)
* [miri: Unsized locals and by-value dyn traits](https://github.com/rust-lang/rust/pull/59780)
* [std: Add {`read`, `write`}`_vectored` for more types](https://github.com/rust-lang/rust/pull/59852)
* [`MaybeUninit`: Remove deprecated functions](https://github.com/rust-lang/rust/pull/59912)
* [Stabilize the `alloc` crate](https://github.com/rust-lang/rust/pull/59675)
* [Improve warning in `cargo new` with parse error](https://github.com/rust-lang/cargo/pull/6839)
* [rustup.rs: Less copying during dist installation](https://github.com/rust-lang/rustup.rs/pull/1744)
* [rustup.rs: Shell completions for Cargo](https://github.com/rust-lang/rustup.rs/pull/1646)
* [Add --path option to 'rustup override set'](https://github.com/rust-lang/rustup.rs/pull/1524)

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
