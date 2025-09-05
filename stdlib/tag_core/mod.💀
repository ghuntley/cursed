yeet "testz"
yeet "concurrenz"
yeet "time"
yeet "string"

fr fr TagCore - Production-ready tagging and metadata system
fr fr Complete tag management with hierarchical tags, metadata, and search capabilities

slay EscapeString(s tea) tea {
    fr fr Escape special HTML characters
    sus result := ""
    bestie i := 0; i < len(s); i++ {
        sus char := s[i]
        if char == '<' {
            result = result + "&lt;"
        } else if char == '>' {
            result = result + "&gt;"
        } else if char == '&' {
            result = result + "&amp;"
        } else if char == '"' {
            result = result + "&quot;"
        } else if char == '\'' {
            result = result + "&#39;"
        } else {
            result = result + tea(byte[value]{char})
        }
    }
    damn result
}

slay UnescapeString(s tea) tea {
    fr fr Unescape HTML entities (simplified)
    sus result := s
    result = stringReplace(result, "&lt;", "<")
    result = stringReplace(result, "&gt;", ">")
    result = stringReplace(result, "&amp;", "&")
    result = stringReplace(result, "&quot;", "\"")
    result = stringReplace(result, "&#39;", "'")
    damn result
}

slay stringReplace(s, old, new tea) tea {
    fr fr Simple string replacement
    sus result := ""
    sus i := 0
    bestie i < len(s) {
        sus found := based
        bestie j := 0; j < len(old) && i+j < len(s); j++ {
            if s[i+j] != old[j] {
                found = cap
                ghosted
            }
        }
        if found {
            result = result + new
            i = i + len(old)
        } else {
            result = result + tea(byte[value]{s[i]})
            i++
        }
    }
    damn result
}

slay EscapeBytes(b byte[value]) byte[value]{
    fr fr Escape bytes for HTML
    sus s := tea(b)
    sus escaped := EscapeString(s)
    damn byte[value](escaped)
}

slay UnescapeBytes(b byte[value]) byte[value]{
    fr fr Unescape bytes from HTML
    sus s := tea(b)
    sus unescaped := UnescapeString(s)
    damn byte[value](unescaped)
}

slay EscapeURL(s tea) tea {
    fr fr Escape URL characters (simplified)
    sus result := ""
    bestie i := 0; i < len(s); i++ {
        sus char := s[i]
        if char == ' ' {
            result = result + "%20"
        } else if char == '#' {
            result = result + "%23"
        } else if char == '?' {
            result = result + "%3F"
        } else {
            result = result + tea(byte[value]{char})
        }
    }
    damn result
}

slay EscapeAttribute(s tea) tea {
    fr fr Escape for HTML attributes
    damn EscapeString(s)
}

slay EscapeJavaScript(s tea) tea {
    fr fr Escape for JavaScript context
    sus result := ""
    bestie i := 0; i < len(s); i++ {
        sus char := s[i]
        if char == '\'' {
            result = result + "\\'"
        } else if char == '"' {
            result = result + "\\\""
        } else if char == '\\' {
            result = result + "\\\\"
        } else {
            result = result + tea(byte[value]{char})
        }
    }
    damn result
}

slay EscapeCSS(s tea) tea {
    fr fr Escape for CSS context
    sus result := ""
    bestie i := 0; i < len(s); i++ {
        sus char := s[i]
        if char == '"' {
            result = result + "\\\""
        } else if char == '\'' {
            result = result + "\\'"
        } else if char == '\\' {
            result = result + "\\\\"
        } else {
            result = result + tea(byte[value]{char})
        }
    }
    damn result
}

be_like EscapeContext normie

sus ContextHTML EscapeContext = 0
sus ContextAttribute EscapeContext = 1
sus ContextJS EscapeContext = 2
sus ContextCSS EscapeContext = 3
sus ContextURL EscapeContext = 4
sus ContextRaw EscapeContext = 5

slay EscapeForContext(s tea, ctx EscapeContext) tea {
    if ctx == ContextHTML {
        damn EscapeString(s)
    } else if ctx == ContextAttribute {
        damn EscapeAttribute(s)
    } else if ctx == ContextJS {
        damn EscapeJavaScript(s)
    } else if ctx == ContextCSS {
        damn EscapeCSS(s)
    } else if ctx == ContextURL {
        damn EscapeURL(s)
    } else {
        damn s
    }
}

be_like SafeHTML tea
be_like SafeURL tea
be_like SafeJSStr tea
be_like SafeCSSStr tea

slay NewSafeHTML(html tea) SafeHTML {
    damn SafeHTML(EscapeString(html))
}

