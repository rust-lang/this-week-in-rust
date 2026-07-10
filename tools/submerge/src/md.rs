use anyhow::{Result, anyhow, bail};
use pulldown_cmark::{Event, HeadingLevel, Parser as MarkdownParser, Tag, TagEnd};
use std::ops::Range;

const COMMUNITY_HEADING: &str = "Updates from Rust Community";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListItem {
    pub(crate) section: Option<String>,
    pub(crate) item: String,
}

#[derive(Debug, Clone, PartialEq)]
struct ComparableEvent {
    event: Event<'static>,
    range: Range<usize>,
    section: Option<String>,
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
        let Some((level, title)) = self.heading.take() else {
            return None;
        };
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
}

pub(crate) fn find_single_added_community_list_item(
    base_text: &str,
    head_text: &str,
) -> Result<ListItem> {
    let base_events = comparable_events(base_text);
    let head_events = comparable_events(head_text);
    let inserted_range = single_inserted_event_range(&base_events, &head_events)?;
    let item = single_inserted_item(head_text, &head_events[inserted_range])?;
    if item.section.is_none() {
        bail!("does not add a valid community list item");
    }
    Ok(item)
}

fn single_inserted_event_range(
    base: &[ComparableEvent],
    head: &[ComparableEvent],
) -> Result<Range<usize>> {
    let mut prefix_len = 0;
    while prefix_len < base.len()
        && prefix_len < head.len()
        && base[prefix_len].event == head[prefix_len].event
    {
        prefix_len += 1;
    }

    let mut suffix_len = 0;
    while suffix_len < base.len() - prefix_len
        && suffix_len < head.len() - prefix_len
        && base[base.len() - 1 - suffix_len].event == head[head.len() - 1 - suffix_len].event
    {
        suffix_len += 1;
    }

    let base_middle = prefix_len..base.len() - suffix_len;
    let head_middle = prefix_len..head.len() - suffix_len;
    if base_middle.is_empty() {
        Ok(head_middle)
    } else if event_range_contains_item(&head[head_middle]) {
        let base_has_item = event_range_contains_item(&base[base_middle]);
        if base_has_item {
            bail!("adds to an existing markdown list item");
        }
        bail!("adds multiple blocks");
    } else {
        bail!("changes existing markdown content");
    }
}

fn event_range_contains_item(events: &[ComparableEvent]) -> bool {
    events
        .iter()
        .any(|event| event.event == Event::Start(Tag::Item))
}

fn single_inserted_item(text: &str, events: &[ComparableEvent]) -> Result<ListItem> {
    let mut start_list_seen = false;
    let mut end_list_seen = false;
    let mut item_depth = 0;
    let mut item_start = None;
    let mut item_end = None;
    let mut item_section = None;

    for event in events {
        match &event.event {
            Event::Start(Tag::List(_)) if item_start.is_none() => {
                if start_list_seen {
                    bail!("adds multiple blocks");
                }
                start_list_seen = true;
            }
            Event::End(TagEnd::List(_)) if item_end.is_some() => {
                if end_list_seen {
                    bail!("adds multiple blocks");
                }
                end_list_seen = true;
            }
            Event::Start(Tag::Item) => {
                if item_start.is_none() && item_is_indented(text, event.range.start) {
                    bail!("adds to an existing markdown list item");
                }
                item_depth += 1;
                if item_depth == 1 {
                    if item_start.replace(event.range.start).is_some() {
                        bail!("adds multiple blocks");
                    }
                    item_section = event.section.clone();
                }
            }
            Event::End(TagEnd::Item) => {
                if item_depth == 0 {
                    bail!("changes existing markdown content");
                }
                if item_depth == 1 {
                    item_end = Some(event.range.end);
                }
                item_depth -= 1;
            }
            _ if item_start.is_some() && item_end.is_none() => {}
            _ => bail!("adds multiple blocks"),
        }
    }

    if item_depth != 0 {
        bail!("changes existing markdown content");
    }
    let start = item_start.ok_or_else(|| anyhow!("does not add a valid community list item"))?;
    let end = item_end.ok_or_else(|| anyhow!("does not add a valid community list item"))?;
    let item_text = text
        .get(start..end)
        .map(str::trim_end)
        .ok_or_else(|| anyhow!("could not locate added item in head draft"))?;
    Ok(ListItem {
        section: item_section,
        item: item_text.to_string(),
    })
}

fn item_is_indented(text: &str, item_start: usize) -> bool {
    let line_start = text[..item_start]
        .rfind('\n')
        .map(|index| index + 1)
        .unwrap_or(0);
    text[line_start..item_start]
        .chars()
        .any(|ch| !matches!(ch, ' ' | '\t'))
        || item_start > line_start
}

fn comparable_events(text: &str) -> Vec<ComparableEvent> {
    let mut events = Vec::new();
    let mut sections = SectionTracker::default();
    let mut heading_depth = 0;

    for (event, range) in MarkdownParser::new(text).into_offset_iter() {
        match &event {
            Event::Start(Tag::Heading { level, .. }) => {
                heading_depth += 1;
                sections.start_heading(*level);
            }
            Event::End(TagEnd::Heading(_)) => {
                sections.end_heading();
                heading_depth -= 1;
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

        if heading_depth == 0
            && let Some(event) = comparable_event(&event)
        {
            events.push(ComparableEvent {
                event,
                range,
                section: sections.current_section(),
            });
        }
    }

    events
}

fn comparable_event(event: &Event<'_>) -> Option<Event<'static>> {
    match event {
        Event::SoftBreak | Event::HardBreak => None,
        _ => Some(event.clone().into_static()),
    }
}

fn markdown_list_items(text: &str) -> Vec<ListItem> {
    let events = comparable_events(text);
    let mut items = Vec::new();
    let mut item_depth = 0;
    let mut item_start = None;
    let mut item_section = None;

    for event in events {
        match event.event {
            Event::Start(Tag::Item) => {
                item_depth += 1;
                if item_depth == 1 {
                    item_start = Some(event.range.start);
                    item_section = event.section;
                }
            }
            Event::End(TagEnd::Item) => {
                if item_depth == 1
                    && let Some(start) = item_start.take()
                    && let Some(item_text) = text.get(start..event.range.end).map(str::trim_end)
                    && !item_text.is_empty()
                {
                    items.push(ListItem {
                        section: item_section.take(),
                        item: item_text.to_string(),
                    });
                }
                item_depth -= 1;
            }
            _ => {}
        }
    }

    items
}

pub(crate) fn is_single_list_item(item: &str) -> bool {
    markdown_list_items(item)
        .into_iter()
        .count()
        == 1
}
