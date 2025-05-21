import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { BrowserRouter, Route, Routes } from "react-router"
import Home from "./routes/Home"
import CreateTask from "./routes/CreateTask.tsx"

function App() {
  const [count, setCount] = useState(0)

  return (
    <>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/create" element={<CreateTask />}/>
        </Routes>
      </BrowserRouter>
    </>
  )
}

export default App