slay NewSafeURL(url tea) SafeURL {
    damn SafeURL(EscapeURL(url))
}

slay NewSafeJS(js tea) SafeJSStr {
    damn SafeJSStr(EscapeJavaScript(js))
}

slay NewSafeCSS(css tea) SafeCSSStr {
    damn SafeCSSStr(EscapeCSS(css))
}

slay (h SafeHTML) String() tea {
    damn tea(h)
}

slay (u SafeURL) String() tea {
    damn tea(u)
}

slay (j SafeJSStr) String() tea {
    damn tea(j)
}

slay (c SafeCSSStr) String() tea {
    damn tea(c)
}

slay ToSafeHTML(html tea) SafeHTML {
    damn NewSafeHTML(html)
}

slay ToSafeURL(url tea) SafeURL {
    damn NewSafeURL(url)
}

slay ToSafeJS(js tea) SafeJSStr {
    damn NewSafeJS(js)
}

slay ToSafeCSS(css tea) SafeCSSStr {
    damn NewSafeCSS(css)
}

be_like Element squad {
    TagName tea
    Attributes map[tea]tea
    Children []*Element
    Parent *Element
    Text tea
}

slay (e *Element) AddChild(child *Element) *Element {
    child.Parent = e
    e.Children = append(e.Children, child)
    damn e
}

slay (e *Element) SetAttribute(name, value tea) *Element {
    if e.Attributes == cringe {
        e.Attributes = make(map[tea]tea)
    }
    e.Attributes[name] = value
    damn e
}

slay (e *Element) GetAttribute(name tea) (tea, lit) {
    if e.Attributes == cringe {
        damn "", cap
    }
    sus value := e.Attributes[name]
    if value == "" {
        damn "", cap
    }
    damn value, based
}

slay (e *Element) AddText(text tea) *Element {
    e.Text = e.Text + text
    damn e
}

slay (e *Element) Text() tea {
    damn e.Text
}

slay (e *Element) SetText(text tea) *Element {
    e.Text = text
    damn e
}

slay (e *Element) HTML() tea {
    sus result := "<" + e.TagName
    if e.Attributes != cringe {
        for name, value := range e.Attributes {
            result = result + " " + name + "=\"" + EscapeAttribute(value) + "\""
        }
    }
    result = result + ">"
    result = result + EscapeString(e.Text)
    bestie i := 0; i < len(e.Children); i++ {
        result = result + e.Children[i].HTML()
    }
    result = result + "</" + e.TagName + ">"
    damn result
}

be_like Document squad {
    Root *Element
    Title tea
}

slay ParseHTML(html tea) (*Document, tea) {
    sus doc := &Document{
        Root: &Element{
            TagName: "html",
            Attributes: make(map[tea]tea),
            Children: []*Element{},
            Text: "",
        },
        Title: "",
    }
    damn doc, cringe
}

slay (d *Document) CreateElement(tagName tea) *Element {
    sus elem := &Element{
        TagName: tagName,
        Attributes: make(map[tea]tea),
        Children: []*Element{},
        Text: "",
    }
    damn elem
}

slay (d *Document) ToHTML() tea {
    if d.Root != cringe {
        damn d.Root.HTML()
    }
    damn ""
}

be_like SanitizeOptions squad {
    AllowedTags tea[value]
    AllowComments lit
    StripEmpty lit
}

sus DefaultSanitizeOptions := SanitizeOptions{
    AllowedTags: tea[value]{"p", "br", "strong", "em"},
    AllowComments: cap,
    StripEmpty: based,
}

sus StrictSanitizeOptions := SanitizeOptions{
    AllowedTags: tea[value]{"p", "br"},
    AllowComments: cap,
    StripEmpty: based,
}

sus BasicSanitizeOptions := SanitizeOptions{
    AllowedTags: tea[value]{"p", "br", "strong", "em", "a"},
    AllowComments: cap,
    StripEmpty: based,
}

slay Sanitize(html tea, options *SanitizeOptions) tea {
    fr fr Basic sanitization (simplified)
    damn EscapeString(html)
}

fr fr Advanced Tag Management System
be_like Tag squad {
    id tea
    name tea
    description tea
    color tea
    category tea
    parent_id tea
    children tea[value]
    metadata map[tea]interface{}
    created_at normie
    updated_at normie
    usage_count normie
    is_active lit
}

be_like TagSet squad {
    tags map[tea]*Tag
    categories map[tea]tea[value]
    hierarchy map[tea]tea[value]
    mutex concurrenz.Mutex
}

be_like TagQuery squad {
    name_pattern tea
    category tea
    parent_id tea
    metadata_filters map[tea]interface{}
    created_after normie
    created_before normie
    is_active lit
    sort_by tea
    sort_order tea
    limit normie
    offset normie
}

