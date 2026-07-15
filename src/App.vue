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
import ConnectionForm from "./components/pages/ConnectionForm.vue";
import DatabaseManagerPage from "./components/pages/DatabaseManagerPage.vue";
import TableManagerPage from "./components/pages/TableManagerPage.vue";
import PmaSectionHeader from "./components/ui/PmaSectionHeader.vue";
import PmaInfoBox from "./components/ui/PmaInfoBox.vue";
import PmaBtn from "./components/ui/PmaBtn.vue";
import PmaToast from "./components/ui/PmaToast.vue";
import { useConnectionStore } from "./stores/connections";
import { invoke } from "@tauri-apps/api/core";

const connectionStore = useConnectionStore();

// ── Toast State ──
const toastShow = ref(false);
const toastType = ref("success");
const toastMsg = ref("");

function triggerToast(type, msg) {
  toastType.value = type;
  toastMsg.value = msg;
  toastShow.value = true;
}

// ── State ──────────────────────────────────────────────────
const activeTab = ref("");
const dbPanelOpen = ref(true);
const activeDb = ref("sakila");

const tabs = ref([]);

const editConnectionId = ref(null);
const forceConnectionManager = ref(true);

const newDbName = ref("");
const creatingDb = ref(false);
const createDbError = ref(null);

function quoteIdentifier(name, driver) {
  if (driver === "postgres") {
    return `"${name.replace(/"/g, '""')}"`;
  }
  return `\`${name.replace(/`/g, "``")}\``;
}

async function handleCreateDatabase() {
  const dbNameInput = newDbName.value.trim();
  if (!dbNameInput) return;

  creatingDb.value = true;
  createDbError.value = null;

  try {
    const driver = activeConnection.value?.driver;
    if (driver === "sqlite") {
      throw new Error("SQLite connections do not support creating separate databases under a single server instance.");
    }

    let password = null;
    try {
      password = await invoke("get_connection_password", {
        id: activeConnection.value.id,
      });
    } catch (e) {}

    const sql = `CREATE DATABASE ${quoteIdentifier(dbNameInput, driver)}`;

    await invoke("execute_connection_query", {
      conn: activeConnection.value,
      password,
      query: sql,
      database: driver === "postgres" ? "postgres" : "",
    });

    newDbName.value = "";
    triggerToast("success", `Database "${dbNameInput}" created successfully!`);
    await connectionStore.loadDatabases(activeConnection.value);
  } catch (err) {
    createDbError.value = String(err);
    triggerToast("error", `Error creating database: ${err}`);
  } finally {
    creatingDb.value = false;
  }
}

const activeConnection = computed(() => connectionStore.activeConnection);
const showConnectionForm = computed(
  () => !activeConnection.value || forceConnectionManager.value,
);
const serverName = computed(() =>
  activeConnection.value ? activeConnection.value.name : "Not Connected",
);

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
  tabs.value = [];
}

function handleDbSettings(dbName) {
  const tabId = `db-${dbName}`;
  const exists = tabs.value.find((t) => t.id === tabId);
  if (!exists) {
    tabs.value.push({
      id: tabId,
      icon: "⚙️",
      label: `DB: ${dbName}`,
      dbName,
      closable: true,
    });
  }
  activeTab.value = tabId;
}

function handleTableSelect({ database, table }) {
  const tabId = `table-${database}-${table}`;
  const exists = tabs.value.find((t) => t.id === tabId);
  if (!exists) {
    tabs.value.push({
      id: tabId,
      icon: "📊",
      label: `Table: ${table}`,
      dbName: database,
      tableName: table,
      closable: true,
    });
  }
  activeTab.value = tabId;
}

function handleTabClose(tabId) {
  const idx = tabs.value.findIndex((t) => t.id === tabId);
  if (idx !== -1) {
    tabs.value.splice(idx, 1);
    if (activeTab.value === tabId) {
      activeTab.value = tabs.value[idx - 1]?.id || tabs.value[0]?.id || "";
    }
  }
}

