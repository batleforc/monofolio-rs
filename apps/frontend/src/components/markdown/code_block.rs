use leptos::prelude::*;

use super::MarkdownNode;

fn extract_code_title(raw: &str) -> Option<String> {
    for key in ["title=\"", "title='"] {
        if let Some(start) = raw.find(key) {
            let value_start = start + key.len();
            let quote = key.chars().last().unwrap_or('"');
            if let Some(end_rel) = raw[value_start..].find(quote) {
                let value = raw[value_start..value_start + end_rel].trim();
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }

    // Support compact form: title=MySnippet
    if let Some(start) = raw.find("title=") {
        let value_start = start + "title=".len();
        let tail = &raw[value_start..];
        let end = tail.find(char::is_whitespace).unwrap_or(tail.len());
        let value = tail[..end].trim_matches(|c| c == '{' || c == '}' || c == '"' || c == '\'');
        if !value.is_empty() {
            return Some(value.to_string());
        }
    }

    None
}

fn clean_language(raw: &str) -> String {
    raw.split(|c: char| c.is_whitespace() || c == '{' || c == '}')
        .find(|part| !part.trim().is_empty() && !part.starts_with("title="))
        .unwrap_or("txt")
        .trim()
        .to_lowercase()
}

fn append_code_text(nodes: &[MarkdownNode], out: &mut String) {
    for child in nodes {
        match child.kind.as_str() {
            "text" | "code" | "html" => out.push_str(&child.text),
            "soft_break" | "hard_break" => out.push('\n'),
            _ => append_code_text(&child.children, out),
        }
    }
}

pub fn render_code_block(node: MarkdownNode) -> impl IntoView {
    let raw_language = node.attrs.get("language").cloned().unwrap_or_default();
    let language = clean_language(&raw_language);
    let title = node
        .attrs
        .get("title")
        .cloned()
        .or_else(|| extract_code_title(&raw_language));

    let code_class = if language.is_empty() {
        "language-text".to_string()
    } else {
        format!("language-{}", language)
    };

    let mut code_text = if node.text.is_empty() {
        String::new()
    } else {
        node.text.clone()
    };
    if code_text.is_empty() {
        append_code_text(&node.children, &mut code_text);
    }

    let copied = RwSignal::new(false);
    #[cfg(not(feature = "ssr"))]
    let copy_text = code_text.clone();

    #[cfg(not(feature = "ssr"))]
    let on_copy = {
        let copied = copied;
        move |_| {
            let copy_text = copy_text.clone();
            let copied = copied;
            wasm_bindgen_futures::spawn_local(async move {
                let escaped =
                    serde_json::to_string(&copy_text).unwrap_or_else(|_| "\"\"".to_string());
                let script = format!(
                    "navigator.clipboard && navigator.clipboard.writeText({}).then(() => true).catch(() => false)",
                    escaped
                );
                if js_sys::eval(&script).is_ok() {
                    copied.set(true);
                }
            });
        }
    };

    #[cfg(feature = "ssr")]
    let on_copy = move |_| {};

    #[cfg(not(feature = "ssr"))]
    {
        let shiki_bundle_version = env!("CARGO_PKG_VERSION").to_string();
        Effect::new(move |_| {
            let version_json =
                serde_json::to_string(&shiki_bundle_version).unwrap_or_else(|_| "\"\"".to_string());

            let script = format!(
                r#"
                (async () => {{
                  await new Promise((resolve) => requestAnimationFrame(() => resolve(true)));
                  const blocks = Array.from(document.querySelectorAll('pre[data-language]:not([data-shiki-done="1"])'));
                  if (!blocks.length) return;

                  const ensureBundle = () => new Promise((resolve, reject) => {{
                    if (window.__mfHighlightCode) return resolve(true);
                                        if (window.__mfShikiBundlePromise) {{
                                            window.__mfShikiBundlePromise.then(() => resolve(true)).catch(reject);
                                            return;
                                        }}

                                        const version = window.__MF_SHIKI_BUNDLE_VERSION || {version_json};
                                        const explicitUrl = window.__MF_SHIKI_BUNDLE_URL
                                            || document.querySelector('meta[name="mf-shiki-bundle-url"]')?.content;

                                        const withVersion = (url) => {{
                                            if (!url || !version) return url;
                                            return url.includes('?')
                                                ? `${{url}}&v=${{encodeURIComponent(version)}}`
                                                : `${{url}}?v=${{encodeURIComponent(version)}}`;
                                        }};

                                        const candidates = [
                                            explicitUrl,
                                            '/assets/js/shiki.bundle.js',
                                            '/js/shiki.bundle.js',
                                            'js/shiki.bundle.js',
                                        ]
                                            .filter(Boolean)
                                            .map(withVersion)
                                            .filter((src, idx, arr) => arr.indexOf(src) === idx);

                                        const loadOne = (src) => new Promise((innerResolve, innerReject) => {{
                                            const existing = document.querySelector(`script[data-mf-shiki-src="${{src}}"]`);
                                            if (existing) {{
                                                existing.addEventListener('load', () => innerResolve(true), {{ once: true }});
                                                existing.addEventListener('error', () => innerReject(new Error(`load-failed:${{src}}`)), {{ once: true }});
                                                return;
                                            }}

                                            const scriptTag = document.createElement('script');
                                            scriptTag.src = src;
                                            scriptTag.defer = true;
                                            scriptTag.dataset.mfShiki = '1';
                                            scriptTag.dataset.mfShikiSrc = src;
                                            scriptTag.addEventListener('load', () => innerResolve(true), {{ once: true }});
                                            scriptTag.addEventListener('error', () => {{
                                                scriptTag.remove();
                                                innerReject(new Error(`load-failed:${{src}}`));
                                            }}, {{ once: true }});
                                            document.head.appendChild(scriptTag);
                                        }});

                                        window.__mfShikiBundlePromise = (async () => {{
                                            let lastError = null;
                                            for (const src of candidates) {{
                                                try {{
                                                    await loadOne(src);
                                                    if (typeof window.__mfHighlightCode === 'function') return true;
                                                }} catch (err) {{
                                                    lastError = err;
                                                }}
                                            }}
                                            throw lastError || new Error('no-shiki-bundle-candidate-worked');
                                        }})();

                                        window.__mfShikiBundlePromise
                                            .then(() => resolve(true))
                                            .catch((err) => {{
                                                window.__mfShikiBundlePromise = null;
                                                reject(err);
                                            }});
                  }});

                  try {{
                    await ensureBundle();
                                        if (typeof window.__mfHighlightCode === 'function') {{
                                            for (const pre of blocks) {{
                                                const language = pre.dataset.language || 'txt';
                                                await window.__mfHighlightCode(pre, language);
                                                pre.dataset.shikiDone = '1';
                                            }}
                    }}
                  }} catch (_) {{
                    // Keep raw fallback rendering.
                  }}
                }})();
                "#
            );

            let _ = js_sys::eval(&script);
        });
    }

    view! {
        <div class="mb-4 border border-border rounded-lg overflow-hidden">
            <div class="px-3 py-2 bg-muted/60 border-b border-border flex items-center justify-between gap-3">
                <div class="flex items-center gap-2 min-w-0">
                    <span class="text-[0.65rem] uppercase tracking-wider font-mono px-2 py-0.5 rounded bg-background border border-border text-muted-foreground">
                        {language.clone()}
                    </span>
                    {title
                        .as_ref()
                        .map(|value| {
                            view! {
                                <span class="text-xs text-foreground/90 truncate">
                                    {value.clone()}
                                </span>
                            }
                        })}
                </div>
                <button
                    type="button"
                    on:click=on_copy
                    class="text-xs px-2 py-1 rounded border border-border text-muted-foreground hover:text-foreground hover:border-foreground/30 transition-colors"
                >
                    {move || if copied.get() { "Copied" } else { "Copy" }}
                </button>
            </div>

            <pre data-language=language class="mb-0 bg-muted rounded-b-lg p-4 overflow-x-auto">
                <code class=code_class>{code_text}</code>
            </pre>
        </div>
    }
}
