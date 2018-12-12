Title: This Week in Rust 264
Number: 264
Date: 2018-12-11
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

* 🎈🎉 [Announcing Rust 1.31 and Rust 2018](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html). 🎉🎈
* [Rust 2018 is here… but what is it](https://hacks.mozilla.org/2018/12/rust-2018-is-here/)?
* [Reflecting on Rust and WebAssembly in 2018](https://rustwasm.github.io/2018/12/06/reflecting-on-rust-and-wasm-in-2018.html).
* [Rocket v0.4: Typed URIs, database support, revamped queries, & more](https://rocket.rs/v0.4/news/2018-12-08-version-0.4/).
* [Inside Rust's async transform](https://blag.nemo157.com/2018/12/09/inside-rusts-async-transform.html).
* [The Swiss army knife of hashmaps](https://blog.waffles.space/2018/12/07/deep-dive-into-hashbrown/).
* [Building Alexa Skills in Rust](https://medium.com/@amalec/building-alexa-skills-in-rust-4cf54a497ea4).
* [Wherefore art thou Romio](https://boats.gitlab.io/blog/post/romio/)? Romio is a port of a small part of the Tokio project to the newer futures APIs.
* [Existential types in Rust](https://adelbertc.github.io/posts/2018-12-10-rust-existentials.html).
* [Currying in Rust](https://hashnode.com/post/currying-in-rust-cjpfb0i2z00cm56s2aideuo4z).
* [More on RLS version numbering](https://www.ncameron.org/blog/more-on-rls-version-numbering/).
* [Patterns of Refactoring C to Rust: The case of librsvg](https://people.gnome.org/~federico/blog/guadec-2018-presentation.html).

### #Rust2019

* [Rust in 2022 (and 2019)](https://www.ncameron.org/blog/rust-in-2022/).
* [Rust 2019: Think bigger](http://fitzgeraldnick.com/2018/12/11/rust-2019-think-bigger.html).
* [The Fallow Year](https://www.jonathanturner.org/2018/12/the-fallow-year.html).
* [eddyb's wishes for Rust in 2019](https://twitter.com/eddyb_r/status/1072444398284300289).
* [pcwalton's plans for 2019](https://pcwalton.github.io/2018/12/07/plans-for-2019.html).
* [Lokathor's Rust 2019 wishpost](https://www.reddit.com/r/rust/comments/a4ygji/lokathors_rust_2019_wishpost/).
* [Rust 2019 - The road ahead](https://llogiq.github.io/2018/12/08/rust.html).
* [2019 wishlist - Finish and ship the work that has already started](https://www.reddit.com/r/rust/comments/a3sav1/2019_roadmap_more_like_a_wishlist_finish_and_ship/).
* [Rust 2019: a newbie's thoughts](https://www.reddit.com/r/rust/comments/a44txf/rust_2019_a_newbies_thoughts/).
* [Rust 2019 — let us pursue composability](https://medium.com/@GolDDranks/rust-2019-let-us-pursue-composability-70f1eb2238c3).
* [2019 Wishlist/Pain points](https://www.reddit.com/r/rust/comments/a59b3a/2019_wishlistpain_points/).
* [Rust 2019 - Named argument syntax](https://twitter.com/PistonDeveloper/status/1072193819855196160).

# Crate of the Week

This week's crate is [cargo-call-stack](https://github.com/japaric/cargo-call-stack), a cargo subcommand for whole-program call stack analysis. Thanks to [Jorge Aparicio](https://mobile.twitter.com/japaricious/status/1069569802241486850) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [A call for Rust 2019 Roadmap blog posts](https://blog.rust-lang.org/2018/12/06/call-for-rust-2019-roadmap-blogposts.html).
* [Rust Latam CFP is now open, deadline is December 31st](https://cfp.rustlatam.org/events/rust-latam).
* [Tarpaulin: OSX support tracking issue](https://github.com/xd009642/tarpaulin/issues/152). Tarpaulin is a code coverage tool for Rust projects.
* [The imag project calls for contributors (2)](https://imag-pim.org/blog/2018/12/04/call-for-participation-2/).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

254 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-11-26..2018-12-03

* [decouple proc_macro from the rest of the compiler](https://github.com/rust-lang/rust/pull/49219)
* [implement chalk unification routines](https://github.com/rust-lang/rust/pull/56214)
* [upgrade LLVM to trunk, still version 8](https://github.com/rust-lang/rust/pull/55835)
* [another LLVM Update](https://github.com/rust-lang/rust/pull/56313) and [Re-enable lldb](https://github.com/rust-lang/rust/pull/56298)
* [use sort_by_cached_key when the key function is not trivial/free](https://github.com/rust-lang/rust/pull/55821)
* [deduplicate literal → constant lowering](https://github.com/rust-lang/rust/pull/56312)
* [use `MaybeUninit` instead of `mem::uninitialized` for Windows Mutex](https://github.com/rust-lang/rust/pull/56275)
* [libcore: add VaList and variadic arg handling intrinsics](https://github.com/rust-lang/rust/pull/49878)
* [arena: speed up TypedArena::clear and improve common patterns](https://github.com/rust-lang/rust/pull/56378)
* [stabilize `macro_at_most_once_rep`](https://github.com/rust-lang/rust/pull/56245)
* [stabilize `dbg!(..)`](https://github.com/rust-lang/rust/pull/56395)
* [stabilize `self_in_typedefs`](https://github.com/rust-lang/rust/pull/56366)
* [stabilize `self_struct_ctor`](https://github.com/rust-lang/rust/pull/56365)
* [remove unsafe `unsafe` inner function](https://github.com/rust-lang/rust/pull/56236)
* [add `TryFrom<&[T]> for [T; $N] where T: Copy`](https://github.com/rust-lang/rust/pull/56216)
* [move VecDeque::resize_with out of the impl<T:Clone> block](https://github.com/rust-lang/rust/pull/56401)
* [use allow-dirty option in `cargo package` to skip vcs checks](https://github.com/rust-lang/cargo/pull/6280)
* [make `ParseIntError` and `IntErrorKind` fully public](https://github.com/rust-lang/rust/pull/55705)
* [use MaybeUninit in libcore](https://github.com/rust-lang/rust/pull/54668)
* [fix futures creating aliasing mutable and shared ref](https://github.com/rust-lang/rust/pull/56319)
* [add libstd Cargo feature `panic_immediate_abort`](https://github.com/rust-lang/rust/pull/55011)
* [cargo: ConflictStoreTrie: faster filtered search](https://github.com/rust-lang/cargo/pull/6366)
* [crates.io: email verification warning](https://github.com/rust-lang/crates.io/pull/1565)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2591: Stabilise exhaustive integer pattern matching](https://github.com/rust-lang/rfcs/pull/2591).
* [RFC 2500: Needle API (née Pattern API)](https://github.com/rust-lang/rfcs/pull/2500).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize memory-releated `std::arch::wasm32` intrinsics](https://github.com/rust-lang/rust/issues/56292).
* [disposition: merge] [Tracking issue for RFC 2300, "`Self` in type definitions"](https://github.com/rust-lang/rust/issues/49303).
* [disposition: merge] [Tracking issue for str::split_ascii_whitespace](https://github.com/rust-lang/rust/issues/48656).
* [disposition: merge] [Tracking issue for Vec::resize_with and resize_default](https://github.com/rust-lang/rust/issues/41758).
* [disposition: close] [Tracking issue for feature `extern_in_paths`](https://github.com/rust-lang/rust/issues/55600).

## New RFCs

* [impl trait expressions](https://github.com/rust-lang/rfcs/pull/2604).

# Upcoming Events

### Online

* [Dec 12. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Dec 17. Rust Community Content Subteam Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Dec 19. Rust Events Team Meeting on Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).

### Asia Pacific

* [Dec  6. Pune, IN - Rust workshop at Pune, India](https://reps.mozilla.org/e/rust-community-meetup-pune/).
* [Dec 12. Hangzhou, CN - Rust Hangzhou](https://www.meetup.com/Rust-Hangzhou/events/256338781/).
* [Dec 16. Sydney, AU - Rust Sydney Meetup 15](https://www.meetup.com/Rust-Sydney/events/256668602/).

### Europe

* [Dec 10. Vienna, AT - Metalab - Rust Workshop](https://metalab.at/wiki/Rust-Workshop).
* [Dec 11. Zurich, CH - Rust Zurich - Rust Embedded Edition 2018](https://www.meetup.com/Rust-Zurich/events/255279763/).
* [Dec 12. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyxqbqb/).
* [Dec 12. Milano, IT - Milano - Hello Open Closed Principle](https://www.meetup.com/rust-language-milano/events/256948632/).
* [Dec 15 & 16. Moscow, RU - RustRush 2018](https://rustrush.ru).
* [Dec 20. Cambridge, GB - The Last Cambridge Rust](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxqbbc/)?
* [Dec 20. Turin, IT - Gruppo di studio Rust](https://www.meetup.com/Mozilla-Torino/events/sbtclqyxqbkc/).

### North America

* [Dec  6. Phoenix, US - Phoenix 2018 Edition Release Party](https://www.meetup.com/Desert-Rustaceans/events/256503618).
* [Dec  9. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxqbmb/).
* [Dec 10. Seattle, US - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/pkggvpyxqbnb/).
* [Dec 12. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rzszlqyxqbqb/).
* [Dec 12. Boulder, US - Rust Boulder/Denver Monthly Meeting](https://www.meetup.com/Rust-Boulder-Denver/events/256949931/).
* [Dec 13. Arlington, US - Rust DC — Mid-month Rustful](https://www.meetup.com/RustDC/events/256181658).
* [Dec 13. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxqbrb/).
* [Dec 13. Utah, US - Utah Rust monthly meetup](https://www.meetup.com/utahrust/events/255209738/).
* [Dec 13. San Diego, US - San Diego Rust December Meetup - Rust 2018 Overview + Memory Allocator](https://www.meetup.com/San-Diego-Rust/events/256264465/).
* [Dec 16. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxqbvb/).
* [Dec 20. Chicago, US - Rust for the Holidays](https://www.meetup.com/Chicago-Rust-Meetup/events/256778181).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at Parity, Berlin, DE](https://paritytech.io/jobs/).
* [Rust Engineer at Commure, Inc. (San Francisco, Boston, Montreal)](https://www.commure.com/#jobSection).
* [Tech Lead at Hashintel, London, GB](https://twitter.com/nonparibus/status/1067893414765764614).
* [Intermediate Software Developer at Finhaven, Vancouver, CA](https://angel.co/finhaven/jobs/411238-intermediate-software-developer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> The bug I did not have

– /u/pacman82's [reddit post](https://www.reddit.com/r/rust/comments/a1w75c/the_bug_i_did_not_have/) title

Thanks to [Felix](https://users.rust-lang.org/t/twir-quote-of-the-week/328/582) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
