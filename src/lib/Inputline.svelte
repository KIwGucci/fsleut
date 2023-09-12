<script lang="ts">
    export let invalue: string;
    export let inplace: string;
</script>


<input
    type="text"
    bind:value={invalue}
    placeholder={inplace}
    autocapitalize="off"
    autocomplete="off"
/>

<style>
  :focus {
    border-color: rgb(85, 85, 201);
  }
</style>