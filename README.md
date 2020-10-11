# 開発ガイド
## Windows

### MSYS2を入れる

### インストール先のパスを通す

C:\msys64\mingw64\bin

他のgccとか入れてる場合は、より上位にしておく

### MSYSで
```
pacman -S mingw-w64-x86_64-toolchain
pacman -S mingw-w64-x86_64-gtk3
pacman -S mingw-w64-x86_64-glade #必要に応じて
pacman -S mingw-w64-x86_64-pkg-config
```
しておく

```
$ pacman -Sl | grep gtk3
mingw32 mingw-w64-i686-gtk3 3.24.22-1
mingw64 mingw-w64-x86_64-gtk3 3.24.22-1
```

### 実行前に

```
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default nightly-x86_64-pc-windows-gnu
```
しておく

### 実行

```
cargo run
```



# その他

- 初回登録画面ができてないので設定ファイルに手動で書き込んでね状態です。
- このアプリと関係ないですが、cargo install cargo-editを入れときましょう
- /mingw64/bin/glade で画面を作成

# 参考リンク

https://www.gtk.org/docs/installations/windows/#using-gtk-from-msys2-packages

http://gtk-rs.org/docs/gtk/