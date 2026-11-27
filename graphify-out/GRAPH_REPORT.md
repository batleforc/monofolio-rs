# Graph Report - monofolio-v2  (2026-11-27)

## Corpus Check
- 182 files · ~112,504 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: .toml 9, (none) 3, .graphify-bak 1)

## Summary
- 1908 nodes · 3529 edges · 177 communities (85 shown, 92 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 188 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `6c1387cf`
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
- createShikiPrimitive
- Language
- home.yaml
- tasks
- constructor
- use_language
- pages/about.rs
- toRegExpDetails
- J2
- build.rs
- docs.rs
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
- content-watch.rs
- ContentDatabase
- tasks
- projects.rs
- Monofolio V2 UI Design Guide
- _tokenize
- prelude
- F
- api/src/home.rs
- content_db.rs
- monofolio
- loadLanguage
- render_heading
- codeToHast
- frontend/src/lib.rs
- pages/home.rs
- Home Page - Developer Guide
- page.rs
- render_html
- deserialize
- database/rss.rs
- check-workspace-deps.py
- markdown/mod.rs
- pages/about.rs
- tracing
- Weebo GitOps
- on
- compileAG
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
- DocSidebarItemKind
- render_code_block
- include
- updates
- whitespace
- knip.json
- footnote.rs
- content_db.rs
- build
- steps
- env
- monofolio-rs.md
- Idea
- Copilot instructions for this repository
- policies
- findNextMatchSync
- render_blockquote
- render_inline_code
- render_container
- render_emphasis
- omission
- render_paragraph
- render_strong
- render_table
- render_text
- permissions
- allowBuilds
- getScopes
- CLAUDE.md
- 2.forgejo-kube-runner.md
- all
- rust-ui
- dev-hot-reload.sh
- 2026-11-27T19-46-28Z__apps-frontend.md
- render_image
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
- basename
- 2026-11-27T17-04-07Z__apps-frontend.md
- render_children
- link.rs
- ContentEntry

## God Nodes (most connected - your core abstractions)
1. `ContentDatabaseError` - 54 edges
2. `ContentDatabase` - 47 edges
3. `use_language()` - 35 edges
4. `Language` - 33 edges
5. `ContentEntry` - 30 edges
6. `HomeData` - 23 edges
7. `use_translations()` - 23 edges
8. `build_content_database()` - 22 edges
9. `RenderMarkdownNode()` - 22 edges
10. `build_content_database_and_bundle()` - 21 edges

## Surprising Connections (you probably didn't know these)
- `from_content_db()` --references--> `ContentDatabase`  [INFERRED]
  apps/frontend/src/services/api.rs → libs/content/src/database/types.rs
- `main()` --references--> `RssFeed`  [INFERRED]
  apps/frontend/src/main.rs → libs/api/src/rss.rs
- `HomeData` --references--> `HomeConfig`  [EXTRACTED]
  apps/frontend/src/pages/home.rs → libs/content/src/home.rs
- `PageData` --references--> `ContentEntry`  [EXTRACTED]
  apps/frontend/src/pages/content.rs → libs/content/src/database/types.rs
- `ProjectSummaryData` --references--> `ContentEntry`  [EXTRACTED]
  apps/frontend/src/pages/home.rs → libs/content/src/database/types.rs

## Import Cycles
- 1-file cycle: `apps/frontend/src/seo.rs -> apps/frontend/src/seo.rs`
- 1-file cycle: `apps/frontend/src/main.rs -> apps/frontend/src/main.rs`
- 1-file cycle: `libs/content/src/database/assets.rs -> libs/content/src/database/assets.rs`
- 1-file cycle: `apps/frontend/src/components/navbar.rs -> apps/frontend/src/components/navbar.rs`
- 2-file cycle: `libs/content/src/database/bundle.rs -> libs/content/src/database/mod.rs -> libs/content/src/database/bundle.rs`
- 2-file cycle: `apps/frontend/src/components/about.rs -> apps/frontend/src/pages/home.rs -> apps/frontend/src/components/about.rs`
- 2-file cycle: `apps/frontend/src/pages/home.rs -> apps/frontend/src/seo.rs -> apps/frontend/src/pages/home.rs`
- 2-file cycle: `apps/frontend/src/components/hero.rs -> apps/frontend/src/pages/home.rs -> apps/frontend/src/components/hero.rs`
- 2-file cycle: `apps/frontend/src/components/timeline.rs -> apps/frontend/src/pages/home.rs -> apps/frontend/src/components/timeline.rs`
- 2-file cycle: `libs/content/src/database/build.rs -> libs/content/src/database/mod.rs -> libs/content/src/database/build.rs`
- 2-file cycle: `libs/content/src/database/rss.rs -> libs/content/src/lib.rs -> libs/content/src/database/rss.rs`
- 2-file cycle: `libs/content/src/database/assets.rs -> libs/content/src/database/mod.rs -> libs/content/src/database/assets.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/toc.rs -> apps/frontend/src/components/markdown/mod.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/paragraph.rs -> apps/frontend/src/components/markdown/mod.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/strong.rs -> apps/frontend/src/components/markdown/mod.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/table.rs -> apps/frontend/src/components/markdown/mod.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/list.rs -> apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/list.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/blockquote.rs -> apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/blockquote.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/container.rs -> apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/container.rs`
- 2-file cycle: `apps/frontend/src/components/markdown/emphasis.rs -> apps/frontend/src/components/markdown/mod.rs -> apps/frontend/src/components/markdown/emphasis.rs`

## Communities (177 total, 92 thin omitted)

### Community 0 - "shiki.bundle.js"
Cohesion: 0.02
Nodes (39): adjustMatchDetailsForOffset(), caseInsensitiveTransform(), caseSensitiveTransform(), compileWhile(), compileWhileAG(), createGrammar(), _equals(), exec() (+31 more)

### Community 1 - "miniature/mod.rs"
Cohesion: 0.06
Nodes (29): Node.js, BG, FG, INTER_FONT, main(), MUTED, parse_args(), parse_bg() (+21 more)

### Community 2 - "markdown.rs"
Cohesion: 0.08
Nodes (42): append_ast_node(), ast_node_from_tag(), AstFrame, Node, collect_headings(), container_node(), converts_media_handle_in_markdown_body(), draft_defaults_to_false() (+34 more)

### Community 3 - "setParentDeep"
Cohesion: 0.07
Nodes (37): A2(), addParentProperties(), areFlagsEqual(), Backreference(), canParticipateWithNode(), CapturingGroup(), cloneCapturingGroup(), createCaptureMap() (+29 more)

### Community 4 - "personal"
Cohesion: 0.04
Nodes (46): certifications, date, issuer, name, education, degree, description, endDate (+38 more)

### Community 5 - "assets.rs"
Cohesion: 0.07
Nodes (39): collect_cv_refs_in_yaml(), collect_non_history_media_refs_in_home_yaml(), fetch_favicon(), fetch_remote_media(), guess_extension_from_https_url(), minia_reference_to_site_url(), node_text(), optimize_or_copy_media_file() (+31 more)

### Community 6 - "nav.rs"
Cohesion: 0.12
Nodes (27): apply_technology_item(), blog_nav_matches_timeline(), BlogSidebarEntry, build_blog_nav(), build_doc_sidebar(), build_projects_nav(), build_technology_mindmap(), doc_sidebar_item_from() (+19 more)

### Community 7 - "createShikiPrimitive"
Cohesion: 0.06
Nodes (55): alignThemesTokenization(), applyColorReplacements(), codeToTokens(), codeToTokensBase(), codeToTokensBase2(), codeToTokensWithThemes(), createAnsiSequenceParser(), createColorPalette() (+47 more)

### Community 8 - "Language"
Cohesion: 0.11
Nodes (23): DinoGame(), filter_search_results(), load_search_index(), nav_active_state(), NAV_ITEMS, nav_matches(), NavBar(), NavItem (+15 more)

### Community 9 - "home.yaml"
Cohesion: 0.06
Nodes (33): contactAvailability, contactAvailabilityEn, contactEmail, contactLocation, coverTitle, coverTitleEn, currentWork, cvUrl (+25 more)

### Community 10 - "tasks"
Cohesion: 0.05
Nodes (37): cmds, desc, cmds, desc, flatten, taskfile, cmds, cmds (+29 more)

### Community 11 - "constructor"
Cohesion: 0.07
Nodes (33): addLanguage(), _buildAnchorCache(), clone(), cloneArray(), cloneObj(), collectInjections(), _collectInjections(), constructor() (+25 more)

### Community 12 - "use_language"
Cohesion: 0.13
Nodes (8): SiteFooter(), ErrorScreen(), LoadingScreen(), use_translations(), ContactPage(), load_home_data(), increment_not_found_counter(), NotFoundPage()

### Community 13 - "pages/about.rs"
Cohesion: 0.15
Nodes (19): Hero(), resolve_icomoon_symbol(), social_icon_type(), SocialIconButton(), SocialIconType, Bluesky, Builtin, Gitea (+11 more)

### Community 14 - "toRegExpDetails"
Cohesion: 0.09
Nodes (26): assertMaxInBounds(), atomic(), createJavaScriptRegexEngine(), defaultJavaScriptRegexConstructor(), execUnescaped(), forEachUnescaped(), generate(), gen() (+18 more)

### Community 15 - "J2"
Cohesion: 0.07
Nodes (39): _2(), AbsenceFunction(), ae(), Assertion(), b2(), C2(), CharacterClass(), CharacterSet() (+31 more)

### Community 16 - "build.rs"
Cohesion: 0.28
Nodes (14): build_content_database(), collect_markdown_files(), git_history_dates(), handle_from_relative_path(), infer_kind(), is_technology_doc_handle(), now_unix(), segment_to_title() (+6 more)

### Community 17 - "docs.rs"
Cohesion: 0.16
Nodes (10): collect_active_path_folders(), collect_folder_with_index_sections(), collect_index_entries(), DocSidebarItemData, DocsReferencePage(), render_doc_tree(), fetch_json(), fetch_json_send_safe() (+2 more)

### Community 18 - "content.rs"
Cohesion: 0.09
Nodes (25): collect_active_path_folders(), ContentHandlePage(), DocSidebarItemData, find_doc_href_for_techno(), flatten_doc_index(), HeaderImageSource, Icomoon, Image (+17 more)

### Community 19 - "spec"
Cohesion: 0.08
Nodes (26): actions, default, autodiscovery, actionid, crawlers, scmid, dockerfile, kind (+18 more)

### Community 20 - "spec"
Cohesion: 0.08
Nodes (26): actions, forgejo, autodiscovery, actionid, crawlers, groupby, scmid, cargo (+18 more)

### Community 21 - "trace/src/lib.rs"
Cohesion: 0.06
Nodes (22): build_content_security_policy(), main(), ApiDoc, api_v1_scope(), absolute_path_returns_404(), absolute_path_with_drive_prefix_returns_404(), directory_target_returns_404(), double_encoded_parent_dir_return_404() (+14 more)

### Community 22 - "spec"
Cohesion: 0.08
Nodes (25): actions, default, autodiscovery, actionid, crawlers, scmid, npm, kind (+17 more)

### Community 23 - "package.json"
Cohesion: 0.10
Nodes (23): LANGS, lineNumberTransformer, dependencies, tailwindcss@^4.3.5, @tailwindcss/cli@^4.3.5, devDependencies, esbuild@^0.28.2, knip@^6.49.0 (+15 more)

### Community 24 - "steps"
Cohesion: 0.08
Nodes (23): GITHUB_TOKEN, jobs, updatecli, name, on, push, schedule, workflow_dispatch (+15 more)

### Community 25 - "stringifyEntities"
Cohesion: 0.09
Nodes (21): camelcase(), ccount(), charactersToExpression(), charactersToExpressionCached(), comment(), encode(), core(), create() (+13 more)

### Community 26 - "technologies.rs"
Cohesion: 0.20
Nodes (17): branch_has_only_technology_children(), branch_technology_count(), count_visible_technologies(), decode_query_component(), encode_query_component(), filter_technology_tree(), node_matches_query(), read_search_query_from_url() (+9 more)

### Community 27 - "tests.rs"
Cohesion: 0.18
Nodes (19): process_home_yaml_media_for_bundle(), build_content_database_and_prepare_bundle(), builds_database_sidebar_and_timeline(), builds_technology_mindmap_entries_from_docs(), creates_output_bundle_with_required_files(), draft_blog_posts_excluded_from_timeline(), full_workflow_helper_builds_and_finalizes_bundle(), index_files_generate_parent_handle() (+11 more)

### Community 28 - "content-watch.rs"
Cohesion: 0.18
Nodes (11): event_paths(), is_generated_path(), keep_relevant_paths(), main(), parse_args(), run_command(), run_rebuild(), run_shiki_pipeline() (+3 more)

### Community 29 - "ContentDatabase"
Cohesion: 0.34
Nodes (17): process_home_yaml_minia_for_bundle(), process_mermaid_codeblocks_for_bundle(), write_content_database_json(), build_content_database_and_bundle(), create_content_output_bundle(), finalize_content_output_bundle(), prepare_content_output_bundle(), build_miniature_context() (+9 more)

### Community 30 - "tasks"
Cohesion: 0.05
Nodes (43): cmd, defer, ignore_error, task, vars, cmds, cmds, desc (+35 more)

### Community 31 - "projects.rs"
Cohesion: 0.15
Nodes (10): format_display_date(), BlogEntryData, decode_query_component(), encode_query_component(), PROJECTS_PER_PAGE, ProjectsPage(), ProjectSummaryData, read_projects_query_from_url() (+2 more)

### Community 32 - "Monofolio V2 UI Design Guide"
Cohesion: 0.10
Nodes (19): Accessibility, Avoid, Buttons and actions, Component rules, Core theme tokens, Example guidelines, Future proofing, Interaction and motion (+11 more)

### Community 33 - "_tokenize"
Cohesion: 0.14
Nodes (18): _checkWhileConditions(), createOnigString(), disposeOnigString(), getFindOptions(), getRule(), handleCaptures(), matchInjections(), matchRule() (+10 more)

### Community 34 - "prelude"
Cohesion: 0.21
Nodes (5): About(), entry_description(), entry_title(), timeline_icon_type(), HistoryEntryData

### Community 35 - "F"
Cohesion: 0.14
Nodes (20): A(), d(), E(), ee(), ee2(), F(), H(), j() (+12 more)

### Community 36 - "api/src/home.rs"
Cohesion: 0.19
Nodes (10): get_home(), get_home_returns_json(), HistoryEntryResponse, home_response_from_config(), HomeResponse, sample_config(), SocialLinkResponse, HistoryEntry (+2 more)

### Community 37 - "content_db.rs"
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

### Community 41 - "codeToHast"
Cohesion: 0.23
Nodes (12): addClassToHast(), codeToHast(), codeToHtml(), flatTokenVariants(), getTokenStyleObject(), getTransformers(), mergeAdjacentStyledTokens(), mergeWhitespaceTokens() (+4 more)

### Community 42 - "frontend/src/lib.rs"
Cohesion: 0.15
Nodes (4): App(), _CONTENT_RELOAD_SENTINEL, BlogReferencePage(), TeapotPage()

### Community 43 - "pages/home.rs"
Cohesion: 0.24
Nodes (11): HomeContent(), HomePage(), LAST_PROJECTS_COUNT, LatestProjectsSection(), load_home_data(), load_home_data_send_safe(), load_projects_data(), load_projects_data_send_safe() (+3 more)

### Community 44 - "Home Page - Developer Guide"
Cohesion: 0.14
Nodes (13): Adding / Updating Translations, Architecture, Development Hot Reload, Editing `contents/home.yaml`, Goals, `history[]` entry fields, Home Page - Developer Guide, i18n Language Toggle (+5 more)

### Community 45 - "page.rs"
Cohesion: 0.20
Nodes (7): ApiErrorResponse, get_page(), get_page_returns_200_when_found(), get_page_returns_404_when_missing(), page_response_from_entry(), PageResponse, sample_database()

### Community 49 - "database/rss.rs"
Cohesion: 0.17
Nodes (14): build_sitemap_xml(), get_sitemap(), sitemap_contains_static_and_content_urls(), blog_link_from_handle(), bundle_at(), escape_xml(), escapes_xml_sensitive_characters_in_rss_fields(), rfc822_date() (+6 more)

### Community 50 - "check-workspace-deps.py"
Cohesion: 0.29
Nodes (5): collect_workspace_dependency_uses(), dep_keys_with_workspace_true(), main(), read_toml(), resolve_member_manifests()

### Community 51 - "markdown/mod.rs"
Cohesion: 0.33
Nodes (6): MarkdownContent, MarkdownFromValue(), MarkdownHashScroll(), MarkdownNode, MarkdownRenderer(), MarkdownWithToc()

### Community 52 - "pages/about.rs"
Cohesion: 0.25
Nodes (10): resolve_icomoon_symbol(), social_icon_type(), SocialIconType, Bluesky, Builtin, Gitea, Icomoon, Media (+2 more)

### Community 53 - "tracing"
Cohesion: 0.20
Nodes (9): endpoint, env, mediaPath, port, tracing, additional, kind, level (+1 more)

### Community 54 - "Weebo GitOps"
Cohesion: 0.20
Nodes (9): Cloud Development Environment, Conclusion, Weebo 1.0 - Weebo 2.0 - 2018 => 2023, Weebo 3.0, Weebo 3.1, Weebo 3 - février 2021 => mai 2026, Weebo 4 - 1er Avril 2025 => Still going, Weebo 5 - 1er Avril 2026 => Still Going (+1 more)

### Community 55 - "on"
Cohesion: 0.20
Nodes (10): on, pull_request, push, schedule, workflow_dispatch, branches, paths, branches (+2 more)

### Community 56 - "compileAG"
Cohesion: 0.50
Nodes (5): collectPatterns(), compile(), compileAG(), _getCachedCompiledPatterns(), _resolveAnchors()

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

### Community 68 - "DocSidebarItemKind"
Cohesion: 0.50
Nodes (4): DocSidebarItemKind, Folder, FolderWithIndex, Markdown

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

### Community 75 - "footnote.rs"
Cohesion: 0.67
Nodes (3): FootnoteSection(), render_footnote_definition(), render_footnote_reference()

### Community 76 - "content_db.rs"
Cohesion: 0.33
Nodes (4): ContentDatabaseStatusResponse, get_content_database_status(), get_content_database_status_returns_json(), sample_database()

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

### Community 94 - "permissions"
Cohesion: 0.50
Nodes (4): permissions, permissions, contents, id-token

### Community 95 - "allowBuilds"
Cohesion: 0.50
Nodes (3): allowBuilds, esbuild, @parcel/watcher

### Community 96 - "getScopes"
Cohesion: 0.67
Nodes (3): getScopes(), pushScope(), toJSON()

### Community 99 - "all"
Cohesion: 1.00
Nodes (3): all(), one(), toHtml()

### Community 178 - "render_children"
Cohesion: 0.60
Nodes (3): render_list(), render_list_item(), render_children()

### Community 180 - "ContentEntry"
Cohesion: 0.17
Nodes (8): SearchEntry, main(), parse_args(), doc_section_label(), generate_miniature_for_entry(), ContentEntry, MarkdownDocument, MarkdownHeading

## Knowledge Gaps
- **32 isolated node(s):** `ui`, `Bluesky`, `Gitea`, `Icomoon`, `api` (+27 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 746 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **92 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `ContentDatabase` connect `ContentDatabase` to `miniature/mod.rs`, `assets.rs`, `nav.rs`, `content_db.rs`, `page.rs`, `build.rs`, `database/rss.rs`, `content.rs`, `docs.rs`, `ContentEntry`, `tests.rs`?**
  _High betweenness centrality (0.024) - this node is a cross-community bridge._
- **Are the 16 inferred relationships involving `use_language()` (e.g. with `About()` and `DinoGame()`) actually correct?**
  _`use_language()` has 16 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ui`, `Bluesky`, `Gitea` to the rest of the system?**
  _32 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `shiki.bundle.js` be split into smaller, more focused modules?**
  _Cohesion score 0.016091954022988506 - nodes in this community are weakly interconnected._
- **Why does `ContentEntry` connect `ContentEntry` to `miniature/mod.rs`, `nav.rs`, `pages/home.rs`, `page.rs`, `build.rs`, `database/rss.rs`, `content.rs`, `ContentDatabase`?**
  _High betweenness centrality (0.013) - this node is a cross-community bridge._
- **Should `miniature/mod.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06327683615819209 - nodes in this community are weakly interconnected._
- **Why does `HomeConfig` connect `api/src/home.rs` to `miniature/mod.rs`, `ContentDatabase`, `pages/home.rs`, `pages/about.rs`?**
  _High betweenness centrality (0.012) - this node is a cross-community bridge._
