# TMyadmin — Style & Structure Reference
> **Untuk AI Agent**: Baca dokumen ini sebelum menulis kode apapun di project ini.
> Update dokumen ini setiap kali menambahkan komponen atau token baru.

---

## 🚫 LARANGAN MUTLAK UNTUK AI

> CAUTION: Melanggar aturan ini berarti kode harus ditulis ulang.

1. **DILARANG** menulis `style="..."` inline di template Vue manapun
2. **DILARANG** menulis warna hardcoded (`#235a81`, `red`, dll) langsung di template — selalu gunakan CSS variable dari `@theme`
3. **DILARANG** menulis nilai spacing/ukuran hardcoded (`margin: 20px`) di template — gunakan CSS component class
4. **DILARANG** membuat class Tailwind utility ad-hoc (`class="flex gap-4 text-red-500"`) untuk pola yang sudah ada sebagai CSS component
5. **DILARANG** menduplikasi logika visual yang sudah ada sebagai komponen Vue (`PmaBtn`, `PmaActionLink`, dll) — selalu gunakan komponen yang sudah ada
6. **DILARANG** membuat component Vue baru yang hanya berisi 1–2 elemen sederhana tanpa props — gunakan CSS class saja

> IMPORTANT: **Kapan membuat CSS component baru?** Jika kombinasi style yang sama muncul di 2+ tempat tulis ke `@layer components` di `style.css`, bukan inline.
> **Kapan membuat Vue component baru?** Jika ada logika reaktif (props, emit, v-model, slot) + dipakai di 2+ tempat buat Vue component.

---

## 📁 Struktur Proyek

```
TMyadmin/
├── src/
│   ├── style.css                    ← Design system UTAMA — satu-satunya sumber kebenaran CSS
│   ├── main.js                      ← Entry point, import style.css
│   ├── App.vue                      ← Root layout
│   └── components/
│       ├── layout/                  ← Komponen struktural halaman
│       │   ├── AppSidebar.vue
│       │   ├── AppTopHeader.vue
│       │   ├── AppTabNav.vue
│       │   └── AppConsoleFooter.vue
│       ├── ui/                      ← Komponen UI reusable
│       │   ├── PmaTable.vue
│       │   ├── PmaActionLink.vue
│       │   ├── PmaInfoBox.vue
│       │   ├── PmaBtn.vue
│       │   ├── PmaSectionHeader.vue
│       │   ├── PmaModal.vue             ← Dialog konfirmasi hapus/drop tabel
│       │   └── PmaToast.vue             ← Notifikasi toast (sukses/error)
│       └── pages/                   ← Komponen halaman (satu per tab/view)
│           ├── UserAccountsPage.vue
│           ├── DatabaseManagerPage.vue  ← Manajemen Database (Structure, SQL, Export, Import)
│           ├── TableManagerPage.vue     ← Halaman Tabel (Tab Container)
│           └── table/                   ← Sub-komponen detail manajemen tabel
│               ├── TableBrowse.vue      ← Tab Data Browser & Insertion
│               ├── TableStructure.vue   ← Tab Metadata Column Structure
│               └── TableRelation.vue    ← Tab Foreign Key Constraints
├── template_sample/
│   ├── code.html                    ← Template referensi asli
│   └── screen.png                   ← Screenshot referensi visual
├── style&structure.md               ← Dokumen ini
└── vite.config.js                   ← Tailwind CSS v4 via @tailwindcss/vite
```

---

## 🎨 Design Tokens — CSS Variables (`@theme`)

Semua token ada di `src/style.css` dalam blok `@theme {}`.
**Jangan pernah menulis nilai warna atau ukuran hardcoded — selalu referensikan token ini.**

### Warna Brand
| Token | Nilai | Penggunaan |
|---|---|---|
| `--color-pma-blue` | `#235a81` | Brand utama, link, focus |
| `--color-pma-blue-light` | `#3a7ab5` | Hover state ringan |
| `--color-pma-blue-hover` | `#1a4562` | Hover state gelap |

