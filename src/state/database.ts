import { defineStore } from "pinia";
import { computed, ref } from "vue";

export const useDatabaseStore = defineStore("database", () => {
  const dbPath = ref<string | null>(null);
  const dbName = ref<string | null>(null);
  const connected = ref(false);

  //Getters
  const isConnected = computed(() => connected.value && !dbPath.value);

  //Actions
  function connect(path: string) {
    dbPath.value = path;
    connected.value = true;
  }

  return { dbPath, dbName, connected, isConnected, connect };
});
