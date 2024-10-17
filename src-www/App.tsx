import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

async function greet(name: string): Promise<string> {
  return await invoke("greet", { name: name });
}

export default function App() {
  const [message, setMessage] = useState<string>("");

  greet("world").then(setMessage);

  return <h1>{message}</h1>;
}
