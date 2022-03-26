(module $runtime
  (type (;0;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;1;) (func (param i32 i32) (result i32)))
  (type (;2;) (func (param f32) (result i32)))
  (type (;3;) (func))
  (import "wasi_snapshot_preview1" "fd_write" (func (;0;) (type 0)))
  (import "wasi_snapshot_preview1" "random_get" (func (;1;) (type 1)))
  (func $add_ab (;2;) (type 1) (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add
    f32.const 0x0p+0 (;=0;)
    call $add_ba
    i32.add
    drop
    loop $for-1@continue ;; label = @1
      block $for-1@break ;; label = @2
        i32.const 0
        i32.const 0
        drop
        drop
        br 0 (;@2;)
      end
    end
    i32.const 0
    return
  )
  (func $add_ba (;3;) (type 2) (param $b f32) (result i32)
    global.get $math.pi
    local.get $b
    f32.add
    i32.trunc_f32_s
  )
  (func $_start (;4;) (type 3))
  (global $math.pi (;0;) (mut f32) f32.const 0x1.91eb86p+1 (;=3.14;))
  (export "add_ab" (func $add_ab))
  (export "add_ba" (func $add_ba))
  (export "_start" (func $_start))
  (@producers
    (language "valkyrie" "2024")
    (language "player" "berserker")
    (processed-by "nyar-wasm" "0.0.0")
  )
)