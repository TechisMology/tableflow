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
  databases: {
    type: Array,
    default: () => [],
  },
});

const emit = defineEmits(["close", "db-select"]);

const databasesList = computed(() => {
  return props.databases.map((dbName) => {
    const isSystem = ["information_schema", "mysql", "performance_schema", "sys", "postgres"].includes(dbName.toLowerCase());
    return {
      name: dbName,
      tables: "-",
      size: "-",
      system: isSystem,
    };
  });
});

// ── Search/Filter ────────────────────────────────────────────
const searchQuery = ref("");

const filteredDatabases = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  if (!q) return databasesList.value;
  return databasesList.value.filter((db) =>
    db.name.toLowerCase().includes(q)
  );
});

// ── Stats ────────────────────────────────────────────────────
const totalDbs = computed(() => databasesList.value.length);
const userDbs = computed(() => databasesList.value.filter((d) => !d.system).length);
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
