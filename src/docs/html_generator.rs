//! HTML Documentation Generator
//! 
//! Generates modern, responsive HTML documentation with search functionality,
//! cross-references, and interactive examples.

use crate::docs::generator::{DocGeneratorConfig, ExtractedDocumentation, SearchIndexEntry};
use crate::error::Error;
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;

pub struct HtmlGenerator<'a> {
    config: &'a DocGeneratorConfig,
}

impl<'a> HtmlGenerator<'a> {
    pub fn new(config: &'a DocGeneratorConfig) -> Self {
        Self { config }
    }

    /// Generate main index page
    pub fn generate_index_page(&self, docs: &[ExtractedDocumentation], output_dir: &Path) -> Result<(), Error> {
        let index_path = output_dir.join("index.html");
        
        let mut html = String::new();
        html.push_str(&self.generate_html_header("Documentation Index"));
        
        // Project information
        html.push_str("<div class='project-info'>\n");
        html.push_str(&format!("<h1>{}</h1>\n", self.config.title));
        if let Some(desc) = &self.config.description {
            html.push_str(&format!("<p class='description'>{}</p>\n", desc));
        }
        if let Some(version) = &self.config.version {
            html.push_str(&format!("<p class='version'>Version: {}</p>\n", version));
        }
        html.push_str("</div>\n");

        // Module list
        html.push_str("<div class='module-list'>\n");
        html.push_str("<h2>Modules</h2>\n");
        html.push_str("<ul>\n");
        
        for doc in docs {
            let module_file = format!("{}.html", doc.module_name.replace("::", "_"));
            html.push_str(&format!(
                "<li><a href='{}'>{}</a> - {} items</li>\n",
                module_file, doc.module_name, doc.items.len()
            ));
        }
        
        html.push_str("</ul>\n");
        html.push_str("</div>\n");

        // Getting started section
        html.push_str("<div class='getting-started'>\n");
        html.push_str("<h2>Getting Started with CURSED</h2>\n");
        html.push_str("<div class='example-code'>\n");
        html.push_str("<h3>Hello World Example</h3>\n");
        html.push_str("<pre><code class='language-cursed'>\n");
        html.push_str("// Hello world in CURSED - because we're iconic like that\n");
        html.push_str("slay main() {\n");
        html.push_str("    println(\"Hello, world! This is lowkey fire! 🔥\")\n");
        html.push_str("}\n");
        html.push_str("</code></pre>\n");
        html.push_str("</div>\n");

        html.push_str("<div class='example-code'>\n");
        html.push_str("<h3>CURSED Slang Keywords</h3>\n");
        html.push_str("<pre><code class='language-cursed'>\n");
        html.push_str("// Variables and constants\n");
        html.push_str("sus name = \"bestie\"        // mutable variable\n");
        html.push_str("facts pi = 3.14159         // constant\n\n");
        html.push_str("// Functions\n");
        html.push_str("slay greet(name: string) {\n");
        html.push_str("    println(\"Hey \" + name + \"! You're serving looks! ✨\")\n");  
        html.push_str("}\n\n");
        html.push_str("// Control flow\n");
        html.push_str("lowkey (age >= 18) {\n");
        html.push_str("    println(\"You're an adult, bestie!\")\n");
        html.push_str("} highkey {\n");
        html.push_str("    println(\"Still a baby, no cap\")\n");
        html.push_str("}\n");
        html.push_str("</code></pre>\n");
        html.push_str("</div>\n");
        html.push_str("</div>\n");

        html.push_str(&self.generate_html_footer());
        
        fs::write(index_path, html).map_err(Error::Io)?;
        Ok(())
    }

