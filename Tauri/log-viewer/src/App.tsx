import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [logs, setLogs] = useState<string>("");
  const [searchResults, setSearchResults] = useState<string[]>([]);
  const [searchTerm, setSearchTerm] = useState<string>("");

  // Function to fetch log content
  const fetchLogs = async () => {
    try {
      const content = await invoke<string>("read_logs");
      setLogs(content);
    } catch (error) {
      console.error("Failed to read logs:", error);
    }
  };
  // Function to search logs
  const searchLogs = async () => {
    try {
      if (!searchTerm) return;
      const results = await invoke<string[]>("search_logs", {
        query: searchTerm,
      });
      setSearchResults(results);
    } catch (error) {
      console.error("Failed to search logs:", error);
    }
  };

  useEffect(() => {
    fetchLogs();
  }, []);
  return (
    <div className="p-4 max-w-2xl mx-auto">
      <h1 className="text-xl font-bold">Log Viewer</h1>

      <button
        onClick={fetchLogs}
        className="bg-blue-500 text-white px-4 py-2 rounded my-2"
      >
        Refresh Logs
      </button>

      <input
        type="text"
        value={searchTerm}
        onChange={(e) => setSearchTerm(e.target.value)}
        placeholder="Search logs..."
        className="border p-2 w-full my-2"
      />

      <button
        onClick={searchLogs}
        className="bg-green-500 text-white px-4 py-2 rounded"
      >
        Search
      </button>

      <h2 className="text-lg font-semibold mt-4">Log Content</h2>
      <pre className="bg-gray-900 text-white p-4 mt-2 rounded h-64 overflow-auto">
        {logs || "No logs available."}
      </pre>

      {searchResults.length > 0 && (
        <>
          <h2 className="text-lg font-semibold mt-4">Search Results</h2>
          <pre className="bg-gray-800 text-yellow-300 p-4 mt-2 rounded h-40 overflow-auto">
            {searchResults.join("\n") || "No matches found."}
          </pre>
        </>
      )}
    </div>
  );
}

export default App;
