@echo off

cargo build --release || exit 1
copy target\riscv64-kartoffel-bot\release\kartoffel-bot kartoffel || exit 1

if exist kartoffel.b64 (
    del kartoffel.b64
)

if "%1" == "--copy" (
    certutil -encodehex kartoffel kartoffel.b64 0x40000001 1>nul & type kartoffel.b64 | clip
)

if "%1" == "--upload" (
    if "%2" == "" (
        echo err: missing session id
        echo:
        echo usage:
        echo     ./build --upload ^<sessionId^>
        echo:
        echo e.g.:
        echo     ./build --upload 1234-1234-1234-1234
        exit 1
    )

    curl ^
        -f ^
        -X POST ^
        -T kartoffel ^
        "https://kartoffels.pwy.io/api/sessions/%2/bots" || exit 1

    echo ok, bot uploaded
)
