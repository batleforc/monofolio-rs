use std::fs;

use super::{ContentDatabase, ContentDatabaseError, ContentOutputBundle};
use crate::SITE_URL;

pub(super) fn write_blog_rss_feed(
    bundle: &ContentOutputBundle,
    database: &ContentDatabase,
) -> Result<(), ContentDatabaseError> {
    let rss_path = bundle.root_dir.join("rss.xml");
    let mut payload = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<rss version=\"2.0\">\n  <channel>\n    <title>Blog</title>\n    <link>{SITE_URL}/blog</link>\n    <description>Latest blog posts</description>\n",
    );

    for entry in &database.blog_timeline {
        let title = escape_xml(&entry.title);
        let description = escape_xml(&entry.description);
        let link = blog_link_from_handle(&entry.handle);
        let link = escape_xml(&link);

        payload.push_str("    <item>\n");
        payload.push_str(&format!("      <title>{title}</title>\n"));
        payload.push_str(&format!("      <link>{link}</link>\n"));
        payload.push_str(&format!("      <guid>{link}</guid>\n"));
        payload.push_str(&format!("      <description>{description}</description>\n"));
        if let Some(pub_date) = rfc822_date(&entry.date) {
            payload.push_str(&format!("      <pubDate>{pub_date}</pubDate>\n"));
        }

        for tag in &entry.tags {
            let category = escape_xml(tag);
            payload.push_str(&format!("      <category>{category}</category>\n"));
        }

        payload.push_str("    </item>\n");
    }

    payload.push_str("  </channel>\n</rss>\n");

    fs::write(&rss_path, payload).map_err(|source| ContentDatabaseError::WriteFile {
        path: rss_path.display().to_string(),
        source,
    })
}

fn blog_link_from_handle(handle: &str) -> String {
    format!("{SITE_URL}/{}", handle.trim_start_matches('/'))
}

/// Convert an ISO 8601 date (`2026-04-18`, `2026-04-18T02:18:00Z`, `...+02:00`)
/// to the RFC 822 form RSS 2.0 requires. `None` when the input is not parseable.
fn rfc822_date(iso: &str) -> Option<String> {
    const DAYS: [&str; 7] = ["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"];
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let iso = iso.trim();
    let year: i64 = iso.get(0..4)?.parse().ok()?;
    let month: usize = iso.get(5..7)?.parse().ok()?;
    let day: i64 = iso.get(8..10)?.parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let rest = iso.get(10..).unwrap_or("");
    let (time, zone) = match rest.strip_prefix('T') {
        Some(t) => {
            let time = t.get(0..8).filter(|s| s.len() == 8)?;
            let zone = match t
                .get(8..)
                .unwrap_or("")
                .trim_start_matches(|c: char| c == '.' || c.is_ascii_digit())
            {
                "" | "Z" => "+0000".to_string(),
                z if z.len() == 6 && (z.starts_with('+') || z.starts_with('-')) => {
                    z.replace(':', "")
                }
                _ => return None,
            };
            (time, zone)
        }
        None if rest.is_empty() => ("00:00:00", "+0000".to_string()),
        None => return None,
    };
    // Days since 1970-01-01 (Howard Hinnant's days_from_civil); 1970-01-01 was a Thursday.
    let y = if month <= 2 { year - 1 } else { year };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let m = month as i64;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    let weekday = DAYS.get(days.rem_euclid(7) as usize)?;
    let month_name = MONTHS.get(month - 1)?;
    Some(format!(
        "{weekday}, {day:02} {month_name} {year} {time} {zone}"
    ))
}

