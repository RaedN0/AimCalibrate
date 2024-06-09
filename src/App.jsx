import React from 'react';
import { BrowserRouter as Router, Route, Routes, NavLink } from 'react-router-dom';
import './App.css';
import MainSensitivity from './pages/MainSensitivity';
import ScopedSensitivity from './pages/ScopedSensitivity';
import MessureFov from './pages/MessureFov';

function App() {

  const setPage = (page) => {
    invoke('set_current_page', { page }).catch((err) => console.error(err));
  };

  return (
    <Router>
      <div className="container">
        <div className="sidebar">
          <h2>AimCalibrate</h2>
          <ul>
            <li><NavLink exact to="/" activeClassName="active" onClick={() => setPage("main_sensitivity")}>Main Sensitivity</NavLink></li>
            <li><NavLink to="/scoped-sensitivity" activeClassName="active" onClick={() => setPage("scoped_sensitivity")}>Scoped Sensitivity</NavLink></li>
            <li><NavLink to="/measure-fov" activeClassName="active" onClick={() => setPage("messure_fov")}>Measure FOV</NavLink></li>
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
