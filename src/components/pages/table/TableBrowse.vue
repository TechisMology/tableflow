<script setup>
/**
 * TableBrowse.vue
 * Browse sub-tab component under TableManagerPage
 */
import { ref, watch, computed } from "vue";
import { useConnectionStore } from "../../../stores/connections";
import { invoke } from "@tauri-apps/api/core";
import PmaTable from "../../ui/PmaTable.vue";
import PmaBtn from "../../ui/PmaBtn.vue";
import PmaActionLink from "../../ui/PmaActionLink.vue";
import PmaSectionHeader from "../../ui/PmaSectionHeader.vue";
import PmaToast from "../../ui/PmaToast.vue";

const props = defineProps({
  dbName: {
    type: String,
    required: true,
  },
  tableName: {
    type: String,
    required: true,
  },
  dbDriver: {
    type: String,
    required: true,
  },
});

const connectionStore = useConnectionStore();

// --- Toast State ---
const toastShow = ref(false);
const toastType = ref("success");
const toastMsg = ref("");

function triggerToast(type, msg) {
  toastType.value = type;
  toastMsg.value = msg;
  toastShow.value = true;
}

// --- Browse Tab State ---
const tableData = ref([]);
const tableHeaders = ref([]);
const totalRows = ref(0);
const currentPage = ref(1);
const limitPerPage = ref(10);
const showAll = ref(false);
const searchFilter = ref("");
const loadingData = ref(false);

// Row editing state
const editingRowIdx = ref(null);
const editingRowData = ref({});

// Insertion state
const showInsertForm = ref(false);
const newRowData = ref({});
const insertingRow = ref(false);

const columnMetadata = ref({});
const primaryKeyColumn = ref("");
const lastExecutedQuery = ref("");

function quote(name) {
  if (props.dbDriver === "postgres") {
    return `"${name.replace(/"/g, '""')}"`;
  }
  return `\`${name.replace(/`/g, "``")}\``;
}

// --- Fetch Column Metadata & Primary Key ---
async function fetchColumnMetadata() {
  columnMetadata.value = {};
  primaryKeyColumn.value = "";
  try {
    let password = null;
    try {
      password = await invoke("get_connection_password", {
        id: connectionStore.activeConnection.id,
      });
    } catch (e) {}

    let sql = "";
    if (props.dbDriver === "postgres") {
      sql = `SELECT column_name, data_type, is_nullable 
             FROM information_schema.columns 
             WHERE table_name = '${props.tableName}'`;
    } else if (props.dbDriver === "mysql") {
      sql = `SELECT COLUMN_NAME, DATA_TYPE, IS_NULLABLE 
             FROM information_schema.COLUMNS 
             WHERE TABLE_SCHEMA = '${props.dbName}' AND TABLE_NAME = '${props.tableName}'`;
    } else {
      sql = `PRAGMA table_info(${quote(props.tableName)})`;
    }

    const res = await invoke("execute_connection_query", {
      conn: connectionStore.activeConnection,
      password,
      query: sql,
      database: props.dbName,
    });
    
    const parsed = JSON.parse(res);
    for (const item of parsed) {
      const colName = item.column_name || item.COLUMN_NAME || item.name || "";
      const rawType = item.data_type || item.DATA_TYPE || item.type || "";
      const isNullableStr = item.is_nullable || item.IS_NULLABLE || "";
      const isNullable = isNullableStr === "YES" || item.notnull === 0;
      
      columnMetadata.value[colName] = {
        type: rawType.toLowerCase(),
        nullable: isNullable
      };

      if (props.dbDriver === "sqlite" && item.pk === 1) {
        primaryKeyColumn.value = colName;
      }
    }

    // Load Primary Key for MySQL / Postgres
    if (props.dbDriver === "mysql") {
      const priSql = `SELECT COLUMN_NAME 
                      FROM information_schema.COLUMNS 
                      WHERE TABLE_SCHEMA = '${props.dbName}' AND TABLE_NAME = '${props.tableName}' AND COLUMN_KEY = 'PRI'`;
      const priRes = await invoke("execute_connection_query", {
        conn: connectionStore.activeConnection,
        password,
        query: priSql,
        database: props.dbName,
      });
      const parsedPri = JSON.parse(priRes);
      if (parsedPri.length > 0) {
        primaryKeyColumn.value = parsedPri[0].COLUMN_NAME || parsedPri[0].column_name || "";
      }
    } else if (props.dbDriver === "postgres") {
      const priSql = `SELECT a.attname AS column_name
                      FROM pg_index i
                      JOIN pg_attribute a ON a.attrelid = i.indrelid AND a.attnum = ANY(i.indkey)
                      WHERE i.indrelid = '${props.tableName}'::regclass AND i.indisprimary`;
      const priRes = await invoke("execute_connection_query", {
        conn: connectionStore.activeConnection,
        password,
        query: priSql,
        database: props.dbName,
      });
      const parsedPri = JSON.parse(priRes);
      if (parsedPri.length > 0) {
        primaryKeyColumn.value = parsedPri[0].column_name || "";
      }
    }
  } catch (err) {}

  // Fallback: default to "id" if present in table headers, otherwise first column
  if (!primaryKeyColumn.value && tableHeaders.value.length > 0) {
    if (tableHeaders.value.includes("id")) {
      primaryKeyColumn.value = "id";
    } else {
      primaryKeyColumn.value = tableHeaders.value[0];
    }
  }
}

