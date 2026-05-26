use content::ContentEntry;

/// Shared visibility rule for content exposed in public navigation/pages.
pub fn is_content_visible(entry: &ContentEntry) -> bool {
    !entry.draft && !entry.dates.released_at.trim().is_empty()
}
