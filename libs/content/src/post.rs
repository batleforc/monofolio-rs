use serde::{Deserialize, Serialize};

use crate::markdown::{parse_markdown_document, MarkdownContent, MarkdownMeta};

/// Metadata for a blog post or project page.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostMeta {
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub date: String,
    /// URL-friendly identifier derived from the file name.
    pub slug: String,
}

impl From<(MarkdownMeta, String)> for PostMeta {
    fn from((meta, slug): (MarkdownMeta, String)) -> Self {
        Self {
            title: meta.title,
            description: meta.description,
            tags: meta.tags,
            draft: meta.draft,
            date: meta.date,
            slug,
        }
    }
}

/// A fully parsed post with its markdown content payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Post {
    pub meta: PostMeta,
    pub content: MarkdownContent,
}

impl Post {
    /// Parse a raw markdown file and produce a [`Post`].
    ///
    /// `slug` is the URL-friendly identifier (typically the filename without
    /// the `.md` extension).
    pub fn from_markdown(slug: &str, input: &str) -> Self {
        let doc = parse_markdown_document(input);
        Self {
            meta: PostMeta::from((doc.meta, slug.to_string())),
            content: doc.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_from_markdown_sets_slug() {
        let md = "---\ntitle: Hello\n---\nBody.";
        let post = Post::from_markdown("hello-world", md);
        assert_eq!(post.meta.slug, "hello-world");
        assert_eq!(post.meta.title, "Hello");
        assert!(!post.content.nodes.is_empty());
    }

    #[test]
    fn post_tags_default_to_empty() {
        let md = "---\ntitle: No Tags\n---\nContent.";
        let post = Post::from_markdown("no-tags", md);
        assert!(post.meta.tags.is_empty());
    }

    #[test]
    fn draft_post_is_marked_as_draft() {
        let md = "---\ntitle: Draft\ndraft: true\n---\nDraft content.";
        let post = Post::from_markdown("draft-post", md);
        assert!(post.meta.draft);
    }
}
