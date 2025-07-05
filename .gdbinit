# GDB initialization for Game Boy Advance debugging
set architecture armv4t
set endian little

# Allow loading of .gdbinit from current directory
set auto-load safe-path /

# Show disassembly when debugging
set disassembly-flavor intel

# GBA Memory map convenience commands
define gba-iwram
    x/256x 0x03000000
end
document gba-iwram
Display Internal Work RAM (IWRAM) - 32KB at 0x03000000
end

define gba-ewram
    x/1024x 0x02000000
end
document gba-ewram
Display External Work RAM (EWRAM) - 256KB at 0x02000000
end

define gba-io
    x/256x 0x04000000
end
document gba-io
Display I/O registers - at 0x04000000
end

define gba-palette
    x/256x 0x05000000
end
document gba-palette
Display Palette RAM - 1KB at 0x05000000
end

define gba-vram
    x/256x 0x06000000
end
document gba-vram
Display Video RAM - 96KB at 0x06000000
end

define gba-oam
    x/256x 0x07000000
end
document gba-oam
Display Object Attribute Memory - 1KB at 0x07000000
end

# Useful aliases
alias r = run
alias c = continue
alias s = step
alias n = next
alias b = break
alias d = delete
alias i = info
alias p = print
alias x = examine

# Display useful info on startup
echo \n=== GBA Debugging Session ===\n
echo Available commands:\n
echo   gba-iwram  - Show Internal Work RAM\n
echo   gba-ewram  - Show External Work RAM\n
echo   gba-io     - Show I/O registers\n
echo   gba-palette- Show Palette RAM\n
echo   gba-vram   - Show Video RAM\n
echo   gba-oam    - Show Object Attribute Memory\n
echo \n