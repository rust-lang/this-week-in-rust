Title: This Week in Rust 300
Number: 300
Date: 2019-08-20
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

* 🎈🎉 [Announcing Rust 1.37.0](https://blog.rust-lang.org/2019/08/15/Rust-1.37.0.html). 🎉🎈
* [Announcing async-std beta: an async port of the Rust standard library](https://async.rs/blog/announcing-async-std/).
* [How Rust optimizes async/await: Part 1](https://tmandry.gitlab.io/blog/posts/optimizing-await-1/).
* [System 76 releases new GTK firmware manager written in Rust](https://blog.system76.com/post/187072707563/the-new-firmware-manager-updating-firmware-across).
* [Writing a Linux kernel module in Rust](https://github.com/lizhuohua/linux-kernel-module-rust).
* [Introducing the Rust Game Development Working Group](https://rust-gamedev.github.io/2019/08/18/introducing-the-rust-game-development-working-group).
* [Steam wishlist announced for Way of Rhea, a sidescrolling puzzle game written in Rust](https://store.steampowered.com/app/1110620/Way_of_Rhea/).
* [Visual embedded Rust programming with Visual Studio Code](https://medium.com/@ly.lee/visual-embedded-rust-programming-with-visual-studio-code-1bc1262e398c?source=friends_link&sk=222de63e45993aacd0db5a2e4b1f33c7).
* [How to diagnose async apps with `tracing`](https://tokio.rs/blog/2019-08-tracing/).
* [Using C libraries in Rust: A practical guide to FFI using bindgen](https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a).
* [Pre-/Post-conf events: Sustainable 🚄 train travels from and to RustFest Barcelona](https://blog.rustfest.eu/pre-post-conf-events-sustainable-train-travels).
* [`<_>::v::<_>` - A fun little piece of Rust artwork](https://chrismorgan.info/blog/rust-artwork-owl/).

# Crate of the Week

This week's crate is [async-std](https://crates.io/crates/async-std), a library with async variants of the standard library's IO etc.

Thanks to [mmmmib](https://users.rust-lang.org/t/crate-of-the-week/2704/602) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [rustc-guide: Make HIR identifiers table more discoverable](https://github.com/rust-lang/rustc-guide/issues/420).
* [rustc-guide: Explain what interning means](https://github.com/rust-lang/rustc-guide/issues/419).
* [rustc-guide: Terms used before being explained](https://github.com/rust-lang/rustc-guide/issues/418).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

268 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-08-12..2019-08-19

* [Hash the remapped sysroot instead of the original](https://github.com/rust-lang/rust/pull/63505)
* [Make sure that all file loading happens via SourceMap](https://github.com/rust-lang/rust/pull/63525)
* [syntax: Account for CVarArgs being in the argument list](https://github.com/rust-lang/rust/pull/63459)
* [Remove redundant `ty` fields from `mir::Constant` and `hair::pattern::PatternRange`](https://github.com/rust-lang/rust/pull/63495)
* [resolve: Remove remaining special cases from built-in macros](https://github.com/rust-lang/rust/pull/63449)
* [resolve: Properly integrate derives and `macro_rules` scopes](https://github.com/rust-lang/rust/pull/63667)
* [Point at the right enclosing scope when using `await` in non-async fn](https://github.com/rust-lang/rust/pull/63509)
* [typeck: Prohibit RPIT types that inherit lifetimes](https://github.com/rust-lang/rust/pull/62849)
* [Handle elision in async fn correctly](https://github.com/rust-lang/rust/pull/63499)
* [When needing type annotations in local bindings, account for impl Trait and closures](https://github.com/rust-lang/rust/pull/63507)
* [Improved error message for break in async block](https://github.com/rust-lang/rust/pull/63659)
* [Suggest Rust 2018 on `<expr>.await` with no such field](https://github.com/rust-lang/rust/pull/63539)
* [Crank up invalid value lint](https://github.com/rust-lang/rust/pull/63657)
* [Refactor Miri ops (unary, binary) to have more types](https://github.com/rust-lang/rust/pull/63658)
* [Do not generate allocations for zero sized allocations](https://github.com/rust-lang/rust/pull/63635)
* [Feature gate 'yield $expr?' pre-expansion](https://github.com/rust-lang/rust/pull/63545)
* [Provide map_ok and map_err method for Poll<Option<Result<T, E>>>](https://github.com/rust-lang/rust/pull/63512)
* [Implement `Clone`, `Display` for `ascii::EscapeDefault`](https://github.com/rust-lang/rust/pull/63421)
* [Add APIs for uninitialized `Box`, `Rc`, and `Arc` (Plus `get_mut_unchecked`)](https://github.com/rust-lang/rust/pull/62451)
* [Reduce the genericity of closures in the iterator traits](https://github.com/rust-lang/rust/pull/62429)
* [Add custom `nth_back` for `Chain`](https://github.com/rust-lang/rust/pull/60492)
* [`cargo install`: Remove orphaned executables](https://github.com/rust-lang/cargo/pull/7246)

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

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Clarify `Box<T>` representation and its use in FFI](https://github.com/rust-lang/rust/pull/62514).
* [disposition: merge] [Constify LinkedList new function](https://github.com/rust-lang/rust/pull/63684).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Africa

* [Sep  4. Johannesburg, ZA - Johannesburg Rust Meetup - informal discussions on topics related to the language](https://www.meetup.com/Johannesburg-Rust-Meetup/events/dgqmbryzmbgb/).

### Asia Pacific

* [Aug 24. Chennai, IN - Rust Chennai - Monthly meetup - August](https://www.meetup.com/mad-rs/events/264125149).
* [Aug 27. Seoul, KR - Seoul Rust Meetup, Hapjeong](https://www.meetup.com/Rust-Seoul-Meetup/events/nxkdfryzlbkc/).
* [Sep  2. Auckland, NZ - Rust AKL - Introduction to Rust (session 1 of 3)](https://www.meetup.com/rust-akl/events/259481026/).

### Europe

* [Aug 26. Thessaloniki, GR - Rust + GNOME Workshop at GUADEC](https://wiki.gnome.org/GUADEC/2019/Hackingdays/RustGtkGstWorkshop).
* [Aug 27. London, GB - Rust London User Group - Rust Hack n Learn + Lightning Talks Evening #16](https://www.meetup.com/Rust-London-User-Group/events/264000041/).
* [Aug 27. Thessaloniki, GR - Rust + GNOME BoF at GUADEC](https://wiki.gnome.org/GUADEC/2019/Hackingdays/RustBoF).
* [Aug 28. Copenhagen, DK - Copenhagen Rust Hack Night #17](https://cph.rs/).
* [Aug 29. Zurich, CH - Rust Zurich - August Community Meetup](https://www.meetup.com/Rust-Zurich/events/263756588/).
* [Sep  4. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryzmbgb/).

### North America

* [Aug 26. Durham, NC, US - Triangle Rustaceans - Build a syslog server with mio](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzlbjc/).
* [Aug 27. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyzlbkc/).
* [Aug 27. Chicago, IL, US - Chicago Rust Meetup - Macros Rule! A Dive Into Rust's Syntax Extension Toolbox](https://www.meetup.com/Chicago-Rust-Meetup/events/263849534).
* [Aug 28. Ann Arbor, MI, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/zdfscryzlblc/).
* [Aug 28. Mesa, AZ, US - Desert Rust - Rust: Frontend Web](https://www.meetup.com/Desert-Rustaceans/events/lftjxqyzlblc/).
* [Sep  4. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryzmbgb/).
* [Sep  4. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyzmbgb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Software Engineer at ConsenSys R&D, Remote](https://consensys.net/open-roles/1792013/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> C++ being memory safe is like saying riding a motorcycle is crash safe.
>
> It totally is, if you happen to have the knowledge and experience to realize this is only true if you remember to put on body-armor, a helmet, a full set of leathers including gloves and reinforced boots, and then remember to operate the motorcycle correctly afterwards. In C/C++ though, that armor is completely 100% optional.

– [cyrusm on /r/rust](https://www.reddit.com/r/rust/comments/cseulx/is_rust_a_new_paradigmclass_of_programing/exeyibc)

Thanks to [Dmitry Kashitsyn](https://users.rust-lang.org/t/twir-quote-of-the-week/328/682) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
