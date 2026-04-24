use std::fs;

use super::{ContentDatabase, ContentDatabaseError, ContentOutputBundle};

pub(super) fn write_blog_rss_feed(
    bundle: &ContentOutputBundle,
    database: &ContentDatabase,
) -> Result<(), ContentDatabaseError> {
    let rss_path = bundle.root_dir.join("rss.xml");
    let mut payload = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<rss version=\"2.0\">\n  <channel>\n    <title>Blog</title>\n    <link>/blog</link>\n    <description>Latest blog posts</description>\n",
    );

    for entry in &database.blog_timeline {
        let title = escape_xml(&entry.title);
        let description = escape_xml(&entry.description);
        let link = blog_link_from_handle(&entry.handle);
        let link = escape_xml(&link);
        let pub_date = escape_xml(&entry.date);

        payload.push_str("    <item>\n");
        payload.push_str(&format!("      <title>{title}</title>\n"));
        payload.push_str(&format!("      <link>{link}</link>\n"));
        payload.push_str(&format!("      <guid>{link}</guid>\n"));
        payload.push_str(&format!("      <description>{description}</description>\n"));
        payload.push_str(&format!("      <pubDate>{pub_date}</pubDate>\n"));

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
    if let Some(rest) = handle.strip_prefix("blogs/") {
        return format!("/blog/{rest}");
    }

    format!("/{handle}")
}

fn escape_xml(value: &str) -> String {
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
        };

        write_blog_rss_feed(&bundle, &database).expect("write rss");

        let rss = fs::read_to_string(root.join("rss.xml")).expect("read rss");
        assert!(rss.contains("<rss version=\"2.0\">"));
        assert!(rss.contains("<title>First post</title>"));
        assert!(rss.contains("<link>/blog/first-post</link>"));
        assert!(rss.contains("<guid>/blog/first-post</guid>"));
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
        };

        write_blog_rss_feed(&bundle, &database).expect("write rss");

        let rss = fs::read_to_string(root.join("rss.xml")).expect("read rss");
        assert!(rss.contains("<title>A &amp; B &lt;C&gt; &quot;D&quot; &apos;E&apos;</title>"));
        assert!(rss.contains("<description>Desc &amp; &lt;tag&gt;</description>"));
        assert!(rss.contains("<link>/custom/path</link>"));
        assert!(rss.contains("<category>ops &amp; infra</category>"));

        fs::remove_dir_all(root).expect("cleanup");
    }
}
