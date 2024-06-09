import React, {useEffect, useState} from 'react';
import {invoke} from '@tauri-apps/api/tauri';

function MainSensitivity() {
    const [cm360, setCm360] = useState(0);
    const [dpi, setDpi] = useState(0);


    useEffect(() => {
        // Fetch initial values from the backend
        invoke('get_initial_values').then((response) => {
            setCm360(response.cm360);
            setDpi(response.dpi);
        }).catch((error) => {
            console.error('Failed to fetch initial values:', error);
        });
    }, []);

    useEffect(() => {
        invoke('set_user_settings', {cm360: parseFloat(cm360), dpi: parseInt(dpi), normalFov: 0, zoomedFov: 0});
    }, [cm360, dpi]);

    return (
        <div>
            <h1>Main Sensitivity</h1>
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
                    onChange={(e) => setDpi(parseFloat(e.target.value))}
                />
            </div>
        </div>
    );
}

export default MainSensitivity;
