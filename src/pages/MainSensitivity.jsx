import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

function MainSensitivity() {
    const [cm360, setCm360] = useState(0);
    const [dpi, setDpi] = useState(0);

    const handleSetValues = () => {
        console.log(`Set values - cm/360: ${cm360}, DPI: ${dpi}`);
        invoke('set_mouse_parameters', { cm360, dpi });
    };

    return (
        <div>
            <h1>GameSensSetup</h1>
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
            <button onClick={handleSetValues}>Set Values</button>
        </div>
    );
}

export default MainSensitivity;
