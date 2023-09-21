<script lang="ts">
  export let search_word: string = "";
  export let file_extention: string = "";
  export let items:string[]=[];
  export let is_file_open:boolean=false;
  export let app_message="";

  function word_clear() {
    search_word = "";
    file_extention = "";
    items = [];
    is_file_open=false;
    app_message="";
  }
</script>

<button on:click|preventDefault={word_clear}>Clear</button>

<style>
  :focus {
    border-color: rgb(85, 85, 201);
  }
</style>
