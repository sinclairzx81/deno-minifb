// ---------------------------------------------------------------------------
// Clean
// ---------------------------------------------------------------------------

export async function clean() {
    await folder(`native/target`).delete().catch(() => {})
    await file('native/Cargo.lock').delete().catch(() => {})
    await folder('lib/native').delete().catch(() => {})
}

// ---------------------------------------------------------------------------
// Build
// ---------------------------------------------------------------------------

export async function build_native() {
    await shell(`cd native && cargo build --release`)
    await file('native/target/release/native.dll').copy('lib/native').catch(() => {})
    await file('native/target/release/libnative.so').copy('lib/native').catch(() => {})
}

// ---------------------------------------------------------------------------
// Start
// ---------------------------------------------------------------------------

export async function start_native() {
    await Promise.all([
        shell(`hammer monitor "native/src native/src/window" "cd native && cargo build --release"`),
        watch('native/target/release/native.dll', async () => {
            await file('native/target/release/native.dll').copy('lib/native').catch(() => {})
            await file('native/target/release/libnative.so').copy('lib/native').catch(() => {})
        })
    ])
}

export async function start_deno() {
    const command = `deno run --unstable --allow-plugin --allow-write mod.ts`
    await shell(`hammer monitor "lib example" "cd example && ${command}"`)
}

export async function start() {
    await build_native()
    await Promise.all([start_native(), start_deno() ])
}

