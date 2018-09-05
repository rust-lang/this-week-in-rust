Title: This Week in Rust 250
Number: 250
Date: 2018-09-04
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

* [A dynamic analysis framework for WebAssembly programs](https://github.com/danleh/wasabi).
* [greenwasm — an implementation of the WebAssembly spec in Rust](https://github.com/Kimundi/greenwasm).
* [Failsafe: A circuit breaker pattern implementation for rust](https://github.com/dmexe/failsafe-rs).
* [Statemaps in Rust - internal Joyent presentation from Bryan Cantrill](https://www.youtube.com/watch?v=aWbGPMxs0AM).
* [Feasible functors in Rust](https://varkor.github.io/blog/2018/08/28/feasible-functors-in-rust.html).
* [Rust pattern: Iterating an over a Rc<Vec<T>>](http://smallcultfollowing.com/babysteps/blog/2018/09/02/rust-pattern-iterating-an-over-a-rc-vec-t/).
* [Learning Generics in Rust - Andrew Johnson](https://tutorialedge.net/rust/learning-generics-in-rust/).
* [Primitives in Rust are Weird (and Cool)](https://speice.io/2018/09/primitives-in-rust-are-weird.html).
* [Why Rust Closures are (Somewhat) Hard](http://stevedonovan.github.io/rustifications/2018/08/18/rust-closures-are-hard.html).
* [Rustcast #7 - How to implement Iterator in Rust programming language](https://www.youtube.com/watch?v=pgFWz0jgqMU).
* [Blog post: Benchmarking a #rustlang web application](https://klausi.github.io/rustnish/2018/08/31/benchmarking-a-rust-web-application.html).
* [Experimental!: Gecko using WebRender running on Vulkan](https://github.com/szeged/webrender/issues/198#issuecomment-416635113).
* [Unsafe Code Guidelines Meetings](https://internals.rust-lang.org/t/unsafe-code-guidelines-meetings/8335).
* [Serde Deserialize This or That into u64](https://noyez.gitlab.io/post/2018-08-28-serilize-this-or-that-into-u64/).
* [The 10th Embedded WG Newsletter (and a new blog!)](https://rust-embedded.github.io/blog/2018-08-28-newsletter-10/).

# Crate of the Week

This week's crate is [cgroups](https://crates.io/crates/cgroups), a native Rust library for managing control groups under Linux. Thanks to [yoshuawuyts](https://users.rust-lang.org/t/crate-of-the-week/2704/450) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

*No issues were submitted for CfP this week.*

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

109 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-08-27..2018-09-03

* [enable ThinLTO with incremental compilation](https://github.com/rust-lang/rust/pull/53673)
* [build LLVM with ThinLTO enabled (2nd attempt)](https://github.com/rust-lang/rust/pull/53245)
* [update LLVM submodule](https://github.com/rust-lang/rust/pull/53611) (mainly WASM improvements)
* [ADD more Cortex-R targets](https://github.com/rust-lang/rust/pull/53679)
* [change the default linker of the ARM Cortex-M targets to rust-lld](https://github.com/rust-lang/rust/pull/53648)
* [begin preparation for Rust 2018](https://github.com/rust-lang/crates.io/pull/1467)
* [fix promotion stability hole in old borrowck](https://github.com/rust-lang/rust/pull/53699)
* [Miri engine cleanup](https://github.com/rust-lang/rust/pull/53671)
* [Miri refactor: Final round](https://github.com/rust-lang/rust/pull/53779)
* [use partial but correct vtable layout](https://github.com/rust-lang/rust/pull/53757)
* [replace `AccumulateVec` by `SmallVec`](https://github.com/rust-lang/rust/pull/53659)
* [`HybridIdxSet` tweaks](https://github.com/rust-lang/rust/pull/53656)
* [NLL: experiment with inverting liveness](https://github.com/rust-lang/rust/pull/53314)
* [fix NLL ICEs](https://github.com/rust-lang/rust/pull/53580)
* [set rustfix auto-applicability for a few lints](https://github.com/rust-lang/rust/pull/53655)
* [use `FxHash`{`Map`, `Set`} instead of the default `Hash`{`Map`, `Set`}](https://github.com/rust-lang/rust/pull/53472)
* [various small diagnostic and code clean up](https://github.com/rust-lang/rust/pull/53842)
* [save-analysis: record info for the types in `where` clauses](https://github.com/rust-lang/rust/pull/53838)
* [fix `u32` `steps_between` for 16-bit systems](https://github.com/rust-lang/rust/pull/53755)
* [reduce number of syscalls in `rand`](https://github.com/rust-lang/rust/pull/53725)
* [reoptimize `VecDeque::append`](https://github.com/rust-lang/rust/pull/53564)
* [add more const int ops](https://github.com/rust-lang/rust/pull/53697)
* [make `std::intrinsics::transmute()` `const fn`](https://github.com/rust-lang/rust/pull/53535)
* [`impl PartialEq for TryFromIntError`](https://github.com/rust-lang/rust/pull/53476)
* [add `--allow-staged` to `cargo fix`](https://github.com/rust-lang/cargo/pull/5943)
* [add rust-gdbgui script](https://github.com/rust-lang/rust/pull/53774)
* [set `cfg(rustdoc)` when rustdoc is running on a crate](https://github.com/rust-lang/rust/pull/53076)
* [rustbuild: distribute libLLVM.so with rustc](https://github.com/rust-lang/rust/pull/53828)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2497: if- and while-let-chains, take 2](https://github.com/rust-lang/rfcs/pull/2497).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Amend RFC 2175 to support for loops and leading vert](https://github.com/rust-lang/rfcs/pull/2530).
* [disposition: merge] [Rustfmt stability](https://github.com/rust-lang/rfcs/pull/2437).
* [disposition: e] [RFC: add futures and task system to libcore](https://github.com/rust-lang/rfcs/pull/2418).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for a minimal subset of RFC 911, const fn](https://github.com/rust-lang/rust/issues/53555).
* [disposition: merge] [Add a implementation of `From` for converting `&'a Option<T>` into `Option<&'a T>`](https://github.com/rust-lang/rust/pull/53218).
* [disposition: merge] [Add trim_start, trim_end etc.; deprecate trim_left, trim_right, etc. in future](https://github.com/rust-lang/rust/pull/52994).
* [disposition: merge] [[eRFC] add -Z emit-stack-sizes](https://github.com/rust-lang/rust/pull/51946).
* [disposition: merge] [Tracking issue for RFC 2070: stable mechanism to specify the behavior of panic! in no-std applications ](https://github.com/rust-lang/rust/issues/44489).
* [disposition: merge] [Tracking issue for the `#[used]` attribute](https://github.com/rust-lang/rust/issues/40289).

## New RFCs

* [RFC: Or patterns, i.e `Foo(Bar(x) | Baz(x))`](https://github.com/rust-lang/rfcs/pull/2535).
* [RFC: Write References for Direct and Partial Initialization using &out T and &uninit T](https://github.com/rust-lang/rfcs/pull/2534).
* [Keeping Secrets in Rust](https://github.com/rust-lang/rfcs/issues/2533).
* [RFC: Associated type defaults and Default groups](https://github.com/rust-lang/rfcs/pull/2532).
* [Amend RFC 2175 to support for loops and leading vert](https://github.com/rust-lang/rfcs/pull/2530).
* [RFC: Hidden trait implementations](https://github.com/rust-lang/rfcs/pull/2529).
* [Type-changing struct update syntax](https://github.com/rust-lang/rfcs/pull/2528).
* [Support underscores as constant names](https://github.com/rust-lang/rfcs/pull/2526).
* [RFC: Permit _ in type aliases](https://github.com/rust-lang/rfcs/pull/2524).

# Upcoming Events

### Online

* [Sep  5. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Sep 11. Rust Community Content Subteam Meeting at channel #rust-community](irc://irc.mozilla.org/rust-community).
* [Sep 12. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Sep 12. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Sep 13. Rust Switchboard Team Meeting]().
* [Sep 19. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-content).

### Asia

* [Sep 15. Chennai, IN - Rust Monthly Meetup](https://www.meetup.com/mad-rs/events/).

### Europe

* [Sep  5. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/253541000/).
* [Sep  5. Cologne, DE - Rust Cologne](http://rust.cologne/2018/09/05/fun-traits.html).
* [Sep  7. Helsinki, FI - Helsinki Rust informal meetup](https://www.meetup.com/Finland-Rust-Meetup/).
* [Sep  7. Darmstadt, DE - Rhein Main Rust Meetup / Mentoring Round](https://www.meetup.com/Rust-Rhein-Main/events/254282818).
* [Sep 18. Amsterdam, NL - Amsterdam Rust Meetup - Concurrency fundamentals, Tokio & WebAssembly](https://www.meetup.com/Rust-Amsterdam/events/253425558).
* [Sep 18. Rapperswil-Jona, CH - Rapperswil-Jona, Zürichsee Meetup - Looking for a speaker](https://www.meetup.com/de-DE/Rust-Zurich/events/251682152/).
* [Sep 18. Paris, FR - Paris - Rust Paris](http://www.meetup.com/Rust-Paris).
* [Sep 19. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/find/events/?allMeetups=false&keywords=Rust+Hack+and+Learn+OpenTechSchool&radius=25&userFreeform=Berlin%2C+Germany&mcName=Berlin%2C+DE&eventFilter=all).

### North America

* [Sep  5. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyxmbhb/).
* [Sep  5. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/cbcmbqyxmbhb/).
* [Sep  5. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/dqldspyxmbhb/).
* [Sep  9. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxmbmb/).
* [Sep 10. Seattle, US - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/pkggvpyxmbnb/).
* [Sep 12. Boulder, US - Rust Boulder/Denver Monthly Meeting](http://www.meetup.com/Rust-Boulder-Denver/).
* [Sep 13. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxmbrb/).
* [Sep 13. Arlington, US - Rust DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/253787454).
* [Sep 13. San Diego, US - San Diego Rust September Meetup - WASM, "failure" library, or ???](https://www.meetup.com/San-Diego-Rust/events/253862312/).
* [Sep 13. Utah, US - Utah Rust monthly meetup](https://www.meetup.com/utahrust/events/253965052/).
* [Sep 16. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View).
* [Sep 19. Vancouver, CA - Vancouver Rust meetup - Study/Hack/Hang-out](https://www.meetup.com/Vancouver-Rust/events/).

### South America

* [Sep  8. Santiago, CHL - Hackday Santiago de Chile]().
* [Sep 15. Sao Paulo, BR - Rust Sao Paulo - Meetup](https://www.meetup.com/Rust-Sao-Paulo-Meetup/events/253842754/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too. 
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Engineer at Anixe, Wrocław, Poland](https://anixe.bamboohr.co.uk/jobs/view.php?id=17).
* [Rust Engineer at TagiFi, Remote](https://www.reddit.com/r/rust/comments/994fcg/job_tagnifi_is_looking_for_a_rust_engineer/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Zeitgeist of Rust: developing load bearing software that will survive us.

– [Bryan Cantrill on Youtube: "The Summer of Rust (1:08:10)"](https://www.youtube.com/watch?v=LjFM8vw3pbU).

Thanks to [Matthieu M](https://users.rust-lang.org/u/matthieum) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
