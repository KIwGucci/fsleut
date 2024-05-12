<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import FileDialog from "./lib/FileDialog.svelte";
  import Inputline from "./lib/Inputline.svelte";
  import ResultList from "./lib/ResultList.svelte";
  import ClearButton from "./lib/ClearButton.svelte";
  import Select from "./lib/Select.svelte";
  type SearchToken = {
    root_path: string;
    extention: string;
    search_word: string;
    deselection: string;
    is_dir: boolean;
  };

  type SearchResultStatus = {
    items: string[];
    message: string;
  };

  let selected_dir: string = "";
  let search_word: string = "";
  let file_extention: string = "";
  let is_dir: boolean = false;
  let is_file_open = false;

  let search_result: SearchResultStatus = { items: [], message: "" };
  let app_message = "";

  async function search_items() {
    let intoken: SearchToken = {
      root_path: selected_dir,
      extention: file_extention,
      search_word: search_word,
      deselection: "",
      is_dir: is_dir,
    };

    search_result = await invoke("search", { token: intoken });
  }

  function search_handler() {
    search_items()
      .then(() => {
        app_message = search_result.message;
      })
      .catch((err) => {
        app_message = err;
      });
  }
</script>

<main class="container">
  {#if selected_dir !== ""}
    <div class="row">
      <FileDialog bind:selected_dir />
      <form on:submit|preventDefault={search_handler}>
        <Inputline bind:invalue={search_word} inplace="Search word" />
      </form>
      <form on:submit|preventDefault={search_handler}>
        <Inputline bind:invalue={file_extention} inplace="Extention" />
      </form>
      <ClearButton
        bind:search_word
        bind:file_extention
        bind:items={search_result.items}
        bind:is_file_open
        bind:app_message
      />
      <Select bind:is_file_open />
    </div>
  {:else}
    <FileDialog bind:selected_dir />
  {/if}

  <div id="infobar">
    <div class="row">
      {#if selected_dir !== ""}
        <div style="margin-right: auto;">
          {"対象=>"}{selected_dir.slice(-45)}
        </div>
        <div style="margin-left: auto;margin-right:1em">
          {app_message}
        </div>
      {/if}
    </div>
  </div>
  {#if search_result.items.length !== 0}
    <ResultList
      bind:app_message
      result_items={search_result.items}
      {is_file_open}
    />
  {/if}
</main>

<style>
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