pub fn escape_xml(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());

    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            _ => escaped.push(ch),
        }
    }

    escaped
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;
    use crate::database::BlogTimelineEntry;

    fn temporary_directory() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be valid")
            .as_nanos();
        std::env::temp_dir().join(format!("content-rss-test-{nanos}"))
    }

    fn bundle_at(root: PathBuf) -> ContentOutputBundle {
        ContentOutputBundle {
            root_dir: root.clone(),
            db_path: root.join("db.json"),
            config_path: root.join("config.yaml"),
            home_path: root.join("home.yaml"),
            public_dir: root.join("public"),
        }
    }

    #[test]
    fn writes_rss_file_for_blog_timeline_entries() {
        let root = temporary_directory();
        fs::create_dir_all(&root).expect("create root");

        let bundle = bundle_at(root.clone());
        let database = ContentDatabase {
            generated_at_unix: 0,
            entries: vec![],
            sidebar: vec![],
            blog_timeline: vec![BlogTimelineEntry {
                title: "First post".to_string(),
                description: "Intro".to_string(),
                handle: "blogs/first-post".to_string(),
                date: "2025-01-01T00:00:00Z".to_string(),
                tags: vec!["rust".to_string(), "leptos".to_string()],
                minia: None,
                image: String::new(),
                reading_time_minutes: 2,
                draft: false,
            }],
            technology_map: vec![],
        };

        write_blog_rss_feed(&bundle, &database).expect("write rss");

        let rss = fs::read_to_string(root.join("rss.xml")).expect("read rss");
        assert!(rss.contains("<rss version=\"2.0\">"));
        assert!(rss.contains("<title>First post</title>"));
        assert!(rss.contains("<link>https://maxleriche.net/blogs/first-post</link>"));
        assert!(rss.contains("<guid>https://maxleriche.net/blogs/first-post</guid>"));
        assert!(rss.contains("<category>rust</category>"));
        assert!(rss.contains("<category>leptos</category>"));

        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn writes_rss_file_even_when_timeline_is_empty() {
        let root = temporary_directory();
        fs::create_dir_all(&root).expect("create root");

        let bundle = bundle_at(root.clone());
        let database = ContentDatabase {
            generated_at_unix: 0,
            entries: vec![],
            sidebar: vec![],
            blog_timeline: vec![],
            technology_map: vec![],
        };

        write_blog_rss_feed(&bundle, &database).expect("write rss");

        let rss = fs::read_to_string(root.join("rss.xml")).expect("read rss");
        assert!(rss.contains("<channel>"));
        assert!(!rss.contains("<item>"));

        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn escapes_xml_sensitive_characters_in_rss_fields() {
        let root = temporary_directory();
        fs::create_dir_all(&root).expect("create root");

        let bundle = bundle_at(root.clone());
        let database = ContentDatabase {
            generated_at_unix: 0,
            entries: vec![],
            sidebar: vec![],
            blog_timeline: vec![BlogTimelineEntry {
                title: "A & B <C> \"D\" 'E'".to_string(),
                description: "Desc & <tag>".to_string(),
                handle: "custom/path".to_string(),
                date: "2025-01-01T00:00:00Z".to_string(),
                tags: vec!["ops & infra".to_string()],
                minia: None,
                image: String::new(),
                reading_time_minutes: 1,
                draft: false,
            }],
            technology_map: vec![],
        };

        write_blog_rss_feed(&bundle, &database).expect("write rss");

        let rss = fs::read_to_string(root.join("rss.xml")).expect("read rss");
        assert!(rss.contains("<title>A &amp; B &lt;C&gt; &quot;D&quot; &apos;E&apos;</title>"));
        assert!(rss.contains("<description>Desc &amp; &lt;tag&gt;</description>"));
        assert!(rss.contains("<link>https://maxleriche.net/custom/path</link>"));
        assert!(rss.contains("<pubDate>Wed, 01 Jan 2025 00:00:00 +0000</pubDate>"));
        assert!(rss.contains("<category>ops &amp; infra</category>"));

        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn rfc822_date_handles_iso_variants() {
        assert_eq!(
            rfc822_date("2026-04-18T02:18:00Z").as_deref(),
            Some("Sat, 18 Apr 2026 02:18:00 +0000")
        );
        assert_eq!(
            rfc822_date("2024-02-29T23:05:09.123+02:00").as_deref(),
            Some("Thu, 29 Feb 2024 23:05:09 +0200")
        );
        assert_eq!(
            rfc822_date("2000-01-01").as_deref(),
            Some("Sat, 01 Jan 2000 00:00:00 +0000")
        );
        assert_eq!(rfc822_date(""), None);
        assert_eq!(rfc822_date("not a date"), None);
        assert_eq!(rfc822_date("2026-13-01"), None);
    }
}
