import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export interface DbConnection {
  id: string;
  name: string;
  driver: "mysql" | "postgres" | "sqlite";
  host?: string;
  port?: number;
  username?: string;
  database?: string;
  ssl_enabled?: boolean;
}

export interface SidebarDb {
  name: string;
  expanded: boolean;
  tables: string[];
  loading: boolean;
}

export const useConnectionStore = defineStore("connections", {
  state: () => ({
    connections: [] as DbConnection[],
    activeConnection: null as DbConnection | null,
    databases: [] as string[],
    sidebarDbs: [] as SidebarDb[],
    tables: [] as string[],
    loading: false,
    error: null as string | null,
  }),

  actions: {
    initSidebarDbs(dbNames: string[]) {
      this.sidebarDbs = dbNames.map(name => ({
        name,
        expanded: false,
        tables: [],
        loading: false
      }));
    },

    async toggleDbExpanded(dbName: string) {
      const db = this.sidebarDbs.find(d => d.name === dbName);
      if (!db) return;
      db.expanded = !db.expanded;
      if (db.expanded && db.tables.length === 0 && !db.loading) {
        db.loading = true;
        try {
          let password = null;
          try {
            password = await invoke<string | null>("get_connection_password", { id: this.activeConnection!.id });
          } catch (e) {
            console.warn("Could not fetch password from keyring");
          }
          const tbls = await invoke<string[]>("list_connection_tables", { 
            conn: this.activeConnection, 
            password, 
            database: dbName
          });
          db.tables = tbls;
        } catch (err) {
          console.error("Failed to load tables for", dbName, err);
        } finally {
          db.loading = false;
        }
      }
    },

    closeSidebarDb(dbName: string) {
      this.sidebarDbs = this.sidebarDbs.filter(d => d.name !== dbName);
    },

    async openSidebarDb(dbName: string) {
      let db = this.sidebarDbs.find(d => d.name === dbName);
      if (!db) {
        db = {
          name: dbName,
          expanded: false,
          tables: [],
          loading: false
        };
        this.sidebarDbs.push(db);
      }
      if (!db.expanded) {
        await this.toggleDbExpanded(dbName);
      }
    },

    async loadDatabases(conn: DbConnection) {
      this.error = null;
      try {
        let password = null;
        try {
          password = await invoke<string | null>("get_connection_password", { id: conn.id });
        } catch (e) {
          console.warn("Could not fetch password from keyring, trying connection without it");
        }
        const dbs = await invoke<string[]>("list_connection_databases", { conn, password });
        this.databases = dbs;
      } catch (err: any) {
        this.error = err.toString();
        console.error("Failed to load databases:", err);
        throw err;
      }
    },

    async loadTables(conn: DbConnection, dbName?: string) {
      this.error = null;
      try {
        let password = null;
        try {
          password = await invoke<string | null>("get_connection_password", { id: conn.id });
        } catch (e) {
          console.warn("Could not fetch password from keyring, trying connection without it");
        }
        const tbls = await invoke<string[]>("list_connection_tables", { 
          conn, 
          password, 
          database: dbName || conn.database || null 
        });
        this.tables = tbls;
      } catch (err: any) {
        this.error = err.toString();
        console.error("Failed to load tables:", err);
        throw err;
      }
    },

    async loadConnections() {
      this.loading = true;
      this.error = null;
      try {
        const conns = await invoke<DbConnection[]>("list_connections");
        this.connections = conns;
      } catch (err: any) {
        this.error = err.toString();
        console.error("Failed to load connections:", err);
      } finally {
        this.loading = false;
      }
    },

    async saveConnection(conn: DbConnection, password?: string) {
      this.loading = true;
      this.error = null;
      try {
        await invoke("save_connection", { conn, password: password || null });
        await this.loadConnections();
      } catch (err: any) {
        this.error = err.toString();
        console.error("Failed to save connection:", err);
        throw err;
      } finally {
        this.loading = false;
      }
    },

    async deleteConnection(id: string) {
      this.loading = true;
      this.error = null;
      try {
        await invoke("delete_connection", { id });
        if (this.activeConnection?.id === id) {
          this.activeConnection = null;
        }
        await this.loadConnections();
      } catch (err: any) {
        this.error = err.toString();
        console.error("Failed to delete connection:", err);
        throw err;
      } finally {
        this.loading = false;
      }
    },

    setActiveConnection(conn: DbConnection | null) {
      this.activeConnection = conn;
    },
  },
});
