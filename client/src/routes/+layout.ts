import { redirect } from "@sveltejs/kit";
import { axios } from "$lib";

// Check if we're authenticated. If not, redirect to login page.
//
// TODO: Refresh token.
export function load({ route }: { route: { id: string } }) {
  let credential = localStorage.getItem("credential");
  if (credential === null && route.id !== "/login") {
    throw redirect(303, `/login?to=${route.id}`);
  } else {
    axios.defaults.headers["Authorization"] = `Bearer ${credential}`;
  }

  axios.post("/problems/request", { topic_ids: [1, 6, 7, 8] }).then(res => console.log(res)).catch((e) => console.warn(e));
}
