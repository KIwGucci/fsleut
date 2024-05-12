<script lang="ts">
    export let is_file_open: boolean;
    let select_items = [
        { itemvalue: true, text: "File" },
        { itemvalue: false, text: "Folder" },
    ];
</script>

<select bind:value={is_file_open}>
    {#each select_items as item}
        <option value={item.itemvalue}>
            {item.text}
        </option>
    {/each}
</select>

<style>
    :focus {
        border-color: rgb(85, 85, 201);
    }
</style>
