<script setup>
/**
 * AppConsoleFooter.vue
 * Console panel dengan Monaco Editor SQL + autocomplete + drag resize.
 *
 * Fitur:
 * - Monaco Editor dengan language SQL + dark theme (vs-dark)
 * - Autocomplete SQL keyword + fungsi + nama tabel dummy
 * - Drag resize: geser resizer ke atas/bawah untuk ubah tinggi panel
 * - Toggle collapse/expand dengan klik judul "Console"
 * - Toolbar: Run, Clear, Copy, History (dummy)
 * - Status bar: info cursor + database aktif
 * - Result area: tampilkan output eksekusi SQL (dummy)
 *
 * Props:
 *   - activeDb: string — database yang sedang aktif (untuk SQL context)
 */
import { ref, onMounted, onBeforeUnmount, shallowRef, nextTick } from "vue";
import loader from "@monaco-editor/loader";

const props = defineProps({
  activeDb: {
    type: String,
    default: "sakila",
  },
});

// ── State ────────────────────────────────────────────────────
const collapsed = ref(false);
const consoleHeight = ref(220); // px — default terbuka
const MIN_HEIGHT = 80;
const MAX_HEIGHT = 600;
const COLLAPSED_HEIGHT = 28;

const editorContainer = ref(null);
const resizerEl = ref(null);
const footerEl = ref(null);
const monacoEditor = shallowRef(null);
const monacoInstance = shallowRef(null);

const sqlValue = ref(
  `-- TMyadmin SQL Console\n-- Database: ${props.activeDb}\n\nSELECT * FROM actor LIMIT 10;\n`
);
const statusLine = ref(1);
const statusCol = ref(1);
const resultVisible = ref(false);
const resultStatus = ref("ok"); // 'ok' | 'error'
const resultMessage = ref("");
const resultRows = ref([]);
const queryHistory = ref([]);

// ── Drag Resize Logic ─────────────────────────────────────────
const isDragging = ref(false);
let dragStartY = 0;
let dragStartH = 0;

function onResizerMousedown(e) {
  if (collapsed.value) return;
  isDragging.value = true;
  dragStartY = e.clientY;
  dragStartH = consoleHeight.value;
  e.preventDefault();
}

function onMousemove(e) {
  if (!isDragging.value) return;
  const delta = dragStartY - e.clientY; // geser ke atas = delta positif = tinggi bertambah
  const newH = Math.min(MAX_HEIGHT, Math.max(MIN_HEIGHT, dragStartH + delta));
  consoleHeight.value = newH;
  // Layout ulang Monaco agar sesuai ukuran baru
  nextTick(() => monacoEditor.value?.layout());
}

function onMouseup() {
  isDragging.value = false;
}

// ── Toggle Collapse ──────────────────────────────────────────
function toggleCollapse() {
  collapsed.value = !collapsed.value;
  nextTick(() => monacoEditor.value?.layout());
}

