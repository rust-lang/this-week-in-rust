Title: This Week in Rust 150
Number: 150
Date: 2016-10-04
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

* 🎈🎉 [Announcing Rust 1.12](https://blog.rust-lang.org/2016/09/29/Rust-1.12.html). 🎉🎈
* [Ethereum users are recommended to switch to Parity (an Ethereum client written in Rust) to mitigate ongoing DoS attack](https://blog.ethereum.org/2016/09/22/ethereum-network-currently-undergoing-dos-attack/). Further reads - [out of memory bug in Geth client](https://blog.ethereum.org/2016/09/18/security-alert-geth-nodes-crash-due-memory-bug/) and [next steps against transaction spam attack](https://blog.ethereum.org/2016/09/22/transaction-spam-attack-next-steps/).
* [Optional arguments in Rust 1.12](http://xion.io/post/code/rust-optional-args.html).
* [Applying Hoare logic to the Rust MIR](https://ticki.github.io/blog/a-hoare-logic-for-rust/). Hoare logic is a formal system with a set of logical rules for reasoning rigorously about the correctness of computer programs (from [Wikipedia article](https://en.wikipedia.org/wiki/Hoare_logic)).
* [Rust as a language for high performance GC implementation](http://ts.data61.csiro.au/publications/nictaabstracts/Lin_BHN_16.abstract.pml).
* [How to use Rust code inside Haskell](https://mgattozzi.github.io/2016/10/01/haskell-rust.html).
* [Rusty dynamic loading](https://damienradtke.com/post/rusty-dynamic-loading/). How to utilize dynamic libraries to reload code on the fly.
* [Safe and efficient bidirectional trees](https://www.reddit.com/r/rust/comments/55ns2m/safe_and_efficient_bidirectional_trees/).
* [How to implement a new DOM API for Servo](http://jeenalee.com/2016/10/03/implementing-doge-for-servo.html).
* [Observational equivalence and unsafe code](http://smallcultfollowing.com/babysteps/blog/2016/10/02/observational-equivalence-and-unsafe-code/). Observational equivalence is the property of two or more underlying entities being indistinguishable on the basis of their observable implications (from [Wikipedia article](https://en.wikipedia.org/wiki/Observational_equivalence)).
* [Distinguishing reuse from override](http://smallcultfollowing.com/babysteps/blog/2016/09/29/distinguishing-reuse-from-override/). Follow-up to last week's [intersection impls](http://smallcultfollowing.com/babysteps/blog/2016/09/24/intersection-impls/) article.
* [Even quicker byte count](https://llogiq.github.io/2016/09/27/count.html). Follow-up to last week's [how to count newlines really fast in Rust](https://llogiq.github.io/2016/09/24/newline.html).
* [Implementing Finite Automata in Rust (Part 1)](https://apanatshka.github.io/compsci/2016/10/03/implementing-finite-automata-part-1/).
* [Building personalized IPC debugging tools using Electron and Rust](https://wraithan.net/2016/10/02/i-can-manage-it-weekly-update-3/).
* [Easier Rust Development on the PJRC Teensy 3](http://jamesmunns.com/update/2016/09/26/teensy3-rs.html). PJRC Teensy is a USB-based microcontroller development system.
* [Why you should be blogging about Rust](https://mgattozzi.github.io/2016/09/27/blog-about-rust.html).

## New Crates & Project Updates

* [into_rust()](http://intorust.com/): screencasts for learning Rust.
* [Rust compilation times compared to C++, D, Go, Pascal](https://www.reddit.com/r/rust/comments/55k577/rust_compilation_times_compared_to_c_d_go_pascal/).
* [Serde is transitioning to Macros 1.1 (with much faster compile times)](https://users.rust-lang.org/t/serde-transitioning-to-macros-1-1/7437).
* [itertools 0.5.0 released](https://bluss.github.io/rust/2016/09/26/itertools-0.5.0/). New features, shorter names, and significant improvements in consistency and clarity.
* [Wayland client 0.7 released](https://github.com/vberger/wayland-client-rs/tree/v0.7.0) after a [(3rd) rewrite](http://blog.levans.fr/wayland_rust_v3-en.html).
* [SUPER 0.1.0 released](http://superanalyzer.rocks/2016/10/02/super-is-here). SUPER, the secure, unified, powerful, and extensible Rust Android Analyzer has released its first version for Windows, Linux and MacOS X.
* [defrag](https://github.com/vitiral/defrag-rs): safe and efficient memory manager for microcontrollers.

### Other weeklies from Rust community

* [podcast] [New Rustacean interview 3](http://www.newrustacean.com/show_notes/interview/_3/). Carol (Nichols || Goulding) on learning Rust, teaching Rust, and building community.
* [This week in Servo 79](https://blog.servo.org/2016/09/26/twis-79/). Servo is a prototype web browser engine written in Rust.
* [This week in Rust docs 24](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-24). Updates from the Rust documentation team.
* [This week in Ruma 2016-10-02](https://www.ruma.io/news/this-week-in-ruma-2016-10-02/). Ruma is a Matrix homeserver written in Rust.
* [This week in TiKV 2016-09-30](http://www.pingcap.com/tikv/2016/09/30/tikv-weekly/). TiKV is a distributed Key-Value database.

# Crate of the Week

*No crate was selected for CotW.*

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [servo: Enable flexbox layout by default](https://github.com/servo/servo/issues/13576).
* [easy] [rust: librustc_llvm: call llvm-config with correct linking mode](https://github.com/rust-lang/rust/issues/36854).
* [hard] [rust: Optimize emscripten targets with emcc](https://github.com/rust-lang/rust/issues/36899).
* [hard] [rust: Tell emscripten to remove exception handling code when the panic runtime is used](https://github.com/rust-lang/rust/issues/36900).
* [moderate] [rust: Create official .deb packages](https://github.com/rust-lang/rust/issues/28307).
* [easy] [imag: We use too much .fold(), there is libimagutil::iter::FoldResult!](https://github.com/matthiasbeyer/imag/issues/777)
* [easy] [imag: Rewrite: libimagdiary: FromStoreId -> Option<_> to use Result<_>](https://github.com/matthiasbeyer/imag/issues/778)
* [moderate] [super: Coloring errors in the console](https://github.com/SUPERAndroidAnalyzer/super/issues/41).
* [easy] [super: Line highlighting in code view](https://github.com/SUPERAndroidAnalyzer/super/issues/36).
* [moderate] [super: Certificate analysis](https://github.com/SUPERAndroidAnalyzer/super/issues/28).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

181(!) pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-09-26..2016-10-03

* [Working asmjs and wasm targets](https://github.com/rust-lang/rust/pull/36339).
* [Add changelog for 1.12](https://github.com/rust-lang/rust/pull/36609).
* [std: Stabilize and deprecate APIs for 1.13](https://github.com/rust-lang/rust/pull/36815).
* [Haiku: Initial work at OS support](https://github.com/rust-lang/rust/pull/36727).
* [rustc: implement -C link-arg](https://github.com/rust-lang/rust/pull/36574).
* [libtest: add a --skip flag to the test runner](https://github.com/rust-lang/rust/pull/36604).
* [Forbid user-defined macros named "macro_rules"](https://github.com/rust-lang/rust/pull/36730).
* [Don't allocate during default HashSet creation](https://github.com/rust-lang/rust/pull/36734).
* [[std::io::Chain] Mark first as done only when reading into non-zero length buffer](https://github.com/rust-lang/rust/pull/36777).
* [Allow attributes on lifetime/type formal parameters](https://github.com/rust-lang/rust/pull/34764). First step for `#[may_dangle]`.
* [remove `ExactSizeIterator` from `RangeInclusive<{u,i}{32,size}>`](https://github.com/rust-lang/rust/pull/36395). Breaking-change for some nightly users.
* [Reject macros with empty repetitions](https://github.com/rust-lang/rust/pull/36721).
* [Add a panic-strategy field to the target specification](https://github.com/rust-lang/rust/pull/36794).
* [Restrict where in the tree platform-specific cfgs may be mentioned](https://github.com/rust-lang/rust/pull/36807).
* [Resolve the callee type in check_call before autoderef](https://github.com/rust-lang/rust/pull/36822).
* [book: New chapter: Fundamental Collections](https://github.com/rust-lang/book/pull/137).
* [crates.io: Show all crates owned by a user or group](https://github.com/rust-lang/crates.io/pull/416).

## New Contributors

* Chris McDonald
* Frank Rehberger
* Jesus Garlea
* Martin Thoresen
* Nathan Musoke
* ParkHanbum
* Paul Lange
* Paulo Matos
* Peter N
* Philip Davis
* Pweaver (Paul Weaver)
* Ross Schulman

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

* [Propose a shorthand syntax for constructing struct-like values with _named_ fields](https://github.com/rust-lang/rfcs/pull/1682).
* [Let a `loop { ... }` expression return a value via `break my_value;`](https://github.com/rust-lang/rfcs/pull/1624).

## New RFCs

* [Use abort as the standard panic method rather than unwind](https://github.com/rust-lang/rfcs/pull/1759).

# Upcoming Events

* [10/5. Open-Space Rust Meetup Cologne/Bonn](http://rustaceans.cologne/2016/10/05/open-space.html).
* 10/5. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* 10/5. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org.
* [10/5. PSU Capstone Rust project summaries Portland](https://www.meetup.com/PDXRust/events/234601233/).
* [10/6. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [10/10. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/233577254/).
* [10/11. Inaugural Tampa Rust Meetup / Install Fest](https://www.meetup.com/Rust-Tampa/events/234485292/).
* 10/12. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* 10/12. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org.
* [10/12. Rust Boulder/Denver](https://www.meetup.com/Rust-Boulder-Denver/events/233784848/).
* [10/13. Columbus Rust Society](https://www.meetup.com/columbus-rs/events/233996456/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# fn work(on: RustProject) -> Money

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Friends of the Forest

Our community likes to recognize people who have made outstanding contributions
to the Rust Project, its ecosystem, and its community. These people are
'friends of the forest'.

This week's friends of the forest are:

* [dtolnay] and [oli-obk] for taking over most of the maintenance of the [serde] stack
* [seanmonstar] for answering my too many questions
* [phildawes] for racer
* [mitsuhiko] for [redis]
* [serde] team ([dtolnay], [oli-obk], [erickt])
* [athemathmo] for [rusty_machine]
* [carllerche] for [mio]/[tokio]
* [killercup] for keeping [diesel] running
* [dikaiosune] - [rusty-dash]
* [nasa42] and [llogiq] - [This Week In Rust]
* [WindowsBunny] - being the fuzziest bunny +1 +1 (ed: the +1's are from multiple people)
* [eddyb] - for knowing everything about rust
* [chriskrycho] for [New Rustacean]
* [steveklabnik], Rust documentation superhero
* [carllerche], [eternaleye], [staticassert]
* [Matthias Beyer]
* [llogiq] and [manishearth]
* [illegalprime] for his work on [rust-websocket]
* [Mark-Simulacrum] for awesome work on the [compiler performance website]
* [sfackler] and [briansmith] for enhancing the crypto/security story for Rust.
  Their efforts have made running Rust in production code much more feasible.
  sfackler: [rust-openssl], [rust-security-framework], [schannel-rs],
  [rust-native-tls], briansmith: [ring], [webpki]
* from [llogiq][llogiq-nominated]:

> I'd like to nominate [Veedrac] for his awesome contributions to various
> performance-related endeavors.

* from [codingcampbell][codingcampbell-nominated]:

> I'd like to highlight [tomaka] for his numerous projects ([glium], [vulkano],
> [glutin]). I know he's also involved in some other crates I take for granted,
> like [gl_generator].
>
> I like to play with gamedev, but I am a newcomer to OpenGL things and I have
> been very grateful for projects like [glium] and [gl_generator] that not only
> give me a good starting point, but through various documentation has informed
> me of OpenGL pitfalls.
>
> He recently wrote a [post-mortem][glium-postmortem] for [glium], which I think
> is good as a matter of reflection, but I'm still very impressed with that
> project, and the others he is tirelessly contributing to.
>
> Well done!

[RustConf]: http://rustconf.com/
[dtolnay]: https://github.com/dtolnay
[oli-obk]: https://github.com/oli-obk
[seanmonstar]: https://github.com/seanmonstar
[phildawes]: https://github.com/phildawes
[mitsuhiko]: https://github.com/mitsuhiko
[redis]: https://github.com/mitsuhiko/redis-rs
[erickt]: https://github.com/erickt
[serde]: https://github.com/serde-rs
[athemathmo]: https://github.com/AtheMathmo
[rusty_machine]: https://github.com/AtheMathmo/rusty-machine
[carllerche]: https://github.com/carllerche
[mio]: https://github.com/carllerche/mio
[tokio]: https://github.com/tokio-rs
[killercup]: https://github.com/killercup
[diesel]: http://diesel.rs/
[dikaiosune]: https://github.com/dikaiosune
[rusty-dash]: http://rusty-dash.com/
[nasa42]: https://github.com/nasa42
[llogiq]: https://github.com/llogiq
[This Week In Rust]: https://this-week-in-rust.org/
[WindowsBunny]: https://github.com/retep998
[eddyb]: https://github.com/eddyb
[chriskrycho]: https://github.com/chriskrycho
[New Rustacean]: http://www.newrustacean.com/
[steveklabnik]: https://github.com/steveklabnik
[eternaleye]: https://github.com/eternaleye
[staticassert]: https://github.com/insanitybit
[Matthias Beyer]: http://beyermatthias.de
[llogiq]: https://github.com/llogiq
[manishearth]: https://github.com/manishearth
[illegalprime]: https://github.com/illegalprime
[rust-websocket]: https://github.com/cyderize/rust-websocket
[Mark-Simulacrum]: https://github.com/Mark-Simulacrum
[compiler performance website]: http://perf.rust-lang.org/
[sfackler]: https://github.com/sfackler
[briansmith]: https://github.com/briansmith
[rust-openssl]: https://github.com/sfackler/rust-openssl
[rust-security-framework]: https://github.com/sfackler/rust-security-framework
[schannel-rs]: https://github.com/steffengy/schannel-rs
[rust-native-tls]: https://github.com/sfackler/rust-native-tls
[ring]: https://github.com/briansmith/ring
[webpki]: https://github.com/briansmith/webpki
[llogiq-nominated]: https://github.com/rust-community/team/issues/77#issuecomment-250074590
[Veedrac]: https://github.com/Veedrac
[codingcampbell-nominated]: https://users.rust-lang.org/t/twir-friends-of-the-forest/7295/2
[tomaka]: https://github.com/tomaka
[glium]: https://github.com/tomaka/glium
[vulkano]: https://github.com/tomaka/vulkano
[glutin]: https://github.com/tomaka/glutin
[gl_generator]: https://github.com/brendanzab/gl-rs
[glium-postmortem]: https://users.rust-lang.org/t/glium-post-mortem/7063

[Submit your Friends-of-the-Forest nominations for next week][foft]!

[foft]: https://users.rust-lang.org/t/twir-friends-of-the-forest/7295

# Quote of the Week

> My favorite new double-meaning programming phrase: "my c++ is a little rusty"

— [Jake Taylor on Twitter](https://twitter.com/ferristweetsnow/status/780392109874569220).

Thanks to [Zachary Dremann](https://twitter.com/Dr_Emann/status/780406723341275137) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
