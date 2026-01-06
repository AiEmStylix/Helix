<script setup lang="ts">
import { Search } from "lucide-vue-next";
import { InputGroup, InputGroupAddon, InputGroupInput } from "@/components/ui/input-group";
import Button from "@/components/ui/button/Button.vue";
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { Connection } from "@/types/Connection";

const availableConnections = ref<Connection[]>();
const newConnection = async () => {
    try {
        await invoke("new_connection");
    } catch (error) {
        console.error("Error creating window:", error);
    }
};

onMounted(async () => {
    const connections = await invoke<Connection[]>("sqlite_list_connections");
    availableConnections.value = connections;
});
</script>

<template>
    <div class="w-full h-screen">
        <div class="flex gap-2 p-2 border-b">
            <InputGroup class="flex-1">
                <InputGroupInput placeholder="Search..." />
                <InputGroupAddon> <Search class="w-4 h-4" /> </InputGroupAddon>
            </InputGroup>
            <Button variant="outline" @click="newConnection">New connection</Button>
        </div>
        <div>
            <div v-for="connection in availableConnections" :key="connection.name">
                {{ connection.name }}
            </div>
        </div>
    </div>
</template>
