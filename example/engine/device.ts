// ----------------------------------------------------
// Device 
// ----------------------------------------------------

const __adapter = await navigator.gpu.requestAdapter()

const __device = await __adapter?.requestDevice()

if(__device === undefined) {

    console.error('no suitable adapter found')

    Deno.exit(1)
}

export const device = __device