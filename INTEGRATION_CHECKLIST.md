# 📋 Checklist d'Intégration - Rendu Markdown AST

## Phase 1: Préparation ✅ TERMINÉE

- [x] Composant `RenderMarkdownNode` implémenté
- [x] Composant `MarkdownRenderer` implémenté
- [x] Composant `MarkdownFromValue` implémenté
- [x] Structures `MarkdownNode` et `MarkdownContent` définis
- [x] 28+ types de nœuds supportés
- [x] Styling Tailwind appliqué
- [x] Module enregistré dans `components/mod.rs`
- [x] Code compilable sans erreurs

**Fichiers créés:** 2
**Fichiers modifiés:** 1
**Lignes de code:** 330+

---

## Phase 2: Intégration dans content.rs 🔄 À FAIRE

### Step 2.1: Ajouter l'import
**Fichier:** `/var/mnt/data/git/monofolio-v2/apps/frontend/src/pages/content.rs`
**Ligne:** ~10 (avec les autres imports)

```rust
use crate::components::markdown::MarkdownFromValue;
```

- [ ] Import ajouté
- [ ] Pas de conflit de noms

### Step 2.2: Remplacer le rendu JSON
**Fichier:** `/var/mnt/data/git/monofolio-v2/apps/frontend/src/pages/content.rs`
**Ligne:** ~460-470 (dans le `is_blog` block)

**AVANT:**
```rust
{if is_blog {
    view! {
        <div class="mt-5 border-t border-border pt-4">
            <p class="text-sm font-semibold mb-2">
                "Blog post content"
            </p>
            <pre class="text-xs text-muted-foreground overflow-x-auto whitespace-pre-wrap bg-background/70 border border-border rounded p-3">
                {content_pretty}
            </pre>
        </div>
    }
        .into_any()
} else {
    view! { <></> }.into_any()
}}
```

**APRÈS:**
```rust
{if is_blog {
    view! {
        <div class="mt-5 border-t border-border pt-4">
            <MarkdownFromValue value=content />
        </div>
    }
        .into_any()
} else {
    view! { <></> }.into_any()
}}
```

- [ ] Code remplacé
- [ ] Compilation réussie

### Step 2.3: Supprimer le code inutile
**Fichier:** `/var/mnt/data/git/monofolio-v2/apps/frontend/src/pages/content.rs`
**Ligne:** ~330-340 (dans le PageData mapping)

Supprimer cette ligne (car `content_pretty` n'est plus utilisé):
```rust
let content_pretty = serde_json::to_string_pretty(&content)
    .unwrap_or_else(|_| String::from("{}"));
```

- [ ] Ligne supprimée
- [ ] Pas d'erreurs "unused variable"

---

## Phase 3: Vérification 🔍 À FAIRE

### Step 3.1: Build local
```bash
cd /var/mnt/data/git/monofolio-v2
cargo build -p frontend --features ssr
```

**Critères de succès:**
- [ ] Compilation sans erreurs
- [ ] Pas de warnings sérieux (dead_code warnings OK)
- [ ] Temps de compilation < 60s

### Step 3.2: Lancer dev server
```bash
cargo leptos watch -p frontend
```

- [ ] Serveur démarre sans erreur
- [ ] Aucun panic dans console
- [ ] Hot reload fonctionne

### Step 3.3: Tester rendu blog
1. Ouvrir navigateur: `http://localhost:3000`
2. Naviguer vers blog: `/blogs/first-one` (ou autre)
3. Vérifier:

- [ ] Headings affichés avec taille correcte (h1 > h2 > etc)
- [ ] Paragraphes espacés correctement
- [ ] **Gras** et *italique* rendus correctement
- [ ] `Code inline` en monospace
- [ ] Blocs de code avec fond sombre
- [ ] Listes (bullets/numbers) indentées
- [ ] Liens cliquables et stylisés
- [ ] Images affichées (si présentes)
- [ ] Tableaux avec bordures (si présents)
- [ ] Aucun rendu "Failed to parse markdown"

### Step 3.4: Tester rendu docs
1. Naviguer vers docs: `/docs` ou `/docs/cicd/portfolio-cicd`
2. Vérifier:

- [ ] Rendu identique aux blogs
- [ ] Sidebar toujours visible et fonctionnel
- [ ] Navigation entre pages fonctionne
- [ ] URLs correctes dans les liens

### Step 3.5: Tester cas limites
1. Naviguer vers page avec beaucoup de contenu:
   - [ ] Pas de lag/ralentissement
   - [ ] Scroll lisse
   - [ ] Mémoire stable

2. Tester avec contenu spécial:
   - [ ] HTML en markdown (doit afficher warning)
   - [ ] Code très long (overflow-x auto fonctionne)
   - [ ] Tables complexes (alignement OK)

---

## Phase 4: Optimisation (Optionnel) 🚀

- [ ] Ajouter syntax highlighting pour code blocks
- [ ] Implémenter HTML sanitization (ammonia crate)
- [x] Ajouter anchor links aux headings
- [x] Générer table of contents
- [ ] Support des footnotes

---

## Rollback Plan 🔙

Si problèmes rencontrés:

1. **Erreur compilation:**
   ```bash
   git checkout apps/frontend/src/pages/content.rs
   cargo build -p frontend --features ssr
   ```

2. **Erreur rendu:**
   - Vérifier format JSON de `/api/v1/page/{handle}`
   - Vérifier console DevTools pour erreurs sérialisation
   - Vérifier que API retourne `content: {format: "markdown_ast", nodes: [...]}`

3. **Performance:**
   - Profiler avec DevTools Performance tab
   - Vérifier pas de re-render excessifs
   - Considérer memoization si document très grand (> 1000 nœuds)

---

## Estimation Temps

| Phase | Durée | Difficultés |
|-------|-------|------------|
| Intégration code | 5-10 min | Aucune - simple copy-paste |
| Build & test | 10-15 min | Dépend build time |
| Debug (si needed) | 15-30 min | Généralement straightforward |
| **TOTAL** | **30-50 min** | **Faible** |

---

## Notes Importantes

⚠️ **Avant d'intégrer:**
- Backend API doit retourner `content.nodes` (vérifier `/api/v1/page/{handle}`)
- Tailwind classes doivent être scannées (vérify tailwind.config.js)
- Leptos 0.5+ requis (check Cargo.toml)

✅ **Après intégration:**
- Run tests (si existants)
- Vérifier responsive design mobile
- Test sur multiple browsers si possible
- Considérer commit message: "feat: implement markdown AST rendering"

---

## Documentation Liée

- [MARKDOWN_AST_RENDERER.md](MARKDOWN_AST_RENDERER.md) - Guide complet
- [IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md) - Vue d'ensemble
- [components/markdown.rs](apps/frontend/src/components/markdown.rs) - Source code
