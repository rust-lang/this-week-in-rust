Title: This Week in Rust 255
Number: 255
Date: 2018-10-09
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

* [Porting C to Rust, a case study: minimp3](https://wiki.alopex.li/PortingCToRust).
* [Lessons learned on writing web applications completely in Rust](https://medium.com/@saschagrunert/lessons-learned-on-writing-web-applications-completely-in-rust-2080d0990287).
* [comacro: a declarative building block for static analysis](http://blog.lambdaverse.org/comacro).
* [Oxidizing Python: Speeding up URL quoting by 10x using Rust](https://tech.blue-yonder.com/oxidizing-python-speeding-up-urlquoting-by-using-rust/).
* [GLSL quasiquoting in Rust](https://phaazon.net/blog/glsl-quasiquoting).
* [Going four times faster using multi-threading](http://worthe-it.co.za/programming/2018/10/03/going-four-times-faster-with-multithreading.html).
* [Bluetooth Low Energy with Rust](https://219design.com/bluetooth-low-energy-with-rust/).
* [Hunting for bugs in Rust](https://blog.troutwine.us/2018/10/08/hunting-for-bugs-in-rust/).
* [Who authors the most popular crates on crates.io](https://words.steveklabnik.com/who-authors-the-most-popular-crates-on-crates-io)?
* [Rust's story for default arguments](https://medium.com/@softprops/default-values-copy-that-ae43831781f3).
* [Remacs continues to improve](http://db48x.net/rust-remacs-2018/).
* [Future directions for cbindgen (rust-ffi)](http://dreamingofbits.com/post/future-directions-for-cbindgen-rust-ffi/).
* [The Embedded WG newsletter 13](https://rust-embedded.github.io/blog/2018-10-09-newsletter-13/).

# Crate of the Week

This week's crate is [pest](https://pest.rs), a Parsing Expression Grammar-based parser library. Thanks to [CAD97](https://users.rust-lang.org/t/crate-of-the-week/2704/460) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Announcing the Tokio doc blitz effort, we need your help](https://tokio.rs/blog/2018-10-doc-blitz/)!
* [Rust Web Survey from Networking Services WG](https://docs.google.com/forms/d/e/1FAIpQLSf9KCUs-8G87pHB08lM8-iXcDSY_VttOI0PvkKseHaZseCGGA/viewform).
* [The imag project calls for contributors](https://imag-pim.org/blog/2018/10/10/call-for-participation-1/).
* [macro_railroad: Update the parser for syn-0.15](https://github.com/lukaslueg/macro_railroad/issues/17). macro_railroad is a library to generate syntax diagrams for Rust macros.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

136 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-10-01..2018-10-08

* [merge `proc_macro_` expansion feature gates as `proc_macro_hygiene`](https://github.com/rust-lang/rust/pull/52121)
* [proptest basic validation](https://github.com/rust-lang/cargo/pull/6149)
* [allow both explicit and elided lifetimes in the same impl header](https://github.com/rust-lang/rust/pull/54458)
* [do not promote comparing function pointers](https://github.com/rust-lang/rust/pull/54702)
* [nest the `impl Trait` existential item inside the return type](https://github.com/rust-lang/rust/pull/54741)
* [do not normalize all non-scalar constants to a ConstValue::ScalarPair](https://github.com/rust-lang/rust/pull/54693)
* [fix dead code lint for functions using `impl Trait`](https://github.com/rust-lang/rust/pull/54810)
* [add suggestion for inverted function parameters](https://github.com/rust-lang/rust/pull/54804)
* [suggest to use self for fake-self from other languages](https://github.com/rust-lang/rust/pull/54694)
* [NLL: improve move error loop detection](https://github.com/rust-lang/rust/pull/54343)
* [make NLL suggest "try removing `&mut` here"](https://github.com/rust-lang/rust/pull/54720)
* [introduce `TyKind::UnnormalizedProjection`](https://github.com/rust-lang/rust/pull/54789)
* [stabilize `min_const_fn`](https://github.com/rust-lang/rust/pull/54835)
* [improve error message when trying to move from an Rc or Arc](https://github.com/rust-lang/rust/pull/54703)
* [revisit work on cvoid](https://github.com/rust-lang/libc/pull/1086)
* [fix Once perf regression](https://github.com/rust-lang/rust/pull/54662)
* [make `CStr::from_bytes_with_nul_unchecked()` a const fn](https://github.com/rust-lang/rust/pull/54745)
* [std: start implementing wasm32 atomics](https://github.com/rust-lang/rust/pull/54017)
* [make `spec_extend` use `for_each()`](https://github.com/rust-lang/rust/pull/54761)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2539: `#[cfg_attr]` expanding to multiple attributes](https://github.com/rust-lang/rfcs/pull/2539).
* [RFC 2412: The optimize attribute](https://github.com/rust-lang/rfcs/pull/2412).
* [RFC 2535: Or patterns, i.e `Foo(Bar(x) | Baz(x))`](https://github.com/rust-lang/rfcs/pull/2535).
* [RFC 2526: Support underscores as constant names](https://github.com/rust-lang/rfcs/pull/2526).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Formatting guidelines](https://github.com/rust-lang/rfcs/pull/2436).
* [disposition: merge] [Union initialization and Drop](https://github.com/rust-lang/rfcs/pull/2514).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Support for disabling PLT for better function call performance](https://github.com/rust-lang/rust/pull/54592).
* [disposition: close] [Introduce the `Result::into_inner` method](https://github.com/rust-lang/rust/pull/54219).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online

* [Oct 17. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Oct 22. Rust Community Content Subteam Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Oct 24. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).

### Asia

* [Oct 13. Bangalore, IN - Flat Buffers: What and How](https://www.meetup.com/rustox/events/254812229/).

### Europe

* [Oct 17. Madrid, ES - Quinto meetup de MadRust](https://www.meetup.com/MadRust/events/255207242/).
* [Oct 17. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/255020858/).
* [Oct 18. Oslo, NO - Fuzzing and property-based testing in Rust](https://www.meetup.com/Rust-Oslo/events/254830021/).
* [Oct 18. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxnbxb/).
* [Oct 25. Wroclaw, PL - Rust Wroclaw Meetup](https://www.meetup.com/Rust-Wroclaw/events/255053694/).

### North America

* [Oct 11. Arlington, US - Rust DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/253787466).
* [Oct 11. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxnbpb/).
* [Oct 11. Utah, US - Utah Rust monthly meetup](https://www.meetup.com/utahrust/events/255209633/).
* [Oct 14. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxnbsb/).
* [Oct 17. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/dqldspyxnbwb/).
* [Oct 19 & 20. Ann Arbor, US - Rust Belt Rust 2018](https://rust-belt-rust.com/).
* [Oct 20 & 21. Vancouver, CA - Vancouver Rust Hackathon](https://www.eventbrite.ca/e/vancouver-rust-hackathon-tickets-50012680273).
* [Oct 21. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxnbcc/).
* [Oct 22. Durham, US - Triangle Rustaceans](https://www.meetup.com/triangle-rustaceans/events/mfglwpyxnbdc/).
* [Oct 23. Chicago, US - The Rust Ecosystem: What to Know After "Hello World"](https://www.meetup.com/Chicago-Rust-Meetup/events/255066746).
* [Oct 24. Zurich, CH - Rust Zurich - Atomics](https://www.meetup.com/Rust-Zurich/events/255279862/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer - Backend at Watchful, San Francisco](https://jobs.lever.co/watchful/7f502bc3-a91d-46fd-908f-19a9cefbb4b2).
* [Rust Engineer at Standard Cognition, San Francisco](https://www.reddit.com/r/rust/comments/9ml170/job_create_high_performance_rust_systems/).
* [Rust Software Engineer at IOHK (Remote work available)](https://iohk.recruiterbox.com/jobs/fk0177c).
* [Compilers & distributed systems engineers in Australia](https://www.reddit.com/r/rust/comments/9kx94z/job_compilers_distributed_systems_engineers_in/).
* [Rust Developer at Parity, Berlin, DE](https://paritytech.io/jobs/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust is a Fast Programming Language. Rust programs are therefore “fast,” especially so if you write them with the correct observations to the arcane ley lines of birth and death known as “lifetimes,” and also remember to pass cargo the `--release` flag.

– Adam Perry [blogging about lolbench](https://blog.anp.lol/rust/2018/09/29/lolbench)

Thanks to [Pascal Hertleif](https://users.rust-lang.org/t/twir-quote-of-the-week/328/565) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/9mzrso/this_week_in_rust_255/).</small>
