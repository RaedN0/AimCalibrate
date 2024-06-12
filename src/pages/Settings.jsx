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