function getInputType(colName) {
  const meta = columnMetadata.value[colName];
  if (!meta) return "text";
  const type = meta.type;
  if (type.includes("timestamp") || type.includes("datetime")) {
    return "datetime-local";
  }
  if (type.includes("date")) {
    return "date";
  }
  if (type.includes("time")) {
    return "time";
  }
  return "text";
}

function formatValueForSQL(colName, val) {
  const meta = columnMetadata.value[colName];
  const isNullable = meta ? meta.nullable : true;
  
  if (val === undefined || val === null || (String(val).trim() === "" && isNullable)) {
    return "NULL";
  }
  const escaped = String(val).replace(/'/g, "''");
  return `'${escaped}'`;
}

// --- Fetch Data ---
async function fetchBrowseData() {
  loadingData.value = true;
  tableData.value = [];
  tableHeaders.value = [];
  try {
    let password = null;
    try {
      password = await invoke("get_connection_password", {
        id: connectionStore.activeConnection.id,
      });
    } catch (e) {}

    // 1. Get total rows
    let countSql = `SELECT COUNT(*) as cnt FROM ${quote(props.tableName)}`;
    if (searchFilter.value.trim() && tableHeaders.value.length > 0) {
      const escaped = searchFilter.value.replace(/'/g, "''");
      const clauses = tableHeaders.value.map(h => {
        if (props.dbDriver === "postgres") {
          return `CAST(${quote(h)} AS text) LIKE '%${escaped}%'`;
        }
        return `${quote(h)} LIKE '%${escaped}%'`;
      });
      countSql += ` WHERE ${clauses.join(" OR ")}`;
    }

    const countRes = await invoke("execute_connection_query", {
      conn: connectionStore.activeConnection,
      password,
      query: countSql,
      database: props.dbName,
    });
    const parsedCount = JSON.parse(countRes);
    totalRows.value = parseInt(parsedCount[0]?.cnt || parsedCount[0]?.count || 0);

    // 2. Fetch records
    let selectSql = `SELECT * FROM ${quote(props.tableName)}`;
    if (searchFilter.value.trim() && tableHeaders.value.length > 0) {
      const escaped = searchFilter.value.replace(/'/g, "''");
      const clauses = tableHeaders.value.map(h => {
        if (props.dbDriver === "postgres") {
          return `CAST(${quote(h)} AS text) LIKE '%${escaped}%'`;
        }
        return `${quote(h)} LIKE '%${escaped}%'`;
      });
      selectSql += ` WHERE ${clauses.join(" OR ")}`;
    }

    if (!showAll.value) {
      const offset = (currentPage.value - 1) * limitPerPage.value;
      selectSql += ` LIMIT ${limitPerPage.value} OFFSET ${offset}`;
    }

    const dataRes = await invoke("execute_connection_query", {
      conn: connectionStore.activeConnection,
      password,
      query: selectSql,
      database: props.dbName,
    });
    const parsedData = JSON.parse(dataRes);
    if (parsedData.length > 0) {
      tableHeaders.value = Object.keys(parsedData[0]);
      tableData.value = parsedData;
    } else {
      await fetchHeadersOnly();
    }
  } catch (err) {
    triggerToast("error", `Failed to load data: ${err}`);
  } finally {
    loadingData.value = false;
  }
}

async function fetchHeadersOnly() {
  try {
    let password = null;
    try {
      password = await invoke("get_connection_password", {
        id: connectionStore.activeConnection.id,
      });
    } catch (e) {}

    let sql = "";
    if (props.dbDriver === "postgres") {
      sql = `SELECT column_name FROM information_schema.columns WHERE table_name = '${props.tableName}' ORDER BY ordinal_position`;
    } else {
      sql = `SHOW COLUMNS FROM ${quote(props.tableName)}`;
    }

    const res = await invoke("execute_connection_query", {
      conn: connectionStore.activeConnection,
      password,
      query: sql,
      database: props.dbName,
    });
    const parsed = JSON.parse(res);
    tableHeaders.value = parsed.map(c => c.column_name || c.Field || c.Name || "");
  } catch (e) {}
}

function getFormattedInputValue(header, val) {
  if (val === undefined || val === null) return "";
  const type = getInputType(header);
  
  if (type === "datetime-local") {
    const d = new Date(val);
    if (!isNaN(d.getTime())) {
      const year = d.getFullYear();
      const month = String(d.getMonth() + 1).padStart(2, "0");
      const date = String(d.getDate()).padStart(2, "0");
      const hours = String(d.getHours()).padStart(2, "0");
      const minutes = String(d.getMinutes()).padStart(2, "0");
      return `${year}-${month}-${date}T${hours}:${minutes}`;
    }
  }
  
  if (type === "date") {
    const d = new Date(val);
    if (!isNaN(d.getTime())) {
      const year = d.getFullYear();
      const month = String(d.getMonth() + 1).padStart(2, "0");
      const date = String(d.getDate()).padStart(2, "0");
      return `${year}-${month}-${date}`;
    }
  }
  
  return String(val);
}

// --- Editing Rows ---
function startEditRow(idx, row) {
  editingRowIdx.value = idx;
  editingRowData.value = {};
  for (const h of tableHeaders.value) {
    editingRowData.value[h] = getFormattedInputValue(h, row[h]);
  }
}

function cancelEditRow() {
  editingRowIdx.value = null;
  editingRowData.value = {};
}

async function saveEditRow(originalRow) {
  try {
    let password = null;
    try {
      password = await invoke("get_connection_password", {
        id: connectionStore.activeConnection.id,
      });
    } catch (e) {}

    const pkCol = primaryKeyColumn.value || tableHeaders.value[0];
    const pkVal = originalRow[pkCol];

    const setClauses = [];
    for (const h of tableHeaders.value) {
      if (h === pkCol) continue;
      const newVal = editingRowData.value[h];
      setClauses.push(`${quote(h)} = ${formatValueForSQL(h, newVal)}`);
    }

    let updateSql = `UPDATE ${quote(props.tableName)} SET ${setClauses.join(", ")} WHERE ${quote(pkCol)} = `;
    if (pkVal === null || pkVal === undefined) {
      updateSql += "NULL";
    } else if (typeof pkVal === "number") {
      updateSql += pkVal;
    } else {
      updateSql += `'${String(pkVal).replace(/'/g, "''")}'`;
    }

    lastExecutedQuery.value = updateSql;

    await invoke("execute_connection_query", {
      conn: connectionStore.activeConnection,
      password,
      query: updateSql,
      database: props.dbName,
    });

    triggerToast("success", "Row updated successfully.");
    editingRowIdx.value = null;
    editingRowData.value = {};
    await fetchBrowseData();
  } catch (err) {
    triggerToast("error", `Update failed: ${err}`);
  }
}

// --- Insert Rows ---
async function handleInsertRow() {
  insertingRow.value = true;
  try {
    let password = null;
    try {
      password = await invoke("get_connection_password", {
        id: connectionStore.activeConnection.id,
      });
    } catch (e) {}

    const cols = [];
    const vals = [];

    for (const h of tableHeaders.value) {
      const val = newRowData.value[h];
      const sqlVal = formatValueForSQL(h, val);
      cols.push(quote(h));
      vals.push(sqlVal);
    }

    if (cols.length === 0) {
      throw new Error("No values provided to insert.");
    }

    const insertSql = `INSERT INTO ${quote(props.tableName)} (${cols.join(", ")}) VALUES (${vals.join(", ")})`;
    lastExecutedQuery.value = insertSql;

    await invoke("execute_connection_query", {
      conn: connectionStore.activeConnection,
      password,
      query: insertSql,
      database: props.dbName,
    });

    triggerToast("success", "Row inserted successfully.");
    newRowData.value = {};
    showInsertForm.value = false;
    await fetchBrowseData();
  } catch (err) {
    triggerToast("error", `Insertion failed: ${err}`);
  } finally {
    insertingRow.value = false;
  }
}

watch(
  () => props.tableName,
  async () => {
    currentPage.value = 1;
    editingRowIdx.value = null;
    editingRowData.value = {};
    await fetchColumnMetadata();
    await fetchBrowseData();
  },
  { immediate: true }
);
</script>

<template>
  <div class="flex flex-col gap-4">
    <h3 class="pma-page-title">Browse table data: {{ tableName }}</h3>

    <!-- Executed Query Display Console (at the very top) -->
    <div v-if="lastExecutedQuery" class="pma-section-mb">
      <PmaSectionHeader icon="💻" label="Last Executed SQL Query">
        <pre class="bg-blue-50/70 border border-blue-200 text-blue-900 rounded p-4 font-mono text-xs overflow-x-auto whitespace-pre-wrap select-all">{{ lastExecutedQuery }}</pre>
      </PmaSectionHeader>
    </div>

    <!-- Search Filter -->
    <div class="pma-section-mb">
      <PmaSectionHeader icon="🔍" label="Filter & Operations">
        <div class="flex gap-3 items-center w-full">
          <input
            v-model="searchFilter"
            type="text"
            class="pma-sidebar-filter-input w-72"
            placeholder="Search across all columns..."
            @keyup.enter="fetchBrowseData"
          />
          <PmaBtn @click="fetchBrowseData">Search</PmaBtn>
          <PmaBtn @click="searchFilter = ''; fetchBrowseData()" class="bg-gray-200 text-gray-800 hover:bg-gray-300">Clear</PmaBtn>
          
          <PmaBtn @click="showInsertForm = !showInsertForm" class="bg-green-600 hover:bg-green-700 text-white ml-auto">
            {{ showInsertForm ? "Cancel Insert" : "+ Add Row" }}
          </PmaBtn>
        </div>
      </PmaSectionHeader>
    </div>

    <!-- Insert New Row Form -->
    <div v-if="showInsertForm" class="pma-section-mb">
      <PmaSectionHeader icon="➕" label="Insert New Row">
        <div class="pma-row gap-y-4">
          <div v-for="header in tableHeaders" :key="header" class="pma-col-4">
            <div class="flex items-center justify-between">
              <label class="pma-form-label">{{ header }}</label>
              <span v-if="columnMetadata[header]?.nullable" class="text-[10px] text-gray-400 italic">(nullable)</span>
            </div>
            <input
              v-model="newRowData[header]"
              :type="getInputType(header)"
              class="pma-sidebar-filter-input w-full mt-1"
              :placeholder="columnMetadata[header]?.nullable ? 'NULL / Leave empty' : 'Enter value...'"
            />
          </div>
        </div>
        <hr class="pma-divider" />
        <div class="pma-section-footer flex justify-between items-center">
          <span class="text-[11px] text-[var(--color-pma-text-muted)]">
            Empty fields of nullable columns will be inserted as <strong>NULL</strong>.
          </span>
          <div class="flex gap-2">
            <PmaBtn @click="showInsertForm = false" class="bg-gray-200 text-gray-800 hover:bg-gray-300">Cancel</PmaBtn>
            <PmaBtn @click="handleInsertRow" :disabled="insertingRow">
              {{ insertingRow ? "Inserting..." : "Insert" }}
            </PmaBtn>
          </div>
        </div>
      </PmaSectionHeader>
    </div>

    <!-- Controls: Pagination -->
    <div class="flex justify-between items-center bg-gray-50 border border-[var(--color-pma-border)] p-3 rounded">
      <div class="flex items-center gap-3 text-[11px]">
        <span>Rows per page:</span>
        <select v-model="limitPerPage" @change="currentPage = 1; fetchBrowseData()" class="pma-sidebar-server-select w-16" :disabled="showAll">
          <option :value="10">10</option>
          <option :value="25">25</option>
          <option :value="50">50</option>
          <option :value="100">100</option>
        </select>
        
        <label class="flex items-center gap-1 cursor-pointer">
          <input type="checkbox" v-model="showAll" @change="currentPage = 1; fetchBrowseData()" />
          <span>Show All</span>
        </label>
      </div>

      <div v-if="!showAll" class="flex items-center gap-2">
        <PmaBtn @click="currentPage = Math.max(1, currentPage - 1); fetchBrowseData()" :disabled="currentPage === 1">Prev</PmaBtn>
        <span class="text-[11px] font-bold">Page {{ currentPage }} (Total Rows: {{ totalRows }})</span>
        <PmaBtn @click="currentPage++; fetchBrowseData()" :disabled="tableData.length < limitPerPage">Next</PmaBtn>
      </div>
      <div v-else class="text-[11px] font-bold">Showing All Rows (Total: {{ totalRows }})</div>
    </div>

    <!-- Table -->
    <div class="pma-table-wrapper">
      <PmaTable>
        <template #head>
          <tr>
            <th class="pma-th-narrow">Action</th>
            <th v-for="header in tableHeaders" :key="header">{{ header }}</th>
          </tr>
        </template>

        <tr v-if="loadingData">
          <td :colspan="tableHeaders.length + 1" class="text-center py-4 italic">Loading table records...</td>
        </tr>
        <tr v-else-if="tableData.length === 0">
          <td :colspan="tableHeaders.length + 1" class="text-center py-4 italic">No rows found matching requirements.</td>
        </tr>
        <tr v-else v-for="(row, idx) in tableData" :key="idx" class="hover:bg-[var(--color-pma-bg-row-hover)]">
          <td>
            <div class="pma-table-actions flex gap-1">
              <template v-if="editingRowIdx === idx">
                <PmaActionLink icon="💾" label="Save" @click="saveEditRow(row)" />
                <PmaActionLink icon="✕" label="Cancel" @click="cancelEditRow" />
              </template>
              <template v-else>
                <PmaActionLink icon="✏️" label="Edit" @click="startEditRow(idx, row)" />
              </template>
            </div>
          </td>
          <td v-for="header in tableHeaders" :key="header">
            <template v-if="editingRowIdx === idx">
              <input
                v-model="editingRowData[header]"
                :type="getInputType(header)"
                class="pma-sidebar-filter-input w-full"
                :placeholder="columnMetadata[header]?.nullable ? 'NULL' : ''"
              />
            </template>
            <template v-else>
              <span class="font-mono text-xs">{{ row[header] }}</span>
            </template>
          </td>
        </tr>
      </PmaTable>
    </div>

    <!-- Toast Success/Error -->
    <PmaToast :show="toastShow" :type="toastType" :message="toastMsg" @close="toastShow = false" />
  </div>
</template>
