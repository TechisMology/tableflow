<script setup>
/**
 * TableRelation.vue
 * Relation Structure sub-tab component under TableManagerPage
 */
import { ref, watch } from "vue";
import { useConnectionStore } from "../../../stores/connections";
import { invoke } from "@tauri-apps/api/core";
import PmaTable from "../../ui/PmaTable.vue";
import PmaBtn from "../../ui/PmaBtn.vue";
import PmaActionLink from "../../ui/PmaActionLink.vue";
import PmaSectionHeader from "../../ui/PmaSectionHeader.vue";
import PmaInfoBox from "../../ui/PmaInfoBox.vue";
import PmaToast from "../../ui/PmaToast.vue";
import PmaModal from "../../ui/PmaModal.vue";

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

// --- Toast & Modals ---
const toastShow = ref(false);
const toastType = ref("success");
const toastMsg = ref("");

function triggerToast(type, msg) {
  toastType.value = type;
  toastMsg.value = msg;
  toastShow.value = true;
}

const showDeleteRelationModal = ref(false);
const relationToDelete = ref("");

// --- Relation State ---
const relationsList = ref([]);
const loadingRelations = ref(false);

const newConstraintName = ref("");
const newColName = ref("");
const newRefDb = ref(props.dbName);
const newRefTable = ref("");
const newRefCol = ref("");
const newOnDelete = ref("RESTRICT");
const newOnUpdate = ref("RESTRICT");
const addingRelation = ref(false);

function quote(name) {
  if (props.dbDriver === "postgres") {
    return `"${name.replace(/"/g, '""')}"`;
  }
  return `\`${name.replace(/`/g, "``")}\``;
}

// --- Fetch Relations ---
async function fetchRelations() {
  loadingRelations.value = true;
  relationsList.value = [];
  try {
    let password = null;
    try {
      password = await invoke("get_connection_password", {
        id: connectionStore.activeConnection.id,
      });
    } catch (e) {}

    let sql = "";
    if (props.dbDriver === "postgres") {
      sql = `SELECT
        tc.constraint_name AS "Constraint name",
        kcu.column_name AS "Column",
        ccu.table_schema AS "Database",
        ccu.table_name AS "Table",
        ccu.column_name AS "Referenced Column",
        rc.update_rule AS "ON UPDATE",
        rc.delete_rule AS "ON DELETE"
      FROM information_schema.table_constraints tc
      JOIN information_schema.key_column_usage kcu
        ON tc.constraint_name = kcu.constraint_name
      JOIN information_schema.constraint_column_usage ccu
        ON ccu.constraint_name = tc.constraint_name
      JOIN information_schema.referential_constraints rc
        ON rc.constraint_name = tc.constraint_name
      WHERE tc.constraint_type = 'FOREIGN KEY'
        AND tc.table_name = '${props.tableName}';`;
    } else if (props.dbDriver === "mysql") {
      sql = `SELECT 
        CONSTRAINT_NAME AS 'Constraint name',
        COLUMN_NAME AS 'Column',
        REFERENCED_TABLE_SCHEMA AS 'Database',
        REFERENCED_TABLE_NAME AS 'Table',
        REFERENCED_COLUMN_NAME AS 'Referenced Column',
        'RESTRICT' AS 'ON UPDATE',
        'RESTRICT' AS 'ON DELETE'
      FROM information_schema.KEY_COLUMN_USAGE
      WHERE TABLE_SCHEMA = '${props.dbName}' AND TABLE_NAME = '${props.tableName}' AND REFERENCED_TABLE_NAME IS NOT NULL;`;
    } else {
      sql = `PRAGMA foreign_key_list(${quote(props.tableName)})`;
    }

    const res = await invoke("execute_connection_query", {
      conn: connectionStore.activeConnection,
      password,
      query: sql,
      database: props.dbName,
    });
    const parsed = JSON.parse(res);
    if (props.dbDriver === "sqlite") {
      relationsList.value = parsed.map((c, i) => ({
        "Constraint name": `fk_${props.tableName}_${i}`,
        Column: c.from,
        Database: props.dbName,
        Table: c.table,
        "Referenced Column": c.to,
        "ON UPDATE": c.on_update,
        "ON DELETE": c.on_delete,
      }));
    } else {
      relationsList.value = parsed;
    }
  } catch (err) {
    // Gracefully handle
  } finally {
    loadingRelations.value = false;
  }
}

