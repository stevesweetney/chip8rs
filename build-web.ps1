# Set ErrorActionPreference to Stop to stop the script if there is an error
$ErrorActionPreference = "Stop"

# Define variables
$target = "wasm32-unknown-unknown"
$outDir = "root"

# Build the Rust project
cargo build --target $target --profile release-small

# Generate bindgen outputs
mkdir -Force $outDir
wasm-bindgen --target web --out-dir $outDir/ target/$target/release-small/chip8-frontend.wasm --no-typescript

# Shim to tie the thing together
$content = (Get-Content "$outDir/chip8-frontend.js") -replace "import \* as __wbg_star0 from 'env';", "" `
    -replace "let wasm;", "let wasm; export const set_wasm = (w) => wasm = w;" `
    -replace "imports\['env'\] = __wbg_star0;", "return imports.wbg;"
$content = $content.Replace("const imports = getImports();", "return getImports();")
$content | Set-Content "$outDir/chip8-frontend.js"
