import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type Option = "LOCAL_SOURCE" | "LOCAL_SOURCE_BRIDGE" | "INSTALLED_BIN";
interface CommandOutput {
  isSuccess: boolean;
  information: string[];
}

function App() {
  const [selectedOption, setSelectedOption] = useState<Option>("LOCAL_SOURCE");
  const [commandOutput, setCommandOutput] = useState<CommandOutput | null>(
    null
  );

  const handleRadioChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSelectedOption(event.target.value as Option);
  };
  useEffect(() => {
    async function fetchData() {
      const result = await invoke<CommandOutput>("load_configuration");

      setCommandOutput(result);
    }

    fetchData();
    // const interval = setInterval(fetchData, 1000);
    //return () => clearInterval(interval);
  }, []);

  const runCommand = async (commandName: string) => {
    const result = await invoke<CommandOutput>(commandName);
    setCommandOutput(result);
  };

  const handleButtonClick = () => {
    console.log(`Selected Option: ${selectedOption}`);
    switch (selectedOption) {
      case "LOCAL_SOURCE":
        runCommand("run_with_local_source");
        break;
      case "LOCAL_SOURCE_BRIDGE":
        runCommand("run_with_local_source_bridge");
        break;
      case "INSTALLED_BIN":
        runCommand("run_from_installed");
        break;
    }
  };

  return (
    <main className="container">
      <h1>Choose an Option</h1>
      <div className="options">
        <div>
          <input
            type="radio"
            id="option1"
            name="options"
            value="LOCAL_SOURCE"
            checked={selectedOption === "LOCAL_SOURCE"}
            onChange={handleRadioChange}
          />
          <label htmlFor="option1">Run Zoom Client with local source</label>
        </div>
        <div>
          <input
            type="radio"
            id="option2"
            name="options"
            value="LOCAL_SOURCE_BRIDGE"
            checked={selectedOption === "LOCAL_SOURCE_BRIDGE"}
            onChange={handleRadioChange}
          />
          <label htmlFor="option2">
            Run Zoom Client with local source + Zoom Bridge
          </label>
        </div>
        <div>
          <input
            type="radio"
            id="option3"
            name="options"
            value="INSTALLED_BIN"
            checked={selectedOption === "INSTALLED_BIN"}
            onChange={handleRadioChange}
          />
          <label htmlFor="option3">Run Zoom Client with installed bin</label>
        </div>
      </div>
      <div
        className="information"
        style={{
          color: commandOutput?.isSuccess ? "green" : "red",
        }}
      >
        {commandOutput?.information.map((info, index) => (
          <p key={index}>{info}</p>
        ))}
      </div>
      <button onClick={handleButtonClick}>Restart Zoom Client</button>
    </main>
  );
}

export default App;
