use std::time::Duration;

pub(crate) fn check_latest_release(current: &str) -> String {
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(15)))
        .timeout_recv_body(Some(Duration::from_secs(10)))
        .build()
        .new_agent();
    let response = agent
        .get("https://api.github.com/repos/davidkodar/spevox/releases?per_page=10")
        .header("user-agent", "Spevox-Linux")
        .call();
    let mut response = match response {
        Ok(response) => response,
        Err(ureq::Error::StatusCode(404)) => {
            return "No GitHub release feed is available for this repository.".to_owned();
        }
        Err(error) => return format!("Update check failed: {error}"),
    };
    let body = match response.body_mut().read_to_string() {
        Ok(body) => body,
        Err(error) => return format!("Release feed could not be read: {error}"),
    };
    let value = match serde_json::from_str::<serde_json::Value>(&body) {
        Ok(value) => value,
        Err(error) => return format!("Release feed returned invalid data: {error}"),
    };
    // The list is newest-first; prereleases count because early Spevox
    // versions are published that way. Drafts are never visible unauthenticated.
    let Some(tag) = value.as_array().and_then(|releases| {
        releases
            .iter()
            .filter(|release| {
                release.get("draft").and_then(serde_json::Value::as_bool) != Some(true)
            })
            .find_map(|release| release.get("tag_name").and_then(serde_json::Value::as_str))
    }) else {
        return "Release feed did not include a version tag.".to_owned();
    };
    let latest = tag.trim_start_matches('v');
    match (parsed_version(current), parsed_version(latest)) {
        (Some(current), Some(latest_version)) if latest_version > current => {
            format!("Version {latest} is available on GitHub Releases.")
        }
        (Some(_), Some(_)) => format!("Spevox {current} is up to date."),
        _ => format!("Latest release tag: {tag}"),
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ParsedVersion {
    numbers: (u32, u32, u32),
    stable: bool,
    prerelease: Vec<PrereleasePart>,
}

#[derive(Debug, Eq, PartialEq)]
enum PrereleasePart {
    Number(u32),
    Text(String),
}

impl Ord for PrereleasePart {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Number(left), Self::Number(right)) => left.cmp(right),
            (Self::Number(_), Self::Text(_)) => std::cmp::Ordering::Less,
            (Self::Text(_), Self::Number(_)) => std::cmp::Ordering::Greater,
            (Self::Text(left), Self::Text(right)) => left.cmp(right),
        }
    }
}

impl PartialOrd for PrereleasePart {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parsed_version(value: &str) -> Option<ParsedVersion> {
    let (numbers, prerelease) = value.split_once('-').map_or((value, ""), |parts| parts);
    let mut parts = numbers.split('.');
    let version = ParsedVersion {
        numbers: (
            parts.next()?.parse().ok()?,
            parts.next()?.parse().ok()?,
            parts.next()?.parse().ok()?,
        ),
        stable: prerelease.is_empty(),
        prerelease: prerelease
            .split(['.', '-'])
            .filter(|part| !part.is_empty())
            .map(|part| {
                part.parse().map_or_else(
                    |_| PrereleasePart::Text(part.to_owned()),
                    PrereleasePart::Number,
                )
            })
            .collect(),
    };
    parts.next().is_none().then_some(version)
}

#[cfg(test)]
mod tests {
    use super::parsed_version;

    #[test]
    fn compares_stable_and_numeric_prerelease_versions() {
        assert!(parsed_version("0.4.0") > parsed_version("0.4.0-beta.1"));
        assert!(parsed_version("0.4.0-beta.10") > parsed_version("0.4.0-beta.2"));
        assert!(parsed_version("1.2.3-beta.2") > parsed_version("0.4.0"));
        assert_eq!(parsed_version("invalid"), None);
    }
}
