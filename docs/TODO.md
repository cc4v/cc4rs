# TODO

現在の実装で残っている、Odin 版との差分

## 次に手を入れる候補

- `frame` の描画前後における matrix と style の push/pop
- `init_pipeline` の移植と alpha / additive pipeline の利用
- `data` と `set_data`
- `PREV_SIZE` を使ったウィンドウサイズ変更への対応
- フルスクリーン設定の反映

## 描画 API

`shape.odin` にある以下の API は未移植。

- line
- square
- circle
- 塗りつぶしと枠線の円
- text

## 入力

- キー入力の状態を参照する API
- マウス位置とスクロール量の getter
- イベントコールバックから描画状態を変更する場合の Mutex の扱い

## 保留

- `CCState` の保持方法の整理
- テキスト描画の初期化とフレーム処理
- 一時アロケータの解放
