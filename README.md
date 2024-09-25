# AimCalibrate

AimCalibrate is a tool designed to help gamers fine-tune their sensitivity settings for hipfire and scoped aiming based on focal length scaling. It allows users to measure their FOV (Field of View) and convert it between different scales.

## Features

- Set up hipfire sensitivity based on cm/360 and DPI.
- Match scoped sensitivity to hipfire sensitivity using focal length scaling.
- Measure your FOV and convert it between different scales.
- Measure the yaw values of games.
- Convert sensitivities between games using the measured yaw values.

## Usage

### Setting Up Hipfire Sensitivity

1. Navigate to the **Main Sensitivity** tab.
2. Enter your desired **cm/360** and **DPI** values.
3. Go into your game and use `hotkey 1` to turn.
4. Adjust your sensitivity to turn exactly 360 degrees and land at the same spot you started.

### Matching Scoped Sensitivity

1. Navigate to the **Scoped Sensitivity** tab.
2. Enter your **cm/360** for hipfire, **DPI**, **hipfire FOV**, and **scoped FOV**.
3. Press `hotkey 1` to turn while scoped in.
4. Adjust your scope sensitivity to turn exactly 360 degrees.

### Measuring Your FOV

1. Navigate to the **Measure FOV** tab.
2. Enter your **cm/360** for hipfire, **DPI**, and **hipfire sensitivity**.
3. Scope in and line up something at the edge of your screen.
4. Scope out, press `hotkey 1`, move your crosshair to the object you lined up, and press `hotkey 1` again.
5. Your FOV will be displayed in the text boxes at the bottom. These can also be used to convert your FOV between different scales.

### Measuring Yaw

There are games that don't use a static yaw value but rather a "dynamic" one that changes depending on sensitivity. These need to be measured at different points, and then you need to find a curve that fits those points.

Also, some games vary when changing the FOV. These should be measured after you have set the FOV you want to use.

1. Navigate to the **Measure Yaw** tab.
2. Enter your game sensitivity.
3. Press `hotkey 1`, turn 360 degrees, and press `hotkey 1` again.
4. Press `hotkey 2` to turn.
5. If you turned less than 360 degrees, press `hotkey 3`; if you turned more than 360 degrees, press `hotkey 4`.
6. Repeat steps 4 and 5 until it turns exactly 360 degrees. The value shown in the textbox is the yaw value.

### Converting Sensitivities

You can only convert between games for which you have previously measured the yaw values or have added their yaw values to the `Games.json` file.  
Example of `Games.json`: [Games.json on GitHub](https://github.com/RaedN0/GameYawList/blob/main/Games.json)

1. Select the source and destination game.
2. Enter source and destination DPI.
3. Enter the sensitivity to convert.
4. The converted sensitivity will be shown in the second sensitivity textbox.

## Contact

If you have any questions or feedback, feel free to reach out:

- **Email**: [marcel.schoerghuber@gmail.com](mailto:marcel.schoerghuber@gmail.com)
- **Discord**: raedn