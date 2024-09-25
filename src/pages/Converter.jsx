import React, {useEffect, useRef, useState} from 'react';
import {invoke} from '@tauri-apps/api/tauri';
import debounce from 'lodash/debounce';
import {Tooltip as ReactTooltip} from 'react-tooltip';
import {FontAwesomeIcon} from '@fortawesome/react-fontawesome';
import {faQuestionCircle} from '@fortawesome/free-solid-svg-icons';

function Converter() {
    const [games, setGames] = useState([]);
    const [sourceGameIndex, setsourceGameIndex] = useState(null);
    const [destGameIndex, setDestGameIndex] = useState(null);
    const [sourceSens, setSourceSens] = useState(0);
    const [newSens, setNewSens] = useState(0);
    const [sourceDpi, setSourceDpi] = useState(0);
    const [destDpi, setDestDpi] = useState(0);

    const isInitialMount = useRef(true);

    useEffect(() => {
        const fetchInitialValues = async () => {
            try {
                const response = await invoke('get_initial_values');
                setSourceDpi(response.dpi);
                setDestDpi(response.dpi);
            } catch (error) {
                console.error('Failed to fetch initial values:', error);
            }
        };

        fetchInitialValues();

        const fetchGames = async () => {
            try {
                const response = await invoke('get_games');
                const updatedGames = [{name: 'cm/360', yaw: 360.0}, ...response];
                setGames(updatedGames);
            } catch (error) {
                console.error('Failed to fetch games: ', error);
            }
        };

        fetchGames();
    }, []);

    const calculateSens = debounce(async () => {
        try {
            if (sourceGameIndex !== null && destGameIndex !== null) {
                const sourceGameData = games[sourceGameIndex];
                const destGameData = games[destGameIndex];

                const response = await invoke('convert_sens', {
                    dpi: sourceDpi,
                    sens: sourceSens,
                    yaw1: sourceGameData.yaw,
                    newDpi: destDpi,
                    yaw2: destGameData.yaw
                });

                setNewSens(response);
            }
        } catch (error) {
            console.error('Failed to calculate sens', error);
        }
    }, 500); // Debounce by 500ms

    useEffect(() => {
        if (isInitialMount.current) {
            isInitialMount.current = false;
        } else {
            calculateSens();
        }
    }, [sourceSens, sourceDpi, destDpi, sourceGameIndex, destGameIndex]);

    const isSourceCm360 = games[sourceGameIndex]?.name === 'cm/360';
    const isDestCm360 = games[destGameIndex]?.name === 'cm/360';

    return (
        <div className="main-container">
            <ReactTooltip id="info-tooltip" className="tooltip-box"/>
            <div className="info-container">
                <FontAwesomeIcon icon={faQuestionCircle}
                                 data-tooltip-id="info-tooltip"
                                 data-tooltip-content="This page lets you convert sensitivities between games you measured before.
1. Select Source and Destination games
2. Enter Source and Destination DPI
3. Enter your Sensitivity for the first game"
                                 data-tooltip-place="left" className="info-icon"/>
            </div>
            <div className="input-group">
                <label htmlFor="source-select">Select Source-Game:</label>
                <select
                    id="source-select"
                    name="source-select"
                    value={sourceGameIndex !== null ? sourceGameIndex : ''}
                    onChange={(e) => setsourceGameIndex(parseInt(e.target.value))}
                    data-tooltip-id="info-tooltip"
                    data-tooltip-content="Select your game"
                    data-tooltip-place="bottom"
                    className="info-icon"
                >
                    <option value="">Select a game</option>
                    {games.map((game, index) => (
                        <option key={index} value={index}>
                            {game.name}
                        </option>
                    ))}
                </select>
            </div>
            <div className="sens-group">
                <div className="input-group">
                    <label htmlFor="source-sens">Sensitivity:</label>
                    <input
                        type="number"
                        id="source-sens"
                        name="source-sens"
                        value={sourceSens}
                        onChange={(e) => setSourceSens(parseFloat(e.target.value))}
                        data-tooltip-id="info-tooltip"
                        data-tooltip-content="Sensitivity you want to convert"
                        data-tooltip-place="bottom" className="info-icon"
                    />
                </div>
                <div className="input-group">
                    <label htmlFor="dpi">DPI:</label>
                    <input
                        type="number"
                        id="dpi"
                        name="dpi"
                        value={sourceDpi}
                        onChange={(e) => setSourceDpi(parseInt(e.target.value))}
                        data-tooltip-id="info-tooltip"
                        data-tooltip-content="DPI of your mouse"
                        data-tooltip-place="bottom" className="info-icon"
                        disabled={isSourceCm360}
                    />
                </div>
            </div>
            <div className="input-group">
                <label htmlFor="dest-select">Select Destination-Game:</label>
                <select
                    id="dest-select"
                    name="dest-select"
                    value={destGameIndex !== null ? destGameIndex : ''}
                    onChange={(e) => setDestGameIndex(parseInt(e.target.value))}
                    data-tooltip-id="info-tooltip"
                    data-tooltip-content="Select your game"
                    data-tooltip-place="bottom"
                    className="info-icon"
                >
                    <option value="">Select a game</option>
                    {games.map((game, index) => (
                        <option key={index} value={index}>
                            {game.name}
                        </option>
                    ))}
                </select>
            </div>
            <div className="sens-group">
                <div className="input-group">
                    <label htmlFor="new-sens">Sensitivity:</label>
                    <input
                        type="number"
                        id="new-sens"
                        name="new-sens"
                        value={newSens}
                        onChange={(e) => setNewSens(parseFloat(e.target.value))}
                        data-tooltip-id="info-tooltip"
                        data-tooltip-content="Sensitivity you want to convert"
                        data-tooltip-place="bottom" className="info-icon"
                    />
                </div>
                <div className="input-group">
                    <label htmlFor="dpi2">DPI:</label>
                    <input
                        type="number"
                        id="dpi2"
                        name="dpi2"
                        value={destDpi}
                        onChange={(e) => setDestDpi(parseInt(e.target.value))}
                        data-tooltip-id="info-tooltip"
                        data-tooltip-content="DPI of your mouse"
                        data-tooltip-place="bottom" className="info-icon"
                        disabled={isDestCm360}
                    />
                </div>
            </div>
        </div>
    );
}

export default Converter;
