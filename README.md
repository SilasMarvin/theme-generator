# theme-generator

A somewhat random interactive theme-generator that pulls from [Color Names](https://github.com/meodai/color-names), generates a theme, and opens up Helix with that theme.

I wrote this because I wanted to make a color scheme but I needed to see it in my editor, not just on a pallete to know whether I liked it or not.

https://github.com/SilasMarvin/theme-generator/assets/19626586/a6a82137-19a0-40f7-8f6a-d9a7372a20ae

## Run

Run with either:
```
./presets/dracula_based.sh
```

Or:
```
./presets/gruvbox_based.sh
```

Make sure to have `expect` installed. This should be installed by default on MacOS.

## Controls

Running any of the two above commands will open up helix with a generated theme. From here you have three options:
- Press `tab` to change to the next generated theme.
- Press `s` to save the theme in your helix themes directory.
- Press `f` to freeze a color and randomly generate all other colors. After pressing `f` press the corresponding number / letter of the color you want to freeze.

## Adding New Editors

It should be pretty easy to add new editors. Just add a template and the corresponding expect script.

## Adding New Templates

You can add your own template pretty easily. Checkout the presets directory for inspiration.
