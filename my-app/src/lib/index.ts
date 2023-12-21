import { math, display } from "mathlifier";
import axiosLib from "axios";

export const axios = axiosLib.create({ baseURL: "http://localhost:3000/" });

enum TexString {
  Text,
  Inline,
  Display,
}

// "foo $$bar$$ $baz$" -> [[Text, "foo"], [Display, "bar"], [Inline, "baz"]]
function parseTex(input: string): [[TexString, string]] {
  let output: [[TexString, string]] = [];
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
