<script setup>
/**
 * TableStructure.vue
 * Structure sub-tab component under TableManagerPage
 */
import { ref, watch } from "vue";
import { useConnectionStore } from "../../../stores/connections";
import { invoke } from "@tauri-apps/api/core";
import PmaTable from "../../ui/PmaTable.vue";
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

const structureColumns = ref([]);
const loadingStructure = ref(false);

function quote(name) {
  if (props.dbDriver === "postgres") {
    return `"${name.replace(/"/g, '""')}"`;
  }
  return `\`${name.replace(/`/g, "``")}\``;
}

// --- Fetch Structure ---
async function fetchStructure() {
  loadingStructure.value = true;
  structureColumns.value = [];
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
        ordinal_position AS "#",
        column_name AS "Name",
        data_type AS "Type",
        collation_name AS "Collation",
        '' AS "Attributes",
        is_nullable AS "Null",
        column_default AS "Default",
        '' AS "Comments",
        '' AS "Extra"
      FROM information_schema.columns 
      WHERE table_name = '${props.tableName}'
      ORDER BY ordinal_position;`;
    } else if (props.dbDriver === "mysql") {
      sql = `SELECT 
        ORDINAL_POSITION AS '#',
        COLUMN_NAME AS 'Name',
        COLUMN_TYPE AS 'Type',
        COLLATION_NAME AS 'Collation',
        '' AS 'Attributes',
        IS_NULLABLE AS 'Null',
        COLUMN_DEFAULT AS 'Default',
        COLUMN_COMMENT AS 'Comments',
        EXTRA AS 'Extra'
      FROM information_schema.COLUMNS 
      WHERE TABLE_SCHEMA = '${props.dbName}' AND TABLE_NAME = '${props.tableName}'
      ORDER BY ORDINAL_POSITION;`;
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
    if (props.dbDriver === "sqlite") {
      structureColumns.value = parsed.map(c => ({
        "#": c.cid,
        Name: c.name,
        Type: c.type,
        Collation: "",
        Attributes: "",
        Null: c.notnull === 1 ? "NO" : "YES",
        Default: c.dflt_value || "",
        Comments: "",
        Extra: c.pk === 1 ? "PRIMARY KEY" : "",
      }));
    } else {
      structureColumns.value = parsed;
    }
  } catch (err) {
    triggerToast("error", `Failed to load structure: ${err}`);
  } finally {
    loadingStructure.value = false;
  }
}

watch(
  () => props.tableName,
  () => {
    fetchStructure();
  },
  { immediate: true }
);
</script>

<template>
  <div class="flex flex-col gap-4">
    <h3 class="pma-page-title">Table Structure: {{ tableName }}</h3>

    <div class="pma-table-wrapper">
      <PmaTable>
        <template #head>
          <tr>
            <th>#</th>
            <th>Name</th>
            <th>Type</th>
            <th>Collation</th>
            <th>Attributes</th>
            <th>Null</th>
            <th>Default</th>
            <th>Comments</th>
            <th>Extra</th>
          </tr>
        </template>

        <tr v-if="loadingStructure">
          <td colspan="9" class="text-center py-4 italic">Loading structure catalog...</td>
        </tr>
        <tr v-else-if="structureColumns.length === 0">
          <td colspan="9" class="text-center py-4 italic">No structure catalog defined.</td>
        </tr>
        <tr v-else v-for="col in structureColumns" :key="col.Name || col.name">
          <td class="font-mono text-xs">{{ col['#'] !== undefined ? col['#'] : col.cid }}</td>
          <td class="font-bold font-mono text-xs">{{ col.Name || col.name }}</td>
          <td class="text-blue-800 font-mono text-xs">{{ col.Type || col.type }}</td>
          <td class="font-mono text-xs">{{ col.Collation }}</td>
          <td class="font-mono text-xs">{{ col.Attributes }}</td>
          <td class="font-mono text-xs">{{ col.Null }}</td>
          <td class="font-mono text-xs">{{ col.Default || col.default }}</td>
          <td class="font-mono text-xs">{{ col.Comments }}</td>
          <td class="text-purple-700 font-mono text-xs">{{ col.Extra }}</td>
        </tr>
      </PmaTable>
    </div>

    <!-- Toast Success/Error -->
    <PmaToast :show="toastShow" :type="toastType" :message="toastMsg" @close="toastShow = false" />
  </div>
</template>
