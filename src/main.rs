use anyhow::Context;
use palette::{color_difference::Wcag21RelativeContrast, convert::FromColorUnclamped, Hsl, Srgb};
use rand::seq::SliceRandom;
use rand::{distributions::Alphanumeric, Rng};
use std::process::{Command, Stdio};

fn gen_random_theme_name() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

#[derive(Debug, serde::Deserialize)]
struct ValidColorItem {
    name: String,
    hex: String,
}

#[derive(Debug)]
struct Color {
    name: String,
    srgb: Srgb<u8>,
    hsl: Hsl,
}

impl Color {
    fn contrast_with(&self, other: &Self) -> f32 {
        let self_color: Srgb<f32> = self.srgb.into_format();
        let other_color: Srgb<f32> = other.srgb.into_format();
        self_color.relative_contrast(other_color)
    }
}

impl TryFrom<ValidColorItem> for Color {
    type Error = anyhow::Error;

    fn try_from(value: ValidColorItem) -> Result<Self, Self::Error> {
        let srgb_color: Srgb<u8> = value.hex.parse().unwrap();
        let color: Srgb<f32> = srgb_color.into_format();
        let hsl_color: Hsl = Hsl::from_color_unclamped(color);
        Ok(Self {
            name: value.name,
            srgb: srgb_color,
            hsl: hsl_color,
        })
    }
}

#[derive(Debug)]
struct Pallete<'a> {
    name: String,

    bg0: &'a Color, // main background
    bg1: &'a Color,
    bg2: &'a Color,
    bg3: &'a Color,
    bg4: &'a Color,

    fg0: &'a Color,
    fg1: &'a Color, // main foreground
    fg2: &'a Color,
    fg3: &'a Color,
    fg4: &'a Color, // gray0

    gray1: &'a Color,

    red0: &'a Color, // neutral
    red1: &'a Color, // bright
    green0: &'a Color,
    green1: &'a Color,
    yellow0: &'a Color,
    yellow1: &'a Color,
    blue0: &'a Color,
    blue1: &'a Color,
    purple0: &'a Color,
    purple1: &'a Color,
    aqua0: &'a Color,
    aqua1: &'a Color,
    orange0: &'a Color,
    orange1: &'a Color,
}

impl<'a> Pallete<'a> {
    fn save_to_helix_themes(&'a self) -> anyhow::Result<()> {
        let theme = self.to_helix_theme();
        std::fs::write(
            &format!("/Users/silas/.config/helix/themes/{}.toml", self.name),
            theme,
        )?;
        Ok(())
    }

