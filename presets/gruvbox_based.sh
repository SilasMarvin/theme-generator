read -d '\n' template << EndOfText
# This was originally the gruvbox theme
# See github.com/morhetz/gruvbox

"attribute" = "aqua1"
"keyword" = { fg = "red1" }
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
"constructor" = { fg = "purple1", modifiers = ["bold"] }
"function" = { fg = "green1", modifiers = ["bold"] }
"function.macro" = "aqua1"
"function.builtin" = "yellow1"
"tag" = "red1"
"comment" = { fg = "gray1", modifiers = ["italic"]  }
"constant" = { fg = "purple1" }
"constant.builtin" = { fg = "purple1", modifiers = ["bold"] }
"string" = "green1"
"constant.numeric" = "purple1"
"constant.character.escape" = { fg = "fg2", modifiers = ["bold"] }
"label" = "aqua1"
"module" = "aqua1"

"diff.plus" = "green1"
"diff.delta" = "orange1"
"diff.minus" = "red1"

"warning" = "orange1"
"error" = "red1"
"info" = "aqua1"
"hint" = "blue1"

"ui.background" = { bg = "bg0" }
"ui.linenr" = { fg = "rcolor0" }
"ui.linenr.selected" = { fg = "yellow1" }
"ui.cursorline" = { bg = "bg1" }
"ui.statusline" = { fg = "fg1", bg = "bg2" }
"ui.statusline.normal" = { fg = "fg1", bg = "bg2" }
"ui.statusline.insert" = { fg = "fg1", bg = "blue0" }
"ui.statusline.select" = { fg = "fg1", bg = "orange0" }
"ui.statusline.inactive" = { fg = "fg4", bg = "bg1" }
"ui.bufferline" = { fg = "fg1", bg = "bg1" }
"ui.bufferline.active" = { fg = "bg0", bg = "yellow0" }
"ui.bufferline.background" = { bg = "bg2" }
"ui.popup" = { bg = "bg1" }
"ui.window" = { bg = "bg1" }
"ui.help" = { bg = "bg1", fg = "fg1" }
"ui.text" = { fg = "fg1" }
"ui.text.focus" = { fg = "fg1" }
"ui.selection" = { bg = "bg2" }
"ui.selection.primary" = { bg = "bg3" }
"ui.cursor.primary" = { bg = "fg4", fg = "bg1" }
"ui.cursor.match" = { bg = "bg3" }
"ui.menu" = { fg = "fg1", bg = "bg2" }
"ui.menu.selected" = { fg = "bg2", bg = "blue1", modifiers = ["bold"] }
"ui.virtual.whitespace" = "bg2"
"ui.virtual.ruler" = { bg = "bg1" }
"ui.virtual.inlay-hint" = { fg = "gray1" }
"ui.virtual.wrap" = { fg = "bg2" }
"ui.virtual.jump-label" = { fg = "purple0", modifiers = ["bold"] }

"diagnostic.warning" = { underline = { color = "orange1", style = "curl" } }
"diagnostic.error" = { underline = { color = "red1", style = "curl" } }
"diagnostic.info" = { underline = { color = "aqua1", style = "curl" } }
"diagnostic.hint" = { underline = { color = "blue1", style = "curl" } }
"diagnostic.unnecessary" = { modifiers = ["dim"] }
"diagnostic.deprecated" = { modifiers = ["crossed_out"] }

"markup.heading" = "aqua1"
"markup.bold" = { modifiers = ["bold"] }
"markup.italic" = { modifiers = ["italic"] }
"markup.strikethrough" = { modifiers = ["crossed_out"] }
"markup.link.url" = { fg = "green1", modifiers = ["underlined"] }
"markup.link.text" = "red1"
"markup.raw" = "red1"

[palette]
bg0 = "{{color0}}"
bg1 = "{{color1}}"
bg2 = "{{color2}}"
bg3 = "{{color3}}"

rcolor0 = "{{color4}}"

fg0 = "{{color5}}"
fg1 = "{{color6}}"
fg2 = "{{color7}}"
fg3 = "{{color8}}"
fg4 = "{{color9}}"

gray1 = "{{color10}}"

red0 = "{{color11}}"
red1 = "{{color12}}"
green0 = "{{color13}}"
green1 = "{{color14}}"
yellow0 = "{{color15}}"
yellow1 = "{{color16}}"
blue0 = "{{color17}}"
blue1 = "{{color18}}"
purple0 = "{{color19}}"
purple1 = "{{color20}}"
aqua0 = "{{color21}}"
aqua1 = "{{color22}}"
orange0 = "{{color23}}"
orange1 = "{{color24}}"
EndOfText


cargo run --release -- --template "$template" --color-count 25 --background-color-count 4
