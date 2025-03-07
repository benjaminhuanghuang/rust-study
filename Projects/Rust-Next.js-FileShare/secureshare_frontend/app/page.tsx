"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Loader2 } from "lucide-react";

export default function Home() {
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const handleClick = () => {
    setIsLoading(true);
    // Simulate a network request
    setTimeout(() => {
      setIsLoading(false);
    }, 2000);
  };
  return (
    <div className="flex justify-center items-center min-h-screen">
      <Button disabled={isLoading} onClick={handleClick}>
        {isLoading && <Loader2 />}
        Click Me
      </Button>
    </div>
  );
}
