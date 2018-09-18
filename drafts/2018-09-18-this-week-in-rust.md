Title: This Week in Rust 252
Number: 252
Date: 2018-09-18
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

This week's crate is [cargo-src](https://crates.io/crates/cargo-src), a Rust source browser with syntax highlighting, jump to def, smart search and much more. Thanks to [mark-i-m](https://users.rust-lang.org/t/crate-of-the-week/2704/452) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [auto_impl has a few issues for beginners interested in working with the new proc macro API](https://users.rust-lang.org/t/twir-call-for-participation/4821/204).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

137 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-09-03..2018-09-10

* [introduce Custom Test Frameworks](https://github.com/rust-lang/rust/pull/53410)
* [ThinLTO: don't keep files open after mmaping them](https://github.com/rust-lang/rust/pull/53962)
* [rustc: prepare the `atomics` feature for wasm](https://github.com/rust-lang/rust/pull/53878)
* [if- and while-let-chains, take 2 - edition changes](https://github.com/rust-lang/rust/pull/53854)
* [resolve: relax shadowing restrictions on macro-expanded macros](https://github.com/rust-lang/rust/pull/53778)
* [`proc_macro::Group::span_open` and `span_close`](https://github.com/rust-lang/rust/pull/53902)
* [fix incorrect outer function type parameter message](https://github.com/rust-lang/rust/pull/53960)
* [fix `is_non_exhaustive` confusion between structs and enums](https://github.com/rust-lang/rust/pull/53721)
* [rewrite `precompute_borrows_out_of_scope` for fewer hash table lookups](https://github.com/rust-lang/rust/pull/53942)
* [make loop detector only consider reachable memory](https://github.com/rust-lang/rust/pull/52626)
* [miri engine: make sure we do not copy unsized data](https://github.com/rust-lang/rust/pull/53883)
* [optimize miri checking of integer array/slices](https://github.com/rust-lang/rust/pull/53903)
* [NLL: do not propagate closure requirements if we can prove them locally](https://github.com/rust-lang/rust/pull/53745)
* [NLL: teach SCC about `'static`](https://github.com/rust-lang/rust/pull/53327)
* [implement Unpin for Box, Rc, and Arc](https://github.com/rust-lang/rust/pull/53874)
* [add trim_start, trim_end etc.; deprecate trim_left, trim_right, etc. in future](https://github.com/rust-lang/rust/pull/52994)
* [allow to check if sync::Once is already initialized](https://github.com/rust-lang/rust/pull/53027)
* [stabilize `#[panic_handler]`](https://github.com/rust-lang/rust/pull/51366)
* [rustdoc: show trait impl docs](https://github.com/rust-lang/rust/pull/51885)
* [rustbuild: tweak LLVM distribution layout](https://github.com/rust-lang/rust/pull/53955)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Amend RFC 2175 to support for loops and leading vert](https://github.com/rust-lang/rfcs/pull/2530).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Deny the `overflowing_literals` lint for the 2018 edition](https://github.com/rust-lang/rfcs/pull/2438).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Fix camel case type warning for types with trailing underscores](https://github.com/rust-lang/rust/pull/54101).
* [disposition: merge] [Limit the promotion of const fns to the libstd and the `rustc_promotable` attribute](https://github.com/rust-lang/rust/pull/53851).
* [disposition: merge] [Support an explicit annotation for marker traits](https://github.com/rust-lang/rust/pull/53693).

## New RFCs

* [RFC: Elide array size](https://github.com/rust-lang/rfcs/pull/2545).
* [Make the turbofish syntax redundant](https://github.com/rust-lang/rfcs/pull/2544).
* [syntactic sugar for `EnumVariant(())`](https://github.com/rust-lang/rfcs/issues/2543).
* [Provide the llvm.is.constant/__builtin_constant_p intrinsic](https://github.com/rust-lang/rfcs/issues/2542).
* [Use `T: ToString` for `thread::Builder::name`](https://github.com/rust-lang/rfcs/pull/2541).
* [Add Duration::ZERO constant](https://github.com/rust-lang/rfcs/issues/2540).
* [#[cfg_attr] expanding to multiple attributes](https://github.com/rust-lang/rfcs/pull/2539).
* [Lint for function call in `unwrap_or(..)` parameter](https://github.com/rust-lang/rfcs/issues/2536).
* [RFC: Or patterns, i.e `Foo(Bar(x) | Baz(x))`](https://github.com/rust-lang/rfcs/pull/2535).
* [RFC: Write References for Direct and Partial Initialization using &out T and &uninit T](https://github.com/rust-lang/rfcs/pull/2534).
* [Keeping Secrets in Rust](https://github.com/rust-lang/rfcs/issues/2533).
* [RFC: Associated type defaults and Default groups](https://github.com/rust-lang/rfcs/pull/2532).
* [RFC: Hidden trait implementations](https://github.com/rust-lang/rfcs/pull/2529).
* [Type-changing struct update syntax](https://github.com/rust-lang/rfcs/pull/2528).
* [Support underscores as constant names](https://github.com/rust-lang/rfcs/pull/2526).
* [RFC: Permit _ in type aliases](https://github.com/rust-lang/rfcs/pull/2524).

# Upcoming Events

### Online

* [Sep 19. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Sep 25. Rust Community Content Subteam Meeting at channel #rust-community](irc://irc.mozilla.org/rust-community).
* [Sep 26. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Sep 26. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Europe

* [Sep 14. Rome, IT - Rust Rome Meetup](https://www.meetup.com/it-IT/Rust-Roma/events/254404386/).
* [Sep 18. Amsterdam, NL - Amsterdam Rust Meetup - Concurrency fundamentals, Tokio & WebAssembly](https://www.meetup.com/Rust-Amsterdam/events/253425558).
* [Sep 18. Rapperswil-Jona, CH - Rapperswil-Jona, Zürichsee Meetup - Looking for a speaker](https://www.meetup.com/de-DE/Rust-Zurich/events/251682152/).
* [Sep 19. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/253541005/).
* [Sep 20. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxmbbc/).

### North America

* [Sep 13. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxmbrb/).
* [Sep 13. San Diego, US - San Diego Rust September Meetup - WASM, "failure" library, or ???](https://www.meetup.com/San-Diego-Rust/events/253862312/).
* [Sep 13. Utah, US - Utah Rust monthly meetup](https://www.meetup.com/utahrust/events/253965052/).
* [Sep 16. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxmbvb/).
* [Sep 17. Boston, US - September Meetup at VMware](https://www.meetup.com/BostonRust/events/254400823/).
* [Sep 18. Denver, US - Denver Rust Meetup](https://www.meetup.com/Rust-Boulder-Denver/events/254386309/).
* [Sep 19. Vancouver, CA - Vancouver Rust meetup - Study/Hack/Hang-out](https://www.meetup.com/Vancouver-Rust/events/dqldspyxmbzb/).
* [Sep 20. Arlington, US - Rust DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/253787454).
* [Sep 23. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxmbfc/).
* [Sep 24. Durham, US - Triangle Rustaceans](https://www.meetup.com/triangle-rustaceans/events/mfglwpyxmbgc/).
* [Sep 25. Dallas, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxmbhc/).
* **[Oct 19 & 20. Ann Arbor, US - Rust Belt Rust 2018](https://rust-belt-rust.com/).**

### South America

* [Sep 15. Sao Paulo, BR - Rust Sao Paulo - Meetup](https://www.meetup.com/Rust-Sao-Paulo-Meetup/events/253842754/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at Parity, Berlin, DE](https://paritytech.io/jobs/).
* [Software Engineer at VMRay, Bochum, DE](https://careers.vmray.com/apply-software-engineer-rust-en/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Bare Metal Attracts Rust

– [Sven Gregori on Hackaday](https://hackaday.com/2018/09/08/pun-intended-bare-metal-attracts-rust/).

Thanks to [llogiq](https://users.rust-lang.org/u/llogiq) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
