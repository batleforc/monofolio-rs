# Graph Report - monofolio-v2  (2026-11-27)

## Corpus Check
- 177 files · ~110,522 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: .toml 9, (none) 3, .graphify-bak 1)

## Summary
- 1833 nodes · 3424 edges · 172 communities (85 shown, 87 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 183 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `f2086d98`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- shiki.bundle.js
- miniature/mod.rs
- markdown.rs
- setParentDeep
- personal
- assets.rs
- nav.rs
- codeToTokens
- Language
- home.yaml
- tasks
- constructor
- pages/about.rs
- pages/home.rs
- toRegExpDetails
- J2
- build.rs
- projects.rs
- content.rs
- spec
- spec
- trace/src/lib.rs
- spec
- package.json
- steps
- stringifyEntities
- technologies.rs
- tests.rs
- public.rs
- ContentDatabase
- createShikiPrimitive
- content-watch.rs
- Monofolio V2 UI Design Guide
- _tokenize
- F
- frontend/src/lib.rs
- api/src/home.rs
- src/rss.rs
- monofolio
- loadLanguage
- render_heading
- main.rs
- seo.rs
- ContentDatabaseError
- Home Page - Developer Guide
- page.rs
- prelude
- deserialize
- docs.rs
- database/rss.rs
- check-workspace-deps.py
- markdown/mod.rs
- SocialIconType
- tracing
- Weebo GitOps
- on
- content_db.rs
- list-codeblock-langs.mjs
- collectExternalReferencesInRules
- resolveParsedThemeRules
- manifest.json
- Déploiement de Forgejo
- with
- github
- CharacterClassRange
- RenderMarkdownNode
- Goal
- build.yaml
- transformerDecorations
- render_code_block
- include
- updates
- api_v1_scope
- whitespace
- createColorPalette
- footnote.rs
- render_children
- build
- steps
- env
- monofolio-rs.md
- Idea
- Copilot instructions for this repository
- policies
- explainThemeScope
- render_blockquote
- render_inline_code
- render_container
- render_emphasis
- link.rs
- render_paragraph
- render_strong
- render_table
- render_text
- permissions
- allowBuilds
- getScopes
- CLAUDE.md
- 2.forgejo-kube-runner.md
- content-build.rs
- rust-ui
- dev-hot-reload.sh
- findNextMatchSync
- omission
- zwitch
- content_reload_sentinel.rs
- .claude/CLAUDE.md
- stream-07-06-2026.md
- poc-argocd-ovh.md
- MinIO.md
- MariaDB.md
- MongoDB.md
- PostgreSQL.md
- Kafka.md
- Vault.md
- ansible.md
- Authentik.md
- Keycloak.md
- Zitadel.md
- EclipseChe.md
- Forgejo.md
- Gitea.md
- Harbor.md
- Application/index.md
- Tuwunel.md
- Vaultwarden.md
- concourse.md
- github.md
- gitlab.md
- tekton.md
- argocd.md
- fluxcd.md
- helm.md
- Kube/index.md
- k3s.md
- microk8s.md
- Coroot.md
- Terraform.md
- Virtualisation/index.md
- kubevirt.md
- proxmox.md
- Netbird.md
- c++.md
- csharp.md
- golang.md
- java.md
- php.md
- python.md
- rust.md
- javascript.md
- react.md
- svelte.md
- vuejs.md
- Language/index.md
- Progicielle/index.md
- end-of-line.sh
- whitespace-fixer.sh
- api
- process-html.sh
- trace

## God Nodes (most connected - your core abstractions)
1. `ContentDatabaseError` - 54 edges
2. `ContentDatabase` - 47 edges
3. `use_language()` - 35 edges
4. `Language` - 33 edges
5. `ContentEntry` - 30 edges
6. `HomeData` - 23 edges
7. `RenderMarkdownNode()` - 22 edges
8. `build_content_database()` - 22 edges
9. `build_content_database_and_bundle()` - 21 edges
10. `ContentOutputBundle` - 20 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `api_v1_scope()`  [INFERRED]
  apps/frontend/src/main.rs → libs/api/src/lib.rs
- `main()` --references--> `RssFeed`  [INFERRED]
  apps/frontend/src/main.rs → libs/api/src/rss.rs
- `main()` --calls--> `shutdown_tracing()`  [INFERRED]
  apps/frontend/src/main.rs → libs/trace/src/lib.rs
- `main()` --calls--> `start_tracing()`  [INFERRED]
  apps/frontend/src/main.rs → libs/trace/src/lib.rs
- `from_content_db()` --references--> `ContentDatabase`  [INFERRED]
  apps/frontend/src/services/api.rs → libs/content/src/database/types.rs

## Import Cycles
- 1-file cycle: `apps/frontend/src/seo.rs -> apps/frontend/src/seo.rs`
- 1-file cycle: `apps/frontend/src/main.rs -> apps/frontend/src/main.rs`
- 1-file cycle: `libs/content/src/database/assets.rs -> libs/content/src/database/assets.rs`
- 2-file cycle: `apps/frontend/src/components/about.rs -> apps/frontend/src/pages/home.rs -> apps/frontend/src/components/about.rs`
- 2-file cycle: `apps/frontend/src/pages/home.rs -> apps/frontend/src/seo.rs -> apps/frontend/src/pages/home.rs`
- 2-file cycle: `apps/frontend/src/components/hero.rs -> apps/frontend/src/pages/home.rs -> apps/frontend/src/components/hero.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/blockquote.rs -> apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/blockquote.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/paragraph.rs -> apps/frontend/src/components/markdown/mod.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/strong.rs -> apps/frontend/src/components/markdown/mod.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/table.rs -> apps/frontend/src/components/markdown/mod.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/toc.rs -> apps/frontend/src/components/markdown/mod.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/container.rs -> apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/container.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/emphasis.rs -> apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/emphasis.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/footnote.rs -> apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/footnote.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/heading.rs -> apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/heading.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/link.rs -> apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/link.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/list.rs -> apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/list.rs`
- 2-file cycle: `libs/content/src/database/rss.rs -> libs/content/src/lib.rs -> libs/content/src/database/rss.rs`
- 2-file cycle: `libs/content/src/database/bundle.rs -> libs/content/src/database/mod.rs -> libs/content/src/database/bundle.rs`
- 2-file cycle: `libs/content/src/database/assets.rs -> libs/content/src/database/mod.rs -> libs/content/src/database/assets.rs`

## Communities (172 total, 87 thin omitted)

### Community 0 - "shiki.bundle.js"
Cohesion: 0.02
Nodes (45): adjustMatchDetailsForOffset(), all(), basename(), caseInsensitiveTransform(), caseSensitiveTransform(), collectPatterns(), compile(), compileAG() (+37 more)

### Community 1 - "miniature/mod.rs"
Cohesion: 0.07
Nodes (28): Node.js, BG, FG, INTER_FONT, main(), MUTED, parse_args(), parse_bg() (+20 more)

### Community 2 - "markdown.rs"
Cohesion: 0.08
Nodes (44): append_ast_node(), ast_node_from_tag(), AstFrame, Node, collect_headings(), container_node(), converts_media_handle_in_markdown_body(), draft_defaults_to_false() (+36 more)

### Community 3 - "setParentDeep"
Cohesion: 0.06
Nodes (48): A2(), AbsenceFunction(), areFlagsEqual(), Assertion(), Backreference(), canParticipateWithNode(), CapturingGroup(), CharacterSet() (+40 more)

### Community 4 - "personal"
Cohesion: 0.04
Nodes (46): certifications, date, issuer, name, education, degree, description, endDate (+38 more)

### Community 5 - "assets.rs"
Cohesion: 0.09
Nodes (24): collect_cv_refs_in_yaml(), collect_non_history_media_refs_in_home_yaml(), fetch_favicon(), fetch_remote_media(), guess_extension_from_https_url(), minia_reference_to_site_url(), node_text(), optimize_or_copy_media_file() (+16 more)

### Community 6 - "nav.rs"
Cohesion: 0.10
Nodes (31): apply_technology_item(), blog_nav_matches_timeline(), BlogSidebarEntry, build_blog_nav(), build_doc_sidebar(), build_projects_nav(), build_technology_mindmap(), doc_sidebar_item_from() (+23 more)

### Community 7 - "codeToTokens"
Cohesion: 0.09
Nodes (35): addClassToHast(), alignThemesTokenization(), applyColorReplacements(), codeToHast(), codeToHtml(), codeToTokens(), codeToTokensBase(), codeToTokensBase2() (+27 more)

### Community 8 - "Language"
Cohesion: 0.10
Nodes (22): filter_search_results(), load_search_index(), NavBar(), SearchEntryData, static_search_entries(), entry_description(), entry_title(), timeline_icon_type() (+14 more)

### Community 9 - "home.yaml"
Cohesion: 0.06
Nodes (33): contactAvailability, contactAvailabilityEn, contactEmail, contactLocation, coverTitle, coverTitleEn, currentWork, cvUrl (+25 more)

### Community 10 - "tasks"
Cohesion: 0.06
Nodes (33): cmds, desc, cmds, desc, cmds, cmds, desc, desc (+25 more)

### Community 11 - "constructor"
Cohesion: 0.07
Nodes (33): addLanguage(), _buildAnchorCache(), clone(), cloneArray(), cloneObj(), collectInjections(), _collectInjections(), constructor() (+25 more)

### Community 12 - "pages/about.rs"
Cohesion: 0.14
Nodes (13): resolve_icomoon_symbol(), social_icon_type(), SocialIconButton(), SocialIconType, Builtin, Gitea, Icomoon, Minia (+5 more)

### Community 13 - "pages/home.rs"
Cohesion: 0.14
Nodes (20): About(), DinoGame(), Hero(), ErrorScreen(), LoadingScreen(), Timeline(), use_language(), use_translations() (+12 more)

### Community 14 - "toRegExpDetails"
Cohesion: 0.08
Nodes (29): addParentProperties(), assertMaxInBounds(), atomic(), createCaptureMap(), createJavaScriptRegexEngine(), defaultJavaScriptRegexConstructor(), execUnescaped(), forEachUnescaped() (+21 more)

### Community 15 - "J2"
Cohesion: 0.10
Nodes (28): _2(), ae(), b2(), C2(), CharacterClass(), D2(), ee2(), F2() (+20 more)

### Community 16 - "build.rs"
Cohesion: 0.13
Nodes (19): SearchEntry, build_content_database(), collect_markdown_files(), git_history_dates(), handle_from_relative_path(), infer_kind(), is_technology_doc_handle(), now_unix() (+11 more)

### Community 17 - "projects.rs"
Cohesion: 0.11
Nodes (13): BlogEntryData, BlogReferencePage(), decode_query_component(), encode_query_component(), PROJECTS_PER_PAGE, ProjectsPage(), ProjectSummaryData, read_projects_query_from_url() (+5 more)

### Community 18 - "content.rs"
Cohesion: 0.14
Nodes (19): collect_active_path_folders(), ContentHandlePage(), DocSidebarItemData, find_doc_href_for_techno(), flatten_doc_index(), HeaderImageSource, Icomoon, Image (+11 more)

### Community 19 - "spec"
Cohesion: 0.08
Nodes (26): actions, default, autodiscovery, actionid, crawlers, scmid, dockerfile, kind (+18 more)

### Community 20 - "spec"
Cohesion: 0.08
Nodes (26): actions, forgejo, autodiscovery, actionid, crawlers, groupby, scmid, cargo (+18 more)

### Community 21 - "trace/src/lib.rs"
Cohesion: 0.12
Nodes (7): Context, get_resource(), init_metrics(), init_traces(), shutdown_tracing(), start_tracing(), TracingOutput

### Community 22 - "spec"
Cohesion: 0.08
Nodes (25): actions, default, autodiscovery, actionid, crawlers, scmid, npm, kind (+17 more)

### Community 23 - "package.json"
Cohesion: 0.10
Nodes (19): LANGS, lineNumberTransformer, dependencies, tailwindcss@^4.2.2, @tailwindcss/cli@^4.2.2, tw-animate-css@^1.4.0, devDependencies, esbuild@^0.28.0 (+11 more)

### Community 24 - "steps"
Cohesion: 0.08
Nodes (23): GITHUB_TOKEN, jobs, updatecli, name, on, push, schedule, workflow_dispatch (+15 more)

### Community 25 - "stringifyEntities"
Cohesion: 0.09
Nodes (21): camelcase(), ccount(), charactersToExpression(), charactersToExpressionCached(), comment(), encode(), core(), create() (+13 more)

### Community 26 - "technologies.rs"
Cohesion: 0.19
Nodes (17): branch_has_only_technology_children(), branch_technology_count(), count_visible_technologies(), decode_query_component(), encode_query_component(), filter_technology_tree(), node_matches_query(), read_search_query_from_url() (+9 more)

### Community 27 - "tests.rs"
Cohesion: 0.19
Nodes (19): process_home_yaml_media_for_bundle(), build_content_database_and_prepare_bundle(), builds_database_sidebar_and_timeline(), builds_technology_mindmap_entries_from_docs(), creates_output_bundle_with_required_files(), draft_blog_posts_excluded_from_timeline(), full_workflow_helper_builds_and_finalizes_bundle(), index_files_generate_parent_handle() (+11 more)

### Community 28 - "public.rs"
Cohesion: 0.21
Nodes (12): main(), absolute_path_returns_404(), absolute_path_with_drive_prefix_returns_404(), directory_target_returns_404(), double_encoded_parent_dir_return_404(), encoded_parent_dir_segments_return_404(), get_public_file(), parent_dir_segments_return_404() (+4 more)

### Community 29 - "ContentDatabase"
Cohesion: 0.31
Nodes (17): process_home_yaml_minia_for_bundle(), process_mermaid_codeblocks_for_bundle(), write_content_database_json(), build_content_database_and_bundle(), create_content_output_bundle(), finalize_content_output_bundle(), prepare_content_output_bundle(), build_miniature_context() (+9 more)

### Community 30 - "createShikiPrimitive"
Cohesion: 0.18
Nodes (19): createShikiPrimitive(), ensureNotDisposed(), getLanguage(), getLoadedLanguages(), getLoadedThemes(), getTheme(), loadLanguage(), loadLanguageSync() (+11 more)

### Community 31 - "content-watch.rs"
Cohesion: 0.18
Nodes (11): event_paths(), is_generated_path(), keep_relevant_paths(), main(), parse_args(), run_command(), run_rebuild(), run_shiki_pipeline() (+3 more)

### Community 32 - "Monofolio V2 UI Design Guide"
Cohesion: 0.11
Nodes (18): Accessibility, Avoid, Buttons and actions, Component rules, Core theme tokens, Example guidelines, Future proofing, Interaction and motion (+10 more)

### Community 33 - "_tokenize"
Cohesion: 0.14
Nodes (18): _checkWhileConditions(), createOnigString(), disposeOnigString(), getFindOptions(), getRule(), handleCaptures(), matchInjections(), matchRule() (+10 more)

### Community 34 - "F"
Cohesion: 0.18
Nodes (17): A(), d(), E(), ee(), F(), j(), K(), M() (+9 more)

### Community 35 - "frontend/src/lib.rs"
Cohesion: 0.12
Nodes (7): App(), _CONTENT_RELOAD_SENTINEL, ContactPage(), load_home_data(), increment_not_found_counter(), NotFoundPage(), TeapotPage()

### Community 36 - "api/src/home.rs"
Cohesion: 0.26
Nodes (10): get_home(), get_home_returns_json(), HistoryEntryResponse, home_response_from_config(), HomeResponse, sample_config(), SocialLinkResponse, HistoryEntry (+2 more)

### Community 37 - "src/rss.rs"
Cohesion: 0.20
Nodes (5): health(), public_root_not_found(), get_rss(), get_rss_returns_xml_content_type(), RssFeed

### Community 38 - "monofolio"
Cohesion: 0.12
Nodes (15): context, dockerfile, CONTENT_DB_PATH, LEPTOS_SITE_ADDR, MEDIA_PATH, TRACKING_SCRIPT_SRC, TRACKING_SITE_ID, build (+7 more)

### Community 39 - "loadLanguage"
Cohesion: 0.14
Nodes (14): addGrammar(), resolveLangAlias$1(), _doLoadSingleGrammar(), getGrammar(), _grammarForScopeName(), loadGrammar(), _loadGrammar(), loadGrammarWithConfiguration() (+6 more)

### Community 40 - "render_heading"
Cohesion: 0.26
Nodes (7): render_heading(), extract_text(), heading_anchor_id(), slugify(), extract_headings(), TableOfContents(), TocEntry

### Community 41 - "main.rs"
Cohesion: 0.20
Nodes (6): build_content_security_policy(), build_sitemap_xml(), get_sitemap(), sitemap_contains_static_and_content_urls(), escape_xml(), SITE_URL

### Community 42 - "seo.rs"
Cohesion: 0.20
Nodes (7): build_person_json_ld(), canonical_url(), canonical_url_handles_root_and_paths(), DEFAULT_OG_IMAGE, person_json_ld_contains_required_fields(), PersonJsonLd(), StaticPageSeo()

### Community 43 - "ContentDatabaseError"
Cohesion: 0.14
Nodes (14): ContentDatabaseError, CopyFile, CreateDir, DecodeImage, EncodeImage, FetchFavicon, InvalidContentPath, MermaidRender (+6 more)

### Community 44 - "Home Page - Developer Guide"
Cohesion: 0.14
Nodes (13): Adding / Updating Translations, Architecture, Development Hot Reload, Editing `contents/home.yaml`, Goals, `history[]` entry fields, Home Page - Developer Guide, i18n Language Toggle (+5 more)

### Community 45 - "page.rs"
Cohesion: 0.24
Nodes (7): ApiErrorResponse, get_page(), get_page_returns_200_when_found(), get_page_returns_404_when_missing(), page_response_from_entry(), PageResponse, sample_database()

### Community 48 - "docs.rs"
Cohesion: 0.31
Nodes (6): collect_active_path_folders(), collect_folder_with_index_sections(), collect_index_entries(), DocSidebarItemData, DocsReferencePage(), render_doc_tree()

### Community 49 - "database/rss.rs"
Cohesion: 0.40
Nodes (9): blog_link_from_handle(), bundle_at(), escapes_xml_sensitive_characters_in_rss_fields(), rfc822_date(), rfc822_date_handles_iso_variants(), temporary_directory(), write_blog_rss_feed(), writes_rss_file_even_when_timeline_is_empty() (+1 more)

### Community 50 - "check-workspace-deps.py"
Cohesion: 0.29
Nodes (5): collect_workspace_dependency_uses(), dep_keys_with_workspace_true(), main(), read_toml(), resolve_member_manifests()

### Community 51 - "markdown/mod.rs"
Cohesion: 0.33
Nodes (6): MarkdownContent, MarkdownFromValue(), MarkdownHashScroll(), MarkdownNode, MarkdownRenderer(), MarkdownWithToc()

### Community 52 - "SocialIconType"
Cohesion: 0.29
Nodes (9): resolve_icomoon_symbol(), social_icon_type(), SocialIconType, Builtin, Gitea, Icomoon, Media, Minia (+1 more)

### Community 53 - "tracing"
Cohesion: 0.20
Nodes (9): endpoint, env, mediaPath, port, tracing, additional, kind, level (+1 more)

### Community 54 - "Weebo GitOps"
Cohesion: 0.20
Nodes (9): Cloud Development Environment, Conclusion, Weebo 1.0 - Weebo 2.0 - 2018 => 2023, Weebo 3.0, Weebo 3.1, Weebo 3 - février 2021 => mai 2026, Weebo 4 - 1er Avril 2025 => Still going, Weebo 5 - 1er Avril 2026 => Still Going (+1 more)

### Community 55 - "on"
Cohesion: 0.20
Nodes (10): on, pull_request, push, schedule, workflow_dispatch, branches, paths, branches (+2 more)

### Community 56 - "content_db.rs"
Cohesion: 0.29
Nodes (4): ContentDatabaseStatusResponse, get_content_database_status(), get_content_database_status_returns_json(), sample_database()

### Community 57 - "list-codeblock-langs.mjs"
Cohesion: 0.20
Nodes (4): BASE_LANGS, collectMarkdownFiles(), LANG_MAP, SHIKI_ENTRY_PATH

### Community 58 - "collectExternalReferencesInRules"
Cohesion: 0.25
Nodes (9): collectExternalReferencesInRules(), collectExternalReferencesInTopLevelRepositoryRule(), collectExternalReferencesInTopLevelRule(), collectReferencesOfReference(), _compilePatterns(), getCompiledRuleId(), mergeObjects(), parseInclude() (+1 more)

### Community 59 - "resolveParsedThemeRules"
Cohesion: 0.25
Nodes (9): createFromParsedTheme(), createFromRawTheme(), _doInsertHere(), insert(), isValidHexColor(), parseTheme(), resolveParsedThemeRules(), strArrCmp() (+1 more)

### Community 60 - "manifest.json"
Cohesion: 0.22
Nodes (8): background_color, display, icons, name, protocol_handlers, short_name, start_url, theme_color

### Community 61 - "Déploiement de Forgejo"
Cohesion: 0.22
Nodes (8): Choix de la forge Git, Conclusion, Déploiement de Forgejo, Installation de Forgejo, Liens utiles, Note, Préparation de forgejo pour GitOps, Prérequis

### Community 62 - "with"
Cohesion: 0.22
Nodes (9): with, caCertificate, fetch-depth, jwtGithubAudience, method, path, role, secrets (+1 more)

### Community 63 - "github"
Cohesion: 0.22
Nodes (8): github, email, owner, repository, test, token, user, username

### Community 64 - "CharacterClassRange"
Cohesion: 0.32
Nodes (8): Character(), CharacterClassRange(), charHasCase(), getCasesOutsideCharClassRange(), getCharEscape(), getCodePointRangesFromChars(), getIgnoreCaseMatchChars(), isDigitCharCode()

### Community 65 - "RenderMarkdownNode"
Cohesion: 0.32
Nodes (4): render_hard_break(), render_soft_break(), RenderMarkdownNode(), render_rule()

### Community 66 - "Goal"
Cohesion: 0.25
Nodes (7): Deploy flow, Deployment flow, Github Events to listen, Goal, Tasks, Test, Test flow

### Community 67 - "build.yaml"
Cohesion: 0.25
Nodes (7): concurrency, cancel-in-progress, group, env, REGISTRY, jobs, name

### Community 68 - "transformerDecorations"
Cohesion: 0.38
Nodes (7): stringify3(), transformerDecorations(), applyDecoration(), applyLine(), applyLineSection(), getContext(), verifyIntersections()

### Community 69 - "render_code_block"
Cohesion: 0.52
Nodes (4): append_code_text(), clean_language(), extract_code_title(), render_code_block()

### Community 70 - "include"
Cohesion: 0.29
Nodes (7): strategy, context, dockerfile, image_name, name, include, matrix

### Community 71 - "updates"
Cohesion: 0.29
Nodes (6): interval, updates, directory, package-ecosystem, schedule, version

### Community 73 - "whitespace"
Cohesion: 0.33
Nodes (6): body2(), empty(), headOrColgroupOrCaption(), siblings(), sibling(), whitespace()

### Community 74 - "createColorPalette"
Cohesion: 0.73
Nodes (6): createColorPalette(), getColorTable(), namedColor(), rgbColor(), tableColor(), value()

### Community 75 - "footnote.rs"
Cohesion: 0.67
Nodes (3): FootnoteSection(), render_footnote_definition(), render_footnote_reference()

### Community 76 - "render_children"
Cohesion: 0.60
Nodes (3): render_list(), render_list_item(), render_children()

### Community 77 - "build"
Cohesion: 0.33
Nodes (6): enable-openid-connect, name, outputs, runs-on, build, image_backend

### Community 78 - "steps"
Cohesion: 0.33
Nodes (6): steps, id, if, name, run, uses

### Community 79 - "env"
Cohesion: 0.33
Nodes (6): ARGOCD_AUTH_TOKEN, ARGOCD_OPTS, ARGOCD_SERVER, ARGOCD_SHA256, ARGOCD_VERSION, env

### Community 80 - "monofolio-rs.md"
Cohesion: 0.40
Nodes (4): Architecture, Conclusion, Content, Processing du contenu

### Community 81 - "Idea"
Cohesion: 0.40
Nodes (4): Blog / Doc / Project, Easter Egg, Home, Idea

### Community 82 - "Copilot instructions for this repository"
Cohesion: 0.40
Nodes (4): Build, test, and lint commands, Copilot instructions for this repository, High-level architecture, Key conventions

### Community 83 - "policies"
Cohesion: 0.40
Nodes (4): policies, config, name, values

### Community 84 - "explainThemeScope"
Cohesion: 0.50
Nodes (4): explainThemeScope(), explainThemeScopesFull(), matches(), matchesOne()

### Community 94 - "permissions"
Cohesion: 0.50
Nodes (4): permissions, permissions, contents, id-token

### Community 95 - "allowBuilds"
Cohesion: 0.50
Nodes (3): allowBuilds, esbuild, @parcel/watcher

### Community 96 - "getScopes"
Cohesion: 0.67
Nodes (3): getScopes(), pushScope(), toJSON()

## Knowledge Gaps
- **30 isolated node(s):** `ui`, `Gitea`, `Icomoon`, `Fr`, `En` (+25 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 714 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **87 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `ContentDatabase` connect `ContentDatabase` to `miniature/mod.rs`, `assets.rs`, `nav.rs`, `main.rs`, `page.rs`, `build.rs`, `projects.rs`, `content.rs`, `database/rss.rs`, `content_db.rs`, `tests.rs`?**
  _High betweenness centrality (0.021) - this node is a cross-community bridge._
- **Are the 16 inferred relationships involving `use_language()` (e.g. with `About()` and `DinoGame()`) actually correct?**
  _`use_language()` has 16 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ui`, `Gitea`, `Icomoon` to the rest of the system?**
  _30 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `shiki.bundle.js` be split into smaller, more focused modules?**
  _Cohesion score 0.015894039735099338 - nodes in this community are weakly interconnected._
- **Why does `Language` connect `Language` to `pages/about.rs`, `pages/home.rs`, `docs.rs`, `projects.rs`, `content.rs`, `technologies.rs`?**
  _High betweenness centrality (0.016) - this node is a cross-community bridge._
- **Should `miniature/mod.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06892230576441102 - nodes in this community are weakly interconnected._
- **Why does `ContentEntry` connect `build.rs` to `miniature/mod.rs`, `markdown.rs`, `nav.rs`, `main.rs`, `page.rs`, `pages/home.rs`, `content.rs`, `ContentDatabase`?**
  _High betweenness centrality (0.015) - this node is a cross-community bridge._
