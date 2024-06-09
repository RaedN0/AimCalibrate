import React, {useEffect, useState} from 'react';
import {invoke} from '@tauri-apps/api/tauri';

function ScopedSensitivity() {

    const [cm360, setCm360] = useState(0);
    const [dpi, setDpi] = useState(0);
    const [normalFov, setNormalFov] = useState(0);
    const [scopedFov, setScopedFov] = useState(0);

    useEffect(() => {
        invoke('set_user_settings', {cm360: parseFloat(cm360), dpi: parseInt(dpi), normalFov: parseFloat(normalFov), zoomedFov: parseFloat(scopedFov)});
    }, [cm360, dpi, normalFov, scopedFov]);

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
                    onChange={(e) => setDpi(parseFloat(e.target.value))}
                />
            </div>
            <div className="input-group">
                <label htmlFor="normalFov">Normal FOV:</label>
                <input
                    type="number"
                    id="normalfov"
                    name="normalFov"
                    value={normalFov}
                    onChange={(e) => setNormalFov(parseFloat(e.target.value))}
                />
            </div>
            <div className="input-group">
                <label htmlFor="scopedFov">Scoped FOV:</label>
                <input
                    type="number"
                    id="scopedfov"
                    name="scopedFov"
                    value={scopedFov}
                    onChange={(e) => setScopedFov(parseFloat(e.target.value))}
                />
            </div>
        </div>
    );
}

export default ScopedSensitivity;
