import { redirect } from "@sveltejs/kit";
import { axios } from "$lib";

export const ssr = false;

// Check if we're authenticated. If not, redirect to login page.
//
// TODO: Refresh token.
export function load({ route }: { route: { id: string } }) {
  let session = localStorage.getItem("session");
  if (session === null && route.id !== "/login" && route.id !== "/register") {
    throw redirect(303, `/login?to=${route.id}`);
  } else {
    axios.defaults.headers["Authorization"] = session;
  }
}
