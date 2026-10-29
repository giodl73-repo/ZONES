"""Build ZONES's bounded browser workbench from its native Rust core."""
import hashlib
import argparse
import json
import shutil
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def run(*args):
    subprocess.run(args, cwd=ROOT, check=True)


def main():
    def capture(*args):
        return subprocess.check_output(args, cwd=ROOT, text=True, encoding="utf-8").strip()
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=Path("dist/ZONES"))
    args = parser.parse_args()
    output = args.output.resolve()
    if output == ROOT / "dist" or ROOT / "dist" not in output.parents:
        raise ValueError("Output must be a directory inside dist")
    version = subprocess.check_output(["wasm-bindgen", "--version"], text=True).strip()
    if version != "wasm-bindgen 0.2.127":
        raise ValueError(f"Expected wasm-bindgen 0.2.127, found {version}")
    cache=ROOT/'.cache';cache.mkdir(exist_ok=True)
    legacy=json.loads((ROOT/'fixtures/pre-evidence-legacy.json').read_text(encoding='utf-8'))['queries']
    current=[json.loads(line) for line in capture('cargo','run','--locked','--quiet','-p','zones-web','--example','legacy_reference').splitlines()]
    if current!=legacy:raise ValueError('Synthetic canonical bytes changed')
    commands={'manifest':['source-report'],'policy':['source-gate-report'],'references':['source-ref-report','data/plan-inputs/us-county-baseline-seed.json'],'assignments':['county-assignment-report','data/legal-assignments/us-county-seed-current-law.json'],'geometry':['geometry-reconciliation-report'],'points':['representative-point-report','data/representative-points/us-county-seed-gazetteer.json'],'context':['rplan-context-report','data/rplan-contexts/us-county-seed-rplan-context.json']}
    refs=[]
    for lens,command in commands.items():
        path=f'fixtures/evidence-{lens}.json';canonical=capture('cargo','run','--locked','--quiet','-p','zones-web','--bin','zones-evidence','--',path)
        native=capture('cargo','run','--locked','--quiet','-p','zones-cli','--',*command)
        if json.loads(canonical)['native_report_canonical']!=native:raise ValueError('Original CLI bytes changed: '+lens)
        refs.append({'input':(ROOT/path).read_text(encoding='utf-8'),'canonical':canonical,'native_report_canonical':native})
    (cache/'evidence-references.json').write_text(json.dumps({'evidence':refs,'legacy':legacy}),encoding='utf-8')
    run("cargo", "build", "--locked", "--release", "--target", "wasm32-unknown-unknown", "-p", "zones-web", "--lib", "--features", "wasm")
    metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--no-deps", "--format-version", "1"], cwd=ROOT))
    wasm = Path(metadata["target_directory"]) / "wasm32-unknown-unknown/release/zones_web.wasm"
    output.mkdir(parents=True, exist_ok=True)
    run("wasm-bindgen", str(wasm), "--target", "web", "--out-dir", str(output / "pkg"))
    for name in ("index.html", "style.css", "app.js", "worker.js", "evidence.js", "evidence-worker.js"):
        shutil.copy2(ROOT / "web" / name, output / name)
    shutil.copy2(ROOT / "LICENSE", output / "LICENSE")
    shutil.copy2(ROOT / "THIRD-PARTY-LICENSES.txt", output / "THIRD-PARTY-LICENSES.txt")
    inputs={'manifest':'source-manifests/us-foundation.json','policy':'source-gates/us-foundation-source-gate.json','seed-plan':'plan-inputs/us-county-baseline-seed.json','assignments':'legal-assignments/us-county-seed-current-law.json','geometry':'geometry-reconciliation/us-county-seed-dot-reconciliation.json','points':'representative-points/us-county-seed-gazetteer.json','context':'rplan-contexts/us-county-seed-rplan-context.json'}
    custody=[];(output/'sources').mkdir(exist_ok=True)
    for key,path in inputs.items():
        source='data/'+path;blob=subprocess.check_output(['git','show','HEAD:'+source],cwd=ROOT)
        if (ROOT/source).read_bytes().replace(b'\r\n',b'\n')!=blob:raise ValueError('Frozen input differs from committed bytes: '+source)
        target='sources/'+key+'.json';(output/target).write_bytes(blob);custody.append({'key':key,'source':source,'published':target,'bytes':len(blob),'sha256':hashlib.sha256(blob).hexdigest(),'byte_basis':'committed LF Git blob; not primary publisher bytes'})
    (output/'source-custody.json').write_text(json.dumps(custody,indent=2),encoding='utf-8');shutil.copy2(ROOT/'docs/source-evidence.md',output/'source-note.md')
    provenance = {"commit": capture("git", "rev-parse", "HEAD"), "dirty": bool(capture("git", "status", "--porcelain")), "rust": capture("rustc", "--version"), "bindings": version, "source": "https://github.com/giodl73-repo/ZONES"}
    (output / "provenance.json").write_text(json.dumps(provenance, indent=2), encoding="utf-8")
    (output / ".nojekyll").touch()
    size = sum(path.stat().st_size for path in output.rglob("*") if path.is_file())
    if size >= 5_000_000:
        raise ValueError("Planner exceeds its 5 MB delivery budget")
    print(f"Pages planner: {size:,} bytes at {output}")


if __name__ == "__main__":
    main()
