import React from "react";
import ReactDOM from "react-dom/client";
import { ArtifactWindow } from "./artifact/ArtifactWindow";
import "./styles/globals.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <ArtifactWindow />
  </React.StrictMode>,
);
