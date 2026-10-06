# Notices

FriesXLR is an independent, open-source control application for TC-Helicon GoXLR
and GoXLR Mini hardware. It is not affiliated with, endorsed by, or supported by
TC-Helicon or Music Tribe.

## GoXLR Utility

Hardware knowledge was studied from the MIT-licensed GoXLR Utility project:

https://github.com/GoXLR-on-Linux/goxlr-utility

The following small protocol facts and Windows driver integration details are
adapted from GoXLR Utility and are attributed under the MIT license:

- TC-Helicon vendor/product IDs for GoXLR and GoXLR Mini.
- Device model mapping from USB product ID.
- Windows TUSBAUDIO driver API names used for read-only enumeration.
- The registry CLSID and default driver DLL path used to locate the official
  GoXLR Windows audio API.
- GoXLR command framing, command-index reset behaviour, hardware-info command
  identifiers, and `GetButtonStates` parsing.
- `GetButtonStates` fader volume bytes, encoder bytes, and fader mute-button
  pressed bits.
- TUSBAUDIO input notification function names and the small notification
  payload shapes used to decide when button or fader state should be reread.
- Fader assignment, mute function, and mute-state enum ordering as exposed by
  GoXLR Utility profile and IPC state.

GoXLR Utility exposes fader assignment and fader mute behaviour from its active
profile state. FriesXLR's current direct physical session does not fabricate
those values from hardware; unsupported fields remain unknown until FriesXLR has
a verified profile/state source.

Original project copyright:

Copyright (c) Nathan Adams, Craig McLure, Lars Muhlbauer, and GoXLR Utility
contributors.

GoXLR Utility is licensed under the MIT license. A copy of the MIT license is
included in this repository as [LICENSE](LICENSE).
