# WebAssembly キャンバス描画デモ

このプロジェクトは、RustとWebAssemblyを使用して作成されたインタラクティブなキャンバス描画デモアプリケーションです。

## 機能

- マウスドラッグによる描画機能
- ランダムな色の円を描画
- キャンバスのクリア機能
- スムーズな描画体験（WebAssemblyによる高速な処理）

## 必要条件

- Rust（2021 Edition）
- wasm-pack
- モダンなWebブラウザ（WebAssembly対応）
- Node.js（開発サーバー用）

## セットアップ

1. リポジトリをクローン：
```bash
git clone https://github.com/timeless-residents/handson-wasm-rust.git
cd handson-wasm-rust
```

2. WebAssemblyにビルド：
```bash
wasm-pack build --target web
```

3. ローカルサーバーで実行（例：using Python）：
```bash
python -m http.server
```

## 使い方

1. ブラウザで `http://localhost:8000` を開きます
2. キャンバス上でマウスをクリック＆ドラッグして描画します
3. 描画中は、マウスの動きに合わせてカラフルな円が描かれます
4. 「Clear Canvas」ボタンをクリックしてキャンバスをクリアできます

## プロジェクト構成

- `src/lib.rs` - Rustで実装されたキャンバス操作のコア機能
- `index.html` - WebアプリケーションのUIとJavaScriptコード
- `Cargo.toml` - Rustプロジェクトの依存関係設定

## 技術スタック

- **Rust** - WebAssemblyにコンパイルされるコアロジック
- **wasm-bindgen** - RustとJavaScript間のバインディング
- **web-sys** - WebブラウザのAPIへのアクセス
- **Canvas API** - 描画機能の実装

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。
