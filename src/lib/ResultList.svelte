<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    export let result_items: string[];
    export let app_message: string;
    export let is_file_open: boolean;

    async function open_item(gopath: string, is_file_open: boolean) {
        app_message = await invoke("open_item", {
            inpath: gopath,
            isFileOpen: is_file_open,
        });
    }
</script>

<div id="result_list">
    <ol>
        {#each result_items as finditem}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
            <li
                on:click={() => open_item(finditem, is_file_open)}
                class="findlist"
            >
                {finditem}
            </li>
        {/each}
    </ol>
</div>

<style>
    #result_list {
        overflow: scroll;
        max-height: 260px;
    }
    .findlist {
        font-size: medium;
        text-align: left;
    }
    .findlist:hover {
        background-color: rgb(19, 19, 19);
        color: rgb(100, 231, 192);
    }
</style>