### Warna Background
| Token | Nilai | Penggunaan |
|---|---|---|
| `--color-pma-bg` | `#f3f3f3` | Background halaman, sidebar |
| `--color-pma-bg-white` | `#ffffff` | Background putih (card, table, input) |
| `--color-pma-bg-row-hover` | `#ffffcc` | Row tabel saat hover |
| `--color-pma-bg-warning` | `#ffffcc` | Background warning box |
| `--color-pma-bg-info` | `#f8f8f8` | Background info box |
| `--color-pma-bg-header-from` | `#ffffff` | Gradient header — start |
| `--color-pma-bg-header-to` | `#e8e8e8` | Gradient header — end |

### Warna Border
| Token | Nilai | Penggunaan |
|---|---|---|
| `--color-pma-border` | `#d3d3d3` | Border umum (sidebar, tabel, input) |
| `--color-pma-border-warning` | `#cccc00` | Border warning box |
| `--color-pma-border-info` | `#aaaaaa` | Border info box |

### Warna Teks
| Token | Nilai | Penggunaan |
|---|---|---|
| `--color-pma-text` | `#333333` | Teks utama |
| `--color-pma-text-muted` | `#888888` | Teks sekunder / hint |
| `--color-pma-text-privilege` | `#cc3399` | Nilai privilege MySQL (merah muda) |
| `--color-pma-text-danger` | `#cc0000` | Error / bahaya (merah) |
| `--color-pma-text-link` | `#235a81` | Warna link action |
| `--color-pma-text-warning` | `#856404` | Teks pada warning |

### Typography
| Token | Nilai | Penggunaan |
|---|---|---|
| `--font-size-pma-base` | `11px` | Font size default seluruh app |
| `--font-size-pma-small` | `10px` | Font size sangat kecil (filter input) |
| `--font-size-pma-heading` | `1.3rem` | Judul halaman (`pma-page-title`) |
| `--font-family-pma` | `sans-serif` | Font family seluruh app |
| `--line-height-pma` | `1.4` | Line height default |

### Spacing
| Token | Nilai | Penggunaan |
|---|---|---|
| `--spacing-pma-xs` | `2px` | Jarak sangat kecil |
| `--spacing-pma-sm` | `4px` | Padding kecil (tab, button) |
| `--spacing-pma-md` | `6px` | Padding default |
| `--spacing-pma-lg` | `10px` | Padding medium |
| `--spacing-pma-xl` | `15px` | Spacing besar |
| `--spacing-pma-2xl` | `20px` | Spacing antar section |

### Border & Radius
| Token | Nilai | Penggunaan |
|---|---|---|
| `--radius-pma-sm` | `2px` | Radius default (button, input) |
| `--radius-pma-md` | `4px` | Radius medium (info/warning box) |

### Gradients
| Token | Nilai | Penggunaan |
|---|---|---|
| `--gradient-pma-header` | `linear-gradient(to bottom, #fff 0%, #e8e8e8 100%)` | Header bar |
| `--gradient-pma-th` | `linear-gradient(to bottom, #fff 0%, #e8e8e8 100%)` | `<th>` tabel |

### Dimensi
| Token | Nilai | Penggunaan |
|---|---|
| `--width-pma-sidebar` | `240px` | Lebar sidebar kiri |
| `--width-pma-db-panel` | `280px` | Lebar database panel kanan |

---

## 📦 CSS Components Catalog (`@layer components`)

Semua class berikut ada di `src/style.css`. Gunakan class ini, jangan buat style baru.

### Layout Root
| Class | Deskripsi |
|---|---|
| `.pma-app` | Container utama — `display: flex; height: 100vh; overflow: hidden` |
| `.pma-main` | Area konten kanan — `flex: 1; overflow-y: auto; flex-direction: column` |

### Sidebar
| Class | Deskripsi |
|---|---|
| `.pma-sidebar` | Aside kiri — `width: 240px; border-right; flex-column` |
| `.pma-sidebar-logo-area` | Area logo + icon bar |
| `.pma-sidebar-logo` | `<img>` logo (`width: 120px`) |
| `.pma-sidebar-icon-bar` | Row icon-icon toolbar |
| `.pma-sidebar-icon` | Satu icon — opacity hover |
| `.pma-sidebar-server-label` | Label "Current server:" |
| `.pma-sidebar-server-select` | `<select>` server |
| `.pma-sidebar-quick-btns` | Area tombol Recent/Favorites |
| `.pma-sidebar-filter` | Area input filter |
| `.pma-sidebar-filter-input` | `<input>` filter database |
| `.pma-sidebar-tree` | Scrollable area database list |
| `.pma-db-item` | Satu item database — hover bg |
| `.pma-db-item.italic` | Modifier — database system (italic) |
| `.pma-db-item-icon` | Icon +/- di item database |

