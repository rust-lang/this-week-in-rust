Title: This Week in Rust 351
Number: 351
Date: 2020-08-11
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/emberian/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*](https://rustacean-station.org/episode/026-twir-350/)

# Updates from Rust Community

No official Rust announcements this week! :)

### Tooling
* [Rust Analyzer Changelog #37](https://rust-analyzer.github.io/thisweek/2020/08/10/changelog-37.html)
* [IntelliJ Rust Changelog #128](https://intellij-rust.github.io/2020/08/10/changelog-128.html)

### Newsletters
* [This Month in Rust GameDev #12 - July 2020](https://rust-gamedev.github.io/posts/newsletter-012/)
* [Rust in Blockchain #14 - Are We Smart (Contract) Yet?](https://rustinblockchain.org/newsletters/2020-08-05-are-we-smart-contract-yet/)

### Observations/Thoughts
* [Steve Klabnik Interview](https://evrone.com/steve-klabnik-interview)
* [Why learning Rust is great... As a second language](https://beyondtheloop.dev/rust-second-language/)
* [Why QEMU should move from C to Rust](http://blog.vmsplice.net/2020/08/why-qemu-should-move-from-c-to-rust.html?m=1)
* [First Impressions of Rust](https://john-millikin.com/first-impressions-of-rust)
* [How to Stick with Rust](https://dev.to/pieohpah/how-to-stick-with-rust-1gpf)
* [Why learning Rust is great...As a second language](https://dev.to/blorente/why-learning-rust-is-great-as-a-second-language-5583)
* [What Is The Minimal Set Of Optimizations Needed For Zero-Cost Abstraction?](https://robert.ocallahan.org/2020/08/what-is-minimal-set-of-optimizations.html)
* [Propane: an experimental generator syntax for Rust](https://without.boats/blog/propane/)
* [ES] [¿Por qué me gusta tanto Rust?](https://blog.categulario.tk/por-que-me-gusta-tanto-rust.html)

### Learn Standard Rust
* [Rust for a Pythonista #2: Building a Rust crate for CSS inlining](https://dygalo.dev/blog/rust-for-a-pythonista-2/)
* [Surviving Rust async interfaces](https://fasterthanli.me/articles/surviving-rust-async-interfaces)
* [Two Easy Ways to Test Async Functions in Rust](https://blog.x5ff.xyz/blog/async-tests-tokio-rust/)
* [Cloning yourself - a refactoring for thread-spawning Rust types](https://www.philipdaniels.com/blog/2020/self-cloning-for-multiple-threads-in-rust/)
* [Allocation API, allocators and virtual memory](https://notes.iveselov.info/programming/allocation-api-and-allocators)
* [Variables and memory management in Rust](https://willcrichton.net/notes/type-safe-printf/)
* [ES] [Polimorfismo con traits en Rust](https://emanuelpeg.blogspot.com/2020/08/polimorfismo-con-traits-en-rust.html#.Xy7c6ukK2fc.reddit)
* [PT] [Aprendendo Rust: 06 - Controles de fluxo](https://dev.to/pehdepano/aprendendo-rust-06-controles-de-fluxo-4mff)
* [PT] [Meia hora aprendendo Rust - Parte 1](https://rodolfoghi.github.io/posts/meia-hora-aprendendo-rust-parte1/)
* [video] [Crust of Rust: Channels](https://www.youtube.com/watch?v=b4mS5UPHh20)

### Learn More Rust
* [Zero To Production #3: How To Bootstrap A Rust Web API From Scratch](https://www.lpalmieri.com/posts/2020-08-09-zero-to-production-3-how-to-bootstrap-a-new-rust-web-api-from-scratch/)
* [Using Rust Lambdas in Production](https://www.cvpartner.com/blog/using-rust-lambdas-in-production)
* [Incorporate Postgres with Rust](https://blog.knoldus.com/incorporate-postgres-with-rust/)
* [Let's implement a Bloom Filter](https://onatm.dev/2020/08/10/let-s-implement-a-bloom-filter/)
* [A Guide to Contiguous Data in Rust](https://github.com/paulkernfeld/contiguous-data-in-rust)
* [Inbound & Outbound FFI](https://www.possiblerust.com/guide/inbound-outbound-ffi)
* [Tutorial: Deno Apps with WebAssembly, Rust, and WASI](https://www.secondstate.io/articles/deno-webassembly-rust-wasi/)
* [Single Page Applications using Rust](http://www.sheshbabu.com/posts/rust-wasm-yew-single-page-application/)
* [Implementing a Type-safe printf in Rust](https://willcrichton.net/notes/type-safe-printf/)
* [Building a Brainf*ck Compiler with Rust and LLVM](https://benkonz.github.io/building-a-brainfuck-compiler-with-rust-and-llvm/)
* [Modernize network function development with this Rust-based framework](https://opensource.com/article/20/8/capsule-networking)
* [Let's build a RedditBot to curate playlist links - I](https://dev.to/delta_maniac/lets-build-a-redditbot-to-curate-playlist-links-1k4o)
* [Let's build a RedditBot to curate playlist links - II](https://dev.to/delta_maniac/lets-build-a-redditbot-to-curate-playlist-links-ii-48kf)
* [Rust on iOS with SDL2](https://blog.aclysma.com/rust-on-ios-with-sdl2/)
* [video] [Using Linux libc in Rust - with the file-locker Crate](https://youtu.be/UgNrDb6hQQ0)
* [video] [Embedded Rust Mob Programming](https://youtu.be/BZqt187RWTw)
* [video] [Implementing TCP in Rust (part 1)](https://youtu.be/bzja9fQWzdA)

### Project Updates
* [Knurling-rs Announcement](https://ferrous-systems.com/blog/knurling-rs/)
* [Meili raises 1.5M€ for open source search in Rust](https://blog.meilisearch.com/meili-fundraise/)
* [hors - instant coding answers via the command line have released v0.6.4, with pretty colorized output by default](https://www.reddit.com/r/rust/comments/i55s5z/hors_instant_coding_answers_via_the_command_line/)

### Miscellaneous
* [Building a faster CouchDB View Server in Rust](https://www.garrensmith.com/blogs/fortuna-rs-couchdb-view-server)
* [The promise of Rust async-await for embedded](https://tweedegolf.nl/blog/41/the-promise-of-rust-async-await-for-embedded)
* [Implementing the .NET Profiling API in Rust](https://code.camdenreslink.com/dev/implementing-the-dot-net-profiling-api-in-rust/)
* [Possibly one step towards named arguments in Rust](https://yoric.github.io/post/rust-named-arguments/)
* [Embedded Rust tooling for the 21st century](https://ferrous-systems.com/blog/21st-century-embedded-tooling/)
* [rustc 1.44.1 is reproducible in Debian](https://www.reddit.com/r/rust/comments/i4ij47/rustc_1441_is_reproducible_in_debian/)
* [Google engineers just submitted a new LLVM optimizer for consideration which gains an average of 2.33% perf.](https://www.reddit.com/r/rust/comments/i44ahg/google_engineers_just_submitted_a_new_llvm/)

# Crate of the Week

This week's crate is [bevy](https://crates.io/crates/bevy), a very capable yet simple game engine.

Thanks to [mmmmib](https://users.rust-lang.org/t/crate-of-the-week/2704/798) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [cargo: Deduplicate Cargo workspace
  information](https://github.com/rust-lang/cargo/issues/8415)
* [dotenv-linter: several good first issues](https://github.com/dotenv-linter/dotenv-linter/issues?q=is%3Aopen+is%3Aissue+label%3A%22help+wanted%22)
* [ruma: several help wanted issues](https://github.com/ruma/ruma/labels/help%20wanted)
* [tensorbase: several good first issues](https://github.com/tensorbase/tensorbase/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
* [kanidm: several good first issues](https://github.com/kanidm/kanidm/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
* [Libre-SOC's first SoC: Add PowerPC64 to Rust's new inline assembly implementation](https://bugs.libre-soc.org/show_bug.cgi?id=451)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

307 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-08-03..2020-08-10

* [add back unwinding support for Sony PSP](https://github.com/rust-lang/rust/pull/75280)
* [fix ICE when using asm! on an unsupported architecture](https://github.com/rust-lang/rust/pull/75227)
* [handle well known traits for more types](https://github.com/rust-lang/chalk/pull/591)
* [resolve `char` as a primitive even if there is a module in scope](https://github.com/rust-lang/rust/pull/75318)
* [forbid `#[track_caller]` on main](https://github.com/rust-lang/rust/pull/75130)
* [remove restriction on type parameters preceding consts w/ feature const-generics](https://github.com/rust-lang/rust/pull/74953)
* [implement the `min_const_generics` feature gate](https://github.com/rust-lang/rust/pull/74877)
* [tweak confusable idents checking](https://github.com/rust-lang/rust/pull/75349)
* [miri: accept some post-monomorphization errors](https://github.com/rust-lang/miri/pull/1503)
* [bubble up errors from `FileDescriptor::as_file_handle`](https://github.com/rust-lang/miri/pull/1501)
* [simplify `array::IntoIter`](https://github.com/rust-lang/rust/pull/75271)
* [polymorphize: unevaluated constants](https://github.com/rust-lang/rust/pull/75260)
* [instance: polymorphize upvar closures/generators](https://github.com/rust-lang/rust/pull/75255)
* [clean up const-hacks in int endianess conversion functions](https://github.com/rust-lang/rust/pull/75253)
* [add `as_mut_ptr` to `NonNull<[T]>`](https://github.com/rust-lang/rust/pull/75248)
* [make `MaybeUninit::as_`(`mut_`)`ptr` const](https://github.com/rust-lang/rust/pull/75250)
* [make `IntoIterator` lifetime bounds of `&BTreeMap` match with `&HashMap`](https://github.com/rust-lang/rust/pull/75203)
* [implement `into_keys` and `into_values` for associative maps](https://github.com/rust-lang/rust/pull/75163)
* [stabilize `Ident::new_raw`](https://github.com/rust-lang/rust/pull/75084)
* [limit I/O vector count on Unix](https://github.com/rust-lang/rust/pull/75005)
* [add `unsigned_abs` to signed integers](https://github.com/rust-lang/rust/pull/74759)
* [BTreeMap: better way to postpone root access in DrainFilter](https://github.com/rust-lang/rust/pull/75257)
* [hashbrown: do not iterate to drop if empty](https://github.com/rust-lang/hashbrown/pull/182)
* [hashbrown: relax bounds on HashSet constructors](https://github.com/rust-lang/hashbrown/pull/185)
* [hashbrown: avoid closures to improve compile times](https://github.com/rust-lang/hashbrown/pull/183)
* [stdarch: add more things that do adds](https://github.com/rust-lang/stdarch/pull/881)
* [futures: avoid writes without any data in write_all_vectored](https://github.com/rust-lang/futures-rs/pull/2187)
* [clean up rustdoc's `main()`](https://github.com/rust-lang/rust/pull/75124)
* [rustdoc: display elided lifetime for non-reference type in doc](https://github.com/rust-lang/rust/pull/75237)

## Rust Compiler Performance Triage

* [2020-08-11](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-08-11.md).
  1 regression, 1 improvements, 1 of them on rollups. No outstanding nags from last week.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

*No Tracking Issues or PRs are currently in the final comment period.*

## New RFCs

* [Proposal for POSIX error numbers in `std::os::unix`](https://github.com/rust-lang/rfcs/pull/2973)
* [Standardize methods for leaking containers](https://github.com/rust-lang/rfcs/pull/2969)
* [Introduce '$self' macro metavar for hygienic macro items](https://github.com/rust-lang/rfcs/pull/2968)

# Upcoming Events

### Online
* [August 11. Saarbrücken, DE - Rust-Saar Meetup `3u16`](https://www.meetup.com/Rust-Saar/events/272044845/)
* [August 11. Dallas, TX, US - Dallas Rust - Second Tuesday](https://www.meetup.com/Dallas-Rust/events/mzzfsrybclbpb/)
* [August 13. San Diego, CA, US - San Diego Rust - August 2020 Tele-Meetup](https://www.meetup.com/San-Diego-Rust/events/272060817/)
* [August 19. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out Night](https://www.meetup.com/Vancouver-Rust/events/vcgsvrybclbzb/)
* [August 20. RustConf](https://rustconf.com/)

### North America
* [August 13. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybclbrb/)
* [August 25. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/nppvrrybclbhc/)

### Asia Pacific
* [August 18. Seoul, KR - Rust Meetup - Rust last 6 months review (also available on Zoom)](https://www.meetup.com/Rust-Seoul-Meetup/events/qfkdvrybclbxb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Rust Engineer at equilibrium (Remote)](https://www.notion.so/Hiring-Senior-Rust-Engineer-e6c94ccc261f426c80a483c7fc642412)
* [Software Developer at DerivaDEX (Remote)](https://angel.co/company/derivadex/jobs/917157-software-developer)
* [Rust Core Engineer at CasperLabs (Remote)](https://apply.workable.com/casperlabs/j/85A300F063/)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> You're not allowed to use references in structs until you think Rust is easy. They're the evil-hardmode of Rust that will ruin your day.

- [Kornel on rust-users](https://users.rust-lang.org/t/perpetual-n00b-struggling-with-ownership-again/46920/4)

Thanks to [Tom Phinney](https://users.rust-lang.org/t/twir-quote-of-the-week/328/918) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/i094wo/this_week_in_rust_349/)</small>
