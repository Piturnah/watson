<script lang="ts">
  import { onMount } from "svelte";
  import { axios, exampleProblem } from "$lib";
  import TexBox from "$lib/TexBox.svelte";
  import Box from "$lib/Box.svelte";
  import { PUBLIC_WATSON_MEDIA_BASE } from "$env/static/public";
  let { body, soln } = exampleProblem;
  let source = "";
  let author = "";

  let solnLink = "";

  let modules = [{ id: -1, title: "New module" }];
  let topics: { id: number; module_id: number; title: string }[] = [];
  let selectedModule = -2;
  let selectedTopic = -2;
  let newModule = "";
  let newTopic = "";

  let errors: string[] = [];
  let waiting = false;

  onMount(() => {
    axios
      .get("/modules")
      .then(({ data }) => {
        modules = [{ id: -1, title: "New module" }, ...data.modules];
        topics = data.topics;
      })
      .catch((e) => console.warn(e));
  });

  function handleFormSubmit() {
    if (waiting) return;

    errors = [];
    if (selectedModule < -1) errors = ["Please select a module", ...errors];
    if (selectedTopic < -1) errors = ["Please select a topic", ...errors];
    if (problemImg === null && body.replaceAll(/\s+/g, "") === "")
      errors = ["Please enter a problem", ...errors];
    if (errors.length > 0) return;

    waiting = true;
    axios
      .post("/problems/create", {
        source,
        author,
        body,
        soln: soln.replaceAll(/\s+/g, "") === "" ? null : soln,
        soln_img: solnImg,
        solnlink: solnLink,
        img_path: problemImg,
        module: selectedModule === -1 ? newModule : selectedModule,
        topic: selectedTopic === -1 ? newTopic : selectedTopic,
      })
      .then(() => {
        body = "";
        soln = "";
        problemImg = null;
        solnImg = null;
        if (selectedModule === -1) selectedModule = -2;
        if (selectedTopic === -1) selectedTopic = -2;
        waiting = false;
      })
      .catch((error) => {
        console.warn(error);
        waiting = false;
      });
  }

  // Srcs for images relative to the media base URL.
  let problemImg: string | null = null;
  let solnImg: string | null = null;

  type ImgSrc = "problem" | "solution";

  const uploadClipboardImg = (imgData, src: ImgSrc) => {
    if (imgData !== undefined && imgData.type.startsWith("image/")) {
      axios
        .post("/upload", { file: imgData }, { headers: { "content-type": "multipart/form-data" } })
        .then((res) => {
          if (src === "problem") {
            problemImg = res.data;
          } else {
            solnImg = res.data;
          }
        })
        .catch((e) => console.log(e));
    }
  };

  const onloadProblem = (el) => {
    el.addEventListener("paste", (ev) => {
      uploadClipboardImg(ev.clipboardData.files[0], "problem");
    });
  };

  const onloadSolution = (el) => {
    el.addEventListener("paste", (ev) => {
      uploadClipboardImg(ev.clipboardData.files[0], "solution");
    });
  };
</script>

<form class="flex flex-col items-center p-12">
  <div class="grid grid-cols-2 w-full gap-8">
    <div class="flex flex-col p-5 gap-4 border border-dust bg-coal">
      <select name="module" bind:value={selectedModule} class="p-2 bg-midnight text-white">
        <option value={-2} disabled selected>Module</option>
        {#each modules as { id, title } (id)}
          <option value={id}>{title}</option>
        {/each}
      </select>
      {#if selectedModule === -1}
        <input
          bind:value={newModule}
          placeholder="Module name"
          class="p-2 bg-midnight text-white"
        />
      {/if}
      <select name="topic" bind:value={selectedTopic} class="p-2 bg-midnight text-white">
        <option value={-2} disabled selected>Topic</option>
        <option value={-1}>New topic</option>
        {#each topics.filter((topic) => topic.module_id === selectedModule) as { id, title } (id)}
          <option value={id}>{title}</option>
        {/each}
      </select>
      {#if selectedTopic === -1}
        <input bind:value={newTopic} placeholder="Topic name" class="p-2 bg-midnight text-white" />
      {/if}
      <input
        bind:value={author}
        placeholder="Problem author"
        class="border-black p-2 w-full bg-midnight text-white"
      />
      <input
        bind:value={source}
        placeholder="Problem source (e.g. 2MVA Lecture Notes)"
        class="p-2 w-full bg-midnight text-white"
      />
      <div>
        <label class="block text-white">
          Problem statement
          <span class="text-xs">(you may also paste an image)</span>
        </label>
        <textarea
          use:onloadProblem
          bind:value={body}
          class="h-36 w-full p-1 font-mono bg-midnight text-white"
        />
      </div>
      <div>
        <label class="block text-white">
          Solution
          <span class="text-xs">(you may also paste an image)</span>
        </label>
        <textarea
          use:onloadSolution
          bind:value={soln}
          class="h-56 w-full p-1 font-mono bg-midnight text-white"
        />
      </div>
      {#if soln === "" && solnImg === null}
        <div>
          <label class="block text-white"
            >Where can the solution be found? (e.g. URL or textbook)</label
          >
          <input bind:value={solnLink} class="p-1 w-full bg-midnight text-white" />
        </div>
      {/if}
      <div>
        {#each errors as error (error)}
          <div class="block text-red-700">{error}</div>
        {/each}
      </div>
    </div>
    <Box>
      {#if problemImg !== null}
        <img class="rounded-md" src={`${PUBLIC_WATSON_MEDIA_BASE}/${problemImg}`} />
      {/if}
      {#if body !== ""}
        <TexBox content={body} />
      {/if}
      {#if solnImg !== null}
        <img class="rounded-md" src={`${PUBLIC_WATSON_MEDIA_BASE}/${solnImg}`} />
      {/if}
      {#if soln !== ""}
        <TexBox content={soln} />
      {/if}
    </Box>
  </div>
  <div class="flex mt-5 gap-5 w-full">
    <button
      on:click={handleFormSubmit}
      class={`btn ${waiting ? "bg-gray-400 cursor-wait" : "btn-green"}`}
    >
      Submit
    </button>
    <a href="/" class="btn btn-grey"> Back </a>
  </div>
</form>
