<script setup>
/**
 * App.vue — Root layout
 * Susunan: AppSidebar (kiri) + pma-main (tengah) + AppDatabasePanel (kanan, show/hide)
 */
import { ref } from "vue";
import AppSidebar from "./components/layout/AppSidebar.vue";
import AppTopHeader from "./components/layout/AppTopHeader.vue";
import AppTabNav from "./components/layout/AppTabNav.vue";
import AppConsoleFooter from "./components/layout/AppConsoleFooter.vue";
import AppDatabasePanel from "./components/layout/AppDatabasePanel.vue";
import UserAccountsPage from "./components/pages/UserAccountsPage.vue";

// ── State ──────────────────────────────────────────────────
const activeTab = ref("users");
const serverName = ref("phpMyAdmin demo - MySQL");
const dbPanelOpen = ref(true);
const activeDb = ref("sakila");

function onTabChange(tabId) {
  activeTab.value = tabId;
}

function onDbSelect(name) {
  activeDb.value = name;
}
</script>

<template>
  <!-- Root: sidebar kiri + main + panel kanan -->
  <div class="pma-app">

    <!-- Sidebar Kiri -->
    <AppSidebar :current-server="serverName" />

    <!-- Area Konten Utama -->
    <main class="pma-main">

      <!-- Top Header — dengan toggle DB panel -->
      <AppTopHeader
        :server-name="serverName"
        :db-panel-open="dbPanelOpen"
        @toggle-db-panel="dbPanelOpen = !dbPanelOpen"
      />

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

      <!-- Console Footer (fixed) -->
      <AppConsoleFooter />

    </main>

    <!-- Database Panel Kanan (show/hide) -->
    <AppDatabasePanel
      :open="dbPanelOpen"
      :active-db="activeDb"
      @close="dbPanelOpen = false"
      @db-select="onDbSelect"
    />

  </div>
</template>
