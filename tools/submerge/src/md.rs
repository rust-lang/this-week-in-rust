use anyhow::{Result, anyhow, bail};
use log::debug;
use pulldown_cmark::{Event, HeadingLevel, Parser as MarkdownParser, Tag, TagEnd};
use similar::{Algorithm, DiffOp, capture_diff_slices};
use std::ops::Range;

const COMMUNITY_HEADING: &str = "Updates from Rust Community";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ListItem {
    pub(crate) section: Option<String>,
    pub(crate) item: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ComparableEvent {
    ListItem(ListItem),
    Raw(String),
}

#[derive(Default)]
pub(crate) struct SectionTracker {
    in_community: bool,
    current_section: Option<String>,
    heading: Option<(HeadingLevel, String)>,
}

impl SectionTracker {
    pub(crate) fn start_heading(&mut self, level: HeadingLevel) {
        self.heading = Some((level, String::new()));
    }

    pub(crate) fn push_text(&mut self, text: &str) {
        if let Some((_level, title)) = self.heading.as_mut() {
            title.push_str(text);
        }
    }

    pub(crate) fn push_break(&mut self) {
        if let Some((_level, title)) = self.heading.as_mut() {
            title.push(' ');
        }
    }

    pub(crate) fn end_heading(&mut self) -> Option<(HeadingLevel, String)> {
        let (level, title) = self.heading.take()?;
        let title = title.trim();
        if title.is_empty() {
            return None;
        }
        let title = title.to_string();
        match level {
            HeadingLevel::H2 => {
                self.in_community = title == COMMUNITY_HEADING;
                self.current_section = None;
            }
            HeadingLevel::H3 if self.in_community => {
                self.current_section = Some(title.clone());
            }
            HeadingLevel::H3 => {
                self.current_section = None;
            }
            _ => {}
        }
        Some((level, title))
    }

    pub(crate) fn current_section(&self) -> Option<String> {
        self.current_section.clone()
    }

    pub(crate) fn in_heading(&self) -> bool {
        self.heading.is_some()
    }
}

pub(crate) fn find_single_added_community_list_item(
    base_text: &str,
    head_text: &str,
) -> Result<ListItem> {
    let base_events = comparable_events(base_text);
    let head_events = comparable_events(head_text);

    debug!("base_events: {base_events:?}");
    debug!("head_events: {head_events:?}");

    let item = single_inserted_item(&base_events, &head_events)?;
    if item.section.is_none() {
        bail!("does not add a valid community list item");
    }
    Ok(item)
}

fn single_inserted_item(
    base_events: &[ComparableEvent],
    head_events: &[ComparableEvent],
) -> Result<ListItem> {
    let mut inserted = None;
    for op in capture_diff_slices(Algorithm::Myers, base_events, head_events) {
        match op {
            DiffOp::Equal { .. } => {}
            DiffOp::Delete { .. } => bail!("submission deletes existing content"),
            DiffOp::Insert {
                new_index,
                new_len: 1,
                ..
            } => {
                // inserts exactly one thing
                if inserted.is_some() {
                    bail!("submission inserts multiple items");
                }

                if let ComparableEvent::ListItem(item) = &head_events[new_index] {
                    inserted = Some(item.clone());
                }
            }
            DiffOp::Insert { .. } => bail!("submission inserts unexpected content"),
            DiffOp::Replace { .. } => bail!("submission edits existing content"),
        }
    }

    inserted.ok_or(anyhow!("no list item inserted"))
}

/// Produce a list of events, normalized to chunks of non-list-item content
/// (with whitespace changes ignored)
/// and top-level list items, so we can then easily find added list items.
fn comparable_events(text: &str) -> Vec<ComparableEvent> {
    let mut events = Vec::new();
    let mut sections = SectionTracker::default();

    let mut md_events = MarkdownParser::new(text).into_offset_iter();
    while let Some((event, range)) = md_events.next() {
        debug!("Processing {event:?}");
        match &event {
            Event::Start(Tag::Heading { level, .. }) => {
                sections.start_heading(*level);
            }
            Event::End(TagEnd::Heading(_)) => {
                sections.end_heading();
            }
            Event::Text(value)
            | Event::Code(value)
            | Event::InlineMath(value)
            | Event::DisplayMath(value)
            | Event::Html(value)
            | Event::InlineHtml(value)
            | Event::FootnoteReference(value) => {
                sections.push_text(value);
            }
            Event::SoftBreak | Event::HardBreak => {
                sections.push_break();
            }
            _ => {}
        }

        if !sections.in_heading() {
            match event {
                // ignore added/removed lists (the items are what matter)
                Event::Start(Tag::List(_)) | Event::End(TagEnd::List(_)) => {}
                // skip newlines
                Event::SoftBreak | Event::HardBreak => {}
                Event::Start(Tag::Item) => {
                    // Collect the (possibly-nested) list item into a single thing to compare
                    let start_range = range.clone();
                    fn end_of_item<'a, I: Iterator<Item = (Event<'a>, Range<usize>)>>(
                        iter: &mut I,
                    ) -> Option<Range<usize>> {
                        // find the end:
                        let mut nesting = 1;
                        for (e, range) in iter.by_ref() {
                            match e {
                                Event::Start(Tag::Item) => nesting += 1,
                                Event::End(TagEnd::Item) => nesting -= 1,
                                _ => {}
                            }
                            if nesting == 0 {
                                return Some(range);
                            }
                        }
                        None
                    }

                    let end_range = end_of_item(&mut md_events).expect("list item has an end");
                    let text = &text[start_range.start..end_range.end];
                    events.push(ComparableEvent::ListItem(ListItem {
                        section: sections.current_section(),
                        item: text.trim().into(),
                    }));
                }
                _ => events.push(ComparableEvent::Raw(text[range].into())),
            }
        }
    }

    events
}

fn markdown_list_items(text: &str) -> Vec<ListItem> {
    comparable_events(text)
        .into_iter()
        .filter_map(|e| match e {
            ComparableEvent::ListItem(i) => Some(i),
            _ => None,
        })
        .collect()
}

pub(crate) fn is_single_list_item(item: &str) -> bool {
    markdown_list_items(item).len() == 1
}

pub(crate) fn contains_list_item(text: &str, section: &str, item: &str) -> bool {
    markdown_list_items(text)
        .iter()
        // TODO: Compare canonicalized URLs so equivalent links are recognized as the same item.
        .any(|candidate| candidate.section.as_deref() == Some(section) && candidate.item == item)
}
