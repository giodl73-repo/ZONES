import {test,expect} from '@playwright/test';
test('actual WASM evaluates clock changes, connectivity and shared settings',async({page})=>{
 const errors=[];page.on('pageerror',e=>errors.push(e.message));const wasm=page.waitForResponse(r=>r.url().endsWith('.wasm'));await page.goto('/ZONES/');expect((await wasm).status()).toBe(200);
 await expect(page.locator('#error')).toHaveText('6.62');await expect(page.locator('#verdict')).toHaveText('Zones connected');
 await page.locator('#unit-1').selectOption('1');await page.locator('#unit-3').selectOption('0');await expect(page.locator('#verdict')).toHaveText('Disconnected zones');await expect(page.locator('#moved')).toHaveText('150');
 await page.locator('#daylight').selectOption('60');await expect(page.locator('#error')).not.toHaveText('6.62');
 await page.getByRole('button',{name:'Share experiment'}).click();await page.reload();await expect(page.locator('#verdict')).toHaveText('Disconnected zones');
 const download=page.waitForEvent('download');await page.locator('#export').click();expect((await download).suggestedFilename()).toBe('zones-synthetic-comparison.json');
 await page.locator('#reset').click();await expect(page.locator('#error')).toHaveText('6.62');expect(errors).toEqual([]);
});
test('mobile invalid query remains bounded and engine failures readable',async({page})=>{
 await page.setViewportSize({width:390,height:844});await page.goto('/ZONES/?west=999999&units=xxxx');await expect(page.locator('#error')).toHaveText('6.62');expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBeTruthy();
 await page.route('**/*.wasm',r=>r.fulfill({status:503,body:'unavailable'}));await page.reload();await expect(page.locator('#verdict')).toHaveText('Could not evaluate');await expect(page.locator('#export')).toBeDisabled();
});