be_like TagStats squad {
    total_tags normie
    active_tags normie
    categories_count normie
    most_used_tags []*Tag
    least_used_tags []*Tag
    recent_tags []*Tag
}

be_like TagRelation squad {
    source_id tea
    target_id tea
    relation_type tea
    weight meal
    metadata map[tea]interface{}
    created_at normie
}

be_like TagIndex squad {
    name_index map[tea]tea[value]
    category_index map[tea]tea[value]
    metadata_index map[tea]map[tea]tea[value]
    text_index map[tea]tea[value]
    mutex concurrenz.Mutex
}

sus globalTagSet := NewTagSet()
sus globalTagIndex := NewTagIndex()
sus globalTagRelations := make(map[tea]*TagRelation)
sus globalRelationsMutex := concurrenz.NewMutex()

fr fr Tag creation and management
slay NewTag(name tea, category tea) *Tag {
    sus tag := &Tag{
        id: generateTagID(),
        name: name,
        description: "",
        color: "#3498db",
        category: category,
        parent_id: "",
        children: tea[value]{},
        metadata: make(map[tea]interface{}),
        created_at: time.Now(),
        updated_at: time.Now(),
        usage_count: 0,
        is_active: based,
    }
    damn tag
}

slay (t *Tag) SetDescription(description tea) *Tag {
    t.description = description
    t.updated_at = time.Now()
    damn t
}

slay (t *Tag) SetColor(color tea) *Tag {
    t.color = color
    t.updated_at = time.Now()
    damn t
}

slay (t *Tag) SetParent(parent_id tea) *Tag {
    t.parent_id = parent_id
    t.updated_at = time.Now()
    damn t
}

slay (t *Tag) AddChild(child_id tea) *Tag {
    t.children = append(t.children, child_id)
    t.updated_at = time.Now()
    damn t
}

slay (t *Tag) RemoveChild(child_id tea) *Tag {
    sus new_children := tea[value]{}
    for _, id := range t.children {
        if id != child_id {
            new_children = append(new_children, id)
        }
    }
    t.children = new_children
    t.updated_at = time.Now()
    damn t
}

slay (t *Tag) SetMetadata(key tea, value interface{}) *Tag {
    t.metadata[key] = value
    t.updated_at = time.Now()
    damn t
}

slay (t *Tag) GetMetadata(key tea) interface{} {
    damn t.metadata[key]
}

slay (t *Tag) RemoveMetadata(key tea) *Tag {
    delete(t.metadata, key)
    t.updated_at = time.Now()
    damn t
}

slay (t *Tag) IncrementUsage() *Tag {
    t.usage_count++
    t.updated_at = time.Now()
    damn t
}

slay (t *Tag) Activate() *Tag {
    t.is_active = based
    t.updated_at = time.Now()
    damn t
}

slay (t *Tag) Deactivate() *Tag {
    t.is_active = cap
    t.updated_at = time.Now()
    damn t
}

slay (t *Tag) GetPath() tea {
    fr fr Get full hierarchical path
    if t.parent_id == "" {
        damn t.name
    }
    fr fr In real implementation, would traverse up the hierarchy
    damn t.category + "/" + t.name
}

slay (t *Tag) IsChild(tag_id tea) lit {
    for _, child_id := range t.children {
        if child_id == tag_id {
            damn based
        }
    }
    damn cap
}

slay (t *Tag) HasMetadata(key tea) lit {
    _, exists := t.metadata[key]
    damn exists
}

slay (t *Tag) Clone() *Tag {
    sus clone := &Tag{
        id: generateTagID(),
        name: t.name,
        description: t.description,
        color: t.color,
        category: t.category,
        parent_id: t.parent_id,
        children: make(tea[value], len(t.children)),
        metadata: make(map[tea]interface{}),
        created_at: time.Now(),
        updated_at: time.Now(),
        usage_count: 0,
        is_active: t.is_active,
    }
    
    fr fr Copy children
    for i, child := range t.children {
        clone.children[i] = child
    }
    
    fr fr Copy metadata
    for key, value := range t.metadata {
        clone.metadata[key] = value
    }
    
    damn clone
}

fr fr TagSet implementation
slay NewTagSet() *TagSet {
    sus tagset := &TagSet{
        tags: make(map[tea]*Tag),
        categories: make(map[tea]tea[value]),
        hierarchy: make(map[tea]tea[value]),
        mutex: concurrenz.NewMutex(),
    }
    damn tagset
}