### Header
| Class | Deskripsi |
|---|---|
| `.pma-header` | Top header bar — gradient background |
| `.pma-header-left` | Sisi kiri header (back arrow + server info) |
| `.pma-header-back` | Tombol panah kembali |
| `.pma-header-server-info` | Label nama server |
| `.pma-header-right` | Sisi kanan header (action buttons) |
| `.pma-header-action-btn` | Tombol aksi header (user, logout) |

### Tab Navigation
| Class | Deskripsi |
|---|---|
| `.pma-tab-nav` | Container tab row — `display: flex; flex-wrap` |
| `.pma-tab` | Satu tab — border, radius atas, cursor pointer |
| `.pma-tab.active` | Tab aktif — bg putih, border bawah putih, bold |
| `.pma-tab-icon` | Icon di dalam tab |

### Console Footer
| Class | Deskripsi |
|---|---|
| `.pma-console-footer` | Fixed bottom bar — `position: fixed; left: 240px` |
| `.pma-console-btn` | Tombol Console di footer |

### Page Area
| Class | Deskripsi |
|---|---|
| `.pma-page-area` | Scrollable content area — `flex: 1; overflow-y: auto` |
| `.pma-page-placeholder` | Placeholder tab belum diimplementasi — muted italic |

### Database Panel (right side)
| Class | Deskripsi |
|---|---|
| `.pma-db-panel` | Container panel kanan — lebar 280px, border kiri, flex-column |
| `.pma-db-panel.closed` | Modifier hide — width 0, opacity 0, pointer-events none |
| `.pma-db-panel-toggle` | Tombol toggle di header — abu default, biru saat `.active` |
| `.pma-db-panel-toggle.active` | State aktif (panel terbuka) — bg biru, teks putih |
| `.pma-db-panel-header` | Header panel — gradient, border bawah, flex space-between |
| `.pma-db-panel-title` | Judul panel — bold, icon + teks |
| `.pma-db-panel-close` | Tombol X tutup panel — hover bg abu |
| `.pma-db-panel-search` | Area input filter database |
| `.pma-db-panel-search-input` | Input filter — full width, focus biru |
| `.pma-db-panel-stats` | Stats bar — jumlah DB, badge user/system |
| `.pma-db-panel-list` | Scrollable list area |
| `.pma-db-panel-list-head` | Header kolom sticky — gradient, bold |
| `.pma-db-panel-list-head-name` | Kolom nama — `flex: 1` |
| `.pma-db-panel-list-head-tables` | Kolom tables — 48px, right-align |
| `.pma-db-panel-list-head-size` | Kolom size — 52px, right-align |
| `.pma-db-panel-row` | Satu baris database — hover kuning |
| `.pma-db-panel-row.active` | Baris aktif — bg biru muda, teks biru bold |
| `.pma-db-panel-row-icon` | Icon database di baris |
| `.pma-db-panel-row-name` | Nama database — truncate overflow |
| `.pma-db-panel-row-name.italic` | Modifier sistem DB — italic muted |
| `.pma-db-panel-row-tables` | Jumlah tables — 48px muted |
| `.pma-db-panel-row-size` | Ukuran database — 52px muted |
| `.pma-db-panel-badge` | Badge kecil — bg abu, border, rounded |
| `.pma-db-panel-footer` | Footer panel — total DB, badge engine |

### Konten Halaman
| Class | Deskripsi |
|---|---|
| `.pma-content` | Container area konten — `padding: 12px 16px; padding-bottom: 36px` |
| `.pma-page-title` | `<h2>` judul halaman — font 1.3rem, weight normal |