// ── Monaco Init ──────────────────────────────────────────────
async function initMonaco() {
  const monaco = await loader.init();
  monacoInstance.value = monaco;

  // SQL autocomplete suggestions
  monaco.languages.registerCompletionItemProvider("sql", {
    provideCompletionItems(model, position) {
      const word = model.getWordUntilPosition(position);
      const range = {
        startLineNumber: position.lineNumber,
        endLineNumber: position.lineNumber,
        startColumn: word.startColumn,
        endColumn: word.endColumn,
      };

      const keywords = [
        "SELECT", "FROM", "WHERE", "AND", "OR", "NOT", "IN", "IS", "NULL",
        "INSERT", "INTO", "VALUES", "UPDATE", "SET", "DELETE",
        "CREATE", "TABLE", "DATABASE", "DROP", "ALTER", "ADD", "COLUMN",
        "JOIN", "LEFT JOIN", "RIGHT JOIN", "INNER JOIN", "OUTER JOIN", "ON",
        "GROUP BY", "ORDER BY", "HAVING", "LIMIT", "OFFSET",
        "DISTINCT", "AS", "UNION", "ALL", "EXISTS", "BETWEEN",
        "LIKE", "CASE", "WHEN", "THEN", "ELSE", "END",
        "PRIMARY KEY", "FOREIGN KEY", "REFERENCES", "INDEX", "UNIQUE",
        "AUTO_INCREMENT", "NOT NULL", "DEFAULT", "CONSTRAINT",
        "BEGIN", "COMMIT", "ROLLBACK", "TRANSACTION",
        "SHOW", "DATABASES", "TABLES", "DESCRIBE", "EXPLAIN",
        "USE", "GRANT", "REVOKE", "FLUSH", "PRIVILEGES",
      ];

      const functions = [
        "COUNT", "SUM", "AVG", "MIN", "MAX",
        "NOW()", "CURDATE()", "CURTIME()", "DATE()", "TIME()",
        "YEAR()", "MONTH()", "DAY()", "HOUR()", "MINUTE()", "SECOND()",
        "CONCAT()", "LENGTH()", "UPPER()", "LOWER()", "TRIM()",
        "SUBSTRING()", "REPLACE()", "COALESCE()", "IFNULL()", "IF()",
        "ROUND()", "FLOOR()", "CEIL()", "ABS()", "MOD()",
        "GROUP_CONCAT()", "CAST()", "CONVERT()", "FORMAT()",
        "UUID()", "MD5()", "SHA1()", "SHA2()",
      ];

      // Tabel dari database aktif (dummy sesuai sakila)
      const tables = [
        "actor", "address", "category", "city", "country",
        "customer", "film", "film_actor", "film_category", "film_text",
        "inventory", "language", "payment", "rental", "staff",
        "store", "user", "orders", "products", "employees",
      ];

      const kwSuggestions = keywords.map((kw) => ({
        label: kw,
        kind: monaco.languages.CompletionItemKind.Keyword,
        insertText: kw,
        range,
        detail: "SQL Keyword",
      }));

      const fnSuggestions = functions.map((fn) => ({
        label: fn,
        kind: monaco.languages.CompletionItemKind.Function,
        insertText: fn.replace("()", "($1)"),
        insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
        range,
        detail: "SQL Function",
      }));

      const tblSuggestions = tables.map((tbl) => ({
        label: tbl,
        kind: monaco.languages.CompletionItemKind.Class,
        insertText: tbl,
        range,
        detail: `Table in ${props.activeDb}`,
      }));

      return { suggestions: [...kwSuggestions, ...fnSuggestions, ...tblSuggestions] };
    },
  });

  // Create editor instance
  const editor = monaco.editor.create(editorContainer.value, {
    value: sqlValue.value,
    language: "sql",
    theme: "vs-dark",
    fontSize: 12,
    fontFamily: "'Consolas', 'Courier New', monospace",
    lineHeight: 18,
    minimap: { enabled: false },
    scrollbar: {
      vertical: "auto",
      horizontal: "auto",
    },
    wordWrap: "off",
    lineNumbers: "on",
    renderLineHighlight: "line",
    automaticLayout: false, // manual via layout()
    suggestOnTriggerCharacters: true,
    quickSuggestions: { other: true, comments: false, strings: false },
    parameterHints: { enabled: true },
    suggest: { showKeywords: true },
    tabSize: 2,
    padding: { top: 6, bottom: 6 },
    smoothScrolling: true,
    cursorBlinking: "smooth",
    scrollBeyondLastLine: false,
  });

  monacoEditor.value = editor;

  // Track cursor pos untuk status bar
  editor.onDidChangeCursorPosition((e) => {
    statusLine.value = e.position.lineNumber;
    statusCol.value = e.position.column;
  });

  // Ctrl+Enter / Cmd+Enter → run query
  editor.addCommand(
    monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter,
    runQuery
  );
}

// ── Run Query (dummy simulation) ─────────────────────────────
function runQuery() {
  const value = monacoEditor.value?.getValue() || "";
  const trimmed = value.trim();
  if (!trimmed) return;

  queryHistory.value.unshift({ sql: trimmed, ts: new Date().toLocaleTimeString() });

  // Simulasi hasil
  const upper = trimmed.toUpperCase();
  if (upper.startsWith("SELECT")) {
    resultStatus.value = "ok";
    resultMessage.value = `Query OK — 10 rows returned in 0.023s`;
    resultRows.value = [
      "→ Returning dummy result set. Connect to real backend to show actual data.",
      "→ Row 1: id=1, name='Example', created_at='2024-01-01'",
      "→ Row 2: id=2, name='Sample', created_at='2024-01-02'",
      "→ ...(10 rows total)",
    ];
  } else if (upper.startsWith("SHOW")) {
    resultStatus.value = "ok";
    resultMessage.value = `Query OK — showing result`;
    resultRows.value = ["→ " + props.activeDb + " (current database)"];
  } else if (upper.startsWith("INSERT") || upper.startsWith("UPDATE") || upper.startsWith("DELETE")) {
    resultStatus.value = "ok";
    resultMessage.value = `Query OK — 1 row affected in 0.005s`;
    resultRows.value = [];
  } else {
    resultStatus.value = "ok";
    resultMessage.value = `Query executed — no rows returned`;
    resultRows.value = [];
  }
  resultVisible.value = true;
}