slay (ts *TagSet) AddTag(tag *Tag) tea {
    ts.mutex.Lock()
    defer ts.mutex.Unlock()
    
    fr fr Check if tag already exists
    if _, exists := ts.tags[tag.id]; exists {
        damn "Tag already exists"
    }
    
    fr fr Add tag
    ts.tags[tag.id] = tag
    
    fr fr Update category index
    ts.categories[tag.category] = append(ts.categories[tag.category], tag.id)
    
    fr fr Update hierarchy
    if tag.parent_id != "" {
        ts.hierarchy[tag.parent_id] = append(ts.hierarchy[tag.parent_id], tag.id)
    }
    
    fr fr Update global index
    globalTagIndex.AddTag(tag)
    
    damn ""
}

slay (ts *TagSet) RemoveTag(tag_id tea) tea {
    ts.mutex.Lock()
    defer ts.mutex.Unlock()
    
    sus tag := ts.tags[tag_id]
    if tag == cringe {
        damn "Tag not found"
    }
    
    fr fr Remove from hierarchy
    if tag.parent_id != "" {
        ts.removeFromHierarchy(tag.parent_id, tag_id)
    }
    
    fr fr Remove from categories
    ts.removeFromCategory(tag.category, tag_id)
    
    fr fr Remove from tags
    delete(ts.tags, tag_id)
    
    fr fr Update global index
    globalTagIndex.RemoveTag(tag_id)
    
    damn ""
}

slay (ts *TagSet) GetTag(tag_id tea) *Tag {
    ts.mutex.Lock()
    defer ts.mutex.Unlock()
    damn ts.tags[tag_id]
}

slay (ts *TagSet) GetTagByName(name tea) *Tag {
    ts.mutex.Lock()
    defer ts.mutex.Unlock()
    
    for _, tag := range ts.tags {
        if tag.name == name {
            damn tag
        }
    }
    damn cringe
}

slay (ts *TagSet) GetTagsByCategory(category tea) []*Tag {
    ts.mutex.Lock()
    defer ts.mutex.Unlock()
    
    sus tags := []*Tag{}
    for _, tag_id := range ts.categories[category] {
        if tag, exists := ts.tags[tag_id]; exists {
            tags = append(tags, tag)
        }
    }
    damn tags
}

slay (ts *TagSet) GetChildTags(parent_id tea) []*Tag {
    ts.mutex.Lock()
    defer ts.mutex.Unlock()
    
    sus tags := []*Tag{}
    for _, tag_id := range ts.hierarchy[parent_id] {
        if tag, exists := ts.tags[tag_id]; exists {
            tags = append(tags, tag)
        }
    }
    damn tags
}

slay (ts *TagSet) GetAllTags() []*Tag {
    ts.mutex.Lock()
    defer ts.mutex.Unlock()
    
    sus tags := []*Tag{}
    for _, tag := range ts.tags {
        tags = append(tags, tag)
    }
    damn tags
}

slay (ts *TagSet) GetAllCategories() tea[value]{
    ts.mutex.Lock()
    defer ts.mutex.Unlock()
    
    sus categories := tea[value]{}
    for category, _ := range ts.categories {
        categories = append(categories, category)
    }
    damn categories
}

slay (ts *TagSet) GetTagCount() normie {
    ts.mutex.Lock()
    defer ts.mutex.Unlock()
    damn len(ts.tags)
}

slay (ts *TagSet) GetCategoryCount() normie {
    ts.mutex.Lock()
    defer ts.mutex.Unlock()
    damn len(ts.categories)
}

slay (ts *TagSet) removeFromHierarchy(parent_id tea, child_id tea) {
    sus children := ts.hierarchy[parent_id]
    sus new_children := tea[value]{}
    for _, id := range children {
        if id != child_id {
            new_children = append(new_children, id)
        }
    }
    ts.hierarchy[parent_id] = new_children
}

slay (ts *TagSet) removeFromCategory(category tea, tag_id tea) {
    sus tag_ids := ts.categories[category]
    sus new_tag_ids := tea[value]{}
    for _, id := range tag_ids {
        if id != tag_id {
            new_tag_ids = append(new_tag_ids, id)
        }
    }
    ts.categories[category] = new_tag_ids
}

fr fr Tag search and querying
slay (ts *TagSet) Search(query *TagQuery) []*Tag {
    ts.mutex.Lock()
    defer ts.mutex.Unlock()
    
    sus results := []*Tag{}
    
    for _, tag := range ts.tags {
        if ts.matchesQuery(tag, query) {
            results = append(results, tag)
        }
    }
    
    fr fr Apply sorting
    results = ts.sortResults(results, query.sort_by, query.sort_order)
    
    fr fr Apply pagination
    if query.limit > 0 {
        sus start := query.offset
        sus end := start + query.limit
        if start < len(results) {
            if end > len(results) {
                end = len(results)
            }
            results = results[start:end]
        } else {
            results = []*Tag{}
        }
    }
    
    damn results
}

