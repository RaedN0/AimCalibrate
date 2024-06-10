import React, { useEffect, useState, useRef } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import debounce from 'lodash.debounce';
function MeasureFov() {
    const [cm360, setCm360] = useState(0);
    const [dpi, setDpi] = useState(0);
    const [gameSens, setGameSens] = useState(0);
    const [fov16, setFov16] = useState(0);
    const [fov43, setFov43] = useState(0);
    const [fov11, setFov11] = useState(0);

    const isInitialMount = useRef(true);

    // Fetch initial values when the component mounts
    useEffect(() => {
        const fetchInitialValues = async () => {
            try {
                const response = await invoke('get_initial_values');
                setCm360(response.cm360);
                setDpi(response.dpi);
                setGameSens(response.game_sens);
                setFov16(response.game_fov)

                //TODO: Convert fovs
            } catch (error) {
                console.error('Failed to fetch initial values:', error);
            }
        };

        fetchInitialValues();
    }, []);

    // Debounced function to update user settings
    const updateSettings = debounce((cm360, dpi, gameSens, gameFov) => {
        invoke('set_user_settings', {
            cm360: parseFloat(cm360),
            dpi: parseInt(dpi),
            gameSens: parseFloat(gameSens),
            gameFov: parseFloat(gameFov)
        }).catch((error) => {
            console.error('Failed to set user settings:', error);
        });
    }, 500); // Debounce by 500ms

    // Update backend when values change, but not on initial load
    useEffect(() => {
        if (isInitialMount.current) {
            isInitialMount.current = false;
        } else {
            updateSettings(cm360, dpi, gameSens, fov16);
        }
    }, [cm360, dpi, gameSens, fov16]);

    return (
        <div>
            <h1>Scoped Sensitivity</h1>
            <div className="input-group">
                <label htmlFor="cm360">cm/360:</label>
                <input
                    type="number"
                    id="cm360"
                    name="cm360"
                    value={cm360}
                    onChange={(e) => setCm360(parseFloat(e.target.value))}
                />
            </div>
            <div className="input-group">
                <label htmlFor="dpi">DPI:</label>
                <input
                    type="number"
                    id="dpi"
                    name="dpi"
                    value={dpi}
                    onChange={(e) => setDpi(parseInt(e.target.value))}
                />
            </div>
            <div className="input-group">
                <label htmlFor="normalFov">Game Sens:</label>
                <input
                    type="number"
                    id="gamesens"
                    name="gameSens"
                    value={gameSens}
                    onChange={(e) => setGameSens(parseFloat(e.target.value))}
                />
            </div>
            <div className="input-group">
                <label htmlFor="scopedFov">16:9:</label>
                <input
                    type="number"
                    id="fov16"
                    name="fov16"
                    value={fov16}
                    onChange={(e) => setFov16(parseFloat(e.target.value))}
                />
            </div>
            <div className="input-group">
                <label htmlFor="scopedFov">4:3:</label>
                <input
                    type="number"
                    id="fov43"
                    name="fov43"
                    value={fov43}
                    onChange={(e) => setFov43(parseFloat(e.target.value))}
                />
            </div>
            <div className="input-group">
                <label htmlFor="scopedFov">1:1:</label>
                <input
                    type="number"
                    id="fov11"
                    name="fov11"
                    value={fov11}
                    onChange={(e) => setFov12(parseFloat(e.target.value))}
                />
            </div>
        </div>
    );
}

export default MeasureFov;
