# TODO

現在の実装で残っている、Odin 版との差分

## 次の作業候補

- `frame` の描画前後における matrix と style の push/pop
- `init_pipeline` の移植と alpha / additive pipeline の利用
- `data` と `set_data`
- `PREV_SIZE` を使ったウィンドウサイズ変更への対応
- フルスクリーン設定の反映

## 入力

- キー入力の状態を参照する API
- マウス位置とスクロール量の getter
- イベントコールバックから描画状態を変更する場合の Mutex の扱い

## 保留

- `CCState` の保持方法の整理
- テキスト描画のフォント拡張
- 一時アロケータの解放
