#!/bin/bash

# Open the Sound preferences pane
open /System/Library/PreferencePanes/Sound.prefPane

# Use AppleScript to select the output device
/usr/bin/osascript <<END
tell application "System Events"
    tell application "System Preferences"
        set current pane to pane id "com.apple.preference.sound"
        delay 1
    end tell
    delay 1
    tell application process "System Preferences"
        click radio button "Output" of tab group 1 of window "Sound"
        delay 1
        set theRows to rows of table 1 of scroll area 1 of tab group 1 of window "Sound"
        repeat with theRow in theRows
            if value of text field 1 of theRow as text is equal to "Bakrum" then
                select theRow
                delay 1
                click button "Use this device for sound output" of window "Sound"
                exit repeat
            end if
        end repeat
    end tell
    delay 1
    quit application "System Preferences"
end tell
END