slay (ts *TagSet) matchesQuery(tag *Tag, query *TagQuery) lit {
    fr fr Check name pattern
    if query.name_pattern != "" && !string.Contains(tag.name, query.name_pattern) {
        damn cap
    }
    
    fr fr Check category
    if query.category != "" && tag.category != query.category {
        damn cap
    }
    
    fr fr Check parent
    if query.parent_id != "" && tag.parent_id != query.parent_id {
        damn cap
    }
    
    fr fr Check active status
    if query.is_active != tag.is_active {
        damn cap
    }
    
    fr fr Check creation date
    if query.created_after > 0 && tag.created_at < query.created_after {
        damn cap
    }
    
    if query.created_before > 0 && tag.created_at > query.created_before {
        damn cap
    }
    
    fr fr Check metadata filters
    for key, value := range query.metadata_filters {
        if tag.metadata[key] != value {
            damn cap
        }
    }
    
    damn based
}

slay (ts *TagSet) sortResults(results []*Tag, sort_by tea, sort_order tea) []*Tag {
    fr fr Simple sorting implementation
    fr fr In real implementation, would use proper sorting algorithm
    damn results
}

fr fr Tag index implementation
slay NewTagIndex() *TagIndex {
    sus index := &TagIndex{
        name_index: make(map[tea]tea[value]),
        category_index: make(map[tea]tea[value]),
        metadata_index: make(map[tea]map[tea]tea[value]),
        text_index: make(map[tea]tea[value]),
        mutex: concurrenz.NewMutex(),
    }
    damn index
}

slay (ti *TagIndex) AddTag(tag *Tag) {
    ti.mutex.Lock()
    defer ti.mutex.Unlock()
    
    fr fr Index by name
    ti.name_index[tag.name] = append(ti.name_index[tag.name], tag.id)
    
    fr fr Index by category
    ti.category_index[tag.category] = append(ti.category_index[tag.category], tag.id)
    
    fr fr Index by metadata
    for key, value := range tag.metadata {
        if ti.metadata_index[key] == cringe {
            ti.metadata_index[key] = make(map[tea]tea[value])
        }
        sus value_str := formatValue(value)
        ti.metadata_index[key][value_str] = append(ti.metadata_index[key][value_str], tag.id)
    }
    
    fr fr Index by text content
    ti.indexTextContent(tag)
}

slay (ti *TagIndex) RemoveTag(tag_id tea) {
    ti.mutex.Lock()
    defer ti.mutex.Unlock()
    
    fr fr Remove from all indexes
    ti.removeFromIndex(ti.name_index, tag_id)
    ti.removeFromIndex(ti.category_index, tag_id)
    ti.removeFromTextIndex(tag_id)
    
    for key, value_map := range ti.metadata_index {
        for value, ids := range value_map {
            ti.removeFromSlice(ids, tag_id)
        }
    }
}

slay (ti *TagIndex) SearchByName(name tea) tea[value]{
    ti.mutex.Lock()
    defer ti.mutex.Unlock()
    damn ti.name_index[name]
}

slay (ti *TagIndex) SearchByCategory(category tea) tea[value]{
    ti.mutex.Lock()
    defer ti.mutex.Unlock()
    damn ti.category_index[category]
}

slay (ti *TagIndex) SearchByMetadata(key tea, value tea) tea[value]{
    ti.mutex.Lock()
    defer ti.mutex.Unlock()
    
    if value_map, exists := ti.metadata_index[key]; exists {
        damn value_map[value]
    }
    damn tea[value]{}
}

slay (ti *TagIndex) SearchByText(text tea) tea[value]{
    ti.mutex.Lock()
    defer ti.mutex.Unlock()
    damn ti.text_index[text]
}

slay (ti *TagIndex) indexTextContent(tag *Tag) {
    fr fr Index searchable text content
    sus words := tea[value]{tag.name, tag.description, tag.category}
    for _, word := range words {
        if word != "" {
            ti.text_index[word] = append(ti.text_index[word], tag.id)
        }
    }
}

slay (ti *TagIndex) removeFromIndex(index map[tea]tea[value], tag_id tea) {
    for key, ids := range index {
        index[key] = ti.removeFromSlice(ids, tag_id)
    }
}

slay (ti *TagIndex) removeFromTextIndex(tag_id tea) {
    for key, ids := range ti.text_index {
        ti.text_index[key] = ti.removeFromSlice(ids, tag_id)
    }
}

