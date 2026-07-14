<script setup>
/**
 * AppTabNav.vue
 * Tab navigation bar — phpMyAdmin style tabs
 * Props:
 *   - tabs: Array<{ id: string, icon: string, label: string }>
 *   - activeTab: string — id tab yang aktif
 * Emits:
 *   - tab-change: string — id tab yang diklik
 */
const props = defineProps({
  tabs: {
    type: Array,
    default: () => [
      { id: "databases",   icon: "📂", label: "Databases" },
      { id: "sql",         icon: "📄", label: "SQL" },
      { id: "status",      icon: "📊", label: "Status" },
      { id: "users",       icon: "👥", label: "User accounts" },
      { id: "export",      icon: "📤", label: "Export" },
      { id: "import",      icon: "📥", label: "Import" },
      { id: "settings",    icon: "⚙️",  label: "Settings" },
      { id: "binlog",      icon: "📜", label: "Binary log" },
      { id: "replication", icon: "🔄", label: "Replication" },
      { id: "variables",   icon: "🏷️",  label: "Variables" },
      { id: "charsets",    icon: "🔠", label: "Charsets" },
      { id: "engines",     icon: "⚙️",  label: "Engines" },
      { id: "plugins",     icon: "🧩", label: "Plugins" },
    ],
  },
  activeTab: {
    type: String,
    default: "users",
  },
});

const emit = defineEmits(["tab-change"]);
</script>

<template>
  <nav class="pma-tab-nav" role="tablist">
    <div
      v-for="tab in tabs"
      :key="tab.id"
      class="pma-tab"
      :class="{ active: activeTab === tab.id }"
      role="tab"
      :aria-selected="activeTab === tab.id"
      :tabindex="activeTab === tab.id ? 0 : -1"
      @click="emit('tab-change', tab.id)"
    >
      <span class="pma-tab-icon">{{ tab.icon }}</span>
      <span>{{ tab.label }}</span>
    </div>
  </nav>
</template>
