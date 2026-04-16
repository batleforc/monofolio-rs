# Monofolio V2

Monofolio V2 is a migration from my previous monofolio attempt, it was build with vueJs.

The goal of this V2 is to strengthen is to redo the design, the codebase, the way i did it and work on the performance and the SEO of the website, and to make it more maintainable and scalable.

## Goals

- Redo the design of the website, make it more modern and more user friendly. Make it more SEO friendly and more performant.
- Fix the markdown parser and maybe do it at build time not at startup time.
- Fix the different pages that has been broken will writing V2, like the blog page and the project page.
- Add the the content that should have been added with all the projects done in the past 2 years, and the blog posts that i wanted to write but never had the time to do it.

## Tech Stack

- Move to [Leptos](https://github.com/leptos-rs/leptos) with SSR (using Actix) for better performance and SEO.
- Use Shadcn UI for the UI and use [Rust/UI](http://github.com/rust-ui/ui) for the components and the design system.
- handle:
  - the markdown parsing with [pulldown-cmark](https://pulldown-cmark.github.io/pulldown-cmark/index.html)
  - the image optimization with [image](https://github.com/image-rs/image)
  - the code block parsing with [Shiki](https://github.com/shikijs/shiki)
  - the RSS feed generation by hand and helped by actix
  - the animation with [Takumi](https://github.com/kane50613/takumi)
  - the graphs with Mermaid.js (maybe turn each graph into image with takumi for better performance and SEO or use [mermaid-rs-renderer](https://github.com/1jehuang/mermaid-rs-renderer))
- Data
  - The whole content of the website is etiher stored in markdown files or in a YAML file, the markdown files are used for the blog posts and the projects, and the YAML file is used for the different pages like the home page, the about page, etc... this way i can easily add new content without having to change the codebase.
  - The markdown file should be parsed have a separate build step to generate a tiny database like that would include all the handle done
  - Build command for that step:
    - `cargo run -p content --bin content-build`
    - Optional paths:
      - `--content-root <path>` (default: `contents`)
      - `--output-dir <path>` (default: `target/content-build`)
  - This should also allow to have search engine inside the website and to have a better user experience when navigating through the website.
  - The sidebar should also be generated based on the front matter of the markdown files
- docs:
  - Take into account the front_matter of the markdown file and add some other header like the creation date, last update date, release date (when the front matter param release become true) and the reading time.
- Hot Reloading:
  - I need the Data to be hot reloaded when i change the markdown files or the YAML file, this way i can see the changes in real time without having to restart the server.
- Testing:
  - I dont know which bees has stung me but i want to make sure that the code is fully tested and covered has much as possible.
  - So i need to do:
    - Unit Tests
      - Rust basic unit tests for the different functions and modules
    - End to End Tests
      - Tests that cover the whole flow of the website, like making sure that the home page has the correct amount of data, that the docs page is working correctly, that the blog page is working correctly, etc...
    - Performance Tests
      - Measure that the build time is not too long (build of the database and the build of the binary)
      - Measure that the loading time (not the rendering time) is not too long (the time it takes for the server to respond to a request and to send the data to the client)
      - Measure that the rendering time is not too long (the time it takes for the client to render the page after receiving the data from the server)
      - Measure the time it takes to go from the request to the page fully rendered and interactive (the time it takes for the user to see the page and to be able to interact with it)
    - Security Tests
      - Make sure that the website is not vulnerable to common attacks like XSS, CSRF, etc...
      - No major CVEs in the dependencies
    - SEO Tests
      - Make sure that the website is SEO friendly and that the different pages are indexed correctly by the search engines. The miniature of each page should be correct, the title and the description should be correct, the different tags should be correct, etc...
    - Accessibility Tests
      - Make sure that the website is accessible for everyone, that it follows the best practices for accessibility, that it has a good contrast ratio, that it has a good font size, that it has a good color scheme, etc...

## Pages Structure

- Home Page [MVP] : Will include a brief intro, my current situation, my projects and my blog posts
- More about me [MVP] : Will include a more detailed intro, my experience, my CV, my skills and my contact information and a timeline of my career and my life.
- Projects [MVP] : Will include my projects, each project will have its own page with a detailed description, the technologies used, the challenges faced and the solutions found.
- Blog [MVP] : Will include my blog posts, each blog post will have different categories, tags, subjects
- Docs [MVP] : Some projects will have a more detailed documentation, some waiting to be moved to their own website or just some knowledge base articles like
- Contact [P2]: Will include a contact form and my social media links.
- Useful Links [P2]: Will include some useful materials that doesn't belong in the other pages but has the potential to be useful for the visitors, like some tools, some libraries, some articles, some videos, etc...
- 404 Page [MVP] : Will include a custom 404 page with a nice design and a link to the home page.
- 418 Page [MVP] : Will include the i'm a teapot page that i love to add to each design
- Easter Eggs [P2]: Because Why not and i have a uge data base of ideas for easter eggs.
- Other pages that i will add later on like a page for my talks, a page for my open source contributions, a page for my certifications, etc...

## Themes

- I won't lie, i stand in between two minds, a fully Cyberpunk design (70%) and a more Steampunk one (30%), i will try to find a way to mix both of them together and create a unique design that represent me and my work.
- It should also be able to switch from French to English (if the page has already been translated)
- I loved my previous design, but it wasn't really me, it was more of a professional one that didn't help me convey my personality across my work.

---

## Home Page - Developer Guide

### Architecture

The home page is **100% client-hydrated**: the server delivers a lightweight loading skeleton, and the WASM bundle fetches content from `/api/v1/home` after hydration.

```
Server (SSR)              Client (WASM)
─────────────             ─────────────
Loading skeleton   →  hydrates  →  fetches /api/v1/home  →  renders content
```

### Editing `contents/home.yaml`

`home.yaml` is the single source of truth for home-page content.

| Field | Description |
|---|---|
| `name` | Full display name |
| `presentation` | Multi-line bio (FR) |
| `presentationEn` | Multi-line bio (EN, optional) |
| `shortDescription` | One-liner below the name (FR) |
| `shortDescriptionEn` | One-liner below the name (EN, optional) |
| `coverTitle` | List of rotating hero subtitles (FR) |
| `coverTitleEn` | List of rotating hero subtitles (EN, optional) |
| `cvUrl` | Path/URL of the downloadable CV |
| `url[]` | Social links (`name`, `url`, `primaire`, `imgUrl`) |
| `history[]` | Career/education timeline entries |

#### `history[]` entry fields

| Field | Description |
|---|---|
| `title` | Entry title (FR) |
| `titleEn` | Entry title (EN, optional) |
| `lieux` | Location |
| `date` | Date range (free text) |
| `weight` | Sort order (ascending) |
| `icoUrl` | Icon: `ico#school`, `ico#work`, `ico#handyman` |
| `imgUrl` | Logo image: `media#filename.png` |
| `description` | Multi-line description (FR) |
| `descriptionEn` | Multi-line description (EN, optional) |

### Adding / Updating Translations

UI strings (nav labels, buttons, loading text) live in:

```
apps/frontend/src/i18n/translations.rs
```

Each language is a `pub static` of type `Translations`. To add a new string:

1. Add a field to the `Translations` struct.
2. Fill in both `FR` and `EN` static instances.
3. Reference the field via `t.get().<field>` in a component.

### i18n Language Toggle

- The active language is stored as a `RwSignal<Language>` in the Leptos context.
- Pressing the language button calls `toggle_language(lang)` which:
  - Switches the signal between `Fr` and `En`.
  - Persists the choice to `localStorage` under the key `"lang"`.
- On page load the stored value is read, defaulting to `Fr` if absent.

### Running Locally

```bash
# Build content database
cargo run -p content --bin content-build

# Run the dev server (SSR + serve WASM)
cargo run --package frontend --features ssr

# Run unit tests
cargo test --package content --package api
```
