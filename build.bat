@echo off

cargo build --release || exit /b
copy target\riscv64-kartoffel-bot\release\kartoffel-bot kartoffel || exit /b

if exist kartoffel.b64 (
    del kartoffel.b64
)

if "%1" == "--copy" (
    certutil -encodehex kartoffel kartoffel.b64 0x40000001 1>nul & type kartoffel.b64 | clip
)