### Tabel
| Class | Deskripsi |
|---|---|
| `.pma-table` | Tabel utama — `border-collapse: collapse; width: 100%` |
| `.pma-table th` | Header cell — gradient bg, border, padding |
| `.pma-table td` | Data cell — border, padding, bg putih |
| `.pma-table tbody tr:hover td` | Row hover — bg kuning `#ffffcc` |
| `.pma-table-actions` | Container link aksi dalam cell — `display: flex; gap: 12px` |
| `.pma-table-wrapper` | Scrollable wrapper tabel — `overflow-x: auto` |

### Table Column Width Helpers
| Class | Lebar | Gunakan pada |
|---|---|---|
| `.pma-th-checkbox` | 28px | Kolom checkbox |
| `.pma-th-narrow` | 60px | Kolom Password, Grant |
| `.pma-th-medium` | 120px | Kolom medium |
| `.pma-th-wide` | 200px | Kolom lebar |
| `.pma-th-action` | 260px | Kolom Action |

### Table Cell Variants
| Class | Deskripsi |
|---|---|
| `.pma-cell-privilege` | Teks privilege MySQL — warna `#cc3399` (pink) |
| `.pma-cell-danger` | Teks bahaya — warna `#cc0000`, bold |

### Table Header Helpers
| Class | Deskripsi |
|---|---|
| `.pma-th-help-icon` | Icon help di header tabel — cursor help, biru |

### Action Link
| Class | Deskripsi |
|---|---|
| `.pma-action-link` | Link aksi biru — inline-flex, no underline default |
| `.pma-action-link:hover` | Underline + warna gelap |
| `.pma-action-link-icon` | Icon di dalam action link |

### Buttons
| Class | Deskripsi |
|---|---|
| `.pma-btn` | Tombol default — bg abu, border, hover |
| `.pma-quick-btn` | Tombol kecil sidebar (Recent, Favorites) |

### Section Panel
| Class | Deskripsi |
|---|---|
| `.pma-section-header` | Label header section — inline-flex, bg abu, border atas |
| `.pma-section-body` | Body section — border (kecuali atas), padding |
| `.pma-section-mb` | Wrapper section dengan `margin-bottom: 20px` |
| `.pma-section-p` | Paragraf dalam section body |
| `.pma-section-footer` | Footer section — `justify-content: flex-end` |

### Notification Boxes
| Class | Deskripsi |
|---|---|
| `.pma-info-box` | Box informasi — bg abu muda, border abu |
| `.pma-warning-box` | Box peringatan — bg kuning, border kuning |
| `.pma-box-icon` | Icon di kiri box |
| `.pma-box-content` | Konten teks box — `flex: 1` |
| `.pma-box-link` | Link di dalam box — underline biru |

### Form Elements
| Class | Deskripsi |
|---|---|
| `.pma-checkbox` | `<input type="checkbox">` — accent color biru |
| `.pma-form-label` | Label + checkbox inline — `display: flex; gap: 4px; cursor: pointer` |
| `.pma-form-label-mb` | Sama + `margin-bottom: 12px` |

### Divider
| Class | Deskripsi |
|---|---|
| `.pma-divider` | `<hr>` — border atas abu, no default border |

### Bulk Action Bar
| Class | Deskripsi |
|---|---|
| `.pma-bulk-bar` | Container bar aksi massal — flex, gap |
| `.pma-bulk-bar-arrow` | Simbol tanda arah — warna muted |
| `.pma-bulk-bar-label` | Label "With selected:" — italic |

---

## 📐 Grid System — `pma-row` + `pma-col-{1..12}`

12-kolom flex grid, konsep mirip Bootstrap. Tidak bergantung pada breakpoint — murni flex.

```html
<!-- 2 kolom sejajar (6+6) -->
<div class="pma-row">
  <div class="pma-col-6">Kiri</div>
  <div class="pma-col-6">Kanan</div>
</div>

<!-- Sidebar 3 + konten 9 -->
<div class="pma-row">
  <div class="pma-col-3">Sidebar</div>
  <div class="pma-col-9">Konten</div>
</div>

<!-- Full width -->
<div class="pma-row">
  <div class="pma-col-12">Full</div>
</div>

<!-- Auto fill sisa -->
<div class="pma-row">
  <div class="pma-col-2">Label fixed</div>
  <div class="pma-col-auto">Isi sisa</div>
</div>
```

