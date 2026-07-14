<script setup>
/**
 * AppDatabasePanel.vue
 * Panel kanan — daftar database lengkap (show/hide).
 * Setara dengan fitur "Databases" di phpMyAdmin.
 *
 * Props:
 *   - open: boolean — apakah panel tampil atau tersembunyi
 *   - activeDb: string — nama database yang sedang aktif
 *
 * Emits:
 *   - close          — saat tombol X diklik
 *   - db-select(name) — saat baris database diklik
 */
import { ref, computed } from "vue";

const props = defineProps({
  open: {
    type: Boolean,
    default: true,
  },
  activeDb: {
    type: String,
    default: "",
  },
});

const emit = defineEmits(["close", "db-select"]);

// ── Data Dummy Databases ────────────────────────────────────
// Representasi daftar database seperti di phpMyAdmin
const databases = ref([
  {
    name: "information_schema",
    tables: 79,
    size: "176 KiB",
    charset: "utf8mb3",
    collation: "utf8mb3_general_ci",
    system: true,
  },
  {
    name: "mysql",
    tables: 37,
    size: "2.5 MiB",
    charset: "utf8mb4",
    collation: "utf8mb4_general_ci",
    system: true,
  },
  {
    name: "performance_schema",
    tables: 110,
    size: "0 B",
    charset: "utf8mb4",
    collation: "utf8mb4_general_ci",
    system: true,
  },
  {
    name: "sys",
    tables: 101,
    size: "176 KiB",
    charset: "utf8mb4",
    collation: "utf8mb4_general_ci",
    system: true,
  },
  {
    name: "alexpalautog",
    tables: 8,
    size: "1.2 MiB",
    charset: "utf8mb4",
    collation: "utf8mb4_unicode_ci",
    system: false,
  },
  {
    name: "AngelicaAncheta",
    tables: 12,
    size: "3.4 MiB",
    charset: "utf8mb4",
    collation: "utf8mb4_unicode_ci",
    system: false,
  },
  {
    name: "Briannn",
    tables: 5,
    size: "512 KiB",
    charset: "utf8mb4",
    collation: "utf8mb4_unicode_ci",
    system: false,
  },
  {
    name: "gymnastics_db",
    tables: 15,
    size: "8.7 MiB",
    charset: "utf8mb4",
    collation: "utf8mb4_unicode_ci",
    system: false,
  },
  {
    name: "Portfolio_Marcus",
    tables: 22,
    size: "14.1 MiB",
    charset: "utf8mb4",
    collation: "utf8mb4_unicode_ci",
    system: false,
  },
  {
    name: "PRUEBA",
    tables: 3,
    size: "96 KiB",
    charset: "latin1",
    collation: "latin1_swedish_ci",
    system: false,
  },
  {
    name: "pw",
    tables: 7,
    size: "1.8 MiB",
    charset: "utf8mb4",
    collation: "utf8mb4_general_ci",
    system: false,
  },
  {
    name: "sakila",
    tables: 23,
    size: "4.2 MiB",
    charset: "utf8mb4",
    collation: "utf8mb4_general_ci",
    system: false,
  },
  {
    name: "serveruwebcoBressolia",
    tables: 18,
    size: "6.3 MiB",
    charset: "utf8mb4",
    collation: "utf8mb4_unicode_ci",
    system: false,
  },
  {
    name: "shoe_db",
    tables: 9,
    size: "2.9 MiB",
    charset: "utf8mb4",
    collation: "utf8mb4_unicode_ci",
    system: false,
  },
  {
    name: "sklepSerohenko",
    tables: 11,
    size: "5.1 MiB",
    charset: "utf8mb4",
    collation: "utf8mb4_polish_ci",
    system: false,
  },
  {
    name: "sku",
    tables: 4,
    size: "256 KiB",
    charset: "utf8mb4",
    collation: "utf8mb4_general_ci",
    system: false,
  },
  {
    name: "st",
    tables: 6,
    size: "784 KiB",
    charset: "utf8mb4",
    collation: "utf8mb4_general_ci",
    system: false,
  },
  {
    name: "students2020",
    tables: 14,
    size: "9.6 MiB",
    charset: "utf8mb4",
    collation: "utf8mb4_unicode_ci",
    system: false,
  },
]);

// ── Search/Filter ────────────────────────────────────────────
const searchQuery = ref("");

const filteredDatabases = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  if (!q) return databases.value;
  return databases.value.filter((db) =>
    db.name.toLowerCase().includes(q)
  );
});

// ── Stats ────────────────────────────────────────────────────
const totalDbs = computed(() => databases.value.length);
const userDbs = computed(() => databases.value.filter((d) => !d.system).length);
const filteredCount = computed(() => filteredDatabases.value.length);
</script>

<template>
  <!-- Panel Container — .closed mematikan animasi & pointer events -->
  <aside class="pma-db-panel" :class="{ closed: !open }">

    <!-- Panel Header -->
    <div class="pma-db-panel-header">
      <div class="pma-db-panel-title">
        <span>🗄️</span>
        <span>Databases</span>
      </div>
      <button class="pma-db-panel-close" title="Hide panel" @click="emit('close')">
        ✕
      </button>
    </div>

    <!-- Search -->
    <div class="pma-db-panel-search">
      <input
        v-model="searchQuery"
        class="pma-db-panel-search-input"
        type="text"
        placeholder="Filter databases..."
      />
    </div>

    <!-- Stats Bar -->
    <div class="pma-db-panel-stats">
      <span>Showing {{ filteredCount }} of {{ totalDbs }}</span>
      <span class="pma-db-panel-badge">{{ userDbs }} user</span>
      <span class="pma-db-panel-badge">{{ totalDbs - userDbs }} system</span>
    </div>

    <!-- Column Header (sticky) -->
    <div class="pma-db-panel-list">
      <div class="pma-db-panel-list-head">
        <span class="pma-db-panel-row-icon">📁</span>
        <span class="pma-db-panel-list-head-name">Database</span>
        <span class="pma-db-panel-list-head-tables">Tables</span>
        <span class="pma-db-panel-list-head-size">Size</span>
      </div>

      <!-- Database Rows -->
      <div
        v-for="db in filteredDatabases"
        :key="db.name"
        class="pma-db-panel-row"
        :class="{ active: activeDb === db.name }"
        :title="`${db.name} — ${db.collation}`"
        @click="emit('db-select', db.name)"
      >
        <span class="pma-db-panel-row-icon">🗄️</span>
        <span
          class="pma-db-panel-row-name"
          :class="{ italic: db.system }"
        >{{ db.name }}</span>
        <span class="pma-db-panel-row-tables">{{ db.tables }}</span>
        <span class="pma-db-panel-row-size">{{ db.size }}</span>
      </div>

      <!-- Empty State -->
      <div v-if="filteredDatabases.length === 0" class="pma-db-panel-stats">
        No databases match "{{ searchQuery }}"
      </div>
    </div>

    <!-- Panel Footer -->
    <div class="pma-db-panel-footer">
      <span>Total: {{ totalDbs }} databases</span>
      <span class="pma-db-panel-badge">MySQL</span>
    </div>

  </aside>
</template>
