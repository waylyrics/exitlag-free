# Waylyrics

[![tg-group](https://img.shields.io/badge/tg%20group-open-blue)](https://t.me/+FWgnE0GRDYZhNjc1)
[![matrix-group](https://img.shields.io/matrix/waylyrics_x:catgirl.cloud.svg?server_fqdn=matrix.catgirl.cloud)](https://matrix.to/#/#waylyrics_x:catgirl.cloud)

桌面歌词，基于GTK4，made with ❤

![](https://github.com/poly000/waylyrics/assets/34085039/43037cb4-9a07-4e77-b112-1408365199e2)

- [Waylyrics](#waylyrics)
  - [构建/安装](#构建安装)
  - [用法](#用法)
  - [依赖](#依赖)
    - [推荐的播放器](#推荐的播放器)
      - [在线](#在线)
      - [本地](#本地)
    - [无法使用的播放器](#无法使用的播放器)
  - [目录](#目录)
  - [实现](#实现)
  - [替代品](#替代品)
  - [Credit](#credit)
  - [License](#license)

## 构建/安装

详阅 [INSTALLATION.md](INSTALLATION.md)

## 用法

右上角菜单有快捷键和对应操作如切换链接的播放器。

SIGUSR1: 断开当前播放器

SIGUSR2: 开关GTK CSD

## 依赖

- 播放器需正确支持MPRIS，独一无二的TrackID也是必需的。
- 你的wm需要允许你手动设置窗口置顶

### 推荐的播放器

#### 在线

- [Qcm](https://github.com/hypengw/Qcm)
- [Electron-NCM](https://github.com/Rocket1184/electron-netease-cloud-music)
- [YesPlayMusic](https://github.com/qier222/YesPlayMusic)
- [FeelUOwn](https://github.com/feeluown/FeelUOwn/), 3.9.12+
- [Telegram](https://t.me/Music163Bot)

Qcm, Feeluown-netease, ElectronNCM, YesPlayMusic，可以直接拿歌曲id

#### 本地

- [mpv-mpris](https://github.com/hoyon/mpv-mpris)
- [VLC](https://www.videolan.org)

### 无法使用的播放器

[netease-cloud-music-gtk]: https://github.com/gmg137/netease-cloud-music-gtk
[flutter-netease-music]: https://github.com/boyan01/flutter-netease-music
[youtube-music]: https://github.com/th-ch/youtube-music


| Player                    | OSD | issue              |
| ------------------------- | --- | ------------------ |
| [netease-cloud-music-gtk] | X   | 播放位置固定给0    |
| Firefox                   | X   | 不提供播放位置调用 |
| qqmusic                   | O   | 什么都不给         |
| [flutter-netease-music]   | X   | 完全不支持mpris    |

## 目录

一般情况会创建的目录（可能被用户XDG设置影响）

```
~/.cache/waylyrics/XX/...
~/.config/waylyrics/...
# waylyrics 会首先尝试在这里加载主题，找不到的话就从全局模板目录找
~/.local/share/waylyrics/_themes/...
```

## 实现

目前的实现比较脏：

1. 连接最可能对的那个播放器，只有它没了才会断开
2. 每2s刷新歌曲状态，20ms刷新歌词
3. 如果不能直接拿id，匹配策略是：
    1. 长度匹配的结果优先
    2. 没的话就用搜索的第一个结果
    3. todo: 允许用户手动选择歌词匹配结果（因为推荐直接拿id所以咕咕了，虽然也有Spotify用户存在）

## 替代品

[YesPlayMusicOSD]: https://github.com/shih-liang/YesPlayMusicOSD
[waybar-netease-music-lyrics]: https://github.com/kangxiaoju/waybar-netease-music-lyrics

Sway用户可以看看 [waybar-netease-music-lyrics].

[YesPlayMusicOSD] 也有不错的歌词支持

BruceZhang1993's [lyricsSeeker](https://github.com/BruceZhang1993/LyricsSeeker) is still WIP, but it may have better-looking and better integration with KDE.

Copay's [caraoke-plasmoid](https://github.com/Copay/caraoke-plasmoid) is currently Plasma-only, though it is easy to remove plasmoid components

## Credit

[gtk4-rs]: https://github.com/gtk-rs/gtk4-rs

- [gtk4-rs], Rust bind to GTK-4.


## License

[The MIT License (MIT)](https://raw.githubusercontent.com/waylyrics/waylyrics/master/LICENSE)

This project icon is licensed under a [Creative Commons Attribution 4.0 International License](https://creativecommons.org/licenses/by/4.0/).
