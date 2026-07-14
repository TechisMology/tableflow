<script setup>
/**
 * UserAccountsPage.vue
 * Halaman "User accounts overview" — konten utama phpMyAdmin.
 * Zero inline styles — semua menggunakan CSS components dari style.css.
 */
import { ref } from "vue";
import PmaTable from "../ui/PmaTable.vue";
import PmaActionLink from "../ui/PmaActionLink.vue";
import PmaInfoBox from "../ui/PmaInfoBox.vue";
import PmaBtn from "../ui/PmaBtn.vue";
import PmaSectionHeader from "../ui/PmaSectionHeader.vue";

// ── Data Users ──────────────────────────────────────────────
const users = ref([
  {
    id: 1,
    username: "kevinmoreno",
    host: "localhost",
    password: "Yes",
    privileges: "ALL PRIVILEGES",
    grant: "Yes",
    locked: true,
  },
  {
    id: 2,
    username: "mysql.infoschema",
    host: "localhost",
    password: "Yes",
    privileges: "SELECT",
    grant: "No",
    locked: false,
  },
  {
    id: 3,
    username: "mysql.session",
    host: "localhost",
    password: "Yes",
    privileges: "SHUTDOWN, SUPER",
    grant: "No",
    locked: false,
  },
  {
    id: 4,
    username: "mysql.sys",
    host: "localhost",
    password: "Yes",
    privileges: "USAGE",
    grant: "No",
    locked: false,
  },
  {
    id: 5,
    username: "njokiadm",
    host: "localhost",
    password: "Yes",
    privileges: "USAGE",
    grant: "No",
    locked: true,
  },
  {
    id: 6,
    username: "pma",
    host: "192.168.30.%",
    password: "Yes",
    privileges: "USAGE",
    grant: "No",
    locked: true,
  },
  {
    id: 7,
    username: "root",
    host: "192.168.30.%",
    password: "No",
    privileges: "ALL PRIVILEGES",
    grant: "Yes",
    locked: true,
  },
  {
    id: 8,
    username: "root",
    host: "localhost",
    password: "No",
    privileges: "ALL PRIVILEGES",
    grant: "Yes",
    locked: true,
  },
  {
    id: 9,
    username: "students2020admin",
    host: "localhost",
    password: "Yes",
    privileges: "USAGE",
    grant: "No",
    locked: true,
  },
]);

// ── Selection State ──────────────────────────────────────────
const selectedIds = ref([]);
const checkAll = ref(false);
const dropDatabases = ref(false);

function toggleAll() {
  checkAll.value = !checkAll.value;
  selectedIds.value = checkAll.value ? users.value.map((u) => u.id) : [];
}

function toggleRow(id) {
  const idx = selectedIds.value.indexOf(id);
  if (idx === -1) {
    selectedIds.value.push(id);
  } else {
    selectedIds.value.splice(idx, 1);
  }
  checkAll.value = selectedIds.value.length === users.value.length;
}
</script>

<template>
  <section class="pma-content">
    <h2 class="pma-page-title">User accounts overview</h2>

    <!-- ── User Table ─────────────────────────────────────── -->
    <PmaTable>
      <!-- Table Head -->
      <template #head>
        <tr>
          <!-- col-1: checkbox -->
          <th class="pma-th-checkbox"></th>
          <!-- col-2: username (auto width) -->
          <th>User name</th>
          <!-- col-3: host -->
          <th>Host name</th>
          <!-- col-4: password -->
          <th class="pma-th-narrow">Password</th>
          <!-- col-5: global privileges -->
          <th>
            Global privileges
            <span class="pma-th-help-icon" title="Note: MySQL privilege system. Click for more info.">❓</span>
          </th>
          <!-- col-6: grant -->
          <th class="pma-th-narrow">Grant</th>
          <!-- col-7: action -->
          <th class="pma-th-action">Action</th>
        </tr>
      </template>

      <!-- Table Body -->
      <tr v-for="user in users" :key="user.id">

        <!-- Checkbox -->
        <td class="pma-th-checkbox">
          <input
            type="checkbox"
            class="pma-checkbox"
            :checked="selectedIds.includes(user.id)"
            @change="toggleRow(user.id)"
          />
        </td>

        <!-- Username -->
        <td>{{ user.username }}</td>

        <!-- Hostname -->
        <td>{{ user.host }}</td>

        <!-- Password — merah bold jika "No" -->
        <td :class="{ 'pma-cell-danger': user.password === 'No' }">
          {{ user.password }}
        </td>

        <!-- Privileges -->
        <td class="pma-cell-privilege">{{ user.privileges }}</td>

        <!-- Grant -->
        <td>{{ user.grant }}</td>

        <!-- Actions -->
        <td>
          <div class="pma-table-actions">
            <PmaActionLink icon="👤" label="Edit privileges" />
            <PmaActionLink icon="📤" label="Export" />
            <PmaActionLink
              :icon="user.locked ? '🔒' : '🔓'"
              :label="user.locked ? 'Lock' : 'Unlock'"
            />
          </div>
        </td>

      </tr>
    </PmaTable>

    <!-- ── Bulk Action Bar ────────────────────────────────── -->
    <div class="pma-bulk-bar">
      <span class="pma-bulk-bar-arrow">↳</span>
      <label class="pma-form-label">
        <input
          type="checkbox"
          class="pma-checkbox"
          :checked="checkAll"
          @change="toggleAll"
        />
        Check all
      </label>
      <span class="pma-bulk-bar-label">With selected:</span>
      <PmaActionLink icon="📤" label="Export" />
    </div>

    <!-- ── Section: Add New User ──────────────────────────── -->
    <div class="pma-section-mb">
      <PmaSectionHeader label="New">
        <PmaActionLink icon="👤" label="Add user account" />
      </PmaSectionHeader>
    </div>

    <!-- ── Section: Remove Selected Users ────────────────── -->
    <div class="pma-section-mb">
      <PmaSectionHeader icon="👥" label="Remove selected user accounts">

        <!-- Layout menggunakan pma-row / pma-col-12 -->
        <div class="pma-row">
          <div class="pma-col-12">
            <p class="pma-section-p">
              Revoke all active privileges from the users and delete them afterwards.
            </p>
          </div>
          <div class="pma-col-12">
            <label class="pma-form-label-mb">
              <input type="checkbox" class="pma-checkbox" v-model="dropDatabases" />
              Drop the databases that have the same names as the users.
            </label>
          </div>
          <div class="pma-col-12">
            <hr class="pma-divider" />
          </div>
          <div class="pma-col-12">
            <div class="pma-section-footer">
              <PmaBtn>Go</PmaBtn>
            </div>
          </div>
        </div>

      </PmaSectionHeader>
    </div>

    <!-- ── Footnotes / Notification Boxes ────────────────── -->
    <PmaInfoBox variant="warning">
      Note: phpMyAdmin gets the users' privileges directly from MySQL's
      privilege tables. The content of these tables may differ from the
      privileges the server uses, if they have been changed manually. In
      this case, you should
      <a class="pma-box-link" href="#">reload the privileges</a>
      before you continue.
    </PmaInfoBox>

    <PmaInfoBox variant="info">
      <a class="pma-box-link" href="#">phpMyAdmin Demo Server:</a>
      Currently running Git revision
      <a class="pma-box-link" href="#">RELEASE_5_2_3-8806-g3e756d69dd</a>
      from the
      <a class="pma-box-link" href="#">master</a> branch.
    </PmaInfoBox>

  </section>
</template>