// --- Add Constraint ---
async function handleAddRelation() {
  const constraint = newConstraintName.value.trim() || `fk_${props.tableName}_${Date.now()}`;
  const col = newColName.value.trim();
  const refTbl = newRefTable.value.trim();
  const refCol = newRefCol.value.trim();

  if (!col || !refTbl || !refCol) {
    triggerToast("error", "Please fill in all relation columns.");
    return;
  }

  addingRelation.value = true;
  try {
    let password = null;
    try {
      password = await invoke("get_connection_password", {
        id: connectionStore.activeConnection.id,
      });
    } catch (e) {}

    let sql = `ALTER TABLE ${quote(props.tableName)} ADD CONSTRAINT ${quote(constraint)} 
      FOREIGN KEY (${quote(col)}) REFERENCES ${quote(refTbl)} (${quote(refCol)})
      ON DELETE ${newOnDelete.value} ON UPDATE ${newOnUpdate.value}`;

    if (props.dbDriver === "sqlite") {
      throw new Error("SQLite does not support ALTER TABLE ADD CONSTRAINT directly.");
    }

    await invoke("execute_connection_query", {
      conn: connectionStore.activeConnection,
      password,
      query: sql,
      database: props.dbName,
    });

    triggerToast("success", `Relation "${constraint}" created successfully.`);
    newConstraintName.value = "";
    newColName.value = "";
    newRefTable.value = "";
    newRefCol.value = "";
    await fetchRelations();
  } catch (err) {
    triggerToast("error", `Relation addition failed: ${err}`);
  } finally {
    addingRelation.value = false;
  }
}

// --- Drop Constraint ---
function startDeleteRelation(name) {
  relationToDelete.value = name;
  showDeleteRelationModal.value = true;
}

async function handleConfirmDeleteRelation() {
  showDeleteRelationModal.value = false;
  const constraint = relationToDelete.value;
  if (!constraint) return;

  try {
    let password = null;
    try {
      password = await invoke("get_connection_password", {
        id: connectionStore.activeConnection.id,
      });
    } catch (e) {}

    let sql = `ALTER TABLE ${quote(props.tableName)} DROP CONSTRAINT ${quote(constraint)}`;
    if (props.dbDriver === "mysql") {
      sql = `ALTER TABLE ${quote(props.tableName)} DROP FOREIGN KEY ${quote(constraint)}`;
    } else if (props.dbDriver === "sqlite") {
      throw new Error("SQLite does not support dropping constraints directly.");
    }

    await invoke("execute_connection_query", {
      conn: connectionStore.activeConnection,
      password,
      query: sql,
      database: props.dbName,
    });

    triggerToast("success", `Relation "${constraint}" dropped successfully.`);
    await fetchRelations();
  } catch (err) {
    triggerToast("error", `Drop relation failed: ${err}`);
  }
}

watch(
  () => props.tableName,
  () => {
    fetchRelations();
  },
  { immediate: true }
);
</script>