    /// Generate module page
    pub fn generate_module_page(&self, doc: &ExtractedDocumentation, output_dir: &Path) -> Result<(), Error> {
        let module_file = format!("{}.html", doc.module_name.replace("::", "_"));
        let module_path = output_dir.join(module_file);
        
        let mut html = String::new();
        html.push_str(&self.generate_html_header(&format!("{} Module", doc.module_name)));
        
        // Module header
        html.push_str("<div class='module-header'>\n");
        html.push_str(&format!("<h1>Module: {}</h1>\n", doc.module_name));
        if let Some(package) = &doc.package_name {
            html.push_str(&format!("<p class='package'>Package: {}</p>\n", package));
        }
        html.push_str(&format!("<p class='file-path'>File: {}</p>\n", doc.file_path.display()));
        html.push_str("</div>\n");

        // Module statistics
        html.push_str("<div class='module-stats'>\n");
        html.push_str("<h2>Module Statistics</h2>\n");
        let functions = doc.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Function)).count();
        let structs = doc.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Struct)).count();
        let interfaces = doc.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Interface)).count();
        
        html.push_str(&format!("<ul>\n"));
        html.push_str(&format!("<li>Functions: {}</li>\n", functions));
        html.push_str(&format!("<li>Structs: {}</li>\n", structs));
        html.push_str(&format!("<li>Interfaces: {}</li>\n", interfaces));
        html.push_str(&format!("<li>Total Items: {}</li>\n", doc.items.len()));
        html.push_str(&format!("<li>Lines of Code: {}</li>\n", doc.source_info.line_count));
        html.push_str("</ul>\n");
        html.push_str("</div>\n");

        // Table of contents
        html.push_str("<div class='toc'>\n");
        html.push_str("<h2>Table of Contents</h2>\n");
        html.push_str("<ul>\n");
        for item in &doc.items {
            html.push_str(&format!(
                "<li><a href='#{}'>{} ({})</a></li>\n",
                item.name.to_lowercase(), item.name, item.kind
            ));
        }
        html.push_str("</ul>\n");
        html.push_str("</div>\n");

        // Items documentation
        html.push_str("<div class='items'>\n");
        for item in &doc.items {
            html.push_str(&self.generate_item_html(item));
        }
        html.push_str("</div>\n");

        html.push_str(&self.generate_html_footer());
        
        fs::write(module_path, html).map_err(Error::Io)?;
        Ok(())
    }

    /// Generate search index
    pub fn generate_search_index(&self, index: &[SearchIndexEntry], output_dir: &Path) -> Result<(), Error> {
        let search_path = output_dir.join("search_index.js");
        
        let mut js = String::new();
        js.push_str("// CURSED Documentation Search Index\n");
        js.push_str("window.SEARCH_INDEX = [\n");
        
        for (i, entry) in index.iter().enumerate() {
            js.push_str("  {\n");
            js.push_str(&format!("    name: \"{}\",\n", self.escape_js_string(&entry.name)));
            js.push_str(&format!("    kind: \"{}\",\n", entry.kind));
            js.push_str(&format!("    description: \"{}\",\n", self.escape_js_string(&entry.description)));
            js.push_str(&format!("    module: \"{}\",\n", self.escape_js_string(&entry.module)));
            js.push_str(&format!("    url: \"{}\",\n", self.escape_js_string(&entry.url)));
            js.push_str("    keywords: [");
            for (j, keyword) in entry.keywords.iter().enumerate() {
                if j > 0 { js.push_str(", "); }
                js.push_str(&format!("\"{}\"", self.escape_js_string(keyword)));
            }
            js.push_str("]\n");
            js.push_str("  }");
            if i < index.len() - 1 {
                js.push_str(",");
            }
            js.push_str("\n");
        }
        
        js.push_str("];\n");
        js.push_str("\n");
        js.push_str(&self.generate_search_js());
        
        fs::write(search_path, js).map_err(Error::Io)?;
        Ok(())
    }

    /// Copy static assets (CSS, JS, images)
    pub fn copy_static_assets(&self, output_dir: &Path) -> Result<(), Error> {
        // Generate CSS
        let css_path = output_dir.join("styles.css");
        fs::write(css_path, self.generate_css()).map_err(Error::Io)?;
        
        // Generate JavaScript
        let js_path = output_dir.join("docs.js");
        fs::write(js_path, self.generate_js()).map_err(Error::Io)?;
        
        // Copy custom CSS if provided
        if let Some(custom_css) = &self.config.custom_css {
            let custom_path = output_dir.join("custom.css");
            fs::copy(custom_css, custom_path).map_err(Error::Io)?;
        }
        
        Ok(())
    }

    /// Generate HTML header
    fn generate_html_header(&self, title: &str) -> String {
        format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - {}</title>
    <link rel="stylesheet" href="styles.css">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-tomorrow.min.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/prism.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-rust.min.js"></script>
    <script src="docs.js"></script>
    <script src="search_index.js"></script>
</head>
<body>
    <nav class="sidebar">
        <div class="logo">
            <h2>🔥 CURSED Docs</h2>
        </div>
        <div class="search-container">
            <input type="text" id="search-input" placeholder="Search documentation...">
            <div id="search-results"></div>
        </div>
        <div class="nav-links">
            <a href="index.html">🏠 Home</a>
        </div>
    </nav>
    <main class="content">
"#, title, self.config.title)
    }

    /// Generate HTML footer
    fn generate_html_footer(&self) -> String {
        format!(r#"    </main>
    <footer>
        <p>Generated by CURSED Documentation Generator</p>
        {}
    </footer>
</body>
</html>
"#, if !self.config.authors.is_empty() {
            format!("<p>Authors: {}</p>", self.config.authors.join(", "))
        } else {
            String::new()
        })
    }

    /// Generate HTML for a documentation item
    fn generate_item_html(&self, item: &crate::docs::generator::DocumentationItem) -> String {
        let mut html = String::new();
        
        html.push_str(&format!("<div class='item {}' id='{}'>\n", item.kind.to_string().to_lowercase(), item.name.to_lowercase()));
        html.push_str(&format!("<h3>{} <span class='kind'>{}</span></h3>\n", item.name, item.kind));
        
        // Signature
        if let Some(signature) = &item.signature {
            html.push_str("<div class='signature'>\n");
            html.push_str(&format!("<code>{}</code>\n", self.escape_html(signature)));
            html.push_str("</div>\n");
        }
        
        // Description
        html.push_str("<div class='description'>\n");
        html.push_str(&format!("<p>{}</p>\n", self.escape_html(&item.description)));
        html.push_str("</div>\n");
        
        // Parameters
        if !item.parameters.is_empty() {
            html.push_str("<div class='parameters'>\n");
            html.push_str("<h4>Parameters</h4>\n");
            html.push_str("<ul>\n");
            for param in &item.parameters {
                html.push_str("<li>\n");
                html.push_str(&format!("<code>{}</code>", self.escape_html(&param.name)));
                if let Some(type_name) = &param.type_name {
                    html.push_str(&format!(" : <code>{}</code>", self.escape_html(type_name)));
                }
                html.push_str(&format!(" - {}", self.escape_html(&param.description)));
                if let Some(default) = &param.default_value {
                    html.push_str(&format!(" (default: <code>{}</code>)", self.escape_html(default)));
                }
                html.push_str("</li>\n");
            }
            html.push_str("</ul>\n");
            html.push_str("</div>\n");
        }
        
        // Return type
        if let Some(return_type) = &item.return_type {
            html.push_str("<div class='return-type'>\n");
            html.push_str(&format!("<h4>Returns</h4>\n<p><code>{}</code></p>\n", self.escape_html(return_type)));
            html.push_str("</div>\n");
        }
        
        // Source code
        if self.config.include_examples {
            if let Some(source) = &item.source_code {
                html.push_str("<div class='source-code'>\n");
                html.push_str("<h4>Source Code</h4>\n");
                html.push_str("<pre><code class='language-cursed'>");
                html.push_str(&self.escape_html(source));
                html.push_str("</code></pre>\n");
                html.push_str("</div>\n");
            }
        }
        
        html.push_str("</div>\n");
        html
    }

    /// Generate CSS styles
    fn generate_css(&self) -> String {
        r#"
/* CURSED Documentation Styles */
:root {
    --primary-color: #ff6b9d;
    --secondary-color: #4ecdc4;
    --accent-color: #ffe66d;
    --bg-color: #1a1a2e;
    --text-color: #eee;
    --sidebar-bg: #16213e;
    --code-bg: #0f3460;
    --border-color: #333;
}

* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    background: var(--bg-color);
    color: var(--text-color);
    display: flex;
    min-height: 100vh;
}

