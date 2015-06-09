Title: This Week in Rust 82
Date: 2015-06-07
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

# In summary

It's been a few weeks since the last TWiR. Things have happened.

[RustCamp 2015](http://rustcamp.com/) was announced, though many
details are still to be determined. Please submit talk proposals.

Several I/O stabilization patches have been nominated for backporting
to 1.1, filling out the somewhat meager story we had for 1.0. With things
moving somewhat slowly since the last release, I/O improvements will
probably be the most interesting part of the next.

If you haven't recently, give [the
playpen](https://play.rust-lang.org) another look as it's recieved
several new features lately, including a new design from Chris Morgan,
the ability to output Intel-flavor asm, automatically post GitHub
gists, and run `#[test]` functions.

# What's cooking on master?

346 pull requests were [merged in the last three weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-05-18..2015-06-07

Now you can follow breaking changes *[as they happen][BitRust2]*!

[BitRust2]: http://killercup.github.io/bitrust/

# Breaking Changes

* [Remove `#[static_assert]`](https://github.com/rust-lang/rust/pull/24910). An underbaked feature.
* [Make `Arc::get_mut` and `make_unique` unsafe](https://github.com/rust-lang/rust/pull/25908). These unstable functions contain races.

# Other Changes

* [Stabilize debug builders](https://github.com/rust-lang/rust/pull/24847).
* [Add support for registering attributes with plugins](https://github.com/rust-lang/rust/pull/25168)
* [Initial MSVC support](https://github.com/rust-lang/rust/pull/25350). That is, support for creating Rust programs without using the MinGW toolchain.
* [Implement lowering and raising for process I/O](https://github.com/rust-lang/rust/pull/25494). Conversion to system I/O handles, that is.
* [Add unstable method `Child::id`](https://github.com/rust-lang/rust/pull/25495). For getting child process handles.
* Michael Sproul, Nick Hamann, and Guillaume Gomez [continue][e1] [to][e2] [document][e3] [errors][e4], which are now included in the [docs][eindex].
* [Initial implementation of `const fn`](https://github.com/rust-lang/rust/pull/25609). Limited compile-time evaluation per [RFC 911](https://github.com/rust-lang/rfcs/blob/master/text/0911-const-fn.md).
* Development of syntax highlighting for [KDE's Kate editor](https://github.com/rust-lang/kate-config) has been moved upstream.
* [Allow patterns in macros to be followed by `if` and `in`](https://github.com/rust-lang/rust/pull/25713)
* [Richo bravely decided to make our scripts python3 compatible](https://github.com/rust-lang/rust/pull/25749). Now somebody needs to do the same for LLVM.
* [Implement `Eq` for `Cell` and `RefCell`](https://github.com/rust-lang/rust/pull/25744). Since `RefCell` can panic, being able to `Eq` it is arguably a misfeature, but the implementation of `PartialEq` for `RefCell` made it into 1.0, so `Eq` has also been added.
* Some foreign-language translations of the book [have appeared](https://github.com/rust-lang/rust/pull/25751). I'm sure they would love contributions.
* [Implemented `std::iter::once/empty`](https://github.com/rust-lang/rust/pull/25817). A few basic iterators.
* [Two](https://github.com/rust-lang/rustc-serialize/pull/118) [patches](https://github.com/rust-lang/rustc-serialize/pull/119) landed to rustc-serialize to
improve performance of base64 encoding. Thanks Veedrac and cristicbz.

[e1]: https://github.com/rust-lang/rust/pull/25501
[e2]: https://github.com/rust-lang/rust/pull/25552
[e3]: https://github.com/rust-lang/rust/pull/25590
[e4]: https://github.com/rust-lang/rust/pull/25593
[e5]: https://github.com/rust-lang/rust/pull/25688
[eindex]: http://doc.rust-lang.org/error-index.html

# Approved RFCs

* [Have collections `impl Extend<&T> where T: Copy`](https://github.com/rust-lang/rfcs/pull/839)
* [Retire RFC 8 (intrinsics) without implementing it](https://github.com/rust-lang/rfcs/pull/948). An old RFC was never implemented.
* [DST custom coercions](https://github.com/rust-lang/rfcs/pull/982)
* [Don't panic when stdout doesn't exist](https://github.com/rust-lang/rfcs/pull/1014)
* [Socket timeouts](https://github.com/rust-lang/rfcs/pull/1047)
* [Remove static assert](https://github.com/rust-lang/rfcs/pull/1096). The static assert as implemented was not very good.

# New RFCs

* [IndexAssign: overloading the `a[b] = c` expression](https://github.com/rust-lang/rfcs/pull/1129)
* [Implement FromIterator for the unit type](https://github.com/rust-lang/rfcs/pull/1130)
* [Add an `expect` intrinsic](https://github.com/rust-lang/rfcs/pull/1131)
* [Make Cargo aware of libstd dependencies](https://github.com/rust-lang/rfcs/pull/1133)
* [Implement raw fat pointer comparisons](https://github.com/rust-lang/rfcs/pull/1135)
* [How to handle API deprecation in std](https://github.com/rust-lang/rfcs/pull/1147)
* [Mutually exclusive traits](https://github.com/rust-lang/rfcs/pull/1148)
* [Rename attribute](https://github.com/rust-lang/rfcs/pull/1150)
* [Add some of `[T]`’s methods to strings and vice versa](https://github.com/rust-lang/rfcs/pull/1152)
* [Disjoins (anonymous enums)](https://github.com/rust-lang/rfcs/pull/1154)

# Internals discussions

* [Thoughts on aggressive deprecation in libstd](https://internals.rust-lang.org/t/thoughts-on-aggressive-deprecation-in-libstd/2176/8). Should we try to fix the warts or leave them?
* [Submodules in rust-lang/rust for external repositories?](https://internals.rust-lang.org/t/submodules-in-rust-lang-rust-for-external-repositories/2200/19). We'll probably start linking to external crates via submodules instead of duplicating them in-tree.
* [Pre-RFC: adjust default trait object bounds](https://internals.rust-lang.org/t/pre-rfc-adjust-default-object-bounds/2199/30). The default bounds for trait objects has an unfortunate inconsistency, and fixing it, though relatively-low impact currently, is a breaking change.

# New Contributors

* Akshay Chiwhane
* Alexander Artemenko
* Alex Stokes
* Andrew Foote
* Austin Hellyer
* benaryorg
* Brian Quinlan
* Christian Stadelmann
* Chuck Bassett
* clatour
* Cornel Punga
* David Campbell
* David Voit
* dmgawel
* econoplas
* edunham
* Eric Ye
* Erik Michaels-Ober
* Felix S Klock II
* funkill
* Iven Hsu
* Jexell
* Kubilay Kocak
* Lorenz
* Marcus Klaas
* Markus Westerlind
* Matej Ľach
* Mathieu David
* Max Jacobson
* Michael Layzell
* Mohammed Attia
* Nick Fitzgerald
* Nils Liberg
* OlegTsyba
* olombard
* Parker Moore
* Paul Oliver
* petrochenkov
* Rein Henrichs
* Rémi Audebert
* Steve Gury
* Thomas Karpiniec
* Tim Ringenbach
* webmobster
* Will Andrews

# Notable Links

* [Rust has a nascent security policy now](http://www.rust-lang.org/security.html)
* [Iterator cheat sheet](https://danielkeep.github.io/itercheat_baked.html). A concise explanation of the std and [itertools](https://github.com/bluss/rust-itertools) iterators.
* [Projects to spread Rust in Brazil](https://users.rust-lang.org/t/projects-to-spread-rust-in-brazil/1575/12). Brazillians unite to spread the word of Rust.
* [My Initial Experience With Rust](http://www.reddit.com/r/rust/comments/3929mj/my_initial_experience_with_rust/).
* [Three Months of Rust](http://scattered-thoughts.net/blog/2015/06/04/three-months-of-rust/).
* A simple web app in Rust: parts [1](http://joelmccracken.github.io/entries/a-simple-web-app-in-rust-pt-1/), [2](http://joelmccracken.github.io/entries/a-simple-web-app-in-rust-pt-2a/), and [2b](http://joelmccracken.github.io/entries/a-simple-web-app-in-rust-pt-2b/).
* [The Flub Paradox](http://steved-imaginaryreal.blogspot.com/2015/06/the-flub-paradox.html). I guess some people think Rust is too good.
* [Expressing L-systems in Rust](http://unconj.ca/blog/expressing-l-systems-in-rust.html).
* [Tutoriel Rust](http://blog.guillaume-gomez.fr/Rust). A much-needed French-language tutorial.
* [My Lint Writing Workflow](https://llogiq.github.io/2015/06/04/workflows.html). llogiq shows us how to write rustc lint plugins.
* [Measure Data Structure Sizes: Firefox vs. Servo](https://blog.mozilla.org/nnethercote/2015/06/03/measuring-data-structure-sizes-firefox-c-vs-servo-rust/). How Nick Nethercote is adding memory instrumentation to Servo.
* [Servo: The Countdown to Your Next Browser Continues](http://blogs.s-osg.org/servo-the-countdown-to-your-next-browser-continues/)
* [Rust on OpenWRT](https://github.com/japaric/rust-on-openwrt). Cross-compiling to popular MIPS devices.
* [Unofficial builds for arm-linux-gnueabihf](http://www.reddit.com/r/rust/comments/37wat6/unofficial_rustcargo_nightlybetastable_builds_for/).
* [I wrote a website in Rust and lived to tell the tale](http://blog.viraptor.info/post/i-wrote-a-website-in-rust-and-lived-to-tell-the-tale).
* [Virtual Structs Part 2](http://smallcultfollowing.com/babysteps/blog/2015/05/29/classes-strike-back/). More motivation for virtual structs. By Niko.
* [How Rust Achieves Thread Safety](http://manishearth.github.io/blog/2015/05/30/how-rust-achieves-thread-safety/). By Manish.
* [Rust for Python Programmers](http://lucumr.pocoo.org/2015/5/27/rust-for-pythonistas/). By Armin Ronacher.
* [Javascript Jabber Episode 161: Rust with David Herman (audio)](http://devchat.tv/js-jabber/161-jsj-rust-with-david-herman). Dave Herman is the director of strategy at Mozilla Research.
* [Defaulting to thread-safety: closures and concurrency](http://huonw.github.io/blog/2015/05/defaulting-to-thread-safety/). By Huon.
* [Rust vs. C++ performance for a path tracer](http://xania.org/201505/on-rust-performance).
* [Implementing methods on builtins](https://blog.dbrgn.ch/2015/5/25/rust-implementing-methods-on-builtins/).

# Project Updates

* [Rust Programming Concepts](https://www.kickstarter.com/projects/1712125778/rust-programming-concepts-book). A Kickstarter to write a Rust book.
* [Primal: Putting Raw Power Into Prime Numbers](https://users.rust-lang.org/t/primal-putting-raw-power-into-prime-numbers/1747/1). Huon releases a library for analyzing prime numbers.
* [pulldown-cmark](https://users.rust-lang.org/t/new-commonmark-parser/1690/15). A parser for the CommonMark markdown standard. Intended to replace the pre-standard Hoedown for use in rustdoc.
* [Feedback request: yaml-rust](https://users.rust-lang.org/t/feedback-request-yaml-rust-the-missing-yaml-1-2-parser-for-pure-rust/1659). @chyh1990 wants feedback on the only Rust YAML parser.
* [Feedback required: mpd](https://users.rust-lang.org/t/feedback-required-mpd-crate/1671). @kstep wants feedback on his interface to mpd - the Music Player Daemon.
* [Feedback request: a client library for HyperDex](https://users.rust-lang.org/t/feedback-request-a-client-library-for-hyperdex/1068). HyperDex is a new key-value store.
* [HttpMuncher](https://users.rust-lang.org/t/httpmuncher-rust-streaming-http-parser/1464). A streaming HTTP parser, wrapping the Joyent parser.
* [Notify 0.2 released](http://www.reddit.com/r/rust/comments/390s21/notify_v200_released_need_help_reimplementing_a/). Cross-platform file-system notification.
* [dbus](http://www.reddit.com/r/rust/comments/38xqqa/dbus_bindings_v010/). New D-Bus bindings!
* [StemJail](http://www.reddit.com/r/rust/comments/38saor/dynamic_useractivity_isolation_using_linux/). An interesting application of Linux containers.
* [Piston-Window 0.2 released](http://blog.piston.rs/2015/06/05/piston-window-0.2/).
* [doapi & docli](http://www.reddit.com/r/rust/comments/38r3os/any_digitalocean_users_heres_docli_and_doapi/). Interfaces to Digital Ocean APIs.
* [sudoku](http://www.reddit.com/r/rust/comments/38pwc2/first_crate_released_sudoku/). A sudoku solver.
* [iota](http://www.reddit.com/r/rust/comments/38jkv2/my_first_compiler_plugin_const_blocks_and_iota/). A plugin that provides Go-like 'iotas'.
* [Visual Studio Code supports Rust syntax highlighting](https://code.visualstudio.com/Updates).
* [rurtle](http://www.reddit.com/r/rust/comments/387rwr/rurtle_turtle_graphics_in_rust/). Turtle graphics.
* [RON](http://www.reddit.com/r/rust/comments/37vozv/ron_rusty_object_notation_yet_another_alternative/). Rusty Object Notation, ala JSON.
* [Maud](http://lfairy.gitbooks.io/maud/content/). A template engine.
* [units](https://github.com/Boddlnagg/units). Type-safe units of measure.
* [This Week in Servo 33](http://blog.servo.org/2015/05/28/twis-33/).
* [Five lists of six things about Rust](http://graydon2.dreamwidth.org/214016.html). By Graydon.
* [Unsafe Rust: An Intro and Open Questions](http://cglab.ca/~abeinges/blah/rust-unsafe-intro/). Gankro is thinking about what unsafety means.
* [Introducing the newest member of Mozilla's Rust team: Emily Dunham](https://internals.rust-lang.org/t/introducing-the-newest-member-of-mozillas-rust-team-emily-dunham/2143). Mozilla Research has a devops person now.
* [Wrapper Types in Rust: Choosing Your Guarantees](http://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees/).
* The Rust Essentials book from Packt [has been released](http://www.reddit.com/r/rust/comments/37hm4o/rust_essentials_book_now_officially_published/).
* [lifeguard](http://www.reddit.com/r/rust/comments/37ghej/lifeguard_an_autocollecting_object_pool/). A memory pool.
* [gj](http://dwrensha.github.io/capnproto-rust/2015/05/25/asynchronous-io-with-promises.html). Async I/O with promises, from the author of capnp-rust.
* Geoffoy Couprie [published a paper on Nom](http://spw15.langsec.org/papers/couprie-nom.pdf), his byte-oriented parser combinator library
* [rim](https://github.com/mathall/rim). A vim-like text editor.
* [Rust documentation is available on DevDocs](http://devdocs.io/rust/).
* [ioctl](https://github.com/cmr/ioctl). A crate to help make ioctls, from cmr.
* [ramp](https://crates.io/crates/ramp). A multi-precision arithmetic library from Aatch.

# Upcoming Events

* [6/8. Seattle](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg)
* [6/9. San Diego](http://sandiego.rs)
* [6/14. Hyderabad](https://www.eventbrite.com/e/rust-meetup-hyderabad-tickets-17213665537)
* [6/15. Paris](http://www.meetup.com/Rust-Paris).
* [6/17. Los Angeles](http://www.meetup.com/Rust-Los-Angeles/events/222656434/)
* [6/17. Montreal](http://www.meetup.com/Montreal-Rust-Language-Meetup/events/223045701/)
* [6/24. Columbus Rust Society](http://www.meetup.com/columbus-rs/)
* [6/29. Sydney](http://www.meetup.com/Rust-Sydney/events/222811456/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# Quote of the Week

*"Dude, you can't just tell people it's the secret plan; then it won't be a secret any more! You keep this up, and you're going to get your Secret Rust Club card revoked! Then you won't be able to get on the awesome Secret Rust Zeppelin. Don't screw this up, man!"*

Quxxy, from [/r/rust](http://www.reddit.com/r/rust/comments/36z2n0/rust_made_me_a_worse_c_programmer_right/cribpl5).

*"The 1st rule of Secret Rust Club is: you don't talk about Secret Rust Club.*

*The 2nd rule of Secret Rust Club is: error: `1st rule` does not live long enough.*

*error: aborting due to previous error"*

JakDrako, from the same thread.

Thanks to drbawb and Manishearth for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
