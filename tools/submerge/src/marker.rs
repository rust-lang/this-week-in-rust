use crate::Submission;
use anyhow::{Result, anyhow};
use gix::ObjectId;
use regex::Regex;
use url::Url;

pub(crate) const TOKEN: &str = "submerge-pr:";

#[derive(Debug, Clone)]
pub(crate) struct Capture {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) text: String,
}

#[derive(Debug, Clone)]
pub(crate) struct Attrs {
    pub(crate) pr: u64,
    pub(crate) url: Url,
    pub(crate) sha: ObjectId,
    pub(crate) author: String,
    pub(crate) pr_title: String,
}

impl Attrs {
    pub(crate) fn from_submission(submission: &Submission) -> Self {
        Self {
            pr: submission.pr,
            url: submission.pr_url.clone(),
            sha: submission.head_sha,
            author: submission.author.clone(),
            pr_title: submission.pr_title.clone(),
        }
    }

    pub(crate) fn parse(marker: &str) -> Result<Self> {
        let marker_re = Regex::new(
            r"^<!-- url=(?P<url>\S+) submerge-pr:(?P<pr>\d+) sha=(?P<sha>[0-9a-fA-F]{40}) author=(?P<author>\S+)(?: title=(?P<pr_title>.*?))? -->$",
        )?;
        let captures = marker_re
            .captures(marker.trim())
            .ok_or_else(|| anyhow!("malformed submerge marker: {marker}"))?;
        Ok(Self {
            pr: captures["pr"].parse()?,
            url: Url::parse(&captures["url"])?,
            sha: captures["sha"].parse()?,
            author: captures["author"].to_string(),
            pr_title: captures
                .name("pr_title")
                .map(|m| m.as_str())
                .unwrap_or("")
                .to_string(),
        })
    }

    pub(crate) fn to_comment(&self) -> String {
        format!(
            "<!-- url={} submerge-pr:{} sha={} author={} title={} -->",
            self.url,
            self.pr,
            self.sha,
            self.author,
            comment_value(&self.pr_title)
        )
    }
}

pub(crate) fn comment_value(value: &str) -> String {
    value
        .replace("-->", "-- >")
        .replace(['\r', '\n'], " ")
        .trim()
        .to_string()
}

pub(crate) fn remove_comments(text: &str, marker_ranges: &[(usize, usize, usize)]) -> String {
    let mut out = text.to_string();
    for (item_start, marker_start, marker_end) in marker_ranges.iter().rev() {
        let mut removal_start = *marker_start;
        while removal_start > *item_start {
            match text.as_bytes()[removal_start - 1] {
                b' ' | b'\t' => removal_start -= 1,
                _ => break,
            }
        }
        out.replace_range(removal_start..*marker_end, "");
    }
    out
}