| Class | Lebar |
|---|---|
| `.pma-col-1` | 8.33% |
| `.pma-col-2` | 16.67% |
| `.pma-col-3` | 25% |
| `.pma-col-4` | 33.33% |
| `.pma-col-5` | 41.67% |
| `.pma-col-6` | 50% |
| `.pma-col-7` | 58.33% |
| `.pma-col-8` | 66.67% |
| `.pma-col-9` | 75% |
| `.pma-col-10` | 83.33% |
| `.pma-col-11` | 91.67% |
| `.pma-col-12` | 100% |
| `.pma-col-auto` | fill sisa |

---

## 🧩 Vue Components Catalog

### Layout Components (`src/components/layout/`)

#### `AppSidebar.vue`
- **Props**: `databases: Array`, `currentServer: string`
- **Emits**: `db-select(name)`, `server-change(value)`
- **CSS**: `.pma-sidebar`, `.pma-sidebar-logo-area`, `.pma-sidebar-icon-bar`, `.pma-sidebar-icon`, `.pma-sidebar-server-label`, `.pma-sidebar-server-select`, `.pma-sidebar-quick-btns`, `.pma-quick-btn`, `.pma-sidebar-filter`, `.pma-sidebar-filter-input`, `.pma-sidebar-tree`, `.pma-db-item`, `.pma-db-item-icon`

#### `AppTopHeader.vue`
- **Props**: `serverName: string`
- **Emits**: `user-profile`, `logout`
- **CSS**: `.pma-header`, `.pma-header-left`, `.pma-header-back`, `.pma-header-server-info`, `.pma-header-right`, `.pma-header-action-btn`

#### `AppTabNav.vue`
- **Props**: `tabs: Array<{id, icon, label, closable}>`, `activeTab: string`
- **Emits**: `tab-change(tabId)`, `tab-close(tabId)`
- **CSS**: `.pma-tab-nav`, `.pma-tab`, `.pma-tab.active`, `.pma-tab-icon`

#### `AppConsoleFooter.vue`
- **Props**: tidak ada
- **CSS**: `.pma-console-footer`, `.pma-console-btn`

#### `AppDatabasePanel.vue`
- **Props**: `open: boolean`, `activeDb: string`
- **Emits**: `close`, `db-select(name)`
- **Data internal**: `databases[]` (18 dummy entries), `searchQuery`, `filteredDatabases`, `totalDbs`, `userDbs`, `filteredCount`
- **CSS**: `.pma-db-panel`, `.pma-db-panel.closed`, `.pma-db-panel-header`, `.pma-db-panel-title`, `.pma-db-panel-close`, `.pma-db-panel-search`, `.pma-db-panel-search-input`, `.pma-db-panel-stats`, `.pma-db-panel-list`, `.pma-db-panel-list-head`, `.pma-db-panel-row`, `.pma-db-panel-row.active`, `.pma-db-panel-row-name.italic`, `.pma-db-panel-badge`, `.pma-db-panel-footer`
- **Fitur**: search/filter reaktif, system DB di-italic, active state baris, stats bar, animasi slide via CSS transition

#### `AppTopHeader.vue` (diupdate)
- **Props tambahan**: `dbPanelOpen: boolean`
- **Emits tambahan**: `toggle-db-panel`
- **CSS tambahan**: `.pma-db-panel-toggle`, `.pma-db-panel-toggle.active`

---

### UI Components (`src/components/ui/`)

#### `PmaTable.vue`
- **Slots**: `#head` (thead rows), `default` (tbody rows)
- **CSS**: `.pma-table-wrapper`, `.pma-table`
```html
<PmaTable>
  <template #head>
    <tr>
      <th class="pma-th-checkbox"></th>
      <th>Username</th>
      <th class="pma-th-narrow">Status</th>
      <th class="pma-th-action">Action</th>
    </tr>
  </template>
  <tr v-for="item in items" :key="item.id">
    <td class="pma-th-checkbox">
      <input type="checkbox" class="pma-checkbox" />
    </td>
    <td>{{ item.name }}</td>
    <td :class="{ 'pma-cell-danger': item.isError }">{{ item.status }}</td>
    <td>
      <div class="pma-table-actions">
        <PmaActionLink icon="✏️" label="Edit" />
      </div>
    </td>
  </tr>
</PmaTable>
```

