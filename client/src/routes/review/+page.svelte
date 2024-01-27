<script lang="ts">
  import TexBox from "$lib/TexBox.svelte";
  import Box from "$lib/Box.svelte";
  import { axios } from "$lib";
  import { get } from "svelte/store";
  import { selected_topic_ids } from "$lib/stores";
  import { onMount } from "svelte";
  import { PUBLIC_WATSON_MEDIA_BASE } from "$env/static/public";

  let showSoln = false;

  let problem: { id: number; body: string | null; solnlink: string; img_path: string | null } = {
    id: -1,
    body: "",
    solnlink: "",
  };
  let soln: string | null = null;
  let soln_img: string | null = null;

  let noProblem = false;
  function getProblem() {
    axios
      .post("/problems/request", { topic_ids: get(selected_topic_ids) })
      .then(({ data }) => {
        showSoln = false;
        if (data.problem === null) noProblem = true;
        else {
          problem = data.problem;
          soln = data.solution;
          soln_img = data.solution_img;
          submitSoln = "";
        }
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
      .post("/solutions", { problem_id: problem.id, body: submitSoln, img_path: solnImg })
      .then(() => {
        soln = submitSoln;
        submitSoln = "";
      })
      .catch((err) => console.warn(err));
  }

  let solnImg: string | null = null;
  const onloadSolution = (el) => {
    el.addEventListener("paste", (ev) => {
      const imgData = ev.clipboardData.files[0];
      if (imgData !== undefined && imgData.type.startsWith("image/")) {
        axios
          .post(
            "/upload",
            { file: imgData },
            { headers: { "content-type": "multipart/form-data" } },
          )
          .then((res) => (solnImg = res.data))
          .catch((e) => console.log(e));
      }
    });
  };
</script>

<div class="grid grid-cols-1 p-12 w-[959px] m-auto gap-6">
  {#if !noProblem}
    <Box>
      {#if problem.img_path !== null}
        <img class="rounded-md" src={`${PUBLIC_WATSON_MEDIA_BASE}/${problem.img_path}`} />
      {/if}
      {#if problem.body !== null}
        <TexBox content={`${problem.id}. ` + problem.body} />
      {/if}
    </Box>
    {#if showSoln}
      <Box>
        {#if soln_img !== null}
          <img class="rounded-md" src={`${PUBLIC_WATSON_MEDIA_BASE}/${problem.soln_img}`} />
        {/if}
        {#if soln !== null}
          <TexBox content={soln} />
        {:else}
          <p>There's no solution :(</p>
          {#if problem.solnlink !== null && problem.solnlink !== ""}
            <p>
              The problem submitter indicated that you can find the solution here: <a
                href={problem.solnlink}
                target="_blank">{problem.solnlink}</a
              >
            </p>
          {/if}
          <p>
            If you know the answer, you can submit the solution below:
            <span class="text-xs">(you may also paste an image)</span>
          </p>
          <textarea
            use:onloadSolution
            bind:value={submitSoln}
            class="h-56 w-full p-1 font-mono bg-midnight text-white"
          />
          {#if submitSoln !== ""}
            <TexBox content={submitSoln} />
          {/if}
          {#if solnImg !== null}
            <img class="rounded-md" src={`${PUBLIC_WATSON_MEDIA_BASE}/${solnImg}`} />
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
        <button on:click={() => (showSoln = !showSoln)} class="btn btn-grey">
          Show solution
        </button>
        <a href="/" class="btn btn-grey"> Back </a>
      </div>
    {/if}
  {:else}
    <Box>
      You have already completed all the problems in these topics in the last month! Try adding some
      more topics, or adding more problems to the database.
    </Box>
    <div class="flex flex-row-reverse justify-between w-full">
      <a href="/" class="btn btn-grey"> Back </a>
    </div>
  {/if}
</div>
