import React, { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { Tooltip as ReactTooltip } from 'react-tooltip';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faQuestionCircle } from '@fortawesome/free-solid-svg-icons';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';
import DialogContentText from '@mui/material/DialogContentText';
import DialogTitle from '@mui/material/DialogTitle';
import Button from '@mui/material/Button';
import TextField from '@mui/material/TextField';

function MeasureYaw() {
    const [sens, setSens] = useState(0);
    const [yaw, setYaw] = useState(0);
    const [lowerLimit, setLowerLimit] = useState(0);
    const [upperLimit, setUpperLimit] = useState(0);
    const [isDialogOpen, setIsDialogOpen] = useState(false);
    const [dialogText, setDialogText] = useState('');

    const isInitialMount = useRef(true);

    useEffect(() => {
        const fetchInitialValues = async () => {
            try {
                startListener();
                const response = await invoke('get_yaw_stuff');
                console.log(response);
                setSens(response.sens);
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
            console.log(event.payload);
            setSens(event.payload.sens);
            setYaw(event.payload.yaw);
            setLowerLimit(event.payload.lower_limit);
            setUpperLimit(event.payload.upper_limit);
        });
    }

    const openDialog = () => setIsDialogOpen(true);
    const closeDialog = () => setIsDialogOpen(false);

    const handleDialogSave = () => {
        // Save the dialog text here
        console.log('Dialog text saved:', dialogText);
        invoke('save_game_yaw', {
            name: dialogText
        })
        closeDialog();
    };

    return (
        <div className="main-container">
            <ReactTooltip id="info-tooltip" className="tooltip-box" />
            <div className="info-container">
                <FontAwesomeIcon icon={faQuestionCircle}
                    data-tooltip-id="info-tooltip"
                    data-tooltip-content="This page lets you measure the yaw value of a game.
1. Enter your game sensitivity.
2. Press hotkey 1, turn 360 degrees and press hotkey 1 again.
3. Press hotkey 2 to turn.
4. If you turned less than 360 degrees, press hotkey 3, if you turned more than 360 degrees press hotkey 4.
5. Repeat step 3 and 4 until it turns exactly 360 degrees. The value shown in the textbox is the yaw value"
                    data-tooltip-place="bottom" className="info-icon" />
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
                    data-tooltip-content="Your sensitivity in the game you want to measure"
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
                    data-tooltip-content="Estimated yaw value of the game"
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
                        data-tooltip-content="Minimum possible yaw value"
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
                        data-tooltip-content="Maximum possible yaw value"
                        data-tooltip-place="top" className="info-icon"
                    />
                </div>
            </div>
            <Button variant="outlined" onClick={openDialog}>Save</Button>
            <Dialog open={isDialogOpen} onClose={closeDialog}>
                <DialogTitle style={{ backgroundColor: '#1f1f1f', color: '#e0e0e0' }}>Save yaw</DialogTitle>
                <DialogContent style={{ backgroundColor: '#1f1f1f', color: '#e0e0e0' }}>
                    <DialogContentText style={{ color: '#e0e0e0' }}>
                        Type in the games name to save it.
                    </DialogContentText>
                    <TextField
                        autoFocus
                        margin="dense"
                        id="dialogText"
                        label="Game name"
                        type="text"
                        fullWidth
                        variant="standard"
                        value={dialogText}
                        onChange={(e) => setDialogText(e.target.value)}
                        InputLabelProps={{ style: { color: '#e0e0e0' } }}
                        InputProps={{
                            style: { color: '#e0e0e0' },
                            sx: {
                                '& .MuiInput-underline:before': { borderBottomColor: '#e0e0e0' },
                                '& .MuiInput-underline:after': { borderBottomColor: '#007bff' },
                                '& .MuiInput-underline:hover:before': { borderBottomColor: '#e0e0e0' },
                            },
                        }}
                    />
                </DialogContent>
                <DialogActions style={{ backgroundColor: '#1f1f1f' }}>
                    <Button onClick={closeDialog} style={{ color: '#007bff' }}>Cancel</Button>
                    <Button onClick={handleDialogSave} style={{ color: '#007bff' }}>Save</Button>
                </DialogActions>
            </Dialog>
        </div>
    );
}

export default MeasureYaw;
