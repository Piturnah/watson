<script lang="ts">
  import { axios, displayTex } from "$lib";
  let body =
    "Find and classify the stationary points of the following function: $$f(x, y) = 2x^3 -6xy + 3y^2$$";
  let soln = `Find the stationary points
  $$
  \\begin{aligned}
  f_x = 6x^2 - 6y&\\therefore f_x = 0 \\implies& x^2 = y \\\\
  f_y = -6x + 6y&\\therefore f_y = 0 \\implies& x = y
  \\end{aligned}
  $$
  Hence there are two stationary points, at $(0, 0)$ and $(1, 1)$. The Hessian matrix is given by:
  $$
  H = \\begin{pmatrix}
  f_{xx} & f_{xy} \\\\ f_{yx} & f_{yy}
  \\end{pmatrix}
  = \\begin{pmatrix}
  12x & -6 \\\\ -6 & 6
  \\end{pmatrix}. \\\\
  $$
  At $(0, 0)$,
  $$
  \\text{det}(H) = \\begin{vmatrix}
  0 & -6 \\\\ -6 & 6
  \\end{vmatrix}
  = -36 < 0,
  $$
  so $(0, 0)$ is a saddle point. At $(1, 1)$,
  $$
  \\text{det}(H) = \\begin{vmatrix}
  12 & -6 \\\\ -6 & 6
  \\end{vmatrix}
  = 36 > 0.
  $$
  Further, det$(H_1) = 12 > 0$, so $(1,1)$ is a local minimum.`;
  let source = "";
  let author = "";

  let selectedModule = "";
  let selectedTopic = "";
  let solnLink = "";

  let modules = [];
  let topics = [];
</script>

<form class="flex flex-col items-center p-12">
  <div class="grid grid-cols-2 w-full gap-8">
    <div class="flex flex-col p-5 gap-4 border border-gray-400">
      <select name="module" bind:value={selectedModule} class="border p-1">
        <option value="" disabled selected>Module</option>
        <option value="new">New module</option>
      </select>
      {#if selectedModule === "new"}
        <input placeholder="Module name" class="border p-1 border-black" />
      {/if}
      <select name="topic" bind:value={selectedTopic} class="border p-1">
        <option value="" disabled selected>Topic</option>
        <option value="new">New topic</option>
      </select>
      {#if selectedTopic === "new"}
        <input placeholder="Topic name" class="border p-1 border-black" />
      {/if}
      <input
        bind:value={author}
        placeholder="Problem author"
        class="border border-black p-1 w-full"
      />
      <input
        bind:value={source}
        placeholder="Problem source (e.g. 2MVA Lecture Notes)"
        class="border border-black p-1 w-full"
      />
      <div>
        <label class="block">Problem statement</label>
        <textarea bind:value={body} class="border border-black h-36 w-full p-1 font-mono" />
      </div>
      <div>
        <label class="block">Solution</label>
        <textarea bind:value={soln} class="border border-black h-56 w-full p-1 font-mono" />
      </div>
      {#if soln === ""}
        <div>
          <label class="block">Where can the solution be found? (e.g. URL or textbook)</label>
          <input bind:value={solnLink} class="border border-black p-1 w-full" />
        </div>
      {/if}
    </div>
    <div class="bg-gray-400 p-6 border border-gray-500">
      <p class="bg-white font-serif rounded-md p-3">
        {@html displayTex(body)}
      </p>
      {#if soln !== ""}
        <p class="bg-white font-serif rounded-md p-3 mt-6">
          {@html displayTex(soln)}
        </p>
      {/if}
    </div>
  </div>
  <button class="mt-5 bg-green-400 py-2 px-8 rounded-sm">Submit</button>
</form>
