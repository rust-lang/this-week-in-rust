Title: This Week in Rust 196
Number: 196
Date: 2017-08-22
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

* [brson - a Rust core team member and a key figure in Rust community is leaving](https://internals.rust-lang.org/t/farewell-rust/5776).
* [crates.io now renders README files of crates](https://github.com/rust-lang/crates.io/pull/869).
* [Inside a super fast CSS engine: Quantum CSS (aka Stylo)](https://hacks.mozilla.org/2017/08/inside-a-super-fast-css-engine-quantum-css-aka-stylo/).
* [rustgo: calling Rust from Go with near-zero overhead](https://blog.filippo.io/rustgo/).
* [Gtk-rs: The huge and long awaited release is finally here](http://gtk-rs.org/blog/2017/08/20/new-release.html).
* [RLSL - a new shading language](https://maikklein.github.io/post/shading-language-part1/).
* [nalgebra - a linear algebra library released v0.13 with pure-rust SVD, LU, Lapack integration, and matrix resizing](https://www.reddit.com/r/rust/comments/6tw57q/nalgebra_v013_released_with_purerust_svd_lu_etc/).
* [Making Redox self-hosting - status report 4](https://redox-os.org/news/gsoc-self-hosting-4/).
* [Building a mobile app in Rust and React Native, part 2](https://medium.com/@marekkotewicz/building-a-mobile-app-in-rust-and-react-native-part-2-hello-world-60970a7d194a).
* [Make docs into code](https://llogiq.github.io/2017/08/20/docs.html).
* [Announcement - try out Rust IDE support in Visual Studio Code](https://users.rust-lang.org/t/try-out-rust-ide-support-in-visual-studio-code/12407).
* [Distributing Rust GTK+ Apps](http://blog.ctaggart.com/2017/08/distributing-rust-gtk-apps.html).
* [Zone of Control is dead. Long life Zemeroth.](https://ozkriff.github.io/2017-08-17--devlog.html). A new turn-based strategy game in Rust.
* [podcast] [Request for explanation #8 - An existential crisis](https://request-for-explanation.github.io/podcast/ep8-an-existential-crisis/index.html). This week's focus is [RFC 2071](https://github.com/rust-lang/rfcs/pull/2071) "Add impl Trait type alias and variable declarations".
* [This week in Rust docs 69](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-69).

# Crate of the Week

This week's crate is [pest](https://crates.io/crates/pest), a PEG-based parsing library. Thanks to [Laurent Wandrebeck](https://users.rust-lang.org/u/lwandrebeck) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

This week's Call for Participation is submitted by RustConf 2017 attendees!

<img src="https://this-week-in-rust.org/images/rustconf-2017-cfp.jpg" alt="A photo of a whiteboard from RustConf 2017 which lists projects looking for help.">

### Transcript

* [Servo](https://starters.servo.org/)
* [Rusoto](https://github.com/rusoto/rusoto)
* [Rust](https://www.rust-lang.org/en-US/contribute-bugs.html)
* [community.rs](https://community.rs/)
* [Alacritty](https://github.com/jwilm/alacritty)
* [cell-gc](https://github.com/jorendorff/cell-gc)
* [bindgen](https://github.com/rust-lang-nursery/rust-bindgen)
* [Robigalia](https://robigalia.org/)
* [Clippy](https://github.com/rust-lang-nursery/rust-clippy)
* [Habitat](https://github.com/habitat-sh)
* [startisd](https://github.com/stratis-storage/stratisd)
* [gfx-rs](https://github.com/gfx-rs/gfx)
* [trust-dns](https://github.com/bluejekyll/trust-dns)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Who's been awesome for the Rust community - RustConf 2017 edition

Our community likes to recognize people who have made outstanding contributions
to the Rust Project, its ecosystem, and its community. This year at RustConf, attendees
nominated people who have been awesome for the Rust community. And here they are:

<img src="https://this-week-in-rust.org/images/rustconf-2017-fotf.jpg" alt="A photo of a whiteboard from RustConf 2017 which displays a list titled - who's been awesome for the Rust community - contributed by attendees.">

### Transcript

* [Paul Faria](https://github.com/Nashenas88)
* [brson](https://github.com/brson)
* [retep998](https://github.com/retep998)
* [John Van Enk](https://github.com/sw17ch)
* [cramertj](https://github.com/cramertj)
* [withoutboats](https://github.com/withoutboats)
* [Manishearth](https://github.com/Manishearth)
* [jdm](https://github.com/jdm)
* [Mark Simulacrum](https://github.com/Mark-Simulacrum)
* [petrochenkov](https://github.com/petrochenkov)
* [Amit Levy](https://github.com/alevy)
* [Leah and the RustConf team](http://rustconf.com/about.html)
* [Ashley Williams](https://github.com/ashleygwilliams)
* [dtolnay](https://github.com/dtolnay)
* [KodrAus](https://github.com/KodrAus)
* [Steve Klabnik](https://github.com/steveklabnik)
* [tomaka](https://github.com/tomaka)
* [bors](https://github.com/bors)
* [budziq](https://github.com/budziq)

# Updates from Rust Core

99 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-08-14..2017-08-21

* [forbid non-standard literal patterns](https://github.com/rust-lang/rust/pull/43844)
* [cleanup for LLVM-less build, second attempt](https://github.com/rust-lang/rust/pull/43842)
* [stabilize rvalue promotion to `'static`](https://github.com/rust-lang/rust/pull/43838)
* [implement `CompilerDesugaringKind`](https://github.com/rust-lang/rust/pull/43832) (was stringly typed before)
* [fix span miscalculation in `save-analysis`](https://github.com/rust-lang/rust/pull/43826)
* [fix ICE with elided lifetimes in foreign function return types](https://github.com/rust-lang/rust/pull/43651)
* [`RefCell::`{`swap`, `replace`}](https://github.com/rust-lang/rust/pull/43574)
* [`String::retain`](https://github.com/rust-lang/rust/pull/43500)
* [`Vec::drain_filter`](https://github.com/rust-lang/rust/pull/43245)
* [MIR borrowck](https://github.com/rust-lang/rust/pull/43108)
* [rerun MIR passes on promoted temporaries](https://github.com/rust-lang/rust/pull/43902)
* [everybody loops🎶 but `impl Trait`](https://github.com/rust-lang/rust/pull/43878)
* [redox now has unwinding panics](https://github.com/rust-lang/rust/pull/43917)
* [ship the rustdoc book](https://github.com/rust-lang/rust/pull/43863)
* [crates.io now shows the README.md on crate pages](https://github.com/rust-lang/crates.io/pull/869)

## New Contributors

* adrian5
* Anthony Clays
* Anthony Defranceschi
* Fourchaux
* Hunter Praska
* Martin Hoffmann
* Seiichi Uchida
* Shanavas M

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Amend RFC 1242 to require an RFC for deprecation of crates from the nursery](https://github.com/rust-lang/rfcs/pull/1983).
* [RFC 1966: Unsafe pointer methods](https://github.com/rust-lang/rfcs/pull/1966).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Future-proofing enums/structs with `#[non_exhaustive]` attribute](https://github.com/rust-lang/rfcs/pull/2008).
* [disposition: merge] [Add external doc attribute to rustc](https://github.com/rust-lang/rfcs/pull/1990).
* [disposition: merge] [Allow an optional vert at the beginning of a match branch](https://github.com/rust-lang/rfcs/pull/1925).
* [disposition: close] [Allow use of pipe operator in patterns](https://github.com/rust-lang/rfcs/pull/1882).
* [disposition: merge] [Generic associated types (associated type constructors)](https://github.com/rust-lang/rfcs/pull/1598).
* [disposition: merge] [Enable nested method calls](https://github.com/rust-lang/rfcs/pull/2025).
* [disposition: merge] [Evolving Rust through checkpoints](https://github.com/rust-lang/rfcs/pull/2052).

## New RFCs

* [Argument-bound lifetimes](https://github.com/rust-lang/rfcs/pull/2115).
* [Fully-qualified names in RustDocs](https://github.com/rust-lang/rfcs/pull/2114).
* [Fallible collection allocation 1.0](https://github.com/rust-lang/rfcs/pull/2116).
* [Debuggable macro expansions](https://github.com/rust-lang/rfcs/pull/2117).
* [Crypto / rand trait & crate split](https://github.com/rust-lang/rfcs/pull/2118).
* [Add the `()` → `Result<(), _>` coercion rule, for removing `Ok(())` everywhere](https://github.com/rust-lang/rfcs/pull/2120).
* [`dyn Trait` syntax for trait objects: Take 2](https://github.com/rust-lang/rfcs/pull/2113).
* [Autoreferencing `Copy` types](https://github.com/rust-lang/rfcs/pull/2111).

# Upcoming Events

* [Aug 24. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Aug 30. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Aug 30. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Aug 31. Rust NYC - Come learn about Rust](https://www.meetup.com/Rust-NYC/events/241866546/).
* [Aug 31. Rust London - Rust learning and hacking evening #2](https://www.meetup.com/Rust-London-User-Group/events/242378000/).
* [Aug 31. Cambridge Rust Meetup - Rust Study Group](https://www.meetup.com/Cambridge-Rust-Meetup/events/242409356/).
* [Aug 31. Rust Roma Italy - Rust learning and hacking evening #1](https://www.meetup.com/Rust-Roma/events/242709171/).
* [Sep  4. Rust Zurich - September Community Meetup](https://www.meetup.com/de-DE/Rust-Zurich/events/242032911/).
* [Sep  6. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/krnczlywmbjb/).
* [Sep  6. Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/241813161/).
* [Sep  6. Rust Cologne - RFC Lightning Talks](https://www.meetup.com/RustCologne/events/242597353/).
* [Sep  6. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Sep  6. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Sep  7. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Sr. Software Development Engineer at Amazon](https://www.amazon.jobs/en/jobs/559813/sr-software-development-engineer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust, but verify.

— [@isislovecruft talking about elliptic curve cryptography in Rust at RustConf 2017](https://twitter.com/jcdyer/status/899058522930921472).

Thanks to [llogiq](https://twitter.com/llogiq/status/899399721609035777) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
