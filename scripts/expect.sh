#!/usr/bin/expect -f

log_user 0

set stty_init raw

set initial_theme [lindex $argv 0]

spawn ./scripts/spawn_helix.sh

send ":theme $initial_theme\r"

set action ""
interact {
  # Tab
  \011 {
    set action "tab"
    send ":q!\r"
  }
  # Lowercase s
  \163 {
    set action "s"
    send ":q!\r"
  }
  # Lowercase d
  \144 {
    set action "d"
    send ":q!\r"
  }
  # Lowercase f
  \146 {
    set action "f"
    send ":q!\r"
  }
}

send_error $action
