read -d '\n' template << EndOfText
# This was originally Dracula
# Author : Sebastian Zivota <loewenheim@mailbox.org>
# Author : Chirikumbrah

"annotation"                      = { fg = "foreground"                                                     }

"attribute"                       = { fg = "green",              modifiers = ["italic"]                     }

"comment"                         = { fg = "comment"                                                        }
"comment.block"                   = { fg = "comment"                                                        }
"comment.block.documentation"     = { fg = "comment"                                                        }
"comment.line"                    = { fg = "comment"                                                        }

"constant"                        = { fg = "purple"                                                         }
"constant.builtin"                = { fg = "purple"                                                         }
"constant.builtin.boolean"        = { fg = "purple"                                                         }
"constant.character"              = { fg = "cyan"                                                           }
"constant.character.escape"       = { fg = "pink"                                                           }
"constant.macro"                  = { fg = "purple"                                                         }
"constant.numeric"                = { fg = "purple"                                                         }
"constructor"                     = { fg = "purple"                                                         }

"definition"                      = { underline = { color = "cyan"                                        } }

"diagnostic"                      = { underline = { color = "orange",          style = "curl"             } }
"diagnostic.hint"                 = { underline = { color = "purple",          style = "curl"             } }
"diagnostic.warning"              = { underline = { color = "yellow",          style = "curl"             } }
"diagnostic.error"                = { underline = { color = "red",             style = "curl"             } }
"diagnostic.info"                 = { underline = { color = "cyan",            style = "curl"             } }
"diagnostic.unnecessary"          = { modifiers = ["dim"]                                                   }
"diagnostic.deprecated"           = { modifiers = ["crossed_out"]                                           }

"error"                           = { fg    = "red"                                                         }
"hint"                            = { fg    = "purple"                                                      }
"info"                            = { fg    = "cyan"                                                        }
"warning"                         = { fg    = "yellow"                                                      }

"diff.delta"                      = { fg    = "orange"                                                      }
"diff.minus"                      = { fg    = "red"                                                         }
"diff.plus"                       = { fg    = "green"                                                       }

# d
"function"                        = { fg = "green"                                                          }
"function.builtin"                = { fg = "green"                                                          }
"function.call"                   = { fg = "green"                                                          }
"function.macro"                  = { fg = "purple"                                                         }
"function.method"                 = { fg = "green"                                                          }

"keyword"                         = { fg = "pink"                                                           }
"keyword.control.conditional"     = { fg = "pink"                                                           }
"keyword.control.exception"       = { fg = "purple"                                                         }
"keyword.control.import"          = { fg = "pink"                                                           }
"keyword.control.repeat"          = { fg = "pink"                                                           }
"keyword.directive"               = { fg = "green"                                                          }
"keyword.function"                = { fg = "pink"                                                           }
"keyword.operator"                = { fg = "pink"                                                           }
"keyword.return"                  = { fg = "pink"                                                           }
"keyword.storage"                 = { fg = "pink"                                                           }
"keyword.storage.modifier"        = { fg = "pink"                                                           }
"keyword.storage.type"            = { fg = "cyan",               modifiers = ["italic"]                     }

"label"                           = { fg = "cyan"                                                           }

"markup.bold"                     = { fg    = "orange",          modifiers = ["bold"]                       }
"markup.heading"                  = { fg    = "purple",          modifiers = ["bold"]                       }
"markup.italic"                   = { fg    = "yellow",          modifiers = ["italic"]                     }
"markup.link.text"                = { fg    = "pink"                                                        }
"markup.link.url"                 = { fg    = "cyan"                                                        }
"markup.list"                     = { fg    = "cyan"                                                        }
"markup.quote"                    = { fg    = "yellow",          modifiers = ["italic"]                     }
"markup.raw"                      = { fg    = "foreground"                                                  }
"markup.strikethrough"            = {                            modifiers = ["crossed_out"]                }

"punctuation"                     = { fg = "foreground"                                                     }
"punctuation.bracket"             = { fg = "foreground"                                                     }
"punctuation.delimiter"           = { fg = "foreground"                                                     }
"punctuation.special"             = { fg = "pink"                                                           }

"special"                         = { fg = "pink"                                                           }

"string"                          = { fg = "yellow"                                                         }
"string.regexp"                   = { fg = "red"                                                            }
"string.special"                  = { fg = "orange"                                                         }
"string.symbol"                   = { fg = "yellow"                                                         }

"tag"                             = { fg = "pink"                                                           }
"tag.attribute"                   = { fg = "purple"                                                         }
"tag.delimiter"                   = { fg = "foreground"                                                     }

