<script setup>
/**
 * ConnectionForm.vue
 * Halaman form pembuatan dan pengeditan koneksi database.
 */
import { ref, watch, computed } from "vue";
import { useConnectionStore } from "../../stores/connections";
import { invoke } from "@tauri-apps/api/core";
import PmaBtn from "../ui/PmaBtn.vue";
import PmaSectionHeader from "../ui/PmaSectionHeader.vue";
import PmaInfoBox from "../ui/PmaInfoBox.vue";

const props = defineProps({
  editConnectionId: {
    type: String,
    default: null,
  },
});

const emit = defineEmits(["saved", "cancel"]);

const connectionStore = useConnectionStore();

const id = ref(crypto.randomUUID());
const name = ref("");
const driver = ref("mysql");
const host = ref("localhost");
const port = ref(3306);
const username = ref("root");
const password = ref("");
const database = ref("");
const sslEnabled = ref(false);

const testing = ref(false);
const testResult = ref(null); // { success: boolean, message: string }

// Reset/Load form on editConnectionId change
watch(
  () => props.editConnectionId,
  (newId) => {
    if (newId) {
      const conn = connectionStore.connections.find((c) => c.id === newId);
      if (conn) {
        id.value = conn.id;
        name.value = conn.name;
        driver.value = conn.driver;
        host.value = conn.host || "";
        port.value = conn.port || (conn.driver === "mysql" ? 3306 : 5432);
        username.value = conn.username || "";
        database.value = conn.database || "";
        sslEnabled.value = conn.ssl_enabled || false;
        // Password is not saved in JSON so we leave it empty.
        // If the user leaves password blank on edit, we can fetch it or ask keyring,
        // or just keep it blank to indicate "no change".
        password.value = "";
      }
    } else {
      resetForm();
    }
  },
  { immediate: true }
);

// Watch driver changes to adjust defaults
watch(driver, (newDriver) => {
  if (!props.editConnectionId) {
    if (newDriver === "mysql") {
      port.value = 3306;
      username.value = "root";
      host.value = "localhost";
    } else if (newDriver === "postgres") {
      port.value = 5432;
      username.value = "postgres";
      host.value = "localhost";
    } else if (newDriver === "sqlite") {
      port.value = 0;
      username.value = "";
      host.value = "";
    }
  }
});

function resetForm() {
  id.value = crypto.randomUUID();
  name.value = "";
  driver.value = "mysql";
  host.value = "localhost";
  port.value = 3306;
  username.value = "root";
  password.value = "";
  database.value = "";
  sslEnabled.value = false;
  testResult.value = null;
}

const isSqlite = computed(() => driver.value === "sqlite");

const connectionPayload = computed(() => {
  return {
    id: id.value,
    name: name.value || `${driver.value.toUpperCase()} Connection`,
    driver: driver.value,
    host: isSqlite.value ? null : host.value,
    port: isSqlite.value ? null : Number(port.value),
    username: isSqlite.value ? null : username.value,
    database: database.value,
    ssl_enabled: isSqlite.value ? null : sslEnabled.value,
  };
});

async function handleTest() {
  testing.value = true;
  testResult.value = null;
  try {
    // If password is empty and we are editing, we can try to fetch the existing password from keychain
    let pwd = password.value;
    if (!pwd && props.editConnectionId) {
      try {
        const storedPwd = await invoke("get_connection_password", { id: id.value });
        if (storedPwd) pwd = storedPwd;
      } catch (e) {
        console.error("Failed to retrieve stored password for testing", e);
      }
    }

    await invoke("test_connection", {
      conn: connectionPayload.value,
      password: pwd || null,
    });
    testResult.value = {
      success: true,
      message: "Connection successful!",
    };
  } catch (err) {
    testResult.value = {
      success: false,
      message: String(err),
    };
  } finally {
    testing.value = false;
  }
}

async function handleSave() {
  testResult.value = null;
  try {
    let pwd = password.value;
    if (!pwd && props.editConnectionId) {
      // If password is empty during edit, fetch the existing one so we don't clear it,
      // or the backend command handles it if we don't pass anything.
      // Let's pass the retrieved one if any, or null to indicate no change.
      try {
        const storedPwd = await invoke("get_connection_password", { id: id.value });
        if (storedPwd) pwd = storedPwd;
      } catch (e) {
        console.error("Failed to retrieve stored password", e);
      }
    }

    await connectionStore.saveConnection(connectionPayload.value, pwd);
    emit("saved");
    resetForm();
  } catch (err) {
    testResult.value = {
      success: false,
      message: `Failed to save: ${err}`,
    };
  }
}
</script>