#### `PmaActionLink.vue`
- **Props**: `icon: string`, `label: string (required)`, `href: string`
- **CSS**: `.pma-action-link`, `.pma-action-link-icon`
```html
<PmaActionLink icon="👤" label="Edit privileges" href="/edit" />
<PmaActionLink icon="📤" label="Export" />
```

#### `PmaInfoBox.vue`
- **Props**: `variant: 'info' | 'warning'`, `icon: string (opsional)`
- **Slots**: `default`
- **CSS**: `.pma-info-box` / `.pma-warning-box`, `.pma-box-icon`, `.pma-box-content`
```html
<PmaInfoBox variant="warning">
  Pesan peringatan dengan <a class="pma-box-link" href="#">link</a>.
</PmaInfoBox>
<PmaInfoBox variant="info">
  Informasi tambahan.
</PmaInfoBox>
```

#### `PmaBtn.vue`
- **Props**: `type: 'button' | 'submit' | 'reset'`
- **Emits**: `click`
- **Slots**: `default`
- **CSS**: `.pma-btn`
```html
<PmaBtn @click="handleSave">Save</PmaBtn>
<PmaBtn type="submit">Go</PmaBtn>
```

#### `PmaSectionHeader.vue`
- **Props**: `icon: string (opsional)`, `label: string (required)`
- **Slots**: `default` (body section)
- **CSS**: `.pma-section-header`, `.pma-section-body`
```html
<!-- Header saja -->
<PmaSectionHeader label="New" />

<!-- Header + body -->
<PmaSectionHeader icon="👥" label="Remove selected user accounts">
  <p class="pma-section-p">Deskripsi aksi...</p>
  <label class="pma-form-label-mb">
    <input type="checkbox" class="pma-checkbox" />
    Opsi tambahan
  </label>
  <hr class="pma-divider" />
  <div class="pma-section-footer">
    <PmaBtn>Go</PmaBtn>
  </div>
</PmaSectionHeader>
```

#### `PmaModal.vue`
- **Props**: `show: boolean`, `title: string`, `message: string`
- **Emits**: `confirm`, `cancel`
- **Fungsi**: Digunakan untuk menampilkan konfirmasi drop tabel di sisi tengah viewport dengan backdrop gelap.

#### `PmaToast.vue`
- **Props**: `show: boolean`, `type: 'success' | 'error'`, `message: string`, `duration: number`
- **Emits**: `close`
- **Fitur**:
  - Success: Tampil di sisi kiri atas (`top-16 left-72`), dilengkapi progress bar penyusutan durasi reaktif untuk autoclose.
  - Error: Tampil di tengah layar (modal-style) secara statis tanpa animasi bergerak/bouncing.

---

### Pages (`src/components/pages/`)

#### `UserAccountsPage.vue`
- **Data**: `users[]`, `selectedIds[]`, `checkAll`, `dropDatabases`
- **Fungsi**: `toggleAll()`, `toggleRow(id)`
- **Components**: `PmaTable`, `PmaActionLink`, `PmaInfoBox`, `PmaBtn`, `PmaSectionHeader`
- **Grid**: `pma-row` + `pma-col-12` di dalam section remove user

#### `DatabaseManagerPage.vue`
- **Props**: `dbName: string (required)`
- **Fungsi/Koneksi**: `reloadTables()`, `loadTableSizes()`, `handleDropTable()`, `handleRenameTable()`, `handleCreateTable()`, `handleRunSQL()`, `handleExport()`, `handleDownloadExport()`, `handleImportExecute()`
- **Components**: `PmaTable`, `PmaActionLink`, `PmaBtn`, `PmaSectionHeader`, `PmaInfoBox`, `PmaModal`, `PmaToast`
- **Sub-Tabs**:
  1. **Structure**: List tabel dengan ukuran tabel dinamis (misal `32.0 KiB`), inline rename (`ALTER TABLE RENAME TO`), drop tabel (via `PmaModal`), dan penambahan tabel baru.
  2. **SQL**: Monaco Editor untuk eksekusi kueri reaktif lengkap dengan rendering data tabel di sisi bawah.
  3. **Export**: Ekspor tabel dalam format SQL/JSON dengan fitur salin clipboard dan unduh file (`handleDownloadExport`).
  4. **Import**: Fitur drag & drop file SQL/CSV reaktif yang membaca file dan mengeksekusi kueri berurutan (sequential splitting).
  5. **Diagram**: Placeholder visual diagram tabel relasional.

