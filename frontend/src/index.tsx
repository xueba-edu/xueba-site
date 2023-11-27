/* @refresh replace */
import { render } from "solid-js/web";

import "./index.scss";
import { Route, Router, Routes } from "@solidjs/router";
import LoginPage from "./pages/LoginPage";
import HomePage from "./pages/HomePage";

const root = document.getElementById("root");

if (window.localStorage.getItem("theme") === "light") {
  root!.className = "light";
} else {
  root!.className = "dark";
}

render(
  () => (
    <Router>
      <Routes>
        <Route path="/" component={HomePage} />
        <Route path="/login" component={LoginPage} />
      </Routes>
    </Router>
  ),
  root!,
);
