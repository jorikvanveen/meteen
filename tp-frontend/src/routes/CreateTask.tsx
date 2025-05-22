import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@radix-ui/react-label";
import { useState, type FormEvent } from "react";

export default function CreateTask() {
  const [description, setDescription] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [submittedTasks, setSubmittedTasks] = useState<string[]>([]);

  async function onSubmit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault()
    setSubmitting(true)
    const response = await fetch("http://localhost:3000/task", {
      method: "post",
      body: JSON.stringify({
        description
      }),
      headers: {
        "Content-Type": "application/json"
      }
    })

    if (response.ok) {
      setSubmitting(false)
      submittedTasks.push(description)
      setSubmittedTasks(submittedTasks)
      setDescription("")
    }
  }

  return (
    <div className="flex flex-col gap-4">
      <Label className="text-4xl text-left" htmlFor="description">
        New task
      </Label>
      <form className="flex flex-row gap-2" onSubmit={onSubmit}>
        <Input
          id="description"
          value={description}
          placeholder="Description"
          onChange={(e) => setDescription(e.target.value)}
        />
        <Button type="submit" className={"flex-1/6"} variant={submitting ? "ghost" : "default"}>Send to inbox</Button>
      </form>
      {submittedTasks.map(task => (
        <div className="flex flex-col"><span>{task}</span></div> 
      ))}
    </div>
  );
}
