struct HistoryContext<'a> {
    raw_text: &'a str,
    provider: &'a str,
    model: &'a str,
    ai_status: &'a str,
    ai_duration_ms: u128,
    source: &'a str,
    audio_path: &'a str,
}

fn record_history(
    mut controller: Pin<&mut ffi::FluidVoiceController>,
    text: &str,
    context: &HistoryContext<'_>,
) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs());
    let _history_guard = HISTORY_IO_LOCK.lock().ok();
    let mut history = load_lines(&history_path());
    history.push(format!(
        "{timestamp}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        history_value(text),
        history_value(context.raw_text),
        history_value(context.provider),
        history_value(context.model),
        history_value(context.ai_status),
        context.ai_duration_ms,
        history_value(context.source),
        history_value(context.audio_path)
    ));
    if history.len() > 500 {
        history.drain(..history.len() - 500);
    }
    save_lines(&history_path(), &history).ok();
    let mut lifetime_stats = LifetimeStats::load_or_migrate(&history[..history.len() - 1]);
    if context.source != "file" {
        lifetime_stats.transcript_count = lifetime_stats.transcript_count.saturating_add(1);
        lifetime_stats.dictated_word_count = lifetime_stats
            .dictated_word_count
            .saturating_add(u64::try_from(text.split_whitespace().count()).unwrap_or(u64::MAX));
        lifetime_stats.save().ok();
    }
    controller
        .as_mut()
        .set_history_entries(history.iter().rev().map(QString::from).collect());
    controller
        .as_mut()
        .set_transcript_count(lifetime_stats.transcript_count_i32());
    controller
        .as_mut()
        .set_dictated_word_count(lifetime_stats.dictated_word_count_i32());
}

fn history_value(value: &str) -> String {
    value.replace(['\n', '\r', '\t'], " ")
}

fn history_field(entry: &str, index: usize) -> Option<&str> {
    entry.split('\t').nth(index)
}

fn history_clipboard_text(entry: &str, mode: i32) -> (String, &'static str) {
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

fn ai_provider_name(config: &AiConfig) -> &str {
    if config.enabled { &config.provider } else { "" }
}

fn write_history_export(path: &PathBuf, format: &str, history: &[String]) -> Result<(), String> {
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

fn decode_file_url(value: &str) -> String {
    let value = value.strip_prefix("file://").unwrap_or(value);
    let bytes = value.as_bytes();
    let mut result = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len()
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