    fn to_helix_theme(&'a self) -> String {
        // Makes building the template easier
        let bg0 = format!("#{:x}", self.bg0.srgb); // main background
        let bg1 = format!("#{:x}", self.bg1.srgb);
        let bg2 = format!("#{:x}", self.bg2.srgb);
        let bg3 = format!("#{:x}", self.bg3.srgb);
        let bg4 = format!("#{:x}", self.bg4.srgb);

        let fg0 = format!("#{:x}", self.fg0.srgb);
        let fg1 = format!("#{:x}", self.fg1.srgb); // main foreground
        let fg2 = format!("#{:x}", self.fg2.srgb);
        let fg3 = format!("#{:x}", self.fg3.srgb);
        let fg4 = format!("#{:x}", self.fg4.srgb); // gray0

        let gray1 = format!("#{:x}", self.gray1.srgb);

        let red0 = format!("#{:x}", self.red0.srgb); // neutral
        let red1 = format!("#{:x}", self.red1.srgb); // bright
        let green0 = format!("#{:x}", self.green0.srgb);
        let green1 = format!("#{:x}", self.green1.srgb);
        let yellow0 = format!("#{:x}", self.yellow0.srgb);
        let yellow1 = format!("#{:x}", self.yellow1.srgb);
        let blue0 = format!("#{:x}", self.blue0.srgb);
        let blue1 = format!("#{:x}", self.blue1.srgb);
        let purple0 = format!("#{:x}", self.purple0.srgb);
        let purple1 = format!("#{:x}", self.purple1.srgb);
        let aqua0 = format!("#{:x}", self.aqua0.srgb);
        let aqua1 = format!("#{:x}", self.aqua1.srgb);
        let orange0 = format!("#{:x}", self.orange0.srgb);
        let orange1 = format!("#{:x}", self.orange1.srgb);

        format!(
            r##"
# Author : Jakub Bartodziej <kubabartodziej@gmail.com>
# The theme uses the gruvbox dark palette with standard contrast: github.com/morhetz/gruvbox

"attribute" = "aqua1"
"keyword" = {{ fg = "red1" }}
"keyword.directive" = "red0"
"namespace" = "aqua1"
"punctuation" = "orange1"
"punctuation.delimiter" = "orange1"
"operator" = "purple1"
"special" = "purple0"
"variable.other.member" = "blue1"
"variable" = "fg1"
"variable.builtin" = "orange1"
"variable.parameter" = "fg2"
"type" = "yellow1"
"type.builtin" = "yellow1"
"constructor" = {{ fg = "purple1", modifiers = ["bold"] }}
"function" = {{ fg = "green1", modifiers = ["bold"] }}
"function.macro" = "aqua1"
"function.builtin" = "yellow1"
"tag" = "red1"
"comment" = {{ fg = "gray1", modifiers = ["italic"]  }}
"constant" = {{ fg = "purple1" }}
"constant.builtin" = {{ fg = "purple1", modifiers = ["bold"] }}
"string" = "green1"
"constant.numeric" = "purple1"
"constant.character.escape" = {{ fg = "fg2", modifiers = ["bold"] }}
"label" = "aqua1"
"module" = "aqua1"

"diff.plus" = "green1"
"diff.delta" = "orange1"
"diff.minus" = "red1"

"warning" = "orange1"
"error" = "red1"
"info" = "aqua1"
"hint" = "blue1"

"ui.background" = {{ bg = "bg0" }}
"ui.linenr" = {{ fg = "bg4" }}
"ui.linenr.selected" = {{ fg = "yellow1" }}
"ui.cursorline" = {{ bg = "bg1" }}
"ui.statusline" = {{ fg = "fg1", bg = "bg2" }}
"ui.statusline.normal" = {{ fg = "fg1", bg = "bg2" }}
"ui.statusline.insert" = {{ fg = "fg1", bg = "blue0" }}
"ui.statusline.select" = {{ fg = "fg1", bg = "orange0" }}
"ui.statusline.inactive" = {{ fg = "fg4", bg = "bg1" }}
"ui.bufferline" = {{ fg = "fg1", bg = "bg1" }}
"ui.bufferline.active" = {{ fg = "bg0", bg = "yellow0" }}
"ui.bufferline.background" = {{ bg = "bg2" }}
"ui.popup" = {{ bg = "bg1" }}
"ui.window" = {{ bg = "bg1" }}
"ui.help" = {{ bg = "bg1", fg = "fg1" }}
"ui.text" = {{ fg = "fg1" }}
"ui.text.focus" = {{ fg = "fg1" }}
"ui.selection" = {{ bg = "bg2" }}
"ui.selection.primary" = {{ bg = "bg3" }}
"ui.cursor.primary" = {{ bg = "fg4", fg = "bg1" }}
"ui.cursor.match" = {{ bg = "bg3" }}
"ui.menu" = {{ fg = "fg1", bg = "bg2" }}
"ui.menu.selected" = {{ fg = "bg2", bg = "blue1", modifiers = ["bold"] }}
"ui.virtual.whitespace" = "bg2"
"ui.virtual.ruler" = {{ bg = "bg1" }}
"ui.virtual.inlay-hint" = {{ fg = "gray1" }}
"ui.virtual.wrap" = {{ fg = "bg2" }}
"ui.virtual.jump-label" = {{ fg = "purple0", modifiers = ["bold"] }}

"diagnostic.warning" = {{ underline = {{ color = "orange1", style = "curl" }} }}
"diagnostic.error" = {{ underline = {{ color = "red1", style = "curl" }} }}
"diagnostic.info" = {{ underline = {{ color = "aqua1", style = "curl" }} }}
"diagnostic.hint" = {{ underline = {{ color = "blue1", style = "curl" }} }}
"diagnostic.unnecessary" = {{ modifiers = ["dim"] }}
"diagnostic.deprecated" = {{ modifiers = ["crossed_out"] }}

"markup.heading" = "aqua1"
"markup.bold" = {{ modifiers = ["bold"] }}
"markup.italic" = {{ modifiers = ["italic"] }}
"markup.strikethrough" = {{ modifiers = ["crossed_out"] }}
"markup.link.url" = {{ fg = "green1", modifiers = ["underlined"] }}
"markup.link.text" = "red1"
"markup.raw" = "red1"

[palette]
bg0 = "{bg0}"
bg1 = "{bg1}"
bg2 = "{bg2}"
bg3 = "{bg3}"
bg4 = "{bg4}"

fg0 = "{fg0}"
fg1 = "{fg1}"
fg2 = "{fg2}"
fg3 = "{fg3}"
fg4 = "{fg4}"

gray1 = "{gray1}"

red0 = "{red0}"
red1 = "{red1}"
green0 = "{green0}"
green1 = "{green1}"
yellow0 = "{yellow0}"
yellow1 = "{yellow1}"
blue0 = "{blue0}"
blue1 = "{blue1}"
purple0 = "{purple0}"
purple1 = "{purple1}"
aqua0 = "{aqua0}"
aqua1 = "{aqua1}"
orange0 = "{orange0}"
orange1 = "{orange1}"
        "##
        )
    }
}

