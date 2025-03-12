# SPDX-FileCopyrightText: 2025 Lukas Heindl
#
# SPDX-License-Identifier: MIT

Set Shell zsh
Set FontSize 20
Set Width 1800
Set Height 1000
Output main.gif

Type@150ms "cargo run -- -d ../data -s scheme2021 -s scheme2025 --op none --op merge-cdu-csu 2025"
Sleep 500ms
Enter
Sleep 15s
