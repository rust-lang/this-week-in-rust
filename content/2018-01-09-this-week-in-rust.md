Title: This Week in Rust 216
Number: 216
Date: 2018-01-09
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

* [New year's Rust: A call for community blogposts](https://blog.rust-lang.org/2018/01/03/new-years-rust-a-call-for-community-blogposts.html).
* <img alt="balloon" class="emoji" title=":balloon:" src="https://discourse-cdn-sjc1.com/business/images/emoji/twitter/balloon.png?v=5"><img alt="tada" class="emoji" title=":tada:" src="https://discourse-cdn-sjc1.com/business/images/emoji/twitter/tada.png?v=5"> [Announcing Rust 1.23](https://blog.rust-lang.org/2018/01/04/Rust-1.23.html). <img alt="tada" class="emoji" title=":tada:" src="https://discourse-cdn-sjc1.com/business/images/emoji/twitter/tada.png?v=5"><img alt="balloon" class="emoji" title=":balloon:" src="https://discourse-cdn-sjc1.com/business/images/emoji/twitter/balloon.png?v=5">
* [Announcing Diesel 1.0 — a safe, extensible query builder and ORM](https://medium.com/@sgrif/announcing-diesel-1-0-a-safe-extensible-query-builder-and-orm-15e6bd8a9ed0).
* [Ashley Williams joins the Core Team and taking lead of the Community Team](https://internals.rust-lang.org/t/announcement-ashley-williams-joins-the-core-team-and-taking-lead-of-the-community-team/6453).
* [Lessons from the impl period](http://smallcultfollowing.com/babysteps/blog/2018/01/05/lessons-from-the-impl-period/).
* [How to use Rust non lexical lifetimes on nightly](https://santiagopastorino.com/how-to-use-rust-non-lexical-lifetimes-on-nightly/).
* [A proof-of-concept GraphQL server framework for Rust](https://www.ncameron.org/blog/a-proof-of-concept-graphql-server-framework-for-rust/).
* [Web scraping with Rust](https://codeburst.io/web-scraping-in-rust-881b534a60f7). A beginner-friendly tutorial highlighting Rust’s viability as a scripting language for everyday use.
* [This week in Rust docs 87](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-87).
* [podcast] [New Rustacean: News – Rust 1.23](http://www.newrustacean.com/show_notes/news/rust_1_23/) – Rustdoc changes, the first impl period, Firefox Quantum, and more wasm! (Also note the [`Script` struct docs](http://www.newrustacean.com/show_notes/news/rust_1_23/struct.Script.html) if you prefer reading to listening!)
* [videos] [Videos from Rust Belt Rust 2017 are now available](https://www.youtube.com/playlist?list=PLgC1L0fKd7Ul71lD_cImGuMxsZ6J8fa06).

## #Rust2018

* [2018 should be boring](https://www.ncameron.org/blog/rust-2018/) by nrc.
* [Don’t be the new Haskell](https://medium.com/@tibotz/rust-2018-dont-be-the-new-haskell-a383dbd74481) by /u/tibodelor.
* [Improving how we improve Rust in 2018](http://www.jonathanturner.org/2018/01/rust2018-and-data.html) by jonathandturner.
* [Three humble paper cuts](https://gist.github.com/cessen/394829673855e56157f63b4447f91e67) by cessen.
* [What Rust needs in 2018 to succeed](https://llogiq.github.io/2018/01/09/rust.html) by llogiq.
* [What I want changed for Rust to help Way Cooler](http://way-cooler.org/blog/2018/01/09/way-cooler-turns-two.html) by Timidger.
* [Back to the roots](https://www.reddit.com/r/rust/comments/7p6n90/rust2018_back_to_the_roots/) by /u/0b_0101_001_1010.
* [Looking back and looking forward](https://gist.github.com/est31/c063704716b6880fd74ce2ba11b11298) by est31.
* [My wish list for 2018](http://www.mmrath.com/post/rust-my-wish-list-for-2018/) by mmrath.
* [Looking in on Rust in 2018](https://kasma1990.gitlab.io/2018/01/07/looking-in-on-rust-in-2018/) by KasMA1990.
* [The new wave of Rust](https://quietmisdreavus.net/code/2018/01/07/the-new-wave-of-rust/) by QuietMisdreavus.
* [New faces for our lovely bots](https://lukaskalbertodt.github.io/2018/01/07/new-faces-for-bots-rust2018.html) by LukasKalbertodt.
* [Better Debug derive](https://www.reddit.com/r/rust/comments/7p4imw/rust2018_wishpost_better_debug_derive/) by lokathor.
* [Machine learning perspective](https://www.reddit.com/r/rust/comments/7p6rpw/rust_2018_machine_learning_perspective/) by /u/osamc.
* [Rust 2018](http://www.suspectsemantics.com/blog/2018/01/07/rust-2018/) by AndrewBrinker.
* [Goals and directions for Rust in 2018](http://www.wezm.net/technical/2018/01/goals-directions-rust-2018/) by wezm.

# Crate of the Week

This week's crate is [artifact](https://github.com/vitrial/artifact), a design documentation tool. Thanks to [musicmatze](https://users.rust-lang.org/u/musicmatze) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).
* [medium] [mdBook: Introduce preprocessors](https://github.com/rust-lang-nursery/mdBook/issues/530).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

130 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-01-01..2018-01-08

* [delete the old docs, lift up the new](https://github.com/rust-lang/cargo/pull/4904)
* [generate code for unused const- and inline-fns if -Clink-dead-code is specified](https://github.com/rust-lang/rust/pull/46916)
* [allow non-alphabetic underscores in camel case](https://github.com/rust-lang/rust/pull/46907)
* [NLL fixes](https://github.com/rust-lang/rust/pull/46984)
* [only bump error count when we are sure that the diagnostic is not a repetition](https://github.com/rust-lang/rust/pull/47146)
* [limit style lint to non-synthetic generic params](https://github.com/rust-lang/rust/pull/47132)
* [try to improve LLVM pass ordering](https://github.com/rust-lang/rust/pull/46739)
  and [the pass manager order](https://github.com/rust-lang/llvm/pull/101)
* [use name-discarding LLVM context](https://github.com/rust-lang/rust/pull/47220)
* [force appropriate extension when converting from int to ptr](https://github.com/rust-lang/rust/pull/47147)
* [delay panic for aliasing violation for static items](https://github.com/rust-lang/rust/pull/47105)
  and [from incoherent drop implementation](https://github.com/rust-lang/rust/pull/47104)
* [add 'Span::parent()' and 'Span::source()' to proc_macro API](https://github.com/rust-lang/rust/pull/47099)
* [`CStore` switch `FxHashMap` to `IndexVec`](https://github.com/rust-lang/rust/pull/46913)
* [implement `TrustedRandomAccess` for `slice::`{`Chunks`, `ChunksMut`, `Windows`}](https://github.com/rust-lang/rust/pull/47142)

## New Contributors

* aheart
* BurntPizza
* Johannes Boczek
* keatinge
* Sam

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

* [disposition: merge] [Fallible collection allocation 1.0](https://github.com/rust-lang/rfcs/pull/2116).
* [disposition: merge] [Implicit caller location (third try to the unwrap/expect line info problem)](https://github.com/rust-lang/rfcs/pull/2091).
* [disposition: merge] [Unsized rvalues](https://github.com/rust-lang/rfcs/pull/1909).
* [disposition: merge] [eRFC: Cargo build system integration](https://github.com/rust-lang/rfcs/pull/2136).
* [disposition: merge] [Type privacy and private-in-public lints](https://github.com/rust-lang/rfcs/pull/2145).

## New RFCs

* [Custom cargo profiles](https://github.com/rust-lang/rfcs/pull/2282).
* [Function Structs](https://github.com/rust-lang/rfcs/pull/2276).

# Upcoming Events

* [Jan 11. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Jan 11. San Diego Rust January Meetup](https://www.meetup.com/San-Diego-Rust/events/246221114/).
* [Jan 11. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/czcwhlyxcbpb/).
* [Jan 15. Rust Sydney - Meetup 12](https://www.meetup.com/Rust-Sydney/events/245798720/).
* [Jan 15. Munich - Thread pools and iterators. Introduction and hands-on coding.](https://www.meetup.com/de-DE/rust-munich/events/245850409/)
* [Jan 16. Rust Karlsruhe](https://www.meetup.com/Rust-Hack-Learn-Karlsruhe/events/246063436/)
* [Jan 16. HTWG Konstanz: Rust - Was steckt hinter "Concurrency without Fear"?](https://www.htwg-konstanz.de/fileadmin/pub/fk_in/Aktuelles/Veranstaltungen/Rust_Concurrency-without-fear_Web.pdf)
* [Jan 17. Linuxing in London: Getting started with CLI programs in Rust](https://www.meetup.com/Linuxing-In-London/events/246605527/)
* [Jan 17. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jan 17. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jan 18. Rust DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/245934654/).
* [Jan 18. Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/mgtcwnyxcbxb/).
* [Jan 22. Durham, NC - Triangle Rustaceans - Rust 101](https://www.meetup.com/triangle-rustaceans/events/kkjnpnyxcbdc/).
* [Jan 23. A deep dive into Rust @ Facebook Developer Circle Ruhr](https://www.meetup.com/Facebook-Developer-Circle-Ruhr/events/246462601/).
* [Jan 24. Milano - Overload di funzioni in Rust - Come ho imparato a vivere felicemente senza](https://www.meetup.com/rust-language-milano/events/246439486/).
* [Jan 24. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jan 24. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jan 25. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
