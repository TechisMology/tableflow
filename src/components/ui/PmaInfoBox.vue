<script setup>
/**
 * PmaInfoBox.vue
 * Notification box — info atau warning.
 * Props:
 *   - variant: "info" | "warning"
 *   - icon: string (default sesuai variant)
 * Slot default: isi konten notifikasi
 */
const props = defineProps({
  variant: {
    type: String,
    default: "info",
    validator: (v) => ["info", "warning"].includes(v),
  },
  icon: {
    type: String,
    default: null,
  },
});

const defaultIcons = {
  info: "⚠️",
  warning: "⚠️",
};

const resolvedIcon = props.icon ?? defaultIcons[props.variant];
</script>

<template>
  <div :class="variant === 'warning' ? 'pma-warning-box' : 'pma-info-box'">
    <span class="pma-box-icon">{{ resolvedIcon }}</span>
    <div class="pma-box-content">
      <slot />
    </div>
  </div>
</template>
