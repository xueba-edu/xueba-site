import { createEffect, createSignal } from "solid-js";
import Navbar from "../Navbar";
import { fetchUserInfo } from "../ServerFunctions";

function HomePage() {
  const [user, setUser] = createSignal("");

  // const [count, setCount] = createSignal(0);
  // const [ready, setReady] = createSignal(false);

  createEffect(async () => {
    if (user() === "") {
      const userInfo = await fetchUserInfo(
        "e7e8ed68-67f4-4dcc-ba74-cade10e73b9d",
      );
      setUser((_) => {
        return JSON.stringify(userInfo);
      });
    }
  });

  return (
    <div>
      <Navbar />
      <h1>wecuome</h1>
      <p>{user()}</p>
    </div>
  );
}

export default HomePage;
