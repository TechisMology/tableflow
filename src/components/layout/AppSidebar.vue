<script setup>
/**
 * AppSidebar.vue
 * Left sidebar: Logo, server selector, quick nav, filter, database tree
 * Props:
 *   - databases: Array<{ name: string, expanded?: boolean, italic?: boolean }>
 *   - currentServer: string
 */
const props = defineProps({
  databases: {
    type: Array,
    default: () => [
      { name: "New", icon: "➖", expanded: true },
      { name: "alexpalautog", icon: "➕" },
      { name: "AngelicaAncheta", icon: "➕" },
      { name: "Briannn", icon: "➕" },
      { name: "gymnastics_db", icon: "➕" },
      { name: "Information", icon: "➕", italic: true },
      { name: "information_schema", icon: "➕", italic: true },
      { name: "mysql", icon: "➕" },
      { name: "performance_schema", icon: "➕" },
      { name: "Portfolio_Marcus", icon: "➕" },
      { name: "PRUEBA", icon: "➕" },
      { name: "pw", icon: "➕" },
      { name: "sakila", icon: "➕" },
      { name: "serveruwebcoBressolia", icon: "➕" },
      { name: "shoe_db", icon: "➕" },
      { name: "sklepSerohenko", icon: "➕" },
      { name: "sku", icon: "➕" },
      { name: "st", icon: "➕" },
    ],
  },
  currentServer: {
    type: String,
    default: "phpMyAdmin demo - MySQL",
  },
});

const emit = defineEmits(["db-select", "server-change"]);

const icons = [
  { emoji: "🏠", title: "Home" },
  { emoji: "📁", title: "Databases" },
  { emoji: "❓", title: "Help" },
  { emoji: "⚙️", title: "Settings" },
  { emoji: "🔄", title: "Reload" },
];
</script>

<template>
  <aside class="pma-sidebar">
    <!-- Logo Area -->
    <div class="pma-sidebar-logo-area">
      <img
        alt="phpMyAdmin Logo"
        class="pma-sidebar-logo"
        src="https://lh3.googleusercontent.com/aida-public/AB6AXuCqjwTU23RFk2NNG6jNQmUap7eQuurQstxABH9QdMhE3JotA8jBRqoe2ChuM3tD3hDV15bSO_-_GIvfDOuj3X6jWhwI76GBVTnJjswm-Un9KVcb98IyywO-ss2EkDQmXBZny80hB7eLzcx4mChN-ldE6if_D2yLNA00ZI2ATmUHBAKJrmhsNfDsnPUq8eZfuV3ml7UzaE9VpIS0v1thnfQARmEcRCYFq69zgE2F7EBFr3vkRv-IPXT4TVjPpqWwJoPj3alS5F65Bn1w"
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

    <!-- Database Tree -->
    <div class="pma-sidebar-tree">
      <div
        v-for="db in databases"
        :key="db.name"
        class="pma-db-item"
        :class="{ italic: db.italic }"
        @click="emit('db-select', db.name)"
      >
        <span class="pma-db-item-icon">{{ db.icon }}</span>
        <span>{{ db.name }}</span>
      </div>
    </div>
  </aside>
</template>
