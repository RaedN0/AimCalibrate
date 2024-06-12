import React, {useEffect, useState} from 'react';
import {invoke} from '@tauri-apps/api/tauri';

function Settings() {
    const [hotkey, setHotkey] = useState('');
    const [settingHotkey, setSettingHotkey] = useState(false);

    useEffect(() => {
        invoke('get_hotkey').then((hotkey) => setHotkey(hotkey));
    }, []);

    const updateHotkey = (newHotkey) => {
        invoke('set_hotkey', {newHotkey}).then(() => setHotkey(newHotkey));
    };

    const handleKeyPress = (event) => {
        if (settingHotkey) {
            const key = event.key;
            const hotkeyString = `${key}`;

            updateHotkey(hotkeyString);
            setSettingHotkey(false);
        }
    };

    return (
        <div
            tabIndex="0"
            onKeyDown={handleKeyPress}
            style={{outline: 'none'}}
        >
            <h1>Set Hotkey</h1>
            <h2>Current Hotkey: {hotkey}</h2>
            <button onClick={() => setSettingHotkey(true)}>
                {settingHotkey ? 'Press any key...' : 'Set Hotkey'}
            </button>
        </div>
    );
}

export default Settings;
