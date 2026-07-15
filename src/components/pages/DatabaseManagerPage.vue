<script setup>
/**
 * DatabaseManagerPage.vue
 * Halaman untuk mengelola database yang dipilih:
 * Sub-tabs: Structure, SQL, Export, Import, Diagram
 */
import { ref, onMounted, watch, computed, shallowRef } from "vue";
import { useConnectionStore } from "../../stores/connections";
import { invoke } from "@tauri-apps/api/core";
import loader from "@monaco-editor/loader";
import PmaTable from "../ui/PmaTable.vue";
import PmaActionLink from "../ui/PmaActionLink.vue";
import PmaBtn from "../ui/PmaBtn.vue";
import PmaSectionHeader from "../ui/PmaSectionHeader.vue";
import PmaInfoBox from "../ui/PmaInfoBox.vue";

import PmaModal from "../ui/PmaModal.vue";
import PmaToast from "../ui/PmaToast.vue";

const props = defineProps({
  dbName: {
    type: String,
    required: true,
  },
});

const connectionStore = useConnectionStore();
const subTab = ref("structure"); // "structure" | "sql" | "export" | "import" | "diagram"

// --- Toast & Modal State ---
const toastShow = ref(false);
const toastType = ref("success");
const toastMsg = ref("");

function triggerToast(type, msg) {
  toastType.value = type;
  toastMsg.value = msg;
  toastShow.value = true;
}

const showDropModal = ref(false);
const tableToDrop = ref("");

// --- State ---
const tablesList = computed(() => {
  const dbItem = connectionStore.sidebarDbs.find(
    (d) => d.name === props.dbName,
  );
  return dbItem ? dbItem.tables : [];
});

const loadingTables = computed(() => {
  const dbItem = connectionStore.sidebarDbs.find(
    (d) => d.name === props.dbName,
  );
  return dbItem ? dbItem.loading : false;
});

// Rename / Update State
const renamingTable = ref(null); // name of table being renamed
const newTableName = ref("");

// Create Table State
const newTableNameInput = ref("");
const createTableError = ref(null);

// SQL Editor State
const editorContainer = ref(null);
const monacoEditor = shallowRef(null);
const queryText = ref("");
const queryRunning = ref(false);
const queryResult = ref(null); // { success: boolean, data?: any[], message?: string }

// Export State
const exportFormat = ref("sql");
const exportOutput = ref("");
const exporting = ref(false);

// Load tables on mount or dbName change
watch(
  () => props.dbName,
  async (newDb) => {
    if (newDb && connectionStore.activeConnection) {
      const dbItem = connectionStore.sidebarDbs.find((d) => d.name === newDb);
      if (!dbItem || dbItem.tables.length === 0) {
        await connectionStore.openSidebarDb(newDb);
      }
    }
  },
  { immediate: true },
);

const tableSizes = ref({});

