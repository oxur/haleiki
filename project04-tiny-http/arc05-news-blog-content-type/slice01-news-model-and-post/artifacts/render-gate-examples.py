from pathlib import Path
import json, subprocess, tempfile, html, hashlib
import yaml

source=Path('/Users/oubiwann/lab/oxur/haleiki')
out=Path(tempfile.mkdtemp(prefix='haleiki-arc05-gate-studies-',dir='/private/tmp'))
profile=yaml.safe_load((source/'demo/profile.yaml').read_text())
post={
 'title':'Search Lands in the Demo Wiki','url':'/news/search-lands-in-the-demo-wiki/',
 'date_display':'August 21, 2026','date_short':'Aug 21','category':'Project Updates',
 'excerpt':'Client-side Pagefind search is now wired into the demo wiki.'}
description='Changes to Haleiki, its demo wiki, and supporting tools.'
categories=[{'name':'Project Updates','slug':'project-updates','count':1,'description':description,'recent':[post]}]
tags=[{'name':name,'slug':name,'count':1,'size':'small'} for name in ['search','pagefind','tiny-http']]
base={'posts':[post],'total_posts':1,'categories':categories,'tags':tags,
 'months':[{'label':'August 2026','posts':[post]}],'pagination':None,
 'category_sections':[{'name':'Project Updates','slug':'project-updates','posts':[post]}],
 'tag_sections':[{'name':tag['name'],'slug':tag['slug'],'posts':[post]} for tag in tags]}
older=[dict(post,title=f'Illustration only — layout row {i:02}',url=f'#illustration-{i}',date_short=f'Aug {21-i}') for i in range(1,11)]
july=[dict(post,title=f'Illustration only — layout row {i:02}',url=f'#illustration-{i}',date_short=f'Jul {50-i}') for i in range(20,29)]
paged=base|{'total_posts':41,'pagination':{'current':2,'total':3,'newer_url':'/news/archive/','older_url':'/news/archive/page/3/'},
 'months':[{'label':'August 2026','posts':[post]+older},{'label':'July 2026','posts':july}]}
studies=[
 ('archive-unpaged','pages/news-archive.jinja',base,'Archive A — recommended','All posts, month groups, newest first; no pager. Uses only the one approved post.'),
 ('archive-paged','pages/news-archive.jinja',paged,'Archive B — alternative','20 posts per page, monthly headings, page 2 of an illustrative 41-post collection. Nineteen rows here are labelled synthetic layout specimens.'),
 ('categories','pages/news-categories.jinja',base,'Category proposal','Project Updates only. The category description is proposed copy, not an approved vocabulary change.'),
 ('tags','pages/news-tags.jinja',base,'Tags proposal','Retain search, pagefind, tiny-http; one real post each. No tag descriptions or new tags are invented.'),
 ('news-index','pages/news-index.jinja',base,'Index template — feed decision','One approved post through the shipped template. Its Subscribe via RSS link is currently unconditional and points at an unimplemented route.')]
packet={'source':'5fe3cf46fa1ea673b0b84f09167e36de8611d3ee','status':'ILLUSTRATIVE; NOT APPROVED OR ROUTED',
 'profile':profile,'studies':[{'id':i,'template':t,'news':n,'label':l,'explanation':e} for i,t,n,l,e in studies]}
(out/'contexts.json').write_text(json.dumps(packet,indent=2,ensure_ascii=False))
rust='''use std::{env,fs,path::Path};
fn main()->Result<(),Box<dyn std::error::Error>> {
 let args:Vec<String>=env::args().collect();
 let input:serde_json::Value=serde_json::from_str(&fs::read_to_string(&args[1])?)?;
 let mut engine=minijinja::Environment::new();
 engine.set_loader(minijinja::path_loader("/Users/oubiwann/lab/oxur/haleiki/templates"));
 engine.set_auto_escape_callback(|_|minijinja::AutoEscape::Html);
 for study in input["studies"].as_array().ok_or("studies must be an array")? {
  let template=study["template"].as_str().ok_or("template missing")?;
  let id=study["id"].as_str().ok_or("id missing")?;
  let context=serde_json::json!({"site":{"data":{"profile":input["profile"]}},"page":{"title":"Illustrative study","slug":"study","data":{}},"news":study["news"]});
  let rendered=engine.get_template(template)?.render(&context)?;
  fs::write(Path::new(&args[2]).join(format!("{}.html",id)),rendered)?;
 }
 Ok(())
}
'''
(out/'render_study.rs').write_text(rust)
deps=source/'target/debug/deps'
latest=lambda crate:max(deps.glob('lib'+crate+'-*.rlib'),key=lambda p:p.stat().st_mtime)
command=['rustc','--edition=2024','--crate-name','haleiki_gate_study',str(out/'render_study.rs'),
 '--extern','minijinja='+str(latest('minijinja')),'--extern','serde_json='+str(latest('serde_json')),
 '-L','dependency='+str(deps),'-o',str(out/'render_study')]
subprocess.run(command,cwd=source,check=True)
subprocess.run([str(out/'render_study'),str(out/'contexts.json'),str(out)],cwd=source,check=True)
css=(source/'assets/css/haleiki.css').read_text()
sections=[]
for i,t,n,label,explanation in studies:
 doc=(out/(i+'.html')).read_text()
 assert 'Search Lands in the Demo Wiki' in doc
 doc=doc.replace('<link rel="stylesheet" href="/assets/css/haleiki.css">','<style>'+css+'</style>')
 sections.append('<section><h2>'+html.escape(label)+'</h2><p>'+html.escape(explanation)+'</p><iframe title="'+html.escape(label)+'" srcdoc="'+html.escape(doc,quote=True)+'"></iframe></section>')
shell='''<!doctype html><html lang="en"><meta charset="utf-8"><title>Haleiki — Arc 05 decision previews</title>
<style>body{margin:0;padding:24px;background:#f4f0e8;color:#24272b;font:16px/1.5 system-ui}header{max-width:1000px;margin:0 auto 24px}h1{margin:0 0 10px;font-size:28px}h2{font-size:20px;margin:0}p{margin:8px 0 16px}.studies{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:24px;max-width:1500px;margin:auto}section{background:#fff;padding:18px;border:1px solid #b9b7af;border-radius:8px}iframe{width:100%;height:760px;border:1px solid #ddd;background:white}section:last-child{grid-column:1/-1}section:last-child iframe{height:850px}.notice{border-left:4px solid #a84b28;padding-left:14px;font-weight:600}@media(max-width:900px){.studies{grid-template-columns:1fr}}</style>
<header><h1>Haleiki · Arc 05 decisions</h1><p class="notice">ILLUSTRATIVE PREVIEWS — not approved product behavior or implemented routes.</p><p>The native minijinja renders below use unchanged shipped templates. Archive A and taxonomy use the single approved demo post. Archive B adds explicitly labelled layout-only rows. Category description and pagination are proposals.</p></header><main class="studies">'''
review_html=shell+''.join(sections)+'</main></html>'
(out/'gate-preview.html').write_text('\n'.join(line.rstrip() for line in review_html.splitlines()).rstrip()+'\n')
print('STUDY_ROOT',out)
print('NATIVE_RENDER_COUNT',len(studies))
print('CONTEXT_SHA256',hashlib.sha256((out/'contexts.json').read_bytes()).hexdigest())
print('UNPAGED_HAS_PAGER','class="pagination"' in (out/'archive-unpaged.html').read_text())
print('PAGED_HAS_PAGER','class="pagination"' in (out/'archive-paged.html').read_text())
print('CATEGORY_DESCRIPTION_PROPOSED',description)
