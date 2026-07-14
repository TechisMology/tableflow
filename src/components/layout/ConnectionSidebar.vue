<script setup>
/**
 * ConnectionSidebar.vue
 * Sidebar untuk mengelola dan memilih koneksi database.
 */
import { onMounted } from "vue";
import { useConnectionStore } from "../../stores/connections";

const props = defineProps({
  activeConnectionId: {
    type: String,
    default: null,
  },
});

const emit = defineEmits(["select-connection", "edit-connection", "create-connection"]);

const connectionStore = useConnectionStore();

onMounted(() => {
  connectionStore.loadConnections();
});

function handleSelect(conn) {
  connectionStore.setActiveConnection(conn);
  emit("select-connection", conn);
}

function handleEdit(conn, event) {
  event.stopPropagation();
  emit("edit-connection", conn.id);
}

async function handleDelete(conn, event) {
  event.stopPropagation();
  if (confirm(`Are you sure you want to delete connection "${conn.name}"?`)) {
    try {
      await connectionStore.deleteConnection(conn.id);
    } catch (err) {
      alert(`Error deleting connection: ${err}`);
    }
  }
}
</script>

<template>
  <aside class="pma-sidebar">
    <!-- Logo & Header Area -->
    <div class="pma-sidebar-logo-area">
      <img
        alt="Tableflow Logo"
        class="pma-sidebar-logo"
        src="../../assets/logo-forapp.png"
      />
      
      <div class="pma-sidebar-server-label" style="margin-top: 12px; font-weight: bold; font-size: 12px;">
        DATABASE CONNECTIONS
      </div>
    </div>

    <!-- Quick Action / Add Connection -->
    <div class="pma-sidebar-quick-btns" style="padding: 6px 12px;">
      <button 
        class="pma-quick-btn" 
        style="width: 100%; text-align: center; font-weight: bold;"
        @click="emit('create-connection')"
      >
        ➕ New Connection
      </button>
    </div>

    <!-- Connection List -->
    <div class="pma-sidebar-tree" style="flex: 1; overflow-y: auto;">
      <div v-if="connectionStore.loading" style="padding: 12px; color: var(--color-pma-text-muted); font-style: italic;">
        Loading connections...
      </div>
      <div v-else-if="connectionStore.connections.length === 0" style="padding: 12px; color: var(--color-pma-text-muted); font-style: italic;">
        No connections saved.
      </div>
      <div
        v-for="conn in connectionStore.connections"
        :key="conn.id"
        class="pma-db-item"
        :class="{ active: activeConnectionId === conn.id }"
        style="display: flex; justify-content: space-between; align-items: center; padding: var(--spacing-pma-md) var(--spacing-pma-lg); cursor: pointer;"
        @click="handleSelect(conn)"
      >
        <div style="display: flex; align-items: center; gap: 6px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
          <span class="pma-db-item-icon">
            {{ conn.driver === 'sqlite' ? '📁' : '🛢️' }}
          </span>
          <span :style="{ fontWeight: activeConnectionId === conn.id ? 'bold' : 'normal' }">
            {{ conn.name }}
          </span>
        </div>
        
        <div class="connection-actions" style="display: flex; gap: 4px; align-items: center;">
          <button 
            title="Edit Connection"
            style="background: none; border: none; padding: 2px; cursor: pointer;"
            @click="handleEdit(conn, $event)"
          >
            ✏️
          </button>
          <button 
            title="Delete Connection"
            style="background: none; border: none; padding: 2px; cursor: pointer; color: var(--color-pma-text-danger);"
            @click="handleDelete(conn, $event)"
          >
            🗑️
          </button>
        </div>
      </div>
    </div>
  </aside>
</template>
