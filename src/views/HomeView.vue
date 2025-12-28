<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { ref } from "vue";
import { Plug } from "lucide-vue-next";

const fileUrl = ref<string | null>(null);
const sqlCommand = ref<string | null>(null);

async function testQuery() {
    try {
        const result = await invoke<number>("query", {
            path: fileUrl.value,
            sql: sqlCommand.value,
        });

        console.log("Result:", result);
        alert("Query OK, rows affected = " + result);
    } catch (err) {
        console.error("Error:", err);
        alert("Error: " + err);
    }
}

const openDialog = async () => {
    const file = await open({
        multiple: false,
        directory: false,
    });
    fileUrl.value = file;
};
</script>

<template>
    <div class="flex min-w-screen min-h-screen "">
        <div class="bg-red-400">
            <button class="bg-blue-400 px-8 py-2 flex mx-8 my-4"><Plug />New connection</button>
        </div>

    </div>
</template>
