use super::{
    AiConfig, HISTORY_IO_LOCK, LifetimeStats, PathBuf, append_private_line, fs, history_path,
    load_lines, save_lines, spreadsheet_safe,
};

pub(super) const HISTORY_VISIBLE_LIMIT: usize = 500;
const HISTORY_COMPACTION_THRESHOLD: usize = 600;

pub(super) struct HistoryContext<'a> {
    pub(super) raw_text: &'a str,
    pub(super) provider: &'a str,
    pub(super) model: &'a str,
    pub(super) ai_status: &'a str,
    pub(super) ai_duration_ms: u128,
    pub(super) source: &'a str,
    pub(super) audio_path: &'a str,
}

pub(super) struct HistoryUpdate {
    pub(super) entries: Vec<String>,
    pub(super) transcript_count: i32,
    pub(super) dictated_word_count: i32,
}

pub(super) fn record_history(text: &str, context: &HistoryContext<'_>) -> HistoryUpdate {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs());
    let _history_guard = HISTORY_IO_LOCK.lock().ok();
    let path = history_path();
    let entry = format!(
        "{timestamp}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        history_value(text),
        history_value(context.raw_text),
        history_value(context.provider),
        history_value(context.model),
        history_value(context.ai_status),
        context.ai_duration_ms,
        history_value(context.source),
        history_value(context.audio_path)
    );
    let appended = append_private_line(&path, &entry).is_ok();
    let mut history = load_lines(&path);
    if !appended {
        history.push(entry);
        save_lines(&path, &history).ok();
    } else if history.is_empty() {
        history.push(entry);
    }
    if compact_history(&mut history) {
        save_lines(&path, &history).ok();
    }
    let mut lifetime_stats = LifetimeStats::load_or_migrate(&history[..history.len() - 1]);
    if context.source != "file" {
        lifetime_stats.transcript_count = lifetime_stats.transcript_count.saturating_add(1);
        lifetime_stats.dictated_word_count = lifetime_stats
            .dictated_word_count
            .saturating_add(u64::try_from(text.split_whitespace().count()).unwrap_or(u64::MAX));
        lifetime_stats.save().ok();
    }
    HistoryUpdate {
        entries: history
            .into_iter()
            .rev()
            .take(HISTORY_VISIBLE_LIMIT)
            .collect(),
        transcript_count: lifetime_stats.transcript_count_i32(),
        dictated_word_count: lifetime_stats.dictated_word_count_i32(),
    }
}

fn compact_history(history: &mut Vec<String>) -> bool {
    if history.len() <= HISTORY_COMPACTION_THRESHOLD {
        return false;
    }
    history.drain(..history.len() - HISTORY_VISIBLE_LIMIT);
    true
}

pub(super) fn history_value(value: &str) -> String {
    value.replace(['\n', '\r', '\t'], " ")
}

pub(super) fn history_field(entry: &str, index: usize) -> Option<&str> {
    entry.split('\t').nth(index)
}

pub(super) fn history_clipboard_text(entry: &str, mode: i32) -> (String, &'static str) {
    let final_text = history_field(entry, 1).unwrap_or(entry);
    let raw_text = history_field(entry, 2).unwrap_or(final_text);
    match mode {
        0 => (
            raw_text.to_owned(),
            "Raw transcript copied · ready to paste",
        ),
        2 => (
            format!("Raw transcript:\n{raw_text}\n\nFinal text:\n{final_text}"),
            "Raw and final text copied · ready to paste",
        ),
        3 => (
            raw_text.to_owned(),
            "AI result undone on clipboard · raw text ready to paste",
        ),
        _ => (
            final_text.to_owned(),
            "Final transcript copied · ready to paste",
        ),
    }
}

pub(super) fn ai_provider_name(config: &AiConfig) -> &str {
    if config.enabled {
        config.provider.as_str()
    } else {
        ""
    }
}

pub(super) fn write_history_export(
    path: &PathBuf,
    format: &str,
    history: &[String],
) -> Result<(), String> {
    let records = history
        .iter()
        .map(|entry| {
            serde_json::json!({
                "timestamp": history_field(entry, 0).and_then(|value| value.parse::<u64>().ok()).unwrap_or(0),
                "text": history_field(entry, 1).unwrap_or(entry),
                "raw_text": history_field(entry, 2).unwrap_or_else(|| history_field(entry, 1).unwrap_or(entry)),
                "ai_provider": history_field(entry, 3).unwrap_or(""),
                "ai_model": history_field(entry, 4).unwrap_or(""),
                "ai_status": history_field(entry, 5).unwrap_or("not_recorded"),
                "ai_duration_ms": history_field(entry, 6).and_then(|value| value.parse::<u128>().ok()).unwrap_or(0),
                "source": history_field(entry, 7).unwrap_or("dictation"),
                "audio_path": history_field(entry, 8).unwrap_or(""),
            })
        })
        .collect::<Vec<_>>();
    let contents = if format.eq_ignore_ascii_case("csv") {
        let mut output =
            "timestamp,text,raw_text,ai_provider,ai_model,ai_status,ai_duration_ms,source,audio_path\n"
                .to_owned();
        for record in &records {
            let fields = [
                record["timestamp"].to_string(),
                record["text"].as_str().unwrap_or_default().to_owned(),
                record["raw_text"].as_str().unwrap_or_default().to_owned(),
                record["ai_provider"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
                record["ai_model"].as_str().unwrap_or_default().to_owned(),
                record["ai_status"].as_str().unwrap_or_default().to_owned(),
                record["ai_duration_ms"].to_string(),
                record["source"].as_str().unwrap_or_default().to_owned(),
                record["audio_path"].as_str().unwrap_or_default().to_owned(),
            ];
            output.push_str(
                &fields
                    .map(|field| format!("\"{}\"", spreadsheet_safe(&field).replace('"', "\"\"")))
                    .join(","),
            );
            output.push('\n');
        }
        output
    } else {
        serde_json::to_string_pretty(&records).map_err(|error| error.to_string())?
    };
    fs::write(path, contents).map_err(|error| error.to_string())
}

pub(super) fn decode_file_url(value: &str) -> String {
    let value = value.strip_prefix("file://").unwrap_or(value);
    let bytes = value.as_bytes();
    let mut result = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%'
            && index + 2 < bytes.len()
            && let (Some(high), Some(low)) =
                (hex_digit(bytes[index + 1]), hex_digit(bytes[index + 2]))
        {
            let decoded = high * 16 + low;
            result.push(decoded);
            index += 3;
            continue;
        }
        result.push(bytes[index]);
        index += 1;
    }
    String::from_utf8_lossy(&result).into_owned()
}

const fn hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{HISTORY_COMPACTION_THRESHOLD, HISTORY_VISIBLE_LIMIT, compact_history};

    #[test]
    fn history_compacts_periodically_to_the_visible_window() {
        let mut history = (0..=HISTORY_COMPACTION_THRESHOLD)
            .map(|index| index.to_string())
            .collect::<Vec<_>>();
        assert!(compact_history(&mut history));
        assert_eq!(history.len(), HISTORY_VISIBLE_LIMIT);
        assert_eq!(history.last().map(String::as_str), Some("600"));
        assert_eq!(history.first().map(String::as_str), Some("101"));
        assert!(!compact_history(&mut history));
    }
}
