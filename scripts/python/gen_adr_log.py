import os
import re

ADR_DIR = os.path.join(os.path.dirname(__file__), '../../docs/arch/decisions')
LOG_PATH = os.path.join(ADR_DIR, 'README.md')

entries = []

for filename in sorted(os.listdir(ADR_DIR)):
    if not filename.endswith('.md') or filename == 'README.md':
        continue

    with open(os.path.join(ADR_DIR, filename), encoding='utf-8') as f:
        content = f.read()

        # Extract the title (first level-1 heading) or fallback to filename
        title_match = re.match(r'#\s+(.*)', content)
        title = title_match.group(1).strip() if title_match else filename

        # Look for ANY Markdown image whose URL contains "img.shields.io/badge/status-..."
        badge_match = re.search(
            r'!\[.*?\]\(https://img\.shields\.io/badge/status-[^\)]+\)',
            content
        )
        # If found, use exactly that string; otherwise, show a generic "unknown" badge
        if badge_match:
            status = badge_match.group(0)
        else:
            status = "![](https://img.shields.io/badge/status-unknown-lightgrey)"

        # Extract the date line that starts with the 📅 emoji (YYYY-MM-DD)
        date_match = re.search(r'^📅\s*(\d{4}-\d{2}-\d{2})', content, re.MULTILINE)
        date = date_match.group(1).strip() if date_match else "?"

        # Build the Markdown link for this ADR
        md_link = f"[{title}]({filename})"

        entries.append({
            'number': filename.split('-')[0],
            'md_link': md_link,
            'status': status,
            'date': date
        })

# Compute column widths dynamically
number_width = max(len(e['number']) for e in entries + [{'number': 'Number'}])
title_width = max(len(e['md_link']) for e in entries + [{'md_link': 'Title'}])
status_width = max(len(e['status']) for e in entries + [{'status': 'Status'}])
date_width = max(len(e['date']) for e in entries + [{'date': 'Date'}])

with open(LOG_PATH, 'w', encoding='utf-8') as f:
    f.write('# Architecture Decision Log\n\n')

    # Header row
    header = (
        f"| {'Number'.ljust(number_width)} "
        f"| {'Title'.ljust(title_width)} "
        f"| {'Status'.ljust(status_width)} "
        f"| {'Date'.ljust(date_width)} |\n"
    )
    # Divider row
    divider = (
        f"|{'-' * (number_width + 2)}"
        f"|{'-' * (title_width + 2)}"
        f"|{'-' * (status_width + 2)}"
        f"|{'-' * (date_width + 2)}|\n"
    )

    f.write(header)
    f.write(divider)

    # Each ADR entry
    for e in entries:
        line = (
            f"| {e['number'].ljust(number_width)} "
            f"| {e['md_link'].ljust(title_width)} "
            f"| {e['status'].ljust(status_width)} "
            f"| {e['date'].ljust(date_width)} |\n"
        )
        f.write(line)
