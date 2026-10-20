# User docs: rules for agents and contributors

These guides are maintained in this repository. Do not describe them as published on the Kit site
until that site contains these pages. Keep file names stable when they are linked from other guides.

## Who reads this

Artists and designers using DesignAdobado, community contributors helping them, and AI assistants looking things up for
either. Write for an artist, not a developer.

## Page rules

- One question or problem per page. Lead with the answer or the fix in the first two lines.
- Plain words, short sentences, numbered steps for anything the reader does.
- Start the body at `##`. The page title comes from frontmatter.
- Link to other pages instead of repeating them. Use relative links to the `.md` file:
  `[Install DesignAdobado](getting-started/install.md)`.
- No marketing copy, no "simply", no claims you have not checked in the code or a release.
- Call the product "DesignAdobado" and link only to current Adobado Kit or GitHub pages.
- Name UI labels exactly as the app shows them, in **bold**.
- Document Windows 11 only. Do not name older Windows versions.
- Troubleshooting pages: `## Symptoms`, `## Fix`, `## Why this happens` (optional),
  `## Still stuck?` (link the issue tracker).

## Frontmatter

    ---
    title: App closes right after it starts
    description: One sentence for search results, link previews and AI summaries.
    order: 10                 # optional, sorts within the folder (lower first)
    verified: 0.6.0           # optional, app version you last checked this page against
    redirect_from: [old/path] # optional, old URLs (relative to /docs/designcraft)
    ---

Only `key: value` and `key: [a, b]` lines work. Quote values that start with a quote or contain ` #`.

Set `verified` only to a DesignAdobado release that has been checked manually. Do not use a DesignCraft version or an upstream release as a verified DesignAdobado version. Omit `verified` when no tested DesignAdobado package is available.

## Folders

- `getting-started/` install and first run
- `guides/` how to do one thing
- `troubleshooting/` symptom → fix
- `faq/` short answers
- `reference/` lists and settings

A folder's `index.md` names the folder in the sidebar. Files or folders starting with `_` are
drafts and are not published.

## Check before you push

    node <path to your artcraft-services checkout>/frontend/apps/artcraft-website-next/scripts/docs/core.mjs docs/user

The checker lives in storytold/artcraft-services. A CI check will be added once it is on that repo's main branch.

## Support PRs

Pages written to answer a user question come in as PRs titled `docs(support): <page title>`.
Review them like code: check every claim against the source. Questions and bugs go to
https://github.com/smouj/DesignAdobado/issues.
