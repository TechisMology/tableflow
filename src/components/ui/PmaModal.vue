<script setup>
/**
 * PmaModal.vue
 * Modal dialog konfirmasi gaya premium.
 * Props:
 *   - show: boolean
 *   - title: string
 *   - message: string
 * Emits:
 *   - confirm
 *   - cancel
 */
defineProps({
  show: {
    type: Boolean,
    required: true,
  },
  title: {
    type: String,
    default: "Confirmation",
  },
  message: {
    type: String,
    default: "Are you sure you want to perform this action?",
  },
});

const emit = defineEmits(["confirm", "cancel"]);
</script>

<template>
  <Teleport to="body">
    <div
      v-if="show"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-xs transition-opacity"
    >
      <div class="bg-white border border-[var(--color-pma-border)] shadow-xl rounded-md w-full max-w-md mx-4 overflow-hidden transform transition-all">
        <!-- Header -->
        <div class="px-4 py-3 bg-[var(--color-pma-bg-header-to)] border-b border-[var(--color-pma-border)] flex items-center justify-between">
          <h3 class="font-bold text-[12px] text-[var(--color-pma-text)] flex items-center gap-1.5">
            ⚠️ {{ title }}
          </h3>
          <button 
            class="text-gray-400 hover:text-[var(--color-pma-text-danger)] font-bold text-[13px] border-none bg-transparent cursor-pointer"
            @click="emit('cancel')"
          >
            ✕
          </button>
        </div>

        <!-- Body -->
        <div class="p-5 text-[11px] text-[var(--color-pma-text)] leading-relaxed bg-[var(--color-pma-bg-white)]">
          {{ message }}
        </div>

        <!-- Footer -->
        <div class="px-4 py-3 bg-gray-50 border-t border-[var(--color-pma-border)] flex justify-end gap-2">
          <button 
            class="px-3 py-1 text-[11px] border border-[var(--color-pma-border)] rounded-sm bg-white hover:bg-gray-100 cursor-pointer"
            @click="emit('cancel')"
          >
            Cancel
          </button>
          <button 
            class="px-3 py-1 text-[11px] font-bold text-white bg-[var(--color-pma-text-danger)] hover:bg-red-700 rounded-sm border-none cursor-pointer"
            @click="emit('confirm')"
          >
            Drop Table
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>