function formatBytes(bytes) {
  if (bytes === 0) return "0.0 B";
  const k = 1024;
  const sizes = ["B", "KiB", "MiB", "GiB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
}

async function loadTableSizes() {
  const driver = connectionStore.activeConnection?.driver;
  if (!driver || !props.dbName) return;

  let query = "";
  if (driver === "mysql") {
    query = `SELECT TABLE_NAME AS name, (DATA_LENGTH + INDEX_LENGTH) AS size 
             FROM information_schema.TABLES 
             WHERE TABLE_SCHEMA = '${props.dbName}'`;
  } else if (driver === "postgres") {
    query = `SELECT table_name AS name, pg_total_relation_size(quote_ident(table_name)) AS size 
             FROM information_schema.tables 
             WHERE table_schema = 'public'`;
  } else {
    // SQLite mock sizes
    tablesList.value.forEach((t) => {
      tableSizes.value[t] = "16.0 KiB";
    });
    return;
  }

  try {
    let password = null;
    try {
      password = await invoke("get_connection_password", {
        id: connectionStore.activeConnection.id,
      });
    } catch (e) {}

    const res = await invoke("execute_connection_query", {
      conn: connectionStore.activeConnection,
      password,
      query,
      database: props.dbName,
    });

    const data = JSON.parse(res);
    data.forEach((row) => {
      const bytes = Number(row.size || row.SIZE || row.pg_total_relation_size || 0);
      tableSizes.value[row.name || row.NAME || row.table_name] = formatBytes(bytes);
    });
  } catch (e) {
    console.error("Error loading table sizes:", e);
    tablesList.value.forEach((t) => {
      tableSizes.value[t] = "32.0 KiB";
    });
  }
}

watch(
  tablesList,
  async (newVal) => {
    if (newVal && newVal.length > 0) {
      await loadTableSizes();
    }
  },
  { immediate: true }
);

// Toggle Expand/Reload tables list helper
async function reloadTables() {
  const dbItem = connectionStore.sidebarDbs.find(
    (d) => d.name === props.dbName,
  );
  if (dbItem) {
    dbItem.tables = [];
    await connectionStore.toggleDbExpanded(props.dbName); // collapses
    await connectionStore.toggleDbExpanded(props.dbName); // expands & reloads
  }
}

// Helper to quote database/table identifiers according to active driver
function quoteIdentifier(name) {
  const driver = connectionStore.activeConnection?.driver;
  if (driver === "postgres") {
    return `"${name}"`;
  }
  return `\`${name}\``;
}

// ── Drop / Delete Table ──────────────────────────────────────
async function handleDropTable(tableName) {
  tableToDrop.value = tableName;
  showDropModal.value = true;
}

async function handleConfirmDrop() {
  showDropModal.value = false;
  const tableName = tableToDrop.value;
  if (!tableName) return;

  try {
    let password = null;
    try {
      password = await invoke("get_connection_password", {
        id: connectionStore.activeConnection.id,
      });
    } catch (e) {}

    const sql = `DROP TABLE ${quoteIdentifier(tableName)}`;
    await invoke("execute_connection_query", {
      conn: connectionStore.activeConnection,
      password,
      query: sql,
      database: props.dbName,
    });

    await reloadTables();
    triggerToast("success", `Table "${tableName}" dropped successfully.`);
  } catch (err) {
    triggerToast("error", `Error dropping table: ${err}`);
  }
}

// ── Rename Table ─────────────────────────────────────────────
function startRename(tableName) {
  renamingTable.value = tableName;
  newTableName.value = tableName;
}

async function handleRenameTable() {
  if (
    !newTableName.value.trim() ||
    newTableName.value === renamingTable.value
  ) {
    renamingTable.value = null;
    return;
  }

  try {
    let password = null;
    try {
      password = await invoke("get_connection_password", {
        id: connectionStore.activeConnection.id,
      });
    } catch (e) {}

    const sql = `ALTER TABLE ${quoteIdentifier(renamingTable.value)} RENAME TO ${quoteIdentifier(newTableName.value.trim())}`;

    await invoke("execute_connection_query", {
      conn: connectionStore.activeConnection,
      password,
      query: sql,
      database: props.dbName,
    });

    await reloadTables();
    triggerToast("success", `Table "${renamingTable.value}" renamed to "${newTableName.value.trim()}" successfully.`);
    renamingTable.value = null;
  } catch (err) {
    triggerToast("error", `Error renaming table: ${err}`);
  }
}

// ── Create Table ─────────────────────────────────────────────
async function handleCreateTable() {
  createTableError.value = null;
  const name = newTableNameInput.value.trim();
  if (!name) return;

  try {
    let password = null;
    try {
      password = await invoke("get_connection_password", {
        id: connectionStore.activeConnection.id,
      });
    } catch (e) {}

    const driver = connectionStore.activeConnection.driver;
    let finalSql = "";
    if (driver === "sqlite") {
      finalSql = `CREATE TABLE ${quoteIdentifier(name)} (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )`;
    } else if (driver === "postgres") {
      finalSql = `CREATE TABLE ${quoteIdentifier(name)} (
        id SERIAL PRIMARY KEY,
        name VARCHAR(100),
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
      )`;
    } else {
      finalSql = `CREATE TABLE ${quoteIdentifier(name)} (
        id INT AUTO_INCREMENT PRIMARY KEY,
        name VARCHAR(100) NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
      )`;
    }

    await invoke("execute_connection_query", {
      conn: connectionStore.activeConnection,
      password,
      query: finalSql,
      database: props.dbName,
    });

    newTableNameInput.value = "";
    await reloadTables();
    triggerToast("success", `Table "${name}" created successfully.`);
  } catch (err) {
    createTableError.value = String(err);
    triggerToast("error", `Error creating table: ${err}`);
  }
}

// ── SQL Monaco Editor ─────────────────────────────────────────
onMounted(async () => {
  // Init monaco editor when subTab is "sql"
  watch(
    subTab,
    async (newVal) => {
      if (newVal === "sql") {
        // Set default query
        if (!queryText.value) {
          const firstTable = tablesList.value[0] || "table_name";
          queryText.value = `SELECT * FROM ${quoteIdentifier(firstTable)} LIMIT 10;`;
        }

        // Wait for container element
        setTimeout(async () => {
          if (editorContainer.value && !monacoEditor.value) {
            const monaco = await loader.init();
            monacoEditor.value = monaco.editor.create(editorContainer.value, {
              value: queryText.value,
              language: "sql",
              theme: "vs",
              automaticLayout: true,
              minimap: { enabled: false },
            });
            // Update ref text on type
            monacoEditor.value.onDidChangeModelContent(() => {
              queryText.value = monacoEditor.value.getValue();
            });
          }
        }, 100);
      }
    },
    { immediate: true },
  );
});

async function handleRunSQL() {
  if (!queryText.value.trim()) return;
  queryRunning.value = true;
  queryResult.value = null;

  try {
    let password = null;
    try {
      password = await invoke("get_connection_password", {
        id: connectionStore.activeConnection.id,
      });
    } catch (e) {}

    const result = await invoke("execute_connection_query", {
      conn: connectionStore.activeConnection,
      password,
      query: queryText.value,
      database: props.dbName,
    });

    const parsedData = JSON.parse(result);
    queryResult.value = {
      success: true,
      data: parsedData,
    };

    await reloadTables();
    triggerToast("success", "Query executed successfully.");
  } catch (err) {
    queryResult.value = {
      success: false,
      message: String(err),
    };
    triggerToast("error", `Query execution failed: ${err}`);
  } finally {
    queryRunning.value = false;
  }
}

// ── Export Logic ──────────────────────────────────────────────
async function handleExport() {
  exporting.value = true;
  exportOutput.value = "";
  try {
    let password = null;
    try {
      password = await invoke("get_connection_password", {
        id: connectionStore.activeConnection.id,
      });
    } catch (e) {}

    const dump = await invoke("export_database", {
      conn: connectionStore.activeConnection,
      password,
      database: props.dbName,
      format: exportFormat.value,
    });

    exportOutput.value = dump;
  } catch (err) {
    alert(`Export failed: ${err}`);
  } finally {
    exporting.value = false;
  }
}

function handleCopyToClipboard() {
  navigator.clipboard.writeText(exportOutput.value);
  triggerToast("success", "Export copied to clipboard!");
}

function handleDownloadExport() {
  if (!exportOutput.value) return;
  const mimeType = exportFormat.value === "sql" ? "text/sql" : "application/json";
  const blob = new Blob([exportOutput.value], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `${props.dbName}_dump.${exportFormat.value}`;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
  triggerToast("success", `Dump downloaded as ${props.dbName}_dump.${exportFormat.value} successfully!`);
}

// --- Import Drag & Drop State ---
const importedFile = ref(null);
const importProgress = ref(false);
const isDragActive = ref(false);

function handleFileSelect(e) {
  const file = e.target.files[0];
  if (file) {
    importedFile.value = file;
  }
}

function handleFileDrop(e) {
  isDragActive.value = false;
  const file = e.dataTransfer.files[0];
  if (file) {
    importedFile.value = file;
  }
}

async function handleImportExecute() {
  if (!importedFile.value) return;
  importProgress.value = true;
  
  const reader = new FileReader();
  reader.onload = async (e) => {
    const fileContent = e.target.result;
    try {
      let password = null;
      try {
        password = await invoke("get_connection_password", {
          id: connectionStore.activeConnection.id,
        });
      } catch (err) {}

      let queryToSend = fileContent;
      const driver = connectionStore.activeConnection?.driver;
      if (driver === "postgres") {
        queryToSend = fileContent.replace(/`/g, '"');
      }

      // Split queries by semicolon and newlines to run sequentially
      const queries = queryToSend
        .split(/;\s*[\r\n]+/)
        .map((q) => q.trim())
        .filter((q) => q.length > 0);

      for (const sqlQuery of queries) {
        await invoke("execute_connection_query", {
          conn: connectionStore.activeConnection,
          password,
          query: sqlQuery,
          database: props.dbName,
        });
      }

      await reloadTables();
      triggerToast("success", `File "${importedFile.value.name}" imported successfully!`);
      importedFile.value = null;
    } catch (err) {
      triggerToast("error", `Import failed: ${err}`);
    } finally {
      importProgress.value = false;
    }
  };
  
  reader.readAsText(importedFile.value);
}
</script>

<template>
  <div class="flex flex-col h-full overflow-hidden">
    <!-- Sub-tab Headers -->
    <div
      class="pma-tab-nav"
      style="border-bottom: 1px solid var(--color-pma-border)"
    >
      <button
        class="pma-tab"
        :class="{ active: subTab === 'structure' }"
        @click="subTab = 'structure'"
      >
        📂 Structure
      </button>
      <button
        class="pma-tab"
        :class="{ active: subTab === 'sql' }"
        @click="subTab = 'sql'"
      >
        📄 SQL
      </button>
      <button
        class="pma-tab"
        :class="{ active: subTab === 'export' }"
        @click="subTab = 'export'"
      >
        📤 Export
      </button>
      <button
        class="pma-tab"
        :class="{ active: subTab === 'import' }"
        @click="subTab = 'import'"
      >
        📥 Import
      </button>
      <button
        class="pma-tab"
        :class="{ active: subTab === 'diagram' }"
        @click="subTab = 'diagram'"
      >
        📊 Diagram
      </button>
    </div>

    <!-- Manager Workspace -->
    <div class="flex-1 overflow-y-auto pma-content">
      <!-- SUB-TAB: STRUCTURE -->
      <div v-if="subTab === 'structure'" class="flex flex-col">
        <h3 class="pma-page-title">Database: {{ dbName }}</h3>

        <div
          v-if="loadingTables"
          class="italic text-[var(--color-pma-text-muted)]"
        >
          Loading tables structure...
        </div>

        <div v-else class="pma-section-mb">
          <PmaSectionHeader icon="📂" label="Tables list">
            <PmaTable v-if="tablesList.length > 0">
              <template #head>
                <tr>
                  <th>Table Name</th>
                  <th>Size</th>
                  <th class="pma-th-action">Actions</th>
                </tr>
              </template>

              <tr v-for="table in tablesList" :key="table">
                <td>
                  <!-- Inline renaming input -->
                  <div
                    v-if="renamingTable === table"
                    class="flex items-center gap-2"
                  >
                    <input
                      type="text"
                      v-model="newTableName"
                      class="pma-sidebar-filter-input w-48"
                      @keyup.enter="handleRenameTable"
                    />
                    <PmaBtn @click="handleRenameTable">Save</PmaBtn>
                    <PmaBtn @click="renamingTable = null">Cancel</PmaBtn>
                  </div>
                  <span v-else class="font-semibold">{{ table }}</span>
                </td>
                <td class="font-mono text-xs text-[var(--color-pma-text-muted)]">
                  {{ tableSizes[table] || "32.0 KiB" }}
                </td>
                <td>
                  <div class="pma-table-actions">
                    <PmaActionLink
                      icon="✏️"
                      label="Rename"
                      @click="startRename(table)"
                    />
                    <PmaActionLink
                      icon="🗑️"
                      label="Drop"
                      class="text-[var(--color-pma-text-danger)]"
                      @click="handleDropTable(table)"
                    />
                  </div>
                </td>
              </tr>
            </PmaTable>

            <div v-else class="italic text-[var(--color-pma-text-muted)] p-2">
              No tables in database. Create one below!
            </div>
          </PmaSectionHeader>
        </div>

        <!-- Add Table Panel -->
        <div class="pma-section-mb">
          <PmaSectionHeader icon="➕" label="Create new table">
            <div class="pma-row items-center">
              <div class="pma-col-3">
                <label class="pma-form-label">Table Name</label>
              </div>
              <div class="pma-col-6">
                <input
                  type="text"
                  v-model="newTableNameInput"
                  class="pma-sidebar-filter-input"
                  placeholder="e.g., users_profile"
                />
              </div>
              <div class="pma-col-3">
                <PmaBtn @click="handleCreateTable">Create Table</PmaBtn>
              </div>
            </div>

            <PmaInfoBox v-if="createTableError" variant="warning" class="mt-2">
              {{ createTableError }}
            </PmaInfoBox>
          </PmaSectionHeader>
        </div>
      </div>

      <!-- SUB-TAB: SQL -->
      <div v-show="subTab === 'sql'" class="flex flex-col gap-4 h-full">
        <h3 class="pma-page-title">
          Run SQL queries on database: {{ dbName }}
        </h3>

        <!-- Editor Container -->
        <div
          ref="editorContainer"
          class="border border-[var(--color-pma-border)] rounded w-full h-48 bg-white"
        ></div>

        <!-- Run Controls -->
        <div class="flex justify-end mt-2">
          <PmaBtn @click="handleRunSQL" :disabled="queryRunning">
            {{ queryRunning ? "Executing..." : "Go" }}
          </PmaBtn>
        </div>

        <!-- Result Output -->
        <div v-if="queryResult" class="mt-3">
          <div v-if="!queryResult.success">
            <PmaInfoBox variant="warning">
              <strong>Query failed:</strong>
              <pre class="mt-1 font-mono text-xs whitespace-pre-wrap">{{
                queryResult.message
              }}</pre>
            </PmaInfoBox>
          </div>

          <div v-else-if="queryResult.data && queryResult.data.length > 0">
            <PmaSectionHeader icon="📋" label="Query Results">
              <div class="overflow-auto max-h-80">
                <PmaTable>
                  <template #head>
                    <tr>
                      <th
                        v-for="key in Object.keys(queryResult.data[0])"
                        :key="key"
                      >
                        {{ key }}
                      </th>
                    </tr>
                  </template>
                  <tr v-for="(row, idx) in queryResult.data" :key="idx">
                    <td
                      v-for="(val, colIdx) in Object.values(row)"
                      :key="colIdx"
                    >
                      {{ val === null ? "NULL" : val }}
                    </td>
                  </tr>
                </PmaTable>
              </div>
            </PmaSectionHeader>
          </div>

          <div v-else>
            <PmaInfoBox variant="info">
              Query executed successfully. Empty result set or statement
              returned no data (e.g. DDL / Update / Delete).
            </PmaInfoBox>
          </div>
        </div>
      </div>

      <!-- SUB-TAB: EXPORT -->
      <div v-if="subTab === 'export'" class="flex flex-col gap-4">
        <h3 class="pma-page-title">Export Database: {{ dbName }}</h3>

        <div class="pma-section-mb">
          <PmaSectionHeader icon="📤" label="Export configurations">
            <div class="pma-row items-center mb-3">
              <div class="pma-col-3">
                <label class="pma-form-label">Export Format</label>
              </div>
              <div class="pma-col-9">
                <select
                  v-model="exportFormat"
                  class="pma-sidebar-server-select w-48"
                >
                  <option value="sql">SQL Dump (.sql)</option>
                  <option value="json">JSON Schema & Data (.json)</option>
                </select>
              </div>
            </div>

            <hr class="pma-divider" />
            <div class="pma-section-footer">
              <PmaBtn @click="handleExport" :disabled="exporting">
                {{ exporting ? "Exporting..." : "Export" }}
              </PmaBtn>
            </div>
          </PmaSectionHeader>
        </div>

        <div v-if="exportOutput" class="pma-section-mb">
          <PmaSectionHeader icon="📜" label="Generated Output">
            <div class="flex justify-end gap-2 mb-2">
              <PmaBtn @click="handleCopyToClipboard">Copy to Clipboard</PmaBtn>
              <PmaBtn @click="handleDownloadExport">Download File</PmaBtn>
            </div>
            <textarea
              v-model="exportOutput"
              class="w-full h-64 p-2 font-mono text-xs border border-[var(--color-pma-border)] bg-gray-50 rounded"
              readonly
            ></textarea>
          </PmaSectionHeader>
        </div>
      </div>

      <!-- SUB-TAB: IMPORT -->
      <div v-if="subTab === 'import'" class="flex flex-col gap-4">
        <h3 class="pma-page-title">Import files into database: {{ dbName }}</h3>

        <PmaInfoBox variant="info">
          Choose a SQL or CSV file and drag it into the box below to import schemas or data into database.
        </PmaInfoBox>

        <div class="pma-section-mb">
          <PmaSectionHeader icon="📥" label="Import schema / data">
            <div 
              class="border-2 border-dashed border-[var(--color-pma-border)] rounded-md p-8 flex flex-col items-center justify-center gap-3 cursor-pointer transition-all hover:bg-gray-50/50"
              :class="{ 'border-[var(--color-pma-blue)] bg-blue-50/50': isDragActive }"
              @dragover.prevent="isDragActive = true"
              @dragleave.prevent="isDragActive = false"
              @drop.prevent="handleFileDrop"
              @click="$refs.fileInput.click()"
            >
              <input 
                type="file" 
                ref="fileInput" 
                class="hidden" 
                accept=".sql,.csv" 
                @change="handleFileSelect" 
              />
              
              <span class="text-3xl">📄</span>
              <div v-if="!importedFile" class="text-center">
                <p class="font-bold text-[12px] text-[var(--color-pma-text)]">
                  Drag and drop your SQL/CSV file here, or click to browse
                </p>
                <p class="text-[10px] text-[var(--color-pma-text-muted)] mt-1">
                  Supports SQL and CSV formats
                </p>
              </div>
              <div v-else class="text-center">
                <p class="font-bold text-[12px] text-green-700">
                  Selected: {{ importedFile.name }}
                </p>
                <p class="text-[10px] text-[var(--color-pma-text-muted)] mt-1">
                  Size: {{ formatBytes(importedFile.size) }}
                </p>
              </div>
            </div>

            <hr class="pma-divider" />
            <div class="pma-section-footer flex justify-between items-center">
              <span v-if="importedFile" class="text-[11px] text-[var(--color-pma-text-muted)]">
                Ready to import into database: <strong>{{ dbName }}</strong>
              </span>
              <span v-else class="text-[11px] text-[var(--color-pma-text-muted)]">
                Please select a file to import.
              </span>
              <PmaBtn 
                @click="handleImportExecute" 
                :disabled="!importedFile || importProgress"
              >
                {{ importProgress ? "Importing..." : "Go" }}
              </PmaBtn>
            </div>
          </PmaSectionHeader>
        </div>
      </div>

      <!-- SUB-TAB: DIAGRAM -->
      <div v-if="subTab === 'diagram'" class="flex flex-col gap-4">
        <h3 class="pma-page-title">E-R Diagram: {{ dbName }}</h3>

        <PmaInfoBox variant="info">
          Diagram page placeholder view. Renders table schemas and foreign key
          relationships interactively.
        </PmaInfoBox>

        <div
          class="pma-section-mb border border-[var(--color-pma-border)] bg-gray-50 h-80 rounded flex items-center justify-center"
        >
          <span class="text-[var(--color-pma-text-muted)] italic font-semibold"
            >Diagram Renderer Placeholder</span
          >
        </div>
      </div>
    </div>

    <!-- Modals & Toasts -->
    <PmaModal 
      :show="showDropModal"
      title="Drop Table"
      :message="`Are you sure you want to DROP (DELETE) table '${tableToDrop}'? All data will be permanently deleted!`"
      @confirm="handleConfirmDrop"
      @cancel="showDropModal = false"
    />

    <PmaToast 
      :show="toastShow"
      :type="toastType"
      :message="toastMsg"
      @close="toastShow = false"
    />
  </div>
</template>
