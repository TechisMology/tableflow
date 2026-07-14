<script setup>
/**
 * AppSidebar.vue
 * Left sidebar: Logo, server selector, quick nav, filter, database tree with tables
 * Props:
 *   - sidebarDbs: Array<{ name: string, expanded: boolean, tables: string[], loading: boolean }>
 *   - currentServer: string
 */
import { useConnectionStore } from "../../stores/connections";

const props = defineProps({
  sidebarDbs: {
    type: Array,
    default: () => [],
  },
  currentServer: {
    type: String,
    default: "phpMyAdmin demo - MySQL",
  },
});

const emit = defineEmits(["db-select", "table-select", "server-change", "db-settings"]);

const connectionStore = useConnectionStore();

const icons = [
  { emoji: "🏠", title: "Home" },
  { emoji: "📁", title: "Databases" },
  { emoji: "❓", title: "Help" },
  { emoji: "⚙️", title: "Settings" },
  { emoji: "🔄", title: "Reload" },
];

function handleDbClick(dbName) {
  connectionStore.toggleDbExpanded(dbName);
  emit("db-select", dbName);
}

function handleTableClick(dbName, tableName) {
  emit("table-select", { database: dbName, table: tableName });
}
</script>

<template>
  <aside class="pma-sidebar">
    <!-- Logo Area -->
    <div class="pma-sidebar-logo-area">
      <img
        alt="Tableflow Logo"
        class="pma-sidebar-logo"
        src="../../assets/logo-forapp.png"
      />
      <!-- Icon Toolbar -->
      <div class="pma-sidebar-icon-bar">
        <span
          v-for="icon in icons"
          :key="icon.title"
          class="pma-sidebar-icon"
          :title="icon.title"
        >{{ icon.emoji }}</span>
      </div>
      <!-- Server Label -->
      <div class="pma-sidebar-server-label">Current server:</div>
      <!-- Server Selector -->
      <select
        class="pma-sidebar-server-select"
        :value="currentServer"
        @change="emit('server-change', $event.target.value)"
      >
        <option>{{ currentServer }}</option>
      </select>
    </div>

    <!-- Quick Buttons -->
    <div class="pma-sidebar-quick-btns">
      <button class="pma-quick-btn">Recent</button>
      <button class="pma-quick-btn">Favorites</button>
    </div>

    <!-- Filter Input -->
    <div class="pma-sidebar-filter">
      <input
        class="pma-sidebar-filter-input"
        type="text"
        placeholder="Type to filter these, Enter to search all"
      />
    </div>

    <!-- Database Tree with Tables -->
    <div class="pma-sidebar-tree">
      <div v-if="sidebarDbs.length === 0" class="pma-db-item italic">
        No databases opened
      </div>
      
      <div v-for="db in sidebarDbs" :key="db.name" class="flex flex-col">
        <!-- Database Header Row -->
        <div 
          class="pma-db-item flex items-center justify-between cursor-pointer"
          @click="handleDbClick(db.name)"
        >
          <div class="flex items-center gap-1 overflow-hidden text-ellipsis whitespace-nowrap">
            <!-- Expand/Collapse toggle icon -->
            <span class="pma-db-item-icon text-[8px] w-3 inline-block">
              {{ db.expanded ? '▼' : '▶' }}
            </span>
            <span>🗄️</span>
            <span class="font-semibold">{{ db.name }}</span>
          </div>

          <!-- Controls: Settings + Close -->
          <div class="flex gap-1 items-center" @click.stop>
            <button 
              title="Database Settings"
              class="bg-none border-none p-0.5 cursor-pointer text-[10px]"
              @click="emit('db-settings', db.name)"
            >
              ⚙️
            </button>
            <button 
              title="Close Database"
              class="bg-none border-none p-0.5 cursor-pointer text-[10px] text-[var(--color-pma-text-danger)]"
              @click="connectionStore.closeSidebarDb(db.name)"
            >
              ✕
            </button>
          </div>
        </div>

        <!-- Sub-tables list if expanded -->
        <div v-if="db.expanded" class="pl-4">
          <div v-if="db.loading" class="pma-db-item italic text-[10px] text-[var(--color-pma-text-muted)]">
            Loading tables...
          </div>
          <div v-else-if="db.tables.length === 0" class="pma-db-item italic text-[10px] text-[var(--color-pma-text-muted)]">
            No tables found
          </div>
          <div
            v-else
            v-for="table in db.tables"
            :key="table"
            class="pma-db-item py-0.5 px-1"
            @click="handleTableClick(db.name, table)"
          >
            <span class="pma-db-item-icon">📄</span>
            <span>{{ table }}</span>
          </div>
        </div>
      </div>
    </div>
  </aside>
</template>
