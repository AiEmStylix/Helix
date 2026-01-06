<script setup lang="ts">
import { Field, FieldGroup, FieldLabel, FieldSet } from "@/components/ui/field";

import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { ButtonGroup } from "@/components/ui/button-group";
import { Folders } from "lucide-vue-next";

import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-dialog";
import { ref } from "vue";

const databaseName = ref<string>();
const databasePath = ref<string>();

const browseFile = async () => {
    const path = await open({
        multiple: false,
        directory: false,
        filters: [
            {
                name: "Sqlite Database",
                extensions: ["db", "db3", "sqlite", "sqlite3", "s3db", "sl3"],
            },
        ],
    });
    if (!path) {
        return;
    }
    databasePath.value = path;
};

const handleSubmit = async () => {
    try {
        const window = getCurrentWindow();
        await invoke("save_connection", { dbName: databaseName.value, path: databasePath.value });
        window.close();
    } catch (error) {
        console.error("Error: " + error);
    }
};
</script>

<template>
    <form @submit.prevent="handleSubmit" class="min-w-md bg-white p-4">
        <FieldSet>
            <FieldGroup>
                <Field>
                    <FieldLabel for="db-name">Database Name</FieldLabel>
                    <Input id="db-name" v-model="databaseName" placeholder="Database name..." />
                </Field>
                <Field>
                    <FieldLabel for="path"> Database path </FieldLabel>
                    <ButtonGroup>
                        <Input class="bg-gray-100" disabled placeholder="Browse file" v-model="databasePath" />
                        <Button @click="browseFile" variant="outline" aria-label="Browse">
                            <Folders />
                        </Button>
                    </ButtonGroup>
                </Field>
                <Field orientation="horizontal">
                    <Button type="submit"> Submit </Button>
                    <Button variant="outline" type="button"> Cancel </Button>
                </Field>
            </FieldGroup>
        </FieldSet>
    </form>
</template>