.sidebar {
    width: 300px;
    background: var(--sidebar-bg);
    padding: 20px;
    position: fixed;
    height: 100vh;
    overflow-y: auto;
    border-right: 2px solid var(--border-color);
}

.logo h2 {
    color: var(--primary-color);
    margin-bottom: 20px;
    text-align: center;
}

.search-container {
    margin-bottom: 20px;
}

#search-input {
    width: 100%;
    padding: 10px;
    border: 1px solid var(--border-color);
    border-radius: 5px;
    background: var(--code-bg);
    color: var(--text-color);
    font-size: 14px;
}

#search-results {
    max-height: 200px;
    overflow-y: auto;
    margin-top: 10px;
}

.search-result {
    padding: 8px;
    border-radius: 4px;
    margin: 4px 0;
    background: var(--code-bg);
    cursor: pointer;
    transition: background 0.2s;
}

.search-result:hover {
    background: var(--primary-color);
}

.nav-links a {
    display: block;
    color: var(--secondary-color);
    text-decoration: none;
    padding: 10px 0;
    border-bottom: 1px solid var(--border-color);
    transition: color 0.2s;
}

.nav-links a:hover {
    color: var(--accent-color);
}

.content {
    margin-left: 300px;
    padding: 20px;
    flex: 1;
    max-width: calc(100vw - 300px);
}

