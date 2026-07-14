<script setup>
/**
 * PmaActionLink.vue
 * Reusable inline action link dengan icon dan label.
 * Props:
 *   - icon: string   — emoji atau teks icon (contoh: "👤")
 *   - label: string  — teks label link
 *   - href: string   — URL tujuan (default "#")
 */
defineProps({
  icon: {
    type: String,
    default: "",
  },
  label: {
    type: String,
    required: true,
  },
  href: {
    type: String,
    default: "#",
  },
});
</script>

<template>
  <a :href="href" class="pma-action-link">
    <span v-if="icon" class="pma-action-link-icon">{{ icon }}</span>
    <span>{{ label }}</span>
  </a>
</template>
