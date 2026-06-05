use crate::types::{FileDetail, Segment};

pub fn format_duration(ms: i64) -> String {
    if ms <= 0 {
        return "-".into();
    }
    let s = ms / 1000;
    let (h, m, sec) = (s / 3600, (s % 3600) / 60, s % 60);
    if h > 0 {
        format!("{h}h{m:02}m")
    } else if m > 0 {
        format!("{m}m{sec:02}s")
    } else {
        format!("{sec}s")
    }
}

pub fn format_date(iso: &str) -> String {
    if iso.len() >= 10 {
        iso[..10].to_string()
    } else {
        "-".into()
    }
}

pub fn format_time(ms: i64) -> String {
    let s = ms / 1000;
    format!("{:02}:{:02}", s / 60, s % 60)
}

/// Pull transcript segments out of a file's `source_list` (data_type "transaction").
pub fn transcript_segments(f: &FileDetail) -> Option<Vec<Segment>> {
    let item = f.source_list.iter().find(|s| s.data_type == "transaction")?;
    serde_json::from_str(&item.data_content).ok()
}

/// Pull the AI summary Markdown out of `note_list` (data_type "auto_sum_note").
pub fn summary_markdown(f: &FileDetail) -> Option<String> {
    f.note_list
        .iter()
        .find(|n| n.data_type == "auto_sum_note")
        .map(|n| n.data_content.clone())
        .filter(|s| !s.is_empty())
}

pub fn segments_to_text(segs: &[Segment]) -> String {
    segs.iter()
        .map(|seg| {
            let t = format!("[{} - {}]", format_time(seg.start_time), format_time(seg.end_time));
            let sp = seg.speaker.as_deref().map(|s| format!("{s}: ")).unwrap_or_default();
            format!("{t} {sp}{}", seg.content)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn segments_to_srt(segs: &[Segment]) -> String {
    fn ts(ms: i64) -> String {
        let s = ms / 1000;
        format!("{:02}:{:02}:{:02},{:03}", s / 3600, (s % 3600) / 60, s % 60, ms % 1000)
    }
    segs.iter()
        .enumerate()
        .map(|(i, seg)| {
            let sp = seg.speaker.as_deref().map(|s| format!("{s}: ")).unwrap_or_default();
            format!("{}\n{} --> {}\n{}{}\n", i + 1, ts(seg.start_time), ts(seg.end_time), sp, seg.content)
        })
        .collect::<Vec<_>>()
        .join("\n")
}