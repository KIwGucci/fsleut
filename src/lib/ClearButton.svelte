<script lang="ts">
    export let search_word: string = "";
    export let file_extention: string = "";

    function word_clear() {
        search_word = "";
        file_extention = "";
    }
</script>

<button on:click={word_clear}>Clear</button>

<style>
  :focus {
    border-color: rgb(85, 85, 201);
  }
</style>
