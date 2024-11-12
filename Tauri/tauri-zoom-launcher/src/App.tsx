import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type Option = "LOCAL_ENV" | "LOCAL_ENV_BRIDGE" | "INSTALLED_BIN";

function App() {
  const [selectedOption, setSelectedOption] = useState<Option>("LOCAL_ENV");

  const handleRadioChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSelectedOption(event.target.value as Option);
  };

  const handleButtonClick = () => {
    console.log(`Selected Option: ${selectedOption}`);
    switch (selectedOption) {
      case "LOCAL_ENV":
        invoke("restartZoomClient", { option: "LOCAL_ENV" });
        break;
      case "LOCAL_ENV_BRIDGE":
        invoke("restartZoomClient", { option: "LOCAL_ENV_BRIDGE" });
        break;
      case "INSTALLED_BIN":
        invoke("restartZoomClient", { option: "INSTALLED_BIN" });
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
            value="LOCAL_ENV"
            checked={selectedOption === "LOCAL_ENV"}
            onChange={handleRadioChange}
          />
          <label htmlFor="option1">Run Zoom Client with local code</label>
        </div>
        <div>
          <input
            type="radio"
            id="option2"
            name="options"
            value="LOCAL_ENV_BRIDGE"
            checked={selectedOption === "LOCAL_ENV_BRIDGE"}
            onChange={handleRadioChange}
          />
          <label htmlFor="option2">
            Run Zoom Client with local code + Zoom Bridge
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
      <button onClick={handleButtonClick}>Restart Zoom Client</button>
    </main>
  );
}

export default App;
