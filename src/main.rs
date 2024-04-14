use anyhow::Context;
use palette::{color_difference::Wcag21RelativeContrast, convert::FromColorUnclamped, Hsl, Srgb};
use rand::seq::SliceRandom;
use rand::{distributions::Alphanumeric, Rng};
use std::process::{Command, Stdio};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    template: String,

    #[arg(short, long)]
    color_count: usize,

    #[arg(short, long)]
    background_color_count: usize,
}

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
struct MaybeLockedColor<'a> {
    locked: bool,
    color: &'a Color,
}

impl<'a> From<&'a Color> for MaybeLockedColor<'a> {
    fn from(color: &'a Color) -> Self {
        Self {
            locked: false,
            color,
        }
    }
}

#[derive(Debug)]
struct Palette<'a> {
    name: String,
    template: String,
    color_count: usize,
    background_color_count: usize,
    palette_colors: Vec<MaybeLockedColor<'a>>,
}

impl<'a> Palette<'a> {
    fn permute_non_frozen_colors(&mut self, colors: &'a Vec<Color>) -> anyhow::Result<()> {
        for i in 0..self.background_color_count {
            if !self.palette_colors[i].locked {
                let color = select_random_color(colors, None, Some(0.2))?;
                self.palette_colors[i] = MaybeLockedColor::from(color);
            }
        }
        let bg_slice: Vec<&Color> = self.palette_colors[0..self.background_color_count]
            .iter()
            .map(|mlc| mlc.color.to_owned())
            .collect();
        for i in self.background_color_count..self.color_count {
            if !self.palette_colors[i].locked {
                let color = try_get_color(colors, &bg_slice, 3., 19., None, None)?;
                self.palette_colors[i] = MaybeLockedColor::from(color);
            }
        }
        Ok(())
    }

    fn generate_random_palette(
        template: String,
        color_count: usize,
        background_color_count: usize,
        colors: &Vec<Color>,
    ) -> anyhow::Result<Palette> {
        let mut palette_colors = Vec::new();
        for _ in 0..background_color_count {
            let color = select_random_color(colors, None, Some(0.2))?;
            palette_colors.push(color);
        }
        let bg_slice = &palette_colors[0..background_color_count].to_owned();
        for _ in 0..color_count - background_color_count {
            let color = try_get_color(colors, bg_slice, 3., 19., None, None)?;
            palette_colors.push(color);
        }
        let name = gen_random_theme_name();
        let palette_colors = palette_colors
            .into_iter()
            .map(MaybeLockedColor::from)
            .collect();
        Ok(Palette {
            name,
            template,
            color_count,
            background_color_count,
            palette_colors,
        })
    }

    fn save_to_helix_themes(&'a self) -> anyhow::Result<()> {
        let theme = self.to_helix_theme();
        std::fs::write(
            &format!("/Users/silas/.config/helix/themes/{}.toml", self.name),
            theme,
        )?;
        Ok(())
    }

    fn to_helix_theme(&'a self) -> String {
        let mut theme = self.template.to_string();
        for (i, color) in self.palette_colors.iter().enumerate() {
            theme = theme.replace(
                &format!("{{{{color{i}}}}}"),
                &format!("#{:x}", color.color.srgb),
            );
        }
        theme
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
    while i < 100 {
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

fn index_to_char(i: usize) -> char {
    if i < 10 {
        char::from_u32(48 + i as u32).unwrap()
    } else if i < 36 {
        char::from_u32(87 + i as u32).unwrap()
    } else {
        char::from_u32(65 + i as u32).unwrap()
    }
}

fn char_to_index(c: char) -> usize {
    let num = c as u32;
    (if num < 58 {
        num - 48
    } else if num < 91 {
        num - 65
    } else {
        num - 87
    }) as usize
}

fn handle_events_during_freeze_color() -> io::Result<Option<char>> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(c) => return Ok(Some(c)),
                    _ => return Ok(None),
                }
            }
        }
    }
    Ok(None)
}

fn freeze_color_ui(frame: &mut Frame, palette: &mut Palette) {
    let constraints: Vec<Constraint> = palette
        .palette_colors
        .iter()
        .map(|_| Constraint::Length(1))
        .collect();
    let areas = Layout::new(Direction::Vertical, constraints).split(frame.size());
    for (i, color) in palette
        .palette_colors
        .iter()
        .filter(|c| !c.locked)
        .enumerate()
    {
        let r_color = ratatui::style::Color::Rgb(
            color.color.srgb.red,
            color.color.srgb.green,
            color.color.srgb.blue,
        );
        let char = index_to_char(i);
        frame.render_widget(
            Paragraph::new(format!("{char}")).style(Style::new().bg(r_color)),
            areas[i],
        );
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut rdr = csv::Reader::from_path("data/colors.csv")?;
    let colors = rdr
        .deserialize()
        .collect::<Result<Vec<ValidColorItem>, csv::Error>>()?;
    let colors = colors
        .into_iter()
        .map(|c| c.try_into())
        .collect::<anyhow::Result<Vec<Color>>>()?;

    let mut palette = Palette::generate_random_palette(args.template, args.color_count, args.background_color_count, &colors)?;
    loop {
        palette.save_to_helix_themes()?;

        let child = Command::new("./scripts/expect.sh")
            .arg(&palette.name)
            .stderr(Stdio::piped())
            .spawn()?;

        let out = child.wait_with_output()?;
        let out = String::from_utf8(out.stderr)?;
        if out.is_empty() {
            anyhow::bail!("bad output");
        } else if out.contains("d") {
            break;
        } else if out.contains("f") {
            enable_raw_mode()?;
            stdout().execute(EnterAlternateScreen)?;
            let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

            let mut key = None;
            while key.is_none() {
                terminal.draw(|frame| freeze_color_ui(frame, &mut palette))?;
                key = handle_events_during_freeze_color()?;
            }
            let index_to_freeze = char_to_index(key.unwrap());
            let mut mut_palette_colors: Vec<&mut MaybeLockedColor<'_>> = palette
                .palette_colors
                .iter_mut()
                .filter(|c| !c.locked)
                .collect();
            mut_palette_colors[index_to_freeze].locked = true;

            disable_raw_mode()?;
            stdout().execute(LeaveAlternateScreen)?;
        }
        palette.permute_non_frozen_colors(&colors)?;
    }

    Ok(())
}