"type"                            = { fg = "cyan",               modifiers = ["italic"]                     }
"type.builtin"                    = { fg = "cyan"                                                           }
"type.enum.variant"               = { fg = "foreground",         modifiers = ["italic"]                     }

"ui.background"                   = { fg    = "foreground",      bg = "background"                          }
"ui.cursor"                       = { fg    = "background",      bg = "purple",        modifiers = ["dim"]  }
"ui.cursor.insert"                = { fg    = "background",      bg = "green",         modifiers = ["dim"]  }	
"ui.cursor.match"                 = { fg    = "foreground",      bg = "grey"                                }
"ui.cursor.normal"                = { fg    = "background",      bg = "purple",        modifiers = ["dim"]  }	
"ui.cursor.primary.insert"        = { fg    = "background",      bg = "green"                               }	
"ui.cursor.primary.normal"        = { fg    = "background",      bg = "purple"                              }	
"ui.cursor.primary.select"        = { fg    = "background",      bg = "cyan"                                }
"ui.cursor.select"                = { fg    = "background",      bg = "cyan",          modifiers = ["dim"]  }
"ui.cursorline.primary"           = {                            bg = "cursorline"                          }
"ui.debug"                        = { fg    = "red"                                                         }
"ui.help"                         = { fg    = "foreground",      bg = "black"                               }
"ui.highlight.frameline"          = { fg    = "background",      bg = "red"                                 }
"ui.linenr"                       = { fg    = "comment"                                                     }
"ui.linenr.selected"              = { fg    = "foreground"                                                  }
"ui.menu"                         = { fg    = "foreground",      bg = "current_line"                        }
"ui.menu.scroll"                  = { fg    = "foreground",      bg = "current_line"                        }
"ui.menu.selected"                = { fg    = "current_line",    bg = "purple",        modifiers = ["dim"]  }
"ui.popup"                        = { fg    = "foreground",      bg = "black"                               }
"ui.selection"                    = {                            bg = "selection"                           }
"ui.selection.primary"            = {                            bg = "current_line"                        }
"ui.statusline"                   = { fg    = "foreground",      bg = "darker"                              }
"ui.statusline.inactive"          = { fg    = "comment",         bg = "darker"                              }
"ui.statusline.insert"            = { fg    = "black",           bg = "green",         modifiers = ["bold"] }
"ui.statusline.normal"            = { fg    = "black",           bg = "purple",        modifiers = ["bold"] }
"ui.statusline.select"            = { fg    = "black",           bg = "cyan",          modifiers = ["bold"] }
"ui.text"                         = { fg    = "foreground"                                                  }
"ui.text.focus"                   = { fg    = "cyan"                                                        }
"ui.virtual.indent-guide"         = { fg    = "indent"                                                      }
"ui.virtual.inlay-hint"           = { fg    = "cyan"                                                        }
"ui.virtual.inlay-hint.parameter" = { fg    = "cyan",            modifiers = ["italic", "dim"]              }
"ui.virtual.inlay-hint.type"      = { fg    = "cyan",            modifiers = ["italic", "dim"]              }
"ui.virtual.jump-label"           = { fg    = "pink",            modifiers = ["bold"]                       }
"ui.virtual.ruler"                = { bg    = "black"                                                       }
"ui.virtual.whitespace"           = { fg    = "whitespace"                                                  }
"ui.virtual.wrap"                 = { fg    = "current_line"                                                }
"ui.window"                       = { fg    = "foreground"                                                  }

"variable"                        = { fg = "foreground"                                                     }
"variable.builtin"                = { fg = "purple",             modifiers = ["italic"]                     }
"variable.other"                  = { fg = "foreground"                                                     }
"variable.other.member"           = { fg = "foreground"                                                     }
"variable.parameter"              = { fg = "orange",             modifiers = ["italic"]                     }


[palette]
background        = "{{color0}}"
purple            = "{{color1}}" 
darker            = "{{color2}}"
black             = "{{color3}}"
comment           = "{{color4}}"
current_line      = "{{color5}}"
cursorline        = "{{color6}}"
cyan              = "{{color7}}"
foreground        = "{{color8}}"
green             = "{{color9}}" 
grey              = "{{color10}}"
indent            = "{{color11}}"
orange            = "{{color12}}" 
pink              = "{{color13}}"
red               = "{{color14}}"
selection         = "{{color15}}"
whitespace        = "{{color16}}"
yellow            = "{{color17}}" 
EndOfText

clear
cargo run --release -- --template "$template" --color-count 18 --background-color-count 1
