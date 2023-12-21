<script lang="ts">
  import { displayTex } from "$lib";
  let problem =
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
</script>

<form class="mt-5">
  <div class="bg-gray-200 p-8 mx-10 rounded-md">
    <div class="flex justify-between gap-x-10">
      <div class="w-full">
        <label class="block">Problem statement</label>
        <textarea
          bind:value={problem}
          class="w-full border border-gray-300 rounded-sm text-sm h-32"
        />
        <p class="bg-white font-serif rounded-md p-3">
          {@html displayTex(problem)}
        </p>
      </div>

      <div class="w-full">
        <label class="block">Solution</label>
        <textarea bind:value={soln} class="w-full border border-gray-300 rounded-sm text-sm h-32" />
        <p class="bg-white font-serif rounded-md p-3">
          {@html displayTex(soln)}
        </p>
      </div>
    </div>

    <div class="flex justify-between">
      <div class="flex flex-col gap-3 mt-3">
        <div>
          <label>Source</label>
          <input bind:value={source} placeholder={"2MVA Lecture Notes"} class="p-1" />
        </div>

        <div>
          <label>Author</label>
          <input bind:value={author} placeholder={"Qianxi Wang"} class="p-1" />
        </div>

        <div>
          <label>Solution author</label>
          <input bind:value={author} placeholder={"Qianxi Wang"} class="p-1" />
        </div>
      </div>
      <button type="submit" class="bottom-0 bg-green-400 rounded-md text-white py-1 px-5">
        Submit
      </button>
    </div>
  </div>
</form>
