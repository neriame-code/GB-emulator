# モジュール別 詳細 TODO

---

# 1️ CPU モジュール（cpu/）
LR35902 CPU の完全実装。  
エミュレーションの最重要部。

## ▼ 1.1 構造定義
- [x] `Registers` 構造体の定義  
- [x] `Flags`（Z/N/H/C）のビット操作  
- [x] 16bit レジスタペア（AF/BC/DE/HL）の get/set  

## ▼ 1.2 命令デコードの枠組み
- [x] 各 opcode のテーブルを定義（0x00〜0xFF）  
- [x] CB prefix 用テーブル（0x00〜0xFF）  

## ▼ 1.3 命令実装（normal 命令）
- [x] 8bit LD 系  
- [x] 16bit LD 系  
- [x] INC/DEC  
- [x] ADD/ADC/SUB/SBC  
- [x] AND/OR/XOR/CP  
- [x] JP/JR  
- [x] CALL/RET  
- [x] PUSH/POP  
- [x] RST  
- [x] ローテート  
- [x] BCD補正・ビット反転・フラグ操作  
- [x] CPU制御  

## ▼ 1.4 CB-prefix 命令
- [ ] RLC/RRC/RL/RR  
- [ ] SLA/SRA/SRL/SWAP  
- [ ] BIT/SET/RES  

## ▼ 1.5 割り込みシステム
- [ ] IF/IE レジスタ  
- [ ] 割り込み優先順位  
- [ ] HALT  
- [ ] STOP（後回しでも可）  

## ▼ 1.6 タイミング / T-cycle
- [ ] 各命令で「経過サイクル数」を返す  
- [ ] HALT時の特殊挙動を実装  
- [ ] WAIT状態の調整（後回し可）  

## ▼ 1.7 テスト
- [ ] CPU log を出力  
- [ ] blargg’s test（cpu_instrs）に対応  
- [ ] 簡単な self test ROM の動作確認  

---

# 2️ MMU モジュール（mmu/）
メモリマップとバンク切替処理。

## ▼ 2.1 メモリマップ
- [ ] ROM（0x0000〜0x7FFF）  
- [ ] VRAM（0x8000〜0x9FFF）  
- [ ] External RAM（0xA000〜0xBFFF）  
- [ ] WRAM（0xC000〜0xDFFF）  
- [ ] Echo RAM（0xE000〜0xFDFF）  
- [ ] OAM（0xFE00〜0xFE9F）  
- [ ] I/O（0xFF00〜0xFF7F）  
- [ ] HRAM（0xFF80〜0xFFFE）  
- [ ] IE（0xFFFF）  

## ▼ 2.2 MBC（mbc1.rs）
- [ ] ROMバンク番号  
- [ ] RAMバンク番号  
- [ ] RAM enable/disable  
- [ ] バンク切替の読み書き  
- [ ] テトリスの ROM が動くことの確認  

## ▼ 2.3 アクセス抽象化
- [ ] PPU向けの VRAM accessor  
- [ ] OAM accessor  
- [ ] 後で Mode 2/3 時のアクセス制限をつけられるようにする  

---

# 3️ PPU モジュール（ppu/）
PPU（三層構造：Controller / Renderer / Frame）の実装。

---

# 3.1 Controller（controller.rs）
LCD の状態管理とタイミング制御。

## ▼ 3.1.1 基本レジスタ
- [ ] LCDC, STAT, SCX, SCY, LY, LYC  
- [ ] BGP, OBP0, OBP1  
- [ ] WY/WX（後で使用）  

## ▼ 3.1.2 モード遷移
- [ ] Mode 0（HBlank）  
- [ ] Mode 1（VBlank）  
- [ ] Mode 2（OAM Search）  
- [ ] Mode 3（Pixel Transfer）  
- [ ] 456 cycles / line  
- [ ] LY 0〜153 増加  

## ▼ 3.1.3 割り込み
- [ ] VBlank割り込み  
- [ ] STAT割り込み（優先度低）  

---

# 3.2 FrameBuffer / LineBuffer（frame.rs）
低レイヤー内部表現。

