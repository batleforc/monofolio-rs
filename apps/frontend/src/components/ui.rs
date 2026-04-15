use leptos::prelude::*;
use leptos_ui::clx;
use leptos_ui::variants;

// ── Reusable UI primitives built with leptos_ui ──────────────────────────

/// Card wrapper with border, rounded corners and a subtle background.
clx! { Card, div, "rounded-lg border border-border bg-card text-card-foreground" }

/// Section title with an orange accent underline.
clx! { SectionTitle, h2, "text-2xl font-bold tracking-tight mb-8 relative inline-block after:block after:h-[3px] after:w-10 after:bg-primary after:rounded-sm after:mt-1" }

/// Inner wrapper that centres content and adds responsive horizontal padding.
clx! { SectionInner, div, "max-w-5xl mx-auto px-5 py-16" }

// ── Button variants via leptos_ui::variants! ─────────────────────────────

variants! {
    Button {
        base: "inline-flex items-center gap-2 font-semibold rounded-lg transition-all duration-150",
        variants: {
            variant: {
                Primary: "bg-primary text-primary-foreground hover:opacity-90 hover:-translate-y-px active:translate-y-0 px-5 py-2.5 text-sm shadow",
                Outline: "border border-border bg-transparent text-foreground hover:bg-accent hover:text-accent-foreground px-5 py-2.5 text-sm",
                Ghost: "bg-transparent text-foreground hover:bg-accent px-3 py-1.5 text-sm",
                Icon: "border border-border rounded-full w-9 h-9 justify-center text-foreground hover:bg-primary hover:text-primary-foreground hover:border-primary",
            },
            size: {
                Default: "",
                Sm: "text-xs px-3 py-1.5",
                Lg: "text-base px-6 py-3",
            }
        }
    }
}
