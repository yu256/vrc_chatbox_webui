# vrc_chatbox_webui

## 概要

このソフトウェアを使用すると、ブラウザからOSC（Open Sound Control）経由でテキストチャットをVRChatに流すことができます。例えば、VRChatを放置している際に入ってきたフレンドにわかるように「ごはん」「おふろ」といったメッセージを表示し続けるようなことが可能です。

## 使用方法

    事前にExpression MenuからOSCを有効化しておく必要があります。

1. このリポジトリをクローンしてビルドするか、[Release ページ](https://github.com/yu256/vrc_chatbox_webui/releases) から最新のリリースをダウンロードします。
2. 解凍したフォルダ内のvrc_chatbox_webui.exeを実行します。
3. デフォルトでポート80が開かれます。ポートが競合する場合は、コマンドライン引数を使用して別のポートを指定できます。
例: `./vrc_chatbox_webui.exe 3000`
4. 起動後、コンソールに表示されたURLにアクセスします。ローカルIPのため同じLAN内でのみアクセスできます。外部からアクセスしたい場合はngrokやCloudflare Tunnelなどを使用して外部に公開することができます。
