<script setup>
/**
 * PmaTable.vue
 * Reusable table wrapper dengan styling pma-table.
 * Gunakan slot "head" untuk <thead> dan slot default untuk <tbody>.
 *
 * Slots:
 *   - head: konten <thead>
 *   - default: konten <tbody>
 */
</script>

<template>
  <div class="pma-table-wrapper">
    <table class="pma-table">
      <thead>
        <slot name="head" />
      </thead>
      <tbody>
        <slot />
      </tbody>
    </table>
  </div>
</template>