## ▼ 3.2.1 内部表現
- [ ] ColorIndex = u8（0〜3）  
- [ ] LineBuffer: [ColorIndex; 160]  
- [ ] FrameBuffer: [[ColorIndex; 160]; 144]  

## ▼ 3.2.2 RGBA 変換（SDL用）
- [ ] 0〜3 → グレースケールRGBA  
- [ ] palette.rs で一元化  

---

# 3.3 Renderer（renderer/simple.rs）
テトリス描画に必要な最小の BG レンダラー。

## ▼ 3.3.1 タイル読み込み
- [ ] 0x8000〜 の 16バイトタイル構造  
- [ ] bitplane（2bit）読み込み処理  

## ▼ 3.3.2 背景マップ
- [ ] BGタイル番号（0x9800 or 0x9C00）  
- [ ] LCDC に応じたマップ位置  
- [ ] SCX/SCY によるオフセット  

## ▼ 3.3.3 1ライン描画
- [ ] LY を元に BG の対象ラインを計算  
- [ ] タイルの横並び読み込み  
- [ ] 1ライン（160 px）の出力  

## ▼ 3.3.4 拡張性の確保
- [ ] SimpleRenderer と PipelineRenderer は Renderer trait を共有  
- [ ] Controller は Renderer を知らない（DI方式）  
- [ ] 後でスプライトを差し込みやすい構造  

---

# 3.4 Renderer（renderer/pipeline.rs）
実機に近い FIFO + Fetcher 描画（後で作るもの）

## ▼ 3.4.1 フェッチャー
- [ ] Tile ID → Tile Data → FIFO  
- [ ] スクロールタイミングの実装  

## ▼ 3.4.2 スプライト合成
- [ ] OAM 走査（最大10個）  
- [ ] プライオリティと透明色  
- [ ] 背景との合成  

---

# 4️ Timer モジュール（timer/）
DIV/TIMA の実装。

## ▼ TODO
- [ ] DIV レジスタの更新（16kHz）  
- [ ] TIMA の周波数選択  
- [ ] TAC のビット判定  
- [ ] オーバーフロー → 割り込み  
- [ ] HALT中の DIV の挙動（余力あれば）  

---

# 5️ Input モジュール（input/）
GBのJoypad仕様の実装。

## ▼ TODO
- [ ] 0xFF00（P1）レジスタの読み書き  
- [ ] 方向キーグループ  
- [ ] ボタングループ  
- [ ] SDL との入力マッピング  
- [ ] 割り込みの発火（キー押下）  

---

# 6️ エミュレーションループ（emulator.rs）
全コンポーネントの統合。

## ▼ TODO
- [ ] 1命令を実行 → 返却された cycle を PPU/Timer に渡す  
- [ ] 70224 cycle（1 frame）ごとに FrameBuffer を出力  
- [ ] 入力/割り込みの処理  
- [ ] PPU の 1ライン更新  
- [ ] PPU の Renderer 呼び出し  

---

# 7️ SDL描画（main.rs）
## ▼ TODO
- [ ] SDL window の作成  
- [ ] FrameBuffer（0〜3）を RGBA に変換  
- [ ] テクスチャへの書き込み  
- [ ] キー入力読み取り → Joypadへ  

---

# 最終対象：テトリス起動
## ▼ 条件
- [ ] CPU が正確に動作  
- [ ] MBC1 の ROM バンクが正しく選択  
- [ ] PPU Controller が正しいタイミング  
- [ ] SimpleRenderer で BG が描画  
- [ ] Timer が正常で速度が合う  
- [ ] Joypad が動作  
- [ ] SDL 表示で画面が見える  

ここまでで **テトリスは確実に動作** する。

---

# 追加要素（後からやる）
- [ ] Window  
- [ ] スプライト描画（OBJ）  
- [ ] FIFOフェッチパイプライン  
- [ ] VRAM/OAMアクセス制限  
- [ ] STAT割り込み  
- [ ] APU（音）  
- [ ] Debugger（CPU step, breakpoint）  