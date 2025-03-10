import React from "react";

interface ProcessCardProps {
  title: string;
  process: {
    id: string;
    name: string;
    running_time_formatted: string;
    memory_in_bytes: number;
  };
}

const ProcessCard: React.FC<ProcessCardProps> = ({ title, process }) => {
  return (
    <div className="process-card">
      <h3>{title} </h3>
      <p>
        {process.name} (ID: {process.id})
      </p>
      <p>Running Time: {process.running_time_formatted}</p>
      <p>Memory: {process.memory_in_bytes} bytes</p>
    </div>
  );
};

export default ProcessCard;
