"""Build ZONES's bounded browser workbench from its native Rust core."""
import argparse
import json
import shutil
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def run(*args):
    subprocess.run(args, cwd=ROOT, check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=Path("dist/ZONES"))
    args = parser.parse_args()
    output = args.output.resolve()
    if output == ROOT / "dist" or ROOT / "dist" not in output.parents:
        raise ValueError("Output must be a directory inside dist")
    version = subprocess.check_output(["wasm-bindgen", "--version"], text=True).strip()
    if version != "wasm-bindgen 0.2.127":
        raise ValueError(f"Expected wasm-bindgen 0.2.127, found {version}")
    run("cargo", "build", "--locked", "--release", "--target", "wasm32-unknown-unknown", "-p", "zones-web", "--lib", "--features", "wasm")
    metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--no-deps", "--format-version", "1"], cwd=ROOT))
    wasm = Path(metadata["target_directory"]) / "wasm32-unknown-unknown/release/zones_web.wasm"
    output.mkdir(parents=True, exist_ok=True)
    run("wasm-bindgen", str(wasm), "--target", "web", "--out-dir", str(output / "pkg"))
    for name in ("index.html", "style.css", "app.js", "worker.js"):
        shutil.copy2(ROOT / "web" / name, output / name)
    shutil.copy2(ROOT / "LICENSE", output / "LICENSE")
    shutil.copy2(ROOT / "THIRD-PARTY-LICENSES.txt", output / "THIRD-PARTY-LICENSES.txt")
    def capture(*args):
        return subprocess.check_output(args, cwd=ROOT, text=True, encoding="utf-8").strip()
    provenance = {"commit": capture("git", "rev-parse", "HEAD"), "dirty": bool(capture("git", "status", "--porcelain")), "rust": capture("rustc", "--version"), "bindings": version, "source": "https://github.com/giodl73-repo/ZONES"}
    (output / "provenance.json").write_text(json.dumps(provenance, indent=2), encoding="utf-8")
    (output / ".nojekyll").touch()
    size = sum(path.stat().st_size for path in output.rglob("*") if path.is_file())
    if size >= 5_000_000:
        raise ValueError("Planner exceeds its 5 MB delivery budget")
    print(f"Pages planner: {size:,} bytes at {output}")


if __name__ == "__main__":
    main()