<template>
  <div class="flex flex-col gap-4">
    <h3 class="pma-page-title">Relation View: {{ tableName }}</h3>

    <PmaInfoBox variant="info">
      Manage foreign key constraints between your table columns and reference tables.
    </PmaInfoBox>

    <!-- Relation Listing -->
    <div class="pma-section-mb">
      <PmaSectionHeader icon="⚙️" label="Foreign Key Constraints">
        <PmaTable>
          <template #head>
            <tr>
              <th class="pma-th-narrow">Actions</th>
              <th>Constraint Name</th>
              <th>Column</th>
              <th>Database</th>
              <th>Table</th>
              <th>Referenced Column</th>
              <th>ON DELETE</th>
              <th>ON UPDATE</th>
            </tr>
          </template>

          <tr v-if="loadingRelations">
            <td colspan="8" class="text-center py-4 italic">Loading constraints...</td>
          </tr>
          <tr v-else-if="relationsList.length === 0">
            <td colspan="8" class="text-center py-4 italic">No foreign key constraints defined for this table.</td>
          </tr>
          <tr v-else v-for="rel in relationsList" :key="rel['Constraint name'] || rel.id">
            <td>
              <PmaActionLink icon="🗑️" label="Drop" @click="startDeleteRelation(rel['Constraint name'])" />
            </td>
            <td class="font-bold text-xs">{{ rel["Constraint name"] }}</td>
            <td class="font-mono text-xs">{{ rel.Column }}</td>
            <td class="text-xs">{{ rel.Database }}</td>
            <td class="font-bold text-xs">{{ rel.Table }}</td>
            <td class="font-mono text-xs">{{ rel["Referenced Column"] }}</td>
            <td class="text-xs text-red-600 font-bold">{{ rel["ON DELETE"] }}</td>
            <td class="text-xs text-blue-600 font-bold">{{ rel["ON UPDATE"] }}</td>
          </tr>
        </PmaTable>
      </PmaSectionHeader>
    </div>

    <!-- Add Constraint Form -->
    <div class="pma-section-mb">
      <PmaSectionHeader icon="➕" label="Add Foreign Key Constraint">
        <div class="pma-row gap-y-4">
          <div class="pma-col-4">
            <label class="pma-form-label">Constraint Name</label>
            <input
              v-model="newConstraintName"
              type="text"
              class="pma-sidebar-filter-input w-full mt-1"
              placeholder="e.g. fk_ref_table"
            />
          </div>
          <div class="pma-col-4">
            <label class="pma-form-label">Column</label>
            <input
              v-model="newColName"
              type="text"
              class="pma-sidebar-filter-input w-full mt-1"
              placeholder="Local column name"
            />
          </div>
          <div class="pma-col-4">
            <label class="pma-form-label">Referenced Database</label>
            <input
              v-model="newRefDb"
              type="text"
              class="pma-sidebar-filter-input w-full mt-1"
              placeholder="e.g. sakila"
            />
          </div>

          <div class="pma-col-4">
            <label class="pma-form-label">Referenced Table</label>
            <input
              v-model="newRefTable"
              type="text"
              class="pma-sidebar-filter-input w-full mt-1"
              placeholder="Reference table name"
            />
          </div>
          <div class="pma-col-4">
            <label class="pma-form-label">Referenced Column</label>
            <input
              v-model="newRefCol"
              type="text"
              class="pma-sidebar-filter-input w-full mt-1"
              placeholder="Reference column name"
            />
          </div>

          <div class="pma-col-2">
            <label class="pma-form-label">ON DELETE</label>
            <select v-model="newOnDelete" class="pma-sidebar-server-select w-full mt-1">
              <option value="RESTRICT">RESTRICT</option>
              <option value="CASCADE">CASCADE</option>
              <option value="SET NULL">SET NULL</option>
              <option value="NO ACTION">NO ACTION</option>
            </select>
          </div>
          <div class="pma-col-2">
            <label class="pma-form-label">ON UPDATE</label>
            <select v-model="newOnUpdate" class="pma-sidebar-server-select w-full mt-1">
              <option value="RESTRICT">RESTRICT</option>
              <option value="CASCADE">CASCADE</option>
              <option value="SET NULL">SET NULL</option>
              <option value="NO ACTION">NO ACTION</option>
            </select>
          </div>
        </div>

        <hr class="pma-divider" />
        <div class="pma-section-footer flex justify-between items-center">
          <span class="text-[11px] text-[var(--color-pma-text-muted)]">
            Creates a new link constraint rule on table: <strong>{{ tableName }}</strong>
          </span>
          <PmaBtn @click="handleAddRelation" :disabled="addingRelation">
            {{ addingRelation ? "Adding..." : "+ Add column / constraint" }}
          </PmaBtn>
        </div>
      </PmaSectionHeader>
    </div>

    <!-- Confirm Drop Modal -->
    <PmaModal
      :show="showDeleteRelationModal"
      title="Drop Foreign Key Constraint"
      :message="`Are you sure you want to drop foreign key constraint '${relationToDelete}'?`"
      @confirm="handleConfirmDeleteRelation"
      @cancel="showDeleteRelationModal = false"
    />

    <!-- Toast Success/Error -->
    <PmaToast :show="toastShow" :type="toastType" :message="toastMsg" @close="toastShow = false" />
  </div>
</template>