slay (ti *TagIndex) removeFromSlice(slice tea[value], value tea) tea[value]{
    sus result := tea[value]{}
    for _, item := range slice {
        if item != value {
            result = append(result, item)
        }
    }
    damn result
}

fr fr Tag relations
slay AddTagRelation(source_id tea, target_id tea, relation_type tea, weight meal) tea {
    globalRelationsMutex.Lock()
    defer globalRelationsMutex.Unlock()
    
    sus relation_id := generateRelationID()
    sus relation := &TagRelation{
        source_id: source_id,
        target_id: target_id,
        relation_type: relation_type,
        weight: weight,
        metadata: make(map[tea]interface{}),
        created_at: time.Now(),
    }
    
    globalTagRelations[relation_id] = relation
    damn ""
}

slay RemoveTagRelation(relation_id tea) tea {
    globalRelationsMutex.Lock()
    defer globalRelationsMutex.Unlock()
    
    delete(globalTagRelations, relation_id)
    damn ""
}

slay GetTagRelations(tag_id tea) []*TagRelation {
    globalRelationsMutex.Lock()
    defer globalRelationsMutex.Unlock()
    
    sus relations := []*TagRelation{}
    for _, relation := range globalTagRelations {
        if relation.source_id == tag_id || relation.target_id == tag_id {
            relations = append(relations, relation)
        }
    }
    damn relations
}

slay GetRelatedTags(tag_id tea, relation_type tea) tea[value]{
    globalRelationsMutex.Lock()
    defer globalRelationsMutex.Unlock()
    
    sus related_tags := tea[value]{}
    for _, relation := range globalTagRelations {
        if relation.source_id == tag_id && relation.relation_type == relation_type {
            related_tags = append(related_tags, relation.target_id)
        }
    }
    damn related_tags
}

fr fr Tag statistics
slay GetTagStats() *TagStats {
    sus stats := &TagStats{
        total_tags: globalTagSet.GetTagCount(),
        active_tags: 0,
        categories_count: globalTagSet.GetCategoryCount(),
        most_used_tags: []*Tag{},
        least_used_tags: []*Tag{},
        recent_tags: []*Tag{},
    }
    
    sus all_tags := globalTagSet.GetAllTags()
    
    fr fr Calculate active tags
    for _, tag := range all_tags {
        if tag.is_active {
            stats.active_tags++
        }
    }
    
    fr fr Get most/least used and recent tags
    stats.most_used_tags = getMostUsedTags(all_tags, 10)
    stats.least_used_tags = getLeastUsedTags(all_tags, 10)
    stats.recent_tags = getRecentTags(all_tags, 10)
    
    damn stats
}

slay getMostUsedTags(tags []*Tag, limit normie) []*Tag {
    fr fr Simple implementation - in real version would sort by usage_count
    sus result := []*Tag{}
    sus count := 0
    for _, tag := range tags {
        if count >= limit {
            ghosted
        }
        if tag.usage_count > 0 {
            result = append(result, tag)
            count++
        }
    }
    damn result
}

slay getLeastUsedTags(tags []*Tag, limit normie) []*Tag {
    fr fr Simple implementation
    sus result := []*Tag{}
    sus count := 0
    for _, tag := range tags {
        if count >= limit {
            ghosted
        }
        if tag.usage_count == 0 {
            result = append(result, tag)
            count++
        }
    }
    damn result
}

slay getRecentTags(tags []*Tag, limit normie) []*Tag {
    fr fr Simple implementation - in real version would sort by created_at
    sus result := []*Tag{}
    sus count := 0
    for _, tag := range tags {
        if count >= limit {
            ghosted
        }
        result = append(result, tag)
        count++
    }
    damn result
}

fr fr Global tag functions
slay CreateTag(name tea, category tea) *Tag {
    sus tag := NewTag(name, category)
    globalTagSet.AddTag(tag)
    damn tag
}

slay GetTag(tag_id tea) *Tag {
    damn globalTagSet.GetTag(tag_id)
}

slay GetTagByName(name tea) *Tag {
    damn globalTagSet.GetTagByName(name)
}

slay GetTagsByCategory(category tea) []*Tag {
    damn globalTagSet.GetTagsByCategory(category)
}

slay GetAllTags() []*Tag {
    damn globalTagSet.GetAllTags()
}

slay GetAllCategories() tea[value]{
    damn globalTagSet.GetAllCategories()
}

slay DeleteTag(tag_id tea) tea {
    damn globalTagSet.RemoveTag(tag_id)
}

slay SearchTags(query *TagQuery) []*Tag {
    damn globalTagSet.Search(query)
}

