<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import FileDialog from "./lib/FileDialog.svelte";
  import Inputline from "./lib/Inputline.svelte";
  import ResultList from "./lib/ResultList.svelte";
  import ClearButton from "./lib/ClearButton.svelte";

  type SearchToken = {
    root_path: string;
    extention: string;
    search_word: string;
    deselection: string;
    is_dir:boolean;
  };

  type SearchResultStatus = {
    items: string[];
    message: string;
  };

  let selected_dir: string = "";
  let search_word: string = "";
  let file_extention: string = "";
  let is_dir:boolean=false;

  let search_result: SearchResultStatus = { items: [], message: "" };
  $: app_message = search_result.message;

  async function search_items() {
    let intoken: SearchToken = {
      root_path: selected_dir,
      extention: file_extention,
      search_word: search_word,
      deselection: "",
      is_dir:is_dir,
    };

    search_result = await invoke("search", { token: intoken });
  }
</script>

<main class="container">
  {#if selected_dir !== ""}
    <div class="row">
      <FileDialog bind:selected_dir />
      <Inputline bind:invalue={search_word} inplace="Search word" />
      <Inputline bind:invalue={file_extention} inplace="Extention" />
      <button on:click={search_items}> Search </button>
      <ClearButton bind:search_word bind:file_extention />
      
    </div>
  {:else}
    <FileDialog bind:selected_dir />
  {/if}

  <div class="row" id="infobar">
    {#if selected_dir !== ""}
      <div style="margin-right: auto;">
        {"対象=>"}{selected_dir}
      </div>
      <input type="checkbox" bind:checked={is_dir}>フォルダを含める
    {/if}
    <div style="margin-left: auto;">
      {app_message}
    </div>

  </div>

  {#if search_result.items.length !== 0}
    <ResultList bind:app_message result_items={search_result.items} />
  {/if}
</main>

<style>
  /* .row {
    display: flex;
    justify-content: space-between;
    font-size: medium;
    align-items: center;
  } */
  #infobar {
    color: rgb(255, 255, 255);
    padding: 0rem 0.5rem;
    background-color: rgb(14, 137, 178);
    border-radius: 0.2rem;
    margin: 0.5em 0.5em;
  }
  :focus {
    border-color: rgb(85, 85, 201);
  }
</style>
