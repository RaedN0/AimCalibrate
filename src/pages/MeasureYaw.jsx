import React, {useEffect, useRef, useState} from 'react';
import {invoke} from '@tauri-apps/api/tauri';
import debounce from 'lodash.debounce';
import {listen} from '@tauri-apps/api/event';
import {Tooltip as ReactTooltip} from 'react-tooltip';
import {FontAwesomeIcon} from '@fortawesome/react-fontawesome';
import {faQuestionCircle} from '@fortawesome/free-solid-svg-icons';

function MeasureFov() {
    const [sens, setSens] = useState(0);
    const [yaw, setYaw] = useState(0);
    const [counts, setCounts] = useState(0);
    const [inc, setInc] = useState(0);
    const [lowerLimit, setLowerLimit] = useState(0);
    const [upperLimit, setUpperLimit] = useState(0);

    const isInitialMount = useRef(true);

    useEffect(() => {
        const fetchInitialValues = async () => {
            try {
                startListener();
                const response = await invoke('get_yaw_stuff');
                setSens(response.sens);
                setCounts(response.counts);
                setYaw(response.yaw);
                setLowerLimit(response.lower_limit);
                setUpperLimit(response.upper_limit);
            } catch (error) {
                console.error('Failed to fetch initial values:', error);
            }
        };

        fetchInitialValues();
    }, []);

    const handleSensChange = async (sens) => {
        setSens(sens);
        try {
            const response = await invoke('set_yaw_stuff', {
                sens: sens
            });
            setYaw(response.yaw);
            setLowerLimit(response.lower_limit);
            setUpperLimit(response.upper_limit);
        } catch (error) {
            console.error('Failed to set user settings:', error);
        }
    };

    async function startListener() {
        await listen('yaw_update', (event) => {
            console.log(event.payload)
            setSens(event.payload.sens);
            setCounts(event.payload.counts);
            setInc(event.payload.inc);
            setYaw(event.payload.yaw);
            setLowerLimit(event.payload.lower_limit);
            setUpperLimit(event.payload.upper_limit);
        });
    }

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
                <label htmlFor="sens">Sens:</label>
                <input
                    type="number"
                    id="sens"
                    name="sens"
                    value={sens}
                    onChange={(e) => handleSensChange(parseFloat(e.target.value))}
                    data-tooltip-id="info-tooltip"
                    data-tooltip-content="How many cm your mouse has to move to turn 360 degree."
                    data-tooltip-place="bottom" className="info-icon"
                />
            </div>
            <div className="input-group">
                <label htmlFor="yaw">yaw:</label>
                <input
                    type="number"
                    id="yaw"
                    name="yaw"
                    value={yaw}
                    readOnly
                    data-tooltip-id="info-tooltip"
                    data-tooltip-content="DPI of your mouse"
                    data-tooltip-place="bottom" className="info-icon"
                />
            </div>
            <div className="input-group">
                <label htmlFor="counts">Counts:</label>
                <input
                    type="number"
                    id="counts"
                    name="counts"
                    value={counts}
                    readOnly
                    data-tooltip-id="info-tooltip"
                    data-tooltip-content="DPI of your mouse"
                    data-tooltip-place="bottom" className="info-icon"
                />
            </div>
            <div className="limits-group">
                <div className="input-group">
                    <label htmlFor="lower">Lower limit:</label>
                    <input
                        type="number"
                        id="lower"
                        name="lower"
                        value={lowerLimit}
                        readOnly
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
                    <label htmlFor="upper">Upper limit:</label>
                    <input
                        type="number"
                        id="upper"
                        name="upper"
                        value={upperLimit}
                        readOnly
                        data-tooltip-id="info-tooltip"
                        data-tooltip-content="Horizontally measured, but vertically locked.
Games using this:
- CS2
- Quake
- Apex"
                        data-tooltip-place="top" className="info-icon"
                    />
                </div>
            </div>
        </div>
    );
}

export default MeasureFov;
