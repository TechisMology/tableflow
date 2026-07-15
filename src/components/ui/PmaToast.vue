<script setup>
/**
 * PmaToast.vue
 * Toast notifikasi premium:
 * - Error: Center screen
 * - Success: Bottom-right screen with autoclose progress bar
 */
import { ref, watch, onBeforeUnmount, onMounted, onUpdated, nextTick } from "vue";

const props = defineProps({
  show: {
    type: Boolean,
    required: true,
  },
  type: {
    type: String, // "success" | "error"
    default: "success",
  },
  message: {
    type: String,
    required: true,
  },
  duration: {
    type: Number,
    default: 3000, // ms
  },
});

const emit = defineEmits(["close"]);

const progressWidth = ref(100);
let timer = null;

watch(
  () => props.show,
  (isShowing) => {
    if (isShowing) {
      progressWidth.value = 100;
      
      // Auto close timer
      if (props.type === "success") {
        const duration = Number(props.duration) || 3000;
        const intervalTime = 30;
        const steps = duration / intervalTime;
        let currentStep = 0;
        
        if (timer) clearInterval(timer);
        timer = setInterval(() => {
          currentStep++;
          progressWidth.value = 100 - (currentStep / steps) * 100;
          if (currentStep >= steps) {
            clearInterval(timer);
            emit("close");
          }
        }, intervalTime);
      }
    } else {
      if (timer) clearInterval(timer);
    }
  },
  { immediate: true }
);

onBeforeUnmount(() => {
  if (timer) clearInterval(timer);
});
</script>

<template>
  <Teleport to="body">
    <!-- 1. ERROR TOAST (Center of the screen) -->
    <div 
      v-if="show && type === 'error'"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/25 backdrop-blur-xs"
    >
      <div class="bg-red-50 border-2 border-[var(--color-pma-text-danger)] p-5 rounded-md shadow-2xl max-w-md w-full mx-4 flex flex-col gap-3 relative transform transition-all">
        <div class="flex items-start gap-3">
          <span class="text-xl">❌</span>
          <div class="flex-1">
            <h4 class="font-bold text-[12px] text-[var(--color-pma-text-danger)] mb-1">Execution Error</h4>
            <p class="text-[11px] text-red-800 leading-relaxed font-mono whitespace-pre-wrap max-h-40 overflow-y-auto">
              {{ message }}
            </p>
          </div>
        </div>
        <div class="flex justify-end mt-2">
          <button 
            class="px-3 py-1 text-[11px] font-bold text-white bg-[var(--color-pma-text-danger)] rounded-sm border-none cursor-pointer hover:bg-red-800"
            @click="emit('close')"
          >
            OK
          </button>
        </div>
      </div>
    </div>

    <!-- 2. SUCCESS TOAST (Top-Left side of the screen, offset by sidebar and header height) -->
    <div 
      v-else-if="show && type === 'success'"
      class="fixed top-16 left-72 z-50 bg-white border border-green-300 rounded shadow-lg p-4 w-72 flex flex-col gap-2 transform transition-all"
    >
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <span class="text-green-600">✅</span>
          <span class="font-bold text-[11px] text-green-800">Success</span>
        </div>
        <button 
          class="text-gray-400 hover:text-gray-600 border-none bg-transparent cursor-pointer font-bold text-[10px]"
          @click="emit('close')"
        >
          ✕
        </button>
      </div>
      <div class="text-[11px] text-gray-700 leading-relaxed">
        {{ message }}
      </div>
      
      <!-- Autoclose Progress Bar -->
      <div class="w-full bg-gray-100 h-1 rounded overflow-hidden mt-1">
        <div 
          class="bg-green-500 h-full transition-all ease-linear"
          :style="{ width: progressWidth + '%' }"
        ></div>
      </div>
    </div>
  </Teleport>
</template>
