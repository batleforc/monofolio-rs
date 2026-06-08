#! /bin/bash

uv init
uv add weasyprint pikepdf

resumectl generate --html --theme tech --color=#ff2b39 --output contents/media/output/ -d contents/media/cv.yaml

## Swap Projects before Education (bloc Projets doit apparaître en premier)
perl -0777 -i -pe 's{(<section class="section">\s*<h2 class="section-title">Education</h2>.*?</section>)(\s*)(<section class="section">\s*<h2 class="section-title">Projects</h2>.*?</section>)}{$3$2$1}gs' contents/media/output/cv.html

# Post-Process

## To FR
sed -i 's/Education/Diplômes et Formations/g' contents/media/output/cv.html
sed -i 's/Professional Experience/Expériences professionnelles/g' contents/media/output/cv.html
sed -i 's/Skills/Compétences/g' contents/media/output/cv.html
sed -i 's/Languages/Langues/g' contents/media/output/cv.html
sed -i 's/Interests/Intérêts/g' contents/media/output/cv.html
sed -i 's/Projects/Projets/g' contents/media/output/cv.html

## Remove margin and put it before the * {
sed -i 's/\* {/@page {\n        size: A4;\n        margin: 0mm;\n      }\n\n\* {/g' contents/media/output/cv.html

## Fix interests-list: WeasyPrint does not support flex-wrap, use inline-block instead
## Inject overrides in a <style> block just before </head> to avoid touching shared rules
perl -0777 -i -pe 's|\.interests-list \{[^}]*\}|.interests-list { display: block; }|g' contents/media/output/cv.html
perl -0777 -i -pe 's|(\.interest-item \{)|.interest-item { display: inline-block; margin: 2px; }\n\n$1|g' contents/media/output/cv.html

sed -i 's|</head>|<style>.interest-item { display: inline-block; margin: 2px; background-color: #1e293b; color: white; }</style>\n</head>|' contents/media/output/cv.html

sed -i 's|content: "## ";| |g' contents/media/output/cv.html
# ## Add space before Education  with a div with height 50px
perl -0777 -i -pe 's|</section>\s*<section class="section">\s*<h2 class="section-title">Projets</h2>|</section>\n\n                <!-- Add space between sections -->\n                <div style="height: 70px;"></div>\n\n                <section class="section">\n                    <h2 class="section-title">Projets</h2>|g' contents/media/output/cv.html

# ## Add space at the end of the last section (Diplômes et Formations) to fill the last page
perl -0777 -i -pe 's|</section>\s*</div>|</section>\n\n                <!-- Add space between sections -->\n                <div style="height: 500px;"></div>\n\n            </div>|g' contents/media/output/cv.html

## Generate PDF using headless Chromium/Chrome
## Try common binary names and fail if none found
CHROME_BIN=""
for cmd in chromium chromium-browser google-chrome chrome; do
	if command -v "$cmd" >/dev/null 2>&1; then
		CHROME_BIN="$(command -v "$cmd")"
		break
	fi
done

if [ -z "$CHROME_BIN" ]; then
	echo "Error: Chromium/Chrome binary not found in PATH" >&2
	exit 1
fi

echo "Generating PDF with $CHROME_BIN..."
## Inject print-optimized CSS to reduce header height and avoid large blank areas
perl -0777 -i -pe 's|</head>|<style media="print">\n@page { size: A4; margin: 0mm; }\n/* Tweak top padding: reduce from 40px to 24px so header is not too low */\n.header { padding: 10px 20px 12px 20px !important; }\n.header::before { display: none !important; }\n.header-main h1 { font-size: 1.2em !important; margin-left: 8px !important; margin-top: 4px !important; }\n.header-photo { margin-left: 8px !important; margin-top: 2px !important; }\n.header-photo img { width: 64px; height: 64px !important; }\n.header-content { gap: 8px !important; align-items: center !important; }\n.left-column { padding: 12px 16px !important; }\n.right-column { padding: 12px !important; }\n.main-content { grid-template-columns: 1fr 200px !important; }\n.section { margin-bottom: 10px !important; }\n.summary { padding: 10px !important; }\n.experience-item, .project-item, .education-item { page-break-inside: avoid; }\n</style>\n</head>|s' contents/media/output/cv.html

"$CHROME_BIN" --headless --disable-gpu --no-sandbox --disable-dev-shm-usage --print-to-pdf="contents/media/output/cv_full.pdf" "contents/media/output/cv.html"

## If pikepdf is available, extract the first 2 pages (preserve previous behavior).
## Otherwise, fall back to the full PDF.
uv run python -c "
import pikepdf
with pikepdf.open('contents/media/output/cv_full.pdf') as pdf:
    dst = pikepdf.Pdf.new()
    dst.pages.extend(pdf.pages[:2])
    dst.save('contents/media/output/cv.pdf')
"
cp contents/media/output/cv.pdf contents/media/cv.pdf