// Watch activeConnection
watch(
  activeConnection,
  async (newConn) => {
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
  },
  { immediate: true },
);

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
      @db-settings="handleDbSettings"
      @table-select="handleTableSelect"
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
          style="display: flex; align-items: center; gap: 4px; font-size: 10px"
          @click="forceConnectionManager = !forceConnectionManager"
        >
          <span>🔌</span>
          <span>{{ forceConnectionManager ? "Go to DB" : "Connections" }}</span>
        </button>
        <button
          v-if="activeConnection"
          class="pma-header-action-btn"
          style="
            display: flex;
            align-items: center;
            gap: 4px;
            font-size: 10px;
            color: var(--color-pma-text-danger);
          "
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
        <AppTabNav
          :active-tab="activeTab"
          :tabs="tabs"
          @tab-change="onTabChange"
          @tab-close="handleTabClose"
        />

        <!-- Konten Halaman -->
        <div class="pma-page-area">
          <DatabaseManagerPage
            v-slot="{ dbName }"
            v-if="activeTab.startsWith('db-')"
            :db-name="tabs.find((t) => t.id === activeTab)?.dbName"
          />
          <TableManagerPage
            v-else-if="activeTab.startsWith('table-')"
            :db-name="tabs.find((t) => t.id === activeTab)?.dbName"
            :table-name="tabs.find((t) => t.id === activeTab)?.tableName"
          />

          <!-- Dashboard if no tabs are open -->
          <div v-else-if="tabs.length === 0" class="pma-content flex flex-col">
            <div class="pma-section-mb">
              <PmaSectionHeader icon="🏠" label="Welcome to Tableflow">
                <p class="pma-section-p font-bold text-lg">
                  Tableflow — Modern Database Client
                </p>
                <p class="pma-section-p">
                  Connected to server:
                  <strong class="text-[var(--color-pma-blue)]">{{
                    activeConnection?.name
                  }}</strong>
                </p>

                <div class="pma-row gap-y-4">
                  <!-- Server Details Box -->
                  <div class="pma-col-6">
                    <PmaInfoBox variant="info">
                      <div class="flex flex-col gap-1.5 text-[11px]">
                        <strong class="text-[12px] block mb-1"
                          >🖥️ Database Server</strong
                        >
                        <div>
                          <span class="font-bold">Server Type:</span>
                          {{
                            activeConnection?.driver === "mysql"
                              ? "MySQL"
                              : activeConnection?.driver === "postgres"
                                ? "PostgreSQL"
                                : "SQLite"
                          }}
                        </div>
                        <div>
                          <span class="font-bold">Server Connection:</span>
                          Localhost via UNIX socket / TCP/IP
                        </div>
                        <div>
                          <span class="font-bold">Server Version:</span>
                          {{
                            activeConnection?.driver === "mysql"
                              ? "10.4.24-MariaDB"
                              : activeConnection?.driver === "postgres"
                                ? "PostgreSQL 16.1"
                                : "SQLite 3.42.0"
                          }}
                        </div>
                        <div>
                          <span class="font-bold">User:</span>
                          {{ activeConnection?.username || "default" }}@{{
                            activeConnection?.host || "localhost"
                          }}
                        </div>
                        <div>
                          <span class="font-bold">SSL / TLS Mode:</span>
                          <span
                            :class="
                              activeConnection?.ssl_enabled
                                ? 'text-green-600 font-bold'
                                : 'text-gray-500'
                            "
                            >{{
                              activeConnection?.ssl_enabled
                                ? "Enabled"
                                : "Disabled"
                            }}</span
                          >
                        </div>
                        <div>
                          <span class="font-bold">Server Charset:</span> UTF-8
                          Unicode (utf8mb4)
                        </div>
                      </div>
                    </PmaInfoBox>
                  </div>

                  <!-- App Metadata Box -->
                  <div class="pma-col-6">
                    <PmaInfoBox variant="info">
                      <div class="flex flex-col gap-1.5 text-[11px]">
                        <strong class="text-[12px] block mb-1"
                          >📦 Application Metadata</strong
                        >
                        <div>
                          <span class="font-bold">Client Version:</span> v0.1.0
                          (Desktop Release)
                        </div>
                        <div>
                          <span class="font-bold">Client Engine:</span> Tauri v2
                          & Vue 3
                        </div>
                        <div>
                          <span class="font-bold">Creator:</span> TechisMology
                        </div>
                        <div>
                          <span class="font-bold">GitHub:</span>
                          <a
                            href="https://github.com/TechisMology/tableflow"
                            class="pma-box-link"
                            target="_blank"
                          >
                            TechisMology/tableflow
                          </a>
                        </div>
                      </div>
                    </PmaInfoBox>
                  </div>
                </div>

                <hr class="pma-divider" />
                <div class="pma-section-footer">
                  <span class="text-[var(--color-pma-text-muted)] italic">
                    Tip: Open a database settings gear icon (⚙️) on the left
                    sidebar to start working.
                  </span>
                </div>
              </PmaSectionHeader>
            </div>

            <!-- Create New Database Section -->
            <div class="pma-section-mb">
              <PmaSectionHeader icon="➕" label="Create New Database">
                <div class="pma-row items-center">
                  <div class="pma-col-3">
                    <label class="pma-form-label">Database Name</label>
                  </div>
                  <div class="pma-col-9 flex gap-2">
                    <input
                      v-model="newDbName"
                      type="text"
                      class="pma-sidebar-filter-input w-64"
                      placeholder="e.g. tableflow_db"
                      :disabled="creatingDb"
                    />
                    <PmaBtn @click="handleCreateDatabase" :disabled="creatingDb">
                      {{ creatingDb ? "Creating..." : "Create" }}
                    </PmaBtn>
                  </div>
                </div>

                <PmaInfoBox v-if="createDbError" variant="warning" class="mt-2">
                  {{ createDbError }}
                </PmaInfoBox>
              </PmaSectionHeader>
            </div>
          </div>

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

    <!-- Toast Success/Error -->
    <PmaToast
      :show="toastShow"
      :type="toastType"
      :message="toastMsg"
      @close="toastShow = false"
    />
  </div>
</template>
