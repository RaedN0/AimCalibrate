import React, {useEffect, useState, useCallback} from 'react';
import {invoke} from '@tauri-apps/api/tauri';
import debounce from 'lodash/debounce';

function Settings() {
    const [hotkeys, setHotkeys] = useState({
        hotkey1: '',
        hotkey2: '',
        hotkey3: '',
        hotkey4: ''
    });
    const [settingHotkey, setSettingHotkey] = useState(null);
    const [sliderValue, setSliderValue] = useState(1);

    useEffect(() => {
        const fetchInitialValues = async () => {
            try {
                const response = await invoke('get_app_settings');
                setSliderValue(response.turn_speed.toFixed(1));
                setHotkeys({
                    hotkey1: response.hotkeys.at(0),
                    hotkey2: response.hotkeys.at(1),
                    hotkey3: response.hotkeys.at(2),
                    hotkey4: response.hotkeys.at(3)
                });
            } catch (error) {
                console.error('Failed to fetch initial values:', error);
            }
        };

        fetchInitialValues();
    }, []);

    useEffect(() => {
        debouncedUpdateSettings(sliderValue, hotkeys)
    }, [hotkeys, sliderValue]);

    const debouncedUpdateSettings = useCallback(
        debounce((sliderValue, hotkeys) => {
            invoke('set_app_settings', {
                turnSpeed: parseFloat(sliderValue),
                hotkeys: [hotkeys.hotkey1, hotkeys.hotkey2, hotkeys.hotkey3, hotkeys.hotkey4]
            }).catch((error) => {
                console.error('Failed to set user settings:', error);
            });
        }, 500), // Debounce delay of 500ms
        []
    );

    const handleSliderChange = (event) => {
        setSliderValue(event.target.value);
    };

    const updateHotkey = (newHotkey, key) => {
        setHotkeys((prevHotkeys) => ({
            ...prevHotkeys,
            [key]: newHotkey
        }));
    };

    const handleKeyPress = (event) => {
        if (settingHotkey) {
            event.preventDefault(); // Prevent default action to avoid conflicts
            const key = event.key;
            const ctrl = event.ctrlKey ? 'Ctrl+' : '';
            const alt = event.altKey ? 'Alt+' : '';
            const shift = event.shiftKey ? 'Shift+' : '';
            const meta = event.metaKey ? 'Meta+' : '';

            // Only set a hotkey if it's not a modifier key by itself
            if (key !== 'Control' && key !== 'Alt' && key !== 'Shift' && key !== 'Meta') {
                const hotkeyString = `${ctrl}${alt}${shift}${meta}${key}`;

                updateHotkey(hotkeyString, settingHotkey);
                setSettingHotkey(null);
            }
        }
    };

    return (
        <div>
            <div className="keybind-container" tabIndex="0" onKeyDown={handleKeyPress}>
                <div className="current-keybind">
                    Current Hotkeys:
                    <div>Hotkey 1: {hotkeys.hotkey1}</div>
                    <div>Hotkey 2: {hotkeys.hotkey2}</div>
                    <div>Hotkey 3: {hotkeys.hotkey3}</div>
                    <div>Hotkey 4: {hotkeys.hotkey4}</div>
                </div>
                <div className="keybind-buttons">
                    <button className="keybind-button" onClick={() => setSettingHotkey('hotkey1')}>
                        {settingHotkey === 'hotkey1' ? 'Press any key...' : 'Set Hotkey 1'}
                    </button>
                    <button className="keybind-button" onClick={() => setSettingHotkey('hotkey2')}>
                        {settingHotkey === 'hotkey2' ? 'Press any key...' : 'Set Hotkey 2'}
                    </button>
                    <button className="keybind-button" onClick={() => setSettingHotkey('hotkey3')}>
                        {settingHotkey === 'hotkey3' ? 'Press any key...' : 'Set Hotkey 3'}
                    </button>
                    <button className="keybind-button" onClick={() => setSettingHotkey('hotkey4')}>
                        {settingHotkey === 'hotkey4' ? 'Press any key...' : 'Set Hotkey 4'}
                    </button>
                </div>
            </div>
            <div className="slider">
                <label htmlFor="sensitivity-slider">Turn speed: {sliderValue}</label>
                <input
                    type="range"
                    id="sensitivity-slider"
                    min="0.1"
                    max="5"
                    step="0.1"
                    value={sliderValue}
                    onChange={handleSliderChange}
                />
            </div>
        </div>
    );
}

export default Settings;
