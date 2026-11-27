use leptos::prelude::*;
use leptos_ui::clx;
use leptos_ui::variants;

// ── Reusable UI primitives built with leptos_ui ──────────────────────────

// Static content panel: translucent red border on the dark card surface.
clx! { Card, div, "rounded border border-border bg-card text-card-foreground transition-colors" }

// Card that links somewhere: the hover border signals it is clickable.
clx! { LinkCard, div, "rounded border border-border bg-card text-card-foreground transition-colors hover:border-primary/40" }

// Page title — the single <h1> of a route; same look as SectionTitle.
clx! { PageTitle, h1, "text-3xl sm:text-4xl leading-tight text-balance font-mono font-bold tracking-tight mb-8 relative inline-block after:block after:h-[2px] after:w-10 after:bg-primary after:rounded-sm after:mt-1 cyber-text-glow" }

// Section title — monospace font with a neon-red underline accent.
clx! { SectionTitle, h2, "text-2xl font-mono font-bold tracking-tight mb-8 relative inline-block after:block after:h-[2px] after:w-10 after:bg-primary after:rounded-sm after:mt-1 cyber-text-glow" }

// Inner wrapper that centres content and adds responsive horizontal padding.
clx! { SectionInner, div, "max-w-5xl mx-auto px-5 py-16" }

// Section wrapper with consistent page spacing.
clx! { PageSection, section, "py-16" }

// Content wrapper for fixed max width inside sections.
clx! { SectionContent, div, "max-w-5xl mx-auto px-5" }

// Alternate card style for content panels.
clx! { PanelCard, div, "rounded border border-border bg-card text-card-foreground shadow-sm transition-colors" }

// Markdown article: reading measure capped at ~72 characters per line.
clx! { ProseContent, article, "max-w-[72ch] text-base leading-relaxed text-pretty [&>:first-child]:mt-0" }

// Reusable tag badge for metadata chips.
clx! { TagBadge, span, "inline-flex items-center px-2 py-0.5 rounded border border-border text-[0.7rem] text-muted-foreground transition-colors" }

// ── Button variants via leptos_ui::variants! ─────────────────────────────

variants! {
    Button {
        base: "inline-flex items-center gap-2 font-mono font-semibold rounded transition-all duration-150",
        variants: {
            variant: {
                Primary: "bg-primary text-primary-foreground hover:opacity-90 hover:-translate-y-px active:translate-y-0 px-5 py-2.5 text-sm shadow cyber-glow",
                Outline: "border border-primary/50 bg-transparent text-primary hover:bg-primary/10 hover:border-primary px-5 py-2.5 text-sm",
                Ghost: "bg-transparent text-foreground hover:bg-primary/10 hover:text-primary px-3 py-1.5 text-sm",
                Icon: "border border-border rounded w-9 h-9 justify-center text-muted-foreground hover:bg-primary/10 hover:text-primary hover:border-primary hover:shadow-[var(--cyber-glow)]",
            },
            size: {
                Default: "",
                Sm: "text-xs px-3 py-1.5",
                Lg: "text-base px-6 py-3",
            }
        }
    }
}
