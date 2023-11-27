import { A } from "@solidjs/router";

function Navbar() {
  function toggleTheme() {
    const root = document.getElementById("root");
    if (!root) {
      return;
    }
    if (root.className === "light") {
      root.className = "dark";
      window.localStorage.setItem("theme", "dark");
    } else {
      root.className = "light";
      window.localStorage.setItem("theme", "light");
    }
  }

  return (
    <nav>
      <A href="/" activeClass="underlined" end>
        Home
      </A>
      <A href="/login" activeClass="underlined" end>
        Login
      </A>
      <button onclick={toggleTheme}>Theme</button>
    </nav>
  );
}

export default Navbar;
