import { math, display } from "mathlifier";
import axiosLib from "axios";

export const axios = axiosLib.create({ baseURL: "http://localhost:3000/" });

enum TexString {
  Text,
  Inline,
  Display,
}

// "foo $$bar$$ $baz$" -> [[Text, "foo"], [Display, "bar"], [Inline, "baz"]]
function parseTex(input: string): [TexString, string][] {
  let output: [TexString, string][] = [];
  let tokenStartIdx = 0;
  let charIdx = 0;
  let ty = TexString.Text;
  while (charIdx < input.length) {
    while (charIdx <= input.length && input[charIdx++] !== "$");
    switch (ty) {
      case TexString.Text:
        output.push([ty, input.slice(tokenStartIdx, charIdx - 1)]);
        if (input[charIdx] === "$") {
          ty = TexString.Display;
          charIdx += 1;
        } else {
          ty = TexString.Inline;
        }
        break;
      case TexString.Inline:
        output.push([ty, input.slice(tokenStartIdx, charIdx - 1)]);
        ty = TexString.Text;
        break;
      case TexString.Display:
        output.push([ty, input.slice(tokenStartIdx, charIdx - 1)]);
        if (input[charIdx++] !== "$") {
          // TODO: error
        }
        ty = TexString.Text;
        break;
    }
    tokenStartIdx = charIdx;
  }
  return output;
}

export function displayTex(src: string): string {
  return parseTex(src)
    .map(([ty, tex]) =>
      ty === TexString.Text ? tex : ty === TexString.Inline ? math(tex) : display(tex),
    )
    .join("");
}

export let exampleProblem = {
  id: -1,
  body: "Find and classify the stationary points of the following function: $$f(x, y) = 2x^3 -6xy + 3y^2$$",
  soln: `Find the stationary points
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
  = 12 \\cdot 6 - 6\\cdot6 > 0.
  $$
  Further, det$(H_1) = 12 > 0$, so $(1,1)$ is a local minimum.`,
};
