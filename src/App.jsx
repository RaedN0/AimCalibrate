import React, {useCallback, useEffect, useState} from 'react';
import {BrowserRouter as Router, NavLink, Route, Routes} from 'react-router-dom';
import {invoke} from '@tauri-apps/api/tauri';
import debounce from 'lodash/debounce';
import './App.css';
import MainSensitivity from './pages/MainSensitivity';
import ScopedSensitivity from './pages/ScopedSensitivity';
import MeasureFov from './pages/MeasureFov';
import logo from '/acLogo.png'; // Adjust the path based on your project structure

function App() {
    const [sliderValue, setSliderValue] = useState(1);

    useEffect(() => {
        debouncedUpdateSettings(sliderValue)
    }, [sliderValue]);

    const handleSliderChange = (event) => {
        setSliderValue(event.target.value);
    };

    const debouncedUpdateSettings = useCallback(
        debounce((sliderValue) => {
            console.log(sliderValue);
            invoke('set_app_settings', {
                turnSpeed: parseFloat(sliderValue)
            }).catch((error) => {
                console.error('Failed to set user settings:', error);
            });
        }, 500), // Debounce delay of 500ms
        []
    );

    const setPage = (page) => {
        invoke('set_current_page', {page}).catch((err) => console.error(err));
    };

    return (
        <Router>
            <div className="app-container">
                <div className="sidebar">
                    <div className="sidebar-header">
                        <img src={logo} alt="AimCalibrate" className="logo"/>
                    </div>
                    <ul className="sidebar-menu">
                        <li>
                            <NavLink exact to="/" onClick={() => setPage("main_sensitivity")}>Main Sensitivity</NavLink>
                        </li>
                        <li>
                            <NavLink to="/scoped-sensitivity" onClick={() => setPage("scoped_sensitivity")}>Scoped
                                Sensitivity</NavLink>
                        </li>
                        <li>
                            <NavLink to="/measure-fov" onClick={() => setPage("measure_fov")}>Measure FOV</NavLink>
                        </li>
                    </ul>
                    <div className="sidebar-slider">
                        <label htmlFor="sensitivity-slider">Turn speed: {sliderValue}</label>
                        <input
                            type="range"
                            id="sensitivity-slider"
                            min="0.1"
                            max="2"
                            step="0.1"
                            value={sliderValue}
                            onChange={handleSliderChange}
                        />
                    </div>
                </div>
                <div className="main-content">
                    <Routes>
                        <Route path="/" element={<MainSensitivity/>}/>
                        <Route path="/scoped-sensitivity" element={<ScopedSensitivity/>}/>
                        <Route path="/measure-fov" element={<MeasureFov/>}/>
                    </Routes>
                </div>
            </div>
        </Router>
    );
}

export default App;
