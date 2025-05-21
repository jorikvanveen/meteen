import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@radix-ui/react-label";
import { useState } from "react";

export default function CreateTask() {
  const [description, setDescription] = useState("");
  let epic = true;
  return (
    <>
      <Label className="text-8xl bg-red-700" htmlFor="description">
        Task description
      </Label>
      <Input
        id="description"
        value={description}
        onChange={(e) => setDescription(e.target.value)}
      />
      <Button onClick={() => console.log("hi")}>Hi</Button>
      <h1 className="text-"></h1>
    </>
  );
}
