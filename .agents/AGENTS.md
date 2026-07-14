# TMyadmin — Project Rules for AI Agents

## Mandatory First Step
Before writing ANY code in this project, read the full design system reference:
`/Users/kalebunna/learning/phpmyadmin-mandiri/TMyadmin/style&structure.md`

## Hard Rules — No Exceptions

- **NO** `style="..."` inline attributes in any Vue template
- **NO** hardcoded color values in templates (use CSS variables from `@theme`)
- **NO** hardcoded spacing/size values in templates (use CSS component classes)
- **NO** ad-hoc Tailwind utility classes when a `pma-*` CSS component already exists
- **NO** duplicate Vue components — check `ui/` folder first
- **ALWAYS** use CSS token variables: `var(--color-pma-...)`, `var(--font-size-pma-...)`, etc.
- **ALWAYS** add new CSS rules to `@layer components` in `src/style.css`
- **ALWAYS** update `style&structure.md` after adding new CSS components or Vue components

## Component Naming Conventions

| Type | Prefix | Location |
|---|---|---|
| Layout component | `App` | `src/components/layout/` |
| Reusable UI component | `Pma` | `src/components/ui/` |
| Page component | (none) | `src/components/pages/` |
| CSS class | `pma-` | `src/style.css @layer components` |

## CSS Architecture

- All design tokens → `src/style.css` in `@theme {}`
- All component classes → `src/style.css` in `@layer components {}`
- Tailwind v4 — no `tailwind.config.js` needed
- Zero scoped styles in `.vue` files unless absolutely necessary
