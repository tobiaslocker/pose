import os
import re
from datetime import datetime

ADR_DIR = os.path.join(os.path.dirname(__file__), '../../docs/arch/decisions')
LOG_PATH = os.path.join(ADR_DIR, 'README.md')

entries = []

def badge_for_status(status):
    if status == "Accepted":
        return "![](https://img.shields.io/badge/status-accepted-brightgreen)"
    elif status == "Proposed":
        return "![](https://img.shields.io/badge/status-proposed-yellow)"
    else:
        return "![](https://img.shields.io/badge/status-unknown-lightgrey)"


for filename in sorted(os.listdir(ADR_DIR)):
    if not filename.endswith('.md') or filename == 'README.md':
        continue

    with open(os.path.join(ADR_DIR, filename)) as f:
        content = f.read()
        title_match = re.match(r'#\s+(.*)', content)
        status_match = re.search(r'\[Status:\s*([a-zA-Z]+)\]', content)
        date_match = re.search(r'^📅\s*(\d{4}-\d{2}-\d{2})', content, re.MULTILINE)
        title = title_match.group(1).strip() if title_match else filename
        raw_status = status_match.group(1).strip() if status_match else "Unknown"
        status = badge_for_status(raw_status)
        date = date_match.group(1).strip() if date_match else "?"
        md_link = f"[{title}]({filename})"
        entries.append({
            'number': filename.split('-')[0],
            'md_link': md_link,
            'status': status,
            'date': date
        })

# Compute max widths
number_width = max(len(e['number']) for e in entries + [{'number': 'Number'}])
title_width = max(len(e['md_link']) for e in entries + [{'md_link': 'Title'}])
status_width = max(len(e['status']) for e in entries + [{'status': 'Status'}])
date_width = max(len(e['date']) for e in entries + [{'date': 'Date'}])

with open(LOG_PATH, 'w') as f:
    f.write('# Architecture Decision Log\n\n')

    header = f"| {'Number'.ljust(number_width)} | {'Title'.ljust(title_width)} | " \
             f"{'Status'.ljust(status_width)} | {'Date'.ljust(date_width)} |\n"
    divider = f"|{'-' * (number_width + 2)}|{'-' * (title_width + 2)}|" \
              f"{'-' * (status_width + 2)}|{'-' * (date_width + 2)}|\n"

    f.write(header)
    f.write(divider)

    for e in entries:
        line = f"| {e['number'].ljust(number_width)} | {e['md_link'].ljust(title_width)} | " \
               f"{e['status'].ljust(status_width)} | {e['date'].ljust(date_width)} |\n"
        f.write(line)