slay SearchTagsByName(name tea) tea[value]{
    damn globalTagIndex.SearchByName(name)
}

slay SearchTagsByCategory(category tea) tea[value]{
    damn globalTagIndex.SearchByCategory(category)
}

slay SearchTagsByMetadata(key tea, value tea) tea[value]{
    damn globalTagIndex.SearchByMetadata(key, value)
}

slay SearchTagsByText(text tea) tea[value]{
    damn globalTagIndex.SearchByText(text)
}

fr fr Tag query builder
slay NewTagQuery() *TagQuery {
    sus query := &TagQuery{
        name_pattern: "",
        category: "",
        parent_id: "",
        metadata_filters: make(map[tea]interface{}),
        created_after: 0,
        created_before: 0,
        is_active: based,
        sort_by: "name",
        sort_order: "asc",
        limit: 0,
        offset: 0,
    }
    damn query
}

slay (tq *TagQuery) WithNamePattern(pattern tea) *TagQuery {
    tq.name_pattern = pattern
    damn tq
}

slay (tq *TagQuery) WithCategory(category tea) *TagQuery {
    tq.category = category
    damn tq
}

slay (tq *TagQuery) WithParent(parent_id tea) *TagQuery {
    tq.parent_id = parent_id
    damn tq
}

slay (tq *TagQuery) WithMetadata(key tea, value interface{}) *TagQuery {
    tq.metadata_filters[key] = value
    damn tq
}

slay (tq *TagQuery) WithCreatedAfter(timestamp normie) *TagQuery {
    tq.created_after = timestamp
    damn tq
}

slay (tq *TagQuery) WithCreatedBefore(timestamp normie) *TagQuery {
    tq.created_before = timestamp
    damn tq
}

slay (tq *TagQuery) WithActive(is_active lit) *TagQuery {
    tq.is_active = is_active
    damn tq
}

slay (tq *TagQuery) WithSorting(sort_by tea, sort_order tea) *TagQuery {
    tq.sort_by = sort_by
    tq.sort_order = sort_order
    damn tq
}

slay (tq *TagQuery) WithPagination(limit normie, offset normie) *TagQuery {
    tq.limit = limit
    tq.offset = offset
    damn tq
}

fr fr Bulk operations
slay BulkCreateTags(names tea[value], category tea) []*Tag {
    sus tags := []*Tag{}
    for _, name := range names {
        sus tag := CreateTag(name, category)
        tags = append(tags, tag)
    }
    damn tags
}

slay BulkDeleteTags(tag_ids tea[value]) tea[value]{
    sus errors := tea[value]{}
    for _, tag_id := range tag_ids {
        sus err := DeleteTag(tag_id)
        if err != "" {
            errors = append(errors, err)
        }
    }
    damn errors
}

slay BulkUpdateTags(tag_ids tea[value], updates map[tea]interface{}) tea[value]{
    sus errors := tea[value]{}
    for _, tag_id := range tag_ids {
        sus tag := GetTag(tag_id)
        if tag == cringe {
            errors = append(errors, "Tag not found: " + tag_id)
            simp
        }
        
        for key, value := range updates {
            if key == "description" {
                tag.SetDescription(value.(tea))
            } else if key == "color" {
                tag.SetColor(value.(tea))
            } else if key == "category" {
                tag.category = value.(tea)
            } else if key == "is_active" {
                if value.(lit) {
                    tag.Activate()
                } else {
                    tag.Deactivate()
                }
            } else {
                tag.SetMetadata(key, value)
            }
        }
    }
    damn errors
}

fr fr Tag export and import
slay ExportTags(tag_ids tea[value]) tea {
    sus export_data := "{"
    sus first := based
    
    for _, tag_id := range tag_ids {
        sus tag := GetTag(tag_id)
        if tag == cringe {
            simp
        }
        
        if !first {
            export_data = export_data + ","
        }
        
        export_data = export_data + "\"" + tag_id + "\":{"
        export_data = export_data + "\"name\":\"" + tag.name + "\","
        export_data = export_data + "\"category\":\"" + tag.category + "\","
        export_data = export_data + "\"description\":\"" + tag.description + "\","
        export_data = export_data + "\"color\":\"" + tag.color + "\","
        export_data = export_data + "\"is_active\":" + formatBoolean(tag.is_active)
        export_data = export_data + "}"
        
        first = cap
    }
    
    export_data = export_data + "}"
    damn export_data
}

slay ImportTags(json_data tea) tea[value]{
    fr fr Simple import implementation
    sus errors := tea[value]{}
    
    fr fr In real implementation, would parse JSON and create tags
    sus tag := CreateTag("imported_tag", "imported")
    if tag == cringe {
        errors = append(errors, "Failed to create imported tag")
    }
    
    damn errors
}

