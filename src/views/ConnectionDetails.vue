<script setup lang="ts">
import SqliteConnection from "@/components/ui/SqliteConnection.vue";
import type { DatabaseType } from "@/types/DatabaseType";
import { computed, type Component } from "vue";

const props = defineProps<{
    database: DatabaseType;
}>();

const componentMap: Partial<Record<DatabaseType, Component>> = {
    sqlite: SqliteConnection,
};

const activeComponent = computed(() => componentMap[props.database] ?? null);
</script>
<template>
    <div class="bg-gray-800 min-h-screen flex justify-center items-center">
        <component v-if="activeComponent" :is="activeComponent" />
    </div>
</template>
