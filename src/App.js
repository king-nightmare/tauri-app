import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [response, setResponse] = useState("");

  const handleClick = async () => {
    try {
      const res = await invoke("handle_request");
      setResponse(res);
    } catch (err) {
      console.error("Error invoking handle_request:", err);
    }
  };

  return (
    <div>
      <button className="button" onClick={handleClick}>
        Send Request
      </button>
      <p className="response">{response}</p>
    </div>
  );
}

export default App;
