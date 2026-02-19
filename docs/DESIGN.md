gb-emulator/
├── Cargo.toml
└── src/
    ├── cpu/
    │   ├── mod.rs
    │   ├── registers.rs
    │   └── instructions/
    │       ├── mod.rs
    │       ├── cb.rs        // CB-prefix instructions
    │       └── normal.rs    // その他の命令
    │
    ├── mmu/
    │   ├── mod.rs
    │   └── mbc/
    │       ├── mod.rs
    │       ├── mbc1.rs
    │       └── none.rs
    │
    ├── ppu/
    │   ├── mod.rs
    │   ├── controller.rs      // LCDステートマシン（Mode 0/1/2/3 & LY管理）
    │   ├── renderer/
    │   │   ├── mod.rs
    │   │   ├── simple.rs      // ライン単位の最小レンダラー（テトリス用）
    │   │   └── pipeline.rs    // 実機に近いFIFOパイプライン実装（後から拡張）
    │   ├── vram.rs            // VRAM/OAM抽象アクセス（将来制限を付与可能）
    │   ├── palette.rs         // 0〜3階調→最終カラーのマッピング
    │   └── frame.rs           // FrameBuffer / LineBuffer（0〜3の内部表現）
    │
    ├── timer/
    │   └── mod.rs
    │
    ├── input/
    │   └── mod.rs
    │
    ├── bus.rs
    ├── rom.rs
    ├── emulator.rs            // CPU/PPU/MMUの統合
    └── main.rs                // CLI or SDLフロントエンド