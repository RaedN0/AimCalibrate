import React from 'react';
import { BrowserRouter as Router, Route, Routes, NavLink } from 'react-router-dom';
import './App.css';
import MainSensitivity from './pages/MainSensitivity';
import ScopedSensitivity from './pages/ScopedSensitivity';
import MeasureFov from './pages/MeasureFov';

function App() {
  const setPage = (page) => {
    invoke('set_current_page', { page }).catch((err) => console.error(err));
  };

  return (
    <Router>
      <div className="app-container">
        <div className="sidebar">
          <div className="sidebar-header">
            <h2>AimCalibrate</h2>
          </div>
          <ul className="sidebar-menu">
            <li><NavLink exact to="/" activeClassName="active" onClick={() => setPage("main_sensitivity")}>Main Sensitivity</NavLink></li>
            <li><NavLink to="/scoped-sensitivity" activeClassName="active" onClick={() => setPage("scoped_sensitivity")}>Scoped Sensitivity</NavLink></li>
            <li><NavLink to="/measure-fov" activeClassName="active" onClick={() => setPage("measure_fov")}>Measure FOV</NavLink></li>
          </ul>
        </div>
        <div className="main-content">
          <div className="content">
            <Routes>
              <Route path="/" element={<MainSensitivity />} />
              <Route path="/scoped-sensitivity" element={<ScopedSensitivity />} />
              <Route path="/measure-fov" element={<MeasureFov />} />
            </Routes>
          </div>
        </div>
      </div>
    </Router>
  );
}

export default App;
