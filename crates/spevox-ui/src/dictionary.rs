use super::{PathBuf, QString, QStringList, dictionary_path, fs, load_lines, save_lines};

pub(super) fn process_transcript(text: &str, command_mode: bool, dictionary: &[String]) -> String {
    let mut processed = preprocess_for_cleanup(text, command_mode);
    for line in dictionary {
        let entry = DictionaryEntry::from_storage(line);
        processed = replace_ascii_case_insensitive(&processed, &entry.spoken, &entry.preferred);
    }
    processed
}

/// Applies only deterministic, meaning-preserving transforms before optional AI.
/// It intentionally does not guess sentence endings, delete fillers, or rewrite grammar.
pub(super) fn preprocess_for_cleanup(text: &str, command_mode: bool) -> String {
    let mut processed = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if command_mode {
        for (spoken, replacement) in [
            ("new paragraph", "\n\n"),
            ("new line", "\n"),
            ("question mark", "?"),
            ("exclamation mark", "!"),
            ("comma", ","),
            ("period", "."),
        ] {
            processed = replace_ascii_case_insensitive(&processed, spoken, replacement);
        }
        processed = processed
            .replace(" ,", ",")
            .replace(" .", ".")
            .replace(" ?", "?")
            .replace(" !", "!")
            .replace(" \n", "\n")
            .replace("\n ", "\n");
    }
    processed
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct DictionaryEntry {
    pub(super) spoken: String,
    pub(super) preferred: String,
}

impl DictionaryEntry {
    fn from_storage(line: &str) -> Self {
        let (spoken, preferred) = line
            .split_once('\t')
            .map_or((line, line), |(spoken, preferred)| (spoken, preferred));
        Self {
            spoken: sanitize_dictionary_field(spoken),
            preferred: sanitize_dictionary_field(preferred),
        }
    }

    fn storage(&self) -> String {
        format!("{}\t{}", self.spoken, self.preferred)
    }
}

pub(super) fn sanitize_dictionary_field(value: &str) -> String {
    value
        .replace(['\t', '\r', '\n'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn load_dictionary_entries() -> Vec<DictionaryEntry> {
    load_lines(&dictionary_path())
        .iter()
        .map(|line| DictionaryEntry::from_storage(line))
        .filter(|entry| !entry.spoken.is_empty() && !entry.preferred.is_empty())
        .collect()
}

pub(super) fn save_dictionary_entries(entries: &[DictionaryEntry]) -> Result<(), String> {
    let mut entries = entries.to_vec();
    entries.sort_by_key(|entry| entry.spoken.to_lowercase());
    entries.dedup_by(|left, right| left.spoken.eq_ignore_ascii_case(&right.spoken));
    save_lines(
        &dictionary_path(),
        &entries
            .iter()
            .map(DictionaryEntry::storage)
            .collect::<Vec<_>>(),
    )
}

pub(super) fn dictionary_display(line: &str) -> String {
    let entry = DictionaryEntry::from_storage(line);
    if entry.spoken == entry.preferred {
        entry.preferred
    } else {
        format!("{}  →  {}", entry.spoken, entry.preferred)
    }
}

pub(super) fn dictionary_ui_list(entries: &[DictionaryEntry]) -> QStringList {
    let mut entries = entries.to_vec();
    entries.sort_by_key(|entry| entry.spoken.to_lowercase());
    entries
        .iter()
        .map(|entry| QString::from(&dictionary_display(&entry.storage())))
        .collect()
}

pub(super) fn read_dictionary_import(path: &PathBuf) -> Result<Vec<DictionaryEntry>, String> {
    let contents = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut entries = Vec::new();
    for (index, line) in contents.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let fields = if line.contains('\t') {
            line.splitn(2, '\t').map(str::to_owned).collect::<Vec<_>>()
        } else {
            parse_csv_record(line)
        };
        if index == 0
            && fields.first().is_some_and(|field| {
                matches!(field.trim().to_lowercase().as_str(), "spoken" | "source")
            })
        {
            continue;
        }
        let spoken = fields
            .first()
            .map_or_else(String::new, |value| sanitize_dictionary_field(value));
        let preferred = fields
            .get(1)
            .map_or_else(|| spoken.clone(), |value| sanitize_dictionary_field(value));
        if !spoken.is_empty() && !preferred.is_empty() {
            entries.push(DictionaryEntry { spoken, preferred });
        }
    }
    Ok(entries)
}

pub(super) fn parse_csv_record(line: &str) -> Vec<String> {
    let mut fields = vec![String::new()];
    let mut quoted = false;
    let mut characters = line.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            '"' if quoted && characters.peek() == Some(&'"') => {
                fields.last_mut().expect("CSV field").push('"');
                characters.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => fields.push(String::new()),
            value => fields.last_mut().expect("CSV field").push(value),
        }
    }
    fields
}

pub(super) fn write_dictionary_csv(
    path: &PathBuf,
    entries: &[DictionaryEntry],
) -> Result<(), String> {
    use std::fmt::Write as _;

    let mut output = "spoken,preferred\n".to_owned();
    for entry in entries {
        writeln!(
            output,
            "\"{}\",\"{}\"",
            spreadsheet_safe(&entry.spoken).replace('"', "\"\""),
            spreadsheet_safe(&entry.preferred).replace('"', "\"\"")
        )
        .map_err(|error| error.to_string())?;
    }
    fs::write(path, output).map_err(|error| error.to_string())
}

pub(super) fn spreadsheet_safe(value: &str) -> String {
    if value.starts_with(['=', '+', '-', '@']) {
        format!("'{value}")
    } else {
        value.to_owned()
    }
}

fn replace_ascii_case_insensitive(text: &str, needle: &str, replacement: &str) -> String {
    let lowercase = text.to_ascii_lowercase();
    let needle = needle.to_ascii_lowercase();
    let mut result = String::with_capacity(text.len());
    let mut cursor = 0;
    while let Some(offset) = lowercase[cursor..].find(&needle) {
        let start = cursor + offset;
        let end = start + needle.len();
        let boundary_before =
            start == 0 || !lowercase.as_bytes()[start - 1].is_ascii_alphanumeric();
        let boundary_after =
            end == lowercase.len() || !lowercase.as_bytes()[end].is_ascii_alphanumeric();
        if boundary_before && boundary_after {
            result.push_str(&text[cursor..start]);
            result.push_str(replacement);
            cursor = end;
        } else {
            result.push_str(&text[cursor..end]);
            cursor = end;
        }
    }
    result.push_str(&text[cursor..]);
    result
}