#### `TableManagerPage.vue`
- **Props**: `dbName: string (required)`, `tableName: string (required)`
- **Fungsi/Koneksi**: `fetchBrowseData()`, `startEditRow()`, `saveEditRow()`, `handleInsertRow()`, `fetchStructure()`, `fetchRelations()`, `handleAddRelation()`, `handleConfirmDeleteRelation()`
- **Components**: `PmaTable`, `PmaActionLink`, `PmaBtn`, `PmaSectionHeader`, `PmaInfoBox`, `PmaModal`, `PmaToast`
- **Sub-Tabs**:
  1. **Browse**: List data tabel reaktif dengan pencarian row search form, paginasi per halaman (10, 25, 50, 100) / show all, inline edit untuk mengupdate nilai baris data secara langsung, serta formulir input penambahan baris data baru (+ Add Row). Dilengkapi dengan konsol penampil query SQL terakhir yang dijalankan (INSERT/UPDATE), pengenalan tipe input tanggal/jam/waktu otomatis, dan opsi pengisian nilai NULL reaktif jika kolom mendukung nullable.
  2. **Structure**: Detail metadata kolom tabel (Name, Type, Collation, Attributes, Null, Default, Comments, Extra).
  3. **Relation Structure**: Relation view menampilkan daftar constraint foreign key relasi tabel, formulir penambahan constraint FK (ON DELETE / ON UPDATE actions), serta tombol drop constraint relasi.

---

## 🏗️ Pola Kombinasi CSS yang Sudah Terbukti

### Pola: Section Panel
```html
<div class="pma-section-mb">
  <PmaSectionHeader icon="⚙️" label="Judul Section">
    <p class="pma-section-p">Deskripsi.</p>
    <label class="pma-form-label-mb">
      <input type="checkbox" class="pma-checkbox" v-model="flag" />
      Opsi
    </label>
    <hr class="pma-divider" />
    <div class="pma-section-footer">
      <PmaBtn>Go</PmaBtn>
    </div>
  </PmaSectionHeader>
</div>
```

### Pola: Tabel Lengkap
```html
<PmaTable>
  <template #head>
    <tr>
      <th class="pma-th-checkbox"></th>
      <th>Nama</th>
      <th class="pma-th-narrow">Status</th>
      <th>
        Privileges
        <span class="pma-th-help-icon" title="Info">?</span>
      </th>
      <th class="pma-th-action">Action</th>
    </tr>
  </template>
  <tr v-for="row in data" :key="row.id">
    <td class="pma-th-checkbox">
      <input type="checkbox" class="pma-checkbox" />
    </td>
    <td>{{ row.name }}</td>
    <td :class="{ 'pma-cell-danger': !row.hasPassword }">
      {{ row.hasPassword ? 'Yes' : 'No' }}
    </td>
    <td class="pma-cell-privilege">{{ row.privileges }}</td>
    <td>
      <div class="pma-table-actions">
        <PmaActionLink icon="👤" label="Edit privileges" />
        <PmaActionLink icon="📤" label="Export" />
        <PmaActionLink :icon="row.locked ? '🔒' : '🔓'" :label="row.locked ? 'Lock' : 'Unlock'" />
      </div>
    </td>
  </tr>
</PmaTable>
```

### Pola: Bulk Action Bar
```html
<div class="pma-bulk-bar">
  <span class="pma-bulk-bar-arrow">↳</span>
  <label class="pma-form-label">
    <input type="checkbox" class="pma-checkbox" :checked="checkAll" @change="toggleAll" />
    Check all
  </label>
  <span class="pma-bulk-bar-label">With selected:</span>
  <PmaActionLink icon="📤" label="Export" />
</div>
```

