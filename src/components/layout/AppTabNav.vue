<script setup>
/**
 * AppTabNav.vue
 * Tab navigation bar — phpMyAdmin style tabs
 * Props:
 *   - tabs: Array<{ id: string, icon: string, label: string }>
 *   - activeTab: string — id tab yang aktif
 * Emits:
 *   - tab-change: string — id tab yang diklik
 */
const props = defineProps({
  tabs: {
    type: Array,
    required: true,
  },
  activeTab: {
    type: String,
    default: "users",
  },
});

const emit = defineEmits(["tab-change", "tab-close"]);
</script>

<template>
  <nav class="pma-tab-nav" role="tablist">
    <div
      v-for="tab in tabs"
      :key="tab.id"
      class="pma-tab flex items-center gap-1"
      :class="{ active: activeTab === tab.id }"
      role="tab"
      :aria-selected="activeTab === tab.id"
      :tabindex="activeTab === tab.id ? 0 : -1"
      @click="emit('tab-change', tab.id)"
    >
      <span class="pma-tab-icon">{{ tab.icon }}</span>
      <span>{{ tab.label }}</span>
      <span
        v-if="tab.closable"
        class="ml-1 cursor-pointer font-bold hover:text-[var(--color-pma-text-danger)] text-[9px]"
        @click.stop="emit('tab-close', tab.id)"
      >
        ✕
      </span>
    </div>
  </nav>
</template>
