# Monofolio V2 UI Design Guide

This design guide explains the global UI pattern used in `monofolio-v2`. It is intended to help keep the frontend consistent, maintainable, and aligned with the existing visual system.

## Purpose

- Capture the current global UI pattern and design philosophy.
- Help future work follow the same visual language across pages and components.
- Encourage reuse of shared components and theme tokens.
- Reduce duplicated styling and one-off Tailwind classes.

## Visual identity

The site combines:

- **Cyberpunk**: dark base, neon red/crimson primary, strong contrast, crisp edges.
- **Steampunk**: warm metallic accents, subtle tactile textures, layered surfaces.

Key style signals:

- Dark background with translucent cards and borders.
- Primary neon red for actions, links, and highlights.
- Cyan glow effects for titles, shadows, and important accents.
- Monospace and cyber-style typography for headings and UI labels.
- Circuit-grid background textures on larger sections.

## Core theme tokens

The shared design relies on the site theme defined in `apps/frontend/style/main.css` and the component palette in `apps/frontend/src/components/ui.rs`.

Use the token styles rather than hardcoded colors:

- `bg-background`, `text-foreground`
- `bg-card`, `text-card-foreground`
- `border-border`
- `bg-primary`, `text-primary`, `text-primary-foreground`
- `text-muted-foreground`
- `cyber-text-glow`, `cyber-glow`

## Shared UI primitives

The main shared primitives are defined in `apps/frontend/src/components/ui.rs`:

- `Card`
- `SectionTitle`
- `SectionInner`
- `PageSection`
- `SectionContent`
- `PanelCard`
- `TagBadge`
- `ProseContent`
- `Button` with variants: `Primary`, `Outline`, `Ghost`, `Icon`

### When to use them

- Use `Card` for grouped content panels, info blocks, list items, and overlay containers.
- Use `SectionTitle` for page headings and section headings.
- Use `SectionInner` to center and constrain the content width inside page sections.
- Use `PageSection` for vertical page sections with consistent spacing.
- Use `SectionContent` to wrap inner content and enforce horizontal padding.
- Use `PanelCard` for information panels and grouped card layouts that need softer elevation.
- Use `TagBadge` for metadata chips, tag labels, and inline badges.
- Use `ProseContent` for markdown content wrappers and article-style content.
- Use the shared `Button` variants for all button-style actions and links.

## Component rules

1. Prefer shared components over custom classes.
   - Example: use `<Card>` instead of `div` with `rounded border border-border bg-card text-card-foreground`.
2. Keep repeated presentation in `apps/frontend/src/components/ui.rs`.
   - Add new UI patterns there first, then consume them in pages.
3. Avoid page-level style duplication.
   - Use `Card`, `SectionTitle`, `SectionInner`, `Button`, and shared utility classes.
4. Use `leptos_ui::clx!` and `variants!` for reusable components.

## Layout and structure

- Each page should be composed of `section` areas.
- Wrap content blocks with `SectionInner` to enforce consistent horizontal padding and max width.
- Use `SectionTitle` before major block content.
- For full-page or large hero sections, use the `cyber-grid-bg` background pattern.
- Keep section spacing consistent with padding and margins.

## Buttons and actions

The global pattern defines four button styles:

- `Primary`: for main calls to action.
- `Outline`: for secondary actions.
- `Ghost`: for low-weight actions and inline controls.
- `Icon`: for small icon buttons and toolbar elements.

Button guidance:

- Always use the shared `Button` component when styling a clickable action.
- Use `Primary` for the most important interactive element in the section.
- Use `Outline` for secondary actions and non-destructive links.
- Use `Ghost` for subtle actions that should not compete visually.
- Use `Icon` for icon-only controls and utilities.

## Text and typography

- Headings should use monospace or strongly geometric type styles for the cyber feel.
- `SectionTitle` is the default section heading component.
- Body copy should remain clean and readable.
- Links and inline actions should use `text-primary` with hover transitions.

## Interaction and motion

- Prefer subtle motion and state transitions.
- Use `transition-colors` and mild transform states on hover.
- Keep hover states consistent across buttons and interactive elements.
- Glow effects are reserved for highlights and titles, not for all text.

## Accessibility

Follow these accessibility rules:

- Keep text contrast strong on dark backgrounds.
- Use semantic tags like `section`, `header`, `nav`, `main`, and `button`.
- Provide `aria-label` and accessible text for icons.
- Ensure focus visibility and keyboard operability for all controls.
- Prefer `Button` variants for actions to keep behavior consistent.

## Patterns for new UI

When adding new UI patterns:

1. Define the style in `apps/frontend/src/components/ui.rs` if it will be reused.
2. Keep tokens and design decisions centralized.
3. Reuse the existing `cyber-` utility classes from `apps/frontend/style/main.css`:
   - `cyber-grid-bg`
   - `cyber-text-glow`
   - `cyber-glow`
4. Prefer shared wrappers for semantic sections and markdown content.
   - Use `PageSection` and `SectionContent` for page layout.
   - Use `ProseContent` for markdown/article rendering.
   - Use `TagBadge` for inline metadata chips and tags.
5. Keep component markup simple and declarative.
6. Avoid hardcoded colors or one-off Tailwind utilities unless the pattern is unique.

## Example guidelines

Use this shared pattern to decide between one-off styling and reusable components.

### Preferred

```rust
view! {
  <SectionInner>
    <SectionTitle>"Projects"</SectionTitle>
    <Card>
      <p class="text-sm text-muted-foreground">"A consistent panel style."</p>
      <Button variant=ButtonVariant::Primary>"View details"</Button>
    </Card>
  </SectionInner>
</SectionInner>
```

### Avoid

```rust
view! {
  <div class="rounded border border-border bg-card px-6 py-5 text-card-foreground">
    <h2 class="text-2xl font-mono mb-8">"Projects"</h2>
    <button class="bg-primary text-primary-foreground px-5 py-2.5 rounded">"View details"</button>
  </div>
```

## Future proofing

- New shared components should be added to `apps/frontend/src/components/ui.rs`.
- Keep the global visual tokens in `apps/frontend/style/main.css`.
- If a new color or style appears in multiple places, add it to the shared component system instead of duplicating it.
- Document new components and patterns here as the design evolves.
- There is also an MCP for the lib rust-ui that can be used

## Ownership

- The frontend team and future contributors should use this file as the source of truth for global UI expectations.
- When a new design pattern is introduced, add a short note here with its intent and usage.
