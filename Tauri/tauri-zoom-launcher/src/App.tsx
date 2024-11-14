import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface CommandOutput {
  is_success: boolean;
  information: string[];
}
interface Option {
  label: string;
  command: string;
  isUIOption?: boolean;
}
const commands: Option[] = [
  {
    label: "Load Configuration",
    command: "load_configuration",
    isUIOption: false,
  },
  {
    label: "Run Zoom Client with local source",
    command: "run_with_local_source",
    isUIOption: true,
  },
  {
    label: "Run Zoom Client with local source and Zoom Bridge",
    command: "run_with_local_source_bridge",
    isUIOption: true,
  },
  {
    label: "Run Zoom Client with installed bin",
    command: "run_from_installed",
    isUIOption: true,
  },
  {
    label: "Close Zoom Client",
    command: "close_zoom_client",
    isUIOption: true,
  },
  {
    label: "Read user preferences",
    command: "read_user_preferences",
    isUIOption: true,
  },
];

function App() {
  const [selectedCommand, setSelectedCommand] = useState<string>(
    "run_with_local_source"
  );
  const [commandOutput, setCommandOutput] = useState<CommandOutput | null>(
    null
  );

  const handleRadioChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    console.log(event.target.value);
    setSelectedCommand(event.target.value);
    setCommandOutput(null);
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
    if (selectedCommand) {
      console.log("Run command", selectedCommand);
      runCommand(selectedCommand);
    }
  };

  return (
    <main className="container">
      <h1>Choose an Option</h1>
      {/* commands */}
      <div className="options">
        {commands
          .filter((command) => command.isUIOption)
          .map((command, index) => {
            return (
              <div key={index}>
                <input
                  type="radio"
                  id={command.command}
                  name={command.command}
                  value={command.command}
                  checked={selectedCommand === command.command}
                  onChange={handleRadioChange}
                />
                <label htmlFor={command.command}>{command.label}</label>
              </div>
            );
          })}
      </div>
      {/* information and commands output */}
      <div
        className="information"
        style={{
          color: commandOutput?.is_success ? "green" : "red",
        }}
      >
        {commandOutput?.information.map((info, index) => (
          <p key={index}>{info}</p>
        ))}
      </div>
      <button onClick={handleButtonClick}>Run Command</button>
    </main>
  );
}

export default App;
