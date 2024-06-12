import React, {useEffect, useState, useCallback} from 'react';
import {invoke} from '@tauri-apps/api/tauri';
import debounce from 'lodash/debounce';

function Settings() {
    const [hotkey, setHotkey] = useState('');
    const [settingHotkey, setSettingHotkey] = useState(false);

    useEffect(() => {
        invoke('get_hotkey').then((hotkey) => setHotkey(hotkey));
    }, []);

    const [sliderValue, setSliderValue] = useState(1);

    useEffect(() => {
        debouncedUpdateSettings(sliderValue)
    }, [sliderValue]);

    const handleSliderChange = (event) => {
        setSliderValue(event.target.value);
    };

    const debouncedUpdateSettings = useCallback(
        debounce((sliderValue) => {
            console.log(sliderValue);
            invoke('set_app_settings', {
                turnSpeed: parseFloat(sliderValue)
            }).catch((error) => {
                console.error('Failed to set user settings:', error);
            });
        }, 500), // Debounce delay of 500ms
        []
    );

    const updateHotkey = (newHotkey) => {
        invoke('set_hotkey', {newHotkey}).then(() => setHotkey(newHotkey));
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

                updateHotkey(hotkeyString);
                setSettingHotkey(false);
            }
        }
    };

    return (
        <div>
            <div className="keybind-container" tabIndex="0"
                 onKeyDown={handleKeyPress}>
                <div className="current-keybind">
                    Current Hotkey: {hotkey}
                </div>
                <button className="keybind-button" onClick={() => setSettingHotkey(true)}>
                    {settingHotkey ? 'Press any key...' : 'Set Hotkey'}
                </button>
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
