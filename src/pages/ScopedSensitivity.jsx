import React, { useEffect, useState, useRef } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import debounce from 'lodash.debounce';
import { Tooltip as ReactTooltip } from 'react-tooltip';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faQuestionCircle } from '@fortawesome/free-solid-svg-icons';

function ScopedSensitivity() {
    const [cm360, setCm360] = useState(0);
    const [dpi, setDpi] = useState(0);
    const [normalFov, setNormalFov] = useState(0);
    const [scopedFov, setScopedFov] = useState(0);

    const isInitialMount = useRef(true);

    // Fetch initial values when the component mounts
    useEffect(() => {
        const fetchInitialValues = async () => {
            try {
                const response = await invoke('get_initial_values');
                setCm360(response.cm360);
                setDpi(response.dpi);
                setNormalFov(response.normal_fov);
                setScopedFov(response.scoped_fov);
            } catch (error) {
                console.error('Failed to fetch initial values:', error);
            }
        };

        fetchInitialValues();
    }, []);

    // Debounced function to update user settings
    const updateSettings = debounce((cm360, dpi, normalFov, scopedFov) => {
        invoke('set_user_settings', {
            cm360: parseFloat(cm360),
            dpi: parseInt(dpi),
            normalFov: parseFloat(normalFov),
            scopedFov: parseFloat(scopedFov)
        }).catch((error) => {
            console.error('Failed to set user settings:', error);
        });
    }, 500); // Debounce by 500ms

    // Update backend when values change, but not on initial load
    useEffect(() => {
        if (isInitialMount.current) {
            isInitialMount.current = false;
        } else {
            updateSettings(cm360, dpi, normalFov, scopedFov);
        }
    }, [cm360, dpi, normalFov, scopedFov]);

    return (
        <div className="main-container">
            <div className="info-container">
                <FontAwesomeIcon icon={faQuestionCircle}
                    data-tooltip-id="info-tooltip"
                    data-tooltip-content="This page lets you match your scoped sensitivity to your hipfire sensitivity based on focal length scaling. Enter your cm/360 for hipfire, your DPI, your hipfire FOV, and your scoped FOV. Then press F1 to turn while scoping and adjust your scope sensitivity to turn exactly 360 degrees."
                    data-tooltip-place="left" className="info-icon"/>
                <ReactTooltip id="info-tooltip" className="tooltip-box" />
            </div>
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
                <label htmlFor="normalFov">Normal FOV:</label>
                <input
                    type="number"
                    id="normalFov"
                    name="normalFov"
                    value={normalFov}
                    onChange={(e) => setNormalFov(parseFloat(e.target.value))}
                />
            </div>
            <div className="input-group">
                <label htmlFor="scopedFov">Scoped FOV:</label>
                <input
                    type="number"
                    id="scopedFov"
                    name="scopedFov"
                    value={scopedFov}
                    onChange={(e) => setScopedFov(parseFloat(e.target.value))}
                />
            </div>
        </div>
    );
}

export default ScopedSensitivity;