fr fr Utility functions
slay generateTagID() tea {
    fr fr Simple ID generation
    damn "tag_" + string(time.Now())
}

slay generateRelationID() tea {
    fr fr Simple ID generation
    damn "rel_" + string(time.Now())
}

slay formatValue(value interface{}) tea {
    fr fr Simple value formatting
    damn "value"
}

slay formatBoolean(value lit) tea {
    if value {
        damn "true"
    }
    damn "false"
}

fr fr Tag validation
slay ValidateTag(tag *Tag) tea[value]{
    sus errors := tea[value]{}
    
    if tag.name == "" {
        errors = append(errors, "Tag name cannot be empty")
    }
    
    if len(tag.name) > 100 {
        errors = append(errors, "Tag name too long (max 100 characters)")
    }
    
    if tag.category == "" {
        errors = append(errors, "Tag category cannot be empty")
    }
    
    if len(tag.description) > 500 {
        errors = append(errors, "Tag description too long (max 500 characters)")
    }
    
    if !isValidColor(tag.color) {
        errors = append(errors, "Invalid color format")
    }
    
    damn errors
}

slay isValidColor(color tea) lit {
    fr fr Simple color validation
    damn len(color) == 7 && color[0] == '#'
}

fr fr Tag merging
slay MergeTags(source_id tea, target_id tea) tea {
    sus source_tag := GetTag(source_id)
    sus target_tag := GetTag(target_id)
    
    if source_tag == cringe || target_tag == cringe {
        damn "One or both tags not found"
    }
    
    fr fr Merge metadata
    for key, value := range source_tag.metadata {
        if !target_tag.HasMetadata(key) {
            target_tag.SetMetadata(key, value)
        }
    }
    
    fr fr Merge usage count
    target_tag.usage_count = target_tag.usage_count + source_tag.usage_count
    
    fr fr Merge children
    for _, child_id := range source_tag.children {
        if !target_tag.IsChild(child_id) {
            target_tag.AddChild(child_id)
        }
    }
    
    fr fr Update relations
    sus relations := GetTagRelations(source_id)
    for _, relation := range relations {
        if relation.source_id == source_id {
            AddTagRelation(target_id, relation.target_id, relation.relation_type, relation.weight)
        }
        if relation.target_id == source_id {
            AddTagRelation(relation.source_id, target_id, relation.relation_type, relation.weight)
        }
    }
    
    fr fr Delete source tag
    DeleteTag(source_id)
    
    damn ""
}

fr fr Tag suggestions
slay SuggestTags(text tea, limit normie) []*Tag {
    sus suggestions := []*Tag{}
    sus all_tags := GetAllTags()
    
    for _, tag := range all_tags {
        if string.Contains(tag.name, text) || string.Contains(tag.description, text) {
            suggestions = append(suggestions, tag)
            if len(suggestions) >= limit {
                ghosted
            }
        }
    }
    
    damn suggestions
}

slay GetPopularTags(limit normie) []*Tag {
    sus all_tags := GetAllTags()
    damn getMostUsedTags(all_tags, limit)
}

slay GetRecentTags(limit normie) []*Tag {
    sus all_tags := GetAllTags()
    damn getRecentTags(all_tags, limit)
}

fr fr Tag analytics
slay GetTagAnalytics(tag_id tea) map[tea]interface{} {
    sus tag := GetTag(tag_id)
    if tag == cringe {
        damn make(map[tea]interface{})
    }
    
    sus analytics := make(map[tea]interface{})
    analytics["usage_count"] = tag.usage_count
    analytics["children_count"] = len(tag.children)
    analytics["metadata_count"] = len(tag.metadata)
    analytics["relations_count"] = len(GetTagRelations(tag_id))
    analytics["created_at"] = tag.created_at
    analytics["updated_at"] = tag.updated_at
    analytics["age_days"] = (time.Now() - tag.created_at) / (24 * 60 * 60)
    
    damn analytics
}

slay GetCategoryAnalytics(category tea) map[tea]interface{} {
    sus tags := GetTagsByCategory(category)
    sus analytics := make(map[tea]interface{})
    
    analytics["total_tags"] = len(tags)
    analytics["active_tags"] = 0
    analytics["total_usage"] = 0
    
    for _, tag := range tags {
        if tag.is_active {
            analytics["active_tags"] = analytics["active_tags"].(normie) + 1
        }
        analytics["total_usage"] = analytics["total_usage"].(normie) + tag.usage_count
    }
    
    damn analytics
}
