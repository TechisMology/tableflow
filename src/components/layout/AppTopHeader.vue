<script setup>
/**
 * AppTopHeader.vue
 * Top header bar: breadcrumb kiri + action buttons kanan.
 * Menyertakan tombol toggle Database Panel.
 *
 * Props:
 *   - serverName: string  — nama server yang aktif
 *   - dbPanelOpen: boolean — apakah database panel sedang terbuka
 *
 * Emits:
 *   - user-profile
 *   - logout
 *   - toggle-db-panel — untuk show/hide panel kanan
 */
defineProps({
  serverName: {
    type: String,
    default: "phpMyAdmin demo - MySQL",
  },
  dbPanelOpen: {
    type: Boolean,
    default: true,
  },
});

const emit = defineEmits(["user-profile", "logout", "toggle-db-panel"]);
</script>

<template>
  <header class="pma-header">
    <!-- Left: Back + Server Info -->
    <div class="pma-header-left">
      <span class="pma-header-back" title="Back">←</span>
      <div class="pma-header-server-info">
        <span>🖥️</span>
        <span>Server: {{ serverName }}</span>
      </div>
    </div>

    <!-- Right: Toggle DB Panel + Action Buttons -->
    <div class="pma-header-right">
      <slot />

      <!-- Toggle Database Panel -->
      <button
        class="pma-db-panel-toggle"
        :class="{ active: dbPanelOpen }"
        :title="dbPanelOpen ? 'Hide database panel' : 'Show database panel'"
        @click="emit('toggle-db-panel')"
      >
        <span>🗄️</span>
        <span>{{ dbPanelOpen ? 'Hide DB' : 'Show DB' }}</span>
      </button>

      <!-- User Profile -->
      <button
        class="pma-header-action-btn"
        title="User profile"
        @click="emit('user-profile')"
      >
        👤
      </button>

      <!-- Logout -->
      <button
        class="pma-header-action-btn"
        title="Logout"
        @click="emit('logout')"
      >
        🚪
      </button>
    </div>
  </header>
</template>
