<script setup lang="ts">
import { ref, type Component } from "vue";
import { InputGroup, InputGroupAddon, InputGroupInput } from "@/components/ui/input-group";
import { MariaDbIcon, MySqlIcon, PostgreSqlIcon, SqLiteIcon } from "vue3-simple-icons";
import { Database, SearchIcon, HardDrive } from "lucide-vue-next";
import Button from "@/components/ui/button/Button.vue";
import { useRouter } from "vue-router";

interface DatabaseType {
    id: string;
    name: string;
    icon: Component;
    color: string;
}

const databaseTypes: DatabaseType[] = [
    { id: "sqlite", name: "SQLite", icon: SqLiteIcon, color: "text-blue-400" },
    { id: "postgres", name: "PostgreSQL", icon: PostgreSqlIcon, color: "text-blue-500" },
    { id: "mysql", name: "MySQL", icon: MySqlIcon, color: "text-amber-500" },
    { id: "mariadb", name: "MariaDB", icon: MariaDbIcon, color: "text-blue-400" },
];

const selectedDb = ref<string | null>(null);

const router = useRouter();

const gotoConnect = () => {
    if (!selectedDb.value) return;
    router.push(`/connect/${selectedDb.value}`);
};
</script>

<template>
    <div
        class="flex items-center justify-center h-screen bg-neutral-950 p-4 font-sans antialiased selection:bg-blue-500/30"
    >
        <div class="flex p-6 flex-col min-w-full h-full bg-white rounded-md gap-4">
            <!-- Header  -->
            <div class="flex flex-row gap-5">
                <div
                    class="w-11 h-11 rounded bg-linear-to-b from-gray-800 to-gray-600 flex items-center justify-center border border-[#555] shadow-[0_1px_2px_rgba(0,0,0,0.3)]"
                >
                    <Database class="w-6 h-6 text-[#DDD]" />
                </div>
                <div>
                    <h2 class="text-sm font-semibold tracking-tight">Select a driver</h2>
                    <p class="text-xs -mt-0.5">Choose the database you want to connect to</p>
                </div>
            </div>

            <!-- Search box -->
            <div>
                <InputGroup>
                    <InputGroupInput placeholder="Search..." />
                    <InputGroupAddon>
                        <SearchIcon />
                    </InputGroupAddon>
                </InputGroup>
            </div>

            <!-- Grid List -->
            <div class="flex-1 overflow-y-auto px-5 pb-5 custom-scrollbar">
                <div class="grid grid-cols-2 gap-2">
                    <Button
                        v-for="item in databaseTypes"
                        :key="item.id"
                        size="sm"
                        class="flex items-center gap-2 px-3 py-2"
                        :class="[selectedDb === item.id && 'ring-2 ring-blue-500 bg-blue-50']"
                        @click="selectedDb = item.id"
                    >
                        <div class="flex items-center justify-center w-6 h-6 text-gray-700">
                            <component :is="item.icon" class="w-4 h-4 text-white" />
                        </div>

                        <span class="text-sm font-medium">
                            {{ item.name }}
                        </span>
                    </Button>
                </div>
            </div>

            <div class="px-5 py-4 flex items-center justify-between">
                <Button class="font-medium">
                    <HardDrive class="w-3.5 h-3.5" />
                    Import from URL...
                </Button>

                <div class="flex gap-2">
                    <Button variant="ghost" class=""> Cancel </Button>
                    <Button :disabled="!selectedDb" @click="gotoConnect"> Create </Button>
                </div>
            </div>
        </div>
    </div>
</template>
