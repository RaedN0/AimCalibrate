import { useState } from "react";
import { BrowserRouter as Router, Route, Routes, Link } from 'react-router-dom';
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import MainSensitivity from './pages/MainSensitivity';
import ScopedSensitivity from './pages/ScopedSensitivity';
import MessureFov from './pages/MessureFov';

function App() {
  return (
    <Router>
      <div className="container">
        <div className="sidebar">
          <h2>Navigation</h2>
          <ul>
            <li><Link to="/">Home</Link></li>
            <li><Link to="/scoped-sensitivity">Settings</Link></li>
            <li><Link to="/measure-fov">About</Link></li>
          </ul>
        </div>
        <div className="main-content">
          <Routes>
            <Route path="/" element={<MainSensitivity />} />
            <Route path="/scoped-sensitivity" element={<ScopedSensitivity />} />
            <Route path="/measure-fov" element={<MessureFov />} />
            </Routes>
        </div>
      </div>
    </Router>
  );
}

export default App;