fn select_random_color(
    colors: &Vec<Color>,
    min_lightness: Option<f32>,
    max_lightness: Option<f32>,
) -> anyhow::Result<&Color> {
    let min_lightness = min_lightness.unwrap_or(0.);
    let max_lightness = max_lightness.unwrap_or(f32::MAX);
    let colors: Vec<&Color> = colors
        .iter()
        .filter(|c| c.hsl.lightness < max_lightness && c.hsl.lightness > min_lightness)
        .collect();
    colors
        .choose(&mut rand::thread_rng())
        .map(|c| *c)
        .context("empty slice")
}

fn try_get_color<'a, 'b>(
    colors: &'a Vec<Color>,
    comparison_colors: &'b [&'a Color],
    min_contrast: f32,
    max_contrast: f32,
    min_lightness: Option<f32>,
    max_lightness: Option<f32>,
) -> anyhow::Result<&'a Color> {
    let mut i = 0;
    // Only try x times
    while i < 10 {
        let color = select_random_color(colors, min_lightness, max_lightness)?;
        if comparison_colors.iter().all(|c| {
            let contrast_ratio = c.contrast_with(&color);
            contrast_ratio > min_contrast && contrast_ratio < max_contrast
        }) {
            return Ok(color);
        }
        i += 1;
    }
    anyhow::bail!("couldn't get a color that has the required contrast")
}

fn generate_random_palette(colors: &Vec<Color>) -> anyhow::Result<Pallete> {
    let bg0 = select_random_color(colors, None, Some(0.2))?;
    let bg1 = select_random_color(colors, None, Some(0.2))?;
    let bg2 = select_random_color(colors, None, Some(0.2))?;
    let bg3 = select_random_color(colors, None, Some(0.2))?;
    let bg4 = select_random_color(colors, None, Some(0.2))?;

    let fg0 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;
    let fg1 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;
    let fg2 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;
    let fg3 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;
    let fg4 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;

    let gray1 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 2., 19., None, None)?;

    let red0 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;
    let red1 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;
    let green0 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;
    let green1 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;
    let yellow0 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;
    let yellow1 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;
    let blue0 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;
    let blue1 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;
    let purple0 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;
    let purple1 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;
    let aqua0 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;
    let aqua1 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;
    let orange0 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;
    let orange1 = try_get_color(colors, &[bg1, bg2, bg3, bg4], 3., 19., None, None)?;

    let name = gen_random_theme_name();
    Ok(Pallete {
        name,
        bg0,
        bg1,
        bg2,
        bg3,
        bg4,
        fg0,
        fg1,
        fg2,
        fg3,
        fg4,
        gray1,
        red0,
        red1,
        green0,
        green1,
        yellow0,
        yellow1,
        blue0,
        blue1,
        purple0,
        purple1,
        aqua0,
        aqua1,
        orange0,
        orange1,
    })
}

fn main() -> anyhow::Result<()> {
    let mut rdr = csv::Reader::from_path("data/colors.csv")?;
    let colors = rdr
        .deserialize()
        .collect::<Result<Vec<ValidColorItem>, csv::Error>>()?;
    let colors = colors
        .into_iter()
        .map(|c| c.try_into())
        .collect::<anyhow::Result<Vec<Color>>>()?;

    loop {
        let pallete = generate_random_palette(&colors)?;
        pallete.save_to_helix_themes()?;

        let child = Command::new("./scripts/expect.sh")
            .arg(pallete.name)
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        let out = child.wait_with_output()?;
        let out = String::from_utf8(out.stderr)?;
        if out.is_empty() {
            anyhow::bail!("bad output");
        } else if out.contains("d") {
            break;
        } else if out.contains("f") {
            // We need to freeze in here
        }
    }

    Ok(())
}
