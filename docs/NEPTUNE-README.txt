Neptune (VIctory Edit.) - build of 2026-11-25, evening
=======================================================

A modified build of the "VIctory" fork (https://github.com/fxceless/victory-fork), a runtime
that runs an Xbox Series title's own executable on Windows. Work in progress, tested on one
PC only (RTX 5070 Ti 16 GB, i7-14700, 64 GB, Windows 11), Series S mode (720p upscaled to
1080p):

  open world, 6,500 to 9,000 draws a frame      9-11 frames a second, less while the game
                                                streams (looking around, moving)
  inside the house                              about 12
  Series X path (4K)                            not measured again with this build

NO GAME FILES ARE IN THIS PACKAGE. You need your own copy of the game. Shaders are
translated on your PC from your own copy.

What is here
------------
  victory.exe              the runtime (Windows x64)
  play.ps1                 launcher with the switch set this build was tuned with
  neptune-ve-source.zip    the complete source this victory.exe was built from
  LICENSE                  GNU General Public License v3 (or later)
  UPSTREAM-README.md       the upstream fork's readme (build steps, background)

What is not here, and why you want it
-------------------------------------
  Series S mode needs a PC copy of AMD's FidelityFX upscaler DLL
  (amd_fidelityfx_upscaler_dx12.dll): the title's upscaler calls are handed to it. It is
  AMD's and not ours to ship. Any one of these gives you one:
    - an extracted OptiScaler release (https://github.com/optiscaler/OptiScaler/releases):
      pass its folder with -OptiScaler. This is also what -FrameGen uses.
    - a PC game you own that ships the file: pass it with -FsrDll, or copy it beside
      play.ps1.
  Without one, play.ps1 starts the Series X path at 4K (slower, sharper).

Running it
----------
  First time (translates the shaders, about 7 minutes, then starts the game):
      .\play.ps1 -Game 'D:\path\to\title.exe' -OptiScaler 'D:\OptiScaler' -Warm
  After that:
      .\play.ps1 -Game 'D:\path\to\title.exe' -OptiScaler 'D:\OptiScaler'

  Options:
      -FrameGen        EXPERIMENTAL, off unless asked for. OptiScaler is loaded into the
                       process: its upscaler (XeSS; -Upscaler fsr31 or dlss for another) and
                       AMD's frame generation on the finished frames. Standing still it
                       doubled the frames on screen for two real ones lost; moving through
                       the world it made things worse. Insert opens OptiScaler's own menu.
      -SeriesX         the 4K path.
      -FrametimeDump   writes frametime.csv into the cache folder: one row a second with the
                       frame time and where it went, under a header that names the PC. Made
                       to be posted in an issue; it costs no frames. (From skornuta's fork.)
      -NoAutoA         do not tap A through the start screens.

  If PowerShell refuses to run the script:
      powershell -ExecutionPolicy Bypass -File .\play.ps1 -Game '...'

  Needs Windows 11, a current GPU driver, and a card with 16 GB for the switch set as it
  is (see the note at the top of play.ps1 for smaller cards). The cache and saves go to
  %LOCALAPPDATA%\victory.

Controls (keyboard and mouse stand in for the pad)
---------------------------------------------------
  Click the picture or press F9 for mouse look; F9 or Alt+Tab lets the pointer go.
  WASD move, mouse look, right / left mouse = left / right trigger, Shift / Ctrl = stick
  clicks, Enter / Backspace / X / Y = A / B / X / Y, Q / E bumpers, Esc menu, Tab view.
  F10 hides the figures in the corner (frames a second, 30 s high / low / average, draws,
  memory); F12 saves a picture.

Good to know
------------
  - Discord: while the title runs, optional local Discord Rich Presence
    through the local Discord client. Set the environment variable VICTORY_DISCORD=0 to
    turn that off.
  - Light indoors is a stand-in. The game lights rooms with rays it traces, which no build
    of this runtime runs yet; this one puts an even light in their place (set
    VICTORY_AMBIENT=0.4 or another level for less or more, VICTORY_AMBIENT=off for the
    game's own output, which is dark with white haze). Ceilings can stay black and shiny
    surfaces sparkle.
  - The first time a new shader is needed the card's driver compiles it. A frame waits at
    most 2.5 s for that (VICTORY_PIPELINE_WAIT_MS in play.ps1); what needs longer is
    missing from the picture until it is ready. Second visits are smooth.
  - Known issues: objects and clothes can still turn black for a moment while the game
    swaps their textures (less than before, not gone); parts of the world are missing
    behind some doors and on some roofs; facial animation does not play; frame time jumps
    while the game streams.

What changed since the afternoon build
--------------------------------------
  - Light indoors (see above): the ambient pass written by Belobug, as ShakiraRED-dt
    carries it.
  - Upstream's fix for records the game copies into its descriptor heap as bytes (fxceless
    a26859f and 9376fd6, by way of myokex's branch), made cheap enough to keep: it cost
    40 to 70 ms a frame as merged, about 3 now.
  - Series S mode: the texture queue no longer stops for want of room on the card (room
    was kept for 4K render targets that a 720p game never makes).
  - Textures are let go early only when the card really has no room: before, copies not
    used for 2.5 s went as soon as the card was three quarters full, and came back black.
  - A material keeps showing the texture it had until its sharper or smaller replacement
    is uploaded, instead of nothing.
  - A frame that waited 9 to 30 s for shaders to compile (the game's own watchdog ended a
    run on one) now waits 2.5 s at most.
  - The loader no longer waits on destinations it has seen reused unchanged: a third of
    its workers' time.
  - The shader translator is the one of soldsawn/victory-nvidia-8gb (64-lane wave programs
    in a paired form on 32-lane NVIDIA cards): no pipeline is refused any more.

Licence and credits
-------------------
  GPL-3.0-or-later; see LICENSE. The source in neptune-ve-source.zip is the corresponding
  source for victory.exe. It stands on:
    Marvisak/VIctory                  (the original runtime)
    myokex/victory-amd, fxceless/victory-fork        (the base; levels on demand; the
                                       descriptor heap sync, with pull requests by
                                       Jonirulah, Flyander and AscilCH)
    soldsawn/victory-nvidia-8gb       (the shader translator)
    Belobug, reddxd-byte/ShakiraRED-dt  (the ambient light pass)
    ZSlayerHQ/victory-world-runtime   (writes seen across views, kernel reads, dispatch in
                                       parts)
    JoshuuDev/victory-amd             (remapping lock by range; blend and descriptor fixes)
    zm0kie                            (overlay, window uploads, memory lines)
    skornuta/victory-fork             (frametime dump)
    Trexies                           (folded small buffers)
    the FidelityFX redirect patch received for DexNom Runtime
  and the Neptune changes on top (frame time, mouse and keys, Discord).
  OptiScaler and AMD's, Intel's and NVIDIA's libraries are theirs, are not in this package,
  and keep their own licences.
  If you pass this package on, pass on the source zip and the LICENSE with it.
