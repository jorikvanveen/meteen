import HomeCard from "@/components/HomeCard";
import Overview from "@/components/Overview";
import { Button } from "@/components/ui/button";
import { useState } from "react";

export default function Home() {
  const [overviewExpanded, setOverviewExpanded] = useState(true);

  const overviewHiddenClasses = "opacity-0 translate-x-[-130%] grow-0 shrink"

  return (
    <div className="h-full flex flex-row gap-4 p-4 justify-center items-center">
      <HomeCard className="max-w-1/2 z-1 transition-all grow-1" title="Today">My epic tasks</HomeCard>
      <Button onClick={() => setOverviewExpanded(!overviewExpanded)}>{overviewExpanded ? "<" : ">"}</Button>
      <HomeCard className={`h-full transition-all ${overviewExpanded ? "grow-1" : overviewHiddenClasses}`} title="Overview"><Overview /></HomeCard>
    </div>
  )
}