<template>
  <section class="pma-content">
    <h2 class="pma-page-title">
      {{ editConnectionId ? "Edit Connection" : "Create Connection" }}
    </h2>

    <div class="pma-section-mb">
      <PmaSectionHeader icon="⚙️" :label="editConnectionId ? 'Modify Connection Details' : 'New Connection Settings'">
        <form @submit.prevent="handleSave">
          <div class="pma-row">
            <!-- Connection Alias -->
            <div class="pma-col-3">
              <label class="pma-form-label">Connection Name</label>
            </div>
            <div class="pma-col-9">
              <input
                v-model="name"
                class="pma-sidebar-filter-input"
                type="text"
                placeholder='e.g., Localhost MySQL'
                required
              />
            </div>

            <!-- Driver Select -->
            <div class="pma-col-3">
              <label class="pma-form-label">Database Driver</label>
            </div>
            <div class="pma-col-9">
              <select v-model="driver" class="pma-sidebar-server-select">
                <option value="mysql">MySQL</option>
                <option value="postgres">PostgreSQL</option>
                <option value="sqlite">SQLite</option>
              </select>
            </div>

            <template v-if="!isSqlite">
              <!-- Host -->
              <div class="pma-col-3">
                <label class="pma-form-label">Host</label>
              </div>
              <div class="pma-col-9">
                <input
                  v-model="host"
                  class="pma-sidebar-filter-input"
                  type="text"
                  placeholder="localhost"
                />
              </div>

              <!-- Port -->
              <div class="pma-col-3">
                <label class="pma-form-label">Port</label>
              </div>
              <div class="pma-col-9">
                <input
                  v-model="port"
                  class="pma-sidebar-filter-input"
                  type="number"
                  placeholder="3306"
                />
              </div>

              <!-- Username -->
              <div class="pma-col-3">
                <label class="pma-form-label">Username</label>
              </div>
              <div class="pma-col-9">
                <input
                  v-model="username"
                  class="pma-sidebar-filter-input"
                  type="text"
                  placeholder="root"
                />
              </div>

              <!-- Password -->
              <div class="pma-col-3">
                <label class="pma-form-label">Password</label>
              </div>
              <div class="pma-col-9">
                <input
                  v-model="password"
                  class="pma-sidebar-filter-input"
                  type="password"
                  placeholder="••••••••"
                />
                <span v-if="editConnectionId" style="font-size: 10px; color: var(--color-pma-text-muted);">
                  Leave blank to keep current password
                </span>
              </div>
            </template>

            <!-- Database / File Path -->
            <div class="pma-col-3">
              <label class="pma-form-label">
                {{ isSqlite ? "SQLite File Path" : "Database Name" }}
              </label>
            </div>
            <div class="pma-col-9">
              <input
                v-model="database"
                class="pma-sidebar-filter-input"
                type="text"
                :placeholder="isSqlite ? 'e.g., path/to/local.db or :memory:' : 'e.g., my_app_db'"
              />
            </div>

            <!-- SSL Enabled -->
            <template v-if="!isSqlite">
              <div class="pma-col-3">
                <label class="pma-form-label">SSL Mode</label>
              </div>
              <div class="pma-col-9">
                <label class="pma-form-label">
                  <input
                    type="checkbox"
                    class="pma-checkbox"
                    v-model="sslEnabled"
                  />
                  Enable SSL / TLS Connection
                </label>
              </div>
            </template>

            <div class="pma-col-12">
              <hr class="pma-divider" />
            </div>

            <!-- Form Actions -->
            <div class="pma-col-12">
              <div class="pma-section-footer" style="display: flex; gap: 8px;">
                <PmaBtn type="button" @click="handleTest" :disabled="testing">
                  {{ testing ? "Testing..." : "Test Connection" }}
                </PmaBtn>
                <PmaBtn type="submit" style="font-weight: bold; background-color: var(--color-pma-blue); color: white;">
                  Save Connection
                </PmaBtn>
                <PmaBtn v-if="editConnectionId" type="button" @click="emit('cancel')">
                  Cancel
                </PmaBtn>
              </div>
            </div>
          </div>
        </form>
      </PmaSectionHeader>
    </div>

    <!-- Test Result Notifications -->
    <div v-if="testResult" class="pma-section-mb">
      <PmaInfoBox :variant="testResult.success ? 'info' : 'warning'">
        <strong>Connection Test Status:</strong>
        <p>{{ testResult.message }}</p>
      </PmaInfoBox>
    </div>
  </section>
</template>
