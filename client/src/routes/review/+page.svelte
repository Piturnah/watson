<script lang="ts">
  import TexBox from "$lib/TexBox.svelte";
  import Box from "$lib/Box.svelte";
  import { axios } from "$lib";
  import { get } from "svelte/store";
  import { selected_topic_ids } from "$lib/stores";
  import { onMount } from "svelte";

  let showSoln = false;

  let problem: { id: number; body: string; solnlink: string } = { id: -1, body: "", solnlink: "" };
  let soln = "";
  function getProblem() {
    axios
      .post("/problems/request", { topic_ids: get(selected_topic_ids) })
      .then(({ data }) => {
        showSoln = false;
        problem = data.problem;
        soln = data.solution;
        submitSoln = "";
      })
      .catch((e) => console.warn(e));
  }
  onMount(getProblem);

  let submitSoln = "";

  function next(successful: boolean) {
    axios
      .put("/problems/solve", { problem_id: problem.id, successful })
      .then(() => getProblem())
      .catch((err) => console.warn(err));
  }

  function handleSolnSubmit() {
    if (problem.id === -1) return;
    axios
      .post("/solutions", { problem_id: problem.id, body: submitSoln })
      .then(() => {
        soln = submitSoln;
        submitSoln = "";
      })
      .catch((err) => console.warn(err));
  }
</script>

<div class="grid grid-cols-1 p-12 w-[959px] m-auto gap-6">
  <Box>
    <TexBox content={problem.body} />
  </Box>
  {#if showSoln}
    <Box>
      {#if soln !== null}
        <TexBox content={soln} />
      {:else}
        <p>There's no solution :(</p>
        {#if problem.solnlink !== null && problem.solnlink !== ""}
          <p>
            The problem submitter indicated that you can find the solution here: <a
              href={problem.solnlink}>{problem.solnlink}</a
            >
          </p>
        {/if}
        <p>If you know the answer, you can submit the solution below:</p>
        <textarea
          bind:value={submitSoln}
          class="h-56 w-full p-1 font-mono bg-midnight text-white"
        />
        {#if submitSoln !== ""}
          <TexBox content={submitSoln} />
        {/if}
      {/if}
    </Box>
    <div class="flex flex-row-reverse w-full gap-4">
      {#if submitSoln === ""}
        <button on:click={() => next(true)} class="btn btn-green"> Correct </button>
        <button on:click={() => next(false)} class="btn btn-red"> Incorrect </button>
      {:else}
        <button on:click={handleSolnSubmit} class="btn btn-white"> Submit </button>
      {/if}
    </div>
  {:else}
    <div class="flex flex-row-reverse justify-between w-full">
      <button on:click={() => (showSoln = !showSoln)} class="btn btn-grey"> Show solution </button>
      <a href="/" class="btn btn-grey"> Back </a>
    </div>
  {/if}
</div>