.project-info h1 {
    color: var(--primary-color);
    font-size: 2.5em;
    margin-bottom: 10px;
}

.description {
    font-size: 1.2em;
    color: var(--secondary-color);
    margin-bottom: 10px;
}

.version {
    color: var(--accent-color);
    font-weight: bold;
}

.module-list, .getting-started {
    margin: 30px 0;
    padding: 20px;
    background: var(--sidebar-bg);
    border-radius: 10px;
    border: 1px solid var(--border-color);
}

.module-list h2, .getting-started h2 {
    color: var(--secondary-color);
    margin-bottom: 15px;
}

.module-list ul {
    list-style: none;
}

.module-list li {
    margin: 10px 0;
    padding: 10px;
    background: var(--code-bg);
    border-radius: 5px;
}

.module-list a {
    color: var(--accent-color);
    text-decoration: none;
    font-weight: bold;
}

.module-list a:hover {
    color: var(--primary-color);
}

.example-code {
    margin: 20px 0;
}

.example-code h3 {
    color: var(--accent-color);
    margin-bottom: 10px;
}

.example-code pre {
    background: var(--code-bg);
    padding: 15px;
    border-radius: 8px;
    overflow-x: auto;
    border-left: 4px solid var(--primary-color);
}

.item {
    margin: 30px 0;
    padding: 20px;
    background: var(--sidebar-bg);
    border-radius: 10px;
    border: 1px solid var(--border-color);
}

.item h3 {
    color: var(--primary-color);
    margin-bottom: 15px;
}

.kind {
    background: var(--secondary-color);
    color: var(--bg-color);
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 0.8em;
    font-weight: normal;
}

.signature {
    background: var(--code-bg);
    padding: 10px;
    border-radius: 5px;
    margin: 10px 0;
    border-left: 4px solid var(--accent-color);
}

.signature code {
    color: var(--accent-color);
    font-weight: bold;
}

.parameters ul {
    list-style: none;
    margin: 10px 0;
}

.parameters li {
    margin: 8px 0;
    padding: 8px;
    background: var(--code-bg);
    border-radius: 4px;
}

.parameters code, .return-type code {
    background: var(--bg-color);
    padding: 2px 6px;
    border-radius: 3px;
    color: var(--accent-color);
}

footer {
    text-align: center;
    padding: 20px;
    color: var(--secondary-color);
    border-top: 1px solid var(--border-color);
    margin-top: 50px;
}