function clearEditor() {
  monacoEditor.value?.setValue("");
  resultVisible.value = false;
}

function copyQuery() {
  const val = monacoEditor.value?.getValue() || "";
  navigator.clipboard.writeText(val);
}

// ── Lifecycle ────────────────────────────────────────────────
onMounted(() => {
  initMonaco();
  window.addEventListener("mousemove", onMousemove);
  window.addEventListener("mouseup", onMouseup);
});

onBeforeUnmount(() => {
  monacoEditor.value?.dispose();
  window.removeEventListener("mousemove", onMousemove);
  window.removeEventListener("mouseup", onMouseup);
});
</script>

<template>
  <!-- Console Footer container — tinggi dinamis via CSS var -->
  <footer
    ref="footerEl"
    class="pma-console-footer"
    :class="{ collapsed }"
    :style="`height: ${collapsed ? 28 : consoleHeight}px`"
  >

    <!-- Drag Resizer — di paling atas footer -->
    <div
      ref="resizerEl"
      class="pma-console-resizer"
      :class="{ dragging: isDragging }"
      @mousedown="onResizerMousedown"
    ></div>

    <!-- Toolbar -->
    <div class="pma-console-toolbar">
      <!-- Title / Toggle -->
      <div class="pma-console-toolbar-title" @click="toggleCollapse">
        <span>⬛</span>
        <span>Console</span>
        <span>{{ collapsed ? '▲' : '▼' }}</span>
      </div>

      <div class="pma-console-toolbar-sep"></div>

      <!-- Tombol aksi — hanya tampil saat expand -->
      <template v-if="!collapsed">
        <button class="pma-console-action-btn run" title="Run (Ctrl+Enter)" @click="runQuery">
          ▶ Run
        </button>
        <button class="pma-console-action-btn" title="Clear editor" @click="clearEditor">
          🗑 Clear
        </button>
        <button class="pma-console-action-btn" title="Copy SQL" @click="copyQuery">
          📋 Copy
        </button>

        <div class="pma-console-toolbar-sep"></div>

        <!-- DB Context Badge -->
        <span class="pma-console-action-btn">
          🗄️ {{ activeDb }}
        </span>

        <div class="pma-console-toolbar-spacer"></div>

        <!-- Shortcut hint -->
        <span class="pma-console-toolbar-title">Ctrl+Enter to run</span>
      </template>
    </div>

    <!-- Body (editor + result) — hanya tampil saat expand -->
    <div v-if="!collapsed" class="pma-console-body">

      <!-- Monaco Editor -->
      <div class="pma-console-editor">
        <div ref="editorContainer" class="pma-console-editor-mount"></div>
      </div>

      <!-- Result Area -->
      <div v-if="resultVisible" class="pma-console-result">
        <div class="pma-console-result-head">
          <span :class="resultStatus === 'ok' ? 'pma-console-result-ok' : 'pma-console-result-err'">
            {{ resultStatus === 'ok' ? '✔' : '✖' }}
          </span>
          <span>{{ resultMessage }}</span>
        </div>
        <div v-if="resultRows.length" class="pma-console-result-rows">
          <div v-for="(row, i) in resultRows" :key="i">{{ row }}</div>
        </div>
      </div>

    </div>

    <!-- Status Bar — hanya tampil saat expand -->
    <div v-if="!collapsed" class="pma-console-statusbar">
      <span class="pma-console-statusbar-text">SQL</span>
      <span class="pma-console-statusbar-sep">|</span>
      <span class="pma-console-statusbar-text">Ln {{ statusLine }}, Col {{ statusCol }}</span>
      <span class="pma-console-statusbar-sep">|</span>
      <span class="pma-console-statusbar-text">{{ activeDb }}</span>
      <span class="pma-console-statusbar-sep">|</span>
      <span class="pma-console-statusbar-text">UTF-8</span>
    </div>

  </footer>
</template>
