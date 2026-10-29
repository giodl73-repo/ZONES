import {test,expect} from '@playwright/test';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
async function ready(page){await page.goto('/ZONES/');await expect(page.locator('#evidence-status')).toContainText('Evidence ready');}
async function download(page){const [f]=await Promise.all([page.waitForEvent('download'),page.locator('#evidence-download').click()]);return JSON.parse(readFileSync(await f.path(),'utf8'));}

test('actual WASM preserves seven native CLI report bytes and four synthetic canonical results',async({page})=>{
 await ready(page);const refs=JSON.parse(readFileSync('.cache/evidence-references.json','utf8'));
 const actual=await page.evaluate(async refs=>{const m=await import('/ZONES/pkg/zones_web.js');await m.default();return {evidence:refs.evidence.map(r=>m.inspect_evidence_json(r.input)),legacy:refs.legacy.map(r=>m.compare_json(JSON.stringify(r.assignment),r.west,r.east,r.daylight))};},refs);
 expect(actual.evidence).toEqual(refs.evidence.map(r=>r.canonical));expect(actual.legacy).toEqual(refs.legacy.map(r=>r.canonical));for(let i=0;i<7;i++)expect(JSON.parse(actual.evidence[i]).native_report_canonical).toBe(refs.evidence[i].native_report_canonical);
});

test('seven lenses keep structural checks separate from method publication and authority holds',async({page})=>{
 await ready(page);for(const lens of ['manifest','policy','references','assignments','geometry','points','context']){
  await page.locator('#evidence-lens').selectOption(lens);await expect(page.locator('#evidence-status')).toContainText('Evidence ready');const d=await download(page);expect(d.lens).toBe(lens);expect(d.fixed_context).toMatchObject({county_units:4,graph_edges:0,internal_points:4,population_centers:0,strong_claim_point_method_ready:false,policy_coverage_ready:true,reference_fields_complete:true});expect(Object.values(d.authority)).toHaveLength(7);expect(Object.values(d.authority).every(v=>v===false)).toBe(true);expect(Object.values(d.unobserved).every(v=>v===null)).toBe(true);expect(d.custody).toMatchObject({raw_primary_bytes_in_bundle:false,primary_hashes_reverified:false,current_law_revalidated:false,fresh_geometry_overlay:false});await expect(page.locator('#evidence-gate-rows')).toContainText('strong_claim_point_method_ready');await expect(page.locator('#evidence-holds')).toContainText('No county scorecard or candidate map');
 }
 await page.locator('#evidence-lens').selectOption('policy');await expect(page.locator('#evidence-meaning')).toContainText('not downloaded or verified publisher bytes');await page.locator('#evidence-lens').selectOption('references');await expect(page.locator('#evidence-meaning')).toContainText('not publication approval');
});

test('unsupported evidence queries clear stale exports and reset leaves the synthetic experiment independent',async({page})=>{
 await ready(page);await page.locator('#evidence-editor summary').click();for(const value of ['{"lens":"unknown"}','{"lens":"points","county":"01001"}','{"lens":"policy","date":"2026-10-29"}','{"lens":"context","candidate":true}','x'.repeat(65537)]){
  await page.locator('#evidence-input').fill(value);await expect(page.locator('#evidence-results')).toBeHidden();await expect(page.locator('#evidence-download')).toBeDisabled();await page.locator('#evidence-run').click();await expect(page.locator('#evidence-status')).toContainText(value.length>65536?'64 KiB':'Cannot inspect');
 }
 await page.locator('#evidence-reset').click();await expect(page.locator('#evidence-status')).toContainText('Evidence ready');await expect(page.locator('#evidence-lens')).toHaveValue('points');await expect(page.locator('#error')).toHaveText('6.62');await page.locator('#daylight').selectOption('60');await expect(page.locator('#error')).not.toHaveText('6.62');await expect(page.locator('#evidence-lens')).toHaveValue('points');
});

test('evidence keyboard mobile source custody and engine failure preserve explicit limits',async({page})=>{
 const errors=[],requests=[];page.on('pageerror',e=>errors.push(e.message));page.on('request',r=>requests.push(r.url()));await page.setViewportSize({width:390,height:844});await ready(page);await page.locator('#evidence-lens').focus();await page.keyboard.press('Tab');await expect(page.locator('#evidence-run')).toBeFocused();await page.keyboard.press('Enter');await expect(page.locator('#evidence-status')).toContainText('Evidence ready');const region=page.getByRole('region',{name:'Fixed source and method gates',exact:true});await region.focus();await page.keyboard.press('ArrowRight');await expect.poll(()=>region.evaluate(el=>el.scrollLeft)).toBeGreaterThan(0);expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
 const custody=await(await page.request.get('/ZONES/source-custody.json')).json();expect(custody).toHaveLength(7);for(const file of custody){const response=await page.request.get('/ZONES/'+file.published);expect(response.status()).toBe(200);const body=await response.body();expect(body.length).toBe(file.bytes);expect(createHash('sha256').update(body).digest('hex')).toBe(file.sha256);expect(file.byte_basis).toContain('not primary publisher bytes');}expect((await page.request.get('/ZONES/source-note.md')).ok()).toBe(true);expect(requests.some(u=>/census.gov|dot.gov|ecfr.gov/.test(u))).toBe(false);expect(errors).toEqual([]);
 await page.route('**/pkg/zones_web_bg.wasm',route=>route.abort());await page.reload();await expect(page.locator('#evidence-status')).toContainText('could not load');await expect(page.locator('#evidence-download')).toBeDisabled();await expect(page.locator('#evidence-results')).toBeHidden();
});
