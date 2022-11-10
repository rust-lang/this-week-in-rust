Title: This Week in Rust 468
Number: 468
Date: 2022-11-09
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the Linked Page](https://example.com/my_article)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official
* [Announcing Rust 1.65.0](https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html)

### Project/Tooling Updates
* [rust-analyzer changelog #154](https://rust-analyzer.github.io/thisweek/2022/11/07/changelog-154.html)
* [IntelliJ Rust Changelog #182](https://intellij-rust.github.io/2022/11/07/changelog-182.html)
* [This Month in Rust OSDev: October 2022 | Rust OSDev](https://rust-osdev.com/this-month/2022-10/)
* [This Week in Fyrox](https://fyrox.rs/blog/post/twif/)
* [Cap'n Proto - Generic Associated Types](https://dwrensha.github.io/capnproto-rust/2022/11/03/0.15-release.html)
* [Gitoxide - Cloning the Linux kernel in under a minute](https://github.com/Byron/gitoxide/discussions/579)
* [This week in Fluvio #48: The programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0048/)

### Observations/Thoughts
* [Scoped threads in Rust, and why its async counterpart would be unsound](https://wishawa.github.io/posts/thread-scoped-async/)
* [34 Must Know Terms for Embedded Rust Newbies](https://apollolabsblog.hashnode.dev/34-must-know-terms-for-embedded-rust-newbies)
* [what if pointer properties were in the rust type system](https://cohost.org/myrrlyn/post/197200-what-if-pointer-prop)
* [mmap(1Tb): A Rust arena allocator (ab)using Linux overcommit](https://vgel.me/posts/mmap-arena-alloc/)
* [Contributing to Rust: Bootstrapping the Rust Compiler (rustc)](https://www.youtube.com/watch?v=oG-JshUmkuA)
* [The ‘Viral’ Secure Programming Language That’s Taking Over Tech](https://www.wired.com/story/rust-secure-programming-language-memory-safe/)
* [FR] [Rust par le Métal : move, clone, copy](https://lafor.ge/rust/move/) (series)
* [FR] [Rust par le Métal (Annexe) : Les bases de nombres](https://lafor.ge/rust/annex_bases/) (series)

### Rust Walkthroughs
* [Tauri vs. Electron for Tray Apps](https://betterprogramming.pub/tauri-vs-electron-for-tray-apps-ed15974f35ce)
* [Running Rust in AWS Lambda with SAM — Part 1](https://medium.com/@shyamsundarb/running-rust-in-aws-lambda-with-sam-part-1-2b7a6963a2ef)
* [Tower, Episode 2: calling a Tower Service ](https://heikoseeberger.de/2022/10/23/2022-10-23-tower-2/)
* [Writing an eBPF/XDP Load-Balancer in Rust](https://konghq.com/blog/writing-an-ebpf-xdp-load-balancer-in-rust)
* [Floating Point Arcade](https://gist.github.com/CrockAgile/09065649ae5a52629599ebc5645922d6)
* [video] [Cost-Efficient Rust in Practice, Rust Tokyo](https://www.youtube.com/watch?v=UkfO5KBFSpY)

### Research
* [Can Rust-wrapped C++ offer performance and stability benefits?](https://medium.com/@adetaylor/can-rust-wrapped-c-offer-stability-and-performance-benefits-e140b7ca1ba9)

### Miscellaneous
* [Lynx Joins AdaCore and Ferrous Systems to Bring Rust to Embedded Developers](https://www.lynx.com/press-releases/rust-compiler-support)
* [video] [RUST 1.65 - let-else statements - new language feature!!!](https://www.youtube.com/watch?v=U-5_bumwH9w)
* [video] [The Rust Programming Language for Game Tooling](https://www.youtube.com/watch?v=GtRo-eF8-TE)
* [video] [Bencher—Catch Performance Regressions in CI @ Rust DC, September 20, 2022 w/ Everett Pompeii](https://www.youtube.com/watch?v=Xw9loUatF7k)
* [video] [Impractical Rust: The HATETRIS World Record @ Rust DC, October 18, 2022 w/Dave Freiberg+Felipe Suero](https://www.youtube.com/watch?v=UgQUvD9gyMk)
* [DE] [Programmiersprache Rust 1.65 führt Generic Associated Types ein](https://www.heise.de/news/Programmiersprache-Rust-1-65-fuehrt-Generic-Associated-Types-ein-7329903.html)
* [DE] [Ferris Talk #12: Web-APIs mit Rust erstellen](https://www.heise.de/hintergrund/Ferris-Talk-12-Web-APIs-mit-Rust-erstellen-7321340.html)

## Crate of the Week

This week's crate is [enum_delegate](https://crates.io/crates/enum_delegate), a crate to replace dynamic dispatch with enum dispatch.

Thanks to [Devin Brite](https://users.rust-lang.org/t/crate-of-the-week/2704/1120) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
[FOSDEM 2023 Rust devroom CFP](https://rust-fosdem.github.io)
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

396 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-10-31..2022-11-07

* [`(almost)` always use `ObligationCtxt` when dealing with canonical queries](https://github.com/rust-lang/rust/pull/103590)
* [`fix(generic_const_exprs): `fix predicate inheritance for children of opaque types](https://github.com/rust-lang/rust/pull/99801)
* [make cpp-like debuginfo type names for slices and str consistent](https://github.com/rust-lang/rust/pull/103691)
* [track where diagnostics were created](https://github.com/rust-lang/rust/pull/103217)
* [`rustc_metadata`: encode even less doc comments](https://github.com/rust-lang/rust/pull/103496)
* [add `multivalue` target feature to WASM target](https://github.com/rust-lang/rust/pull/103638)
* [allow use of `-Clto=thin` with `-Ccodegen-units=1` in general](https://github.com/rust-lang/rust/pull/103610)
* [lint against usages of `struct_span_lint_hir`](https://github.com/rust-lang/rust/pull/103720)
* [stable Lower lint level for `read_zero_byte_vec`](https://github.com/rust-lang/rust/pull/103859)
* [reduce span of let else `irrefutable_let_patterns` warning](https://github.com/rust-lang/rust/pull/103749)
* [suggest use `..` to fill in the rest of the struct fields](https://github.com/rust-lang/rust/pull/103012)
* [better error for `rustc_strict_coherence` misuse](https://github.com/rust-lang/rust/pull/103772)
* [correctly resolve Inherent Associated Types](https://github.com/rust-lang/rust/pull/103621)
* [don't silently eat label before block in block-like expr](https://github.com/rust-lang/rust/pull/103986)
* [interpret: fix `align_of_val` on packed types](https://github.com/rust-lang/rust/pull/103729)
* [make `proc_macro_derive_resolution_fallback` a hard error](https://github.com/rust-lang/rust/pull/84022)
* [make `underscore_literal_suffix` a hard error](https://github.com/rust-lang/rust/pull/103914)
* [normalize types when deducing closure signature from supertraits](https://github.com/rust-lang/rust/pull/104038)
* [miri: implement condvars for Windows](https://github.com/rust-lang/miri/pull/2638)
* [miri: initOnce: synchronize with completion when already complete](https://github.com/rust-lang/miri/pull/2641)
* [rewrite implementation of `#[alloc_error_handler]`](https://github.com/rust-lang/rust/pull/103061)
* [remove bounds check when array is indexed by enum](https://github.com/rust-lang/rust/pull/103584)
* [stabilize the `instruction_set` feature](https://github.com/rust-lang/rust/pull/102458)
* [implement `std::marker::Tuple`, use it in `extern "rust-call"` and `Fn`-family traits](https://github.com/rust-lang/rust/pull/99943)
* [futures: do not require `Clone` for `Shared::peek`](https://github.com/rust-lang/futures-rs/pull/2662)
* [libtest: run all tests in their own thread, if supported by the host](https://github.com/rust-lang/rust/pull/103681)
* [rustdoc: fix merge of attributes for reexports of local items](https://github.com/rust-lang/rust/pull/103886)
* [rustdoc: make `Item::visibility` computed on-demand](https://github.com/rust-lang/rust/pull/103690)
* [bindgen: add support for the `"C-unwind"` ABI](https://github.com/rust-lang/rust-bindgen/pull/2334)
* [bindgen: add the `--override-abi` option](https://github.com/rust-lang/rust-bindgen/pull/2329)
* [bindgen: allow callback composition](https://github.com/rust-lang/rust-bindgen/pull/2330)
* [bindgen: wrap unsafe function's bodies in unsafe blocks](https://github.com/rust-lang/rust-bindgen/pull/2266)
* [clippy: add `allow-print-in-tests` config](https://github.com/rust-lang/rust-clippy/pull/9797)
* [clippy: add new lint `let_underscore_future`](https://github.com/rust-lang/rust-clippy/pull/9760)
* [clippy: extend `needless_collect`](https://github.com/rust-lang/rust-clippy/pull/8744)
* [clippy: fix ICE in `redundant_allocation`](https://github.com/rust-lang/rust-clippy/pull/9773)
* [clippy: fix `unnecessary_join` turbofish in suggest message](https://github.com/rust-lang/rust-clippy/pull/9779)
* [clippy: improve `needless_lifetimes`](https://github.com/rust-lang/rust-clippy/pull/9743)
* [clippy: move `needless_collect` to nursery](https://github.com/rust-lang/rust-clippy/pull/9705)
* [clippy: shrink `missing_`{`safety`, `errors`, `panics`}`_doc` spans](https://github.com/rust-lang/rust-clippy/pull/9772)
* [rust-analyzer: add "Convert match to `let-else`" assist](https://github.com/rust-lang/rust-analyzer/pull/13516)
* [rust-analyzer: add config for inserting `must_use` in `generate_enum_as_method`](https://github.com/rust-lang/rust-analyzer/pull/13359)
* [rust-analyzer: extracted method from trait impl is placed in existing impl](https://github.com/rust-lang/rust-analyzer/pull/12991)
* [rust-analyzer: generalize reborrow hints as adjustment hints](https://github.com/rust-lang/rust-analyzer/pull/13545)
* [rust-analyzer: show signature help when calling generic types implementing `FnOnce`](https://github.com/rust-lang/rust-analyzer/pull/13525)
* [rust-analyzer: fix the length displayed for byte string literals with escaped newlines](https://github.com/rust-lang/rust-analyzer/pull/13568)
* [rust-analyzer: async trait method for `unnecessary_async`](https://github.com/rust-lang/rust-analyzer/pull/13508)
* [rust-analyzer: fix reference searching only accounting substrings instead of whole identifiers](https://github.com/rust-lang/rust-analyzer/pull/13549)
* [rust-analyzer: make custom expr prefix completions to understand refs](https://github.com/rust-lang/rust-analyzer/pull/13517)
* [rust-analyzer: fixed local shadowing the caller's argument issue](https://github.com/rust-lang/rust-analyzer/pull/13454)
* [rust-analyzer: lower unsafety of fn pointer and fn item types](https://github.com/rust-lang/rust-analyzer/pull/13546)
* [rust-analyzer: migrate assists to format args captures, part 3](https://github.com/rust-lang/rust-analyzer/pull/13435)
* [rust-analyzer: scip: generate symbols for local crates](https://github.com/rust-lang/rust-analyzer/pull/13456)

### Rust Compiler Performance Triage

A relatively noisy week (most of those have been dropped below, and comments
left on GitHub), but otherwise a quiet one in terms of performance changes,
with essentially no significant changes occuring.

Triage done by **@simulacrum**.
Revision range: [822f8c2..57d3c58](https://perf.rust-lang.org/?start=822f8c22f540b12f296d844ad5bf39aaa47bfeb4&end=57d3c58ed6e0faf89a62411f96c000ffc9fd3937&absolute=false&stat=instructions%3Au)

2 Regressions, 2 Improvements, 3 Mixed; 3 of them in rollups
39 artifact comparisons made in total

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-11-08.md) for more details.

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [crates.io token scopes](https://github.com/rust-lang/rfcs/pull/2947)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: close] [RFC 3283: Backward compatible default features](https://github.com/rust-lang/rfcs/pull/3283)
* [disposition: close] [RFC: Rust SemVer 2](https://github.com/rust-lang/rfcs/pull/3266)
* [disposition: postpone] [Add named path bases to cargo](https://github.com/rust-lang/rfcs/pull/3074)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* *No Tracking Issues or PRs entered Final Comment Period this week.*

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [Create an Operational Semantics Team](https://github.com/rust-lang/rfcs/pull/3346)
* [new] [Add a `--compile-time-deps` build flag to `cargo build`](https://github.com/rust-lang/rfcs/pull/3344)
* [new] [RFC: Anonymous Associated Types](https://github.com/rust-lang/rfcs/pull/3342)

## Upcoming Events

Rusty Events between 2022-11-09 - 2022-12-07 🦀

### Virtual

* 2022-11-09 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Introduction to Rust Atomics**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/289052285/)
* 2022-11-09 | Virtual (Darmstadt, DE) | [betterCode](https://www.bettercode.eu/)
    * [**betterCode Rust**](https://rust.bettercode.eu/)
* 2022-11-09 | Virtual (Malaysia, MY) | [Rust Malaysia](https://forms.gle/zWXcMDAnnibiL4ni9)
    * [**Rust Meetup November 2022 - a couple of lightning talks**](https://discord.gg/9Xj8H2EXTD)
* 2022-11-10 | Virtual (Budapest, HU) | [HWSW free!](https://www.meetup.com/hwswfree/)
    * [**RUST! RUST! RUST! meetup (online formában!)**](https://www.meetup.com/hwswfree/events/289044458/)
* 2022-11-10 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #19**](https://www.meetup.com/rust-noris/events/hlvbvsydcpbnb/)
* 2022-11-12 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://www.google.com/url?q=https%3A%2F%2Fdiscord.gg%2FyNtPTb2&sa=D&ust=1666661760000000&usg=AOvVaw13uHY9m-8bJJwgeP58VS8l)
* 2022-11-15 | Virtual (Nairobi, KE / New York, NY, US)| [Data Umbrella Africa](https://www.meetup.com/data-umbrella-africa2/)
    * [**Online: Introduction to Rust Programming**](https://www.meetup.com/data-umbrella-africa2/events/289308825/) | [**New York Mirror**](https://www.meetup.com/data-umbrella/events/289308172/)
* 2022-11-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc//)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/289015883/)
* 2022-11-16 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/tqvhxsydcpbvb/)
* 2022-11-17 | Virtual (Amsterdam, NL) | [ITGilde Tech-Talks](https://www.meetup.com/itgilde-cooperatie-amsterdam-unix-linux-meetups)
    * [**Introduction “Rust” an ITGilde Tech Talk delivered by Pascal van Dam**](https://www.meetup.com/itgilde-cooperatie-amsterdam-unix-linux-meetups/events/289167373/)
* 2022-11-17 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydcpbwb/)
* 2022-11-21 | Virtual (Paris, FR) | [Meetup Paris - École Supérieure de Génie Informatique (ESGI)](https://www.meetup.com/meetup-paris-ecole-superieur-du-genie-informatique)
    * [**Découverte de WebAssembly**](https://www.meetup.com/meetup-paris-ecole-superieur-du-genie-informatique/events/289112753/)
* 2022-11-22 | Virtual (Berlin, DE) | [Berlin.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2022/11/22/rust-hack-and-learn.html)
* 2022-11-24 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 27th Edition**](https://www.meetup.com/rust-linz/events/289251460/)
* 2022-11-29 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcpbmc/)
* 2022-11-30 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 3 - hybrid**](https://www.meetup.com/rust-munich/events/289065390/)
* 2022-12-01 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Exploring USB with Rust**](https://www.meetup.com/charlottesville-rust-meetup/events/289563986/)
* 2022-12-06 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/hlgvxsydcqbjb/)
* 2022-12-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/287027660/)

### Europe

* 2022-11-15 | Tampere, FI | [Rust Finland](https://www.rust-finland.org/)
    * [**Rust Meetup in Tampere**](https://www.lyyti.fi/reg/Finland_Rustlang_Group_meetup_2564)
* 2022-11-16 | Paris, FR | [Stockly](https://www.eventbrite.fr/o/stockly-42274765293)
    * [**Rust Meetup in Paris - hosted by Stockly**](https://www.eventbrite.fr/e/rust-meetup-in-paris-hosted-by-stockly-tickets-444152621447?aff=ebdssbdestsearch)
* 2022-11-23 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Initial Meet and Greet Rust meetup**](https://www.meetup.com/bratislava-rust-meetup-group/events/289028178/)
* 2022-11-24 | København, DK | [Copenhagen Rust Group](https://cph.rs/)
    * [**Hack Night #31**](https://www.meetup.com/copenhagen-rust-meetup-group/events/288179132/)
* 2022-11-30 | Amsterdam, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Rust in Critical Infrastructure**](https://www.meetup.com/rust-nederland/events/289204146/)
* 2022-11-30 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet)
    * [**Rust Lille #1**](https://www.meetup.com/meetup-group-zgphbyet/events/289620614/)
* 2022-11-30 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 3 - hybrid**](https://www.meetup.com/rust-munich/events/289065390/)

### North America

* 2022-11-10 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/events/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcpbnb/)
* 2022-11-15 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcpbtb/)
* 2022-11-29 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**Atx Rustaceans Meetup**](https://www.meetup.com/atx-rustaceans/events/289594614/)
* 2022-12-01 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Utah Rust meetup**](https://www.meetup.com/utah-rust/events/dsbpxsydcpbgc/)

### Oceania

* 2022-11-09 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**RustAU Sydney - Last physical for 2022 !**](https://www.meetup.com/rust-sydney/events/289061840/)
* 2022-11-22 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**November Meetup**](https://www.meetup.com/rust-canberra/events/288615873/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org


<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/ymepy8/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Meanwhile the Rust shop has covers on everything and tag-out to even change settings of the multi-axis laser cutter, but you get trusted with said laser cutter on your first day, and if someone gets hurt people wonder how to make the shop safer.

– [masklinn on r/rust](https://www.reddit.com/r/rust/comments/yo6ju6/comment/ivdxbdw)

Thanks to [Anton Fetisov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1329) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/yr1z35/this_week_in_rust_468/)</small>
