import init,{inspect_evidence_json} from './pkg/zones_web.js';
self.onmessage=async({data})=>{try{if(data.type==='init'){await init();self.postMessage({type:'ready'});}else{const canonical=inspect_evidence_json(data.input);self.postMessage({type:'result',id:data.id,canonical,result:JSON.parse(canonical)});}}catch(e){self.postMessage({type:'error',id:data.id,message:String(e)});}};
