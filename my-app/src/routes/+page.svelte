<script lang="ts">
  import { afterUpdate } from "svelte";
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
  let showSoln = false;
</script>

<svelte:head></svelte:head>

<p>
  {@html displayTex(problem)}
</p>

<button on:click={() => (showSoln = !showSoln)}>{showSoln ? "Hide" : "Show"} solution</button>

{#if showSoln}
  <p>{@html displayTex(soln)}</p>
{/if}
