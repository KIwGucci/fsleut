<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import { desktopDir } from "@tauri-apps/api/path";
    // export let selected_files;
    export let selected_dir: string;

    async function select() {
        let desktoppath: string;
        if (selected_dir === "") {
            desktoppath = await desktopDir();
        } else {
            desktoppath = selected_dir;
        }

        let inselect = await open({
            directory: true,
            defaultPath: desktoppath,
        });
        if (inselect !== null && !Array.isArray(inselect)) {
            selected_dir = inselect;
        } else {
            selected_dir = "folder select error";
        }
    }
</script>

{#if selected_dir === ""}
    <button on:click|preventDefault={select}> 対象フォルダ選択 </button>
{:else}
    <button on:click|preventDefault={select}> 対象変更 </button>
{/if}

<style>
    :focus {
        border-color: rgb(85, 85, 201);
    }
</style>
