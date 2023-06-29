Title: This Week in Rust 501
Number: 501
Date: 2023-06-28
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
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
* [Improved API tokens for crates.io](https://blog.rust-lang.org/2023/06/23/improved-api-tokens-for-crates-io.html)

### Foundation

### Newsletters

### Project/Tooling Updates
* [rust-analyzer changelog #187](https://rust-analyzer.github.io/thisweek/2023/06/26/changelog-187.html)

### Observations/Thoughts
* [Rust fact vs. fiction: 5 Insights from Google's Rust journey in 2022](https://opensource.googleblog.com/2023/06/rust-fact-vs-fiction-5-insights-from-googles-rust-journey-2022.html)
* [Escaping Macrophages](https://www.unscript.net/redblog/posts/trait-impl-macros/)
* [Code coverage in Rust](https://rrmprogramming.com/article/code-coverage-in-rust/)
* [video] [Verus - Verified Rust for low-level systems code by Andrea Lattuada](https://www.youtube.com/watch?v=ZZTk-zS4ZCY)
* [audio] [Daily with Kwindla Hultman Kramer :: Rustacean Station](https://rustacean-station.org/episode/kwindla-hultman-kramer/)
* [audio] [Fish Folk with Erlend Sogge Heggen :: Rustacean Station](https://rustacean-station.org/episode/erlend-sogge-heggen/)

### Rust Walkthroughs
* [Serde Errors When Deserializing Untagged Enums Are Bad - But Easy to Make Better](https://www.gustavwengel.dk/serde-untagged-enum-errors-are-bad)
* [Blowing up my compile times for dubious benefits](https://claytonwramsey.github.io/2023/06/20/fiddler-const-magic.html)
* [Walk-Through: Prefix Ranges in Rust, a Surprisingly Deep Dive](https://www.thecodedmessage.com/posts/prefix-ranges/)
* [ESP32 Embedded Rust at the HAL: Remote Control Peripheral](https://apollolabsblog.hashnode.dev/esp32-embedded-rust-at-the-hal-remote-control-peripheral)
* [video] [Writing a Rust-based ring buffer](https://www.youtube.com/watch?v=TQVwv_e_rMw)
* [video] [Supercharge your I/O in Rust with io_uring](https://www.youtube.com/watch?v=IHAPVK1nOrQ)
* [video] [Graph Traversal with Breadth-First Search in Rust](https://www.youtube.com/watch?v=ZDy3tqn-DKA)
* [video] [Nine Rules for Writing Python Extensions in Rust | PyData Seattle 2023](https://www.youtube.com/watch?v=B6E0Jb6yj34)

### Research
* [Friend or Foe Inside? Exploring In-Process Isolation to Maintain Memory Safety for Unsafe Rust](https://arxiv.org/abs/2306.08127)
* [Agile Development of Linux Schedulers with Ekiben](https://arxiv.org/abs/2306.15076)

### Miscellaneous

* [How to Deploy Cross-Platform Rust Binaries with GitHub Actions](https://dzfrias.dev/blog/deploy-rust-cross-platform-github-actions)

## Crate of the Week

This week's crate is [Parsel](https://docs.rs/parsel), an easy to use parser generator.

Thanks to [jacg](https://users.rust-lang.org/t/crate-of-the-week/2704/1208) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [RustQuant - Issue 56: Implementing Logistic Regression: weights matrix becomes singular.](https://github.com/avhz/RustQuant/issues/56)
* [RustQuant - Issue 30: Error handling for the library.](https://github.com/avhz/RustQuant/issues/30)
* [RustQuant - Issue 22: Pricing model calibration module.](https://github.com/avhz/RustQuant/issues/22)
* [RustQuant - Issue 14: Add/improve documentation (esp. math related docs).](https://github.com/avhz/RustQuant/issues/14)
* [RustQuant - Issue 57: Increase test coverage (chore).](https://github.com/avhz/RustQuant/issues/57)
* [Ockam - Add API endpoint to retrieve the project's version](https://github.com/build-trust/ockam/issues/4062)
* [Ockam - Validate the credential before storing it](https://github.com/build-trust/ockam/issues/4380)
* [Ockam - Update CLI documentation for `lease` commands](https://github.com/build-trust/ockam/issues/4778)
* [Send File - create hotspot on Linux operating system](https://github.com/opeolluwa/send-file/issues/83)
* [Send File - Get device storage information (used disk size and total memory)](https://github.com/opeolluwa/send-file/issues/137)
* [Send File - Create hotspot on Windows Operating system](https://github.com/opeolluwa/send-file/issues/132)
* [Send File - Use Tauri store plugin to persist app data](https://github.com/opeolluwa/send-file/issues/29)
* [mirrord - Alert user when running on OpenShift](https://github.com/metalbear-co/mirrord/issues/1560)
* [mirrord - Add integration test for listening on the same port again after closing](https://github.com/metalbear-co/mirrord/issues/1552)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

400 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-06-19..2023-06-26

* [syntactically accept `become` expressions (explicit tail calls experiment)](https://github.com/rust-lang/rust/pull/112790)
* [`hir`: Add `Become` expression kind (explicit tail calls experiment)](https://github.com/rust-lang/rust/pull/112887)
* [add esp-idf missing targets](https://github.com/rust-lang/rust/pull/112559)
* [accept `ReStatic` for RPITIT](https://github.com/rust-lang/rust/pull/113036)
* [account for sealed traits in privacy and trait bound errors](https://github.com/rust-lang/rust/pull/112686)
* [add support for NetBSD/aarch64-be (big-endian arm64)](https://github.com/rust-lang/rust/pull/111326)
* [always register sized obligation for argument](https://github.com/rust-lang/rust/pull/112643)
* [better error for non const `PartialEq` call generated by `match`](https://github.com/rust-lang/rust/pull/112232)
* [don't ICE on unnormalized `struct` tail in layout computation](https://github.com/rust-lang/rust/pull/112810)
* [don't structurally resolve during method ambiguity in probe](https://github.com/rust-lang/rust/pull/111747)
* [don't substitute a GAT that has mismatched generics in `OpaqueTypeCollector`](https://github.com/rust-lang/rust/pull/112876)
* [expose `compiler-builtins-weak-intrinsics` feature for `-Zbuild-std`](https://github.com/rust-lang/rust/pull/112956)
* [fix return type notation associated type suggestion when -Zlower-impl-trait-in-trait-to-assoc-ty](https://github.com/rust-lang/rust/pull/112983)
* [fix return type notation errors with -Zlower-impl-trait-in-trait-to-assoc-ty](https://github.com/rust-lang/rust/pull/112981)
* [add cfg diagnostic for unresolved import error](https://github.com/rust-lang/rust/pull/112854)
* [inline before merging cgus](https://github.com/rust-lang/rust/pull/112695)
* [liberate bound vars properly when suggesting missing async-fn-in-trait](https://github.com/rust-lang/rust/pull/112868)
* [make `closure_saved_names_of_captured_variables` a query](https://github.com/rust-lang/rust/pull/112759)
* [make queries traceable again](https://github.com/rust-lang/rust/pull/112883)
* [merge `BorrowKind::Unique` into `BorrowKind::Mut`](https://github.com/rust-lang/rust/pull/112119)
* [sort the errors from arguments checking so that suggestions are handled properly](https://github.com/rust-lang/rust/pull/112762)
* [suggest correct signature on missing fn returning RPITIT/AFIT](https://github.com/rust-lang/rust/pull/112596)
* [support Apple tvOS in libstd](https://github.com/rust-lang/rust/pull/103503)
* [test the cargo args generated by bootstrap.py](https://github.com/rust-lang/rust/pull/112281)
* [use ErrorGuaranteed instead of booleans in `rustc_builtin_macros`](https://github.com/rust-lang/rust/pull/112802)
* [various impl trait in assoc tys cleanups](https://github.com/rust-lang/rust/pull/112891)
* [warn on unused `offset_of!()` result](https://github.com/rust-lang/rust/pull/111684)
* [fix: generalize types before generating built-in `Normalize` clauses](https://github.com/rust-lang/chalk/pull/797)
* [support `FnPtr` trait](https://github.com/rust-lang/chalk/pull/798)
* [miri: mmap/munmap/mremamp shims](https://github.com/rust-lang/miri/pull/2520)
* [`Default`: Always inline primitive data types](https://github.com/rust-lang/rust/pull/113024)
* [add `alloc::rc::UniqueRc`](https://github.com/rust-lang/rust/pull/111849)
* [make {`Arc`, `Rc`, `Weak`}`::ptr_eq` ignore pointer metadata](https://github.com/rust-lang/rust/pull/106450)
* [alter `Display` for `Ipv6Addr` for IPv4-compatible addresses](https://github.com/rust-lang/rust/pull/112606)
* [fix windows `Socket::connect_timeout` overflow](https://github.com/rust-lang/rust/pull/112464)
* [specialize `StepBy<Range<{integer}>>`](https://github.com/rust-lang/rust/pull/111850)
* [implement `PartialOrd` for `Vec`s over different allocators](https://github.com/rust-lang/rust/pull/112632)
* [implement `Sync` for `mpsc::Sender`](https://github.com/rust-lang/rust/pull/111087)
* [cargo: Support `cargo Cargo.toml`](https://github.com/rust-lang/cargo/pull/12281)
* [cargo: add `.toml` file extension restriction for `-Zconfig-include`](https://github.com/rust-lang/cargo/pull/12298)
* [cargo: allow embedded manifests in all commands](https://github.com/rust-lang/cargo/pull/12289)
* [rustdoc: partially fix invalid files creation](https://github.com/rust-lang/rust/pull/112836)
* [rustdoc: fix union fields display](https://github.com/rust-lang/rust/pull/112894)
* [rustdoc: handle assoc const equalities in cross-crate impl-Trait-in-arg-pos](https://github.com/rust-lang/rust/pull/113028)
* [rustdoc: render the body of associated types before the where-clause](https://github.com/rust-lang/rust/pull/112906)
* [rustfmt: handling of numbered markdown lists](https://github.com/rust-lang/rustfmt/pull/5423)
* [rustfmt: implement `let-else` formatting](https://github.com/rust-lang/rustfmt/pull/5690) (finally!)
* [clippy: `borrow_as_ptr`: Ignore temporaries](https://github.com/rust-lang/rust-clippy/pull/10948)
* [clippy: `format_push_string`: look through `match` and `if` expressions](https://github.com/rust-lang/rust-clippy/pull/11021)
* [clippy: `get_unwrap`: include a borrow in the suggestion if argument is not an integer literal](https://github.com/rust-lang/rust-clippy/pull/10979)
* [clippy: `items_after_test_module`: Ignore in-proc-macros items](https://github.com/rust-lang/rust-clippy/pull/10992)
* [clippy: `ptr_arg`: Don't lint when return type uses `Cow`'s lifetime](https://github.com/rust-lang/rust-clippy/pull/11019)
* [clippy: `single_match`: don't lint if block contains comments](https://github.com/rust-lang/rust-clippy/pull/10990)
* [clippy: `type_repetition_in_bounds`: respect MSRV for combining bounds](https://github.com/rust-lang/rust-clippy/pull/10994)
* [clippy: allow safety comment above attributes](https://github.com/rust-lang/rust-clippy/pull/10986)
* [clippy: avoid linting `extra_unused_type_parameters` on procedural macros](https://github.com/rust-lang/rust-clippy/pull/11022)
* [clippy: check if `if` conditions always evaluate to true in `never_loop`](https://github.com/rust-lang/rust-clippy/pull/11005)
* [clippy: don't lint `excessive_precision` on inf](https://github.com/rust-lang/rust-clippy/pull/10952)
* [clippy: don't lint `iter_nth_zero` in `next`](https://github.com/rust-lang/rust-clippy/pull/10993)
* [clippy: lint `mem_forget` if any fields are `Drop`](https://github.com/rust-lang/rust-clippy/pull/10996)
* [rust-analyzer: feature: assist delegate impl](https://github.com/rust-lang/rust-analyzer/pull/14948)
* [rust-analyzer: fix some unsizing problems in mir](https://github.com/rust-lang/rust-analyzer/pull/15135)
* [rust-analyzer: skip mutable diagnostics on synthetic bindings](https://github.com/rust-lang/rust-analyzer/pull/15104)
* [rust-analyzer: support manual impl of fn traits in mir interpreter](https://github.com/rust-lang/rust-analyzer/pull/15112)
* [rust-analyzer: support more intrinsics in mir interpreter](https://github.com/rust-lang/rust-analyzer/pull/15119)

### Rust Compiler Performance Triage

Relatively quiet week outside of a large win on one incremental benchmark in a
regression test (i.e., not real world code). Bimodality in a number of
benchmarks continues to be an issue.

Triage done by **@simulacrum**.
Revision range: [b9d608c9..b5e51db](https://perf.rust-lang.org/?start=b9d608c979ad3c0700f9f0237a8c12feb0ba44fb&end=b5e51db16dfbf5685e32dfe2d9a835a5c695afe4&absolute=false&stat=instructions%3Au)

5 Regressions, 6 Improvements, 3 Mixed; 5 of them in rollups

44 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-06-27.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: close] [Tracking issue for std::default::default()](https://github.com/rust-lang/rust/issues/73014)
* [disposition: merge] [Create `unnecessary_send_constraint` lint for `&(dyn ... + Send)`](https://github.com/rust-lang/rust/pull/110961)
* [disposition: merge] [Change default panic handler message format.](https://github.com/rust-lang/rust/pull/112849)
* [disposition: close] [MSVC and rustc disagree on minimum stack alignment on x86 Windows](https://github.com/rust-lang/rust/issues/112480)
* [disposition: merge] [style-guide: Add chapter about formatting for nightly-only syntax](https://github.com/rust-lang/rust/pull/111119)
* [disposition: merge] [rustdoc: Allow whitespace as path separator like double colon](https://github.com/rust-lang/rust/pull/108537)
* [disposition: merge] [Add `internal_features` lint](https://github.com/rust-lang/rust/pull/108955)
* [disposition: merge] [Don't require associated types with Self: Sized bounds in `dyn Trait` objects](https://github.com/rust-lang/rust/pull/112319)
* [disposition: merge] [Return Ok on kill if process has already exited](https://github.com/rust-lang/rust/pull/112594)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* [eRFC: single-file packages ("cargo script") integration](https://github.com/rust-lang/rfcs/pull/3424)
  * [Testing steps](https://github.com/rust-lang/rfcs/pull/3424#issuecomment-1609687109)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-06-28 - 2023-07-26 🦀

### Virtual

* 2023-06-28 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Building Our Own 'Arc' in Rust (Atomics & Locks Chapter 6)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/294184120/)
* 2023-06-28 | Virtual (Chicago, IL, US) | [Chicago Healthcare Cloud Technology Community](https://www.meetup.com/chicago-healthcare-tech-and-ai/)
    * [**Rust for Mission-Critical AI: A Journey into Healthcare's Safest Language**](https://www.meetup.com/chicago-healthcare-tech-and-ai/events/293278396)
* 2023-06-29 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/294294905)
* 2023-07-01 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 4: Protohackers Exercises Mob Coding (Problem II onwards)**](https://www.meetup.com/rust-noris/events/293800373)
* 2023-07-04 | Virtual (Berlin, DE) | [Berline.rs / OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/zdrzpsyfckbgb/)
* 2023-07-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfckbgb/)
* 2023-07-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/293309295)
* 2023-07-05 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfckbhb)
* 2023-07-06 | Virtual (Ciudad de México, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Rust y Haskell**](https://www.meetup.com/rust-mx/events/294152158)
* 2023-07-11 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsyfckbpb)
* 2023-07-13 - 2023-07-14 | Virtual | [Scientific Computing in Rust](https://scientificcomputing.rs/)
    * [**Scientific Computing in Rust workshop**](https://scientificcomputing.rs/)
* 2023-07-13 | Virtual (Edinburgh, UK) | [Rust Edinburgh](https://www.meetup.com/rust-edi/)
    * [**Reasoning about Rust: an introduction to Rustdoc’s JSON format**](https://www.meetup.com/rust-edi/events/293820336/)
* 2023-07-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763486)
* 2023-07-20 | Virtual (Tehran, IR) | [Iran Rust Meetup](https://rust-meetup.ir/)
    * [**Iran Rust Meetup #12 - Ownership and Memory management**](https://rust-meetup.ir/2023/07/20/12th-meetup.html)

### Asia

* 2023-06-29 | Seoul, KR | [T-RUST meetup](https://www.meetup.com/t-rust-meetup/)
    * [**🦀 T-RUST Meetup 🦀**](https://www.meetup.com/t-rust-meetup/events/294280140/)

### Europe

* 2023-06-28 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake**](https://www.meetup.com/bratislava-rust-meetup-group/events/293732916)
* 2023-06-29 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup**](https://www.meetup.com/rust-meetup-augsburg/events/293566071/)
* 2023-06-29 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #37 at Samsung!**](https://www.meetup.com/copenhagen-rust-community/events/294024476)
* 2023-06-29 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna)
    * [**Rust Vienna Meetup - June - final meetup before a summer break**](https://www.meetup.com/rust-vienna/events/294225540/)
* 2023-07-01 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**(Beginner) Rust Workshop**](https://www.meetup.com/rust-basel/events/293906330/)
* 2023-07-03 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**Rust in the Linux Kernel - July Meetup**](https://www.meetup.com/rust-zurich/events/293322905)
* 2023-07-05 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #5**](https://www.meetup.com/fr-FR/rust-lyon/events/294325808)
* 2023-07-11 | Breda, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Rust: Advanced Graphics and User Interfaces**](https://www.meetup.com/rust-nederland/events/294199533/)
* 2023-07-13 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns***](https://www.meetup.com/reading-rust-workshop/events/mstlftyfckbrb/)

### North America

* 2023-06-21 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/BostonRust/)
    * [**Ball Square Rust Lunch, June 21**](https://www.meetup.com/BostonRust/events/293725119)
* 2023-06-22 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Learn How to Use cargo-semver-checks and Closure Traits to Write Better Code**](https://www.meetup.com/rust-nyc/events/294123104)
* 2023-06-24 | San Jose, CA, US | [Rust Breakfast & Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/jnmgftyfcjbgc/)
* 2023-06-28 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/BostonRust/)
    * [**Harvard Square Rust Lunch**](https://www.meetup.com/BostonRust/events/293725559)
* 2023-06-29 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/294223607)
* 2023-07-01 | San Jose, CA, US | [Rust Breakfast & Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/jnmgftyfckbcb/)
* 2023-07-07 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Lunch**](https://www.meetup.com/deep-dish-rust/events/293794930/)
* 2023-07-12 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/294373345)
* 2023-07-12 | Waterloo, ON, CA | [Rust KW](https://www.meetup.com/rust-kw/)
    * [**Overengineering FizzBuzz**](https://www.meetup.com/rust-kw/events/294355516/)
* 2023-07-13 | Seattle, WA, US | [Seattle Rust User Group Meetup](https://www.meetup.com/seattle-rust-user-group/)
    * [**July Meetup**](https://www.meetup.com/seattle-rust-user-group/events/294191599/)
* 2023-07-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfckbxb)

### Oceania

* 2023-07-11 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/294447461/)
* 2023-07-11 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Hybrid - in person & online) July 2023 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/294274774/)

### South America

* 2023-07-04 | Medellín, CO | [Rust Medellín](https://www.meetup.com/rust-medellin/)
    * [**Introduccion a rust, ownership and safety code**](https://www.meetup.com/rust-medellin/events/294438119/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs
<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> It's a compiler not a Jedi, don't expect it to read minds.

– [Nishant on github](https://github.com/juspay/hyperswitch/wiki/Ask-not-what-the-compiler-can-do-for-you#whos-to-blame-here)

Thanks to [Nishant](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1440) for the self-suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
