<script setup lang="ts">
import { SplitterGroup, SplitterPanel } from "reka-ui";
import { Search } from "lucide-vue-next";
import { InputGroup, InputGroupAddon, InputGroupInput } from "@/components/ui/input-group";
import Button from "@/components/ui/button/Button.vue";
import { invoke } from "@tauri-apps/api/core";

const newConnection = async () => {
    try {
        await invoke("new_connection");
    } catch (error) {
        console.error("Error creating window:", error);
    }
};
</script>

<template>
    <div class="w-full h-screen">
        <SplitterGroup id="splitter-group-1" direction="horizontal">
            <SplitterPanel :defaultSize="30" class="bg-gray-50 border-r flex items-center justify-center">
                Sidebar (Connections)
            </SplitterPanel>

            <SplitterPanel :min-size="20">
                <SplitterGroup id="splitter-group-2" direction="vertical">
                    <div class="flex gap-2 p-2 border-b">
                        <InputGroup class="flex-1">
                            <InputGroupInput placeholder="Search..." />
                            <InputGroupAddon> <Search class="w-4 h-4" /> </InputGroupAddon>
                        </InputGroup>
                        <Button variant="outline" @click="newConnection">New connection</Button>
                    </div>

                    <SplitterPanel class="bg-white flex items-center justify-center"> Content Area </SplitterPanel>
                </SplitterGroup>
            </SplitterPanel>
        </SplitterGroup>
    </div>
</template>
