<script setup>
/**
 * App.vue — Root layout
 * Susunan: AppSidebar (kiri) + pma-main (tengah) + AppDatabasePanel (kanan, show/hide)
 */
import { ref, computed, watch } from "vue";
import AppSidebar from "./components/layout/AppSidebar.vue";
import ConnectionSidebar from "./components/layout/ConnectionSidebar.vue";
import AppTopHeader from "./components/layout/AppTopHeader.vue";
import AppTabNav from "./components/layout/AppTabNav.vue";
import AppConsoleFooter from "./components/layout/AppConsoleFooter.vue";
import AppDatabasePanel from "./components/layout/AppDatabasePanel.vue";
import UserAccountsPage from "./components/pages/UserAccountsPage.vue";
import ConnectionForm from "./components/pages/ConnectionForm.vue";
import { useConnectionStore } from "./stores/connections";

const connectionStore = useConnectionStore();

// ── State ──────────────────────────────────────────────────
const activeTab = ref("users");
const dbPanelOpen = ref(true);
const activeDb = ref("sakila");

const editConnectionId = ref(null);
const forceConnectionManager = ref(true);

const activeConnection = computed(() => connectionStore.activeConnection);
const showConnectionForm = computed(() => !activeConnection.value || forceConnectionManager.value);
const serverName = computed(() => activeConnection.value ? activeConnection.value.name : "Not Connected");

function onTabChange(tabId) {
  activeTab.value = tabId;
}

function onDbSelect(name) {
  activeDb.value = name;
}

function handleSelectConnection(conn) {
  forceConnectionManager.value = false;
  editConnectionId.value = null;
}

function handleEditConnection(id) {
  editConnectionId.value = id;
  forceConnectionManager.value = true;
}

function handleCreateConnection() {
  editConnectionId.value = null;
  forceConnectionManager.value = true;
}

function handleDisconnect() {
  connectionStore.setActiveConnection(null);
  forceConnectionManager.value = true;
}

// Watch activeConnection
watch(activeConnection, async (newConn) => {
  if (newConn) {
    try {
      await connectionStore.loadDatabases(newConn);
      connectionStore.initSidebarDbs([]);
      const defaultDb = newConn.database || "";
      activeDb.value = defaultDb;
      if (defaultDb) {
        await connectionStore.openSidebarDb(defaultDb);
      }
    } catch (e) {
      console.error("Error loading databases for active connection", e);
    }
  } else {
    connectionStore.databases = [];
    connectionStore.sidebarDbs = [];
    activeDb.value = "";
  }
}, { immediate: true });

// Watch activeDb
watch(activeDb, async (newDb) => {
  if (activeConnection.value && newDb) {
    try {
      await connectionStore.openSidebarDb(newDb);
    } catch (e) {
      console.error("Error opening database in sidebar", newDb, e);
    }
  }
});
</script>

<template>
  <!-- Root: sidebar kiri + main + panel kanan -->
  <div class="pma-app">

    <!-- Sidebar Kiri: Tampilkan ConnectionSidebar jika form ditampilkan, atau AppSidebar jika sudah terhubung -->
    <ConnectionSidebar 
      v-if="showConnectionForm"
      :active-connection-id="activeConnection?.id"
      @select-connection="handleSelectConnection"
      @edit-connection="handleEditConnection"
      @create-connection="handleCreateConnection"
    />
    <AppSidebar 
      v-else 
      :current-server="serverName"
      :sidebar-dbs="connectionStore.sidebarDbs"
      @db-select="onDbSelect"
    />

    <!-- Area Konten Utama -->
    <main class="pma-main">

      <!-- Top Header — dengan toggle DB panel -->
      <AppTopHeader
        :server-name="serverName"
        :db-panel-open="dbPanelOpen"
        @toggle-db-panel="dbPanelOpen = !dbPanelOpen"
      >
        <!-- Tombol Tambahan untuk Manajemen Koneksi -->
        <button 
          v-if="activeConnection" 
          class="pma-header-action-btn"
          style="display: flex; align-items: center; gap: 4px; font-size: 10px;"
          @click="forceConnectionManager = !forceConnectionManager"
        >
          <span>🔌</span>
          <span>{{ forceConnectionManager ? "Go to DB" : "Connections" }}</span>
        </button>
        <button 
          v-if="activeConnection" 
          class="pma-header-action-btn"
          style="display: flex; align-items: center; gap: 4px; font-size: 10px; color: var(--color-pma-text-danger);"
          @click="handleDisconnect"
        >
          <span>🚪</span>
          <span>Disconnect</span>
        </button>
      </AppTopHeader>

      <!-- Connection Form jika tidak ada koneksi / dipaksa ke manager, atau Tab views jika terhubung -->
      <template v-if="showConnectionForm">
        <ConnectionForm 
          :edit-connection-id="editConnectionId"
          @saved="forceConnectionManager = false"
          @cancel="forceConnectionManager = false"
        />
      </template>
      <template v-else>
        <!-- Tab Navigation -->
        <AppTabNav :active-tab="activeTab" @tab-change="onTabChange" />

        <!-- Konten Halaman -->
        <div class="pma-page-area">
          <UserAccountsPage v-if="activeTab === 'users'" />

          <!-- Placeholder untuk tab lain -->
          <div v-else class="pma-content pma-page-placeholder">
            Tab "{{ activeTab }}" — belum diimplementasi.
          </div>
        </div>

        <!-- Console Footer (fixed) dengan Monaco SQL -->
        <AppConsoleFooter :active-db="activeDb" />
      </template>

    </main>

    <!-- Database Panel Kanan (show/hide) jika terhubung -->
    <AppDatabasePanel
      v-if="!showConnectionForm"
      :open="dbPanelOpen"
      :active-db="activeDb"
      :databases="connectionStore.databases"
      @close="dbPanelOpen = false"
      @db-select="onDbSelect"
    />

  </div>
</template>
