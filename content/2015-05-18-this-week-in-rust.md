Title: This Week in Rust 81
Date: 2015-05-18
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/wiki/Note-guide-for-new-contributors).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors or omissions in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# What's cooking on master?

273 pull requests were [merged in the last two weeks][merged], and 4 [RFC PRs][rfcs].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-05-04..2015-05-18
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-05-04..2015-05-18

Now you can follow breaking changes *[as they happen][BitRust2]*!

[BitRust2]: http://killercup.github.io/bitrust/

# Breaking Changes

* [New floating-point to decimal formatting
  routine](https://github.com/rust-lang/rust/pull/24612). lifthrasiir
  completely rewrote Rust's floating point to string conversion to
  employ the
  [Grisu](http://www.cs.tufts.edu/%7Enr/cs257/archive/florian-loitsch/printf.pdf)
  algorithm. This causes some slight changes to Rust's formatting
  output.
* [Redesign `Duration`](https://github.com/rust-lang/rust/pull/24920).
* [Destabilize
  `io::BufStream`](https://github.com/rust-lang/rust/pull/25009). Some
  last minute uncertainty about the semantics of `seek`.
* [Remove addition on vectors for
  now](https://github.com/rust-lang/rust/pull/25157). These entered
  the tree under the radar, so to speak, so were removed before 1.0 to
  give more consideration.
* [Mark `mem::forget`
  safe](https://github.com/rust-lang/rust/pull/25187). Memory / dtor
  leaks are not considered unsafe.
* [dropck: must assume `Box<Trait + 'a>` has a destructor of
  interest](https://github.com/rust-lang/rust/pull/25212).  Because
  these types may have destructors, so the compiler must assume that
  any contained regions strictly outlive the type, per [RFC
  769](https://github.com/rust-lang/rfcs/blob/master/text/0769-sound-generic-drop.md#the-drop-check-rule).

# Other Changes

* Documentation includes a new [error
  index](http://doc.rust-lang.org/error-index.html), cataloging
  extended error explanations.
* [DST
  coercions](https://github.com/rust-lang/rust/pull/24619). Allows
  smart pointers of statically-sized types to be cast to smart
  pointers of dynamically sized
  types. [RFC](https://github.com/rust-lang/rfcs/blob/master/text/0401-coercions.md).
* [Add lint to deny transmuting &T to &mut
  T](https://github.com/rust-lang/rust/pull/24392). The
  `mutable_transmutes` lint catches transmutes that are almost always
  incorrect.
* [Implement `append` and `split_off` for
  `BitVec`](https://github.com/rust-lang/rust/pull/24890).
* [Add decorator syntax extensions on trait and impl
  items](https://github.com/rust-lang/rust/pull/25024).
* [Optimize iterator
  adaptors](https://github.com/rust-lang/rust/pull/25035).
* [Implement Debug for
  std::net::{UdpSocket,TcpStream,TcpListener,Shutdown}](https://github.com/rust-lang/rust/pull/25078).
* [Stabilize from_raw_os feature in
  1.1](https://github.com/rust-lang/rust/pull/25125). For creating I/O
  types from raw handles.
* [Add AsRef<[u8]> for both str and
  String](https://github.com/rust-lang/rust/pull/25162).
* More extended diagnostics from ruud-v-a, cactorium, michaelsproul,
  nham, meqif, caipre: [e1], [e2], [e3], [e4], [e5], [e6], [e7], [e8],
  [e9], [e10], [e11], [e12], [e13].
* Lots and lots of documentation improvements have been landing in the
  push for 1.0.
* [Allow #[derive(...)] to generate unsafe methods](https://github.com/rust-lang/rust/pull/25524).

[e1]: https://github.com/rust-lang/rust/pull/24966
[e2]: https://github.com/rust-lang/rust/pull/24576
[e3]: https://github.com/rust-lang/rust/pull/25114
[e4]: https://github.com/rust-lang/rust/pull/25200
[e5]: https://github.com/rust-lang/rust/pull/25190
[e6]: https://github.com/rust-lang/rust/pull/25267
[e7]: https://github.com/rust-lang/rust/pull/25255
[e8]: https://github.com/rust-lang/rust/pull/25272
[e9]: https://github.com/rust-lang/rust/pull/25302
[e10]: https://github.com/rust-lang/rust/pull/25363
[e11]: https://github.com/rust-lang/rust/pull/25398
[e12]: https://github.com/rust-lang/rust/pull/25422
[e13]: https://github.com/rust-lang/rust/pull/25501

# New Contributors

* らいどっと
* Aaron Gallagher
* Alexander Polakov
* Alex Burka
* Andrei Oprea
* Andrew Kensler
* Andrew Straw
* Ben Gesoff
* Chris Hellmuth
* Cole Reynolds
* Colin Walters
* David Reid
* Don Petersen
* Emilio Cobos Álvarez
* Franziska Hinkelmann
* Garming Sam
* Hika Hibariya
* Isaac Ge
* Jan Andersson
* Jan-Erik Rediger
* Jannis Redmann
* Jason Yeo
* Jeremy Schlatter
* Johann
* Johann Hofmann
* Lee Jeffery
* leunggamciu
* Marin Atanasov Nikolov
* Mário Feroldi
* Mathieu Rochette
* Michael Park
* Michael Wu
* Michał Czardybon
* Mike Sampson
* Nick Platt
* parir
* Paul Banks
* Paul Faria
* Paul Quint
* peferron
* Pete Hunt
* robertfoss
* Rob Young
* Russell Johnston
* Shmuale Mark
* Simon Kern
* Sindre Johansen
* sumito3478
* Swaroop C H
* Tincan
* Wei-Ming Yang
* Wilfred Hughes
* Will Engler
* Wojciech Ogrodowczyk
* XuefengWu
* Z1

# Approved RFCs

* [RFC 1066: Alter `mem::forget` to be
  safe](https://github.com/rust-lang/rfcs/pull/1066). Leaking memory
  and not running dtors is considered safe.
* [RFC 1068: Scaling Rust's
  Governance](https://github.com/rust-lang/rfcs/pull/1068). Establishes
  'subteams' to spread responsibility to more people.

# New RFCs

* [Result::expect](https://github.com/rust-lang/rfcs/pull/1119).
* [Semantic versioning for the language](https://github.com/rust-lang/rfcs/pull/1122).
* [Introduce `split_at` on `str`](https://github.com/rust-lang/rfcs/pull/1123).
* [Stabilize `TcpStream::set_keepalive`](https://github.com/rust-lang/rfcs/pull/1126).

# Betawatch!

The current beta is `1.1.0-beta (cd7d89af9 2015-05-16) (built 2015-05-16)`.

# Notable Links

* [Rust 1.0 is here](http://blog.rust-lang.org/2015/05/15/Rust-1.0.html).
* [Rust 1.0 t-shirts are available for order](http://devswag.com/products/rust-t-shirt).
* [Announcing the subteams](https://internals.rust-lang.org/t/announcing-the-subteams/2042). The official Rust governance structure has expanded.
* [Finding closure in
  Rust](http://huonw.github.io/blog/2015/05/finding-closure-in-rust/). The
  best explanation of Rust closures.
* [How to write a Rust syntax
  extension](http://brodoyouevencode.com/posts/how-to-write-a-rust-syntax-extension/).
* [Virtual structs part 1: where Rust's enum
  shines](http://smallcultfollowing.com/babysteps/blog/2015/05/05/where-rusts-enum-shines/).
* [Where Self meets Sized: Revisiting Object
  Safety](http://huonw.github.io/blog/2015/05/where-self-meets-sized-revisiting-object-safety/).
* [Slides from cburgdorf's talk on
  Nickel](http://thoughtram.io/rust-and-nickel/).
* [RailsConf 2015 - Bending the Curve: How Rust Helped Us Write Better
  Ruby (video)](https://www.youtube.com/watch?v=LazvK39Oc4U).
* [rust-learning](https://github.com/ctjhoa/rust-learning) and
  [awesome-rust](https://github.com/kud1ing/awesome-rust). Resources
  for newbies.
* [Servo's dependency graph](http://brson.github.io/images/servo-deps.svg). It's massive.
* [rust-rss](https://github.com/frewsxcv/rust-rss). Library for serializing the RSS web content syndication format.
* [Rust Discovery, or: How I Figure Things Out](http://carol-nichols.com/2015/05/10/rustc-discovery/). Carol's thought process for contributing to Rust.
* [Rust's ownership model for JavaScript developers](http://blog.thoughtram.io/rust/2015/05/11/rusts-ownership-model-for-javascript-developers.html).
* [Abstraction without overhead: traits in Rust](http://blog.rust-lang.org/2015/05/11/traits.html).
* [Support for building components in Rust lands in Firefox Nightly](https://twitter.com/rillian/status/597150813639684096). This just means that the Fx build system *can* build Rust, not that it is.
* [Rust ownership, the hard way](http://chrismorgan.info/blog/rust-ownership-the-hard-way.html).
* [Criticizing the Rust language, and why C/C++ will never die](http://www.viva64.com/en/b/0324/). This article was [seen as something of a misleading hit-piece](http://www.reddit.com/r/rust/comments/35pn5a/criticizing_the_rust_language_and_why_cc_will/) by a C++ static analysis vendor but it got a fair bit of attention.
* [A Taste of Rust](http://www.evanmiller.org/a-taste-of-rust.html). An experience report with some critique.
* [Proposed security disclosure policy](https://internals.rust-lang.org/t/proposed-security-disclosure-policy/2024). A security policy for Rust.
* [Rust makes me excited about the future](http://www.reddit.com/r/rust/comments/35w2nq/rust_makes_me_excited_about_the_future/). Rust will make everything better.
* [Pre-RFC: std::net expansion](https://internals.rust-lang.org/t/pre-rfc-std-net-expansion-refinement/2079/1).
* [Error handling in Rust](http://blog.burntsushi.net/rust-error-handling/). From Andrew Gallant, who knows a few things about the subject.
* [Diversity on the governance teams](https://internals.rust-lang.org/t/diversity-on-the-governance-teams/2048/3).
* [Porting a ray tracer to Rust, part3](http://www.willusher.io/2015/05/15/porting-a-ray-tracer-to-rust-part-3/).
* [O'Reilly book on Rust available for pre-order](http://www.amazon.com/Programming-Rust-Jim-Blandy/dp/1491927283/).
* [Diving into Rust for the first time](https://hacks.mozilla.org/2015/05/diving-into-rust-for-the-first-time/). On the Mozilla hacks blog.
* [My Python's a little Rust-y - PyCon 2015 (video)](http://youtube.com/watch?v=3CwJ0MH-4MA).
* [Rust 1.0 Bay Area release party (video)](https://air.mozilla.org/rust-release-party/). Some lightning talks about production users.
* [Mozilla-backed Rust language stabilizes at version 1.0](http://arstechnica.com/information-technology/2015/05/mozilla-backed-rust-language-stabilizes-at-version-1-0/). First time Ars has mentioned Rust in my recollection, but shallow coverage.
* [Clean Rust Off Campaign](https://users.rust-lang.org/t/clean-rust-off-campaign/1393). Ridding the Internet of out-dated information.
* [The problem with single-threaded mutability](http://manishearth.github.io/blog/2015/05/17/the-problem-with-shared-mutability/).

# Project Updates

* [Vagga](https://github.com/tailhook/vagga). Docker-inspired
  container engine.
* [rust-adorn](https://github.com/Manishearth/rust-adorn). Python
  style function decorators.
* [recycler](https://github.com/frankmcsherry/recycler). Recycling
  types with owned memory.
* [MemBuf](https://github.com/reem/rust-membuf). Managing heap
  buffers.
* [A Rust cartridge for OpenShift](http://blog.flaper87.com/post/rust-cartridge-for-openshift/).
* [Routing](https://github.com/dirvine/routing). A distributed hash
  table.
* [Rustful 0.1 was released](http://www.reddit.com/r/rust/comments/35bqak/rustful_010_is_now_available_on_cratesio/). A simple web framework.
* [twox-hash](http://www.reddit.com/r/rust/comments/35d029/hashing_at_over_9000_mbsec_twoxhash/). A fash hash algorithm.
* [http_replay](https://github.com/ucarion/http_replayer). Hyper middleware for recording and replaying HTTP requests.
* [Rust + Go](https://github.com/medimatrix/rust-plus-golang). An example of calling Rust from Go.
* [Glutin running on a Raspberry Pi](https://www.reddit.com/r/rust_gamedev/comments/3560hy/glutin_on_a_raspberry_pi/)
* [rust-api-docs-helper](https://atom.io/packages/rust-api-docs-helper). [atom.io](https://atom.io/) package for accessing std docs.
* [Chapter 1 of Iron by Example: Decomposing Hello World](https://github.com/iron/byexample/blob/master/chapters/hello.md).
* [kafka-rust](https://github.com/spicavigo/kafka-rust). A [Kafka](https://kafka.apache.org/index.html) client.
* [Google.rs 0.2 was released](https://github.com/Byron/google-apis-rs/releases/tag/cli-v0.2.0).
* [Google.rs dev diary #3 (video)](https://youtu.be/zrw2Qy-Ho5A).
* Packt is releasing a book called [Rust Essentials](https://www.packtpub.com/application-development/rust-essentials) soon.
* [Pulse](https://github.com/csherratt/pulse). A 'composable wait system'.
* [Servo: The embeddable browser engine](http://blogs.s-osg.org/servo-the-embeddable-browser-engine/). An article for Samsung by Lars and Mike about Servo embedding.
* Chris Morgan overhauled the [design of play.rust-lang.org](http://www.reddit.com/r/rust/comments/35vwsg/playrustlangorg_is_pretty/).
* [Rust running on a PlayStation](http://www.reddit.com/r/rust/comments/35ypb8/rust_running_on_a_playstation/).
* [Visual Rust 0.1 released](http://blog.piston.rs/2015/05/14/Visual-Rust-0.1/).
* [sxd-document](https://github.com/shepmaster/sxd-document). XML and XPath in pure Rust.
* [This Week in Servo 32](http://blog.servo.org/2015/05/24/twis-32/).
* Bloomberg CodeCon, a programming competition, [supports Rust](http://codecon.bloomberg.com/).
* [docker-rust has been updated to 1.0 stable](https://registry.hub.docker.com/u/jimmycuadra/rust/).
* Hematite, the Minecraft clone, [running in the browser via Emscripten](http://myth.aaronlindsay.com/test/).
* [A Racer-based code completion plugin for KDE's Kate editor](https://blogs.kde.org/2015/05/18/basic-code-completion-rust-kdes-kate-and-later-kdevelop).
* A Kickstarter for a book called [Rust Programming Concepts](https://www.kickstarter.com/projects/1712125778/1409335994?token=951963e5).
* [KISS-UI](https://github.com/cybergeek94/kiss-ui), a simple UI toolkit for Rust.
* [gtk](https://github.com/rust-gnome/gtk), the GTK+ 3.x's binding. 

# Upcoming Events

* [5/22. Boulder, CO](
http://www.degoesconsulting.com/lambdaconf-2015/). Lambda conf. 'An Introduction to Rust: Or, "Who Got Types in My Systems Programming!" - Jared Roesch'
* [5/23. Bangalore, India](http://www.eventbrite.com/e/rust-10-release-party-tickets-16908882924)
* [5/23. Pune, India](https://www.facebook.com/events/983011235082826)
* [5/27. Columbus Rust Society](http://www.meetup.com/columbus-rs/)
* [5/27. Lithuania](http://www.meetup.com/functional-vilnius/events/222319383/). Functional Vilnius #4: Rust & Monoids.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# Quote of the Week

*"Yes, because laundry eating has evolved to be a specific design goal now; and the initial portions of the planned laundry eating API have been landed behind the #![feature(no_laundry)] gate. no_laundry should become stable in 6-8 weeks, though the more complicated portions, including DRY cleaning, Higher Kinded T-shirts, Opt-in Builtin Detergent, and Rinse Time Optimization will not be stabilized until much later."*

*"We hope this benefits the Laundry as a Service community immensely."*

Manish [explains](http://www.reddit.com/r/rust/comments/35vyej/10_stable_is_nearly_here/cr8pxi2) Rust's roadmap for laundry-eating.

Thanks to filsmick for the tip.

And since there were so many quotables in the last two weeks, here's one from
[Evan Miller's evaluation of Rust](http://www.evanmiller.org/a-taste-of-rust.html):

*"Rust is a systems language. I’m not sure what that term means, but it
seems to imply some combination of native code compilation, not being
Fortran, and making no mention of category theory in the documentation."*

Thanks to ruudva for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
