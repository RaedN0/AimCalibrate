import React, {useEffect, useRef, useState} from 'react';
import {invoke} from '@tauri-apps/api/tauri';
import debounce from 'lodash.debounce';
import { listen } from '@tauri-apps/api/event';

function MeasureFov() {
    const [cm360, setCm360] = useState(0);
    const [dpi, setDpi] = useState(0);
    const [gameSens, setGameSens] = useState(0);
    const [fov16, setFov16] = useState(0);
    const [fov43, setFov43] = useState(0);
    const [fov11, setFov11] = useState(0);

    const isInitialMount = useRef(true);

    useEffect(() => {
        const fetchInitialValues = async () => {
            try {
                await startListener();
                const response = await invoke('get_initial_values');
                setCm360(response.cm360);
                setDpi(response.dpi);
                setGameSens(response.game_sens);
                setFov16(response.game_fov);
                setFov43(horizontalToFourThree(response.game_fov));
                setFov11(horizontalToOneOne(response.game_fov));
            } catch (error) {
                console.error('Failed to fetch initial values:', error);
            }
        };

        fetchInitialValues();
    }, []);

    const updateSettings = debounce((cm360, dpi, gameSens, gameFov) => {
        invoke('set_user_settings', {
            cm360: parseFloat(cm360),
            dpi: parseInt(dpi),
            gameSens: parseFloat(gameSens),
            gameFov: parseFloat(gameFov)
        }).catch((error) => {
            console.error('Failed to set user settings:', error);
        });
    }, 500);

    useEffect(() => {
        if (isInitialMount.current) {
            isInitialMount.current = false;
        } else {
            updateSettings(cm360, dpi, gameSens, fov16);
        }
    }, [cm360, dpi, gameSens, fov16]);

    async function startListener() {
        await listen('fov_update', (event) => {
            const { fov16 } = event.payload;
            console.log("updated");
            handleFov16Change(fov16)
        });
    }

    const handleFov16Change = (value) => {
        const newFov16 = parseFloat(value);
        setFov16(newFov16);
        setFov43(horizontalToFourThree(newFov16));
        setFov11(horizontalToOneOne(newFov16));
    };

    const handleFov43Change = (value) => {
        const newFov43 = parseFloat(value);
        const newFov16 = fourThreeToHorizontal(newFov43);
        setFov16(newFov16);
        setFov43(newFov43);
        setFov11(horizontalToOneOne(newFov16));
    };

    const handleFov11Change = (value) => {
        const newFov11 = parseFloat(value);
        const newFov16 = oneOneToHorizontal(newFov11);
        setFov16(newFov16);
        setFov43(horizontalToFourThree(newFov16));
        setFov11(newFov11);
    };

    const horizontalToFourThree = (fov) => {
        return (2 * Math.atan(((Math.tan((fov / 2) * (Math.PI / 180))) * 3) / 4)) * (180 / Math.PI);
    };

    const horizontalToOneOne = (fov) => {
        return (2 * Math.atan(((Math.tan((fov / 2) * (Math.PI / 180))) * 0.5625))) * (180 / Math.PI);
    };

    const fourThreeToHorizontal = (fov) => {
        return 2 * Math.atan((Math.tan((fov / 2) * (Math.PI / 180)) * 4) / 3) * (180 / Math.PI);
    };

    const oneOneToHorizontal = (fov) => {
        return 2 * Math.atan((Math.tan((fov / 2) * (Math.PI / 180)) / 0.5625)) * (180 / Math.PI);
    };

    return (
        <div className="main-container">
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
                <label htmlFor="gameSens">Game Sens:</label>
                <input
                    type="number"
                    id="gameSens"
                    name="gameSens"
                    value={gameSens}
                    onChange={(e) => setGameSens(parseFloat(e.target.value))}
                />
            </div>
            <div className="fov-group">
                <div className="input-group">
                    <label htmlFor="fov16">16:9:</label>
                    <input
                        type="number"
                        id="fov16"
                        name="fov16"
                        value={fov16}
                        onChange={(e) => handleFov16Change(e.target.value)}
                    />
                </div>
                <div className="input-group">
                    <label htmlFor="fov43">4:3:</label>
                    <input
                        type="number"
                        id="fov43"
                        name="fov43"
                        value={fov43}
                        onChange={(e) => handleFov43Change(e.target.value)}
                    />
                </div>
                <div className="input-group">
                    <label htmlFor="fov11">1:1:</label>
                    <input
                        type="number"
                        id="fov11"
                        name="fov11"
                        value={fov11}
                        onChange={(e) => handleFov11Change(e.target.value)}
                    />
                </div>
            </div>
        </div>
    );
}

export default MeasureFov;
