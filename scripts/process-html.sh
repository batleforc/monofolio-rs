#! /bin/bash

uv init
uv add weasyprint pikepdf

resumectl generate --html --theme tech --color=#ff2b39 --output contents/media/output/ -d contents/media/cv.yaml

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

## Add space before Education  with a div with height 50px
perl -0777 -i -pe 's|</section>\s*<section class="section">\s*<h2 class="section-title">Diplômes et Formations</h2>|</section>\n\n                <!-- Add space between sections -->\n                <div style="height: 50px;"></div>\n\n                <section class="section">\n                    <h2 class="section-title">Diplômes et Formations</h2>|g' contents/media/output/cv.html

## Add space at the end of the Projets section to fill the last page
perl -0777 -i -pe 's|</section>\s*</div>|</section>\n\n                <!-- Add space between sections -->\n                <div style="height: 200px;"></div>\n\n            </div>|g' contents/media/output/cv.html

.venv/bin/weasyprint contents/media/output/cv.html contents/media/output/cv_full.pdf
uv run python -c "
import pikepdf
with pikepdf.open('contents/media/output/cv_full.pdf') as pdf:
    dst = pikepdf.Pdf.new()
    dst.pages.extend(pdf.pages[:2])
    dst.save('contents/media/output/cv.pdf')
"
cp contents/media/output/cv.pdf contents/media/cv.pdf
