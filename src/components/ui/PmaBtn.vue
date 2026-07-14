<script setup>
/**
 * PmaBtn.vue
 * Reusable button component.
 * Props:
 *   - type: string — HTML button type ("button" | "submit" | "reset")
 * Emits:
 *   - click
 * Slot default: label/content tombol
 */
defineProps({
  type: {
    type: String,
    default: "button",
  },
});

const emit = defineEmits(["click"]);
</script>

<template>
  <button
    :type="type"
    class="pma-btn"
    @click="emit('click', $event)"
  >
    <slot />
  </button>
</template>
