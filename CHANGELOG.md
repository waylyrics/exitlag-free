# Changelog

All notable changes to this project will be documented in this file.

## [0.3.18] - 2024-11-10

### 🚀 Features

- Set cookies for QQMusicApi in config
- Show errors from providers in search_window
- Support LRCLib provider

### 🐛 Bug Fixes

- Set text wrap to show very-long lyric
- *(test)* Initialize QQMusic
- Add artists to QQMusic keyword
- Search failure dialog cannot be spawned outside GTK thread
- `login_qqmusic` call was not awaited

### 📚 Documentation

- *(readme)* Add alternative `lyrica`
- *(i18n)* Translation for contribution.md and build guide ubuntu (#272)
- Fix typo in filename

### 🧪 Testing

- Search/query via lrclib

### ⚙️ Miscellaneous Tasks

- Support nix flake (#264)
- Migrate to qqmusic-rs 0.2.0
- Apply clippy fix

### Build

- *(deps)* Bump tray-icon from 0.19.0 to 0.19.1 (#270)
- *(deps)* Bump regex from 1.11.0 to 1.11.1 (#269)
- *(deps)* Bump thiserror from 1.0.64 to 1.0.66 (#268)
- *(deps)* Bump glib-macros from 0.20.4 to 0.20.5 (#267)
- *(deps)* Bump the serde group with 2 updates (#266)

### Enhance

- Init lyric provider with any struct

## [0.3.17] - 2024-10-06

### 🐛 Bug Fixes

- *(win32)* Crash with Motrix runnig
- *(tray)* Export translated lyrics
- *(tray/unix)* Avoid calling list_players() from tray thread (#262)

### ⚙️ Miscellaneous Tasks

- Release v0.3.17

### Build

- *(deps)* Bump reqwest from 0.12.7 to 0.12.8 (#261)

### I18n

- Create Italian Translation  it_IT (#260)

## [0.3.16] - 2024-09-28

### 🚀 Features

- Show error dialog if search failed

### 🐛 Bug Fixes

- Build error on linux

### 🚜 Refactor

- Dedup function signature

### 📚 Documentation

- *(readme)* Add alternative
- *(readme)* Intro SPlayer [ci skip]
- *(readme)* Desktop support of desktop_lyric, #258
- *(build)* Fix i18n part (#259)

### 🧪 Testing

- Add test for make_lrc_line

### ⚙️ Miscellaneous Tasks

- Release v0.3.16

### Build

- *(fix)* Fix build with MSVC
- *(fix)* ""desktop

### Enhance

- TrackMeta from Metadata will not fail now

### I18n

- Translate tips

## [0.3.15] - 2024-09-17

### 🐛 Bug Fixes

- Sec field in exported lyric

### ⚙️ Miscellaneous Tasks

- Release v0.3.15

### Enhance

- Make import/export pormpt different
- Select output file after generation

## [0.3.14] - 2024-09-17

### 🚀 Features

- Export-lyric

### 🐛 Bug Fixes

- Migrate to window-rs 0.58
- Support new type of HWND inner
- Build with `opencc` disabled
- Build with export-lyric/import-lyric disabled

### 🚜 Refactor

- Migrate to gtk4-rs clone! new style
- Migrate to new-style clone!
- Prefer lazy-binding rather than multiple `cfg` flag
- Unify naming of same component in different mod
- Dedup import-*-lyric

### 📚 Documentation

- Mark issue/workaround for Better Comments
- *(readme)* Fix broken link for DeaDBeeF plugin [ci skip]

### ⚙️ Miscellaneous Tasks

- Bump dependencies
- Disable unused warn
- *(dep)* Bump dependencies
- Release v0.3.14

### Build

- *(deps)* Bump serde_json from 1.0.118 to 1.0.119 in the serde group (#244)
- *(deps)* Bump openssl from 0.10.64 to 0.10.66 (#246)
- *(deps)* Bump serde_json from 1.0.120 to 1.0.121 in the serde group (#247)
- *(deps)* Bump tokio from 1.39.1 to 1.39.2 (#248)
- *(deps)* Bump orhun/git-cliff-action from 3 to 4 (#257)
- *(deps)* Bump rust_decimal from 1.35.0 to 1.36.0 (#256)
- *(deps)* Bump reqwest from 0.12.5 to 0.12.7 (#255)
- *(deps)* Bump gettext-rs from 0.7.0 to 0.7.1 (#254)
- *(deps)* Bump opencc-rust from 1.1.18 to 1.1.19 (#253)
- *(deps)* Bump the serde group with 2 updates (#252)
- Enable export-lyric for win build [ci skip]

### Enhance

- Dedup import-lyric

### I18n

- Update locale for 0.3.14

## [0.3.13] - 2024-06-27

### 🐛 Bug Fixes

- Do not remove original lyric on extracting translated ones
- Missing import statement

### 📚 Documentation

- *(readme)* Add `ncmpcpp` via mpd-mpris

### ⚙️ Miscellaneous Tasks

- Bump dependencies
- Release v0.3.13

## [0.3.12] - 2024-06-20

### 🐛 Bug Fixes

- `extract-translated-lyric` flag is ignored
- Extract lyric lines should compare start_time

### 📚 Documentation

- Fix typo

### ⚙️ Miscellaneous Tasks

- *(log)* Add log for textdomain to use
- Release v0.3.12

## [0.3.11] - 2024-06-06

### 📚 Documentation

- *(install)* User-fiendly version (#240)

### ⚙️ Miscellaneous Tasks

- Release v0.3.11

### Build

- *(deps)* Bump async-channel from 2.2.1 to 2.3.1
- *(deps)* Bump anyhow from 1.0.82 to 1.0.86
- *(deps)* Bump the serde group with 2 updates
- *(deps)* Bump mimalloc from 0.1.41 to 0.1.42
- *(deps)* Bump toml_edit from 0.22.12 to 0.22.13

## [0.3.10] - 2024-05-30

### 🐛 Bug Fixes

- *(log)* I18n bind result will not be logged

### ⚙️ Miscellaneous Tasks

- Release v0.3.10

## [0.3.9] - 2024-05-26

### 🐛 Bug Fixes

- *(test)* Do not test time
- *(windows)* Auto connect to active player

### 📚 Documentation

- *(readme)* Intro musicfox, TUI music player supports SMTC [ci skip]

### ⚙️ Miscellaneous Tasks

- Release v0.3.9

### Build

- Enable import lyric feature for windows

### Enhance

- Workaround for musicfox v4.4.0 [ci skip]

## [0.3.8] - 2024-04-29

### 🚀 Features

- Import local lrc in tray & csd menu

### 📚 Documentation

- *(readme)* Intro waylyrics-sakura-translator [ci skip]

### ⚙️ Miscellaneous Tasks

- Release v0.3.8

### Build

- Do not strip binary by default [ci skip]

### I18n

- Translate 'import lyric' & translated/original lyric

## [0.3.7] - 2024-04-29

### 🚀 Features

- Emit LoadLyricCache signal on lyric cache load

### ⚙️ Miscellaneous Tasks

- Release v0.3.7

## [0.3.6] - 2024-04-29

### 🚀 Features

- *(tray)* Refetch-lyric for windows
- Disable loading local lyric
- Allow LRC line that starts with [xxx] without ':'
- Select local LRC file to import

### 🐛 Bug Fixes

- Missing import
- Disable local lrc hint on `enable-local-lyric=false`
- *(typo)* Import-lrc -> import-lyric

### 🚜 Refactor

- Store dbus connection instead of gapplication
- Change net-test to offline-test
- Inline variable "text"

### 📚 Documentation

- *(install)* Add flatpak installation link
- *(install-en)* Add flatpak installation link

### ⚙️ Miscellaneous Tasks

- Add archlinuxcn in installation guide
- Release v0.3.6

### Build

- Migrate to ksni 0.2.2

## [0.3.5] - 2024-04-25

### 🚀 Features

- Reload lyric from cache
- Emit NewLyricCache signal on lyric cache update

### ⚙️ Miscellaneous Tasks

- Release v0.3.5

### Build

- *(test)* Set profile.test.debug a boolean

## [0.3.4] - 2024-04-25

### ⚙️ Miscellaneous Tasks

- Release v0.3.4

## [0.3.3] - 2024-04-25

### ⚙️ Miscellaneous Tasks

- *(metainfo)* H3 tags are not supported
- *(desktopfile)* Remove action `Launch`
- Release v0.3.3
- *(metainfo)* Fix warnings
- *(metainfo)* Add screenshot of search window
- *(metainfo)* Use v0.3.3 tag

## [0.3.2] - 2024-04-25

### 🐛 Bug Fixes

- *(readme)* Broken link of desktop file
- Enum tuple variant cannot be destrcucted by tuple matching

### ⚙️ Miscellaneous Tasks

- Make extracting values from tuples more readable
- Use for loop to refactor entries appending in imp.vbox
- Use for loop to refactor items appending in ui_section & play_section
- Release v0.3.2

### Log

- *(tray)* Do not flood start_time on smtc

## [0.3.1] - 2024-04-24

### 🚀 Features

- Set search-window ColumnView row padding in themes

### 📚 Documentation

- *(changelog)* Update changelog

### ⚙️ Miscellaneous Tasks

- Release v0.3.1
- Fix test.yml
- Remove microsoft-prod.list
- Fix documents deploy CI

## [0.3.0] - 2024-04-24

### 🐛 Bug Fixes

- *(i18n)* En_US translation
- Set titlebar before set decorated
- *(ci)* New schema path

### 🚜 Refactor

- Pass WeakRef<Window> to register_sync_task

### 📚 Documentation

- *(install)* Update packaging script
- *(changelog)* Update changelog

### ⚙️ Miscellaneous Tasks

- Release v0.3.0

### ◀️ Revert

- "Translate display modes"

### Log

- Mpris position delay
- Set time diff log level to TRACE so we will not flood debug log

## [0.2.21] - 2024-04-23

### 🐛 Bug Fixes

- Stop lyric update when show_lyric_on_pause not set on pause

### 📚 Documentation

- *(changelog)* Update changelog
- *(changelog)* Update changelog

### ⚙️ Miscellaneous Tasks

- Release v0.2.21

## [0.2.20] - 2024-04-21

### 📚 Documentation

- *(changelog)* Update changelog

### ⚙️ Miscellaneous Tasks

- Release v0.2.20

### Build

- Remove git dependencies

## [0.2.19] - 2024-04-17

### 🚀 Features

- Convert zh-hans/zh-hant in fuzzy match with opencc
- Use song_search_detailed in search box to apply aliases

### 🐛 Bug Fixes

- Use '/' as splitte on searchbox creating
- Underscore not showing in display mode menu
- *(i18n)* Load i18n on windows

### 📚 Documentation

- *(changelog)* Update changelog
- *(build)* Gettext-rs on windows cannot builds out-of-box with MSVC

### ⚙️ Miscellaneous Tasks

- Bump dependencies
- Release v0.2.19

### Build

- *(windows)* Add build script
- Enable i18n for msvc build

### Enhance

- Fine-tune fuzzy match factor
- Make fuzzy-match weight more than length based match
- Apply alias for artist name on `netease`

## [0.2.18] - 2024-04-14

### 🚀 Features

- Hide lyric on pause

### 🐛 Bug Fixes

- Set paused as false after resumed to playing

### 📚 Documentation

- *(changelog)* Update changelog

### ⚙️ Miscellaneous Tasks

- Release v0.2.18

### Log

- Print control status at trace level

## [0.2.17] - 2024-04-09

### 🚀 Features

- Restart in tray-icon for windows

### 🐛 Bug Fixes

- Gtk4 freezes on `Window::close` on windows

### 📚 Documentation

- *(readme)* Update missing translation

### ⚙️ Miscellaneous Tasks

- Switch to upstream `tray-icon`
- Release v0.2.17

### Enhance

- Support LastUpdateTime on windows

## [0.2.16] - 2024-04-08

### 🚀 Features

- Initial tray-icon support for windows

### 🐛 Bug Fixes

- Feature gate for unix should be `cfg(unix)`

### 📚 Documentation

- *(readme)* The only compatible player on windows
- *(changelog)* Update changelog

### ⚙️ Miscellaneous Tasks

- *(test)* Cleanup unused imports
- Release v0.2.16

### Build

- Add icon for win32 build
- *(deps)* Bump h2 from 0.4.3 to 0.4.4

## [0.2.15] - 2024-04-03

### 🚀 Features

- Full mouse click-through for windows

### 🐛 Bug Fixes

- Misuse of `windows-rs`
- Windows smtc position

### 📚 Documentation

- *(readme)* Intro windows user directories
- *(changelog)* Update changelog

### ⚙️ Miscellaneous Tasks

- Reset pkgrel after a pkgver bump
- Release v0.2.15

## [0.2.14] - 2024-04-03

### 🚀 Features

- Initial support for Windows SMTC

### 🐛 Bug Fixes

- Wrong import path
- Duplicated import
- Do workarounds for windows-rs bug

### 🚜 Refactor

- Use `bind_shortcut` in switch-passthrough
- Extract `try_sync_track` and `reconnect_player` from `interop::*`
- Rename `list_player_names` to `list_players`
- Define OS-specified helpers in a trait
- Cleanup old code

### 📚 Documentation

- *(changelog)* Update changelog
- *(readme)* Musixfox-go released the required patch [ci skip]
- *(readme)* AutoLyric `C/C++`
- *(install)* Build on windows

### ⚙️ Miscellaneous Tasks

- Example environment is no longer required [ci skip]
- Automatically update AUR PKGBUILD
- Release v0.2.14

### ◀️ Revert

- Vendored -> openssl

### Build

- *(rpm)* Packit is no longer needed [ci skip]
- Reduce binary size
- *(deps)* Bump documented from 0.3.0 to 0.4.0
- *(deps)* Bump actions/checkout from 3 to 4
- *(deps)* Bump orhun/git-cliff-action from 2 to 3
- *(deps)* Bump actions/upload-artifact from 3 to 4
- Make gettext, openssl, journald optional

## [0.2.13] - 2024-03-23

### 🚀 Features

- Fix secondary lyric will not end
- Blacklist players by name/identity

### ⚙️ Miscellaneous Tasks

- Release v0.2.13

### ◀️ Revert

- "Update README.md"

### Build

- *(deps)* Bump mio from 0.8.10 to 0.8.11
- *(dep)* Update reqwest to 0.12

## [Setup] - 2024-03-01

### 🚀 Features

- Select labels by origin/translation in theme
- Add theme no-background [ci skip]

### 🚜 Refactor

- Remove unneeded player_meta helper
- Implement Debug for dyn LyricProvider
- Remove repeated bind-shortcut code
- Do not set display mode manually
- Use builder api in widget setup

### 📚 Documentation

- *(changelog)* Update changelog
- *(readme)* List recommended players in chart [ci skip]
- *(readme)* Complain some bad support [ci skip]
- *(readme)* Intro listen1 [ci skip]

### ⚙️ Miscellaneous Tasks

- Fix missing bracket

### Enhance

- Replace extension with PathBuf::set_extension

## [0.2.12] - 2024-02-25

### 🚀 Features

- Set Priority::HIGH for lyric_scroll

### 📚 Documentation

- *(install)* Show packaging status
- *(build)* Update packaging docs
- *(build)* Fix install command for schema [ci skip]
- *(readme)* Go-musicfox merged fix-position [ci skip]

### ⚙️ Miscellaneous Tasks

- Remove CSS stylelintrc
- Release v0.2.12

## [0.2.11] - 2024-02-23

### 🐛 Bug Fixes

- Restart in tray-icon set click-through
- Use hsla for transparent window background
- Hsl color format
- Show_both mode place origin to above if no translation

### 🚜 Refactor

- Use `clone!` macro rather than calling upgrade/downgrade manually

### 📚 Documentation

- *(changelog)* Update changelog
- *(changelog)* Update changelog
- *(readme)* Remove dead `flutter-netease-music`
- *(changelog)* Update changelog

### ⚡ Performance

- Store default MainContext

### ⚙️ Miscellaneous Tasks

- Disable changelog on tag due to shitty failure [ci skip]
- Bump dependencies
- Release v0.2.11

### Enhance

- Switch-passthrough in tray-icon without restart

## [0.2.10] - 2024-02-21

### 🚀 Features

- Set lyric-display-mode in GSettings
- Split popover menu to UI section and Play section
- Set mouse click-through in GSettings
- *(theme)* New style
- (optional) select labels by translation/origin

### 🚜 Refactor

- Cleanup `set_lyric_with_mode` [ci skip]
- Move logic-related fields to Window::new

### 📚 Documentation

- *(changelog)* Update changelog

### 🎨 Styling

- *(theme)* Format CSS with Prettier and Stylelint

### ⚙️ Miscellaneous Tasks

- Release v0.2.10

### ◀️ Revert

- "feat: (optional) select labels by translation/origin"

## [0.2.9] - 2024-02-21

### 🚀 Features

- Only show set-lyric button when avaliable
- Show icons on tray-icon menu
- [**breaking**] Set lyric align mode on run time

### 📚 Documentation

- *(changelog)* Update changelog

### ⚙️ Miscellaneous Tasks

- Fix changelog ci skipped unexpectedly
- Remove invalid github template
- Clean up theme comments
- Release v0.2.9

### I18n

- Translate length to 时长
- Translate display_mode
- Translate lyric align

## [0.2.8] - 2024-02-20

### 🚀 Features

- Sort search result by fuzzy-match weight
- Show tooltip for search entries
- I18n support

### 🐛 Bug Fixes

- Changelog CI need write permission
- Incomplete i18n in menu

### 🚜 Refactor

- Skip fuzzy-match in it's block

### 📚 Documentation

- *(changelog)* Update changelog
- *(changelog)* Update changelog
- *(readme)* Update readme
- *(changelog)* Update changelog
- *(changelog)* Update changelog
- *(changelog)* Update changelog
- *(build)* Add gettext to dependencies

### 🎨 Styling

- Package_name as a constant [ci-skip]

### ⚙️ Miscellaneous Tasks

- Generate changelogs for master
- Update changelog on each commit
- Do not flood commit logs
- Remove unnessacary use of Arc<dyn LyricProvider>
- Release v0.2.8

### Build

- *(rpm)* Add icon
- Make global theme presets optional
- *(rpm)* Package with i18n files

### I18n

- Add translation for Simplefied Chinese

## [0.2.7] - 2024-02-19

### 🚀 Features

- Restart waylyrics in tray-icon
- Fuzzy-match lyrics with Sorensen-Dice coefficient
- Skip fuzzy-match if we got only title in metadata

### ⚙️ Miscellaneous Tasks

- Release v0.2.7

### Enhance

- Remove unnessacary global variable for PlayerId's
- Migrate to better dice-coefficient lib

### Log

- Log detailed likelihood with trace-level

## [0.2.6] - 2024-02-18

### 🚀 Features

- Set lyric display mode in run time
- Initial tray-icon support
- *(tray-icon)* PlayAction control
- Optionally start tray-icon service

### 🐛 Bug Fixes

- Missing feature gate for re-export in sync

### 🚜 Refactor

- Remove repeated function prefix

### ⚙️ Miscellaneous Tasks

- Release v0.2.6

### Todo

- Impl real tray-icon functions

## [0.2.5] - 2024-02-18

### 🚀 Features

- Generate comments for config

### 🐛 Bug Fixes

- Do not hide below label in set_lyric
- Remove `hide_label_on_empty_text = false` support
- Append comments with `documented`
- Repeated docs appends to commented config.toml

### 🚜 Refactor

- Move search_window to crate::app
- Move config.rs to config/

### 📚 Documentation

- *(config)* Lyric-align accepts CamelCase value

### ⚙️ Miscellaneous Tasks

- Release v0.2.5

## [0.2.4] - 2024-02-17

### 🚀 Features

- Invoke app actions with mpsc::channel
- Invoke ui actions with mpsc::channel
- Set unsupported reason on below label
- Hide default text on idle

### 🐛 Bug Fixes

- Should re-export PlayAction
- Re-export play-action channel
- Do not hide below label in set_lyric
- Remove `hide_label_on_empty_text = false` support
- Do not show lyric on `Stopped` status

### 🚜 Refactor

- Rename AppAction to PlayAction
- Move UI-related actions to crate::app::actions
- Rename to play_action
- Move search_window to crate::app

### 📚 Documentation

- *(contribute)* Introduce Conventional Commit [ci skip]
- *(readme)* Add logo and intro video [ci skip]
- *(readme)* Move chat banners to center block [ci skip]
- *(readme)* Waybar supports all wlroots-based compositors

### 🎨 Styling

- Remove unused Downgrade import

### ⚙️ Miscellaneous Tasks

- *(dep)* Migrate to gtk4-rs 0.8.0
- Release 0.2.4

### Build

- Make tray-icon feature optional
- Rename feature to 'action-event' rather than tray-icon
- Add optional ksni dep

### Log

- Log reveiced action event

### Other

- *(logo)* Make logo not so shining in dark background [ci skip]
- *(logo)* White shade for text and logo

## [0.2.3] - 2024-02-13

### 🚀 Features

- Intro `is_likely_songid` for songid verification
- Set empty label explictly
- Allow to show origin lyric in above
- Add `trans` theme [ci skip]

### 🐛 Bug Fixes

- Set 20ms as default lyric update interval

### 🚜 Refactor

- Rename confusing `match_lyric` to `verify_lyric` [ci skip]
- Apply clippy fix [ci skip]

### 📚 Documentation

- Play with musicfox need position fix patch
- Add `osdlyrics` to alternatives [ci skip]
- Explain more fields in `Config`
- Firefox via Plasma Integration [ci skip]
- `lollypop`, GTK3-based local music player
- Restate outdated doc of `TrackMeta`

### 🧪 Testing

- Add unit tests for QQMusic::init

### ⚙️ Miscellaneous Tasks

- Remove unreachable `title.unwrap_or()` call
- Migrate to dtolnay/rust-toolchain
- Release 0.2.3

## [0.2.2] - 2024-02-10

### 🐛 Bug Fixes

- QQMusic::init should panic on `Err()` rather than `Ok()`

### ⚙️ Miscellaneous Tasks

- Test qqmusic provider initializing
- Release `v0.2.2`

### Build

- Add ability to disable tests require network

## [0.2.1] - 2024-02-08

### 🚜 Refactor

- Make `init_dirs` a public method so we could write tests
- Remove `CONFIG_HOME`
- Impl into_owned for LyricLine
- Setup `QQMusic` with `init()` call
- Rename `get_lyric` to `parse_lyric` [ci skip]

### 📚 Documentation

- Define `lrc_iter` behaviour
- *(install)* Explain build environment variable
- *(install)* Download pre-built executables [ci skip]

### 🧪 Testing

- Test netease lyric get & parse
- Test LRC parsing
- Move unit tests to inside `src/`
- Move out doctest for `get_lrc_path`

### ⚙️ Miscellaneous Tasks

- Remove unused import
- Run real test in CI
- Release 0.2.1

### Breaking

- *(ui)* Drop `SIGUSR` control support

### Build

- Improve test build time

## [0.2.0] - 2024-02-08

### 🚀 Features

- Lyric play
- Custom font family
- Sync with mpris2
- Allow auto hide empty label & code refactor
- #8
- Rpm spec (#31)
- Disconnect from current player if received SIGUSR1, fix #43
- Switch decoration on SIGUSR2, fix #49
- Add new fields to old configuration
- Allow to build libdbus-sys vendored
- Place menubutton at end
- Switch passthrough
- Change project icon
- Switch decoration with shortcut
- Reload theme
- Reload lyric (from cache)
- Switch click-through with Alt-P
- Change project icon
- Change project icon
- Query_lyric for QQMusic
- Init api client
- Init multi provider support
- Weighted multi-source lyric search
- Change project icon
- Change project icon
- Change project icon
- Search songs from QQ音乐
- Append Reload lyric to CSD menu
- Refetch lyric (ignore cache)
- Mimalloc as default allocator
- Support players do not provide TrackID
- Song_id trick for NCM-gtk
- *(icon)* New design
- Try load lyric from disk
- Metadata from LyricHint (for music_file without local lyrics)
- *(build)* Improve RPM group
- Add support for musicfox

### 🐛 Bug Fixes

- Correct variable name typo in lib.rs
- Better Gtk CSD UI
- Clickthrough not working with decoration
- Cannot restore decoration after set invisable
- *(ci)* Gen_config_example was removed
- Translation lyric not showing
- *(ci)* OpenSUSE packaging
- *(ci)* Update smoketest.yml
- Lyrics are not trimed
- *(ci)* Do not try build doc on PR
- Get songmid for lyric query
- XML decode for LRC from QQMusic api
- Set serde default for Triggers
- Return Ok when fetch successfully
- Refetch lyric should ignore cache
- *(ci)* RPM distros packaging
- *(icon)* Add white background
- Set default on empty lyric status
- Deepsource: anti-pattern use of
- Recorrect player_name part for NCM-gtk4
- Missing white background
- *(icon)* Missing white color
- Make cargo pass dbus check by introducing dbus-dummy
- LyricHint::File from mpris should decode with `url::Url::to_file_path`
- Unconfigured provider from hint will cause lyrics not to be loaded
- Set default lyric_update interval to 50ms
- Skip UTF-8 BOM so lrc-nom will work
- Cannot return value referencing function parameter
- Desktop file does not need a launch action [ci skip]
- Ignore invalid LRC lines
- Override user theme
- Hint support for musicfox

### 🚜 Refactor

- Sync::player
- Interop/lyric-fetch/lyric-scroll
- Replace unneeded field pattern with `..`
- Setup helpers
- Move out update_lyric

### 📚 Documentation

- Fix typo and add schema compilation
- Explain triggers fields
- Mention default shortcuts
- Update outdated requirement
- Note to install pango
- *(readme)* Remove `lx-music` because it's dead [ci skip]

### 🎨 Styling

- *(build)* Use RPM macros to replace hard-coded directories
- Define `glib-macros` under `gtk`

### 🧪 Testing

- Add test for `get_lyric_path`
- Fix get_lrc_path doctesr

### ⚙️ Miscellaneous Tasks

- Migrate to anyhow::Result
- *(ci)* Add smoketest
- *(ci)* Rust caching and weston
- Configurarion for better performance
- Log track_id
- Use default value for missing config fields
- Log PID
- Update dependencies weekly
- Update doc
- Fix broken group link
- Telegram group
- Update doc
- Log cache_path with info level
- Keep example env clear
- Update doc
- Remove useless declaration
- Fix default configuration
- Redesign icon
- 🎵 instead of 🎶
- *(ci)* Build with nightly toolchain
- Remove some unreachable code
- Remove some unreachable code
- Use Cell for T: Copy
- Typing for TrackState
- `extract connect_factory`
- Add debug msg for config and theme path
- *(log)* Also log result weight
- Replace `RefCell` with `Cell`
- Add debug msg for get_cache_path
- Specify revision in Cargo.toml for repreducablily
- Do not encrypt heap allocations
- Add dhat.out.* to .gitignore
- Add mimalloc to dependencies
- Avoid hard coding a must be same value twice
- *(search_window)* Log selected song id
- Introduce 洛雪
- Add .deepsource.toml
- Update .deepsource.toml
- Logging for future spawn
- Update ncm-gtk to upstream
- Remove some outdated log
- Correct legacy field name
- Code cleanup
- Code cleanup
- Apply lint const_block
- Cargo BuildRequires for Fedora
- Use sort_by_key
- Refactor doc directory [ci skip]
- Update description [ci skip]
- Intro openssl/vendored to vendored feature [ci skip]
- *(doc)* Mention desktop file for global shortcut
- Replace `with_borrow_mut` with `set` [ci skip]
- Clean up lyric scroll
- *(doc)* Update alternatives [ci skip]
- *(doc)* Update translation [ci skip]
- Drop support for ncm-gtk legacy name
- *(doc)* Update doc for youtube-music [ci skip]
- *(doc)* Add `musicfox`, a TUI based music player [ci skip]
- Release 0.2.0

### Build

- Let install to make directory
- *(rpm)* Package LICENSE and README
- *(rpm)* Fix install

### Config

- Label alignment

### Enhance

- Adjust layout about switch player
- Direct id support for YesPlayMusic
- Do not copy template themes
- Ship with a generated template is useless
- Save GTK+ CSD state
- Trim lyrics on set
- *(ui)* Set Align::Start for column entries
- *(ui)* Better layout for search_window
- Log to journalctld
- Asynchronously fetch lyric
- *(ui)* Distingush label for whether lyrics were cached
- Apply Mutex<()> lock for update_lyrics
- Album+title for qqmusic
- *(lyric/qqmusic)* Handle -1901 error

### Hotfix

- Disable QQMusic source by default
- Fix mpris player connect

### Log

- Info - player hint
- Log errors on local lyric loading

### Merge

- Remote/master
- Switch-click-through-shortcut

### Search

- Set resizeable for title, singer and album

### Todo

- Query play status and sync on update
- Fix not fetching lyrics
- Impl search & fetch_lyric
- Use an external build system

<!-- generated by git-cliff -->
