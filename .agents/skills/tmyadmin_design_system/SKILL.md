---
name: tmyadmin_design_system
description: >
  TMyadmin project design system reference. Triggers when working on any Vue component,
  CSS style, layout, or UI element in the TMyadmin / phpmyadmin-mandiri project.
  Enforces no inline styles, consistent CSS component usage, and correct Vue component patterns.
---

# TMyadmin Design System Skill

Saat bekerja di project TMyadmin, **WAJIB** membaca dan mengikuti panduan di:

```
/Users/kalebunna/learning/phpmyadmin-mandiri/TMyadmin/style&structure.md
```

## Aturan Wajib

1. **Baca `style&structure.md`** sebelum menulis kode apapun
2. **DILARANG** `style="..."` inline di template Vue — gunakan CSS component class
3. **DILARANG** warna hardcoded di template — gunakan token `var(--color-pma-...)`
4. **Selalu periksa** CSS Components Catalog di `style&structure.md` sebelum membuat class baru
5. **Selalu periksa** Vue Components Catalog sebelum membuat component baru
6. **Update `style&structure.md`** setiap kali menambahkan CSS component atau Vue component baru

## Sumber Kebenaran

- **Design tokens**: `src/style.css` blok `@theme {}`
- **CSS components**: `src/style.css` blok `@layer components {}`
- **Vue components**: `src/components/` (layout/, ui/, pages/)
- **Referensi visual**: `template_sample/screen.png`
- **Panduan lengkap**: `style&structure.md`
