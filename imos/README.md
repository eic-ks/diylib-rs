# imosライブラリ
いもす法のライブラリ

## 使い方

```toml
# Cargo.toml
[dependencies]
Imos = {git = "https://github.com/eic-ks/diylib-rs.git", package="Imos"}

```rust
use Imos::Ims;
let mut ims = Ims::new(7); //[0,0,0,0,0,0,0]
ims.query((1,5,2)); // [2,0,0,0,0,-2,0]
assert!(ims.result() == &[2,2,2,2,2,0,0]);

## 注意
・mutでインスタンスの作成をしてください
・ims.resultは最後に一回だけ使ってください