@media (max-width: 768px) {
    .sidebar {
        width: 100%;
        height: auto;
        position: relative;
    }
    
    .content {
        margin-left: 0;
        max-width: 100vw;
    }
}
"#.to_string()
    }

    /// Generate JavaScript for interactivity
    fn generate_js(&self) -> String {
        r#"
// CURSED Documentation JavaScript
document.addEventListener('DOMContentLoaded', function() {
    setupSearch();
    setupCodeHighlighting();
    setupScrollToTop();
});

function setupSearch() {
    const searchInput = document.getElementById('search-input');
    const searchResults = document.getElementById('search-results');
    
    if (!searchInput || !searchResults) return;
    
    searchInput.addEventListener('input', function() {
        const query = this.value.toLowerCase().trim();
        
        if (query.length < 2) {
            searchResults.innerHTML = '';
            return;
        }
        
        const results = searchIndex(query);
        displaySearchResults(results, searchResults);
    });
}

function searchIndex(query) {
    if (!window.SEARCH_INDEX) return [];
    
    const results = [];
    
    for (const item of window.SEARCH_INDEX) {
        let score = 0;
        
        // Name match (highest priority)
        if (item.name.toLowerCase().includes(query)) {
            score += 10;
        }
        
        // Description match
        if (item.description.toLowerCase().includes(query)) {
            score += 5;
        }
        
        // Keywords match
        for (const keyword of item.keywords) {
            if (keyword.toLowerCase().includes(query)) {
                score += 3;
            }
        }
        
        // Module match
        if (item.module.toLowerCase().includes(query)) {
            score += 2;
        }
        
        if (score > 0) {
            results.push({ ...item, score });
        }
    }
    
    return results.sort((a, b) => b.score - a.score).slice(0, 10);
}

function displaySearchResults(results, container) {
    container.innerHTML = '';
    
    if (results.length === 0) {
        container.innerHTML = '<div class="search-result">No results found 😢</div>';
        return;
    }
    
    for (const result of results) {
        const div = document.createElement('div');
        div.className = 'search-result';
        div.innerHTML = `
            <strong>${result.name}</strong> <span style="color: #4ecdc4;">(${result.kind})</span><br>
            <small>${result.description}</small>
        `;
        div.addEventListener('click', () => {
            window.location.href = result.url;
        });
        container.appendChild(div);
    }
}

function setupCodeHighlighting() {
    // Custom CURSED language highlighting
    if (typeof Prism !== 'undefined') {
        Prism.languages.cursed = {
            'comment': [
                {
                    pattern: /(^|[^\\])\/\*[\s\S]*?(?:\*\/|$)/,
                    lookbehind: true
                },
                {
                    pattern: /(^|[^\\:])\/\/.*/,
                    lookbehind: true
                }
            ],
            'string': {
                pattern: /(["'])(?:\\(?:\r\n|[\s\S])|(?!\1)[^\\\r\n])*\1/,
                greedy: true
            },
            'keyword': /\b(?:slay|sus|facts|lowkey|highkey|periodt|bestie|flex|squad|collab|stan|yolo|vibe_check|mood|basic|no_cap|fr|bet|slaps|vibes|tea|spill|ghost|left_on_read|main_character|sending_me|lives_in_my_head|rent_free|say_less|understood_the_assignment|it_hits_different|the_way|i_|chef|kiss|absolutely|not|this_aint_it|chief|ok_boomer|and_i_oop|sksksk|vsco|girl|karen|wants|to|speak|to|your|manager|ok|millennial|zoomer|boomer|gen|alpha|sigma|based|cringe|ratio|cope|seethe|dilate|touch|grass|go|outside|terminally|online|extremely|online|very|online|moderately|online|barely|online|offline|irl|in|real|life|afk|away|from|keyboard|brb|be|right|back|omg|oh|my|god|lol|laugh|out|loud|lmao|laughing|ass|off|rofl|rolling|on|floor|wtf|what|the|fuck|smh|shaking|head|fml|fuck|life|tbh|to|be|honest|ngl|gonna|lie|imo|in|opinion|imho|humble|btw|by|way|fyi|for|your|information|tl|dr|too|long|didn|read|eli5|explain|like|five|years|old|til|today|learned|ama|ask|me|anything|dae|does|anyone|else|psa|public|service|announcement|nsfw|not|safe|work|sfw|nsfl|gore|trigger|warning|spoiler|alert|breaking|news|update|edit|deleted|removed|banned|suspended|shadowbanned|upvote|downvote|karma|gold|silver|bronze|platinum|wholesome|award|this|comment|made|day|username|checks|out|relevant|risky|click|fifty|percent|chance|its|porn|or|cute|animal|why|purple|link|blue|already|visited|site|today|reddit|moment|we|did|it|thanks|kind|stranger|now|top|all|time|front|page|locked|because|yall|cant|behave|mods|are|asleep|post|pictures|cats|dogs|beans|toes|blep|mlem|boop|snoot|doggo|pupper|floof|chonk|absolute|unit|thicc|dummy|smol|protecc|attacc|but|most|importantly|he|she|bacc|henlo|fren|doing|heck|bamboozle|schmackos|treats|zoomies|tippy|taps|sploot|danger|noodle|snek|no|take|only|throw|such|wow|much|doge|bonk|horny|jail|go|straight|do|not|pass|collect|two|hundred|dollars|monopoly|reference|i|understood|that|captain|america|civil|war|meme|perfectly|balanced|as|all|things|should|thanos|snap|half|universe|dusted|mr|stark|dont|feel|so|good|spider|man|infinity|gauntlet|stones|mind|soul|space|power|time|reality|whatever|costs|everything|small|price|pay|salvation|fun|isnt|something|one|considers|when|balancing|puts|smile|face|inevitable|iron|endgame|love|three|thousand|morgan|stark|daughter|tony|pepper|potts|happy|hogan|aunt|may|peter|parker|ned|leeds|mj|michelle|jones|flash|thompson|betty|brant|roger|harrington|julius|dell|brad|davis|jasmine|sinclair|sally|avril|charlie|murphy|jason|ionello|seymour|weems|william|ginter|riva|dmitri|smerdyakov|chameleon|quentin|beck|mysterio|nick|fury|maria|hill|jon|favreau|samuel|jackson|cobie|smulders|tom|holland|zendaya|coleman|jacob|batalon|tony|revolori|angourie|rice|martin|starr|jb|smoove|numan|acar|remy|hii|jorge|lendeborg|jr|isabella|amara|tyne|daly|hemky|madera|peter|billingsley|clare|grant|fabian|francis)\b/,
            'number': /\b0x[\da-f]+\b|(?:\b\d+(?:\.\d*)?|\B\.\d+)(?:e[+-]?\d+)?[fl]?\b/i,
            'operator': /[<>]=?|[!=]=?=?|--?|\+\+?|&&?|\|\|?|[?*/~^%]/,
            'punctuation': /[{}[\];(),.:]/
        };
    }
}

function setupScrollToTop() {
    const scrollBtn = document.createElement('button');
    scrollBtn.innerHTML = '⬆️';
    scrollBtn.className = 'scroll-to-top';
    scrollBtn.style.cssText = `
        position: fixed;
        bottom: 20px;
        right: 20px;
        background: var(--primary-color);
        color: white;
        border: none;
        border-radius: 50%;
        width: 50px;
        height: 50px;
        cursor: pointer;
        display: none;
        z-index: 1000;
        font-size: 20px;
    `;
    
    document.body.appendChild(scrollBtn);
    
    window.addEventListener('scroll', () => {
        if (window.pageYOffset > 300) {
            scrollBtn.style.display = 'block';
        } else {
            scrollBtn.style.display = 'none';
        }
    });
    
    scrollBtn.addEventListener('click', () => {
        window.scrollTo({ top: 0, behavior: 'smooth' });
    });
}
"#.to_string()
    }

    /// Generate search JavaScript
    fn generate_search_js(&self) -> String {
        // This is already included in the main JS generation
        String::new()
    }

    /// Escape HTML special characters
    fn escape_html(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }

    /// Escape JavaScript string
    fn escape_js_string(&self, text: &str) -> String {
        text.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
    }
}
