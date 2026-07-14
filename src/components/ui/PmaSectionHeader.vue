<script setup>
/**
 * PmaSectionHeader.vue
 * Header label untuk sebuah section/panel.
 * Props:
 *   - icon: string   — emoji icon (opsional)
 *   - label: string  — teks label section
 *
 * Slot default: konten body section (opsional).
 * Jika ada slot, akan dirender dalam pma-section-body.
 */
defineProps({
  icon: {
    type: String,
    default: null,
  },
  label: {
    type: String,
    required: true,
  },
});
</script>

<template>
  <div>
    <!-- Section Header Label -->
    <div class="pma-section-header">
      <span v-if="icon">{{ icon }}</span>
      <span>{{ label }}</span>
    </div>
    <!-- Section Body (slot) -->
    <div v-if="$slots.default" class="pma-section-body">
      <slot />
    </div>
  </div>
</template>