### Pola: Form Grid 2 Kolom
```html
<div class="pma-row">
  <div class="pma-col-3">
    <label class="pma-form-label">Username</label>
  </div>
  <div class="pma-col-9">
    <input class="pma-sidebar-filter-input" type="text" />
  </div>
  <div class="pma-col-12">
    <hr class="pma-divider" />
    <div class="pma-section-footer">
      <PmaBtn type="submit">Save</PmaBtn>
    </div>
  </div>
</div>
```

### Pola: Notification Boxes
```html
<PmaInfoBox variant="warning">
  Teks peringatan. <a class="pma-box-link" href="#">Reload privileges</a>.
</PmaInfoBox>
<PmaInfoBox variant="info">
  Teks info. <a class="pma-box-link" href="#">Link</a>
</PmaInfoBox>
```

---

## 🔄 Decision Tree: Kapan Buat Apa?

```
Ada pola style yang ingin diterapkan?
│
├─ Sudah ada CSS component class? ──→ Gunakan class yang ada
│
├─ Belum ada, dipakai 2+ tempat? ──→ Tambahkan ke @layer components di style.css
│
├─ Hanya dipakai 1 tempat? ─────── → Gunakan CSS class terdekat yang ada
│
└─ Ada logika reaktif (v-model, emit, slot, kondisi)?
   ├─ Ya, dipakai 2+ tempat? ─────→ Buat Vue component baru di ui/
   └─ Ya, hanya 1 tempat? ────────→ Tulis langsung di page component
```

**DILARANG (Salah):**
```html
<!-- inline style -->
<div style="margin-bottom: 20px; display: flex; justify-content: flex-end;">

<!-- Tailwind ad-hoc untuk pola yang sudah ada -->
<div class="flex justify-end mb-5 text-[11px] text-gray-700">

<!-- warna hardcoded -->
<span style="color: #cc3399;">ALL PRIVILEGES</span>
```

**BENAR:**
```html
<!-- CSS component class -->
<div class="pma-section-mb">
  <div class="pma-section-footer">

<!-- cell variant yang sudah ada -->
<td class="pma-cell-privilege">ALL PRIVILEGES</td>

<!-- komponen reusable -->
<PmaBtn @click="save">Save</PmaBtn>
```

---

## ✍️ Panduan Menambahkan Sesuatu yang Baru

### Tambah CSS Component
1. Buka `src/style.css`
2. Tambahkan di dalam `@layer components { ... }` sebelum baris `} /* end @layer components */`
3. Nama: prefix `pma-` + nama semantik
4. Warna: selalu gunakan `var(--color-pma-...)` bukan nilai hardcoded
5. **Update tabel di dokumen ini** bagian CSS Components Catalog

### Tambah Vue Component
1. Folder: `layout/` (struktural), `ui/` (reusable), `pages/` (halaman)
2. Nama: `AppXxx.vue` (layout), `PmaXxx.vue` (ui), `XxxPage.vue` (pages)
3. JSDoc comment mendefinisikan props + emits
4. Zero `style=""` inline
5. **Update tabel di dokumen ini** bagian Vue Components Catalog

### Tambah Design Token
1. Buka `src/style.css`, blok `@theme { ... }`
2. Prefix sesuai kategori:
   - Warna: `--color-pma-...`
   - Font: `--font-size-pma-...`
   - Spacing: `--spacing-pma-...`
   - Radius: `--radius-pma-...`
   - Dimensi: `--width-pma-...`
3. **Update tabel di dokumen ini** bagian Design Tokens

---

## 🛠️ Tech Stack

| Layer | Teknologi | Versi |
|---|---|---|
| Framework | Tauri + Vue 3 | Vue ^3.5.13, Tauri ^2 |
| Build | Vite | ^6.0.3 |
| CSS | Tailwind CSS v4 | via `@tailwindcss/vite` |
| CSS Pattern | CSS Custom Properties + `@layer components` | — |
| State | Vue `ref()` / `reactive()` | Composition API |

> NOTE: Tailwind CSS v4 tidak memerlukan `tailwind.config.js`. Semua konfigurasi ada di `@theme {}` dalam `style.css`. Gunakan utility Tailwind hanya untuk hal yang belum ada CSS component-nya.
