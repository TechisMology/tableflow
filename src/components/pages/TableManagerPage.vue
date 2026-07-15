<script setup>
/**
 * TableManagerPage.vue
 * Halaman utama untuk mengelola tabel (Browse, Structure, Relation)
 * Terbagi menjadi sub-komponen terpisah untuk menjaga kesederhanaan kode.
 */
import { ref, computed } from "vue";
import { useConnectionStore } from "../../stores/connections";
import TableBrowse from "./table/TableBrowse.vue";
import TableStructure from "./table/TableStructure.vue";
import TableRelation from "./table/TableRelation.vue";

const props = defineProps({
  dbName: {
    type: String,
    required: true,
  },
  tableName: {
    type: String,
    required: true,
  },
});

const connectionStore = useConnectionStore();
const currentSubTab = ref("browse");
const dbDriver = computed(() => connectionStore.activeConnection?.driver || "mysql");
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
        :class="{ active: currentSubTab === 'browse' }"
        @click="currentSubTab = 'browse'"
      >
        <span>📂</span> Browse
      </button>
      <button
        class="pma-tab"
        :class="{ active: currentSubTab === 'structure' }"
        @click="currentSubTab = 'structure'"
      >
        <span>📐</span> Structure
      </button>
      <button
        class="pma-tab"
        :class="{ active: currentSubTab === 'relation' }"
        @click="currentSubTab = 'relation'"
      >
        <span>🔗</span> Relation Structure
      </button>
    </div>

    <!-- Main Content Area -->
    <div class="pma-content overflow-y-auto flex-1">
      <TableBrowse
        v-if="currentSubTab === 'browse'"
        :db-name="dbName"
        :table-name="tableName"
        :db-driver="dbDriver"
      />
      <TableStructure
        v-else-if="currentSubTab === 'structure'"
        :db-name="dbName"
        :table-name="tableName"
        :db-driver="dbDriver"
      />
      <TableRelation
        v-else-if="currentSubTab === 'relation'"
        :db-name="dbName"
        :table-name="tableName"
        :db-driver="dbDriver"
      />
    </div>
  </div>
</template>
