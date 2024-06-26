import React, {useEffect, useRef, useState} from 'react';
import {invoke} from '@tauri-apps/api/tauri';
import debounce from 'lodash.debounce';
import {listen} from '@tauri-apps/api/event';
import {Tooltip as ReactTooltip} from 'react-tooltip';
import {FontAwesomeIcon} from '@fortawesome/react-fontawesome';
import {faQuestionCircle} from '@fortawesome/free-solid-svg-icons';

function MeasureFov() {
    const [cm360, setCm360] = useState(0);
    const [dpi, setDpi] = useState(0);
    const [gameSens, setGameSens] = useState(0);
    const [fovHorizontal, setFovHorizontal] = useState(0);
    const [fov4ML3, setFov4ML3] = useState(0);
    const [fovVertical, setFovVertical] = useState(0);

    const isInitialMount = useRef(true);

    useEffect(() => {
        const fetchInitialValues = async () => {
            try {
                await startListener();
                const response = await invoke('get_initial_values');
                setCm360(response.cm360);
                setDpi(response.dpi);
                setGameSens(response.game_sens);
                setFovHorizontal(response.game_fov);
                setFov4ML3(horizontalTo4ML3(response.game_fov));
                setFovVertical(horizontalToVertical(response.game_fov));
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
            updateSettings(cm360, dpi, gameSens, fovHorizontal);
        }
    }, [cm360, dpi, gameSens, fovHorizontal]);

    async function startListener() {
        await listen('fov_update', (event) => {
            const {fov16} = event.payload;
            handleFov16Change(fov16)
        });
    }

    const handleFov16Change = (value) => {
        const newFov16 = parseFloat(value);
        setFovHorizontal(newFov16);
        setFov4ML3(horizontalTo4ML3(newFov16));
        setFovVertical(horizontalToVertical(newFov16));
    };

    const handleFov43Change = (value) => {
        const newFov43 = parseFloat(value);
        const newFov16 = fov4ML3ToHorizontal(newFov43);
        setFovHorizontal(newFov16);
        setFov4ML3(newFov43);
        setFovVertical(horizontalToVertical(newFov16));
    };

    const handleFov11Change = (value) => {
        const newFov11 = parseFloat(value);
        const newFov16 = verticalToHorizontal(newFov11);
        setFovHorizontal(newFov16);
        setFov4ML3(horizontalTo4ML3(newFov16));
        setFovVertical(newFov11);
    };

    const horizontalToVertical = (fov) => {
        return (2 * Math.atan(((Math.tan((fov / 2) * (Math.PI / 180))) * (window.screen.height / window.screen.width)))) * (180 / Math.PI);
    }

    const verticalToHorizontal = (fov) => {
        return 2 * Math.atan((Math.tan((fov / 2) * (Math.PI / 180)) / (window.screen.height / window.screen.width))) * (180 / Math.PI);
    }

    const horizontalTo4ML3 = (fov) => {
        return 2 * Math.atan(((window.screen.height / window.screen.width) / (3 / 4)) * Math.tan((fov / 2) * (Math.PI / 180))) * (180 / Math.PI);
    };

    const fov4ML3ToHorizontal = (fov) => {
        return 2 * Math.atan(((3 / 4) / (window.screen.height / window.screen.width)) * Math.tan((fov / 2) * (Math.PI / 180))) * (180 / Math.PI);
    };

    return (
        <div className="main-container">
            <ReactTooltip id="info-tooltip" className="tooltip-box"/>
            <div className="info-container">
                <FontAwesomeIcon icon={faQuestionCircle}
                                 data-tooltip-id="info-tooltip"
                                 data-tooltip-content="This page lets you measure your FOV.
1. Enter your cm/360 for hipfire, DPI and game sensitivity for hipfire that matches the cm/360.
2. Scope in and line up something at the edge of your screen.
3. Scope out, press F1, move your crosshair to the object you lined up, and press F1 again.
4. Your FOV will be displayed in the textboxes at the bottom. These can also be used to convert your FOV.
IMPORTANT: For the conversion to be accurate, have AimCalibrate on the screen you game on, when switching to this tab. It looks at your aspect ratio of your screen, so if the screen you have AimCalibrate on, has another aspect ratio than the one you game on, the only correct value will be the horizontal one. The other two might be wrong."
                                 data-tooltip-place="bottom" className="info-icon"/>
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
                    data-tooltip-content="How many cm your mouse has to move to turn 360 degree."
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
            <div className="input-group">
                <label htmlFor="gameSens">Game Sens:</label>
                <input
                    type="number"
                    id="gameSens"
                    name="gameSens"
                    value={gameSens}
                    onChange={(e) => setGameSens(parseFloat(e.target.value))}
                    data-tooltip-id="info-tooltip"
                    data-tooltip-content="Hipfire sensitivity that matches the cm/360"
                    data-tooltip-place="bottom" className="info-icon"
                />
            </div>
            <div className="fov-group">
                <div className="input-group">
                    <label htmlFor="fovHorizontal">Horizontal:</label>
                    <input
                        type="number"
                        id="fovHorizontal"
                        name="fovHorizontal"
                        value={fovHorizontal}
                        onChange={(e) => handleFov16Change(e.target.value)}
                        data-tooltip-id="info-tooltip"
                        data-tooltip-content="Actual horizontal FOV.
Games using this:
- Overwatch
- Valorant
- xDefiant
- The Finals"
                        data-tooltip-place="top" className="info-icon"
                    />
                </div>
                <div className="input-group">
                    <label htmlFor="fov4ML3">4ML3:</label>
                    <input
                        type="number"
                        id="fov4ML3"
                        name="fov4ML3"
                        value={fov4ML3}
                        onChange={(e) => handleFov43Change(e.target.value)}
                        data-tooltip-id="info-tooltip"
                        data-tooltip-content="Horizontally measured, but vertically locked.
Games using this:
- CS2
- Quake
- Apex"
                        data-tooltip-place="top" className="info-icon"
                    />
                </div>
                <div className="input-group">
                    <label htmlFor="fovVertical">Vertical:</label>
                    <input
                        type="number"
                        id="fovVertical"
                        name="fovVertical"
                        value={fovVertical}
                        onChange={(e) => handleFov11Change(e.target.value)}
                        data-tooltip-id="info-tooltip"
                        data-tooltip-content="Actual Vertical FOV.
Games using this:
- Battlefield
- R6"
                        data-tooltip-place="top-end" className="info-icon"
                    />
                </div>
            </div>
        </div>
    );
}

export default MeasureFov;
