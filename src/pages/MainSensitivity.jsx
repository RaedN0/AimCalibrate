import React, { useEffect, useState, useRef, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import debounce from 'lodash/debounce';
import { Tooltip as ReactTooltip } from 'react-tooltip';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faQuestionCircle } from '@fortawesome/free-solid-svg-icons';

function MainSensitivity() {
    const [cm360, setCm360] = useState(0);
    const [dpi, setDpi] = useState(0);

    const isInitialMount = useRef(true);

    // Fetch initial values when the component mounts
    useEffect(() => {
        const fetchInitialValues = async () => {
            try {
                const response = await invoke('get_initial_values');
                setCm360(response.cm360);
                setDpi(response.dpi);
            } catch (error) {
                console.error('Failed to fetch initial values:', error);
            }
        };

        fetchInitialValues();
    }, []);

    // Debounced function to update user settings
    const debouncedUpdateSettings = useCallback(
        debounce((cm360, dpi) => {
            invoke('set_user_settings', {
                cm360: parseFloat(cm360),
                dpi: parseInt(dpi)
            }).catch((error) => {
                console.error('Failed to set user settings:', error);
            });
        }, 500), // Debounce delay of 500ms
        []
    );

    // Update backend when values change, but not on initial load
    useEffect(() => {
        if (isInitialMount.current) {
            isInitialMount.current = false;
        } else {
            debouncedUpdateSettings(cm360, dpi);
        }
    }, [cm360, dpi, debouncedUpdateSettings]);

    return (
        <div className="main-container">
            <ReactTooltip id="info-tooltip" className="tooltip-box" />
            <div className="info-container">
                <FontAwesomeIcon icon={faQuestionCircle}
                    data-tooltip-id="info-tooltip"
                    data-tooltip-content="This page lets you set up your hipfire sensitivity.
1. Enter desired cm/360 and your DPI value
2. Go into game and use the F1 keybind to turn
3. Adjust your sensitivity to turn exactly 360 degrees and land at the same spot you started"
                    data-tooltip-place="left" className="info-icon"/>
            </div>
            <div className="input-group">
                <label htmlFor="cm360">cm/360:</label>
                <input
                    type="number"
                    id="cm360"
                    name="cm360"
                    value={cm360}
                    onChange={(e) => setCm360(parseFloat(e.target.value))}
                    data-tooltip-id="info-tooltip"
                    data-tooltip-content="Desired hipfire cm needed to turn 360 degree."
                    data-tooltip-place="bottom" className="info-icon"
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
                    data-tooltip-id="info-tooltip"
                    data-tooltip-content="DPI of your mouse"
                    data-tooltip-place="bottom" className="info-icon"
                />
            </div>
        </div>
    );
}

export default MainSensitivity;
