import React from "react";
import { useEffect } from "react";
import ProcessCard from "./ProcessCard";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface ProcessInfo {
  id: string;
  name: string;
  running_time: string;
  memory: number;
}

const App: React.FC = () => {
  const [processes, setProcesses] = React.useState<ProcessInfo[]>([]);
  const [maxMemoryProcess, setMaxMemoryProcess] =
    React.useState<ProcessInfo | null>(null);

  const [maxRunningProcess, setMaxRunningProcess] =
    React.useState<ProcessInfo | null>(null);

  useEffect(() => {
    async function fetchData() {
      const processList = await invoke<ProcessInfo[]>("list_processes");
      const maxMemory = await invoke<ProcessInfo>("max_memory");
      const maxRunning = await invoke<ProcessInfo>("max_running_process");

      setProcesses(processList);
      setMaxMemoryProcess(maxMemory);
      setMaxRunningProcess(maxRunning);
    }

    fetchData();
    const interval = setInterval(fetchData, 1000);
    return () => clearInterval(interval);
  }, []);

  return (
    <main className="container">
      {maxMemoryProcess && (
        <ProcessCard title="Max Memory Process" process={maxMemoryProcess} />
      )}
      {maxRunningProcess && (
        <ProcessCard title="Max Running Process" process={maxRunningProcess} />
      )}
      <div className="process-list">
        {processes.map((process) => {
          return (
            <div key={process.id} className="process-item">
              <span>
                {process.name} (ID: {process.id})
              </span>
              <span>Running Time: {process.running_time}</span>
              <span>Memory: {process.memory} bytes</span>
            </div>
          );
        })}
      </div>
    </main>
  );
};

export default App;
