import { Route, Routes } from 'react-router-dom'
import './App.css'
import WebSocketComponent from './WebSocketComponent'
import Home from './Home'

function App() {
  return (
    <Routes>
      <Route path="/" element={<Home />} />
      <Route path="/quotes" element={<WebSocketComponent />} />
    </Routes>
  )
}

export default App
